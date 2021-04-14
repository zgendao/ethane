use super::utils::*;
use ethereum_types::H256;

#[derive(Clone)]
pub enum Parameter {
    Address(H256),
    Bool(H256),
    Int(H256, usize),
    Uint(H256, usize),
    String(Vec<u8>),
    Bytes(Vec<u8>),
    Array(Vec<Parameter>),
    Tuple(Vec<Parameter>),
}

/*
impl Parameter {
    /// Returns the encoded data length in bytes.
    #[inline]
    #[rustfmt::skip]
    pub fn data_len(&self) -> usize {
        // TODO what about dynamic types
        match self {
            // add 32 bytes to types encoding the length
            Self::String(data) | Self::Bytes(data) => 32 + data.len(),
            Self::Array(data) => if data.is_empty() { 0 } else { 32 + data.len() * data[1].data_len() },
            Self::Tuple(data) => data.iter().fold(32, |acc, d| acc + d.data_len()),
            _ => 32,
        }
    }
}
*/

impl Parameter {
    #[inline]
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::Address(data) | Self::Bool(data) | Self::Int(data, _) | Self::Uint(data, _) => {
                data.as_bytes().to_vec()
            }
            Self::String(data) | Self::Bytes(data) => {
                let mut encoded = left_pad_to_32_bytes(&data.len().to_be_bytes()).to_vec();
                encoded.extend_from_slice(&right_pad_to_32_multiples(data));
                encoded
            }
            Self::Array(params) | Self::Tuple(params) => {
                let mut encoded = left_pad_to_32_bytes(&params.len().to_be_bytes()).to_vec();
                for p in params {
                    encoded.extend_from_slice(&p.encode());
                }
                encoded
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn parameter_encode() {
        assert_eq!(Parameter::Address(H256::zero()).encode(), vec![0u8; 32]);
        assert_eq!(Parameter::from("Hello, World!").encode(), vec![
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x0d,
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57,
            0x6f, 0x72, 0x6c, 0x64, 0x21, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]); 
        assert_eq!(Parameter::Array(vec![
                Parameter::Uint(H256::from_low_u64_be(0x4a), 8),
                Parameter::Uint(H256::from_low_u64_be(0xff), 8),
                Parameter::Uint(H256::from_low_u64_be(0xde), 8),
        ]).encode(),
        vec![
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x03, // length
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x4a, // first
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0xff, // second
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0xde, // third
        ]);
    }
}
