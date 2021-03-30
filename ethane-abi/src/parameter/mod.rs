pub mod parameter_type;

use byteorder::{BigEndian, ByteOrder};
use ethereum_types::{Address, U128, U256, U64};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Parameter {
    Address(Address),
    Uint8([u8; 1]),
    Uint16([u8; 2]),
    Uint32([u8; 4]),
    Uint64(U64),
    Uint128(U128),
    Uint256(U256),
    Int8([u8; 1]),
    Int16([u8; 2]),
    Int32([u8; 4]),
    Int64(U64),
    Int128(U128),
    Int256(U256),
    Bool(bool),
    // Vector of bytes with known size.
    FixedBytes(Vec<u8>),
    // dynamic sized byte sequence
    Bytes(Vec<u8>),
    // dynamic sized unicode string assumed to be UTF-8 encoded.
    String(String),
}

#[allow(dead_code)]
impl Parameter {
    pub fn encode(&self) -> Value {
        match self {
            Parameter::Address(address) => Value::from(address.as_bytes()),
            Parameter::Uint8(val) | Parameter::Int8(val) => {
                Value::from(left_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Uint16(val) | Parameter::Int16(val) => {
                Value::from(left_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Uint32(val) | Parameter::Int32(val) => {
                Value::from(left_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Uint64(val) | Parameter::Int64(val) => {
                let mut padded: [u8; 8] = [0u8; 8];
                val.to_big_endian(&mut padded);
                let padded = left_pad_to_32_bytes(&padded);
                Value::from(padded.to_vec())
            }
            Parameter::Uint128(val) | Parameter::Int128(val) => {
                let mut padded: [u8; 16] = [0u8; 16];
                val.to_big_endian(&mut padded);
                let padded = left_pad_to_32_bytes(&padded);
                Value::from(padded.to_vec())
            }
            Parameter::Uint256(val) | Parameter::Int256(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                val.to_big_endian(&mut padded);
                Value::from(padded.to_vec())
            }
            Parameter::Bool(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                if *val {
                    padded[31] = 1;
                }
                Value::from(padded.to_vec())
            }
            Parameter::FixedBytes(val) => Value::from(right_pad_to_32_bytes(val).to_vec()),
            Parameter::Bytes(bytes) => {
                let mut res: Vec<u8> = vec![];
                // number of bytes is encoded as a uint256
                let length: [u8; 32] = left_pad_to_32_bytes(&bytes.len().to_be_bytes());
                res.extend_from_slice(&length);
                res.extend_from_slice(right_pad_bytes(&bytes).as_slice());
                Value::from(res)
            }
            Parameter::String(val) => Parameter::Bytes(Vec::from(val.as_bytes())).encode(),
        }
    }

    pub fn from_u8(param: u8) -> Self {
        Self::Uint8([param])
    }

    pub fn to_u8(&self) -> Option<u8> {
        match self {
            Parameter::Uint8(val) => Some(val[0]),
            _ => None,
        }
    }

    pub fn from_u16(param: u16) -> Self {
        Self::Uint16(param.to_be_bytes())
    }

    pub fn to_u16(&self) -> Option<u16> {
        match self {
            Parameter::Uint16(val) => Some(BigEndian::read_u16(val)),
            _ => None,
        }
    }

    pub fn from_u32(param: u32) -> Self {
        Self::Uint32(param.to_be_bytes())
    }

    pub fn to_u32(&self) -> Option<u32> {
        match self {
            Parameter::Uint32(val) => Some(BigEndian::read_u32(val)),
            _ => None,
        }
    }

    pub fn from_u64(param: u64) -> Self {
        Self::Uint64(U64::from(param))
    }

    pub fn to_u64(&self) -> Option<u64> {
        match self {
            Parameter::Uint64(val) => Some(val.as_u64()),
            _ => None,
        }
    }

    pub fn from_i8(param: i8) -> Self {
        Self::Int8([param as u8])
    }

    pub fn to_i8(&self) -> Option<i8> {
        match self {
            Parameter::Int8(val) => Some(val[0] as i8),
            _ => None,
        }
    }

    pub fn from_i16(param: i16) -> Self {
        Self::Int16(param.to_be_bytes())
    }

    pub fn to_i16(&self) -> Option<i16> {
        match self {
            Parameter::Int16(val) => Some(BigEndian::read_i16(val)),
            _ => None,
        }
    }

    pub fn from_i32(param: i32) -> Self {
        Self::Int32(param.to_be_bytes())
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Parameter::Int32(val) => Some(BigEndian::read_i32(val)),
            _ => None,
        }
    }

    pub fn from_i64(param: i64) -> Self {
        Self::Int64(U64::from(param))
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Parameter::Int64(val) => Some(val.as_u64() as i64),
            _ => None,
        }
    }

    // pub fn decode(raw: Vec<u8>,p: ParameterType) -> Self {
    //     match p {
    //         ParameterType::Address(_) => ParameterType::Address(Address::from_slice(remove_padding_bytes(12,&raw))),
    //         ParameterType::Uint256(_) => ParameterType::Uint256(U256::from_big_endian(remove_padding_bytes(0,&raw))),
    //         ParameterType::Uint128(_) => ParameterType::Uint128(U128::from_big_endian(remove_padding_bytes(16,&raw))),
    //     }
    // }
}

pub fn left_pad_to_32_bytes(value: &[u8]) -> [u8; 32] {
    assert!(
        value.len() <= 32,
        format!(
            "Cannot pad to 32 bytes, input is too long ({} bytes)",
            value.len()
        )
    );
    let mut padded = [0u8; 32];
    let diff = 32 - value.len();
    padded[diff..].copy_from_slice(value);
    padded
}

pub fn right_pad_to_32_bytes(value: &[u8]) -> [u8; 32] {
    assert!(
        value.len() <= 32,
        format!(
            "Cannot pad to 32 bytes, input is too long ({} bytes)",
            value.len()
        )
    );
    let mut padded = [0u8; 32];
    let diff = 32 - value.len();
    padded[..(32 - diff)].copy_from_slice(value);
    padded
}

pub fn right_pad_bytes(value: &[u8]) -> Vec<u8> {
    let mut length = value.len();
    while length % 32 != 0 {
        length += 1;
    }
    let mut padded: Vec<u8> = vec![0; length];
    let diff = length - value.len();
    padded[..(length - diff)].copy_from_slice(value);
    padded
}

#[allow(dead_code)]
pub fn remove_padding_bytes(pad_length: usize, value: &[u8]) -> &[u8] {
    &value[pad_length..]
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_encode_address() {
        let address = Parameter::Address(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        let encoded = address.encode();
        let hex_val = hex!("95eDA452256C1190947f9ba1fD19422f0120858a");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u8() {
        let uint8 = Parameter::from_u8(11);
        let encoded = uint8.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000000B");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u16() {
        let uint16 = Parameter::from_u16(123);
        let encoded = uint16.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000007B");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u32() {
        let uint32 = Parameter::from_u16(65535);
        let encoded = uint32.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000FFFF");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u64() {
        let uint64 = Parameter::from_u64(18_446_744_073_709_551_615);
        let encoded = uint64.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000FFFFFFFFFFFFFFFF");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u128() {
        let uint256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u256() {
        let uint256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i8() {
        let int8 = Parameter::from_i8(11);
        let encoded = int8.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000000B");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i16() {
        let int16 = Parameter::from_i16(123);
        let encoded = int16.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000007B");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i32() {
        let int32 = Parameter::from_i16(32767);
        let encoded = int32.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000007FFF");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i64() {
        let int64 = Parameter::from_i64(9_223_372_036_854_775_807);
        let encoded = int64.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000007FFFFFFFFFFFFFFF");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i128() {
        let int256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = int256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i256() {
        let int256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = int256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_string() {
        let str = Parameter::String(String::from("AAAA"));
        let encoded = str.encode();
        println!("{:?}",encoded);

        let hex_val = hex!("00000000000000000000000000000000000000000000000000000000000000044141414100000000000000000000000000000000000000000000000000000000");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }
    //
    // #[test]
    // fn decode_u128() {
    //     let uint256 = ParameterType::Uint128(U128::from_str("1555").unwrap());
    //     let encoded = uint256.encode();
    //     let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
    //     let expected = Value::from(hex_val.to_vec());
    //
    //     let p = ParameterType::decode(hex_val.to_vec(),ParameterType::Uint128(U128::zero()));
    //     println!("{:?}",p)
    // }
    #[test]
    fn decode_u128() {}
}
