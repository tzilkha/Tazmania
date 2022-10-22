use ff::*;
use mimc_sponge_rs::Fr;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

// hex str -> Fr
pub fn hex_to_fr(hex: &str) -> Fr {
    let n = hex.parse::<U256>().unwrap().to_string();
    return Fr::from_str(n.as_str()).unwrap();
}
