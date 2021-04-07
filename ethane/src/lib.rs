//! Ethane is an alternative web3 implementation with the aim of being slim and simple.
//! It does not depend on futures or any executors. It currently supports http and
//! websockets (both plain and TLS) and inter process communication via Unix domain sockets (Unix only). For
//! http and websockets it also supports Http Basic and Bearer Authentication.
//!
//! Currently only Eth1 is supported and not all namespaces are implemented. You can have a look here
//! ([supported RPCs](crate::rpc)) to see what is supported. Please note that the
//! [JSON spec](https://eth.wiki/json-rpc/API) is a bit outdated, and there is some effort to create
//! a new one, so expect some breaking changes in the future.
//!
//! **This library is very raw and under heavy development.
//! Expect to find some bugs and use at your own risk!**
//!
//! In order to get started, create a [connector](crate::Connector) over some transport.
//! The following examples show you how to make a request and how to subscribe to events.
//!
//! # Examples
//!
//! ## Request over http
//! ```no_run
//! use ethane::Connector;
//! use ethane::rpc::eth_get_balance;
//! use ethane::types::Address;
//! # use test_helper::NodeProcess;
//! # let node = NodeProcess::new_http("8545");
//!
//! // Start up connector
//! let node_endpoint = "http://127.0.0.1:8545";
//! let mut connector = Connector::http(node_endpoint, None).unwrap();
//!
//! // Make a request
//! let address = Address::zero();
//! let balance = connector.call(eth_get_balance(address, None)).unwrap();
//! ```
//!
//! ## Starting a subscription over websocket
//! ```no_run
//! use ethane::Connector;
//! use ethane::rpc::sub::eth_subscribe_new_pending_transactions;
//! # use test_helper::NodeProcess;
//! # use ethane::rpc::{eth_send_transaction, eth_coinbase};
//! # use ethane::types::{TransactionRequest, Address, U256};
//!
//! # let node = NodeProcess::new_ws("8546");
//!
//! // Start up connector with websockets
//! let node_endpoint = "ws://127.0.0.1:8546";
//! let mut connector = Connector::websocket(node_endpoint, None).unwrap();
//!
//! // Subscribe to pending transactions
//! let mut tx_subscription = connector
//!     .subscribe(eth_subscribe_new_pending_transactions()).unwrap();
//! # let tx_request = TransactionRequest {
//! # from: connector.call(eth_coinbase()).unwrap(),
//! # to: Some(Address::zero()),
//! # value: Some(U256::zero()),
//! # ..Default::default()
//! # };
//! # let _tx_hash = connector.call(eth_send_transaction(tx_request));
//!
//! // Get next transaction item
//! let tx = tx_subscription.next_item().unwrap();
//! ```

pub use connector::subscription::{Subscription, SubscriptionError};
pub use connector::{Connector, ConnectorError};
pub use transport::{http::Http, websocket::WebSocket, Credentials};

#[cfg(target_family = "unix")]
pub use transport::uds::Uds;

mod connection;
pub mod connector;
//pub mod contract;
pub mod rpc;
pub mod transport;
pub mod types;
