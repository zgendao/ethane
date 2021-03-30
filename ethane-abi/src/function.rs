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
    // TODO this should return with a Result
    pub fn parse(raw_func: &serde_json::Value) -> Self {
        Self {
            _type: "function",
            inputs: Self::inputs(raw_func),
            outputs: Self::outputs(raw_func),
            state_mutability: StateMutability::parse(raw_func),
            payable: Self::payable(raw_func),
            constant: Self::constant(raw_func),
        }
    }

    fn inputs(raw_func: &serde_json::Value) -> Vec<ParameterType> {
        match &raw_func["inputs"] {
            serde_json::Value::Array(v) => {
                let mut res = vec![];
                for input in v {
                    // TODO handle unwrap
                    res.push(ParameterType::parse(input.as_str().unwrap()).unwrap())
                }
                res
            }
            _ => panic!("ABI input is not properly formatted"),
        }
    }

    fn outputs(raw_func: &serde_json::Value) -> Vec<ParameterType> {
        match &raw_func["outputs"] {
            serde_json::Value::Array(v) => {
                let mut res = vec![];
                for input in v {
                    // TODO handle unwrap
                    res.push(ParameterType::parse(input.as_str().unwrap()).unwrap())
                }
                res
            }
            _ => panic!("ABI output is not properly formatted"),
        }
    }

    fn payable(raw_func: &serde_json::Value) -> Option<bool> {
        let payable = &raw_func["payable"];
        match payable {
            serde_json::Value::Bool(val) => Some(*val),
            _ => None,
        }
    }

    fn constant(raw_func: &serde_json::Value) -> Option<bool> {
        let raw_constant = &raw_func["constant"];
        match raw_constant {
            serde_json::Value::Bool(val) => Some(*val),
            _ => None,
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
        let raw_state_mutability = &raw_func["stateMutability"];
        match raw_state_mutability {
            serde_json::Value::String(val) => match val.as_str() {
                "pure" => Some(StateMutability::Pure),
                "view" => Some(StateMutability::View),
                "nonpayable" => Some(StateMutability::NonPayable),
                "payable" => Some(StateMutability::Payable),
                _ => {
                    panic!("TODO");
                }
            },
            _ => None,
        }
    }
}
