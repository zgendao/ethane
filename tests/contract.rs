// use ethane::rpc;
use std::path::Path;
use test_helper::*;

#[test]
fn test_contract_call() {
    let mut client = ConnectorWrapper::new_from_env();
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );

    let raw_contract = compile_contract(&Path::new(TEST_CONTRACT_PATH), TEST_CONTRACT_NAME);
    let abi = abi(raw_contract);
    let from = create_account(&mut client).1;

    let tx_hash = client.call(ethane::contract::query(ethane::types::ContractCall{
        from,
        abi: abi.clone(),
        to: contract_address,
    })).unwrap();
    println!("{:?}", tx_hash);
    ethane::contract::fetch_query_result(ethane::types::ContractCall{
        from,
        abi: abi.clone(),
        to: contract_address,
    }, tx_hash);
    // wait_for_transaction(&mut client, tx_hash);
    // rpc_call_test_some(
    //     &mut client,
    //     rpc::eth_get_transaction_by_block_number_and_index(None, U64::zero()),
    // );

}