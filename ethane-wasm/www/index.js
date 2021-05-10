import { RequestArguments, Web3 } from "ethane-wasm";

let web3 = Web3.new("eth_someCall", ["0x1234567890123456789", "hello", "world"]);

console.log(web3.as_json_string())
