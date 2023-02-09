import { gasLimit, alicePair, api, contractAbi, __dirname } from "./base.js";
import { ContractPromise } from '@polkadot/api-contract';
import fs from 'fs'

const address_contract = fs.readFileSync(__dirname + "address_contract.txt", "utf-8");
const contract = new ContractPromise(api, contractAbi, address_contract);

// console.log(contract.tx);

//读文件 跳过第一行


const data = fs.readFileSync('test/word_list.txt', 'UTF-8');

// split the contents by new line
const lines = data.split(/\r?\n/);
console.log(lines.length);

let word_list = [];

for (var i = 1; i < lines.length; i++) {
    let line = lines[i];
    let w = line.split(",");
    word_list.push({ word: w[6], mean: w[5], level: w[3] });
}

let writCount = 0;
async function writLine() {
    if (writCount > word_list.length) {
        return;
    }
    console.log(writCount);
    await contract.tx
        .initWlist({ gasLimit: gasLimit }, word_list[writCount])
        .signAndSend(alicePair, async r => {
            if (r.status.isInBlock || r.status.isFinalized) {
                writLine();
                if (r.dispatchError != null) {
                    console.log(r.toHuman());
                } else {
                    console.log("ok");
                }
            }
        });
    writCount++;
}

await writLine();

// console.log(word_list);
//process.exit(0);
// contract.tx
//     .initWlist({ gasLimit: gasLimit }, word_list)
//     .signAndSend(alicePair, async r => {
//         if (r.status.isInBlock || r.status.isFinalized) {
//             console.log(r.toHuman());
//             console.log("end");
//             process.exit(0);
//         }
//         // console.log(r.toHuman());
//     });
