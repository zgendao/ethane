mod construction;
pub mod display;
mod encode_into;
mod parameter_type;
mod utils;

pub use encode_into::encode_into;
pub use parameter_type::ParameterType;
use utils::*;

use ethane_types::{Address, H256};

/// An ABI function parameter type enclosing the underlying
/// numeric data bytes.
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
    pub fn static_encode(&self) -> Vec<u8> {
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
            Self::FixedArray(params) | Self::Tuple(params) => {
                let mut encoded = Vec::<u8>::new();
                for p in params {
                    encoded.extend_from_slice(&p.static_encode());
                }
                encoded
            }
            Self::Array(_) => panic!("Array type cannot be statically encoded!"),
        }
    }

    /// Recursively checks wether a given parameter is dynamic.
    ///
    /// For example, a [`Tuple`](Parameter::Tuple) can be dynamic if any of its
    /// contained types are dynamic. Additionally, a
    /// [`FixedArray`](Parameter::FixedArray) is static if it contains values
    /// with static type and dynamic otherwise.
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Array(_) | Self::Bytes(_) | Self::String(_) => true,
            Self::FixedArray(parameters) | Self::Tuple(parameters) => {
                parameters.iter().any(|x| x.is_dynamic())
            }
            _ => false,
        }
    }

    pub fn decode(parameter_type: &ParameterType, raw_bytes: &[u8]) -> (Self, usize) {
        match parameter_type {
            ParameterType::Address => {
                let mut bytes = [0u8; 20];
                bytes.copy_from_slice(&raw_bytes[12..32]);
                (Self::from(Address::from(bytes)), 32)
            }
            ParameterType::Bool => {
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&raw_bytes[..32]);
                (Self::Bool(H256::from(bytes)), 32)
            }
            ParameterType::Int(_) => {
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&raw_bytes[..32]);
                (Self::new_int(bytes, true), 32)
            }
            ParameterType::Uint(_) => {
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&raw_bytes[..32]);
                (Self::new_int(bytes, false), 32)
            }
            //ParameterType::String  => {
            //    (Self::String(raw_bytes.to_vec()), )
            //},
            //ParameterType::Bytes  => {
            //    Ok(Self::Bytes(raw_bytes.to_vec()))
            //},
            //ParameterType::FixedBytes(len) => {
            //    Ok(Self::FixedBytes(raw_bytes.to_vec())
            //},
            //// TODO do we need more complicated types?
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn parameter_encode() {
        assert_eq!(Parameter::Address(H256::zero()).static_encode(), vec![0u8; 32]);
        assert_eq!(Parameter::from("Hello, World!").static_encode(), vec![
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x0d,
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57,
            0x6f, 0x72, 0x6c, 0x64, 0x21, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]); 
        assert_eq!(Parameter::FixedArray(vec![
                Parameter::Uint(H256::from_int_unchecked(0x4a_u8), 8),
                Parameter::Uint(H256::from_int_unchecked(0xff_u8), 8),
                Parameter::Uint(H256::from_int_unchecked(0xde_u8), 8),
        ]).static_encode(),
        vec![
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

    #[test]
    #[rustfmt::skip]
    fn decode_parameter() {
        let result = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0a, 0xff,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0b, 0xff,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0c, 0xff,
        ];

        let mut index = 0;
        let (addr, bytes_read) = Parameter::decode(&ParameterType::Address, &result);
        index += bytes_read;
        let (a, bytes_read) = Parameter::decode(&ParameterType::Uint(16), &result[index..]);
        index += bytes_read;
        let (b, bytes_read) = Parameter::decode(&ParameterType::Uint(16), &result[index..]);
        index += bytes_read;
        let (c, bytes_read) = Parameter::decode(&ParameterType::Uint(16), &result[index..]);
        index += bytes_read;

        assert_eq!(index, 128);
        assert_eq!(addr.to_string(), String::from("0xffffffffffffffffffffffffffffffffffffffff"));
        assert_eq!(a.to_string(), String::from("0xaff"));
        assert_eq!(b.to_string(), String::from("0xbff"));
        assert_eq!(c.to_string(), String::from("0xcff"));
    }
}
