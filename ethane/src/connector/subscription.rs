use super::{Connector, ConnectorError};
use crate::rpc::eth_unsubscribe;
use crate::transport::{Request, Subscribe, TransportError};
use crate::types::U128;
use log::{error, info, trace};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;

/// An active subscription
///
/// Supports the [real-time events](https://geth.ethereum.org/docs/rpc/pubsub) namespace.
/// Can be created by calling [subscribe](crate::connector::Connector::subscribe).
/// In order to yield the next subscription item call [next_item](Self::next_item).
pub struct Subscription<T: DeserializeOwned + Debug, U: Subscribe + Request> {
    /// The subscription id, which is returned when subscribing
    pub id: U128,
    pub(crate) connector: Connector<U>,
    pub(crate) result_type: PhantomData<T>,
}

impl<T: DeserializeOwned + Debug, U: Subscribe + Request> Subscription<T, U> {
    /// Yields the next item of this subscription.
    pub fn next_item(&mut self) -> Result<T, SubscriptionError> {
        trace!("Fetching next item from subscription");
        let response = self.connector.connection.read_next()?;
        deserialize_from_sub(&response)
    }

    /// Cancel the subscription. This will first unsubscribe and then close the underlying connection.
    pub fn close(self) {
        info!("Closing subscription with id {}", self.id);
    }
}

impl<T: DeserializeOwned + Debug, U: Subscribe + Request> Drop for Subscription<T, U> {
    fn drop(&mut self) {
        match self.connector.call(eth_unsubscribe(self.id)) {
            Ok(true) => (),
            Ok(_) => error!("Unable to cancel subscription"),
            Err(err) => error!("{}", err),
        }
    }
}

fn deserialize_from_sub<U: DeserializeOwned + Debug>(
    response: &str,
) -> Result<U, SubscriptionError> {
    trace!("Deserializing response {}", response);
    let value = serde_json::from_str::<Value>(response)?;
    serde_json::from_value::<U>(value["params"]["result"].clone()).map_err(SubscriptionError::from)
}

/// An error type collecting what can go wrong during a subscription
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Subscription Transport Error {0}")]
    Read(#[from] TransportError),
    #[error("Subscription Error during canceling subscription: {0}")]
    Cancel(#[from] ConnectorError),
    #[error("Subscription De-/Serialization Error: {0}")]
    Serde(#[from] serde_json::Error),
}
