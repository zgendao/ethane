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

    pub fn keccak_hash(
        &self,
        function_name: &str,
        parameters: Vec<Parameter>,
    ) -> Result<Vec<u8>, AbiParserError> {
        if let Some(function) = self.functions.get(function_name) {
            // Check whether the correct number of parameters were provided
            if parameters.len() != function.inputs.len() {
                return Err(AbiParserError::MissingData(format!(
                    "Wrong number of parameters were provided. Expected {}, found {}",
                    function.inputs.len(),
                    parameters.len(),
                )));
            }

            // Create function signature and hash
            let input_type_str = function
                .inputs
                .iter()
                .map(|input| input.parameter_type.as_abi_string())
                .collect::<Vec<String>>()
                .join(",");

            let signature = format!("{}({})", function_name, input_type_str);
            let mut keccak = Keccak256::new();
            keccak.update(signature);
            // Take first 4 bytes of the Keccak hash
            let mut hash = keccak.finalize()[0..4].to_vec();
            // Append the encoded parameters to the hash
            for (parameter, input) in parameters.iter().zip(function.inputs.iter()) {
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

    use ethereum_types::Address;
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_new() {
        // let path = Path::new("src/abi/abi.json");
        let path = Path::new("../ethane/test-helper/src/fixtures/foo.abi");

        let mut abi = Abi::new();
        abi.parse(path).expect("unable to parse abi");
        let addr = Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap();
        let hash = abi.keccak_hash("bar", vec![Parameter::Address(addr)]);
        let expected = hex!("646ea56d00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a");
        assert_eq!(hash.unwrap(), expected);
    }

    // #[test]
    // fn test_new() {
    //     // let path = Path::new("src/abi/abi.json");
    //     let path = Path::new("test-helper/src/fixtures/TestABI.json");
    //
    //
    //     let mut abi = Abi::new();
    //     println!("{:?}", abi);
    //     let f = abi.parse(path).expect("unable to parse abi");
    //     println!("{:?}",f);
    //     println!("{:?}",f[0].outputs[0].to_string());
    //
    //     abi.encode("WETH",vec![]);
    // }
}
