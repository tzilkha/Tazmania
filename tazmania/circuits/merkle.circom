include "../node_modules/circomlib/circuits/mimcsponge.circom";
include "../node_modules/circomlib/circuits/poseidon.circom";

// Hash left node and right node to get parent hash in merkle
// uses the Mimc hash on left,right nodes
template HashLeftRight() {

	// get input signals
    signal input left;
    signal input right;

    // the output signal
    signal output hash;

    // instantiate the poseidon hasher
    component hasher = MiMCSponge(2, 220, 1);

    // signals in hasher
    hasher.inputs[0] <== left;
    hasher.inputs[1] <== right;

    // get the output
    hash <== hasher.out;
}

// if s == 0 returns [in[0], in[1]]
// if s == 1 returns [in[1], in[0]]
// Why do we do this
template DualMux() {

	// 2 input hashes
	signal input in[2];

	// the actual index
	signal input indice;

	// the outputs
	signal output outs[2];

	// if s == 0 returns [in[0], in[1]]
	// if s == 1 returns [in[1], in[0]]
	// also assert s equal to 0 or 1
	indice * (1 - indice) === 0; 
	outs[0] <== (in[1] - in[0]) * indice + in[0];
	outs[1] <== (in[0] - in[1]) * indice + in[1];
}

// Checking that the path hashes to the root given
template MerkleTreeCheck(levels) {

	// Get the leaf value and the root
	signal input leaf;
	signal input root;

	// Get the elements needed to hash along the path
	signal input pathElements[levels];

	// Get the indices of the path [0,1] vector 0 if left and 1 if right
	// Its simply the original indeces mod 2.
	signal input pathIndices[levels];

	// The hashes we will use for every level as well as selectors to distinguish
	// hashing order (remember we hash (left, right), order matters!)
	component hashers[levels];
	component selectors[levels];

	// From bottom up...
	for (var i = 0; i < levels; i++) {

		// Create a selector
		selectors[i] = DualMux();

		// If were at the bottom the value is actually the leaf value rather than
		// previous levels hash. Feed values in the DualMux as well as modded index
		selectors[i].in[0] <== i == 0 ? leaf : hashers[i - 1].hash;
		selectors[i].in[1] <== pathElements[i];
		selectors[i].indice <== pathIndices[i];

		// Hash the values according to the right order based on the DualMux
		hashers[i] = HashLeftRight();
		hashers[i].left <== selectors[i].outs[0];
		hashers[i].right <== selectors[i].outs[1];
		log("left", selectors[i].outs[0]);
		log("right", selectors[i].outs[1]);
		log("hashed", hashers[i].hash);
	}

	// At the end the last hashed value should be the input root to confirm the proof
	log("root", root);
	root === hashers[levels - 1].hash;
}
