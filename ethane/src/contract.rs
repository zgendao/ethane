use crate::types::{Bytes, Call, Address};
use crate::{rpc, Connector};
use ethane_abi::{Abi, AbiCall, Parameter};
use std::path::Path;

pub struct Caller<T> {
    abi: Abi,
    contract_address: Address,
    connector: Connector<T>,
}

impl Caller {
    pub fn new(connector: Connector<T>, abi: serde_json::Value, contract_address: Address) -> Caller {
        let mut abi = Abi::new();
        Caller {
            abi,
            contract_address,
            connector,
        }
    }

    pub fn new_from_path(connector: Connector<T>, path: &str, contract_address: Address) -> Caller {
        let mut abi = Abi::new();
        abi.parse(Path::new(path)).expect("unable to parse abi");
        Caller {
            abi,
            contract_address,
            connector,
        }
    }

    pub fn call(&mut self, function_name: &str, params: Vec<Parameter>) -> Vec<Parameter> {
        // @TODO remove unwraps
        let data = self
            .abi
            .encode(&AbiCall {
                function_name: "decimals",
                parameters: vec![],
            })
            .unwrap();

        let payload = Call {
            to: contract_address,
            data: Some(Bytes::from_slice(&data)),
            ..Default::default()
        };

        let result = self.connector.call(rpc::eth_call(payload, None)).unwrap();
        println!("{:?}", result);
        // @TODO decode result
        vec![]
    }
}
