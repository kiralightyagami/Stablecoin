use anchor_lang::prelude::*;


pub const CONFIG_SEED: &[u8] = b"config";
pub const MINT_SEED: &[u8] = b"mint";
pub const MINT_DECIMALS: u8 = 9;
pub const LIQUIDATION_THRESHOLD: u64 = 50;
pub const LIQUIDATION_BONUS: u64 = 10;
pub const MIN_HEALTH_FACTOR: u64 = 1;
