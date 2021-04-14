use super::utils::*;
use ethereum_types::H256;

use std::collections::HashMap;
use std::ops::Range;

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
    fn encode(&self) -> Vec<u8> {
        match self {
            Self::Address(data) | Self::Bool(data) | Self::Int(data, _) | Self::Uint(data, _) => {
                data.as_bytes().to_vec()
            }
            Self::FixedBytes(data) | Self::Bytes(data) | Self::String(data) => {
                let mut encoded = left_pad_to_32_bytes(&data.len().to_be_bytes()).to_vec();
                encoded.extend_from_slice(&right_pad_to_32_multiples(data));
                encoded
            }
            Self::FixedArray(params) | Self::Array(params) | Self::Tuple(params) => {
                let mut encoded = left_pad_to_32_bytes(&params.len().to_be_bytes()).to_vec();
                for p in params {
                    encoded.extend_from_slice(&p.encode());
                }
                encoded
            }
        }
    }

    /// Recursively checks wether a given type is dynamic.
    ///
    /// For example, a [`Tuple`] can be dynamic if any of its contained types
    /// are dynamic. Additionally, a [`FixedArray`] is static if it contains
    /// values with static type and dynamic otherwise.
    fn is_dynamic(&self) -> bool {
        match self {
            Self::Array(_) | Self::Bytes | Self::String => true,
            Self::FixedArray(parameter_type, _) => parameter_type.is_dynamic(),
            Self::Tuple(value) => value.iter().any(|x| x.is_dynamic()),
            _ => false,
        }
    }
}

fn encode_into(hash: &mut [u8], parameters: Vec<Parameter>) -> usize {
    let mut hash_len = hash.len();
    let mut dynamic_type_map = HashMap::<usize, Range>::with_capacity(parameters.len());
    for (i, param) in parameters.iter().enumerate() {
        if param.is_dynamic() {
            // save range where we will insert the data pointer since
            // we don't know (YET) where exactly the dynamic data will
            // start
            dynamic_type_map.insert(i, hash_len..hash_len + 32);
            // append a 32 byte zero slice as a placeholder for our
            // future dynamic data pointer
            hash.extend_from_slice(&[0u8; 32]);
            // update hash position (length)
            hash_len = hash.len();
        } else {
            hash.extend_from_slice(param.static_encode());
        }
    }

    for (i, range) in dynamic_type_map {
    }

    0
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
        assert_eq!(Parameter::Array(vec![
                Parameter::Uint(H256::from_low_u64_be(0x4a), 8),
                Parameter::Uint(H256::from_low_u64_be(0xff), 8),
                Parameter::Uint(H256::from_low_u64_be(0xde), 8),
        ]).static_encode(),
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
        assert!(Parameter::Bytes.is_dynamic());
        assert!(!Parameter::FixedBytes(128).is_dynamic());
        assert!(!Parameter::Function.is_dynamic());
        assert!(!Parameter::Uint(32).is_dynamic());
        assert!(!Parameter::Int(256).is_dynamic());
        assert!(Parameter::String.is_dynamic());
        assert!(Parameter::Array(Box::new(Parameter::Address)).is_dynamic());
        assert!(Parameter::Array(Box::new(Parameter::Bytes)).is_dynamic());
        assert!(!Parameter::FixedArray(Box::new(Parameter::Function), 3).is_dynamic());
        assert!(Parameter::FixedArray(Box::new(Parameter::String), 2).is_dynamic());
        assert!(!Parameter::Tuple(vec![
            Parameter::Function,
            Parameter::Uint(32),
            Parameter::FixedBytes(64)
        ])
        .is_dynamic());
        assert!(Parameter::Tuple(vec![
            Parameter::Function,
            Parameter::Uint(32),
            Parameter::String
        ])
        .is_dynamic());
        assert!(!Parameter::FixedArray(
            Box::new(Parameter::FixedArray(
                Box::new(Parameter::Int(8)),
                5
            )),
            2
        )
        .is_dynamic());
        assert!(Parameter::Tuple(vec![
            Parameter::Function,
            Parameter::Uint(32),
            Parameter::FixedArray(Box::new(Parameter::String), 3)
        ])
        .is_dynamic());
    }
}
