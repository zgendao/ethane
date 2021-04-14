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
    FixedBytes(Vec<u8>),
    Array(Vec<Parameter>),
    FixedArray(Vec<Parameter>),
    Tuple(Vec<Parameter>),
}

impl Parameter {
    /// Encodes strictly the data part of the underlying type.
    ///
    /// It will not check whether the parameter is dynamic or not, it simply
    /// encodes the enclosed data in place. For some types, it first writes the
    /// number of elements of the data in bytes. For further info, check the
    /// Solidity [contract ABI
    /// specification](https://docs.soliditylang.org/en/v0.5.3/abi-spec.html#function-selector).
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::Address(data) | Self::Bool(data) | Self::Int(data, _) | Self::Uint(data, _) => {
                data.as_bytes().to_vec()
            }
            Self::FixedBytes(data) => right_pad_to_32_multiples(data).to_vec(),
            Self::Bytes(data) | Self::String(data) => {
                let mut encoded = left_pad_to_32_bytes(&data.len().to_be_bytes()).to_vec();
                encoded.extend_from_slice(&right_pad_to_32_multiples(data));
                encoded
            }
            Self::FixedArray(params) => {
                let mut encoded = Vec::<u8>::new();
                for p in params {
                    encoded.extend_from_slice(&p.encode());
                }
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

    /// Recursively checks wether a given parameter is dynamic.
    ///
    /// For example, a [`Tuple`] can be dynamic if any of its contained types
    /// are dynamic. Additionally, a [`FixedArray`] is static if it contains
    /// values with static type and dynamic otherwise.
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Array(_) | Self::Bytes(_) | Self::String(_) => true,
            Self::FixedArray(parameters) | Self::Tuple(parameters) => {
                parameters.iter().any(|x| x.is_dynamic())
            }
            _ => false,
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

    #[test]
    fn parameter_is_dynamic() {
        assert!(!Parameter::Address(H256::zero()).is_dynamic());
        assert!(!Parameter::Bool(H256::zero()).is_dynamic());
        assert!(Parameter::Bytes(Vec::new()).is_dynamic());
        assert!(!Parameter::FixedBytes(Vec::new()).is_dynamic());
        assert!(!Parameter::Uint(H256::zero(), 16).is_dynamic());
        assert!(!Parameter::Int(H256::zero(), 32).is_dynamic());
        assert!(Parameter::String(Vec::new()).is_dynamic());
        assert!(Parameter::Array(vec![Parameter::Address(H256::zero()); 5]).is_dynamic());
        assert!(Parameter::Array(vec![Parameter::Bytes(Vec::new())]).is_dynamic());
        assert!(!Parameter::FixedArray(vec![Parameter::Uint(H256::zero(), 64); 3]).is_dynamic());
        assert!(Parameter::FixedArray(vec![Parameter::String(Vec::new()); 2]).is_dynamic());
        assert!(!Parameter::Tuple(vec![
            Parameter::Address(H256::zero()),
            Parameter::Uint(H256::zero(), 32),
            Parameter::FixedBytes(Vec::new())
        ])
        .is_dynamic());
        assert!(Parameter::Tuple(vec![
            Parameter::FixedBytes(Vec::new()),
            Parameter::Uint(H256::zero(), 32),
            Parameter::String(Vec::new())
        ])
        .is_dynamic());
        assert!(!Parameter::FixedArray(vec![
            Parameter::FixedArray(vec![
                Parameter::Int(
                    H256::zero(),
                    8
                );
                5
            ]);
            2
        ])
        .is_dynamic());
        assert!(Parameter::Tuple(vec![
            Parameter::FixedBytes(Vec::new()),
            Parameter::Uint(H256::zero(), 32),
            Parameter::FixedArray(vec![Parameter::String(Vec::new()); 3])
        ])
        .is_dynamic());
    }
}
