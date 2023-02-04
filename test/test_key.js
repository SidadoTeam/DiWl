import { ApiPromise,Keyring } from '@polkadot/api';
import {readFile,readFileSync} from 'fs';

let wasmPath = "../diwl-contract/target/ink/diwl_contract.contract";
let abiMetadataPath = "../diwl-contract/target/ink/metadata.json";

const data = readFileSync(wasmPath, 'utf8');
const contractWasm = JSON.parse(data);

// const contractAbi = await readFile(abiMetadataPath).then(json => JSON.parse(json)).catch(() => null);

// console.log(contractWasm);


// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

// initialise via static create
const api = await ApiPromise.create();

// // make a call to retrieve the current network head
// api.rpc.chain.subscribeNewHeads((header) => {
//   console.log(`Chain is at #${header.number}`);
// });

// Some mnemonic phrase
const PHRASE = 'entire material egg meadow latin bargain dutch coral blood melt acoustic thought';

// Add an account, straight mnemonic
const newPair = keyring.addFromUri(PHRASE);

// (Advanced) add an account with a derivation path (hard & soft)
const newDeri = keyring.addFromUri(`${PHRASE}//hard-derived/soft-derived`);

// (Advanced, development-only) add with an implied dev seed and hard derivation
const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

console.log(newPair);
console.log(newDeri);
console.log(alice);