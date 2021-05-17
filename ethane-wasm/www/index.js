import { RequestArguments, Web3 } from "ethane-wasm";

let web3 = Web3.new("http://localhost:8545")
const args = RequestArguments.new("eth_someCall", ["0x1234567890123456789", "hello", "world"]);
web3.call(args).then((val) => document.write(val));
