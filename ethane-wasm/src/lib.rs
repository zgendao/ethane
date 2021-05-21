use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use std::collections::VecDeque;

#[wasm_bindgen]
pub struct RequestArguments {
    method: String,
    params: Array, // NOTE Serialize is not implemented for js_sys::Array
}

#[wasm_bindgen]
impl RequestArguments {
    pub fn new(method: String, params: Array) -> Self {
        Self { method, params }
    }

    pub fn as_json_string(&self, id: usize) -> String {
        let param_vec = self
            .params
            .iter()
            .map(|val| {
                val.as_string()
                    .unwrap_or_else(|| "Error: couldn't turn JsValue into String".to_owned())
            })
            .collect::<Vec<String>>();

        format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":{:?},\"params\":{:?},\"id\":{}}}",
            self.method, param_vec, id
        )
    }
}

/*
#[wasm_bindgen]
pub struct Web3 {
    id_pool: VecDeque<usize>,
    client: ethane::AsyncHttp,
}

#[wasm_bindgen]
impl Web3 {
    pub fn new(address: String) -> Self {
        Self {
            id_pool: (0..1000).collect(),
            client: ethane::AsyncHttp::new(&address, None),
        }
    }

    // NOTE Async calls in `wasm` take `self` by value, not reference!
    pub fn call(&mut self, args: RequestArguments) -> Promise {
        let id = if let Some(id) = self.id_pool.pop_front() {
            self.id_pool.push_back(id);
            id
        } else {
            1001
        };

        let client = self.client.clone();

        future_to_promise(async move {
            let result = client.request(args.as_json_string(id)).await;
            let response = if let Ok(response) = result {
                response
            } else {
                format!("Error: {:?}", result.err().unwrap())
            };
            Ok(JsValue::from(response))
        })
    }
}
*/
