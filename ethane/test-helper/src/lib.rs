use ethane::rpc::{self, Rpc};
use ethane::types::{Bytes, PrivateKey, TransactionRequest, H160, H256, U256};

use rand::Rng;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};

mod spin_up;
pub use spin_up::{ConnectorNodeBundle, ConnectorWrapper, NodeProcess};

mod fixtures;
use ethane::ConnectorError;
pub use fixtures::*;

pub fn wait_for_transaction(client: &mut ConnectorWrapper, tx_hash: H256) {
    loop {
        let transaction = client
            .call(rpc::eth_get_transaction_by_hash(tx_hash))
            .unwrap();
        if transaction.block_hash.is_some() {
            break;
        }
    }
}

pub fn create_secret() -> H256 {
    const HEX_CHARSET: &[u8] = b"abcdef0123456789";
    const PK_LEN: usize = 64;
    let mut rng = rand::thread_rng();

    let secret: String = (0..PK_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..HEX_CHARSET.len());
            HEX_CHARSET[idx] as char
        })
        .collect();
    H256::from_str(&secret).unwrap()
}

pub fn import_account(client: &mut ConnectorWrapper, secret: H256) -> Result<H160, ConnectorError> {
    client.call(rpc::personal_import_raw_key(
        PrivateKey::NonPrefixed(secret),
        String::from(ACCOUNTS_PASSWORD),
    ))
}

pub fn unlock_account(client: &mut ConnectorWrapper, address: H160) -> bool {
    client
        .call(rpc::personal_unlock_account(
            address,
            String::from(ACCOUNTS_PASSWORD),
            None,
        ))
        .unwrap()
}

pub fn prefund_account(client: &mut ConnectorWrapper, address: H160) -> H256 {
    let coinbase = client.call(rpc::eth_coinbase()).unwrap();
    let tx = TransactionRequest {
        from: coinbase,
        to: Some(address),
        value: Some(U256::exp10(20)),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(client, tx_hash);
    tx_hash
}

pub fn create_account(client: &mut ConnectorWrapper) -> (H256, H160) {
    let secret = create_secret();
    let address = import_account(client, secret).unwrap();
    unlock_account(client, address);
    prefund_account(client, address);
    (secret, address)
}

pub fn compile_contract(path: &Path, contract_name: &str) -> Value {
    let path_as_str = path.to_str().unwrap();
    let output = Command::new("solc")
        .args(&[path_as_str, "--optimize", "--combined-json", "abi,bin"])
        .output()
        .expect("Failed to compile contract. Is Solidity compiler solc installed?")
        .stdout;
    let output: Value =
        serde_json::from_slice(output.as_slice()).expect("Failed to deserialize compiled contract");
    output["contracts"][String::from(path_as_str) + ":" + contract_name].clone()
}

pub fn deploy_contract(
    client: &mut ConnectorWrapper,
    address: H160,
    path: &Path,
    contract_name: &str,
) -> (H160, Value) {
    let raw_contract = compile_contract(path, contract_name);
    let bin = bin(raw_contract.clone());
    let abi = abi(raw_contract);
    let contract_bytes = Bytes::from_str(&bin).unwrap();
    let transaction = TransactionRequest {
        from: address,
        data: Some(contract_bytes),
        gas: Some(U256::from(1000000 as u64)),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(client, transaction_hash);

    let receipt = client
        .call(rpc::eth_get_transaction_receipt(transaction_hash))
        .unwrap()
        .unwrap();
    let contract_address = receipt.contract_address.unwrap();
    (contract_address, abi)
}

pub fn simulate_transaction(
    client: &mut ConnectorWrapper,
    from: H160,
    to: &str,
    value: U256,
) -> H256 {
    let transaction = TransactionRequest {
        from,
        to: Some(to.parse().unwrap()),
        value: Some(value),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(client, tx_hash);
    tx_hash
}

pub fn bin(contract_input: Value) -> String {
    contract_input["bin"].as_str().unwrap().to_string()
}

pub fn abi(contract_input: Value) -> Value {
    contract_input["abi"].clone()
}

pub fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(input);
    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    out
}

pub fn rpc_call_test_expected<T: DeserializeOwned + Debug + PartialEq>(
    client: &mut ConnectorWrapper,
    rpc: Rpc<T>,
    expected: T,
) {
    match client.call(rpc) {
        Ok(res) => {
            assert_eq!(res, expected);
        }
        Err(err) => panic!("{:?}", err),
    }
}

pub fn rpc_call_test_some<T: DeserializeOwned + Debug + PartialEq>(
    client: &mut ConnectorWrapper,
    rpc: Rpc<T>,
) {
    match client.call(rpc) {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => panic!("{:?}", err),
    }
}

pub fn rpc_call_with_return<T: DeserializeOwned + Debug + PartialEq>(
    client: &mut ConnectorWrapper,
    rpc: Rpc<T>,
) -> T {
    match client.call(rpc) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    }
}
