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
    pub fn parse(parsed_str: &str) -> Result<Self, String> {
        let result = match parsed_str {
            "address" => Self::Address,
            "bool" => Self::Bool,
            "bytes" => Self::Bytes,
            "string" => Self::String,
            param_type if param_type.starts_with("int") => {
                let len = usize::from_str_radix(&param_type[3..], 10)
                    .map_err(|e| format!("Failed to parse type: {}", e))?;
                Self::Int(len)
            }
            param_type if param_type.starts_with("uint") => {
                let len = usize::from_str_radix(&param_type[4..], 10)
                    .map_err(|e| format!("Failed to parse numeric data of type: {}", e))?;
                Self::Uint(len)
            }
            param_type if param_type.starts_with("bytes") => {
                let len = usize::from_str_radix(&param_type[5..], 10)
                    .map_err(|e| format!("Failed to parse numeric data of type: {}", e))?;
                Self::FixedBytes(len)
            }
            _ => return Err(format!("Invalid ABI type {}", parsed_str)),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::ParameterType;
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
    #[should_panic(expected = "Failed to parse numeric data of type")]
    fn invalid_abi_uint() {
        ParameterType::parse("uint2i6").unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to parse numeric data of type")]
    fn invalid_abi_int() {
        ParameterType::parse("uint2i6").unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to parse numeric data of type")]
    fn invalid_abi_bytes() {
        ParameterType::parse("bytes32x").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid ABI type")]
    fn invalid_abi_type() {
        ParameterType::parse("invalid_stuff").unwrap();
    }
}
