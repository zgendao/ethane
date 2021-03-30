use crate::AbiParserError;

pub enum ParameterType {
    Address,
    Bool,
    Bytes,
    FixedBytes(usize),
    Int(usize),
    Uint(usize),
    String,
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
            _ => {
                return Err(AbiParserError::InvalidAbiEncoding(format!(
                    "{}",
                    parsed_str.to_owned()
                )))
            }
        };

        Ok(result)
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::Address => "address".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Bytes => "bytes".to_owned(),
            Self::FixedBytes(len) => format!("bytes{}", len),
            Self::Int(len) => format!("int{}", len),
            Self::Uint(len) => format!("uint{}", len),
            Self::String => "string".to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        assert_eq!(&ParameterType::Address.as_string(), "address");
        assert_eq!(&ParameterType::Bool.as_string(), "bool");
        assert_eq!(&ParameterType::Bytes.as_string(), "bytes");
        assert_eq!(&ParameterType::String.as_string(), "string");
        assert_eq!(&ParameterType::FixedBytes(32).as_string(), "bytes32");
        assert_eq!(&ParameterType::Uint(256).as_string(), "uint256");
        assert_eq!(&ParameterType::Int(128).as_string(), "int128");
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
}
