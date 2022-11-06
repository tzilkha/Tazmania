const ZERO = "0x74617a6d616e6961"; // Encoded "tazmania" string in hex

export class MerkleTree {

	constructor(levels=20, leaves = [], hasher){
		// Number of levels in the tree (longest path from root to leaf)
		this.levels = levels;

		// The zero hash of the first level
		this.zero = ZERO;

		// Number of leaves stored
		this.n_leaves = leaves.length;

		// Number of leaves already updated
		this.n_updated = 0
		
		// Saving hashed results
		this.hashes = new Map();
		for (var i = 0; i <= this.levels; i++){
			this.hashes.set(i, new Map());
		}

		// Instantiate the hasher
		this.hasher = hasher;

		// Create the zero hashes
		this.zeros = [this.zero];
		for (var i = 1; i <= this.levels; i++){
			this.zeros.push(this.hasher.hash(this.zeros[i-1], this.zeros[i-1]))
		}

		// CHECK THAT NUMBER OF LEAVES IS LESS THAN MAX NUMBER OF LEAVES

		// Populate the merkle tree with given input
		for (var i = 0; i < this.n_leaves; i++){
			this.hashes.get(0).set(i, leaves[i]);
		}
		
		// Update the tree based on given configuration
		if (this.n_leaves > 0) { this.update(); }
	}

	update(){
		// Check if there are new leaves to update
		if (this.n_leaves == this.n_updated) { return }

		var current_min_idx = this.n_updated;
		var current_max_idx = this.n_leaves;

		// console.log(current_min_idx, current_max_idx);
		for (var i = 0; i < this.levels; i++){

			current_min_idx = Math.floor((current_min_idx) / 2);
			current_max_idx = Math.ceil((current_max_idx) / 2);

			for (var j = current_min_idx; j < current_max_idx ; j++){
				const l_hash = this.get_hash(i, 2  * j);
				const r_hash = this.get_hash(i, 2  * j + 1);;
				const h = this.hasher.hash(l_hash, r_hash);
				this.put_hash(i + 1, j, h);
			}
		}

		this.n_updated = this.n_leaves;
	}

	insert(hashes){
		if (!Array.isArray(hashes)){
			hashes = [hashes];
		}

		// Leaves can't exceed maximum
		if (hashes.length + this.n_leaves > 2**this.levels){
			console.log("Can't insert leaves as they exceed capacity of tree.");
			return
		}

		for (var i = 0; i < hashes.length; i++){
			this.put_hash(0, this.n_leaves + i, hashes[i]);
		}
		this.n_leaves += hashes.length;

	}

	// Get the stored hash of an index at a certain level, 
	// if not stored returns the zeros hash at that level
	get_hash(level, index){
		return this.hashes.get(level).get(index) || this.zeros[level];
	}

	// Insert a new hash into an index on a level
	put_hash(level, index, hash){
		this.hashes.get(level).set(index, hash);
	}

	// Return 
	get_zeros(){
		return this.zeros;
	}

	// Returns the root of the merkle tree
	get_root(){
		return this.hashes.get(this.levels).get(0) || this.zeros[this.levels];
	}

	get_leaves(){
		return this.hashes.get(0);
	}

	// 
	get_proof(idx){
		// Check that the index is acctually within the leaves stored in this merkle tree
		if (idx >= this.n_updated) {
			console.log(idx, "not stored, can't generate proof.")
			return
		}

		var leaf = this.get_hash(0, idx);
		var root = this.get_root();
		var path = [];
		var path_values = [];

		for (var i = 0; i < this.levels; i++){
			path.push(idx % 2);
			path_values.push(this.get_hash(i, idx + 1 - 2 * (idx % 2)));

			idx = Math.floor(idx / 2);
		}

		return {
			"leaf": leaf,
			"root": root,
			"path_indices": path,
			"path_elements": path_values
		}		
	}
}

