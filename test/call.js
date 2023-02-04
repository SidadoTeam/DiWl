import { gasLimit, alicePair, api, contractAbi, __dirname } from "./base.js";
import { ContractPromise } from '@polkadot/api-contract';
import fs from 'fs'

const address_contract = fs.readFileSync(__dirname + "address_contract.txt", "utf-8");
const contract = new ContractPromise(api, contractAbi, address_contract);

contract.tx
    .flip({ gasLimit: gasLimit })
    .signAndSend(alicePair, async r => {
        if (r.status.isInBlock || r.status.isFinalized) {
            const { gasRequired, storageDeposit, result, output } = await contract.query.getw(
                alicePair.address, { gasLimit: gasLimit, storageDepositLimit: null }
            );
            console.log(output.toHuman());
            process.exit(0);
        }
    });
