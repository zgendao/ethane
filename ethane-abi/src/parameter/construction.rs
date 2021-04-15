use super::utils::*;
use super::Parameter;
use ethereum_types::{Address, H256, U128, U256, U64};

use std::convert::From;

impl Parameter {
    #[inline]
    pub fn new_int(bytes: [u8; 32], signed: bool) -> Self {
        if signed {
            Self::Int(H256::from(&bytes), 256)
        } else {
            Self::Uint(H256::from(&bytes), 256)
        }
    }

    /// Method for creating [`FixedBytes`] from a slice of bytes.
    #[inline]
    pub fn new_fixed_bytes(bytes: &[u8]) -> Self {
        Self::FixedBytes(bytes.to_vec())
    }

    #[inline]
    pub fn new_bytes(bytes: &[u8]) -> Self {
        Self::Bytes(bytes.to_vec())
    }
}

// Unsigned elementary integer types
impl From<u8> for Parameter {
    #[inline]
    fn from(input: u8) -> Self {
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(&[input])), 8)
    }
}

impl From<u16> for Parameter {
    #[inline]
    fn from(input: u16) -> Self {
        Self::Uint(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            16,
        )
    }
}

impl From<u32> for Parameter {
    #[inline]
    fn from(input: u32) -> Self {
        Self::Uint(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            32,
        )
    }
}

impl From<u64> for Parameter {
    #[inline]
    fn from(input: u64) -> Self {
        Self::Uint(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            64,
        )
    }
}

impl From<u128> for Parameter {
    #[inline]
    fn from(input: u128) -> Self {
        Self::Uint(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            128,
        )
    }
}

// Signed elementary integer types
impl From<i8> for Parameter {
    #[inline]
    fn from(input: i8) -> Self {
        Self::Int(H256::from_slice(&left_pad_to_32_bytes(&[input as u8])), 8)
    }
}

impl From<i16> for Parameter {
    #[inline]
    fn from(input: i16) -> Self {
        Self::Int(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            16,
        )
    }
}

impl From<i32> for Parameter {
    #[inline]
    fn from(input: i32) -> Self {
        Self::Int(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            32,
        )
    }
}

impl From<i64> for Parameter {
    #[inline]
    fn from(input: i64) -> Self {
        Self::Int(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            64,
        )
    }
}

impl From<i128> for Parameter {
    #[inline]
    fn from(input: i128) -> Self {
        Self::Int(
            H256::from_slice(&left_pad_to_32_bytes(&input.to_be_bytes())),
            128,
        )
    }
}

// Boolean type
impl From<bool> for Parameter {
    #[inline]
    fn from(input: bool) -> Self {
        Self::Bool(H256::from_slice(&left_pad_to_32_bytes(&[u8::from(input)])))
    }
}

// String literal into a dynamic array of bytes
impl From<&str> for Parameter {
    #[inline]
    fn from(input: &str) -> Self {
        Self::String(input.as_bytes().to_vec())
    }
}

// From Ethereum types
impl From<Address> for Parameter {
    #[inline]
    fn from(input: Address) -> Self {
        Self::Address(H256::from_slice(&left_pad_to_32_bytes(&input.as_bytes())))
    }
}

impl From<U64> for Parameter {
    #[inline]
    fn from(input: U64) -> Self {
        let mut bytes = [0u8; 8];
        input.to_big_endian(&mut bytes);
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(&bytes)), 64)
    }
}

impl From<U128> for Parameter {
    #[inline]
    fn from(input: U128) -> Self {
        let mut bytes = [0u8; 16];
        input.to_big_endian(&mut bytes);
        Self::Uint(H256::from_slice(&left_pad_to_32_bytes(&bytes)), 128)
    }
}

impl From<U256> for Parameter {
    #[inline]
    fn from(input: U256) -> Self {
        let mut padded = [0u8; 32];
        input.to_big_endian(&mut padded);
        Self::Uint(H256::from_slice(&padded), 256)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hex_literal::hex;
    use std::str::FromStr;

    #[test]
    fn parameter_from_elementary_numeric_type() {
        // from u8
        let param = Parameter::from(17u8);
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[31] = 17;
            assert_eq!(len, 8);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From u8 test failed")
        }

        // from u16
        let param = Parameter::from(0b1110_0100_0000_0010_u16);
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[30..].copy_from_slice(&[228, 2]);
            assert_eq!(len, 16);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From u16 test failed")
        }

