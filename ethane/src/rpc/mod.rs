//! Functions to generate Rpcs
//!
//! These functions are a type-safe implementation of
//! - the official [JSON RPC spec](https://eth.wiki/json-rpc/API)
//! - some custom [Geth JSON namespaces](https://geth.ethereum.org/docs/rpc/server)
//!     - [real-time events](https://geth.ethereum.org/docs/rpc/pubsub)
//!     - [personal](https://geth.ethereum.org/docs/rpc/ns-personal)
//!     - [txpool](https://geth.ethereum.org/docs/rpc/ns-txpool)
//!
//! There are some deviations between what is really supported and the official specification.
//! This is why some functions are marked as deprecated. They will probably be removed.
//!
//! Use these functions to generate [Rpc](Rpc) objects and pass them to the
//! [call](crate::Connection::call) function of a [connection](crate::Connection).

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::marker::PhantomData;

pub use eth::*;
pub use net::*;
pub use personal::*;
pub(crate) use sub::eth_unsubscribe;
pub use txpool::*;
pub use web3::*;

mod eth;
mod net;
mod personal;
pub mod sub;
mod txpool;
mod web3;

/// Wrapper for the remote procedure call
///
/// This is usually not directly needed and returned by the [functions](crate::rpc) which
/// wrap the different namespaces. However, it is also possible to create custom Rpc structs.
#[repr(C)]
#[derive(Serialize, Debug)]
pub struct Rpc<T: DeserializeOwned + Debug> {
    #[serde(rename = "jsonrpc")]
    /// The version of the JSON RPC spec
    pub json_rpc: &'static str,
    /// The method which is called
    pub method: String,
    /// Arguments supplied to the method. Can be an empty Vec.
    pub params: Vec<Value>,
    /// The id for the request
    pub id: usize,
    /// Type annotation needed for the result
    #[serde(skip_serializing)]
    pub result_type: PhantomData<T>,
}

#[derive(Deserialize)]
pub struct RpcResponse<'a, T: 'a> {
    pub id: usize,
    pub jsonrpc: &'a str,
    pub result: T,
}

impl<T: DeserializeOwned + Debug> Rpc<T> {
    const JSON_RPC: &'static str = "2.0";

    pub(crate) fn new(method: &str) -> Rpc<T> {
        Rpc {
            json_rpc: Self::JSON_RPC,
            method: String::from(method),
            params: Vec::new(),
            id: Default::default(),
            result_type: PhantomData,
        }
    }

    pub(crate) fn add_param<U: Serialize + Debug>(&mut self, parameter: U) {
        if let Ok(serialized_param) = serde_json::to_value(&parameter) {
            self.params.push(serialized_param);
        }
    }
}
