// use ethane::rpc;
use std::path::Path;
use test_helper::*;
use ethane::types::H256;
use std::str::FromStr;
use ethereum_types::H160;

#[test]
fn test_contract_call() {
    let mut client = ConnectorWrapper::new_from_env();
    let secret =  H256::from_str("0x31c354f57fc542eba2c56699286723e94f7bd02a4891a0a7f68566c2a2df6795").unwrap();
    let address = H160::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );
    println!("{}", contract_address);

    // let raw_contract = compile_contract(&Path::new(TEST_CONTRACT_PATH), TEST_CONTRACT_NAME);
    // let abi = abi(raw_contract);
    // let from = H160::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
    //
    // let tx_hash = client.call(ethane::contract::query(ethane::types::ContractCall{
    //     from,
    //     abi: abi.clone(),
    //     to: contract_address,
    // })).unwrap();
    // println!("{:?}", tx_hash);
    // ethane::contract::fetch_query_result(ethane::types::ContractCall{
    //     from,
    //     abi: abi.clone(),
    //     to: contract_address,
    // }, tx_hash);
    // wait_for_transaction(&mut client, tx_hash);
    // rpc_call_test_some(
    //     &mut client,
    //     rpc::eth_get_transaction_by_block_number_and_index(None, U64::zero()),
    // );

}