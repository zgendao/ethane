use super::{Connection, ConnectionError, Request, Subscribe};

use crate::rpc::eth_unsubscribe;
use crate::types::U128;

use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::marker::PhantomData;

/// An active subscription
///
/// Supports the [real-time events](https://geth.ethereum.org/docs/rpc/pubsub) namespace.
/// Can be created by calling [subscribe](crate::connection::Connection::subscribe).
/// In order to yield the next subscription item call [next_item](Self::next_item).
pub struct Subscription<T: Subscribe + Request, U: DeserializeOwned + Debug> {
    /// The subscription id, which is returned when subscribing
    pub id: U128,
    pub(crate) connection: Connection<T>,
    pub(crate) result_type: PhantomData<U>,
}

impl<T: Subscribe + Request, U: DeserializeOwned + Debug> Subscription<T, U> {
    /// Yields the next item of this subscription.
    pub fn next_item(&mut self) -> Result<U, ConnectionError> {
        let response = self.connection.transport.read_next()?;
        deserialize_from_sub(&response)
    }

    /// Cancel the subscription. This will first unsubscribe and then close the underlying connection.
    pub fn close(self) {
        println!("Closing subscription with id {}", self.id);
    }
}

impl<T: Subscribe + Request, U: DeserializeOwned + Debug> Drop for Subscription<T, U> {
    fn drop(&mut self) {
        match self.connection.call(eth_unsubscribe(self.id)) {
            Ok(true) => (),
            Ok(_) => println!("Unable to cancel subscription"),
            Err(err) => println!("{:?}", err),
        }
    }
}

fn deserialize_from_sub<U: DeserializeOwned + Debug>(response: &str) -> Result<U, ConnectionError> {
    let value: serde_json::Value =
        serde_json::from_str(response).map_err(|e| ConnectionError::Serde(e.to_string()))?;
    serde_json::from_value::<U>(value["params"]["result"].clone())
        .map_err(|e| ConnectionError::SubscriptionError(e.to_string()))
}
