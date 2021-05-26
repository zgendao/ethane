use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

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

#[wasm_bindgen]
pub struct Web3 {
    id: usize,
    client: ethane::AsyncHttp,
}

#[wasm_bindgen]
impl Web3 {
    pub fn new(address: String) -> Self {
        Self {
            id: 0,
            client: ethane::AsyncHttp::new(&address, None),
        }
    }

    pub fn call(&mut self, args: RequestArguments) -> Promise {
        let id = if self.id > 10000 { 1 } else { self.id + 1 };
        self.id = id;
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
