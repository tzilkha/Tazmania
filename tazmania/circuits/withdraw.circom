include "../node_modules/circomlib/circuits/bitify.circom";
include "../node_modules/circomlib/circuits/pedersen.circom";
include "merkle.circom";

template commitmentCheck() {
	signal input secret;
	signal input nullifier;
	signal output nullifierHash;
	signal output commitment;

	component commitmentHasher = Pedersen(31 * 8 * 2);
	component nullifierHasher = Pedersen(31 * 8);
	component secretBits = Num2Bits(31 * 8);
	component nullifierBits = Num2Bits(31 * 8);


	nullifierBits.in <== nullifier;
	secretBits.in <== secret;


	for (var i = 0; i < 31 * 8; i++){
		commitmentHasher.in[i] <== secretBits.out[i];
		commitmentHasher.in[i + 31 * 8] <== nullifierBits.out[i];
		nullifierHasher.in[i] <== nullifierBits.out[i];
	}

	commitment <== commitmentHasher.out[0];
	nullifierHash <== nullifierHasher.out[0];

}

template withdraw(levels) {
	signal input root;
    signal input nullifierHash;
    signal private input nullifier;
    signal private input secret;
    signal private input pathElements[levels];
    signal private input pathIndices[levels];

    //signal input recipient; // not taking part in any computations
    //signal input relayer;  // not taking part in any computations
    //signal input fee;      // not taking part in any computations
    //signal input refund;   // not taking part in any computations

    // Produce commitment
    component hasher = commitmentCheck();
    hasher.nullifier <== nullifier;
    hasher.secret <== secret;

    // Check that the hasher is infact ok
    hasher.nullifierHash === nullifierHash;

    // Check that the secret and nullifier produce the root
    component tree = MerkleTreeCheck(levels);
    tree.leaf <== hasher.commitment;
    tree.root <== root;
    for (var i = 0; i < levels; i++) {
        tree.pathElements[i] <== pathElements[i];
        tree.pathIndices[i] <== pathIndices[i];
    }

    // Add hidden signals to make sure that tampering with recipient or fee will invalidate the snark proof
    // Most likely it is not required, but it's better to stay on the safe side and it only takes 2 constraints
    
    // Squares are used to prevent optimizer from removing those constraints
    //signal recipientSquare;
    //signal feeSquare;
    //signal relayerSquare;
    //signal refundSquare;
    //recipientSquare <== recipient * recipient;
    //feeSquare <== fee * fee;
    //relayerSquare <== relayer * relayer;
    //refundSquare <== refund * refund;

}

component main = withdraw(25);
