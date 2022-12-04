use crate::merkle::MerkleTree;
use electron_rs::verifier::near::*;
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
        let v_res = verify_proof(self.vkey.clone(), proof, public).unwrap();
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

/*
 *
 *          Tazmania Tests
 *
 */
#[cfg(test)]
mod tests {
    use super::*;

    const vkey: &str = r#"
        {
         "protocol": "groth16",
         "curve": "bn128",
         "nPublic": 2,
         "vk_alpha_1": [
          "20491192805390485299153009773594534940189261866228447918068658471970481763042",
          "9383485363053290200918347156157836566562967994039712273449902621266178545958",
          "1"
         ],
         "vk_beta_2": [
          [
           "6375614351688725206403948262868962793625744043794305715222011528459656738731",
           "4252822878758300859123897981450591353533073413197771768651442665752259397132"
          ],
          [
           "10505242626370262277552901082094356697409835680220590971873171140371331206856",
           "21847035105528745403288232691147584728191162732299865338377159692350059136679"
          ],
          [
           "1",
           "0"
          ]
         ],
         "vk_gamma_2": [
          [
           "10857046999023057135944570762232829481370756359578518086990519993285655852781",
           "11559732032986387107991004021392285783925812861821192530917403151452391805634"
          ],
          [
           "8495653923123431417604973247489272438418190587263600148770280649306958101930",
           "4082367875863433681332203403145435568316851327593401208105741076214120093531"
          ],
          [
           "1",
           "0"
          ]
         ],
         "vk_delta_2": [
          [
           "17608963753378099486245458723771923191425930747139112370610976690633207576600",
           "502327822159726970000387749281741394036053489703183429816461629884219340919"
          ],
          [
           "6381936594791359602770991568909069629025254801614368938598939969718914424925",
           "11353215306399907187438531994284586041711480912639561106259508619293648058580"
          ],
          [
           "1",
           "0"
          ]
         ],
         "vk_alphabeta_12": [
          [
           [
            "2029413683389138792403550203267699914886160938906632433982220835551125967885",
            "21072700047562757817161031222997517981543347628379360635925549008442030252106"
           ],
           [
            "5940354580057074848093997050200682056184807770593307860589430076672439820312",
            "12156638873931618554171829126792193045421052652279363021382169897324752428276"
           ],
           [
            "7898200236362823042373859371574133993780991612861777490112507062703164551277",
            "7074218545237549455313236346927434013100842096812539264420499035217050630853"
           ]
          ],
          [
           [
            "7077479683546002997211712695946002074877511277312570035766170199895071832130",
            "10093483419865920389913245021038182291233451549023025229112148274109565435465"
           ],
           [
            "4595479056700221319381530156280926371456704509942304414423590385166031118820",
            "19831328484489333784475432780421641293929726139240675179672856274388269393268"
           ],
           [
            "11934129596455521040620786944827826205713621633706285934057045369193958244500",
            "8037395052364110730298837004334506829870972346962140206007064471173334027475"
           ]
          ]
         ],
         "IC": [
          [
           "14197337208806770545889220037358805071954256513497618666763526856238699710862",
           "9453653003318842103590170162898346038399530062335296624636426850906339758932",
           "1"
          ],
          [
           "7374419710074923783649892211890616707963750737341310197238451007583332192797",
           "4777866817750211338926228749453665894287783251391950411282042253958828314422",
           "1"
          ],
          [
           "5944135141027406582923479745934505228510427791131178445052716386361609615767",
           "12206970540198405095222264634510513580619878699676273073206648968392358034589",
           "1"
          ]
         ]
        }
    "#;

    #[test]
    fn deposit() {
        let mut contract = Contract::new(25_u8, 10_u128, vkey.to_string());
    }
}
