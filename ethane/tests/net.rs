use ethane::rpc;

use ethane_helper::*;

#[test]
fn test_net_version() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::net_version());
}

#[test]
#[ignore]
fn test_net_peer_count() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::net_peer_count());
}

#[test]
fn test_net_listening() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::net_listening());
}
