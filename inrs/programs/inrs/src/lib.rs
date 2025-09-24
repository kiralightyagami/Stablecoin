use anchor_lang::prelude::*;
use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;


declare_id!("");

#[program]
pub mod inrs {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        init_config(ctx)
    }
}


