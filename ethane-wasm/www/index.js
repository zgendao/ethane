import * as wasm from "ethane-wasm";
import {RequestArguments} from "ethane-wasm";

// let ethane = wasm.new();
let ethane = wasm.Ethane.new()
let args = new wasm.RequestArguments();
// console.log(args);
// console.log(args.method);
// console.log(args.params);
let res = ethane.eth_request_accounts(args).then(
    (res) => console.log(res)
);
// console.log(wasm);

