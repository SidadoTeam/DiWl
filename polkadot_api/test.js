import { apiPromise, getwCommon } from './lib.js'

await apiPromise;

let res = await getwCommon(1, 200);
console.log(res);
