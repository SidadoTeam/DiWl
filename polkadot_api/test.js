import { apiPromise, getwCommon, getwUser, userWordIn, userWordUpdate } from './lib.js'

await apiPromise;

let res = await getwCommon(1, 200);
console.log("getwCommon:" + res.length);

// await userWordIn("hello", "你好", "0");

// setTimeout(async () => {
//     let ress = await getwUser(1, 200);
//     console.log(ress);
//     await userWordUpdate("hello", "你好啊", 0, 0);
// }, 2000);
let x = await userWordUpdate("hello", "你好啊", 3, 0);
console.log(x);
// setTimeout(async () => {
//     let ress = await getwUser(1, 200);
//     console.log(ress);
//     process.exit(0);
// }, 2000);
