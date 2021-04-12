use super::tmp::Parameter;
use crate::parameter::utils::*;
use ethereum_types::{Address, U256};

use std::convert::From;

impl From<u8> for Parameter {
    fn from(input: u8) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&[input])))
    }
}

impl From<u16> for Parameter {
    fn from(input: u16) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<u32> for Parameter {
    fn from(input: u32) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<u64> for Parameter {
    fn from(input: u64) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<u128> for Parameter {
    fn from(input: u128) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<U256> for Parameter {
    fn from(input: U256) -> Self {
        Self::Uint(input)
    }
}

impl From<i8> for Parameter {
    fn from(input: i8) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&[input as u8])))
    }
}

impl From<i16> for Parameter {
    fn from(input: i16) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<i32> for Parameter {
    fn from(input: i32) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<i64> for Parameter {
    fn from(input: i64) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
    }
}

impl From<i128> for Parameter {
    fn from(input: i128) -> Self {
        Self::Uint(U256::from(left_pad_to_32_bytes(&input.to_be_bytes())))
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
        Self::Bytes(bytes.to_vec())
    }
}

impl From<&str> for Parameter {
    fn from(input: &str) -> Self {
        Self::from(input.as_bytes())
    }
}

impl From<Address> for Parameter {
    fn from(input: Address) -> Self {
        Self::Address(U256::from(left_pad_to_32_bytes(&input.as_bytes())))
    }
}
