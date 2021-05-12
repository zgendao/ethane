//! Implementation of a websocket transport.

use super::super::{ConnectionError, Credentials, Request, Subscribe};
use tungstenite::handshake::client::Request as TungsteniteRequest;

/// Wraps a websocket connection
pub struct WebSocket {
    address: String,
    credentials: Option<Credentials>,
    websocket: tungstenite::WebSocket<tungstenite::client::AutoStream>,
}

impl WebSocket {
    pub fn new(address: &str, credentials: Option<Credentials>) -> Result<Self, ConnectionError> {
        let request = create_handshake_request(address, &credentials).unwrap();
        let ws = tungstenite::connect(request)?;
        Ok(Self {
            address: address.to_owned(),
            credentials,
            websocket: ws.0,
        })
    }

    fn read_message(&mut self) -> Result<String, ConnectionError> {
        match self.read() {
            Ok(tungstenite::Message::Text(response)) => Ok(response),
            Ok(_) => self.read_message(),
            Err(err) => Err(err),
        }
    }

    fn read(&mut self) -> Result<tungstenite::Message, ConnectionError> {
        let message = self.websocket.read_message()?;
        Ok(message)
    }

    fn write(&mut self, message: tungstenite::Message) -> Result<(), ConnectionError> {
        self.websocket.write_message(message)?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), ConnectionError> {
        use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};
        let close_frame = CloseFrame {
            code: CloseCode::Normal,
            reason: std::borrow::Cow::from("Finished"),
        };
        self.websocket.close(Some(close_frame))?;
        self.websocket
            .write_pending()
            .map_err(ConnectionError::from)
    }
}

impl Request for WebSocket {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError> {
        let write_msg = tungstenite::Message::Text(cmd);
        self.write(write_msg)?;
        self.read_message()
            .map_err(|e| ConnectionError::WebSocketError(e.to_string()))
    }
}

impl Subscribe for WebSocket {
    fn read_next(&mut self) -> Result<String, ConnectionError> {
        self.read_message().map_err(ConnectionError::from)
    }

    fn fork(&self) -> Result<Self, ConnectionError> {
        Self::new(&self.address, self.credentials.clone())
            .map_err(|e| ConnectionError::WebSocketError(e.to_string()))
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        if self.close().is_err() {
            println!("Error while closing websocket");
        }
    }
}

fn create_handshake_request(
    address: &str,
    credentials: &Option<Credentials>,
) -> Result<TungsteniteRequest, ConnectionError> {
    let mut request = TungsteniteRequest::get(address).body(()).map_err(|_| {
        ConnectionError::WebSocketError(format!("Couldn't bind WS to address {}", address))
    })?;
    if let Some(cred) = credentials {
        request.headers_mut().insert(
            "Authorization",
            cred.to_auth_string().parse().map_err(|_| {
                ConnectionError::WebSocketError("Couldn't parse auth string".to_string())
            })?,
        );
    }

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, TcpStream};
    use tungstenite::{accept, Message};

    fn spawn_websocket_server<F>(mut handle_ws_stream: F, port: u16)
    where
        F: FnMut(&mut tungstenite::WebSocket<TcpStream>) + Send + 'static,
    {
        let tcp_listener =
            std::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();

        let _thread = std::thread::Builder::new()
            .name("Websocket Server".to_string())
            .spawn(move || loop {
                match tcp_listener.accept() {
                    Ok((tcp_stream, _address)) => match accept(tcp_stream) {
                        Ok(mut websocket) => handle_ws_stream(&mut websocket),
                        Err(err) => panic!("{}", err),
                    },
                    Err(err) => panic!("{}", err),
                }
            })
            .is_ok();
    }

    fn ping_pong(ws_stream: &mut tungstenite::WebSocket<TcpStream>) {
        match ws_stream.read_message() {
            Ok(message) => match message {
                Message::Text(echo) => ws_stream
                    .write_message(Message::Text(echo + " Pong"))
                    .unwrap(),
                _ => panic!("Received other message type."),
            },
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn handshake_request_with_credentials() {
        let credentials = Credentials::Basic(String::from("YWJjOjEyMw=="));
        let request =
            create_handshake_request("http://localhost:8000", &Some(credentials)).unwrap();
        assert_eq!(
            request.headers().get("Authorization").unwrap(),
            "Basic YWJjOjEyMw=="
        );
    }

    #[test]
    fn handshake_request_without_credentials() {
        let request = create_handshake_request("ws://localhost:8000", &None).unwrap();
        assert_eq!(request.uri(), "ws://localhost:8000/");
    }

    #[test]
    fn ping_pong_request() {
        spawn_websocket_server(ping_pong, 3001);
        let mut ws_client = WebSocket::new("ws://localhost:3001", None).unwrap();
        let response = ws_client.request(String::from("Ping")).unwrap();
        assert_eq!(response, "Ping Pong");
    }
}
