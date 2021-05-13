mod be_bytes;
mod ethereum_type;
use ethereum_type::EthereumType;

/// A 160 bit (20 bytes) type.
pub type Address = EthereumType<20_usize>;
/// A 256 bit (32 bytes) Bloom hash type.
pub type Bloom = EthereumType<32_usize>;
/// A 256 bit (32 bytes) hash type.
pub type H256 = EthereumType<32_usize>;
/// A 64 bit (8 bytes) hash type.
pub type H64 = EthereumType<8_usize>;
/// A 256 bit (32 bytes) unsigned integer type.
pub type U256 = EthereumType<32_usize>;
/// A 128 bit (16 bytes) unsigned integer type.
pub type U128 = EthereumType<16_usize>;
/// A 128 bit (8 bytes) unsigned integer type.
pub type U64 = EthereumType<8_usize>;
