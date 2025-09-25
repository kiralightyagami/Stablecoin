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
        
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        update_config(ctx, min_health_factor)
    }
}


