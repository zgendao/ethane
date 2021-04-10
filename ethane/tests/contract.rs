use ethane::contract::Caller;
use ethane::types::{Address, Bytes, Call, U256};
use ethane::{rpc, Connection, Http};
use ethane_abi::*;
use std::path::Path;
use std::str::FromStr;
use test_helper::{deploy_contract, ConnectionWrapper, TEST_ERC20_NAME, TEST_ERC20_PATH};

const ADDRESS1: &str = "0x007ccffb7916f37f7aeef05e8096ecfbe55afc2f";

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

    let result = caller.call("balanceOf", vec![Parameter::Address(address)]);
    assert_eq!(result[0].get_type(), ParameterType::Uint(256));
    assert_eq!(result[0].to_u256().unwrap(), U256::from(1000000000));
    println!("{:?}", result[0])
}

#[test]
#[ignore]
fn test_eth_call_contract_decimals() {
    // deploy contract
    let mut client = ConnectionWrapper::new_from_env(None);
    let address = Address::from_str("0x007ccffb7916f37f7aeef05e8096ecfbe55afc2f").unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_ERC20_PATH),
        TEST_ERC20_NAME,
    );
    // let contract_address = H160::from_str("0xea23602c4de23d94332567ab172dcda778d129b3").unwrap();
    // parse contract
    let mut abi = Abi::new();
    abi.parse_file(Path::new(
        "../ethane/test-helper/src/fixtures/TestERC20.abi",
    ))
    .expect("unable to parse abi");

    // TODO there is no "decimals" function in TestERC20.abi
    let test_hash = abi
        .encode(&AbiCall {
            function_name: "decimals",
            parameters: vec![],
        })
        .unwrap();
    println!("{:X?}", test_hash);
    let call = Call {
        to: contract_address,
        data: Some(Bytes::from_slice(&test_hash)),
        ..Default::default()
    };
    let res = client.call(rpc::eth_call(call, None)).unwrap();
    println!("{:?}", res)
}
