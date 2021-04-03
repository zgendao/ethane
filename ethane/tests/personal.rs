use ethane::rpc;
use ethane::types::{Bytes, PrivateKey, TransactionRequest, H160, H256};
use std::str::FromStr;

use test_helper::*;

#[test]
fn test_personal_list_accounts() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(&mut client, rpc::personal_list_accounts());
}

#[test]
fn test_personal_import_raw_key() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let secret = create_secret();
    let pk: PrivateKey = PrivateKey::NonPrefixed(secret);
    rpc_call_test_some(
        &mut client,
        rpc::personal_import_raw_key(pk, String::from("")),
    )
}

#[test]
fn test_personal_unlock_account() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let secret = create_secret();
    let address = import_account(&mut client, secret).unwrap();

    rpc_call_test_expected(
        &mut client,
        rpc::personal_unlock_account(address, String::from(ACCOUNTS_PASSWORD), None),
        true,
    );
}

#[test]
fn test_personal_lock_account() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let secret = create_secret();
    let address = import_account(&mut client, secret).unwrap();
    unlock_account(&mut client, address);
    rpc_call_test_expected(&mut client, rpc::personal_lock_account(address), true);
}

#[test]
fn test_personal_new_account() {
    let mut client = ConnectorWrapper::new_from_env(None);
    rpc_call_test_some(
        &mut client,
        rpc::personal_new_account(String::from(ACCOUNTS_PASSWORD)),
    );
}

#[test]
fn test_personal_send_transaction() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let (_secret, address) = create_account(&mut client);
    let tx = TransactionRequest {
        from: address,
        to: Some(create_account(&mut client).1),
        ..Default::default()
    };
    rpc_call_test_some(
        &mut client,
        rpc::personal_send_transaction(tx, String::from(ACCOUNTS_PASSWORD)),
    );
}

#[test]
fn test_personal_sign() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let address = import_account(&mut client, H256::from_str(FIX_SECRET).unwrap()).unwrap();
    let message = Bytes::from_slice("checkmate".as_bytes());
    let expected_signature = Bytes::from_str(
        "67e4a4cf3b8cfb7d9a568482e9b6deb6350bc7701ae0448b92752b463e7dc97\
        c09c424607fbcf1cb4f6ec1c6a6c60a3527dcfe11412a3bff26218ca9f0bdef9d1b",
    )
    .unwrap();

    rpc_call_test_expected(
        &mut client,
        rpc::personal_sign(message, address, String::from(ACCOUNTS_PASSWORD)),
        expected_signature,
    );
}

#[test]
fn test_personal_ec_recover() {
    let mut client = ConnectorWrapper::new_from_env(None);
    let message = Bytes::from_slice("checkmate".as_bytes());
    let signature = Bytes::from_str(
        "67e4a4cf3b8cfb7d9a568482e9b6deb6350bc7701ae0448b92752b463e7dc97\
        c09c424607fbcf1cb4f6ec1c6a6c60a3527dcfe11412a3bff26218ca9f0bdef9d1b",
    )
    .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::personal_ec_recover(message, signature),
        H160::from_str(FIX_ADDRESS).unwrap(),
    )
}
