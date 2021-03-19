use ethane::rpc;
use ethane::types::{TransactionRequest, U256};

use test_helper::*;

#[test]
fn test_txpool_status() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::txpool_status())
}

#[test]
fn test_txpool_content() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    for _ in 0..10 {
        let _tx_hash = client
            .call(rpc::eth_send_transaction(transaction.clone()))
            .unwrap();
    }
    rpc_call_test_some(&mut client, rpc::txpool_content());
}

#[test]
fn test_txpool_inspect() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    for _ in 0..10 {
        let _tx_hash = client
            .call(rpc::eth_send_transaction(transaction.clone()))
            .unwrap();
    }
    rpc_call_test_some(&mut client, rpc::txpool_inspect());
}
