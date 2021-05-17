import { RequestArguments, Web3 } from "ethane-wasm";

const args = RequestArguments.new("eth_syncing", []);
const web3 = Web3.new("http://localhost:8545");
web3.call(args).then((val) => document.write(val));
