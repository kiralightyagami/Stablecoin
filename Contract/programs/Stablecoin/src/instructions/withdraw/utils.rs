use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token_2022::{burn, Burn}, token_interface::{Mint, TokenAccount, Token2022}};
use crate::SOL_ACCOUNT_SEED;

pub fn withdraw_sol<'info>(
    bump: u8,
    depositor_key: &Pubkey,
    system_program: &Program<'info, System>,
    from_account: &SystemAccount<'info>,
    to_account: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SOL_ACCOUNT_SEED, depositor_key.as_ref(),&[bump]]];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: from_account.to_account_info(),
                to: to_account.to_account_info(),
            },
            signer_seeds,
        )
    ,
    amount,
    )
}

pub fn burn_tokens<'info>(
    token_program: &Program<'info, Token2022>,
    mint_account: &InterfaceAccount<'info, Mint>,
    from_account: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint_account.to_account_info(),
                from: from_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        )
    , amount)
}
