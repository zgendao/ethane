use crate::parameter_type::ParameterType;
use crate::AbiParserError;

/// An ABI function instance.
///
/// Contains the fields of a properly encoded ABI function. The function name
/// is available as the key to the respective function in the `HashMap` of the
/// [`crate::Abi`] parser.
pub struct Function {
    pub inputs: Vec<FunctionParameter>,
    pub outputs: Vec<FunctionParameter>,
    pub state_mutability: Option<StateMutability>,
    pub payable: Option<bool>,
    pub constant: Option<bool>,
}

impl Function {
    /// Tries to parse a `.json` file into a [`Function`].
    pub fn parse(raw_func: &serde_json::Value) -> Result<Self, AbiParserError> {
        let inputs = Self::parse_parameters(&raw_func["inputs"])?;
        let outputs = Self::parse_parameters(&raw_func["outputs"])?;
        Ok(Self {
            inputs,
            outputs,
            state_mutability: StateMutability::parse(raw_func),
            payable: raw_func["payable"].as_bool(), // as_bool() automatically returns an Option<bool>
            constant: raw_func["constant"].as_bool(), // as_bool() automatically returns an Option<bool>
        })
    }

    /// Tries to parse a `.json` string  into an array of ABI function
    /// parameters.
    ///
    /// If the ABI file is properly formatted, both the function inputs and
    /// outputs can be parsed using this function.
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

/// ABI function parameter type.
///
/// Contains the name of the parameter (which can be an empty string) and the
/// type of the parameter as a [`ParameterType`].
pub struct FunctionParameter {
    pub name: String,
    pub parameter_type: ParameterType,
}

/// Possible variants of an ABI function's respective state mutability flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl StateMutability {
    /// Attempts to parse a `.json` string into an optional [`StateMutability`]
    /// flag.
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
