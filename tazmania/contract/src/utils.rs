use ff::*;
use mimc_sponge_rs::Fr;
use near_sdk::env;
use num_bigint::BigUint;
use std::str::FromStr;
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

pub fn hex_to_ark_fr(hex: &str) -> ark_bn254::Fr {
    let n = hex.parse::<U256>().unwrap().to_string();
    return ark_bn254::Fr::from_str(n.as_str()).unwrap();
}

// str -> sha256 -> 32 byte array -> BigUint
pub fn str_to_big(input: &str) -> BigUint {
    let b = env::sha256(format!("{}", input).as_bytes());
    return BigUint::from_radix_be(&b, 256).unwrap();
}
