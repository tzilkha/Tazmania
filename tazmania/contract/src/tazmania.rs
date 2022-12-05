use crate::merkle::MerkleTree;
use crate::utils::{hex_to_ark_fr, str_to_big};
use electron_rs::verifier::near::*;
use near_sdk::base64::encode;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId, Promise};
use num_bigint::BigUint;
use serde_json_wasm;

// Tazmania contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    merkle_tree: MerkleTree,
    amount: u128,
    nullifier_hashes: LookupMap<String, bool>,
    commitments: LookupMap<String, bool>,
    vkey: PreparedVerifyingKey,
}

// No default, we construct with second call once deployed
impl Default for Contract {
    fn default() -> Self {
        env::panic_str("Tazmania contract should be initialized before usage");
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(_height: u8, _amount: u128, _vkey: String) -> Self {
        // Check that if it has been instantiated and that its the deployer
        assert!(
            !env::state_exists(),
            "Error - tazmania already initialized."
        );
        assert!(
            env::signer_account_id() == env::current_account_id(),
            "Error - deployer must initialize."
        );

        // Check height is smaller than or eq to 25
        if _height > 25 {
            log!("Error - height must be maximum 25.");
            env::panic_str("Must be maximum height of 25.");
        }

        // Init verifier
        // let vkey = parse_verification_key(_vkey).expect("Error - cannot deserialize verification key.Err()");
        let vkey = parse_verification_key(_vkey).unwrap();
        let vkey = get_prepared_verifying_key(vkey);

        // Log init
        log!("Created a new Tazmania of height {}. YAY", _height);

        // Instantiate Tazmania
        Self {
            merkle_tree: MerkleTree::new(_height),
            amount: _amount,
            nullifier_hashes: LookupMap::<String, bool>::new(b'a'),
            commitments: LookupMap::<String, bool>::new(b'a'),
            vkey: vkey,
        }
    }

    #[payable]
    pub fn deposit(&mut self, commitment: String) -> bool {
        // TODO: Check money sent if greater than ammount
        if env::attached_deposit() < self.amount {
            env::panic_str("Insufficient deposit amount.");
        }

        // Check commitment is unique
        if !self.commitments.get(&commitment).is_none() {
            env::panic_str("Commitment has already been used before.");
        }

        // Insert commitment
        self.merkle_tree.insert(&commitment);

        // Log and keep track
        log!(
            "New commitment {} - New root {}.",
            commitment,
            self.get_root()
        );
        self.commitments.insert(&commitment, &true);

        return true;
    }

    pub fn withdraw(
        &mut self,

        // publics
        root: String,
        nullifier_hash: String,
        receipt_address: AccountId,
        relayer_address: AccountId,
        fee: u128,

        proof_str: String,
    ) {
        // Check nullifier is unique
        if !self.commitments.get(&nullifier_hash).is_none() {
            env::panic_str("Nullifier hash has been used before.");
            log!("Unsuccessful withdrawl - invalid nullifier hash.")
        }
        // Check if proof root is in proof history
        if !self.merkle_tree.is_root(&root) {
            env::panic_str("Outdated root behing used for proof.");
            log!("Unsuccessful withdrawl - outdated proof.")
        }

        // Check fee isn't greater than the amount transfer
        if fee >= self.amount {
            env::panic_str("Fee requested is larger the transfer amount.");
            log!("Unsuccessful withdrawl - fee larger than denomination.")
        }

        // Prepare public inputs
        let pub_inputs = vec![
            hex_to_ark_fr(&root),
            hex_to_ark_fr(&nullifier_hash),
            ark_bn254::Fr::from(str_to_big(receipt_address.as_str())),
            ark_bn254::Fr::from(str_to_big(relayer_address.as_str())),
            ark_bn254::Fr::from(fee),
        ];

        log!("FEE - {}", fee);

        // Prepare the proof
        let proof: CircomProofJson =
            serde_json_wasm::from_str(&proof_str).expect("Error - problem with proof string.");

        // Verify based on public inputs
        let v_res =
            ark_groth16::verify_proof(&self.vkey.clone().into(), &proof.into(), &pub_inputs[..])
                .expect("Error - problem verifying proof.");

        // Verification result
        if v_res == false {
            env::panic_str("Error - proof invalid.")
        }

        // Process withdrawl - send money-fee to recipient, send fee to relayer
        Promise::new(receipt_address).transfer(self.amount - fee);
        Promise::new(relayer_address).transfer(fee);

        // Add nullifier hash
        self.nullifier_hashes.insert(&nullifier_hash, &true);

        // Log success
        log!("Successful withdrawl.");
    }

    // Front facing get_root for people trying to create proofs
    pub fn get_root(&self) -> String {
        return self.merkle_tree.get_root();
    }

    // Front facing get_leaves for relayers maintaining local merkle
    pub fn get_leaves(&self) -> Vec<String> {
        return self.merkle_tree.get_leaves().clone();
    }

    // Front facing get_leaf for relayers maintaining local merkle
    pub fn get_leaf(&self, index: u32) -> String {
        return self.merkle_tree.get_leaves()[index as usize].clone();
    }

    // Front facing to get number of leaves for relayers
    pub fn n_leaves(&self) -> usize {
        return self.merkle_tree.get_leaves().len();
    }
}
