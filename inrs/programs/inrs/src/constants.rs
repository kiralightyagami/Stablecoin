use anchor_lang::prelude::*;


pub const CONFIG_SEED: &[u8] = b"config";
pub const MINT_SEED: &[u8] = b"mint";
pub const SOL_ACCOUNT_SEED: &[u8] = b"sol_account";
pub const COLLATERAL_SEED: &[u8] = b"collateral";

#[constant]
pub const USD_FEED_ID: &str = "0xfe650f0367d4a7ef9815a593ea15d36593f0643aaaf0149bb04be67ab851decd";
pub const MAX_AGE: u64 = 100;
pub const PRICE_FEED_DECIMALS_ADJUSTMENT: u128 = 10;


pub const MINT_DECIMALS: u8 = 9;
pub const LIQUIDATION_THRESHOLD: u64 = 50;
pub const LIQUIDATION_BONUS: u64 = 10;
pub const MIN_HEALTH_FACTOR: u64 = 1;

