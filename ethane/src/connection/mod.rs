mod credentials;
mod http;
mod websocket;

use credentials::Credentials;

use crate::Rpc;

use serde::{Deserialize, de::DeserializeOwned};
use thiserror::Error;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError>;
}

pub trait Subscribe {
    fn read_next(&mut self) -> Result<String, ConnectionError>;
    fn fork(&mut self) -> Result<String, ConnectionError>;
}

pub trait Connection {
    type Error;

    fn new(address: &str, credentials: Option<Credentials>) -> Result<Self, Self::Error>
    where Self: Sized;
    fn call<U: DeserializeOwned + std::fmt::Debug>(&mut self, rpc: &mut Rpc<U>) -> Result<U, ConnectionError>;
    //fn close();
}

/// Wraps the different transport errors that can happen
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("{0}")]
    WebSocketError(#[from] websocket::WebSocketError),
    #[error("{0}")]
    HttpError(#[from] http::HttpError),
    #[error("Node Response Error: {0:?}")]
    JsonRpc(#[from] JsonError),
    #[error("Connector De-/Serialization Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Connector Error: Maximum number of connections reached")]
    NoTicketId,
}

/// Used to deserialize errors returned from the ethereum node
#[derive(Deserialize, Debug, Error)]
#[error("{message}")]
pub struct JsonError {
    code: i32,
    message: String,
}
