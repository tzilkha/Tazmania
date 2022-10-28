import { create_deposit } from './proof';
import { MerkleTree } from './merkle';
import {MimcSpongeHasher} from './hashers';
import * as snarkjs from 'snarkjs';

import path from "path";
import {toBufferLE, buff2hex, toBigIntLE, } from "./utils";
import { buildMimcSponge, buildPedersenHash, buildBabyJub } from 'circomlibjs';
import { randomBytes } from 'crypto';



export function hex2buff(hex){
	if (hex.substring(0,2) == '0x'){ hex = hex.substring(2, hex.length); }
	return Uint8Array.from(hex.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));
}

function buffer2bits(buff) {
    const res = [];
    for (let i = 0; i < buff.length; i++) {
        for (let j = 0; j < 8; j++) {
            if ((buff[i] >> j) & 1) {
                res.push('1');
            } else {
                res.push('0');
            }
        }
    }
    return res;
}

export async function t(){

	var dep = await create_deposit();
	console.log("secret", dep.get_secret());
	console.log("nullifier", dep.get_nullifier());
	console.log("nullifier hash", dep.get_nullifier_hash());
	console.log("comm", dep.get_commitment());




	var ms = new MimcSpongeHasher();
	await ms.build()

	var mt = new MerkleTree(25, [dep.get_commitment()], ms);

	const pr = mt.get_proof(0);

	const input = {
		'root': mt.get_root(),
    	'nullifierHash': dep.get_nullifier_hash(),
    	'nullifier': dep.get_nullifier(),
    	'secret': dep.get_secret(),
    	'pathElements': pr.path_elements,
    	'pathIndices': pr.path_indices
	}

	console.log(JSON.stringify(input));
	console.log();

	// const wasmPath = path.join("assets/circuit.wasm");
	// const zkeyPath = path.join("assets/circuit_final.zkey");

	// fs.readFileSync(wasmPath);

 //    let { proof, publicSignals } = await snarkjs.groth16.fullProve(wtns, wasmPath, zkeyPath);
    // const args = await genProofArgs(proof, publicSignals);

	// const res = await snarkjs.groth16.prove(zkey_final, wtns);

}