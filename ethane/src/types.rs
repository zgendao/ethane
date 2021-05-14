//! Collects all the needed Ethereum types
//!
//! This module provides custom types, but also re-exports some types from [ethereum_types].

pub use ethane_types::{Address, Bloom, H256, H64, U128, U256, U64};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

/// Information about block number, defaults to `BlockParameter::Latest`
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum BlockParameter {
    Latest,
    Earliest,
    Pending,
    Custom(U64),
}

impl Serialize for BlockParameter {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        match *self {
            BlockParameter::Latest => serializer.serialize_str("latest"),
            BlockParameter::Earliest => serializer.serialize_str("earliest"),
            BlockParameter::Pending => serializer.serialize_str("pending"),
            BlockParameter::Custom(num) => serializer.serialize_str(&num.to_string()), // TODO non-prefixed string?
        }
    }
}

impl Default for BlockParameter {
    fn default() -> Self {
        BlockParameter::Latest
    }
}

/// Used for creating transactions
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct TransactionRequest {
    pub from: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,
}

/// A pending or processed transaction
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub from: Option<Address>,
    pub gas: U256,
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    pub hash: H256,
    pub input: Bytes,
    pub nonce: U256,
    pub to: Option<Address>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    pub value: U256,
    pub v: Option<U64>,
    pub r: Option<U256>,
    pub s: Option<U256>,
}

/// Transaction receipt of a processed transaction
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: H256,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: U64,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    #[serde(rename = "blockNumber")]
    pub block_number: U64,
    pub from: Address,
    pub to: Option<Address>,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<Address>,
    pub logs: Vec<Log>,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Bloom,
    pub status: U64,
}

///Contains information about events
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Bytes,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<H256>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    #[serde(rename = "logIndex")]
    pub log_index: Option<U256>,
    #[serde(rename = "transactionLogIndex")]
    pub transaction_log_index: Option<U256>,
    pub removed: bool,
}

/// A type for hex values of arbitrary length
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Self {
        Bytes(slice.to_vec())
    }
}

impl Serialize for Bytes {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_str(&(String::from("0x") + &hex::encode(&self.0)))
    }
}

impl FromStr for Bytes {
    type Err = hex::FromHexError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim_start_matches("0x");
        Ok(Bytes(hex::decode(trimmed)?))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<T>(deserializer: T) -> Result<Bytes, T::Error>
    where
        T: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a hex string")
    }

    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let result = Self::Value::from_str(value)
            .map_err(|err| serde::de::Error::custom(format!("Invalid hex string: {}", err)))?;
        Ok(result)
    }

    fn visit_string<T: serde::de::Error>(self, value: String) -> Result<Self::Value, T> {
        self.visit_str(&value)
    }
}

/// Wrapper for private keys to allow for 0x-prefixed and plain serialization
#[derive(Clone, Debug, PartialEq)]
pub enum PrivateKey {
    ZeroXPrefixed(H256),
    NonPrefixed(H256),
}

impl Serialize for PrivateKey {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        match *self {
            PrivateKey::ZeroXPrefixed(pk) => pk.serialize(serializer),
            PrivateKey::NonPrefixed(pk) => serializer.serialize_str(&hex::encode(pk.into_bytes())),
        }
    }
}

/// Standard block type
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Block {
    pub number: Option<U64>,
    pub hash: Option<H256>,
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    pub nonce: Option<H64>,
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: H256,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<Bloom>,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    pub miner: Address,
    pub difficulty: U256,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: U256,
    #[serde(rename = "extraData")]
    pub extra_data: Bytes,
    pub size: U256,
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    pub timestamp: U256,
    pub transactions: Vec<TransactionOrHash>,
    pub uncles: Vec<H256>,
}

/// BlockHeader returned by subscription
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BlockHeader {
    pub number: Option<U64>,
    pub hash: Option<H256>,
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    pub nonce: Option<H64>,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    pub difficulty: U256,
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: H256,
    pub miner: Address,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<Bloom>,
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "extraData")]
    pub extra_data: Bytes,
    #[serde(rename = "mixHash")]
    pub mix_hash: Option<H256>,
}

/// Wrapper to allow returned blocks to contain complete transactions or hashes
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TransactionOrHash {
    Transaction(Transaction),
    Hash(H256),
}

