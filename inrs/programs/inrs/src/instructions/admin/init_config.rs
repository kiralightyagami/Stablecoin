use anchor_lang::prelude::*;
use crate::{Config, CONFIG_SEED, MINT_SEED, MINT_DECIMALS, LIQUIDATION_THRESHOLD, LIQUIDATION_BONUS, MIN_HEALTH_FACTOR};
use anchor_spl::{token::{Mint, Token}, token_2022::Token2022, token_interface::TokenInterface};



pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
    *ctx.accounts.config_account= Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account,
    };
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [MINT_SEED],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,

    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}