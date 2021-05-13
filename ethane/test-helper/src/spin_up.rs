use ethane::rpc::{sub::SubscriptionRequest, Rpc};
use ethane::{Connection, ConnectionError, Http, Request, Subscribe, Subscription, WebSocket};
use regex::{Regex, RegexBuilder};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command};

#[cfg(target_family = "unix")]
use ethane::Uds;
#[cfg(target_family = "unix")]
use rand::distributions::Alphanumeric;
#[cfg(target_family = "unix")]
use rand::{thread_rng, Rng};

pub enum ConnectionWrapper {
    Websocket(ConnectionNodeBundle<WebSocket>),
    Http(ConnectionNodeBundle<Http>),
    #[cfg(target_family = "unix")]
    Uds(ConnectionNodeBundle<Uds>),
}

impl ConnectionWrapper {
    pub fn new_from_env(conn_type: Option<&str>) -> ConnectionWrapper {
        let conn = match conn_type {
            Some(conn) => String::from(conn),
            None => String::from(
                std::env::var("CONNECTION")
                    .unwrap_or_else(|_| String::from("ganache")) // @TODO
                    .as_str(),
            ),
        };
        match &conn[..] {
            "ganache" => Self::Http(ConnectionNodeBundle::ganache()),
            "http" => Self::Http(ConnectionNodeBundle::http()),
            "ws" => Self::Websocket(ConnectionNodeBundle::ws()),
            #[cfg(target_family = "unix")]
            "uds" => Self::Uds(ConnectionNodeBundle::uds()),
            #[cfg(target_family = "unix")]
            _ => panic!("Please set environment variable 'CONNECTION'. Valid values are either 'http', 'ws' or 'uds'"),
            #[cfg(not(target_family = "unix"))]
            _ => panic!("Please set environment variable 'CONNECTION'. Valid values are either 'http' or 'ws'"),
        }
    }

    pub fn call<U: DeserializeOwned + Debug>(&mut self, rpc: Rpc<U>) -> Result<U, ConnectionError> {
        match self {
            Self::Websocket(connection) => connection.call(rpc),
            Self::Http(connection) => connection.call(rpc),
            #[cfg(target_family = "unix")]
            Self::Uds(connection) => connection.call(rpc),
        }
    }

    pub fn subscribe<U: DeserializeOwned + Debug + 'static>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Box<dyn DynSubscription<U>>, ConnectionError> {
        match self {
            Self::Websocket(connection) => connection.subscribe(sub_request),
            #[cfg(target_family = "unix")]
            Self::Uds(connection) => connection.subscribe(sub_request),
            _ => panic!("Subscription not supported for this transport"),
        }
    }
}

pub trait DynSubscription<U: DeserializeOwned + Debug> {
    fn next_item(&mut self) -> Result<U, ConnectionError>;
}

impl<T: Subscribe + Request, U: DeserializeOwned + Debug> DynSubscription<U>
    for Subscription<T, U>
{
    fn next_item(&mut self) -> Result<U, ConnectionError> {
        self.next_item()
    }
}

#[allow(dead_code)]
pub struct ConnectionNodeBundle<T: Request> {
    connection: Connection<T>,
    process: Option<NodeProcess>,
}

impl<T: Request> ConnectionNodeBundle<T> {
    fn call<U: DeserializeOwned + Debug>(&mut self, rpc: Rpc<U>) -> Result<U, ConnectionError> {
        self.connection.call(rpc)
    }
}

impl<T: Subscribe + Request + 'static> ConnectionNodeBundle<T> {
    pub fn subscribe<U: DeserializeOwned + Debug + 'static>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Box<dyn DynSubscription<U>>, ConnectionError> {
        let sub_result = self.connection.subscribe(sub_request);
        sub_result.map(|el| Box::new(el) as Box<dyn DynSubscription<U>>)
    }
}

impl ConnectionNodeBundle<WebSocket> {
    pub fn ws() -> Self {
        let process = NodeProcess::new_ws("0");
        let connection =
            Connection::new(WebSocket::new(&format!("ws://{}", process.address), None).unwrap());
        ConnectionNodeBundle {
            connection,
            process: Some(process),
        }
    }
}

