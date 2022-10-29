use crate::merkle::MerkleTree;
use crate::verifier::Verifier;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId, Promise};

// Tazmania contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    merkle_tree: MerkleTree,
    amount: u128,
    nullifier_hashes: LookupMap<String, bool>,
    commitments: LookupMap<String, bool>,
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
    pub fn new(_height: u8, _amount: u128) -> Self {
        // Check that if it has been instantiated and that its the deployer
        assert!(!env::state_exists(), "Already initialized");
        assert!(env::signer_account_id() == env::current_account_id());

        // Check height is smaller than or eq to 25
        if _height > 25 {
            log!("Error - height must be maximum 25.");
            env::panic_str("Must be maximum height of 25.");
        }

        // Log init
        log!("Created a new Tazmania of height {}.", _height);

        // Instantiate Tazmania
        Self {
            merkle_tree: MerkleTree::new(_height),
            amount: _amount,
            nullifier_hashes: LookupMap::<String, bool>::new(b'a'),
            commitments: LookupMap::<String, bool>::new(b'a'),
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
        public: String,
        proof: String,

        nullifier_hash: String,
        root: String,
        fee: u128,

        receipt_address: AccountId,
        relayer_address: AccountId,
    ) {
        // Check nullifier is unique
        if !self.commitments.get(&nullifier_hash).is_none() {
            env::panic_str("Nullifier hash has been used before.");
            log!("Unsuccessful withdrawl - invalid nullifier hash.")
        }
        // Check if proof root is in proof history
        if !self.merkle_tree.is_root(root) {
            env::panic_str("Outdated root behing used for proof.");
            log!("Unsuccessful withdrawl - outdated proof.")
        }

        // Check fee isn't greater than the amount transfer
        if fee >= self.amount {
            env::panic_str("Fee requested is larger the transfer amount.");
            log!("Unsuccessful withdrawl - fee larger than denomination.")
        }

        log!("{}", receipt_address);

        // Verify the proof
        let verifier = Verifier::new();
        let v_res = verifier.verify(proof, public);
        if !v_res {
            env::panic_str("Invalid proof.");
            log!("Unsuccessful withdrawl - invalid proof.")
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
}

/*
 *
 *          Tazmania Tests
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let contract = Contract::new(25_u8, 10_u128);
    }

    #[test]
    fn deposit() {
        let mut contract = Contract::new(25_u8, 10_u128);
        contract.deposit("0xffff".to_string());
    }
}
