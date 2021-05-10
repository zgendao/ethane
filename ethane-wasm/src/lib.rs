use ethane::Request;
use wasm_bindgen::prelude::*;

/*
#[wasm_bindgen]
pub struct Ethane {
    provider: Provider
}

#[wasm_bindgen]
impl Ethane {
    pub fn new() -> Ethane {
        Ethane {
            provider: get_provider_js().unwrap(),
        }
    }

    pub fn eth_request_accounts(&self) -> Promise {
        let args = RequestArguments {
            method: "eth_requestAccounts".to_string(),
            params: js_sys::Array::new(),
        };
        self.provider.request(args)
    }
}

#[wasm_bindgen(inline_js = "export function get_provider_js() {return window.ethereum}")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn get_provider_js() -> Result<Provider, JsValue>;
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
    /// An EIP-1193 provider object. Available by convention at `window.ethereum`
    pub type Provider;

    #[wasm_bindgen(method)]
    fn request(_: &Provider, args: RequestArguments) -> Promise;
}
*/
#[wasm_bindgen]
pub struct RequestArguments {
    method: String,
    params: js_sys::Array, // NOTE Serialize is not implemented for js_sys::Array
}

#[wasm_bindgen]
impl RequestArguments {
    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn params(&self) -> js_sys::Array {
        self.params.clone()
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
    client: ethane::Http,
}

#[wasm_bindgen]
impl Web3 {
    pub fn call(&mut self, args: RequestArguments) -> String {
        let result = self.client.request(args.as_json_string());
        if let Ok(response) = result {
            response
        } else {
            format!("Error: {:?}", result.err().unwrap())
        }
    }
}
