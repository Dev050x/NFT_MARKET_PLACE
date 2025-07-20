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

declare_id!("2Y4w9BbqBDL1du4AkXHY14u4zX56HGbtnYMd6kS5LErL");

#[program]
pub mod nft_market_place {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        Ok(())
    }
}
