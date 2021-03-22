use ethane::rpc;
use ethane::types::{
    BlockParameter, Bytes, Call, Filter, GasCall, TransactionRequest, ValueOrVec, H256, U256, U64,
};
use std::path::Path;
use std::str::FromStr;

use ethereum_types::H160;
use test_helper::*;

const ADDRESS1: &str = "0x95eDA452256C1190947f9ba1fD19422f0120858a";
const ADDRESS2: &str = "0x1A4C0439ba035DAcf0D573394107597CEEBF9FF8";
const ADDRESS3: &str = "0x5354fcfeB16E8B36FcE591d8A9fc44aAD81c7ca6";

#[test]
fn test_eth_protocol_version() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::eth_protocol_version(), String::from("63"));
}

#[test]
fn test_eth_syncing() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::eth_syncing());
}

#[test]
fn test_eth_coinbase() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_coinbase(),
        H160::from_str(ADDRESS1).unwrap(),
    );
}

#[test]
fn test_eth_mining() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::eth_mining(), true);
}

#[test]
fn test_eth_hashrate() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::eth_hashrate(), U256::from(0));
}

#[test]
fn test_eth_gas_price() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_gas_price(),
        U256::from(20000000000 as u64),
    );
}

#[test]
fn test_eth_accounts() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let accounts = rpc_call_with_return(&mut client, rpc::eth_accounts());
    assert_eq!(accounts[0], H160::from_str(ADDRESS1).unwrap());
    assert_eq!(accounts[1], H160::from_str(ADDRESS2).unwrap());
    assert_eq!(accounts[2], H160::from_str(ADDRESS3).unwrap());
}

#[test]
fn test_eth_block_number() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let block_number = rpc_call_with_return(&mut client, rpc::eth_block_number());
    if !block_number.gt(&U64::from(12000000)) {
        panic!("Invalid block number");
    }
}

#[test]
fn test_eth_get_balance() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let balance = rpc_call_with_return(
        &mut client,
        rpc::eth_get_balance(H160::from_str(ADDRESS3).unwrap(), None),
    );
    if !balance.gt(&U256::from(900000000000000000 as u64)) {
        panic!("Invalid balance should be bigger than 900 ETH");
    }
}

#[test]
fn test_eth_send_transaction_to_address() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let value = 1000000000000000000 as u64;
    let transaction = TransactionRequest {
        from: H160::from_str(ADDRESS1).unwrap(),
        to: Some(H160::from_str(ADDRESS2).unwrap()),
        value: Some(U256::from(value)),
        ..Default::default()
    };
    let tx_hash = rpc_call_with_return(&mut client, rpc::eth_send_transaction(transaction));
    wait_for_transaction(&mut client, tx_hash);

    let tx_receipt =
        rpc_call_with_return(&mut client, rpc::eth_get_transaction_receipt(tx_hash)).unwrap();
    let tx = rpc_call_with_return(&mut client, rpc::eth_get_transaction_by_hash(tx_hash));
    assert_eq!(tx.value, U256::from(value));
    assert_eq!(tx_receipt.status, U64::from(1 as i64));
    assert_eq!(tx_receipt.cumulative_gas_used, U256::from(21000 as i32));
    assert_eq!(tx_receipt.gas_used, U256::from(21000 as i32));
}

#[test]
fn test_eth_send_transaction_contract_creation() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let bin = bin(compile_contract(
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    ));
    let contract_bytes = Bytes::from_str(&bin).unwrap();
    let transaction = TransactionRequest {
        from: ADDRESS2.parse().unwrap(),
        data: Some(contract_bytes),
        gas: Some(U256::from(1000000 as u64)),
        ..Default::default()
    };
    let tx_hash = rpc_call_with_return(&mut client, rpc::eth_send_transaction(transaction));
    wait_for_transaction(&mut client, tx_hash);

    let tx_receipt =
        rpc_call_with_return(&mut client, rpc::eth_get_transaction_receipt(tx_hash)).unwrap();
    let tx = rpc_call_with_return(&mut client, rpc::eth_get_transaction_by_hash(tx_hash));
    assert_eq!(tx_receipt.status, U64::from(1 as i64));
    // assert_eq!(tx_receipt.cumulative_gas_used, U256::from(117799 as i32));
    // assert_eq!(tx_receipt.gas_used, U256::from(117799 as i32));
    assert_ne!(tx_receipt.contract_address, None);
    if tx.input.0.len() < 200 {
        panic!("Invalid input length: {}", tx.input.0.len());
    }
}

