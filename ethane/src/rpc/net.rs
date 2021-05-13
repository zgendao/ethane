use super::Rpc;
use crate::types::U64;

pub fn net_version() -> Rpc<String> {
    Rpc::new("net_version")
}

pub fn net_peer_count() -> Rpc<U64> {
    Rpc::new("net_peerCount")
}

pub fn net_listening() -> Rpc<bool> {
    Rpc::new("net_listening")
}
