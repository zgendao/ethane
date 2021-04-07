//! Implementation of a websocket transport.

use super::{Connection, Credentials};

use log::{debug, error, trace};
use std::str::FromStr;
use thiserror::Error;
//use std::borrow::Cow;
//use tungstenite::client::AutoStream;
//use tungstenite::protocol::frame::coding::CloseCode;
//use tungstenite::protocol::CloseFrame;

/// Wraps a websocket connection
pub struct WebSocket {
    /// The endpoint of the websocket connection
    pub address: String,
    pub(crate) credentials: Option<Credentials>,
    ws: tungstenite::WebSocket<tungstenite::client::AutoStream>,
}

impl Connection for WebSocket {

    type Error = WebSocketError;

    fn new(address: &str, credentials: Option<Credentials>) -> Result<Self, Self::Error> {
        debug!("Initiating websocket connection to {}", address);
        let uri = http::Uri::from_str(address)?;
        //let handshake_request = create_handshake_request(&uri, &credentials)?;

        let mut request_builder = http::Request::get(&uri);
        if let Some(ref credentials) = credentials {
            let headers = request_builder.headers_mut().ok_or(WebSocketError::Handshake)?;
            headers.insert("Authorization", credentials.to_auth_string().parse()?);
        }

        let handshake_request = request_builder.body(())?;
        trace!("Built websocket handshake request: {:?}", &handshake_request);

        let ws = tungstenite::connect(handshake_request)?;
        trace!("Handshake Response: {:?}", ws.1);
        Ok(Self {
            address: address.to_owned(),
            credentials,
            ws: ws.0,
        })
    }
}

/*
impl WebSocket {
    fn read_message(&mut self) -> Result<String, WebSocketError> {
        match self.read() {
            Ok(Message::Text(response)) => Ok(response),
            Ok(_) => self.read_message(),
            Err(err) => Err(err),
        }
    }

    fn read(&mut self) -> Result<Message, WebSocketError> {
        let message = self.ws.read_message()?;
        trace!("Reading from websocket: {}", &message);
        Ok(message)
    }

    fn write(&mut self, message: Message) -> Result<(), WebSocketError> {
        trace!("Writing to websocket: {}", &message);
        self.ws.write_message(message)?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), WebSocketError> {
        debug!("Closing websocket connection");
        let close_frame = CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from("Finished"),
        };
        self.ws.close(Some(close_frame))?;
        self.ws.write_pending().map_err(WebSocketError::from)
    }
}

impl Request for WebSocket {
    fn request(&mut self, cmd: String) -> Result<String, TransportError> {
        let _write = self.write(Message::Text(cmd))?;
        self.read_message().map_err(TransportError::from)
    }
}

impl Subscribe for WebSocket {
    fn read_next(&mut self) -> Result<String, TransportError> {
        self.read_message().map_err(TransportError::from)
    }

    fn fork(&self) -> Result<Self, TransportError> {
        Self::new(self.address.clone(), self.credentials.clone()).map_err(TransportError::from)
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        let close = self.close();
        if let Err(err) = close {
            error!("{}", err);
        }
    }
}

fn create_handshake_request(
    uri: &Uri,
    credentials: Option<Credentials>,
) -> Result<HttpRequest<()>, WebSocketError> {
    let mut req_builder = HttpRequest::get(uri);
    if let Some(ref credentials) = credentials {
        let headers = req_builder.headers_mut().ok_or(WebSocketError::Handshake)?;
        headers.insert("Authorization", credentials.to_auth_string().parse()?);
    }

    let request = req_builder.body(())?;
    trace!("Built websocket handshake request: {:?}", &request);
    Ok(request)
}
*/
/// An error type collecting what can go wrong with a websocket
#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("WebSocket Error: {0}")]
    Tungstenite(#[from] tungstenite::Error),
    #[error("WebSocket Invalid Handshake Request Error: {0}")]
    Http(#[from] http::Error),
    #[error("WebSocket Invalid Address Error: {0}")]
    Url(#[from] http::uri::InvalidUri),
    #[error("WebSocket Handshake Header Error")]
    Handshake,
    #[error("WebSocket Error. Unable to parse credentials {0}")]
    InvalidHeader(#[from] http::header::InvalidHeaderValue),
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, TcpStream};
    use tungstenite::{accept, Message};

    fn spawn_websocket_server<F>(mut handle_ws_stream: F, port: u16)
    where
        F: FnMut(&mut WebSocketTungstenite<TcpStream>) + Send + 'static,
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

    fn ping_pong(ws_stream: &mut WebSocketTungstenite<TcpStream>) {
        match ws_stream.read_message() {
            Ok(message) => match message {
                Message::Text(echo) => ws_stream
                    .write_message(Message::Text(echo + " Pong"))
                    .unwrap(),
                _ => panic!("Received other message type."),
            },
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn test_websocket_create_handshake_request_with_credentials() {
        let uri = Uri::from_static("localhost");
        let credentials = Credentials::Basic(String::from("YWJjOjEyMw=="));
        let request = create_handshake_request(&uri, Some(credentials)).unwrap();
        assert_eq!(
            request.headers().get("Authorization").unwrap(),
            "Basic YWJjOjEyMw=="
        );
    }

    #[test]
    fn test_websocket_create_handshake_request_without_credentials() {
        let uri = Uri::from_static("localhost");
        let request = create_handshake_request(&uri, None).unwrap();
        assert_eq!(request.method(), http::method::Method::GET);
        assert_eq!(request.uri(), &uri);
    }

    #[test]
    fn test_websocket_request() {
        spawn_websocket_server(ping_pong, 3001);
        let mut ws_client = WebSocket::new(String::from("ws://localhost:3001"), None).unwrap();
        let response = ws_client.request(String::from("Ping")).unwrap();
        assert_eq!(response, "Ping Pong");
    }
}
*/
