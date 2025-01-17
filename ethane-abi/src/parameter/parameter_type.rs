use super::Parameter;
use crate::AbiParserError;

/// ABI function input/output parameter type.
#[derive(Debug, PartialEq)]
pub enum ParameterType {
    // TODO make this public only on crate level
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
    /// Encoded the same way as a [`FixedBytes`](Parameter::FixedBytes) parameter
    /// containing 24 bytes.
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
    /// Parses a [`ParameterType`] from a string literal based on the
    /// Solidity/ABI type syntax.
    ///
    /// It only supports tuples enclosing at least 2 different types. Thus,
    /// tuples like (T) and () are not supported.
    pub fn parse(parsed_str: &str) -> Result<Self, AbiParserError> {
        let len = parsed_str.len();
        // TODO how can tuple parsing be better?
        if parsed_str.ends_with(')') {
            // we have a tuple
            let mut parameter_type_vec = Vec::<ParameterType>::new();
            // throw away the parentheses and split arguments
            let internal_types = parsed_str[1..len - 1].split(',').collect::<Vec<&str>>();
            let mut start_index = 0;
            let mut parsing_nested_tuple = false;
            for (end_index, t) in internal_types.iter().enumerate() {
                if t.contains('(') {
                    // found the start of a nested tuple
                    start_index = end_index;
                    parsing_nested_tuple = true;
                } else if t.contains(')') {
                    // found the end of the nested tuple
                    // join everything between the nested parentheses, e.g.
                    // [(bytes32, bool, string)].join(',') -> "(bytes32,bool,string)"
                    parameter_type_vec.push(Self::parse(
                        &internal_types[start_index..end_index + 1].join(","),
                    )?);
                    parsing_nested_tuple = false;
                } else if !parsing_nested_tuple {
                    parameter_type_vec.push(Self::parse(t)?);
                }
            }

            Ok(Self::Tuple(parameter_type_vec))
        } else if parsed_str.ends_with(']') {
            // we have an array
            let tokens = parsed_str.split('[').collect::<Vec<&str>>();
            let mut array_type = Self::parse(tokens[0])?; // first token is the internal type name of the array
            for t in &tokens[1..] {
                // iterate over the array lengths (if any), starting from the back
                let trimmed = t.trim_end_matches(']');
                if trimmed.is_empty() {
                    array_type = Self::Array(Box::new(array_type));
                } else {
                    array_type = Self::FixedArray(
                        Box::new(array_type),
                        trimmed.parse().map_err(|e| {
                            AbiParserError::InvalidAbiEncoding(format!("{}, {}", e, parsed_str))
                        })?,
                    );
                }
            }

            Ok(array_type)
        } else {
            let result = match parsed_str {
                // we have an elementary type
                "address" => Self::Address,
                "bool" => Self::Bool,
                "bytes" => Self::Bytes,
                "int" => Self::Int(256),
                "uint" => Self::Uint(256),
                "string" => Self::String,
                param_type if param_type.starts_with("int") => {
                    let len = (&param_type[3..]).parse::<usize>().map_err(|e| {
                        AbiParserError::InvalidAbiEncoding(format!("{}: {}", parsed_str, e))
                    })?;
                    Self::Int(len)
                }
                param_type if param_type.starts_with("uint") => {
                    let len = (&param_type[4..]).parse::<usize>().map_err(|e| {
                        AbiParserError::InvalidAbiEncoding(format!("{}: {}", parsed_str, e))
                    })?;
                    Self::Uint(len)
                }
                param_type if param_type.starts_with("bytes") => {
                    let len = (&param_type[5..]).parse::<usize>().map_err(|e| {
                        AbiParserError::InvalidAbiEncoding(format!("{}: {}", parsed_str, e))
                    })?;
                    Self::FixedBytes(len)
                }
                _ => return Err(AbiParserError::InvalidAbiEncoding(parsed_str.to_string())),
            };

            Ok(result)
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Array(_) | Self::Bytes | Self::String => true,
            Self::FixedArray(parameter_type, _) => parameter_type.is_dynamic(),
            Self::Tuple(value) => value.iter().any(|x| x.is_dynamic()),
            _ => false,
        }
    }

    /// Generates an ABI contract string representation of the underlying type.
    pub fn as_abi_string(&self) -> String {
        match self {
            Self::Address => "address".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Bytes => "bytes".to_owned(),
            Self::FixedBytes(len) => format!("bytes{}", len),
            Self::Function => "function".to_owned(),
            Self::Int(len) => format!("int{}", len),
            Self::Uint(len) => format!("uint{}", len),
            Self::String => "string".to_owned(),
            Self::Array(param_type) => param_type.as_abi_string() + "[]",
            Self::FixedArray(param_type, len) => param_type.as_abi_string() + &format!("[{}]", len),
            Self::Tuple(param_types) => {
                let mut abi_string = String::from("(");
                for pt in param_types {
                    abi_string += &(pt.as_abi_string() + ",");
                }
                // TODO could this be more elegant? maybe use .join(',') on a Vec<String>?
                abi_string.pop(); // pop last added comma
                abi_string + ")"
            }
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
            Self::FixedBytes(len) => if let Parameter::FixedBytes(data) = parameter { return data.len() == *len },
            Self::Function => if let Parameter::FixedBytes(data) = parameter { return data.len() == 24 },
            Self::Int(len) => if let Parameter::Int(_, data_len) = parameter { return  data_len == len },
            Self::Uint(len) => if let Parameter::Uint(_, data_len) = parameter { return data_len == len },
            Self::String => if let Parameter::String(_) = parameter { return true },
            Self::Array(param_type) => if let Parameter::Array(data) = parameter {
                return data.iter().all(|d| param_type.type_check(&d))
            }
            Self::FixedArray(param_type, len) => if let Parameter::FixedArray(data) = parameter {
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
    use std::mem::discriminant;

    #[test]
    fn parse_elementary_parameter_type() {
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

        match ParameterType::parse("int").unwrap() {
            ParameterType::Int(a) => assert_eq!(a, 256),
            _ => panic!("Failed to parse int"),
        }

        match ParameterType::parse("uint").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 256),
            _ => panic!("Failed to parse uint"),
        }

        match ParameterType::parse("uint16").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 16),
            _ => panic!("Failed to parse uint16"),
        }

