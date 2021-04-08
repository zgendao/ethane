mod credentials;
mod subscription;
mod transport;

pub use credentials::Credentials;
pub use subscription::{Subscription, SubscriptionError};
pub use transport::http::{Http, HttpError};
#[cfg(target_family = "unix")]
pub use transport::uds::{Uds, UdsError};
pub use transport::websocket::{WebSocket, WebSocketError};

use crate::rpc::{sub::SubscriptionRequest, Rpc};

use log::{debug, info, trace};
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
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            id_pool: (0..1000).collect(),
        }
    }

    pub fn call<U>(&mut self, mut rpc: Rpc<U>) -> Result<U, ConnectionError>
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

impl<T> Connection<T>
where
    T: Request + Subscribe,
{
    /// Starts a new subscription.
    /// Use one of these rpc generating [functions](crate::rpc::sub) to provide the subscription request.
    /// Returns a [subscription](Subscription) which you can poll for new items.
    pub fn subscribe<U: DeserializeOwned + std::fmt::Debug>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<T, U>, ConnectionError> {
        info!("Starting a new subscription");
        let mut connection = Connection {
            transport: self.transport.fork()?,
            id_pool: self.id_pool.clone(),
        };
        let subscription_id = connection.call(sub_request.rpc)?;
        Ok(Subscription {
            id: subscription_id,
            connection,
            result_type: std::marker::PhantomData,
        })
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
    #[cfg(target_family = "unix")]
    #[error("{0}")]
    UdsError(#[from] UdsError),
    #[error("Node Response Error: {0:?}")]
    JsonRpc(#[from] JsonError),
    #[error("Connector De-/Serialization Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Connector Error: Maximum number of connections reached")]
    NoTicketId,
}
