use super::tmp::Parameter;
use crate::AbiParserError;

/// ABI function input/output parameter type.
#[derive(Debug, PartialEq)]
pub enum ParameterType {
    /// A 160 bit (20 bytes) unsigned integer.
    Address,
    /// A simple boolean with its value restricted to 0 or 1.
    Bool,
    /// A dynamic sequence of bytes.
    Bytes,
    /// A static sequence of bytes with `n` elements.
    FixedBytes(usize),
    /// Contains an address (20 bytes) followed by a function selector (4
    /// bytes).
    ///
    /// Encoded the same way as a [`FixedBytes`] parameter containing 24 bytes.
    Function,
    /// A generic `n` bit signed integer type.
    Int(usize),
    /// A generic `n` bit unsigned integer type.
    Uint(usize),
    /// A generic string encoded to a sequence of `UTF-8` bytes.
    String,
    /// A dynamic array holding the same, arbitrary type.
    Array(Box<ParameterType>),
    /// A fixed length array, with length `n`, holding the same, arbitrary type.
    FixedArray(Box<ParameterType>, usize),
    /// A tuple holding a sequence of various arbitrary types.
    Tuple(Vec<ParameterType>),
}

impl ParameterType {
    pub fn parse(parsed_str: &str) -> Result<Self, AbiParserError> {
        let result = match parsed_str {
            "address" => Self::Address,
            "bool" => Self::Bool,
            "bytes" => Self::Bytes,
            "string" => Self::String,
            param_type if param_type.starts_with("int") => {
                let len = usize::from_str_radix(&param_type[3..], 10).map_err(|e| {
                    AbiParserError::InvalidAbiEncoding(format!("{}: {}", parsed_str, e))
                })?;
                Self::Int(len)
            }
            param_type if param_type.starts_with("uint") => {
                let len = usize::from_str_radix(&param_type[4..], 10).map_err(|e| {
                    AbiParserError::InvalidAbiEncoding(format!("{}: {}", parsed_str, e))
                })?;
                Self::Uint(len)
            }
            param_type if param_type.starts_with("bytes") => {
                let len = usize::from_str_radix(&param_type[5..], 10).map_err(|e| {
                    AbiParserError::InvalidAbiEncoding(format!("{}: {}", parsed_str, e))
                })?;
                Self::FixedBytes(len)
            }
            _ => return Err(AbiParserError::InvalidAbiEncoding(parsed_str.to_string())),
        };

        Ok(result)
    }

    pub fn as_abi_string(&self) -> String {
        match self {
            Self::Address => "address".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Bytes => "bytes".to_owned(),
            Self::FixedBytes(len) => format!("bytes{}", len),
            Self::Int(len) => format!("int{}", len),
            Self::Uint(len) => format!("uint{}", len),
            Self::String => "string".to_owned(),
            _ => unimplemented!(),
        }
    }

