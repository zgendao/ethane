use crate::AbiParserError;
use crate::ParameterType;

// NOTE the function name is available as key to the respective function in the HashMap
pub struct Function {
    pub _type: &'static str,
    pub inputs: Vec<FunctionParameter>,
    pub outputs: Vec<FunctionParameter>,
    pub state_mutability: Option<StateMutability>,
    pub payable: Option<bool>,
    pub constant: Option<bool>,
}

impl Function {
    pub fn parse(raw_func: &serde_json::Value) -> Result<Self, AbiParserError> {
        let inputs = Self::parse_parameters(&raw_func["inputs"])?;
        let outputs = Self::parse_parameters(&raw_func["outputs"])?;
        Ok(Self {
            _type: "function",
            inputs,
            outputs,
            state_mutability: StateMutability::parse(raw_func),
            payable: raw_func["payable"].as_bool(), // as_bool() automatically returns an Option<bool>
            constant: raw_func["constant"].as_bool(), // as_bool() automatically returns an Option<bool>
        })
    }

    fn parse_parameters(
        raw_func: &serde_json::Value,
    ) -> Result<Vec<FunctionParameter>, AbiParserError> {
        match raw_func {
            serde_json::Value::Array(parameters) => {
                let mut result = Vec::new();
                for parameter in parameters {
                    let p_type = parameter["type"].as_str().ok_or_else(|| {
                        AbiParserError::MissingData("Missing parameter type".to_owned())
                    })?;
                    let p_name = parameter["name"].as_str().ok_or_else(|| {
                        AbiParserError::MissingData("Missing parameter name".to_owned())
                    })?;
                    let parameter_type = ParameterType::parse(p_type)?;
                    result.push(FunctionParameter {
                        name: p_name.to_owned(),
                        parameter_type,
                    });
                }
                Ok(result)
            }
            _ => Err(AbiParserError::InvalidAbiEncoding(
                "Function parameters are not given as an array".to_owned(),
            )),
        }
    }
}

pub struct FunctionParameter {
    pub name: String,
    pub parameter_type: ParameterType,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::ParameterType;

    #[test]
    fn parse_function() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{
            "constant": true,
            "name": "stuff",
            "inputs": [
                {
                    "name": "_spender",
                    "type": "address"
                },
                {
                    "name": "",
                    "type": "bytes64"
                }
            ],
            "outputs": [],
            "payable": false,
            "type": "function",
            "stateMutability": "view"
        }"#,
        )
        .unwrap();

        let function = Function::parse(&json).unwrap();
        assert_eq!(function.inputs.len(), 2);
        assert_eq!(function.inputs[0].parameter_type, ParameterType::Address);
        assert_eq!(function.inputs[0].name, "_spender");
        assert_eq!(
            function.inputs[1].parameter_type,
            ParameterType::FixedBytes(64)
        );
        assert!(function.inputs[1].name.is_empty());
        assert!(function.outputs.is_empty());
        assert_eq!(function.constant, Some(true));
        assert_eq!(function.payable, Some(false));
        assert_eq!(function.state_mutability, Some(StateMutability::View));
    }
}
