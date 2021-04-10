use super::Rpc;
use crate::types::{Address, Bytes, PrivateKey, TransactionRequest, H256};

pub fn personal_import_raw_key(private_key: PrivateKey, password: String) -> Rpc<Address> {
    let mut rpc = Rpc::new("personal_importRawKey");
    rpc.add_param(private_key);
    rpc.add_param(password);
    rpc
}

pub fn personal_list_accounts() -> Rpc<Vec<Address>> {
    Rpc::new("personal_listAccounts")
}

pub fn personal_unlock_account(
    address: Address,
    password: String,
    wrapped_duration: Option<u32>,
) -> Rpc<bool> {
    let mut rpc = Rpc::new("personal_unlockAccount");
    rpc.add_param(address);
    rpc.add_param(password);
    let duration = match wrapped_duration {
        Some(duration) => duration,
        None => 0,
    };
    rpc.add_param(duration);
    rpc
}

pub fn personal_lock_account(address: Address) -> Rpc<bool> {
    let mut rpc = Rpc::new("personal_lockAccount");
    rpc.add_param(address);
    rpc
}

pub fn personal_new_account(password: String) -> Rpc<Address> {
    let mut rpc = Rpc::new("personal_newAccount");
    rpc.add_param(password);
    rpc
}

pub fn personal_send_transaction(transaction: TransactionRequest, password: String) -> Rpc<H256> {
    let mut rpc = Rpc::new("personal_sendTransaction");
    rpc.add_param(transaction);
    rpc.add_param(password);
    rpc
}

pub fn personal_sign(message: Bytes, address: Address, password: String) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("personal_sign");
    rpc.add_param(message);
    rpc.add_param(address);
    rpc.add_param(password);
    rpc
}

pub fn personal_ec_recover(message: Bytes, signature: Bytes) -> Rpc<Address> {
    let mut rpc = Rpc::new("personal_ecRecover");
    rpc.add_param(message);
    rpc.add_param(signature);
    rpc
}
