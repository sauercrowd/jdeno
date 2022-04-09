import {encryptJSON, decryptJSON} from './lockbox.js';


const enc = encryptJSON({1:1}, "hello world");
console.log(decryptJSON(enc, "hello world"));

globalThis.i = 1;