#[test]
fn test_eth_get_transaction_by_hash() {
    let value = 1000000000000000000 as u64;
    let mut client = ConnectorWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: ADDRESS1.parse().unwrap(),
        to: Some(ADDRESS2.parse().unwrap()),
        value: Some(U256::from(value)),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    let tx = rpc_call_with_return(
        &mut client,
        rpc::eth_get_transaction_by_hash(transaction_hash),
    );
    assert_eq!(tx.value, U256::from(value));
    assert_eq!(tx.from.unwrap(), H160::from_str(ADDRESS1).unwrap());
    assert_eq!(tx.to.unwrap(), H160::from_str(ADDRESS2).unwrap());
}

#[test]
fn test_eth_get_transaction_receipt() {
    let value = 1000000000000000000 as u64;
    let mut client = ConnectorWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: ADDRESS2.parse().unwrap(),
        to: Some(ADDRESS3.parse().unwrap()),
        value: Some(U256::from(value)),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    let tx_receipt = rpc_call_with_return(
        &mut client,
        rpc::eth_get_transaction_receipt(transaction_hash),
    )
    .unwrap();
    assert_eq!(tx_receipt.status, U64::from(1 as i64));
    // assert_eq!(tx_receipt.cumulative_gas_used, U256::from(21000 as i32));
    // assert_eq!(tx_receipt.gas_used, U256::from(21000 as i32));
    assert_eq!(tx_receipt.contract_address, None);
    assert_eq!(tx_receipt.from, H160::from_str(ADDRESS2).unwrap());
    assert_eq!(tx_receipt.to.unwrap(), H160::from_str(ADDRESS3).unwrap());
}

#[test]
#[ignore]
fn test_eth_get_storage_at() {
    // @TODO rewrite with ERC-20 and put data into the storage
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = H160::from_str(ADDRESS2).unwrap();
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
    let mut client = ConnectorWrapper::new_from_env(None);
    let sender = create_account(&mut client).1;
    simulate_transaction(&mut client, sender, ADDRESS1, U256::zero());
    simulate_transaction(&mut client, sender, ADDRESS1, U256::zero());
    simulate_transaction(&mut client, sender, ADDRESS1, U256::zero());

    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_transaction_count(sender, None),
        U256::from(3),
    );
}

#[test]
fn test_eth_get_block_by_number_full_tx() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block =
        rpc_call_with_return(&mut client, rpc::eth_get_block_by_number(None, true)).unwrap();

    assert_eq!(block.transactions.len(), 1);
}

#[test]
fn test_eth_get_block_by_number_only_hashes() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS3,
        U256::zero(),
    );
    let block =
        rpc_call_with_return(&mut client, rpc::eth_get_block_by_number(None, false)).unwrap();

    assert_eq!(block.transactions.len(), 1);
}

#[test]
fn test_eth_get_block_by_number_no_block() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS3,
        U256::zero(),
    );
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_block_by_number(
            Some(BlockParameter::Custom(U64::from(12000000000000 as u64))),
            false,
        ),
        None,
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_hash() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_block_transaction_count_by_hash(block.unwrap().hash.unwrap()),
        U64::from(1),
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_number() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_block_transaction_count_by_number(None),
        U64::from(1),
    );
}

#[test]
fn test_eth_get_uncle_count_by_block_hash() {
    // @TODO it's really hard to replicate
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_count_by_block_hash(block.unwrap().hash.unwrap()),
        U64::from(0),
    )
}

#[test]
fn test_eth_get_uncle_count_by_block_number() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS2.parse().unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_count_by_block_number(None),
        U64::from(0),
    );
}

#[test]
fn test_eth_get_code_missing() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let (_, address) = create_account(&mut client);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_code(address, None),
        Bytes::from_str("0x").unwrap(),
    )
}

#[test]
fn test_eth_get_code_contract() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = ADDRESS1.parse().unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    let code = rpc_call_with_return(&mut client, rpc::eth_get_code(contract_address, None));

    if code.0.len() < 100 {
        panic!("Invalid code length: {}", code.0.len());
    }
}

