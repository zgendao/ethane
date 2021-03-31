use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use sha3::{Digest, Keccak256};
use thiserror::Error;

pub use function::{Function, StateMutability};
pub use parameter::{Parameter, ParameterType};

mod function;
mod parameter;

pub struct Abi {
    pub functions: HashMap<String, Function>,
}

impl Default for Abi {
    fn default() -> Self {
        Self::new()
    }
}

impl Abi {
    #[inline]
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn parse(&mut self, path_to_abi: &Path) -> Result<(), AbiParserError> {
        let file = File::open(path_to_abi)?;
        let reader = BufReader::new(file);
        let functions: serde_json::Value = serde_json::from_reader(reader)?;

        let mut i: usize = 0;
        while functions[i] != serde_json::Value::Null {
            if functions[i]["type"] == "function" {
                if functions[i]["name"] != serde_json::Value::Null {
                    let name = functions[i]["name"].as_str().unwrap().to_owned();
                    self.functions.insert(name, Function::parse(&functions[i])?);
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

    pub fn encode(&self, abi_call: &AbiCall) -> Result<Vec<u8>, AbiParserError> {
        if let Some(function) = self.functions.get(abi_call.function_name) {
            // Check whether the correct number of parameters were provided
            if abi_call.parameters.len() != function.inputs.len() {
                return Err(AbiParserError::MissingData(format!(
                    "Wrong number of parameters were provided. Expected {}, found {}",
                    function.inputs.len(),
                    abi_call.parameters.len(),
                )));
            }

            // Create function signature and hash
            let input_type_str = function
                .inputs
                .iter()
                .map(|input| input.parameter_type.as_abi_string())
                .collect::<Vec<String>>()
                .join(",");

            let signature = format!("{}({})", abi_call.function_name, input_type_str);
            let mut keccak = Keccak256::new();
            keccak.update(signature);
            // Take first 4 bytes of the Keccak hash
            let mut hash = keccak.finalize()[0..4].to_vec();
            // Append the encoded parameters to the hash
            for (parameter, input) in abi_call.parameters.iter().zip(function.inputs.iter()) {
                if parameter.get_type() != input.parameter_type {
                    return Err(AbiParserError::InvalidAbiEncoding(format!(
                        "Invalid parameter type supplied. Expected {:?}, found {:?}",
                        input.parameter_type,
                        parameter.get_type()
                    )));
                }
                hash.append(&mut parameter.encode());
            }

            Ok(hash)
        } else {
            Err(AbiParserError::MissingData(
                "Function name not found in ABI".to_owned(),
            ))
        }
    }

    pub fn decode(&self) -> Result<AbiCall, AbiParserError> {
        Ok(AbiCall {
            function_name: "hello",
            parameters: Vec::new(),
        })
    }
}

pub struct AbiCall<'a> {
    pub function_name: &'a str,
    pub parameters: Vec<Parameter>,
}

#[derive(Error, Debug)]
pub enum AbiParserError {
    #[error("Couldn't open ABI file: {0}")]
    FileIOError(#[from] std::io::Error),
    #[error("De-/Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Missing data error: {0}")]
    MissingData(String),
    #[error("Invalid ABI encoding error: {0}")]
    InvalidAbiEncoding(String),
}

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
        abi.parse(path).expect("unable to parse abi");
        let addr = Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
        let hash = abi.encode(&AbiCall {
            function_name: "bar",
            parameters: vec![Parameter::Address(addr)],
        });
        let expected = hex!("646ea56d95eda452256c1190947f9ba1fd19422f0120858a");
        assert_eq!(hash.unwrap(), expected);
    }

    #[test]
    fn test_encode_approve() {
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse(path).expect("unable to parse abi");
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
            hex!("095ea7b395eda452256c1190947f9ba1fd19422f0120858a0000000000000000000000000000000000000000000000000000000000000613");
        assert_eq!(hash.unwrap(), expected);
    }

    #[test]
    fn test_encode_total_supply() {
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse(path).expect("unable to parse abi");
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
        abi.parse(path).expect("unable to parse abi");
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
            hex!("23b872dd95eda452256c1190947f9ba1fd19422f0120858a1a4c0439ba035dacf0d573394107597ceebf9ff80000000000000000000000000000000000000000000000000000000000014ddd");
        assert_eq!(hash.unwrap(), expected);
    }
}
