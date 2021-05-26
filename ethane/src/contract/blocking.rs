use crate::types::{Address, Bytes, Call, TransactionRequest};
use crate::{rpc, Connection, Request};
use ethane_abi::{Abi, Parameter, StateMutability};
use std::path::Path;

use super::{CallOpts, CallResult, CallType};

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
        abi.parse_json(abi_json).expect("unable to parse abi");
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

    pub fn call(
        &mut self,
        function_name: &str,
        params: Vec<Parameter>,
        opts: Option<CallOpts>,
    ) -> CallResult {
        let mut call_type = if let Some(m) = self.abi.get_state_mutability(function_name) {
            match m {
                StateMutability::Pure => CallType::Call,
                StateMutability::View => CallType::Call,
                StateMutability::NonPayable => CallType::Transaction,
                StateMutability::Payable => CallType::Transaction,
            }
        } else {
            CallType::Transaction
        };

        let mut from_address: Address = Default::default();
        if let Some(o) = opts {
            from_address = o.from.unwrap();
            if let Some(ct) = o.force_call_type {
                call_type = ct;
            }
        }

        let data = self.abi.encode(function_name, params).unwrap();

        match call_type {
            CallType::Transaction => self.eth_send_transaction(data, from_address),
            CallType::Call => self.eth_call(function_name, data),
        }
    }

    fn eth_call(&mut self, function_name: &str, data: Vec<u8>) -> CallResult {
        let payload = Call {
            to: self.contract_address,
            data: Some(Bytes::from_slice(&data)),
            ..Default::default()
        };

        let call_result = self.connection.call(rpc::eth_call(payload, None)).unwrap();
        CallResult::Call(
            self.abi
                .decode(function_name, call_result.0.as_slice())
                .unwrap(),
        )
    }

    fn eth_send_transaction(&mut self, data: Vec<u8>, from_address: Address) -> CallResult {
        let payload = TransactionRequest {
            from: from_address,
            to: Some(self.contract_address),
            data: Some(Bytes::from_slice(&data)),
            ..Default::default()
        };

        CallResult::Transaction(
            self.connection
                .call(rpc::eth_send_transaction(payload))
                .unwrap(),
        )
    }
}
