const snarkjs = require('snarkjs');
const vk = require('./verification_key.json')
const { relayer_id, relayer_key, tazmania_id, fee } = require('./config')
const { keyStores, KeyPair, connect, Contract } = require('near-api-js');



async function contract_withdraw(public, proof) {
	const myKeyStore = new keyStores.InMemoryKeyStore();
	const keyPair = KeyPair.fromString(relayer_key);
	await myKeyStore.setKey("testnet", relayer_id, keyPair);

	const connectionConfig = {
	  networkId: "testnet",
	  keyStore: myKeyStore, 
	  nodeUrl: "https://rpc.testnet.near.org",
	  walletUrl: "https://wallet.testnet.near.org",
	  helperUrl: "https://helper.testnet.near.org",
	  explorerUrl: "https://explorer.testnet.near.org",
	};
	const nearConnection = await connect(connectionConfig);


	const account = await nearConnection.account(relayer_id);


	// Load contract 
	const contract = new Contract(
	  account, // the account object that is connecting
	  tazmania_id,
	  {
	    // name of contract you're connecting to
	    // viewMethods: ["getMessages"], // view methods do not change state but usually return a value
	    changeMethods: ["withdraw"], // change methods modify state
	  }
	);

	const res = await contract.withdraw({
		public: JSON.stringify(public),
        proof: JSON.stringify(proof),

        nullifier_hash: "0x" + BigInt(public[1]).toString(16).padStart(64, 0),
        root: "0x" + BigInt(public[0]).toString(16).padStart(64, 0),
        fee: fee,

        receipt_address: relayer_id,
        relayer_address: relayer_id,
	},
	gas = 300000000000000)

	console.log(res);

}


async function withdraw(req, res) {
    const verify_res = await snarkjs.groth16.verify(vk, req.body.public_signals, req.body.proof);
    if (verify_res == true) {
    	console.log("Proof verified!");
    	await contract_withdraw(req.body.public_signals, req.body.proof)
    }
	
}

module.exports = {withdraw}

