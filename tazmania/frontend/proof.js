import { MerkleTree } from './merkle';
import { MimcSpongeHasher, PedersenHasher} from './hashers';
import { randomBytes } from 'crypto';

import {toBufferLE, toBigIntLE} from './utils';

export class Deposit{
	constructor(hasher, old_deposit = null) {
		if (old_deposit == null){
			this.nullifier = randomBytes(31);
			this.secret = randomBytes(31);			
		}
		else {
			this.nullifier = toBufferLE(BigInt(old_deposit.nullifier), 31);
			this.secret = toBufferLE(BigInt(old_deposit.secret), 31);
		}

		this.nullifierHash = hasher.hash(this.nullifier);
		this.commitment = hasher.hash(Buffer.concat([this.secret, this.nullifier]));
	}
	get_private(){
		return {
			'secret': '0x' + toBigIntLE(this.secret).toString(16),
			'nullifier': '0x' + toBigIntLE(this.nullifier).toString(16)
		}
	}
	get_commitment() { return '0x' + this.commitment.toString(16); }
	get_nullifier_hash() { return '0x' + this.nullifierHash.toString(16); }
	get_secret() { return '0x' + toBigIntLE(this.secret).toString(16); }
	get_nullifier() { return '0x' + toBigIntLE(this.nullifier).toString(16); }
}

export async function create_deposit(old_deposit = null) {
	var ph = new PedersenHasher();
	await ph.build();

	var dep = new Deposit(ph, old_deposit);
	return dep;
}