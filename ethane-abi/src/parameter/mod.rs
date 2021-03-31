mod parameter_type;
mod decode;

pub use parameter_type::ParameterType;

use byteorder::{BigEndian, ByteOrder};
use ethereum_types::{Address, U128, U256, U64};

#[derive(Debug,PartialEq)]
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

impl Parameter {
    // NOTE this should return a Vec instead of a serde_json::Value.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Parameter::Address(address) => left_pad_to_32_bytes(address.as_bytes()).to_vec(),
            Parameter::Uint8(val) | Parameter::Int8(val) => left_pad_to_32_bytes(val).to_vec(),
            Parameter::Uint16(val) | Parameter::Int16(val) => left_pad_to_32_bytes(val).to_vec(),
            Parameter::Uint32(val) | Parameter::Int32(val) => left_pad_to_32_bytes(val).to_vec(),
            Parameter::Uint64(val) | Parameter::Int64(val) => {
                let mut padded: [u8; 8] = [0u8; 8];
                val.to_big_endian(&mut padded);
                left_pad_to_32_bytes(&padded).to_vec()
            }
            Parameter::Uint128(val) | Parameter::Int128(val) => {
                let mut padded: [u8; 16] = [0u8; 16];
                val.to_big_endian(&mut padded);
                left_pad_to_32_bytes(&padded).to_vec()
            }
            Parameter::Uint256(val) | Parameter::Int256(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                val.to_big_endian(&mut padded);
                padded.to_vec()
            }
            Parameter::Bool(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                if *val {
                    padded[31] = 1;
                }
                padded.to_vec()
            }
            Parameter::FixedBytes(val) => right_pad_to_32_bytes(val).to_vec(),
            Parameter::Bytes(bytes) => {
                let mut res: Vec<u8> = vec![];
                // number of bytes is encoded as a uint256
                let length: [u8; 32] = left_pad_to_32_bytes(&bytes.len().to_be_bytes());
                res.extend_from_slice(&length);
                res.extend_from_slice(right_pad_bytes(&bytes).as_slice());
                res
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

    pub fn get_type(&self) -> ParameterType {
        match self {
            Self::Address(_) => ParameterType::Address,
            Self::Bool(_) => ParameterType::Bool,
            Self::Bytes(_) => ParameterType::Bytes,
            Self::FixedBytes(data) => ParameterType::FixedBytes(data.len()),
            Self::String(_) => ParameterType::String,
            Self::Uint8(_) => ParameterType::Uint(8),
            Self::Uint16(_) => ParameterType::Uint(16),
            Self::Uint32(_) => ParameterType::Uint(32),
            Self::Uint64(_) => ParameterType::Uint(64),
            Self::Uint128(_) => ParameterType::Uint(128),
            Self::Uint256(_) => ParameterType::Uint(256),
            Self::Int8(_) => ParameterType::Int(8),
            Self::Int16(_) => ParameterType::Int(16),
            Self::Int32(_) => ParameterType::Int(32),
            Self::Int64(_) => ParameterType::Int(64),
            Self::Int128(_) => ParameterType::Int(128),
            Self::Int256(_) => ParameterType::Int(256),
        }
    }
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
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u8() {
        let uint8 = Parameter::from_u8(11);
        let encoded = uint8.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000000B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u16() {
        let uint16 = Parameter::from_u16(123);
        let encoded = uint16.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000007B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u32() {
        let uint32 = Parameter::from_u16(65535);
        let encoded = uint32.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000FFFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u64() {
        let uint64 = Parameter::from_u64(18_446_744_073_709_551_615);
        let encoded = uint64.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000FFFFFFFFFFFFFFFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u128() {
        let uint256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u256() {
        let uint256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i8() {
        let int8 = Parameter::from_i8(11);
        let encoded = int8.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000000B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i16() {
        let int16 = Parameter::from_i16(123);
        let encoded = int16.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000007B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i32() {
        let int32 = Parameter::from_i16(32767);
        let encoded = int32.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000007FFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i64() {
        let int64 = Parameter::from_i64(9_223_372_036_854_775_807);
        let encoded = int64.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000007FFFFFFFFFFFFFFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i128() {
        let int256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = int256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i256() {
        let int256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = int256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_string() {
        let str = Parameter::String(String::from("AAAA"));
        let encoded = str.encode();
        println!("{:?}", encoded);

        let hex_val = hex!("00000000000000000000000000000000000000000000000000000000000000044141414100000000000000000000000000000000000000000000000000000000");
        let expected = hex_val.to_vec();
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

    #[test]
    fn correct_parameter_types() {
        assert_eq!(
            Parameter::Address(Address::zero()).get_type(),
            ParameterType::Address
        );
        assert_eq!(Parameter::Bool(true).get_type(), ParameterType::Bool);
        assert_eq!(
            Parameter::Bytes(Vec::new()).get_type(),
            ParameterType::Bytes
        );
        assert_eq!(
            Parameter::FixedBytes(vec![0; 32]).get_type(),
            ParameterType::FixedBytes(32)
        );
        assert_eq!(
            Parameter::String("hello".to_owned()).get_type(),
            ParameterType::String
        );
        assert_eq!(
            Parameter::Uint8([0u8; 1]).get_type(),
            ParameterType::Uint(8)
        );
        assert_eq!(
            Parameter::Uint16([0u8; 2]).get_type(),
            ParameterType::Uint(16)
        );
        assert_eq!(
            Parameter::Uint32([0u8; 4]).get_type(),
            ParameterType::Uint(32)
        );
        assert_eq!(
            Parameter::Uint64(U64::zero()).get_type(),
            ParameterType::Uint(64)
        );
        assert_eq!(
            Parameter::Uint128(U128::zero()).get_type(),
            ParameterType::Uint(128)
        );
        assert_eq!(
            Parameter::Uint256(U256::zero()).get_type(),
            ParameterType::Uint(256)
        );
        assert_eq!(Parameter::Int8([0u8; 1]).get_type(), ParameterType::Int(8));
        assert_eq!(
            Parameter::Int16([0u8; 2]).get_type(),
            ParameterType::Int(16)
        );
        assert_eq!(
            Parameter::Int32([0u8; 4]).get_type(),
            ParameterType::Int(32)
        );
        assert_eq!(
            Parameter::Int64(U64::zero()).get_type(),
            ParameterType::Int(64)
        );
        assert_eq!(
            Parameter::Int128(U128::zero()).get_type(),
            ParameterType::Int(128)
        );
        assert_eq!(
            Parameter::Int256(U256::zero()).get_type(),
            ParameterType::Int(256)
        );
    }
}
