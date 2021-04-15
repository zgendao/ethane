use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use sha3::{Digest, Keccak256};
use thiserror::Error;

mod function;
mod parameter;

use function::Function;
pub use function::StateMutability;
pub use parameter::Parameter;
use parameter::ParameterType;

/// Parses a `.json` file containing ABI encoded Solidity functions.
///
/// It stores the functions in a `HashMap` with the function name being the key
/// and the parsed function the value.
pub struct Abi {
    pub functions: HashMap<String, Function>,
}

impl Default for Abi {
    fn default() -> Self {
        Self::new()
    }
}

impl Abi {
    /// Creates a new `Abi` instance with an empty `HashMap` within.
    #[inline]
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Parses an ABI value into the `Abi` instance.
    pub fn parse_json(&mut self, abi: serde_json::Value) -> Result<(), AbiParserError> {
        let mut i: usize = 0;
        while abi[i] != serde_json::Value::Null {
            if abi[i]["type"] == "function" {
                if abi[i]["name"] != serde_json::Value::Null {
                    let name = abi[i]["name"].as_str().unwrap().to_owned();
                    self.functions.insert(name, Function::parse(&abi[i])?);
                } else {
                    return Err(AbiParserError::MissingData(
                        "Function name is missing from ABI.".to_owned(),
                    ));
                }
            }
            i += 1;
        }

        Ok(())
    }

    /// Parses an ABI `.json` file into the `Abi` instance.
    pub fn parse_file(&mut self, path_to_abi: &Path) -> Result<(), AbiParserError> {
        let file = File::open(path_to_abi)?;
        let reader = BufReader::new(file);
        let abi: serde_json::Value = serde_json::from_reader(reader)?;

        self.parse_json(abi)
    }

    pub fn get_state_mutability(&self, function_name: &str) -> Option<StateMutability> {
        if let Some(function) = self.functions.get(function_name) {
            return function.state_mutability;
        }

        None
    }

    pub fn encode(
        &self,
        function_name: &str,
        parameters: Vec<Parameter>,
    ) -> Result<Vec<u8>, AbiParserError> {
        if let Some(function) = self.functions.get(function_name) {
            let mut abi_arguments = Vec::<String>::with_capacity(parameters.len());
            for (input, param) in function.inputs.iter().zip(parameters.iter()) {
                if input.parameter_type.type_check(param) {
                    abi_arguments.push(input.parameter_type.as_abi_string())
                } else {
                    return Err(AbiParserError::InvalidAbiEncoding(format!(
                        "Invalid parameter type supplied. Expected {:?}",
                        input.parameter_type
                    )));
                }
            }
            let signature = format!("{}({})", function_name, abi_arguments.join(","));
            let mut hasher = Keccak256::new();
            hasher.update(signature);
            // Take first 4 bytes of the Keccak hash
            let mut hash = hasher.finalize()[0..4].to_vec();
            // Append the encoded parameters to the hash
            parameter::encode_into(&mut hash, parameters);
            Ok(hash)
        } else {
            Err(AbiParserError::MissingData(
                "Function name not found in ABI".to_owned(),
            ))
        }
    }

    /// Decodes a hash into a [`Parameter`] vector.
    ///
    /// Based on the given ABI function name, the `Abi` parser iterates over that
    /// function's output parameter types and decodes the output hash accordingly.
    pub fn decode(
        &self,
        function_name: &str,
        hash: &[u8],
    ) -> Result<Vec<Parameter>, AbiParserError> {
        if let Some(function) = self.functions.get(function_name) {
            let mut start_index = 4_usize; // starting from 5th byte, since the first four is reserved
            let mut parameters = Vec::<Parameter>::with_capacity(function.outputs.len());
            for output in &function.outputs {
                let (parameter, i) =
                    Parameter::decode(&output.parameter_type, &hash[start_index..])?;
                start_index += i;
                parameters.push(parameter);
            }

            Ok(parameters)
        } else {
            Err(AbiParserError::MissingData(
                "Function name not found in ABI".to_owned(),
            ))
        }
    }
}

#[derive(Error, Debug)]
pub enum AbiParserError {
    #[error("Couldn't open ABI file: {0}")]
    FileIoError(#[from] std::io::Error),
    #[error("De-/Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Missing data error: {0}")]
    MissingData(String),
    #[error("Invalid ABI encoding error: {0}")]
    InvalidAbiEncoding(String),
    #[error("Parameter type doesn't match the internal data type")]
    TypeError,
}

/*
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ethereum_types::{Address, U256};
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_encode_bar() {
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse_file(path).expect("unable to parse abi");
        let addr = Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
        let hash = abi.encode(&AbiCall {
            function_name: "bar",
            parameters: vec![Parameter::Address(addr)],
        });
        let expected =
            hex!("646ea56d00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a");
        assert_eq!(hash.unwrap(), expected);
    }

    #[test]
    fn test_encode_approve() {
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse_file(path).expect("unable to parse abi");
        let hash = abi.encode(&AbiCall {
            function_name: "approve",
            parameters: vec![
                Parameter::Address(
                    Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
                ),
                Parameter::Uint256(U256::from_str("613").unwrap()),
            ],
        });
        let expected =
            hex!("095ea7b300000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a0000000000000000000000000000000000000000000000000000000000000613");
        assert_eq!(hash.unwrap(), expected);
    }

    #[test]
    fn test_encode_total_supply() {
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse_file(path).expect("unable to parse abi");
        let hash = abi.encode(&AbiCall {
            function_name: "totalSupply",
            parameters: vec![],
        });
        let expected = hex!("18160DDD");
        assert_eq!(hash.unwrap(), expected);
    }

    #[test]
    fn test_encode_transfer_from() {
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse_file(path).expect("unable to parse abi");
        let hash = abi.encode(&AbiCall {
            function_name: "transferFrom",
            parameters: vec![
                Parameter::Address(
                    Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
                ),
                Parameter::Address(
                    Address::from_str("0x1A4C0439ba035DAcf0D573394107597CEEBF9FF8").unwrap(),
                ),
                Parameter::Uint256(U256::from_str("14DDD").unwrap()),
            ],
        });
        let expected =
            hex!("23b872dd00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a0000000000000000000000001a4c0439ba035dacf0d573394107597ceebf9ff80000000000000000000000000000000000000000000000000000000000014ddd");
        assert_eq!(hash.unwrap(), expected);
    }
}
*/
