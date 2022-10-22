use crate::merke::MerkleTree;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{log, near_bindgen};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    merkle_tree: MerkleTree,
    amount: u32,
    nullifier_hashes: LookupMap<String, bool>,
    commitments: LookupMap<String, bool>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        env::panic_str("Tazmania contract should be initialized before usage");
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(_height: u8, _amount: u32) -> Self {
        // Check that if it has been instantiated and that its the deployer
        // assert!(!env::state_exists(), "Already initialized");
        // assert!(env::signer_account_id() == env::current_account_id());

        // Log
        log!("Created a new Tazmania of height {}.", _height);

        // Instantiate Tazmania
        Self {
            merkle_tree: MerkleTree::new(_height),
            amount: _amount,
            nullifier_hashes: LookupMap::<String, bool>::new(b'a'),
            commitments: LookupMap::<String, bool>::new(b'a'),
        }
    }

    pub fn deposit(&mut self, commitment: String) -> bool {
        // TODO: Check money sent if greater than ammount

        // Check commitment is unique
        if !self.commitments.get(&commitment).is_none() {
            env::panic_str("Commitment has already been used before.");
        }

        // Insert commitment
        self.merkle_tree.insert(&commitment);

        // Log and keep track
        log!("New commitment {}.", commitment);
        self.commitments.insert(&commitment, &true);

        return true;
    }

    pub fn withdraw(&mut self) {
        todo!();
    }

    pub fn get_root(&self) -> String {
        return self.merkle_tree.get_root();
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_init() {
        let contract = Contract::new(33_u8, 10_u32);
    }

    #[test]
    fn good_init() {
        let contract = Contract::new(25_u8, 10_u32);
    }

    #[test]
    fn deposit() {
        let mut contract = Contract::new(25_u8, 10_u32);
        contract.deposit("0xffff".to_string());
    }
}
