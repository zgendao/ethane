use ethane::rpc;
use ethane::types::{BlockParameter, Bytes, Call, GasCall, TransactionRequest, H256, U256, U64};
use std::path::Path;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};

pub mod helper;
use helper::*;

#[test]
fn test_eth_protocol_version() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_protocol_version());
}

#[test]
fn test_eth_syncing() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_syncing());
}

#[test]
fn test_eth_coinbase() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_coinbase());
}

#[test]
fn test_eth_mining() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_mining());
}

#[test]
fn test_eth_hashrate() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_hashrate());
}

#[test]
fn test_eth_gas_price() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_gas_price());
}

#[test]
fn test_eth_accounts() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_accounts());
}

#[test]
fn test_eth_block_number() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_block_number());
}

#[test]
fn test_eth_get_balance() {
    let mut client = Client::ws();
    let (_secret, address) = create_account(&mut client);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_balance(address, None),
        U256::exp10(20),
    );
}

#[test]
fn test_eth_send_transaction_to_address() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    rpc_call_test_some(&mut client, rpc::eth_send_transaction(transaction));
}

#[test]
fn test_eth_send_transaction_contract_creation() {
    let mut client = Client::ws();
    let bin = bin(compile_contract(
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    ));
    let contract_bytes = Bytes::from_str(&bin).unwrap();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        data: Some(contract_bytes),
        ..Default::default()
    };
    rpc_call_test_some(&mut client, rpc::eth_send_transaction(transaction));
}

#[test]
fn test_eth_get_transaction_by_hash() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_transaction_by_hash(transaction_hash),
    );
}

#[test]
fn test_eth_get_transaction_receipt() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_transaction_receipt(transaction_hash),
    );
}

#[test]
fn test_eth_get_storage_at() {
    let mut client = Client::ws();
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_storage_at(contract_address, U256::zero(), None),
        Bytes({
            let mut inner = vec![0; 32];
            inner[31] = 11;
            inner
        }),
    );
}

#[test]
fn test_eth_get_transaction_count() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash_1 = client
        .call(rpc::eth_send_transaction(transaction.clone()))
        .unwrap();
    let transaction_hash_2 = client
        .call(rpc::eth_send_transaction(transaction.clone()))
        .unwrap();
    let transaction_hash_3 = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, transaction_hash_1);
    wait_for_transaction(&mut client, transaction_hash_2);
    wait_for_transaction(&mut client, transaction_hash_3);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_transaction_count(sender, None),
        U256::from(3),
    );
}

#[test]
fn test_eth_get_block_by_number_full_tx() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_block_by_number(None, true));
}

#[test]
fn test_eth_get_block_by_number_only_hashes() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_block_by_number(None, false));
}

#[test]
fn test_eth_get_block_by_number_no_block() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_by_number(Some(BlockParameter::Custom(U64::from(120))), false),
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_hash() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_transaction_count_by_hash(block.unwrap().hash.unwrap()),
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_number() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_transaction_count_by_number(None),
    );
}

#[test]
fn test_eth_get_uncle_count_by_block_hash() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_uncle_count_by_block_hash(block.unwrap().hash.unwrap()),
    )
}

#[test]
fn test_eth_get_uncle_count_by_block_number() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_uncle_count_by_block_number(None));
}

#[test]
fn test_eth_get_code_missing() {
    let mut client = Client::ws();
    let (_, address) = create_account(&mut client);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_code(address, None),
        Bytes::from_str("0x").unwrap(),
    )
}

#[test]
fn test_eth_get_code_contract() {
    let mut client = Client::ws();
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    rpc_call_test_some(&mut client, rpc::eth_get_code(contract_address, None));
}

#[test]
fn test_eth_sign() {
    let mut client = Client::ws();
    let address = import_account(&mut client, H256::from_str(FIX_SECRET).unwrap());
    let message = Bytes::from_slice("checkmate".as_bytes());
    let expected_signature = Bytes::from_str(
        "67e4a4cf3b8cfb7d9a568482e9b6deb6350bc7701ae0448b92752b463e7dc97\
        c09c424607fbcf1cb4f6ec1c6a6c60a3527dcfe11412a3bff26218ca9f0bdef9d1b",
    )
    .unwrap();
    client
        .call(rpc::personal_unlock_account(
            address,
            String::from(ACCOUNTS_PASSWORD),
            None,
        ))
        .unwrap();

    rpc_call_test_expected(
        &mut client,
        rpc::eth_sign(address, message),
        expected_signature,
    );
}

// TODO: This test currently fails. However I am not sure if this is a short coming
// TODO: of the test case or geth. Compare https://github.com/ethereum/go-ethereum/issues/22223
#[test]
#[ignore]
fn test_eth_sign_transaction() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    rpc_call_test_some(&mut client, rpc::eth_sign_transaction(transaction));
}

// TODO: Geth returns something like: {raw: H160, tx: Transaction}, however according to JSON RPC
// TODO: it should return only the transaction hash
#[test]
#[ignore]
fn test_eth_send_raw_transaction() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        gas: Some(U256::exp10(12)),
        gas_price: Some(U256::exp10(3)),
        value: Some(U256::zero()),
        nonce: Some(U256::zero()),
        ..Default::default()
    };
    let raw_tx = client.call(rpc::eth_sign_transaction(transaction)).unwrap();
    rpc_call_test_some(&mut client, rpc::eth_send_raw_transaction(raw_tx));
}

#[test]
fn test_eth_call() {
    let mut client = Client::ws();
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );

    let mut hasher = Keccak::v256();
    hasher.update(b"solution()");
    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    let call = Call {
        to: contract_address,
        data: Some(Bytes::from_slice(&out[..4])),
        ..Default::default()
    };
    let mut expected = [0u8; 32];
    expected[31] = 42;

    rpc_call_test_expected(
        &mut client,
        rpc::eth_call(call, None),
        Bytes::from_slice(&expected),
    );
}

#[test]
fn test_eth_estimate_gas() {
    let mut client = Client::ws();
    let gas_call = GasCall {
        from: Some(create_account(&mut client).1),
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    rpc_call_test_expected(
        &mut client,
        rpc::eth_estimate_gas(gas_call, None),
        U256::from(21000),
    );
}

#[test]
fn test_eth_get_block_by_hash() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_by_hash(block.unwrap().hash.unwrap(), true),
    );
}

#[test]
fn test_eth_get_transaction_by_block_hash_and_index() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_transaction_by_block_hash_and_index(block.unwrap().hash.unwrap(), U64::zero()),
    );
}

#[test]
fn test_eth_get_transaction_by_block_number_and_index() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_transaction_by_block_number_and_index(None, U64::zero()),
    );
}

#[test]
fn test_eth_get_uncle_by_block_hash_and_index() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_by_block_hash_and_index(block.unwrap().hash.unwrap(), U64::zero()),
        None,
    );
}

#[test]
fn test_eth_get_uncle_by_block_number_and_index() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_by_block_number_and_index(None, U64::zero()),
        None,
    );
}

// TODO: This test fails with geth because the method is not available, although it should according to JSON RPC spec
#[test]
#[ignore]
fn test_eth_get_compilers() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_get_compilers());
}