use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use tiny_keccak::{Hasher, Keccak};

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
        let reader =
            File::open(path_to_abi).map_err(|e| AbiParserError::FileIoError(e.to_string()))?;
        let abi: serde_json::Value =
            serde_json::from_reader(reader).map_err(|e| AbiParserError::Serde(e.to_string()))?;

        self.parse_json(abi)
    }

    pub fn get_state_mutability(&self, function_name: &str) -> Option<StateMutability> {
        if let Some(function) = self.functions.get(function_name) {
            return function.state_mutability;
        }

        None
    }

    /// Encodes a function call according to Solidity's contract [ABI
    /// specification](https://docs.soliditylang.org/en/v0.5.3/abi-spec.html#function-selector).
    ///
    /// If the given function is found in the parsed ABI file, the function
    /// encodes the function signature and the input data provided in the form
    /// of a [`Parameter`] vector.
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
            let mut hasher = Keccak::v256();
            hasher.update(signature.as_bytes());
            // Take first 4 bytes of the Keccak hash
            let mut out = [0_u8; 32];
            hasher.finalize(&mut out);
            let mut hash = out[0..4].to_vec();
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
            let mut start_index = 0;
            let mut parameters = Vec::<Parameter>::with_capacity(function.outputs.len());
            for output in &function.outputs {
                let (parameter, i) =
                    Parameter::decode(&output.parameter_type, &hash[start_index..]);
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

#[derive(Debug)]
pub enum AbiParserError {
    FileIoError(String),
    Serde(String),
    MissingData(String),
    InvalidAbiEncoding(String),
    TypeError,
}