    /// Recursively checks wether a given type is dynamic.
    ///
    /// For example, a [`Tuple`] can be dynamic if any of its contained types
    /// are dynamic. Additionally, a [`FixedArray`] is static if it contains
    /// values with static type and dynamic otherwise.
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Array(_) | Self::Bytes | Self::String => true,
            Self::FixedArray(parameter_type, _) => parameter_type.is_dynamic(),
            Self::Tuple(value) => value.iter().any(|x| x.is_dynamic()),
            _ => false,
        }
    }

    /// Recursively checks whether the given [`Parameter`] data matches the expected
    /// [`ParameterType`].
    #[rustfmt::skip]
    pub fn type_check(&self, parameter: &Parameter) -> bool {
        match self {
            Self::Address => if let Parameter::Address(_) = parameter { return true },
            Self::Bool => if let Parameter::Bool(_) = parameter { return true },
            Self::Bytes => if let Parameter::Bytes(_) = parameter { return true },
            Self::FixedBytes(len) => if let Parameter::Bytes(data) = parameter { return data.len() == *len },
            Self::Function => if let Parameter::Bytes(data) = parameter { return data.len() == 24 },
            Self::Int(len) => if let Parameter::Int(_, data_len) = parameter { return  data_len == len },
            Self::Uint(len) => if let Parameter::Uint(_, data_len) = parameter { return data_len == len },
            Self::String => if let Parameter::String(_) = parameter { return true },
            Self::Array(param_type) => if let Parameter::Array(data) = parameter {
                return data.iter().all(|d| param_type.type_check(&d))
            }
            Self::FixedArray(param_type, len) => if let Parameter::Array(data) = parameter {
                return data.iter().all(|d| param_type.type_check(&d)) && data.len() == *len
            }
            Self::Tuple(param_type) => if let Parameter::Tuple(data) = parameter {
                return data.iter().zip(param_type.iter()).all(|(d, t)| t.type_check(&d))
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ethereum_types::{Address, H256, U256};
    use std::mem::discriminant;

    #[test]
    fn parse_parameter_type() {
        assert_eq!(
            discriminant(&ParameterType::parse("address").unwrap()),
            discriminant(&ParameterType::Address)
        );
        assert_eq!(
            discriminant(&ParameterType::parse("bool").unwrap()),
            discriminant(&ParameterType::Bool)
        );
        assert_eq!(
            discriminant(&ParameterType::parse("bytes").unwrap()),
            discriminant(&ParameterType::Bytes)
        );
        assert_eq!(
            discriminant(&ParameterType::parse("string").unwrap()),
            discriminant(&ParameterType::String)
        );

        match ParameterType::parse("uint16").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 16),
            _ => panic!("Error while parsing uint16"),
        }

        match ParameterType::parse("uint64").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 64),
            _ => panic!("Error while parsing uint64"),
        }

        match ParameterType::parse("uint256").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 256),
            _ => panic!("Error while parsing uint256"),
        }

        match ParameterType::parse("int256").unwrap() {
            ParameterType::Int(a) => assert_eq!(a, 256),
            _ => panic!("Error while parsing int256"),
        }

        match ParameterType::parse("bytes32").unwrap() {
            ParameterType::FixedBytes(a) => assert_eq!(a, 32),
            _ => panic!("Error while parsing bytes32"),
        }
    }

    #[test]
    fn parameter_type_as_string() {
        assert_eq!(&ParameterType::Address.as_abi_string(), "address");
        assert_eq!(&ParameterType::Bool.as_abi_string(), "bool");
        assert_eq!(&ParameterType::Bytes.as_abi_string(), "bytes");
        assert_eq!(&ParameterType::String.as_abi_string(), "string");
        assert_eq!(&ParameterType::FixedBytes(32).as_abi_string(), "bytes32");
        assert_eq!(&ParameterType::Uint(256).as_abi_string(), "uint256");
        assert_eq!(&ParameterType::Int(128).as_abi_string(), "int128");
    }

    #[test]
    fn invalid_abi_uint() {
        match ParameterType::parse("uint2i6") {
            Err(AbiParserError::InvalidAbiEncoding(e)) => {
                assert!(e.starts_with("uint2i6: invalid digit"))
            }
            _ => panic!("This test failed!"),
        }
    }

    #[test]
    fn invalid_abi_int() {
        match ParameterType::parse("int2i6") {
            Err(AbiParserError::InvalidAbiEncoding(e)) => {
                assert!(e.starts_with("int2i6: invalid digit"))
            }
            _ => panic!("This test failed!"),
        }
    }

    #[test]
    fn invalid_abi_bytes() {
        match ParameterType::parse("bytes32x") {
            Err(AbiParserError::InvalidAbiEncoding(e)) => {
                assert!(e.starts_with("bytes32x: invalid digit"))
            }
            _ => panic!("This test failed!"),
        }
    }

    #[test]
    fn invalid_abi_type() {
        match ParameterType::parse("invalid_type") {
            Err(AbiParserError::InvalidAbiEncoding(e)) => assert!(e.starts_with("invalid_type")),
            _ => panic!("This test failed!"),
        }
    }

    #[test]
    fn type_is_dynamic() {
        assert!(!ParameterType::Address.is_dynamic());
        assert!(!ParameterType::Bool.is_dynamic());
        assert!(ParameterType::Bytes.is_dynamic());
        assert!(!ParameterType::FixedBytes(128).is_dynamic());
        assert!(!ParameterType::Function.is_dynamic());
        assert!(!ParameterType::Uint(32).is_dynamic());
        assert!(!ParameterType::Int(256).is_dynamic());
        assert!(ParameterType::String.is_dynamic());
        assert!(ParameterType::Array(Box::new(ParameterType::Address)).is_dynamic());
        assert!(ParameterType::Array(Box::new(ParameterType::Bytes)).is_dynamic());
        assert!(!ParameterType::FixedArray(Box::new(ParameterType::Function), 3).is_dynamic());
        assert!(ParameterType::FixedArray(Box::new(ParameterType::String), 2).is_dynamic());
        assert!(!ParameterType::Tuple(vec![
            ParameterType::Function,
            ParameterType::Uint(32),
            ParameterType::FixedBytes(64)
        ])
        .is_dynamic());
        assert!(ParameterType::Tuple(vec![
            ParameterType::Function,
            ParameterType::Uint(32),
            ParameterType::String
        ])
        .is_dynamic());
        assert!(!ParameterType::FixedArray(
            Box::new(ParameterType::FixedArray(
                Box::new(ParameterType::Int(8)),
                5
            )),
            2
        )
        .is_dynamic());
        assert!(ParameterType::Tuple(vec![
            ParameterType::Function,
            ParameterType::Uint(32),
            ParameterType::FixedArray(Box::new(ParameterType::String), 3)
        ])
        .is_dynamic());
    }

    #[test]
    fn parameter_type_check() {
        assert!(ParameterType::Address.type_check(&Parameter::Address(H256::zero())));
        assert!(!ParameterType::Address.type_check(&Parameter::Uint(H256::zero(), 8)));

        assert!(ParameterType::Bool.type_check(&Parameter::Bool(H256::zero())));
        assert!(!ParameterType::Bool.type_check(&Parameter::Bytes(Vec::new())));

        assert!(ParameterType::Bytes.type_check(&Parameter::Bytes(Vec::new())));
        assert!(!ParameterType::Bytes.type_check(&Parameter::String(Vec::new())));

        assert!(ParameterType::FixedBytes(16).type_check(&Parameter::Bytes(vec![14u8; 16])));
        assert!(!ParameterType::FixedBytes(16).type_check(&Parameter::Bytes(vec![14u8; 32])));
        assert!(!ParameterType::FixedBytes(16)
            .type_check(&Parameter::Array(vec![Parameter::String(Vec::new())])));

        assert!(ParameterType::Function.type_check(&Parameter::Bytes(vec![14u8; 24])));
        assert!(!ParameterType::Function.type_check(&Parameter::Bytes(vec![14u8; 25])));
        assert!(!ParameterType::Function.type_check(&Parameter::Address(H256::zero())));

        assert!(ParameterType::Uint(128).type_check(&Parameter::Uint(H256::zero(), 128)));
        assert!(!ParameterType::Uint(128).type_check(&Parameter::Uint(H256::zero(), 32)));
        assert!(!ParameterType::Uint(128).type_check(&Parameter::Int(H256::zero(), 128)));

        assert!(ParameterType::Int(128).type_check(&Parameter::Int(H256::zero(), 128)));
        assert!(!ParameterType::Int(128).type_check(&Parameter::Int(H256::zero(), 32)));
        assert!(!ParameterType::Int(128).type_check(&Parameter::Uint(H256::zero(), 128)));

        assert!(ParameterType::String.type_check(&Parameter::String(Vec::new())));
        assert!(!ParameterType::String.type_check(&Parameter::Bool(H256::zero())));

        assert!(ParameterType::FixedArray(
            Box::new(ParameterType::Array(Box::new(ParameterType::Uint(8)))),
            7
        )
        .type_check(&Parameter::Array(vec![
            Parameter::Array(vec![
                Parameter::Uint(H256::zero(), 8),
                Parameter::Uint(H256::zero(), 8),
            ]);
            7
        ])));
        assert!(ParameterType::FixedArray(Box::new(ParameterType::Bool), 10)
            .type_check(&Parameter::Array(vec![Parameter::from(true); 10])));
        assert!(
            !ParameterType::FixedArray(Box::new(ParameterType::Bool), 10)
                .type_check(&Parameter::Array(vec![Parameter::from(true); 7]))
        );

        assert!(ParameterType::Array(Box::new(ParameterType::Bool))
            .type_check(&Parameter::Array(vec![Parameter::from(true); 7])));
        assert!(ParameterType::Array(Box::new(ParameterType::Bool))
            .type_check(&Parameter::Array(vec![Parameter::from(false); 2])));

        assert!(ParameterType::Tuple(vec![
            ParameterType::Address,
            ParameterType::Uint(256),
            ParameterType::FixedArray(Box::new(ParameterType::Address), 5),
            ParameterType::String,
        ])
        .type_check(&Parameter::Tuple(vec![
            Parameter::from(Address::zero()),
            Parameter::from(U256::zero()),
            Parameter::Array(vec![Parameter::from(Address::zero()); 5]),
            Parameter::String(Vec::new()),
        ])));
    }
}
