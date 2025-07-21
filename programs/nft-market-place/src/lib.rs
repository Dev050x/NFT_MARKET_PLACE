#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4BN1fvJTHgSWpvpMEFRK73RmLSrfd9fyQHiVT3PfBpq7");

#[program]
pub mod nft_market_place {
    use super::*;

    pub fn initialize(ctx: Context<Initialize> , name:String) -> Result<()> {
        ctx.accounts.initialize_market_place(name, 250 , &ctx.bumps)?; 
        Ok(())
    }
    pub fn list_nft(ctx: Context<List> , price:u64) -> Result<()>{
        ctx.accounts.list(price , ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }
}
