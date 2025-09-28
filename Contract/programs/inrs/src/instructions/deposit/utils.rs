use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token::{mint_to}, token_2022::{MintTo, Token2022}, token_interface::{Mint, TokenAccount}};
use crate::{constants::MINT_SEED};

pub fn mint_tokens<'info>(
    token_program: &Program<'info, Token2022>,
    bump: u8,
    mint_account: &InterfaceAccount<'info, Mint>,
    to_account: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,

) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[MINT_SEED, &[bump]]];
    
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint_account.to_account_info(),
                to: to_account.to_account_info(),
                authority: mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}


pub fn deposit_sol<'info>(
    system_program: &Program<'info, System>,
    from_account: &Signer<'info>,
    to_account: &SystemAccount<'info>,
    amount: u64,
) -> Result<()> {
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: from_account.to_account_info(),
                to: to_account.to_account_info(),
            },
        )
    ,
    amount,
    )
}