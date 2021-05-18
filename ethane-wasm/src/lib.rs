use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RequestArguments {
    method: String,
    params: js_sys::Array, // NOTE Serialize is not implemented for js_sys::Array
}

#[wasm_bindgen]
impl RequestArguments {
    pub fn new(method: String, params: js_sys::Array) -> Self {
        Self { method, params }
    }

    pub fn as_json_string(&self) -> String {
        let id = 64; // TODO how to assign id
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
    client: ethane::AsyncHttp,
}

#[wasm_bindgen]
impl Web3 {
    pub fn new(address: String) -> Self {
        Self {
            client: ethane::AsyncHttp::new(&address, None),
        }
    }

    // NOTE Async calls in `wasm` take `self` by value, not reference!
    pub async fn call(mut self, args: RequestArguments) -> String {
        let result = self.client.request(args.as_json_string()).await;
        if let Ok(response) = result {
            response
        } else {
            format!("Error: {:?}", result.err().unwrap())
        }
    }
}
