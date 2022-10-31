use crate::utils::hex_to_fr;
use ff::*;
use mimc_sponge_rs::{Fr, MimcSponge};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MerkleTree {
    pub levels: u8,
    pub leaves: Vec<String>,
    pub next_index: u32,
    pub root_index: u8,
    pub nullifier_hashes: Vec<String>,
    pub root_history: Vec<String>,
    pub state: Vec<String>,
}

impl MerkleTree {
    pub fn new(levels: u8) -> Self {
        if levels > 32 {
            env::panic_str("Must be at most height 32.");
        }

        let mut state: Vec<String> = Vec::with_capacity(levels as usize);
        for i in 0..levels {
            state.push(ZEROS[i as usize].to_owned());
        }

        Self {
            leaves: Vec::new(),
            nullifier_hashes: Vec::new(),
            root_history: vec!["EMPTY".to_string(); HISTORY_SIZE as usize],
            levels: levels,
            next_index: 0 as u32,
            root_index: 0 as u8,
            state: state,
        }
    }

    // Get most recent root hash value
    pub fn get_root(&self) -> String {
        return self.root_history[self.root_index as usize].clone();
    }

    // Insert commitment hash as leaf to merkle tree
    pub fn insert(&mut self, leaf: &str) {
        let mut current_index: u32 = self.next_index;
        if (current_index as u64) >= 2_u64.pow(self.levels as u32) {
            env::panic_str("Tree is full, cannot add more commits.");
        }

        // Instantiate hasher
        let k = Fr::zero();
        let mt = MimcSponge::default();

        self.next_index += 1;
        let mut current_hash: &str = leaf;
        let mut left: &str;
        let mut right: &str;
        let mut hashed = "".to_string();

        for i in 0..self.levels {
            if current_index % 2 == 0 {
                left = current_hash;
                right = ZEROS[i as usize];
                self.state[i as usize] = current_hash.to_string();
            } else {
                left = self.state[i as usize].as_str();
                right = current_hash;
            }

            let inputs = vec![hex_to_fr(left), hex_to_fr(right)];
            hashed = mt.multi_hash(&inputs, k, 1)[0].to_string();

            // Remove "Fr(" and ")" around Fr string
            current_hash = &hashed[3..69];
            current_index /= 2;
        }

        self.root_index = (self.root_index + 1) % HISTORY_SIZE;
        self.root_history[self.root_index as usize] = current_hash.to_string();
    }

    // Check if given string is a valid root from history
    pub fn is_root(&self, r: String) -> bool {
        let mut i = self.root_index;
        loop {
            if self.root_history[i as usize] == r {
                return true;
            }
            i = (i + 1) % HISTORY_SIZE;
            if i == self.root_index {
                return false;
            }
        }
    }

    pub fn get_leaves(&self) -> Vec<String> {
        return self.leaves.clone();
    }
}

// TODO: Make this instantiated through tazmania init
const HISTORY_SIZE: u8 = 100;

const ZEROS: [&str; 33] = [
    "0x74617a6d616e6961",
    "0x305a2b117cf07880b964f7eddd20799b297ee80ef8537c5bc7833f8d2d571f5f",
    "0x16050be587c66d0924f67ff0d832588a2092e9595f6df64448bbf380de80c613",
    "0x299c64975a1e97de11df9a92c30e8c7973bd821848cd994b7f1d95fa24d72ea4",
    "0x3191d1e658da25d6559ffa1adcd5df8b60671a395de828d51096eafbc17a4b9",
    "0x8f85441590dceea0e77beff6fc04382b635c02bffd2a385772dd6c840662b37",
    "0x256bb96340f6f87eec151ad4e3a2fcc6f46bbeacd5564dbf4b25c35bb7da24ea",
    "0x1627f7bda5de58f1d1a9e164b1bc384e08baf384d80fce20b920651757398148",
    "0x2f7a83cd822d5f6a8b20800595cfad1164a505d00d20f9f32719c30c6d662c9e",
    "0x104619aeaf863a0597f5a74bb220351557d77958aab121c69ca4969fe266a873",
    "0x2a24dd75614a7075baf37ecf331987f7e92f1c27e5219e7a2c54dad96e817410",
    "0x2b98411279138795c98d5ff5bdbc735ddf68aaa49c82a925fd2e8726836a1c35",
    "0x278e0a47f28268dc349c51d0638369762579fa91019d961d11ffe7b84bbdf5a8",
    "0x1dcda51c67c1f4291895bf129c25bbe3dc4c3a8354bacd6f43b79ee02212e7d7",
    "0x17f307d50ca48da8e0ac6922f993002377f0f1b1730e1896bb2d26d177d237",
    "0x180cfde390a54fbb9c76623cb2f5deb418a95ae0f88c4c8864a226ad133aa7ee",
    "0x1cd23cbd9d5283608729666068e0a7d992e0068940a5584938e886321b6b0bbe",
    "0x2eaa00c715706ab4a3f1f68b1906abeb0d38d2e10c31722095d77cd2d301f92",
    "0x225396dc4d29f7209d12aa146e65b2e88866e7412ac7dc4d2eb70fe2ba77d13a",
    "0x18b9ca61a0f191987f4b34504a8273c6a48c7a0e07ba01a80ecac55ad4c8b189",
    "0x12fb8776ec68aaacacaa497e23c9f858a3e0fa24a8e789f79cb107b78228893d",
    "0x1345bad939c9b7985aa39ddfd77755d0355b514045e039e7be7b21e73e577dbb",
    "0x3e3f11eca024407231d3f79d6204b19695fdfd78fda7a4d870ca5ce94fc9cbd",
    "0x267fb7fa2a2ca309e00ee9926623d813e90889b187b1684d8d6b3bcedbd5f12e",
    "0x208f7f347b8cf57fc8dfcdd51131ee7e6f7747f19ee67dc65125146ae5a74d0c",
    "0x5a2a396ae1e1f4f8afddb6180fb9467e8a9a7af762c2efdf29361fa24e23755",
    "0x1661c398cdccbd6575de1784b65ae52fcb75da3001bae2d2a66cf15da799297b",
    "0x8e7c7a0f3ef397910596551255346a12ed7e085e03d183118ee2767b0e35abe",
    "0x1b79cec93fe151e31faccabacdad6d5582a41629b9042a1a12bd699f45f421df",
    "0x27a081a2c370883df77821d25ff93455a7908d2d23f4494a7b20ed70f1ca6534",
    "0x4b7c361f93ffe2dca302e7db9668cb413e8cb1da693695f1f8b21bbd8af78a8",
    "0x23d1dc906f135295f804e532d09587c9f01a6f0d91798d63b265cec16485d3d8",
    "0x97281050c346c248a95edb496431947ece5b7089cd01cc720d210b0c6565b8e",
];
