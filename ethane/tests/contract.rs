use ethane::contract::{CallOpts, CallResult, Caller};
use ethane::types::{Address, U256};
use ethane::{Connection, Http};
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

    let result = caller.call("balanceOf", vec![Parameter::Address(address)], None);
    match result {
        CallResult::Transaction(_) => panic!("Should be eth_call"),
        CallResult::Call(r) => {
            assert_eq!(r[0].get_type(), ParameterType::Uint(256));
            assert_eq!(r[0].to_u256().unwrap(), U256::from(1000000000));
        }
    }
}

#[test]
#[ignore]
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
            Parameter::Address(to_address),
            Parameter::Uint256(U256::from(1000)),
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

            let result = caller.call("balanceOf", vec![Parameter::Address(to_address)], None);
            match result {
                CallResult::Transaction(_) => panic!("Should be eth_call"),
                CallResult::Call(r) => {
                    assert_eq!(r[0].get_type(), ParameterType::Uint(256));
                    assert_eq!(r[0].to_u256().unwrap(), U256::from(1000));
                }
            }
        }
    }
}
