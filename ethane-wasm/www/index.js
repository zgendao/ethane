import * as wasm from "ethane-wasm";

let ethane = wasm.Ethane.new()

let res = async () =>  {
    let resp = await ethane.eth_request_accounts()
    return resp
}

console.log(res());

