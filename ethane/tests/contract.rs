use test_helper::{ConnectorWrapper, deploy_contract, TEST_ERC20_PATH, TEST_ERC20_NAME};
use ethane::types::{H160, Call, Bytes};
use ethane::rpc;
use std::str::FromStr;
use std::path::Path;
use ethane_abi::*;

#[test]
fn test_eth_call_contract() {
    // deploy contract
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = H160::from_str("0x007ccffb7916f37f7aeef05e8096ecfbe55afc2f").unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_ERC20_PATH),
        TEST_ERC20_NAME,
    );
    // let contract_address = H160::from_str("0xea23602c4de23d94332567ab172dcda778d129b3").unwrap();
    // parse contract
    let mut abi = Abi::new();
    abi.parse(Path::new("../ethane/test-helper/src/fixtures/TestERC20.abi")).expect("unable to parse abi");

    let test_hash = abi.keccak_hash("balanceOf",vec![Parameter::Address(address)]).unwrap();
    println!("{:X?}",test_hash);

    let call = Call {
        to: contract_address,
        data: Some(Bytes::from_slice(&test_hash)),
        ..Default::default()
    };
    let res = client.call(rpc::eth_call(call,None));
    println!("{:?}",res)
}

#[test]
fn test_eth_call_contract_decimals() {
    // deploy contract
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = H160::from_str("0x007ccffb7916f37f7aeef05e8096ecfbe55afc2f").unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_ERC20_PATH),
        TEST_ERC20_NAME,
    );
    // let contract_address = H160::from_str("0xea23602c4de23d94332567ab172dcda778d129b3").unwrap();
    // parse contract
    let mut abi = Abi::new();
    abi.parse(Path::new("../ethane/test-helper/src/fixtures/TestERC20.abi")).expect("unable to parse abi");

    let mut test_hash = abi.keccak_hash("decimals",vec![]).unwrap();
    let call = Call {
        to: contract_address,
        data: Some(Bytes::from_slice(&test_hash)),
        ..Default::default()
    };
    let res = client.call(rpc::eth_call(call,None)).unwrap();
    println!("{:?}",res)
}
