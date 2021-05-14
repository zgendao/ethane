mod be_bytes;
mod bytes;
mod ethereum_type;
use ethereum_type::EthereumType;

pub use bytes::Bytes;
/// A 160 bit (20 bytes) special Address type.
pub type Address = EthereumType<20_usize, true>;
/// A 2048 bit (256 bytes) Bloom hash type.
pub type Bloom = EthereumType<256_usize, true>;
/// A 256 bit (32 bytes) hash type.
pub type H256 = EthereumType<32_usize, true>;
/// A 64 bit (8 bytes) hash type.
pub type H64 = EthereumType<8_usize, true>;
/// A 256 bit (32 bytes) unsigned integer type.
pub type U256 = EthereumType<32_usize, false>;
/// A 128 bit (16 bytes) unsigned integer type.
pub type U128 = EthereumType<16_usize, false>;
/// A 128 bit (8 bytes) unsigned integer type.
pub type U64 = EthereumType<8_usize, false>;
