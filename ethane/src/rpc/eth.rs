use super::Rpc;
use crate::types::{
    Address, Block, BlockParameter, Bytes, Call, Filter, GasCall, HashOrLog, SignedTransaction,
    SyncInfo, Transaction, TransactionReceipt, TransactionRequest, H256, H64, U128, U256, U64,
};

pub fn eth_protocol_version() -> Rpc<String> {
    Rpc::new("eth_protocolVersion")
}

pub fn eth_syncing() -> Rpc<SyncInfo> {
    Rpc::new("eth_syncing")
}

pub fn eth_coinbase() -> Rpc<Address> {
    Rpc::new("eth_coinbase")
}

pub fn eth_mining() -> Rpc<bool> {
    Rpc::new("eth_mining")
}

pub fn eth_hashrate() -> Rpc<U256> {
    Rpc::new("eth_hashrate")
}

pub fn eth_gas_price() -> Rpc<U256> {
    Rpc::new("eth_gasPrice")
}

pub fn eth_accounts() -> Rpc<Vec<Address>> {
    Rpc::new("eth_accounts")
}

pub fn eth_block_number() -> Rpc<U64> {
    Rpc::new("eth_blockNumber")
}

pub fn eth_get_balance(address: Address, block_param: Option<BlockParameter>) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getBalance");
    rpc.add_param(address);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_send_transaction(transaction: TransactionRequest) -> Rpc<H256> {
    let mut rpc = Rpc::new("eth_sendTransaction");
    rpc.add_param(transaction);
    rpc
}

pub fn eth_get_transaction_by_hash(transaction_hash: H256) -> Rpc<Transaction> {
    let mut rpc = Rpc::new("eth_getTransactionByHash");
    rpc.add_param(transaction_hash);
    rpc
}

pub fn eth_get_transaction_receipt(transaction_hash: H256) -> Rpc<Option<TransactionReceipt>> {
    let mut rpc = Rpc::new("eth_getTransactionReceipt");
    rpc.add_param(transaction_hash);
    rpc
}

pub fn eth_get_storage_at(
    address: Address,
    storage_pos: U256,
    block_param: Option<BlockParameter>,
) -> Rpc<Bytes> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getStorageAt");
    rpc.add_param(address);
    rpc.add_param(storage_pos);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_transaction_count(
    address: Address,
    block_param: Option<BlockParameter>,
) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getTransactionCount");
    rpc.add_param(address);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_block_by_number(
    block_param: Option<BlockParameter>,
    full_transactions: bool,
) -> Rpc<Option<Block>> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getBlockByNumber");
    rpc.add_param(block_param);
    rpc.add_param(full_transactions);
    rpc
}

pub fn eth_get_block_transaction_count_by_hash(block_hash: H256) -> Rpc<U64> {
    let mut rpc = Rpc::new("eth_getBlockTransactionCountByHash");
    rpc.add_param(block_hash);
    rpc
}

pub fn eth_get_block_transaction_count_by_number(block_param: Option<BlockParameter>) -> Rpc<U64> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getBlockTransactionCountByNumber");
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_uncle_count_by_block_hash(block_hash: H256) -> Rpc<U64> {
    let mut rpc = Rpc::new("eth_getUncleCountByBlockHash");
    rpc.add_param(block_hash);
    rpc
}

pub fn eth_get_uncle_count_by_block_number(block_param: Option<BlockParameter>) -> Rpc<U64> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getUncleCountByBlockNumber");
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_code(address: Address, block_param: Option<BlockParameter>) -> Rpc<Bytes> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getCode");
    rpc.add_param(address);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_sign(address: Address, data: Bytes) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("eth_sign");
    rpc.add_param(address);
    rpc.add_param(data);
    rpc
}

// DEVIATION FROM SPEC
// c.f. https://github.com/ethereum/go-ethereum/issues/22223
// also geth returns something like: {raw: hex_encoded_tx, tx: json_encoded_tx}, however according to JSON RPC
// it should return only the transaction hash
//
// We decide here to use what geth currently does and not follow the spec
pub fn eth_sign_transaction(transaction: TransactionRequest) -> Rpc<SignedTransaction> {
    let mut rpc = Rpc::new("eth_signTransaction");
    rpc.add_param(transaction);
    rpc
}

pub fn eth_send_raw_transaction(raw_transaction: Bytes) -> Rpc<H256> {
    let mut rpc = Rpc::new("eth_sendRawTransaction");
    rpc.add_param(raw_transaction);
    rpc
}

pub fn eth_call(call: Call, block_param: Option<BlockParameter>) -> Rpc<Bytes> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_call");
    rpc.add_param(call);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_estimate_gas(gas_call: GasCall, block_param: Option<BlockParameter>) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_estimateGas");
    rpc.add_param(gas_call);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_block_by_hash(block_hash: H256, full_transactions: bool) -> Rpc<Option<Block>> {
    let mut rpc = Rpc::new("eth_getBlockByHash");
    rpc.add_param(block_hash);
    rpc.add_param(full_transactions);
    rpc
}

