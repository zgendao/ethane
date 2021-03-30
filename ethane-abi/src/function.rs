use crate::AbiParserError;
use crate::ParameterType;

pub struct Function {
    pub _type: &'static str,
    pub inputs: Vec<ParameterType>,
    pub outputs: Vec<ParameterType>,
    pub state_mutability: Option<StateMutability>,
    pub payable: Option<bool>,
    pub constant: Option<bool>,
}

impl Function {
    pub fn parse(raw_func: &serde_json::Value) -> Result<Self, AbiParserError> {
        let inputs = Self::inputs(raw_func)?;
        let outputs = Self::inputs(raw_func)?;
        Ok(Self {
            _type: "function",
            inputs,
            outputs,
            state_mutability: StateMutability::parse(raw_func),
            payable: raw_func["payable"].as_bool(),
            constant: raw_func["constant"].as_bool(),
        })
    }

    fn inputs(raw_func: &serde_json::Value) -> Result<Vec<ParameterType>, AbiParserError> {
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

    fn outputs(raw_func: &serde_json::Value) -> Result<Vec<ParameterType>, AbiParserError> {
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
        if let Some(state_mutability) = raw_func["stateMutability"].as_str() {
            match state_mutability {
                "pure" => Some(StateMutability::Pure),
                "view" => Some(StateMutability::View),
                "nonpayable" => Some(StateMutability::NonPayable),
                "payable" => Some(StateMutability::Payable),
                _ => None,
            }
        } else {
            None
        }
    }
}
