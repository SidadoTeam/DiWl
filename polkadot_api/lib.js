import { ApiPromise, Keyring } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider/ws';
import { ContractPromise } from '@polkadot/api-contract';
import { contractAbi, address_contract } from './data.js'

// const uri = "ws://localhost:9944";
let apiPromise, api, alicePair;
const storageDepositLimit = null;

const keyring = new Keyring({ type: 'sr25519' });

const uri = "wss://rpc.shibuya.astar.network";
const provider = new WsProvider(uri);
apiPromise = ApiPromise.create({ provider }).then((_api) => {
    api = _api;
    alicePair = keyring.addFromUri('//Alice');
});

// //构造异步的请求函数
var getwCommon = async function (page_index, page_size) {
    if (api == null) {
        console.error("api is null");
        // return [];
        await apiPromise;
    }
    const gasLimit = api.registry.createType('WeightV2', {
        refTime: 11168207347 * 40,
        proofSize: 131072 * 40
    })

    const contract = new ContractPromise(api, contractAbi, address_contract);
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

export { getwCommon, apiPromise }

console.log("hello lib.js");
console.log(uri);
try {
    window.getwCommon = getwCommon;
}catch{
    
}