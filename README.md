<img src="/web/assets/ethane.png" alt="Ethane Logo" width="400" /> 

Package originally created by [thojest](https://github.com/thojest) and later maintained by ZGen DAO. This repository considerd as the development line of the project, and the mainline hosted under [ethane](https://github.com/thojest/ethane).

Ethane is an alternative web3 implementation with the aim of being slim and simple.
It does not depend on futures or any executors. It currently supports http and
websockets (both plain and TLS) and inter process communication via Unix domain sockets (Unix only). For
http and websockets it also supports Http Basic and Bearer Authentication.

**This library is very raw and under heavy development.
Expect to find some bugs and use at your own risk!**

Please also have a look at the [documentation](https://docs.rs/ethane).
If you just want to use this crate, it is also available on crates.io
([Ethane](https://crates.io/crates/ethane)). If you find any bugs please
do not hesitate to open an issue.

## How to use this library

In order to get started, create a connector over some transport.
The following examples show you how to make a request and how to subscribe to events.

### Request over http
```rust
use ethane::Connector;
use ethane::rpc::eth_get_balance;
use ethane::types::H160;

// Start up connector
let node_endpoint = "http://127.0.0.1:8545";
let mut connector = Connector::http(node_endpoint, None).unwrap();

// Make a request
let address = H160::zero();
let balance = connector.call(eth_get_balance(address, None)).unwrap();
```

### Starting a subscription over websocket
```rust
use ethane::Connector;
use ethane::rpc::sub::eth_subscribe_new_pending_transactions;

// Start up connector with websockets
let node_endpoint = "ws://127.0.0.1:8546";
let mut connector = Connector::websocket(node_endpoint, None).unwrap();

// Subscribe to pending transactions
let mut tx_subscription = connector
    .subscribe(eth_subscribe_new_pending_transactions()).unwrap();

// Get next transaction item
let tx = tx_subscription.next_item().unwrap();
```
