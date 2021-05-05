use wasm_bindgen::{JsCast, prelude::*};
use js_sys::Promise;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use std::future::Future;

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

    pub fn eth_request_accounts(&self, args: RequestArguments) -> Promise {
        // let args = RequestArguments {
        //     method: "eth_requestAccounts".into(),
        //     params: js_sys::Array::new(),
        // };
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

#[wasm_bindgen]
pub struct RequestArguments {
    method: String,
    // pub method: js_sys::JsString,
    // pub params: js_sys::Array,
}

#[wasm_bindgen]
impl RequestArguments {
    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone()
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        RequestArguments {
            method: "eth_requestAccounts".to_string(),
            // params: js_sys::Array::new(),
        }
    }

    // pub fn set(&mut self, method: js_sys::JsString) {
    //     self.method = method;
    // }
}