impl ConnectionNodeBundle<Http> {
    pub fn http() -> Self {
        let process = NodeProcess::new_http("0");
        let connection = Connection::new(Http::new(&format!("http://{}", process.address), None));
        ConnectionNodeBundle {
            connection,
            process: Some(process),
        }
    }

    pub fn ganache() -> Self {
        let connection = Connection::new(Http::new("http://localhost:8545", None));
        ConnectionNodeBundle {
            connection,
            process: None,
        }
    }
}

#[cfg(target_family = "unix")]
impl ConnectionNodeBundle<Uds> {
    pub fn uds() -> Self {
        let process = NodeProcess::new_uds(None);
        let connection = Connection::new(Uds::new(&process.address).unwrap());
        ConnectionNodeBundle {
            connection,
            process: Some(process),
        }
    }
}

pub struct NodeProcess {
    pub address: String,
    process: Child,
}

impl NodeProcess {
    pub fn new_http(port: &str) -> Self {
        let regex = RegexBuilder::new(r"HTTP server started\s+endpoint=([0-9.:]+)")
            .build()
            .unwrap();
        let cmd = vec![
            "--http".to_string(),
            "--http.api".to_string(),
            "personal,eth,net,web3,txpool".to_string(),
            "--http.port".to_string(),
            port.to_string(),
            "--allow-insecure-unlock".to_string(),
            "--ipcdisable".to_string(),
        ];
        Self::new(cmd, regex)
    }

    pub fn new_ws(port: &str) -> Self {
        let regex = RegexBuilder::new(r"WebSocket enabled\s+url=ws://([0-9.:]+)")
            .build()
            .unwrap();
        let cmd = vec![
            "--ws".to_string(),
            "--ws.api".to_string(),
            "personal,eth,net,web3,txpool".to_string(),
            "--ws.port".to_string(),
            port.to_string(),
            "--allow-insecure-unlock".to_string(),
            "--ipcdisable".to_string(),
        ];
        Self::new(cmd, regex)
    }

    #[cfg(target_family = "unix")]
    pub fn new_uds(path: Option<&str>) -> Self {
        let regex = RegexBuilder::new(r"IPC endpoint opened\s+url=([a-z0-9/\\:_]+.ipc)")
            .build()
            .unwrap();
        let mut cmd = vec!["--ipcpath".to_string()];

        if let Some(ipc_path) = path {
            cmd.push(ipc_path.to_string());
        } else {
            let mut rng = thread_rng();
            let chars = std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(8)
                .collect::<String>()
                .to_lowercase();

            let ipc_path = String::from("/tmp/geth_") + &chars + ".ipc";
            cmd.push(ipc_path)
        }
        Self::new(cmd, regex)
    }

    fn new(settings: Vec<String>, regex: Regex) -> Self {
        let mut cmd = vec!["--dev".to_string()];
        cmd.extend(settings);
        let mut geth = Command::new("geth")
            .args(cmd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Unable to start local geth node for integration tests. Is geth installed?");

        let mut reader = BufReader::new(geth.stderr.take().unwrap());
        let mut buffer = String::new();
        let mut parsed = String::new();
        loop {
            reader.read_line(&mut buffer).unwrap();
            for capture in regex.captures_iter(&buffer) {
                if let Some(cap) = capture.get(1) {
                    parsed = cap.as_str().to_string();
                }
            }
            if !parsed.is_empty() {
                break;
            }
        }

        // For some reason the process dies, if we drop stderr. This is why we need to reattach it here
        geth.stderr = Some(reader.into_inner());

        NodeProcess {
            address: parsed,
            process: geth,
        }
    }
}

impl Drop for NodeProcess {
    fn drop(&mut self) {
        let e_message = format!(
            "Unable to tear down eth node. Please kill PID {} manually.",
            self.process.id()
        );
        let mut cmd = Command::new("kill");
        if let Ok(mut child) = cmd.arg(self.process.id().to_string()).spawn() {
            if !child.wait().expect(&e_message).success() {
                println!("{}", &e_message);
            }
        } else {
            println!("{}", &e_message);
        }
    }
}
