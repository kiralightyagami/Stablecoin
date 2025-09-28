use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{state::Collateral, Config, COLLATERAL_SEED, CONFIG_SEED, SOL_ACCOUNT_SEED, deposit_sol, mint_tokens, check_health_factor};
use anchor_spl::{token_2022::Token2022, token_interface::{Mint, TokenAccount}, associated_token::AssociatedToken};

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = config_account.bump,
        has_one = mint_account,
        
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [COLLATERAL_SEED, depositor.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [SOL_ACCOUNT_SEED, depositor.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn deposit_collateral_and_mint_tokens(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;
    
    if !collateral_account.is_init {
        collateral_account.is_init = true;
        collateral_account.depositor_account = ctx.accounts.depositor.key();
        collateral_account.sol_account= ctx.accounts.sol_account.key();
        collateral_account.token_account= ctx.accounts.token_account.key();
        collateral_account.sol_account_bump = ctx.bumps.sol_account;
        collateral_account.bump = ctx.bumps.collateral_account;
    }
    
    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;
    

    deposit_sol(
        &ctx.accounts.system_program,
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        amount_collateral,
    )?;

    mint_tokens(
        &ctx.accounts.token_program,
        ctx.accounts.config_account.bump_mint_account,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        amount_to_mint,
    )?;
    

    Ok(())
}