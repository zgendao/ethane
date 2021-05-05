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
        let req = <JsFuture as From<T: dyn Future>>>::from(self.provider.request(args));
        future_to_promise(
            async move {
                let output = match req.await {
                    Ok(_resolved) => {
                        // match resolved.into_serde::<Info>() {
                        //     Ok(val) => format!("{:?}", &val),
                        //     Err(_) => "Deserialize error".to_string(),
                        // }
                        "response".to_string()
                    }
                    Err(_) => "Promise error".to_string(),
                };
                Ok(JsValue::from(output))
            }
        )
        // let _res = self.provider.request(args).await;
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

    #[wasm_bindgen(catch, method)]
    async fn request(_: &Provider, args: RequestArguments) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
struct RequestArguments {
    method: String,
    params: js_sys::Array,
}
