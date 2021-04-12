use super::tmp::Parameter;
use crate::parameter::utils::*;
use ethereum_types::{Address, H256, U256};

use std::convert::From;

impl From<u8> for Parameter {
    fn from(input: u8) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(&[input])))
    }
}

impl From<u16> for Parameter {
    fn from(input: u16) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<u32> for Parameter {
    fn from(input: u32) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<u64> for Parameter {
    fn from(input: u64) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<u128> for Parameter {
    fn from(input: u128) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<[u8; 32]> for Parameter {
    fn from(input: [u8; 32]) -> Self {
        Self::Uint(H256::from(input))
    }
}

impl From<i8> for Parameter {
    fn from(input: i8) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(&[input as u8])))
    }
}

impl From<i16> for Parameter {
    fn from(input: i16) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<i32> for Parameter {
    fn from(input: i32) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<i64> for Parameter {
    fn from(input: i64) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<i128> for Parameter {
    fn from(input: i128) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(
            &input.to_be_bytes(),
        )))
    }
}

impl From<bool> for Parameter {
    fn from(input: bool) -> Self {
        Self::from(u8::from(input))
    }
}

impl From<&[u8]> for Parameter {
    fn from(input: &[u8]) -> Self {
        let len = input.len();
        let rem = len % 32;
        let mut bytes = vec![0u8; len + rem];

        bytes[..len].copy_from_slice(input);
        Self::Bytes(right_pad_to_32_bytes(&bytes).to_vec())
    }
}

impl From<&str> for Parameter {
    fn from(input: &str) -> Self {
        Self::from(input.as_bytes())
    }
}

impl From<Address> for Parameter {
    fn from(input: Address) -> Self {
        Self::Address(H256::from_slice(&left_pad_to_32_bytes(&input.as_bytes())))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parameter_from_numeric() {
        // from u8
        let param = Parameter::from(17u8);
        if let Parameter::Uint(value) = param {
            let mut expected = [0u8; 32];
            expected[31] = 17;
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("Test failed")
        }

        // from u16
        let param = Parameter::from(0b1110_0100_0000_0010_u16);
        if let Parameter::Uint(value) = param {
            let mut expected = [0u8; 32];
            expected[30..].copy_from_slice(&[228, 2]);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("Test failed")
        }
    }
}
