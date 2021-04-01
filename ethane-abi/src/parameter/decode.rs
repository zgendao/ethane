use crate::{Parameter, ParameterType};
use byteorder::{BigEndian, ByteOrder};
use ethereum_types::Address;
use std::convert::TryInto;
use std::str;

impl Parameter {
    pub fn decode(param_type: &ParameterType, raw_bytes: &[u8]) -> (Self, usize) {
        // TODO validate raw_bytes length
        match *param_type {
            ParameterType::Address => (
                Self::Address(Address::from_slice(remove_left_padding_bytes(
                    12,
                    &raw_bytes[..32],
                ))),
                32,
            ),
            ParameterType::Bool => (Self::Bool(raw_bytes[31] == 1), 32),
            ParameterType::Int(length) => {
                let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                match length {
                    8 => (
                        Self::Int8(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    16 => (
                        Self::Int16(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    32 => (
                        Self::Int32(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    64 => (
                        Self::Int64(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    128 => (
                        Self::Int128(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    256 => (
                        Self::Int256(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    _ => unimplemented!(),
                }
            }
            ParameterType::Uint(length) => {
                let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                match length {
                    8 => (
                        Self::Uint8(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    16 => (
                        Self::Uint16(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    32 => (
                        Self::Uint32(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    64 => (
                        Self::Uint64(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    128 => (
                        Self::Uint128(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    256 => (
                        Self::Uint256(cleaned.try_into().expect("input with incorrect length")),
                        32,
                    ),
                    _ => unimplemented!(),
                }
            }
            ParameterType::Bytes => {
                let length = BigEndian::read_u64(&raw_bytes[..32]) as usize;
                (
                    Self::Bytes(raw_bytes[12..length].to_vec()),
                    32 + length + get_right_padding_length(length),
                )
            }
            ParameterType::String => {
                let length = BigEndian::read_u64(&raw_bytes[..32]) as usize;
                (
                    Self::String(str::from_utf8(&raw_bytes[12..length]).unwrap().to_string()),
                    32 + length + get_right_padding_length(length),
                )
            }
            ParameterType::FixedBytes(length) => {
                (Self::FixedBytes(raw_bytes[..length].to_vec()), 32)
            }
        }
    }
}

pub fn remove_left_padding_bytes(pad_length: usize, value: &[u8]) -> &[u8] {
    &value[pad_length..]
}

pub fn get_right_padding_length(mut length: usize) -> usize {
    let mut res: usize = 0;
    while length % 32 != 0 {
        length += 1;
        res += 1;
    }
    res
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_decode_address() {
        let encoded_address =
            hex!("00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a1234");
        let decoded = Parameter::decode(&ParameterType::Address, &encoded_address);

        let expected = Parameter::Address(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u8() {
        let encoded = hex!("000000000000000000000000000000000000000000000000000000000000007F");
        let decoded = Parameter::decode(&ParameterType::Uint(8), &encoded);

        let expected = Parameter::from_u8(127);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u16() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000000FF");
        let decoded = Parameter::decode(&ParameterType::Uint(16), &encoded);

        let expected = Parameter::from_u16(255);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u32() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Uint(32), &encoded);

        let expected = Parameter::from_u32(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u64() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Uint(64), &encoded);

        let expected = Parameter::from_u64(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i8() {
        let encoded = hex!("000000000000000000000000000000000000000000000000000000000000007F");
        let decoded = Parameter::decode(&ParameterType::Int(8), &encoded);

        let expected = Parameter::from_i8(127);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i16() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000000FF");
        let decoded = Parameter::decode(&ParameterType::Int(16), &encoded);

        let expected = Parameter::from_i16(255);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i32() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Int(32), &encoded);

        let expected = Parameter::from_i32(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i64() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Int(64), &encoded);

        let expected = Parameter::from_i64(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }
}