        match ParameterType::parse("uint64").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 64),
            _ => panic!("Failed to parse uint64"),
        }

        match ParameterType::parse("uint256").unwrap() {
            ParameterType::Uint(a) => assert_eq!(a, 256),
            _ => panic!("Failed to parse uint256"),
        }

        match ParameterType::parse("int256").unwrap() {
            ParameterType::Int(a) => assert_eq!(a, 256),
            _ => panic!("Failed to parse int256"),
        }

        match ParameterType::parse("bytes32").unwrap() {
            ParameterType::FixedBytes(a) => assert_eq!(a, 32),
            _ => panic!("Failed to parse bytes32"),
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
    fn parse_tuple() {
        match ParameterType::parse("(address,bool,uint32,string)").unwrap() {
            ParameterType::Tuple(vec) => {
                assert_eq!(vec[0], ParameterType::Address);
                assert_eq!(vec[1], ParameterType::Bool);
                assert_eq!(vec[2], ParameterType::Uint(32));
                assert_eq!(vec[3], ParameterType::String);
            }
            _ => panic!("Failed to parse simple tuple"),
        }

        match ParameterType::parse("(address,(bytes32,uint256,bool),int32,string)").unwrap() {
            ParameterType::Tuple(vec) => {
                assert_eq!(vec[0], ParameterType::Address);
                match &vec[1] {
                    ParameterType::Tuple(inner_vec) => {
                        assert_eq!(inner_vec[0], ParameterType::FixedBytes(32));
                        assert_eq!(inner_vec[1], ParameterType::Uint(256));
                        assert_eq!(inner_vec[2], ParameterType::Bool);
                    }
                    _ => panic!("Failed to parse nested tuple"),
                }
                assert_eq!(vec[2], ParameterType::Int(32));
                assert_eq!(vec[3], ParameterType::String);
            }
            _ => panic!("Failed to parse outer tuple"),
        }
    }

    #[test]
    fn parse_array() {
        match ParameterType::parse("address[3]").unwrap() {
            ParameterType::FixedArray(param_type, len) => {
                assert_eq!(*param_type, ParameterType::Address);
                assert_eq!(len, 3);
            }
            _ => panic!("Failed to parse fixed array of lenght 3"),
        }

        match ParameterType::parse("uint256[][][5]").unwrap() {
            ParameterType::FixedArray(outer_type, len) => {
                assert_eq!(len, 5);
                match *outer_type {
                    ParameterType::Array(inner_type) => match *inner_type {
                        ParameterType::Array(param_type) => {
                            assert_eq!(*param_type, ParameterType::Uint(256))
                        }
                        _ => panic!("Failed to parse nested array"),
                    },
                    _ => panic!("Failed to parse nested array"),
                }
            }
            _ => panic!("Failed to parse nested array"),
        }

        match ParameterType::parse("(bool,string)[2][4]").unwrap() {
            ParameterType::FixedArray(outer_type, len) => {
                assert_eq!(len, 4);
                match *outer_type {
                    ParameterType::FixedArray(inner_type, len) => {
                        assert_eq!(len, 2);
                        match *inner_type {
                            ParameterType::Tuple(vec) => {
                                assert_eq!(vec[0], ParameterType::Bool);
                                assert_eq!(vec[1], ParameterType::String);
                            }
                            _ => panic!("Failed to parse nested tuple array"),
                        }
                    }
                    _ => panic!("Failed to parse nested tuple array"),
                }
            }
            _ => panic!("Failed to parse nested tuple array"),
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
        assert_eq!(&ParameterType::Function.as_abi_string(), "function");
        assert_eq!(
            &ParameterType::Array(Box::new(ParameterType::Bool)).as_abi_string(),
            "bool[]"
        );
        assert_eq!(
            &ParameterType::Array(Box::new(ParameterType::Address)).as_abi_string(),
            "address[]"
        );
        assert_eq!(
            &ParameterType::Array(Box::new(ParameterType::String)).as_abi_string(),
            "string[]"
        );
        assert_eq!(
            &ParameterType::FixedArray(Box::new(ParameterType::Int(8)), 3).as_abi_string(),
            "int8[3]"
        );
        assert_eq!(
            &ParameterType::FixedArray(Box::new(ParameterType::Address), 10).as_abi_string(),
            "address[10]"
        );
        assert_eq!(
            &ParameterType::FixedArray(Box::new(ParameterType::Uint(256)), 2).as_abi_string(),
            "uint256[2]"
        );
    }

    #[test]
    fn complex_parameter_type_as_string() {
        let abi_str = "(uint256,bytes,bytes32,address[3])";
        assert_eq!(
            &ParameterType::parse(abi_str).unwrap().as_abi_string(),
            abi_str
        );
        let abi_str = "address[][2][]";
        assert_eq!(
            &ParameterType::parse(abi_str).unwrap().as_abi_string(),
            abi_str
        );
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
