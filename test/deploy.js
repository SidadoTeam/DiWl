import { storageDepositLimit, gasLimit, alicePair, api, contractAbi, wasm, __dirname } from "./base.js";
import { CodePromise } from '@polkadot/api-contract';
import fs from 'fs'

const code = new CodePromise(api, contractAbi, wasm);
const tx = code.tx.new({ gasLimit: gasLimit, storageDepositLimit }, false);

let address_contract;

tx.signAndSend(alicePair, async ({ contract, status }) => {
  if (status.isInBlock || status.isFinalized) {
    address_contract = contract.address.toString();
    console.log("deploy contract success : " + address_contract);
    fs.writeFileSync(__dirname + 'address_contract.txt', address_contract);
    process.exit(0);
  } else {
    console.log("deploy contract fail status: " + status);
  }
});
