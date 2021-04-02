use ethane::rpc;
use ethane::types::U64;

use test_helper::*;

#[test]
fn test_net_version() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::net_version(), String::from("1337"));
}

#[test]
#[ignore]
fn test_net_peer_count() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::net_peer_count(), U64::from(0));
}

#[test]
fn test_net_listening() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_expected(&mut client, rpc::net_listening(), true);
}
