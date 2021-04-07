mod credentials;
mod http;
mod websocket;

use credentials::Credentials;

use thiserror::Error;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, TransportError>;
}

pub trait Subscribe {
    fn read_next(&mut self) -> Result<String, TransportError>;
    fn fork(&mut self) -> Result<String, TransportError>;
}

pub trait Connection {
    type Error;

    fn new(address: &str, credentials: Option<Credentials>) -> Result<Self, Self::Error>
    where Self: Sized;
    //fn call();
    //fn get_command_id();
    //fn close();
}

/// Wraps the different transport errors that can happen
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] websocket::WebSocketError),
    #[error("{0}")]
    HttpError(#[from] http::HttpError),
}
