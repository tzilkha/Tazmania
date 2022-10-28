import { buildMimcSponge, buildPedersenHash } from 'circomlibjs';
import { buff2hex, toBigIntLE } from './utils';

// MimcSponge hasher
export class MimcSpongeHasher {
	constructor(){
		this.hasher =  null;
	}

	async build() {
		this.hasher = await buildMimcSponge();
	}


  	hash(left, right) {
  		var r = this.hasher.multiHash([BigInt(left), BigInt(right)]);
  		return "0x" + this.hasher.F.toString(r, 16)
  	}
}




// Pedersen Hasher
export class PedersenHasher {
	constructor(){
		this.hasher = null;
	}

	async build() {
		this.hasher = await buildPedersenHash();
	}

  	hash(data) {
  		var point = this.hasher.hash(data);
  		var unpacked = this.hasher.babyJub.unpackPoint(point)[0];
  		return '0x' + this.hasher.babyJub.F.toString(unpacked, 16);
  	}
}