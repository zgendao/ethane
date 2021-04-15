use ethane::contract::{CallOpts, CallResult, Caller};
use ethane::types::{Address, H256, U256};
use ethane::{rpc, Connection, Http};
use ethane_abi::*;
use std::path::Path;
use std::str::FromStr;
use test_helper::{
    deploy_contract, wait_for_transaction, ConnectionWrapper, TEST_ERC20_NAME, TEST_ERC20_PATH,
};

const ADDRESS1: &str = "0x007ccffb7916f37f7aeef05e8096ecfbe55afc2f";
const ADDRESS2: &str = "0x99429f64cf4d5837620dcc293c1a537d58729b68";

#[test]
fn test_eth_call_contract() {
    // deploy contract
    let mut client = ConnectionWrapper::new_from_env(None);
    let address = Address::from_str(ADDRESS1).unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_ERC20_PATH),
        TEST_ERC20_NAME,
    );

    let conn = Connection::new(Http::new("http://localhost:8545", None));

    let mut caller = Caller::new_from_path(
        conn,
        "../ethane/test-helper/src/fixtures/TestERC20.abi",
        contract_address,
    );

    let result = caller.call(
        "balanceOf",
        vec![Parameter::from(Address::from(address))],
        None,
    );
    match result {
        CallResult::Transaction(_) => panic!("Should be eth_call"),
        CallResult::Call(r) => match r[0] {
            Parameter::Uint(data, 256) => assert_eq!(data, H256::from_low_u64_be(1000000000_u64)),
            _ => panic!("Invalid data received!"),
        },
    }
}

#[test]
fn test_eth_call_contract_transfer() {
    // deploy contract
    let mut client = ConnectionWrapper::new_from_env(None);
    let address = Address::from_str(ADDRESS1).unwrap();
    let to_address = Address::from_str(ADDRESS2).unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_ERC20_PATH),
        TEST_ERC20_NAME,
    );

    let conn = Connection::new(Http::new("http://localhost:8545", None));

    let mut caller = Caller::new_from_path(
        conn,
        "../ethane/test-helper/src/fixtures/TestERC20.abi",
        contract_address,
    );

    let result = caller.call(
        "transfer",
        vec![
            Parameter::from(Address::from(to_address)),
            Parameter::from(U256::from(1000)),
        ],
        Some(CallOpts {
            force_call_type: None,
            from: Some(address),
        }),
    );
    match result {
        CallResult::Call(_) => panic!("Should be a transaction"),
        CallResult::Transaction(tx_hash) => {
            wait_for_transaction(&mut client, tx_hash);
            let result = caller.call(
                "balanceOf",
                vec![Parameter::from(Address::from(to_address))],
                None,
            );
            match result {
                CallResult::Transaction(_) => panic!("Should be eth_call"),
                CallResult::Call(r) => match r[0] {
                    Parameter::Uint(data, 256) => assert_eq!(data, H256::from_low_u64_be(1000_u64)),
                    _ => panic!("Invalid data received!"),
                },
            }
        }
    }
}
