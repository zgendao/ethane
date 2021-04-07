mod credentials;
mod subscription;
mod transport;

pub use credentials::Credentials;
pub use subscription::{Subscription, SubscriptionError};
pub use transport::http::{Http, HttpError};
pub use transport::websocket::{WebSocket, WebSocketError};

use crate::rpc::Rpc;

use log::{debug, trace};
use serde::de::DeserializeOwned;
use thiserror::Error;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError>;
}

pub trait Subscribe {
    fn read_next(&mut self) -> Result<String, ConnectionError>;
    fn fork(&self) -> Result<Self, ConnectionError>
    where
        Self: Sized;
}

pub struct Connection<T: Request> {
    transport: T,
    id_pool: std::collections::VecDeque<usize>,
}

impl<T> Connection<T>
where
    T: Request,
{
    fn new(transport: T) -> Self {
        Self {
            transport,
            id_pool: (0..1000).collect(),
        }
    }

    fn call<U>(&mut self, rpc: &mut Rpc<U>) -> Result<U, ConnectionError>
    where
        U: DeserializeOwned + std::fmt::Debug,
    {
        if let Some(id) = self.id_pool.pop_front() {
            trace!("Using id {} for request", id);
            rpc.id = id;
            debug!("Calling rpc method: {:?}", &rpc);
            self.id_pool.push_back(id);
            let result_data = self.transport.request(serde_json::to_string(&rpc)?)?;
            let result: U = serde_json::from_str(&result_data)?;
            Ok(result)
        } else {
            Err(ConnectionError::NoTicketId)
        }
    }
}

/// Used to deserialize errors returned from the ethereum node.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct JsonError {
    code: i32,
    message: String,
}

/// Wraps the different transport errors that may occur.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("{0}")]
    WebSocketError(#[from] WebSocketError),
    #[error("{0}")]
    HttpError(#[from] HttpError),
    #[error("Node Response Error: {0:?}")]
    JsonRpc(#[from] JsonError),
    #[error("Connector De-/Serialization Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Connector Error: Maximum number of connections reached")]
    NoTicketId,
}