pub fn eth_get_transaction_by_block_hash_and_index(
    block_hash: H256,
    index_position: U64,
) -> Rpc<Transaction> {
    let mut rpc = Rpc::new("eth_getTransactionByBlockHashAndIndex");
    rpc.add_param(block_hash);
    rpc.add_param(index_position);
    rpc
}
//
pub fn eth_get_transaction_by_block_number_and_index(
    block_param: Option<BlockParameter>,
    index_position: U64,
) -> Rpc<Transaction> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getTransactionByBlockNumberAndIndex");
    rpc.add_param(block_param);
    rpc.add_param(index_position);
    rpc
}

pub fn eth_get_uncle_by_block_hash_and_index(
    block_hash: H256,
    index_position: U64,
) -> Rpc<Option<Block>> {
    let mut rpc = Rpc::new("eth_getUncleByBlockHashAndIndex");
    rpc.add_param(block_hash);
    rpc.add_param(index_position);
    rpc
}

pub fn eth_get_uncle_by_block_number_and_index(
    block_param: Option<BlockParameter>,
    index_position: U64,
) -> Rpc<Option<Block>> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getUncleByBlockNumberAndIndex");
    rpc.add_param(block_param);
    rpc.add_param(index_position);
    rpc
}

// DEVIATION FROM SPEC
// Not supported by geth
// c.f. https://github.com/ethereum/EIPs/issues/209
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_get_compilers() -> Rpc<Vec<String>> {
    Rpc::new("eth_getCompilers")
}

// DEVIATION FROM SPEC
// Not supported by geth
// c.f. https://github.com/ethereum/EIPs/issues/209
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_compile_lll(source_code: String) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("eth_compileLLL");
    rpc.add_param(source_code);
    rpc
}

// DEVIATION FROM SPEC
// Not supported by geth
// c.f. https://github.com/ethereum/EIPs/issues/209
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_compile_solidity(source_code: String) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("eth_compileSolidity");
    rpc.add_param(source_code);
    rpc
}

// DEVIATION FROM SPEC
// Not supported by geth
// c.f. https://github.com/ethereum/EIPs/issues/209
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_compile_serpent(source_code: String) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("eth_compileSerpent");
    rpc.add_param(source_code);
    rpc
}

pub fn eth_new_filter(filter: Filter) -> Rpc<U128> {
    let mut rpc = Rpc::new("eth_newFilter");
    rpc.add_param(filter);
    rpc
}

pub fn eth_new_block_filter() -> Rpc<U128> {
    Rpc::new("eth_newBlockFilter")
}

pub fn eth_new_pending_transaction_filter() -> Rpc<U128> {
    Rpc::new("eth_newPendingTransactionFilter")
}

pub fn eth_uninstall_filter(filter_id: U128) -> Rpc<bool> {
    let mut rpc = Rpc::new("eth_uninstallFilter");
    rpc.add_param(filter_id);
    rpc
}

pub fn eth_get_filter_changes(filter_id: U128) -> Rpc<Vec<HashOrLog>> {
    let mut rpc = Rpc::new("eth_getFilterChanges");
    rpc.add_param(filter_id);
    rpc
}

// DEVIATION FROM SPEC
// Does not seem to work with a block_filter_id or pending_transactions_filter_id
pub fn eth_get_filter_logs(filter_id: U128) -> Rpc<Vec<HashOrLog>> {
    let mut rpc = Rpc::new("eth_getFilterLogs");
    rpc.add_param(filter_id);
    rpc
}

pub fn eth_get_logs(filter: Filter) -> Rpc<Vec<HashOrLog>> {
    let mut rpc = Rpc::new("eth_getLogs");
    rpc.add_param(filter);
    rpc
}

// DEVIATION FROM SPEC
// Not supported by geth
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_get_work() -> Rpc<Vec<H256>> {
    Rpc::new("eth_getWork")
}

// DEVIATION FROM SPEC
// Not supported by geth
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_submit_work(nonce: H64, hash: H256, digest: H256) -> Rpc<bool> {
    let mut rpc = Rpc::new("eth_submitWork");
    rpc.add_param(nonce);
    rpc.add_param(hash);
    rpc.add_param(digest);
    rpc
}

// DEVIATION FROM SPEC
// Not supported by geth
#[deprecated(note = "This functionality seems to be not provided anymore by ethereum nodes.")]
pub fn eth_submit_hashrate(hash_rate: H256, client_id: H256) -> Rpc<bool> {
    let mut rpc = Rpc::new("eth_submitHashrate");
    rpc.add_param(hash_rate);
    rpc.add_param(client_id);
    rpc
}
