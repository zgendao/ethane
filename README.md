<p align="center">
    <img src="/web/assets/ethane.png" alt="Ethane Logo" width="400" /> 
</p>

[![Stake to support us](https://badge.devprotocol.xyz/0xE25F166Ae42a8c08b5B18fc2Ce1EEE2Db4911604/descriptive)](https://stakes.social/0xE25F166Ae42a8c08b5B18fc2Ce1EEE2Db4911604)
[![Latest Version](https://img.shields.io/crates/v/ethane.svg)](https://crates.io/crates/ethane)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31.0+-green.svg)

## Story

Package originally created by [thojest](https://github.com/thojest) and later maintained by ZGEN DAO.
Created on purpose to make the web3 connection much simpler when using Rust.
This creates a simple alternative to other web3 packages in Rust.

## Description

Ethane is an alternative web3 implementation with the aim of being slim and simple.
It does not depend on futures or any executors. It currently supports http and
websockets (both plain and TLS) and inter process communication via Unix domain sockets (Unix only). For
http and websockets it also supports Http Basic and Bearer Authentication. Also it has a built-in ABI parser library. 
It's hidden under the contract functionalities but also it can be used alongside with the main crate.

Please also have a look at the [documentation](https://docs.rs/ethane).
If you just want to use this crate, it is also available on crates.io
([Ethane](https://crates.io/crates/ethane)). If you find any bugs please
do not hesitate to open an issue.

## Usage

Guidelines to use the Ethane library.

### Connection

Everything starts with a connection.

```rust
use ethane::{Connection, Http, WebSocket};

fn main() {
    let conn = Connection::new(Http::new("http://localhost:8545", None));
    // or
    let conn = Connection::new(WebSocket::new("ws://localhost:8546", None));
}
```

### Methods

After, we created the connection to the Ethereum Network we can call several methods. 
More details on the supported methods later.

```rust
use ethane::{Connection, Http};
use ethane::types::Address;

fn main() {
    let conn = Connection::new(Http::new("http://localhost:8545", None));
    
    match conn.call(rpc::eth_get_balance(Address::from_str(ADDRESS1).unwrap(), None)) {
        Ok(res) => res,
        Err(err) => println!("{:?}", err),
    }
}
```

### Contract call

The library supports contract calls as well via `ethane-abi`.

```rust
use ethane::{Connection, Http};
use ethane::contract::{CallOpts, CallResult, Caller};
use ethane::types::Address;
use ethane_abi::Parameter;

fn main() {
    let conn = Connection::new(Http::new("http://localhost:8545", None));

    let mut caller = Caller::new_from_path(
        conn,
        "path/to/contract.abi",
        Address::from_str("0x141770c471a64bcde74c587e55a1ffd9a1bffd31").unwrap(),
    );

    let result = caller.call("balanceOf", vec![Parameter::Address(address)], None);
    match result {
        CallResult::Transaction(_) => panic!("Should be eth_call"),
        CallResult::Call(r) => {
            assert_eq!(r[0].get_type(), ParameterType::Uint(256));
            assert_eq!(r[0].to_u256().unwrap(), U256::from(1000000000));
        }
    }
}
```

### Subscribe

```rust
// @TODO
```


### Starting a subscription over websocket
```rust
// @TODO
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

## Contribution

Issues and PRs are warmly welcomed. 
Development follows the [OSS standard](https://github.com/PumpkinSeed/oss-standard).