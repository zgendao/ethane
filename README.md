<p align="center">
    <img src="/web/assets/ethane.png" alt="Ethane Logo" width="400" /> 
</p>

[![Stake to support us](https://badge.devprotocol.xyz/0xE25F166Ae42a8c08b5B18fc2Ce1EEE2Db4911604/descriptive)](https://stakes.social/0xE25F166Ae42a8c08b5B18fc2Ce1EEE2Db4911604)
[![Latest Version](https://img.shields.io/crates/v/ethane.svg)](https://crates.io/crates/ethane)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31.0+-green.svg)

## Story

Package originally created by [thojest](https://github.com/thojest) and later maintained by ZGEN DAO.
Created with the purpose to provide a simple alternative for `Web3` packages written in Rust.

## Description

Ethane is an alternative `Web3` implementation with the aim of being slim and
simple. It has two features `blocking` and `non-blocking` that determines the
http connection type it should use.

A blocking http client does not depend on futures or any executors. Furthermore, it currently supports
websockets (both plain and TLS) and inter process communication via Unix domain sockets (Unix only). For
http and websockets it also supports Http Basic and Bearer Authentication. It also has a built-in ABI parser library.
It's hidden under the contract functionalities, but it can be used alongside with the main crate.

If you still need a non-blocking http client (e.g. for `wasm` compatibility),
you may compile `Ethane` with the `non-blocking` feature flag to enable an
`async` http client.

Please also take a look at the [documentation](https://docs.rs/ethane).
If you just want to use this crate, it is also available on crates.io.
([Ethane](https://crates.io/crates/ethane)). If you find any bugs please
do not hesitate to open an issue.

## Usage

Guidelines to use the Ethane library. The examples were worked out for the
blocking client, however the non-blocking version is quite similar. The main
difference is that an `AsyncConnection` can only wrap an `AsyncHttp` client without any
type generics, so there is currently no implementation for a non-blocking websocket.

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

After we have connected to the Ethereum Network we can call several methods.
More details on the supported methods later.

```rust
use ethane::{Connection, Http};
use ethane::types::Address;

fn main() {
    let conn = Connection::new(Http::new("http://localhost:8545", None));
    
    match conn.call(rpc::eth_get_balance(Address::try_from_str(ADDRESS1).unwrap(), None)) {
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
use ethane::types::{Address, U256};
use ethane_abi::Parameter;

fn main() {
    let conn = Connection::new(Http::new("http://localhost:8545", None));

    let mut caller = Caller::new_from_path(
        conn,
        "path/to/contract.abi",
        Address::try_from_str("0x141770c471a64bcde74c587e55a1ffd9a1bffd31").unwrap(),
    );

    // The call function determine the call_type based on the state_mutability.
    // This calls to function from an ERC-20 compliant token
    // eth_call
    let address = Address::try_from_str("0x141770c471a64bcde74c587e55a1ffd9a1bffd31").uwnrap();
    let result = caller.call(
        "balanceOf",
        vec![Parameter::from(address)],
        None,
    );
    match result {
        CallResult::Transaction(_) => panic!("Should be eth_call"),
        CallResult::Call(r) => match r[0] {
            Parameter::Uint(data, 256) => assert_eq!(data, H256::from_int_unchecked(1000000000_u64)),
            _ => panic!("Invalid data received!"),
        },
    }

    // eth_sendTransaction
    let to_address = Address::try_from_str("0x...").unwrap();
    let result = caller.call(
        "transfer",
        vec![
            Parameter::from(to_address),
            Parameter::from(U256::try_from_int(1000_u128).unwrap()),
        ],
        Some(CallOpts {
            force_call_type: None, // NOTE: the call_type can be forced
            from: Some(address),
        }),
    );
    match result {
        CallResult::Call(_) => panic!("Should be a transaction"),
        CallResult::Transaction(tx_hash) => println!("{}", tx_hash),
    }
}
```

### Subscribe

Subscription has a different connection method.

```rust
use ethane::{Connection, WebSocket};

fn main() {
    let conn = Connection::new(WebSocket::new("ws://localhost:8546", None));
    let mut tx_subscription = conn.subscribe(eth_subscribe_new_pending_transactions()).unwrap();

    // Get next transaction item
    let tx = tx_subscription.next_item().unwrap();
}
```

## Contribution

Issues and PRs are warmly welcomed. 
Development follows the [OSS standard](https://github.com/PumpkinSeed/oss-standard).
