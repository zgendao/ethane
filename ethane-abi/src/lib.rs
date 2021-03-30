mod function;
mod parameter;

pub use function::{Function, StateMutability};
pub use parameter::{Parameter, parameter_type::ParameterType};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use thiserror::Error;

pub struct Abi {
    pub functions: HashMap<String, Function>,
}

impl Abi {
    #[inline]
    pub fn new() -> Self {
        Abi {
            functions: HashMap::new(),
        }
    }

    pub fn parse(&mut self, path_to_abi: &Path) -> Result<(), AbiParserError> {
        let file = File::open(path_to_abi)?;
        let reader = BufReader::new(file);
        let functions: serde_json::Value =
            serde_json::from_reader(reader)?;

        let mut i: usize = 0;
        while functions[i] != serde_json::Value::Null {
            if functions[i]["type"] == "function" && functions[i]["name"] != serde_json::Value::Null
            {
                let name = functions[i]["name"].as_str().unwrap().to_owned();
                self.functions.insert(name, Function::parse(&functions[i])?);
            } else {
                return Err(AbiParserError::MissingData("Function name is missing from ABI.".to_owned()));
            }
            i += 1;
        }

        Ok(())
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



//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_new() {
//        // let path = Path::new("src/abi/abi.json");
//        let path = Path::new("test-helper/src/fixtures/TestABI.json");
//
//
//        let mut abi = Abi::new();
//        println!("{:?}", abi);
//        let f = abi.parse(path).expect("unable to parse abi");
//        println!("{:?}",f);
//        println!("{:?}",f[0].outputs[0].to_string());
//
//        abi.encode("WETH",vec![]);
//    }
//}
