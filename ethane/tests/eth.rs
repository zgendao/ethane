use ethane::rpc;
use ethane::types::{
    Address, BlockParameter, Bytes, Call, Filter, GasCall, SyncInfo, TransactionRequest,
    ValueOrVec, H256, U256, U64,
};
use std::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;

use test_helper::*;

const ADDRESS1: &str = "0x007ccffb7916f37f7aeef05e8096ecfbe55afc2f";
const ADDRESS2: &str = "0x99429f64cf4d5837620dcc293c1a537d58729b68";
const ADDRESS3: &str = "0xca247d7425a29c6645fa991f9151f994a830882d";

#[test]
fn test_eth_protocol_version() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_protocol_version(),
        String::from("0x41"),
    );
}

#[test]
fn test_eth_syncing() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::eth_syncing(), SyncInfo::NotSyncing(false));
}

#[test]
fn test_eth_coinbase() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_coinbase(),
        Address::try_from(ADDRESS1).unwrap(),
    );
}

#[test]
fn test_eth_mining() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::eth_mining(), true);
}

#[test]
fn test_eth_hashrate() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::eth_hashrate(), U256::zero());
}

#[test]
fn test_eth_gas_price() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_gas_price(),
        U256::from_int_unchecked(1_u8),
    );
}

#[test]
fn test_eth_accounts() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let accounts = rpc_call_with_return(&mut client, rpc::eth_accounts());
    assert_eq!(accounts[0], Address::try_from(ADDRESS1).unwrap());
    assert_eq!(accounts[1], Address::try_from(ADDRESS2).unwrap());
    assert_eq!(accounts[2], Address::try_from(ADDRESS3).unwrap());
}

#[test]
#[ignore]
fn test_eth_block_number() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let block_number = rpc_call_with_return(&mut client, rpc::eth_block_number());
    if !block_number.as_bytes().ge(U64::zero().as_bytes()) {
        panic!("Invalid block number, it's {:?}", block_number);
    }
}

#[test]
fn test_eth_get_balance() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let balance = rpc_call_with_return(
        &mut client,
        rpc::eth_get_balance(Address::try_from(ADDRESS1).unwrap(), None),
    );
    if !balance
        .as_bytes()
        .gt(U256::from_int_unchecked(900000000000000000_u64).as_bytes())
    {
        panic!("Invalid balance should be bigger than 900 ETH");
    }
}

#[test]
fn test_eth_send_transaction_to_address() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let value = 1000000000000000000 as u64;
    let transaction = TransactionRequest {
        from: Address::try_from(ADDRESS1).unwrap(),
        to: Some(Address::try_from(ADDRESS2).unwrap()),
        value: Some(U256::from_int_unchecked(value)),
        ..Default::default()
    };
    let tx_hash = rpc_call_with_return(&mut client, rpc::eth_send_transaction(transaction));
    wait_for_transaction(&mut client, tx_hash);

    let tx_receipt =
        rpc_call_with_return(&mut client, rpc::eth_get_transaction_receipt(tx_hash)).unwrap();
    let tx = rpc_call_with_return(&mut client, rpc::eth_get_transaction_by_hash(tx_hash));
    assert_eq!(tx.value, U256::from_int_unchecked(value));
    assert_eq!(tx_receipt.status, U64::from_int_unchecked(1_i64));
    assert_eq!(
        tx_receipt.cumulative_gas_used,
        U256::from_int_unchecked(21000_i32)
    );
    assert_eq!(tx_receipt.gas_used, U256::from_int_unchecked(21000_i32));
}

#[test]
fn test_eth_send_transaction_contract_creation() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS1).unwrap());
    let bin = bin(compile_contract(
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    ));
    let contract_bytes = Bytes::from_str(&bin).unwrap();
    let transaction = TransactionRequest {
        from: Address::try_from(ADDRESS1).unwrap(),
        data: Some(contract_bytes),
        gas: Some(U256::from_int_unchecked(1000000_u64)),
        ..Default::default()
    };
    let tx_hash = rpc_call_with_return(&mut client, rpc::eth_send_transaction(transaction));
    wait_for_transaction(&mut client, tx_hash);

    let tx_receipt =
        rpc_call_with_return(&mut client, rpc::eth_get_transaction_receipt(tx_hash)).unwrap();
    let tx = rpc_call_with_return(&mut client, rpc::eth_get_transaction_by_hash(tx_hash));
    assert_eq!(tx_receipt.status, U64::from_int_unchecked(1_i64));
    // assert_eq!(tx_receipt.cumulative_gas_used, U256::from_int_unchecked(117799_i32));
    // assert_eq!(tx_receipt.gas_used, U256::from_int_unchecked(117799_i32));
    assert_ne!(tx_receipt.contract_address, None);
    if tx.input.0.len() < 200 {
        panic!("Invalid input length: {}", tx.input.0.len());
    }
}

