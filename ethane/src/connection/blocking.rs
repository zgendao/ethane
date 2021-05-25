use super::{ConnectionError, Request, Subscribe, Subscription};
use crate::rpc::{Rpc, RpcResponse, SubscriptionRequest};

use serde::de::DeserializeOwned;

pub struct Connection<T: Request> {
    pub(super) transport: T, // subscription uses this field
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
            let result_data = self.transport.request(
                serde_json::to_string(&rpc).map_err(|e| ConnectionError::Serde(e.to_string()))?,
            )?;
            let result = serde_json::from_str::<RpcResponse<U>>(&result_data)
                .map_err(|e| ConnectionError::Serde(e.to_string()))?;
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
