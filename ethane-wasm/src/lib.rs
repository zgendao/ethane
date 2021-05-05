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

#[wasm_bindgen]
pub struct RequestArguments {
    method: String,
    params: js_sys::Array,
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
}
*/
#[wasm_bindgen]
pub struct EthaneClient {
    client: ethane::Connection<ethane::Http>,
}

// TODO it seems we need a wrapper around each function :(
#[wasm_bindgen]
impl EthaneClient {
    pub fn eth_protocol_version(&mut self) -> String {
        self.client
            .call(ethane::rpc::eth_protocol_version())
            .unwrap()
    }

    pub fn eth_syncing(&mut self) -> bool {
        let result = self.client.call(ethane::rpc::eth_syncing()).unwrap();
        match result {
            ethane::types::SyncInfo::Syncing(_) => true,
            ethane::types::SyncInfo::NotSyncing(_) => false,
        }
    }
}
