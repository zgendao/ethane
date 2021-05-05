import * as wasm from "ethane-wasm";

// let ethane = wasm.new();
let ethane = wasm.Ethane.new()
let res = ethane.eth_request_accounts().then(
    (res) => console.log(res)
);
// console.log(wasm);
