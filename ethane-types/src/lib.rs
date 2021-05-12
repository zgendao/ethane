mod be_bytes;
mod ethereum_type;
use ethereum_type::EthereumType;

pub type Address = EthereumType<20_usize>;
pub type H256 = EthereumType<32_usize>;
pub type U256 = EthereumType<32_usize>;