/// Call object for querying data
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Call {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    pub to: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

/// Used to estimate gas
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct GasCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

/// A filter object to listen for events
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Filter {
    #[serde(rename = "fromBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockParameter>,
    #[serde(rename = "toBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ValueOrVec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Option<ValueOrVec<H256>>>>,
}

/// Filter object used in subscriptions
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct FilterSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ValueOrVec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Option<ValueOrVec<H256>>>>,
}

/// Wrapper to allow a single value or a list of values
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ValueOrVec<T> {
    Value(T),
    Vec(Vec<T>),
}

/// Wrapper to allow filter RPCs to return a transaction hash or a log object
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum HashOrLog {
    H256(H256),
    Log(Log),
}

/// Status of the transaction pool
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TxPoolStatus {
    pub pending: U256,
    pub queued: U256,
}

/// Content of the transaction pool
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TxPoolContent {
    pub pending: HashMap<Address, HashMap<String, Transaction>>,
    pub queued: HashMap<Address, HashMap<String, Transaction>>,
}

/// Content of transaction pool in debug view
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TxPoolInspect {
    pub pending: HashMap<Address, HashMap<String, String>>,
    pub queued: HashMap<Address, HashMap<String, String>>,
}

/// Wrapper for node synchronization info
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum SyncInfo {
    Syncing(SyncStatus),
    NotSyncing(bool),
}

/// Wrapper for node synchronization info in a subscription
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SyncInfoSubscription {
    pub syncing: bool,
    pub status: Option<SyncStatus>,
}

/// Status object containing information about node synchronization
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SyncStatus {
    #[serde(rename = "startingBlock")]
    pub starting_block: u64,
    #[serde(rename = "currentBlock")]
    pub current_block: u64,
    #[serde(rename = "highestBlock")]
    pub highest_block: u64,
    #[serde(rename = "pulledStates")]
    pub pulled_states: Option<u64>,
    #[serde(rename = "knownStates")]
    pub known_states: Option<u64>,
}

/// A wrapper for a signed transaction
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SignedTransaction {
    pub raw: Bytes,
    pub tx: Transaction,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_types_bytes() {
        let bytes_0 = Bytes::from_slice(&[]);
        let bytes_1 = Bytes::from_slice(&[0, 0]);
        let bytes_2 = Bytes::from_slice(&[17, 234]);
        let bytes_3 = Bytes::from_str("0x").unwrap();
        let bytes_4 = Bytes::from_str("0x00").unwrap();
        let bytes_5 = Bytes::from_str("00421100").unwrap();

        let expected_0 = "\"0x\"";
        let expected_1 = "\"0x0000\"";
        let expected_2 = "\"0x11ea\"";
        let expected_3 = Bytes(vec![]);
        let expected_4 = Bytes(vec![0]);
        let expected_5 = Bytes(vec![0, 66, 17, 0]);

        assert_eq!(serde_json::to_string(&bytes_0).unwrap(), expected_0);
        assert_eq!(serde_json::to_string(&bytes_1).unwrap(), expected_1);
        assert_eq!(serde_json::to_string(&bytes_2).unwrap(), expected_2);
        assert_eq!(bytes_3, expected_3);
        assert_eq!(bytes_4, expected_4);
        assert_eq!(bytes_5, expected_5);
    }

    #[test]
    fn test_types_block_parameter() {
        let block_param_default = BlockParameter::default();
        let block_param_custom = BlockParameter::Custom(U64::from_int_unchecked(11827902_u64));

        assert_eq!(
            serde_json::to_string(&block_param_default).unwrap(),
            "\"latest\""
        );
        assert_eq!(
            serde_json::to_string(&block_param_custom).unwrap(),
            "\"0xb47abe\""
        );
    }

    #[test]
    fn test_types_private_key() {
        let raw_hex_key = "0xe4745d1287b67412ce806746e83d49efe5cec53f5a27aa666fb9e8092a8dbd43";
        let private_key_prefixed = PrivateKey::ZeroXPrefixed(H256::try_from(raw_hex_key).unwrap());
        let private_key_non_prefixed =
            PrivateKey::NonPrefixed(H256::try_from(raw_hex_key).unwrap());

        assert_eq!(
            serde_json::to_string(&private_key_prefixed).unwrap(),
            "\"0xe4745d1287b67412ce806746e83d49efe5cec53f5a27aa666fb9e8092a8dbd43\""
        );
        assert_eq!(
            serde_json::to_string(&private_key_non_prefixed).unwrap(),
            "\"e4745d1287b67412ce806746e83d49efe5cec53f5a27aa666fb9e8092a8dbd43\""
        );
    }
}
