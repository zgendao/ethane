mod credentials;
mod subscription;
mod transport;

pub use credentials::Credentials;
pub use subscription::{Subscription, SubscriptionError};
pub use transport::http::Http;
#[cfg(target_family = "unix")]
pub use transport::uds::Uds;
pub use transport::websocket::WebSocket;

use crate::rpc::{sub::SubscriptionRequest, Rpc, RpcResponse};

use serde::de::DeserializeOwned;

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
            rpc.id = id;
            self.id_pool.push_back(id);
            let result_data = self.transport.request(serde_json::to_string(&rpc)?)?;
            let result = serde_json::from_str::<RpcResponse<U>>(&result_data)?;
            Ok(result.result)
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

/// Wraps the different transport errors that may occur.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum ConnectionError {
    WebSocketError(String),
    HttpError(String),
    UdsError(String),
    JsonRpc(String),
    Serde(String),
    SubscriptionError(String),
    NoTicketId,
}