#[test]
fn test_eth_get_transaction_by_hash() {
    let value = 2000000000000000000 as u64;
    let mut client = ConnectionWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: Address::try_from(ADDRESS1).unwrap(),
        to: Some(Address::try_from(ADDRESS2).unwrap()),
        value: Some(U256::from_int_unchecked(value)),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    let tx = rpc_call_with_return(
        &mut client,
        rpc::eth_get_transaction_by_hash(transaction_hash),
    );
    assert_eq!(tx.value, U256::from_int_unchecked(value));
    assert_eq!(tx.from.unwrap(), Address::try_from(ADDRESS1).unwrap());
    assert_eq!(tx.to.unwrap(), Address::try_from(ADDRESS2).unwrap());
}

#[test]
fn test_eth_get_transaction_receipt() {
    let value = 1000000000000000000 as u64;
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    let transaction = TransactionRequest {
        from: Address::try_from(ADDRESS2).unwrap(),
        to: Some(Address::try_from(ADDRESS3).unwrap()),
        value: Some(U256::from_int_unchecked(value)),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let tx_receipt =
        rpc_call_with_return(&mut client, rpc::eth_get_transaction_receipt(tx_hash)).unwrap();
    assert_eq!(tx_receipt.status, U64::from_int_unchecked(1_i64));
    assert_eq!(
        tx_receipt.cumulative_gas_used,
        U256::from_int_unchecked(21000_i32)
    );
    assert_eq!(tx_receipt.gas_used, U256::from_int_unchecked(21000_i32));
    assert_eq!(tx_receipt.contract_address, None);
    assert_eq!(tx_receipt.from, Address::try_from(ADDRESS2).unwrap());
    assert_eq!(tx_receipt.to.unwrap(), Address::try_from(ADDRESS3).unwrap());
}

#[test]
#[ignore]
fn test_eth_get_storage_at() {
    // @TODO rewrite with ERC-20 and put data into the storage
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    let address = Address::try_from(ADDRESS2).unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_ERC20_PATH),
        TEST_ERC20_NAME,
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
    let mut client = ConnectionWrapper::new_from_env(None);
    let sender = create_account(&mut client).1;
    prefund_account(&mut client, sender);
    simulate_transaction(&mut client, sender, ADDRESS1, U256::zero());
    simulate_transaction(&mut client, sender, ADDRESS1, U256::zero());
    simulate_transaction(&mut client, sender, ADDRESS1, U256::zero());

    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_transaction_count(sender, None),
        U256::from_int_unchecked(3_u8),
    );
}

#[test]
#[ignore]
fn test_eth_get_block_by_number_full_tx() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block =
        rpc_call_with_return(&mut client, rpc::eth_get_block_by_number(None, true)).unwrap();

    assert_eq!(block.transactions.len(), 1);
}

#[test]
#[ignore]
fn test_eth_get_block_by_number_only_hashes() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS3,
        U256::zero(),
    );
    let block =
        rpc_call_with_return(&mut client, rpc::eth_get_block_by_number(None, false)).unwrap();

    assert_eq!(block.transactions.len(), 1);
}

#[test]
fn test_eth_get_block_by_number_no_block() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS3,
        U256::zero(),
    );
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_block_by_number(
            Some(BlockParameter::Custom(U64::from_int_unchecked(
                12000000000000_u64,
            ))),
            false,
        ),
        None,
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_hash() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_block_transaction_count_by_hash(block.unwrap().hash.unwrap()),
        U64::from_int_unchecked(1u8),
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_number() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_transaction_count_by_number(None),
    );
}

#[test]
fn test_eth_get_uncle_count_by_block_hash() {
    // @TODO it's really hard to replicate
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_count_by_block_hash(block.unwrap().hash.unwrap()),
        U64::zero(),
    )
}

#[test]
fn test_eth_get_uncle_count_by_block_number() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
        ADDRESS1,
        U256::zero(),
    );
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_count_by_block_number(None),
        U64::zero(),
    );
}

#[test]
fn test_eth_get_code_missing() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let (_, address) = create_account(&mut client);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_code(address, None),
        Bytes::from_str("0x").unwrap(),
    )
}

#[test]
fn test_eth_get_code_contract() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let address = Address::try_from(ADDRESS1).unwrap();
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
    let mut client = ConnectionWrapper::new_from_env(None);
    let address = match import_account(&mut client, H256::try_from(FIX_SECRET).unwrap()) {
        Ok(a) => a,
        Err(_) => Address::try_from("0xdc677f7c5060b0b441d30f361d0c8529ac04e099").unwrap(),
    };
    println!("{:?}", address);
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

// DEVIATION FROM SPEC
// c.f. https://github.com/ethereum/go-ethereum/issues/22223
// also geth returns something like: {raw: hex_encoded_tx, tx: json_encoded_tx}, however according to JSON RPC
// it should return only the transaction hash
//
// We decide here to use what geth currently does and not follow the spec
#[test]
fn test_eth_sign_transaction() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        gas: Some(U256::from_int_unchecked(10_u64.pow(5))),
        gas_price: Some(U256::from_int_unchecked(10_u64.pow(9))),
        value: Some(U256::zero()),
        nonce: Some(U256::zero()),
        ..Default::default()
    };
    rpc_call_test_some(&mut client, rpc::eth_sign_transaction(transaction));
}

