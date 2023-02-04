import * as url from 'url';
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

console.log(__dirname);

import { ApiPromise, Keyring } from '@polkadot/api';
import { readFile, readFileSync } from 'fs';

let wasmPath = __dirname + "test_contract/diwl_contract.contract";
let abiMetadataPath = __dirname + "test_contract/metadata.json";

const contractWasm = JSON.parse(readFileSync(wasmPath, 'utf8'));
const contractAbi = JSON.parse(readFileSync(abiMetadataPath, 'utf8'));

// initialise via static create
// const api = await ApiPromise.create({ rpc: "ws://localhost:9944" });
const api = await ApiPromise.create({ rpc: "wss://rpc.shibuya.astar.network" });

const keyring = new Keyring({ type: 'sr25519' });
const alicePair = keyring.addFromUri('//Alice', { name: 'Alice default' });

const wasm = contractWasm.source.wasm;

// maximum gas to be consumed for the instantiation. if limit is too small the instantiation will fail.
const gasLimit = api.registry.createType('WeightV2', {
    refTime: 6219235328,
    proofSize: 131072,
})

const storageDepositLimit = null;

export { alicePair, gasLimit, storageDepositLimit, api, contractAbi, wasm ,__dirname}