        // from u32
        let param = Parameter::from(0b1110_0100_0000_0010_u32);
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[30..].copy_from_slice(&[228, 2]);
            assert_eq!(len, 32);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From u16 test failed")
        }

        // from u64
        let param = Parameter::from(0b1110_0100_0000_0010_0000_0000_u64);
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[29..].copy_from_slice(&[228, 2, 0]);
            assert_eq!(len, 64);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From u16 test failed")
        }

        // from u128
        // this value starts with 1101_0000 ... then a lot of zeros
        let param = Parameter::from(276479423123262501563991868538311671808u128);
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[16] = 208;
            assert_eq!(len, 128);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From u128 test failed")
        }

        // from i8
        let param = Parameter::from(-123i8);
        if let Parameter::Int(value, len) = param {
            let mut expected = [0u8; 32];
            expected[31] = 133;
            assert_eq!(len, 8);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From i8 test failed")
        }

        // from i16
        let param = Parameter::from(-123i16);
        if let Parameter::Int(value, len) = param {
            let expected = hex!("000000000000000000000000000000000000000000000000000000000000ff85");
            assert_eq!(len, 16);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From i16 test failed")
        }

        // from i16
        let param = Parameter::from(-123i32);
        if let Parameter::Int(value, len) = param {
            let expected = hex!("00000000000000000000000000000000000000000000000000000000ffffff85");
            assert_eq!(len, 32);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From i32 test failed")
        }

        // from i64
        let param = Parameter::from(-12345678987654321_i64);
        if let Parameter::Int(value, len) = param {
            let expected = hex!("000000000000000000000000000000000000000000000000ffd423ab9d6e0b4f");
            assert_eq!(len, 64);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From i64 test failed")
        }

        // from i128
        let param = Parameter::from(-12345678987654321_i128);
        if let Parameter::Int(value, len) = param {
            let expected = hex!("00000000000000000000000000000000ffffffffffffffffffd423ab9d6e0b4f");
            assert_eq!(len, 128);
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From i128 test failed")
        }

        // from bool
        let param = Parameter::from(true);
        if let Parameter::Bool(value) = param {
            let mut expected = [0u8; 32];
            expected[31] = 1;
            assert_eq!(value, H256::from(expected));
        } else {
            panic!("From bool test failed")
        }
    }

    #[test]
    fn parameter_from_ethereum_types() {
        // from U64
        let param = Parameter::from(U64::zero());
        if let Parameter::Uint(value, len) = param {
            assert_eq!(len, 64);
            assert_eq!(value.to_fixed_bytes(), [0u8; 32]);
        } else {
            panic!("From U64 test failed")
        }

        // from U128
        let param = Parameter::from(U128::from_str("12345").unwrap());
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[28..].copy_from_slice(&0x12345u32.to_be_bytes());
            assert_eq!(len, 128);
            assert_eq!(value.to_fixed_bytes(), expected);
        } else {
            panic!("From U128 test failed")
        }

        // from U256
        let param = Parameter::from(U256::from_str("123456789").unwrap());
        if let Parameter::Uint(value, len) = param {
            let mut expected = [0u8; 32];
            expected[24..].copy_from_slice(&0x123456789u64.to_be_bytes());
            assert_eq!(len, 256);
            assert_eq!(value.to_fixed_bytes(), expected);
        } else {
            panic!("From U256 test failed")
        }

        // from Address
        let param = Parameter::from(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        if let Parameter::Address(value) = param {
            let expected = hex!("00000000000000000000000095eDA452256C1190947f9ba1fD19422f0120858a");
            assert_eq!(value.to_fixed_bytes(), expected);
        } else {
            panic!("From U256 test failed")
        }
    }

    #[test]
    #[rustfmt::skip]
    fn parameter_from_dynamic_types() {
        // From &str string literal
        let param = Parameter::from("Hello, world!");
        if let Parameter::String(value) = param {
            let expected = hex!("48656c6c6f2c20776f726c6421");
            assert_eq!(value, expected);
        } else {
            panic!("From &str test failed")
        }

        // From byte slice
        let bytes = [1u8; 43];
        let param = Parameter::new_bytes(&bytes[..]);
        if let Parameter::Bytes(value) = param {
            let expected = hex!(
            "01010101010101010101010101010101
                01010101010101010101010101010101
                0101010101010101010101");
            assert_eq!(value, expected);
        } else {
            panic!("From &[u8] test failed")
        }

        let bytes = [1u8; 43];
        let param = Parameter::new_fixed_bytes(&bytes[..]);
        if let Parameter::FixedBytes(value) = param {
            let expected = hex!(
            "01010101010101010101010101010101
                01010101010101010101010101010101
                0101010101010101010101");
            assert_eq!(value, expected);
        } else {
            panic!("From &[u8] test failed")
        }
    }

    #[test]
    fn parameter_from_fixed_bytes() {
        // Signed integer from fixed slice
        let param = Parameter::new_int([14; 32], true); // signed = true
        if let Parameter::Int(value, len) = param {
            assert_eq!(len, 256);
            assert_eq!(value.to_fixed_bytes(), [14u8; 32]);
        } else {
            panic!("Signed integer from fixed slice test failed");
        }

        // Unsigned integer from fixed slice
        let param = Parameter::new_int([12; 32], false); // signed = false
        if let Parameter::Uint(value, len) = param {
            assert_eq!(len, 256);
            assert_eq!(value.to_fixed_bytes(), [12u8; 32]);
        } else {
            panic!("Unsigned integer from fixed slice test failed");
        }
    }
}