#[test]
fn test_eth_send_raw_transaction() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        gas: Some(U256::from_int_unchecked(10_u64.pow(5))),
        gas_price: Some(U256::from_int_unchecked(10_u64.pow(9))),
        value: Some(U256::zero()),
        nonce: Some(U256::zero()),
        ..Default::default()
    };
    let raw_tx = client.call(rpc::eth_sign_transaction(transaction)).unwrap();
    rpc_call_test_some(&mut client, rpc::eth_send_raw_transaction(raw_tx.raw));
}

#[test]
fn test_eth_call() {
    let mut client = ConnectionWrapper::new_from_env(None);
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
    let mut client = ConnectionWrapper::new_from_env(None);
    let gas_call = GasCall {
        from: Some(create_account(&mut client).1),
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    rpc_call_test_expected(
        &mut client,
        rpc::eth_estimate_gas(gas_call, None),
        U256::from_int_unchecked(21000_u32),
    );
}

#[test]
fn test_eth_get_block_by_hash() {
    let mut client = ConnectionWrapper::new_from_env(None);
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS1).unwrap(),
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
    assert_eq!(
        block
            .gas_limit
            .as_bytes()
            .gt(U256::from_int_unchecked(10000_u16).as_bytes()),
        true
    );
    // assert_eq!(block.gas_used, U256::from_int_unchecked(21000_u16));
    assert_eq!(
        block
            .size
            .as_bytes()
            .gt(U256::from_int_unchecked(400_u16).as_bytes()),
        true
    );
}

#[test]
fn test_eth_get_transaction_by_block_hash_and_index() {
    let mut client = ConnectionWrapper::new_from_env(None);
    unlock_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    prefund_account(&mut client, Address::try_from(ADDRESS2).unwrap());
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS2).unwrap(),
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
    assert_eq!(tx.gas, U256::from_int_unchecked(21000_u32));
    assert_eq!(tx.gas_price, U256::from_int_unchecked(1_u8));
    assert_eq!(tx.block_hash.unwrap(), block.hash.unwrap());
    assert_eq!(tx.block_number.unwrap(), block.number.unwrap());
}

#[test]
#[ignore]
fn test_eth_get_transaction_by_block_number_and_index() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let value = 100000000_u64;
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS1).unwrap(),
        ADDRESS2,
        U256::from_int_unchecked(value),
    );

    let tx = rpc_call_with_return(
        &mut client,
        rpc::eth_get_transaction_by_block_number_and_index(None, U64::zero()),
    );
    assert_eq!(tx.gas, U256::from_int_unchecked(21000_u32));
    assert_eq!(tx.gas_price, U256::from_int_unchecked(1_u8));
    assert_eq!(tx.value, U256::from_int_unchecked(value));
}

#[test]
fn test_eth_get_uncle_by_block_hash_and_index() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let value = 100000000_u64;
    simulate_transaction(
        &mut client,
        Address::try_from(ADDRESS1).unwrap(),
        ADDRESS2,
        U256::from_int_unchecked(value),
    );

    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap()
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_uncle_by_block_hash_and_index(block.hash.unwrap(), U64::zero()),
        None,
    );
}

#[test]
fn test_eth_get_uncle_by_block_number_and_index() {
    let mut client = ConnectionWrapper::new_from_env(None);
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
    let mut client = ConnectionWrapper::new_from_env(None);
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
        topics: Some(vec![Some(ValueOrVec::Value(
            H256::try_from(&topic).unwrap(),
        ))]),
    };
    rpc_call_test_some(&mut client, rpc::eth_new_filter(filter));
}

#[test]
fn test_eth_new_block_filter() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::eth_new_block_filter());
}

#[test]
fn test_eth_new_pending_transaction_filter() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::eth_new_pending_transaction_filter());
}

#[test]
fn test_eth_uninstall_filter() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let filter_id = client.call(rpc::eth_new_block_filter()).unwrap();
    rpc_call_test_expected(&mut client, rpc::eth_uninstall_filter(filter_id), true);
}

#[test]
fn test_eth_get_filter_changes_new_filter() {
    let mut client = ConnectionWrapper::new_from_env(None);
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
        topics: Some(vec![Some(ValueOrVec::Value(
            H256::try_from(&topic).unwrap(),
        ))]),
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
    let mut client = ConnectionWrapper::new_from_env(None);
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
fn test_eth_get_filter_logs_new_filter() {
    let mut client = ConnectionWrapper::new_from_env(None);
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
        topics: Some(vec![Some(ValueOrVec::Value(
            H256::try_from(&topic).unwrap(),
        ))]),
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
    let mut client = ConnectionWrapper::new_from_env(None);
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
#[ignore] // @TODO
fn test_eth_get_logs() {
    let mut client = ConnectionWrapper::new_from_env(None);
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
        topics: Some(vec![Some(ValueOrVec::Value(
            H256::try_from(&topic).unwrap(),
        ))]),
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
