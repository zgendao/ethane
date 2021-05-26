use ethane::rpc;
use ethane::types::{Bytes, H256};
use std::convert::TryFrom;

use test_helper::*;

#[test]
fn test_web3_client_version() {
    let mut client = ConnectionWrapper::new_from_env(None);
    rpc_call_test_expected(
        &mut client,
        rpc::web3_client_version(),
        String::from("Geth/v1.9.25-stable-e7872729/linux-amd64/go1.15.6"),
    );
}

#[test]
fn test_web3_sha3() {
    let mut client = ConnectionWrapper::new_from_env(None);
    let empty = Bytes::from_slice("".as_bytes());
    let expected = H256::try_from(KECCAK_HASH_OF_EMPTY_STRING).unwrap();
    rpc_call_test_expected(&mut client, rpc::web3_sha3(empty), expected);
}
