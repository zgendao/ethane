use super::Rpc;
use crate::types::{Bytes, H256};

pub fn web3_client_version() -> Rpc<String> {
    Rpc::new("web3_clientVersion")
}

pub fn web3_sha3(input: Bytes) -> Rpc<H256> {
    let mut rpc = Rpc::new("web3_sha3");
    rpc.add_param(input);
    rpc
}