#[test]
fn test_eth_sign() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = import_account(&mut client, H256::from_str(FIX_SECRET).unwrap());
    let message = Bytes::from_slice("checkmate".as_bytes());
    let expected_signature = Bytes::from_str(
        "67e4a4cf3b8cfb7d9a568482e9b6deb6350bc7701ae0448b92752b463e7dc97\
        c09c424607fbcf1cb4f6ec1c6a6c60a3527dcfe11412a3bff26218ca9f0bdef9d00",
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

// DEVIATION FROM SPEC
// c.f. https://github.com/ethereum/go-ethereum/issues/22223
// also geth returns something like: {raw: hex_encoded_tx, tx: json_encoded_tx}, however according to JSON RPC
// it should return only the transaction hash
//
// We decide here to use what geth currently does and not follow the spec
// @TODO Not supported in Ganache, skip if test against ganache
#[test]
#[ignore]
fn test_eth_sign_transaction() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        gas: Some(U256::exp10(5)),
        gas_price: Some(U256::exp10(9)),
        value: Some(U256::zero()),
        nonce: Some(U256::zero()),
        ..Default::default()
    };
    rpc_call_test_some(&mut client, rpc::eth_sign_transaction(transaction));
}

// @TODO Not supported in Ganache, skip if test against ganache
#[test]
#[ignore]
fn test_eth_send_raw_transaction() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        gas: Some(U256::exp10(5)),
        gas_price: Some(U256::exp10(9)),
        value: Some(U256::zero()),
        nonce: Some(U256::zero()),
        ..Default::default()
    };
    let raw_tx = client.call(rpc::eth_sign_transaction(transaction)).unwrap();
    rpc_call_test_some(&mut client, rpc::eth_send_raw_transaction(raw_tx.raw));
}

#[test]
fn test_eth_call() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    let out = keccak(b"solution()");
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
    let mut client = ConnectorWrapper::new_from_env(None);
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
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS1.parse().unwrap(),
        ADDRESS3,
        U256::zero(),
    );
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    let block = rpc_call_with_return(
        &mut client,
        rpc::eth_get_block_by_hash(block.unwrap().hash.unwrap(), true),
    )
    .unwrap();
    assert_eq!(block.gas_limit, U256::from(6721975));
    assert_eq!(block.gas_used, U256::from(21000));
    assert_eq!(block.size, U256::from(1000));
}

#[test]
fn test_eth_get_transaction_by_block_hash_and_index() {
    let mut client = ConnectorWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        ADDRESS3.parse().unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap()
        .unwrap();
    let tx = rpc_call_with_return(
        &mut client,
        rpc::eth_get_transaction_by_block_hash_and_index(block.hash.unwrap(), U64::zero()),
    );
    assert_eq!(tx.gas, U256::from(90000));
    assert_eq!(tx.gas_price, U256::from(20000000000 as u64));
    assert_eq!(tx.block_hash.unwrap(), block.hash.unwrap());
    assert_eq!(tx.block_number.unwrap(), block.number.unwrap());
}

#[test]
#[ignore]
fn test_eth_get_transaction_by_block_number_and_index() {
    // @TODO "Serde(Error("missing field `parentHash`", ..."
    let mut client = ConnectorWrapper::new_from_env(None);
    let value = 100000000 as u64;
    simulate_transaction(
        &mut client,
        ADDRESS1.parse().unwrap(),
        ADDRESS2,
        U256::from(value),
    );

    let tx = rpc_call_with_return(
        &mut client,
        rpc::eth_get_transaction_by_block_number_and_index(None, U64::zero()),
    );
    assert_eq!(tx.gas, U256::from(90000));
    assert_eq!(tx.gas_price, U256::from(20000000000 as u64));
    assert_eq!(tx.value, U256::from(value));
}

#[test]
#[ignore]
fn test_eth_get_uncle_by_block_hash_and_index() {
    // @TODO fix me
    let mut client = ConnectorWrapper::new_from_env(None);
    let value = 100000000 as u64;
    simulate_transaction(
        &mut client,
        ADDRESS1.parse().unwrap(),
        ADDRESS2,
        U256::from(value),
    );

    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap()
        .unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_uncle_by_block_hash_and_index(block.hash.unwrap(), U64::zero()),
        // None,
    );
}

