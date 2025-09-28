use anchor_lang::prelude::*;
use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;
pub use error::*;
mod error;



declare_id!("3v3LMm9XD3eYBsehQAfkmewZRbJ7pCZukqFg7nVfE6Rn");

#[program]
pub mod Stablecoin {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        instructions::init_config(ctx)
    }
        
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        instructions::update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(ctx: Context<DepositCollateralAndMintTokens>, amount_collateral: u64, amount_to_mint: u64) -> Result<()> {
        instructions::deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(ctx: Context<RedeemCollateralAndBurnTokens>, amount_collateral: u64, amount_to_burn: u64) -> Result<()> {
        instructions::redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }
}


