import { ApiPromise, Keyring } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider/ws';
import { ContractPromise } from '@polkadot/api-contract';
import { contractAbi, address_contract } from './data.js'

const uri = "ws://localhost:9944";
// const uri = "wss://rpc.shibuya.astar.network";

let apiPromise, api, alicePair;
const storageDepositLimit = null;

const keyring = new Keyring({ type: 'sr25519' });
const provider = new WsProvider(uri);
apiPromise = ApiPromise.create({ provider }).then((_api) => {
    api = _api;
    alicePair = keyring.addFromUri('//Alice');
});

async function init() {
    if (api == null) {
        console.error("api is null");
        // return [];
        await apiPromise;
    }

}

export async function getwCommon(page_index, page_size) {
    await init();
    let gasLimit = api.registry.createType('WeightV2', {
        refTime: 11168207347 * 40,
        proofSize: 131072 * 40
    });
    let contract = new ContractPromise(api, contractAbi, address_contract);
    const { gasRequired, storageDenposit, debugMessage, result, output } = await contract.query.getwCommon(
        alicePair.address, { gasLimit: gasLimit, storageDepositLimit: null }, page_index, page_size
    );
    if (debugMessage != null && !debugMessage.isEmpty)
        console.log(debugMessage.toHuman());
    if (output != null) {
        //console.log(output.toHuman());
        return output.toHuman();
    }
    return []
}

export async function getwUser(page_index, page_size) {
    await init();
    let gasLimit = api.registry.createType('WeightV2', {
        refTime: 11168207347 * 40,
        proofSize: 131072 * 40
    });
    let contract = new ContractPromise(api, contractAbi, address_contract);
    const { gasRequired, storageDenposit, debugMessage, result, output } = await contract.query.getwUser(
        alicePair.address, { gasLimit: gasLimit, storageDepositLimit: null }, page_index, page_size
    );
    if (debugMessage != null && !debugMessage.isEmpty)
        console.log(debugMessage.toHuman());
    if (output != null) {
        //console.log(output.toHuman());
        return output.toHuman();
    }
    return []
}

export async function userWordIn(word, mean, level) {
    return new Promise(async function (resolve, reject) {
        await init();
        let gasLimit = api.registry.createType('WeightV2', {
            refTime: 11168207347 * 1,
            proofSize: 131072 * 1
        });
        let contract = new ContractPromise(api, contractAbi, address_contract);
        await contract.tx
            .userWordIn({ gasLimit: gasLimit }, { word: word, mean: mean, level: level })
            .signAndSend(alicePair, async r => {
                if (r.status.isInBlock || r.status.isFinalized) {
                    if (r.dispatchError != null) {
                        console.log(r.toHuman());
                        reject(r.toHuman());
                    } else {
                        console.log("ok");
                        resolve("ok");
                    }
                }
            });
    });
}

export async function userWordUpdate(word, mean, level, id) {
    return new Promise(async function (resolve, reject) {
        await init();
        let gasLimit = api.registry.createType('WeightV2', {
            refTime: 11168207347 * 40,
            proofSize: 131072 * 40
        });
        let contract = new ContractPromise(api, contractAbi, address_contract);
        await contract.tx
            .userWordUpdate({ gasLimit: gasLimit }, { word: word, mean: mean, level: level }, id)
            .signAndSend(alicePair, async r => {
                if (r.status.isInBlock || r.status.isFinalized) {
                    if (r.dispatchError != null) {
                        console.log(r.toHuman());
                        reject(r.toHuman());
                    } else {
                        resolve("ok");
                        console.log("ok");
                    }
                }
            });
    });


}


export { apiPromise }

console.log("hello lib.js");
console.log(uri);
try {
    window.getwCommon = getwCommon;
    window.getwUser = getwUser;
    window.userWordIn = userWordIn;
    window.userWordUpdate = userWordUpdate;
} catch {

}