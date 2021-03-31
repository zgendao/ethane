mod decode;
mod encode;
mod parameter_type;

pub use parameter_type::ParameterType;

use byteorder::{BigEndian, ByteOrder};
use ethereum_types::{Address, U128, U256, U64};

/// ABI function input/output parameter.
///
/// It wraps the actual value as a slice of bytes.
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

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
