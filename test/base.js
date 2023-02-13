import * as url from 'url';
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

console.log(__dirname);

import { ApiPromise, Keyring } from '@polkadot/api';
import { readFile, readFileSync } from 'fs';
import { WsProvider } from '@polkadot/rpc-provider/ws';

// let wasmPath = __dirname + "test_contract/diwl_contract.contract";
// let abiMetadataPath = __dirname + "test_contract/metadata.json";

let wasmPath = __dirname + "../diwl-contract/target/ink/diwl_contract.contract";
let abiMetadataPath = __dirname + "../diwl-contract/target/ink/metadata.json";

const contractWasm = JSON.parse(readFileSync(wasmPath, 'utf8'));
const contractAbi = JSON.parse(readFileSync(abiMetadataPath, 'utf8'));

// initialise via static create
const uri = "wss://rpc.shibuya.astar.network";
// const uri = "ws://localhost:9944";
const provider = new WsProvider(uri);
const api = await ApiPromise.create({ provider });
// const api = await ApiPromise.create({ rpc: "ws://localhost:9944" });
// const api = await ApiPromise.create({ rpc: "wss://rococo-contracts-rpc.polkadot.io" });
// const api = await ApiPromise.create({ rpc: "wss://ws.test.azero.dev" });
// const api = await ApiPromise.create({ rpc: "wss://rpc.shiden.astar.network" });
// const api = await ApiPromise.create({ rpc: "wss://rpc.shibuya.astar.network" });

const keyring = new Keyring({ type: 'sr25519' });
// const alicePair = keyring.addFromUri('//Alice', { name: 'Alice default' });
const alicePair = keyring.addFromUri('//Alice');

const wasm = contractWasm.source.wasm;

//ZfRoeLUReVfGf92nx2GWfTSMEZ9VFAZQxUPQgdAq8T5AvUe
// maximum gas to be consumed for the instantiation. if limit is too small the instantiation will fail.
const gasLimit = api.registry.createType('WeightV2', {
    refTime: 11168207347 * 1,
    proofSize: 131072 * 1
})

const storageDepositLimit = null;

export { alicePair, gasLimit, storageDepositLimit, api, contractAbi, wasm, __dirname }
