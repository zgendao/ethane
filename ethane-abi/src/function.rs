use crate::AbiParserError;
use crate::ParameterType;

// NOTE or TODO by storing only ParameterTypes, we don't store the type names. Is that a problem?
// NOTE the function name is available as key to the respective function in the HashMap
pub struct Function {
    pub _type: &'static str,
    pub input_types: Vec<ParameterType>,
    pub output_types: Vec<ParameterType>,
    pub state_mutability: Option<StateMutability>,
    pub payable: Option<bool>,
    pub constant: Option<bool>,
}

impl Function {
    pub fn parse(raw_func: &serde_json::Value) -> Result<Self, AbiParserError> {
        let input_types = Self::parse_inputs(raw_func)?;
        let output_types = Self::parse_outputs(raw_func)?;
        Ok(Self {
            _type: "function",
            input_types,
            output_types,
            state_mutability: StateMutability::parse(raw_func),
            payable: raw_func["payable"].as_bool(), // as_bool() automatically returns an Option<bool>
            constant: raw_func["constant"].as_bool(), // as_bool() automatically returns an Option<bool>
        })
    }

    fn parse_inputs(raw_func: &serde_json::Value) -> Result<Vec<ParameterType>, AbiParserError> {
        match &raw_func["inputs"] {
            serde_json::Value::Array(inputs) => {
                let mut result = Vec::new();
                for input in inputs {
                    if let Some(inp) = input.as_str() {
                        result.push(ParameterType::parse(inp)?);
                    } else {
                        return Err(AbiParserError::MissingData(
                            "No input name defined".to_owned(),
                        ));
                    }
                }
                Ok(result)
            }
            _ => Err(AbiParserError::InvalidAbiEncoding(
                "Function inputs are not given as an array".to_owned(),
            )),
        }
    }

    fn parse_outputs(raw_func: &serde_json::Value) -> Result<Vec<ParameterType>, AbiParserError> {
        match &raw_func["outputs"] {
            serde_json::Value::Array(outputs) => {
                let mut result = Vec::new();
                for output in outputs {
                    if let Some(out) = output.as_str() {
                        result.push(ParameterType::parse(out)?);
                    } else {
                        return Err(AbiParserError::MissingData(
                            "No output name defined".to_owned(),
                        ));
                    }
                }
                Ok(result)
            }
            _ => Err(AbiParserError::InvalidAbiEncoding(
                "Function outputs are not given as an array".to_owned(),
            )),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl StateMutability {
    pub fn parse(raw_func: &serde_json::Value) -> Option<Self> {
        match raw_func["stateMutability"].as_str() {
            Some("pure") => Some(StateMutability::Pure),
            Some("view") => Some(StateMutability::View),
            Some("nonpayable") => Some(StateMutability::NonPayable),
            Some("payable") => Some(StateMutability::Payable),
            _ => None,
        }
    }
}