#[test]
#[ignore]
fn test_eth_get_uncle_by_block_number_and_index() {
    // @TODO fix me
    let mut client = ConnectorWrapper::new_from_env(None);
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

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_get_compilers() {
    assert!(false, "This RPC is not supported anymore.");
}

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_compile_lll() {
    assert!(false, "This RPC is not supported anymore.");
}

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_compile_solidity() {
    assert!(false, "This RPC is not supported anymore.");
}

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_compile_serpent() {
    assert!(false, "This RPC is not supported anymore.");
}

#[test]
fn test_eth_new_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    let topic = keccak(b"Solution(uint256)");
    let filter = Filter {
        from_block: Some(BlockParameter::Earliest),
        to_block: Some(BlockParameter::Latest),
        address: Some(ValueOrVec::Value(contract_address)),
        topics: Some(vec![Some(ValueOrVec::Value(H256::from_slice(&topic)))]),
    };
    rpc_call_test_some(&mut client, rpc::eth_new_filter(filter));
}

#[test]
fn test_eth_new_block_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::eth_new_block_filter());
}

#[test]
fn test_eth_new_pending_transaction_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::eth_new_pending_transaction_filter());
}

#[test]
fn test_eth_uninstall_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let filter_id = client.call(rpc::eth_new_block_filter()).unwrap();
    rpc_call_test_expected(&mut client, rpc::eth_uninstall_filter(filter_id), true);
}

#[test]
fn test_eth_get_filter_changes_new_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    let topic = keccak(b"Solution(uint256)");
    let filter = Filter {
        from_block: Some(BlockParameter::Earliest),
        to_block: Some(BlockParameter::Latest),
        address: Some(ValueOrVec::Value(contract_address)),
        topics: Some(vec![Some(ValueOrVec::Value(H256::from_slice(&topic)))]),
    };
    let filter_id = client.call(rpc::eth_new_filter(filter)).unwrap();
    let out = keccak(b"set_pos0()");
    let tx = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(contract_address),
        data: Some(Bytes::from_slice(&out[..4])),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_filter_changes(filter_id));
}

#[test]
fn test_eth_get_filter_changes_block_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let tx = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let filter_id = client.call(rpc::eth_new_block_filter()).unwrap();
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_filter_changes(filter_id));
}

#[test]
#[ignore]
fn test_eth_get_filter_logs_new_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    let topic = keccak(b"Solution(uint256)");
    let filter = Filter {
        from_block: Some(BlockParameter::Earliest),
        to_block: Some(BlockParameter::Latest),
        address: Some(ValueOrVec::Value(contract_address)),
        topics: Some(vec![Some(ValueOrVec::Value(H256::from_slice(&topic)))]),
    };
    let filter_id = client.call(rpc::eth_new_filter(filter)).unwrap();
    let out = keccak(b"set_pos0()");
    let tx = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(contract_address),
        data: Some(Bytes::from_slice(&out[..4])),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_filter_logs(filter_id));
}

// This does not seem to work, although this is very similar to the test eth_get_filter_changes_block_filter
// c.f. https://github.com/ethereum-oasis/eth1.x-JSON-RPC-API-standard/issues/5#issuecomment-773132429 number 5
#[test]
#[ignore]
fn test_eth_get_filter_logs_block_filter() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let tx = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let filter_id = client.call(rpc::eth_new_block_filter()).unwrap();
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_filter_logs(filter_id));
}

#[test]
#[ignore]
fn test_eth_get_logs() {
    let mut client = ConnectorWrapper::new_from_env(Some("http"));
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    let topic = keccak(b"Solution(uint256)");
    let filter = Filter {
        from_block: Some(BlockParameter::Earliest),
        to_block: Some(BlockParameter::Latest),
        address: Some(ValueOrVec::Value(contract_address)),
        topics: Some(vec![Some(ValueOrVec::Value(H256::from_slice(&topic)))]),
    };
    let out = keccak(b"set_pos0()");
    let tx = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(contract_address),
        data: Some(Bytes::from_slice(&out[..4])),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_logs(filter));
}

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_get_work() {
    assert!(false, "This RPC is not supported anymore.");
}

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_submit_work() {
    assert!(false, "This RPC is not supported anymore.");
}

// DEVIATION FROM SPEC
// Not supported by geth
#[test]
#[ignore]
fn test_eth_submit_hashrate() {
    assert!(false, "This RPC is not supported anymore.");
}
