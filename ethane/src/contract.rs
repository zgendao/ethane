use crate::types::{Address, Bytes, Call};
use crate::{rpc, Connection, Request};
use ethane_abi::{Abi, AbiCall, Parameter};
use std::path::Path;

pub struct Caller<T: Request> {
    abi: Abi,
    contract_address: Address,
    connection: Connection<T>,
}

impl<T> Caller<T>
where
    T: Request,
{
    pub fn new(
        connection: Connection<T>,
        abi_json: serde_json::Value,
        contract_address: Address,
    ) -> Caller<T> {
        let mut abi = Abi::new();
        abi.parse_json(abi_json);
        Caller {
            abi,
            contract_address,
            connection,
        }
    }

    pub fn new_from_path(
        connection: Connection<T>,
        path: &str,
        contract_address: Address,
    ) -> Caller<T> {
        let mut abi = Abi::new();
        abi.parse_file(Path::new(path))
            .expect("unable to parse abi");
        Caller {
            abi,
            contract_address,
            connection,
        }
    }

    pub fn call(&mut self, function_name: &str, params: Vec<Parameter>) -> Vec<Parameter> {
        // @TODO remove unwraps
        let data = self
            .abi
            .encode(&AbiCall {
                function_name,
                parameters: params,
            })
            .unwrap();

        let payload = Call {
            to: self.contract_address,
            data: Some(Bytes::from_slice(&data)),
            ..Default::default()
        };

        let call_result = self.connection.call(rpc::eth_call(payload, None)).unwrap();
        self.abi
            .decode(function_name, call_result.0.as_slice())
            .unwrap()
    }
}
