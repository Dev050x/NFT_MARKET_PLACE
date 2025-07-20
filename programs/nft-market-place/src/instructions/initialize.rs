use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{Mint,TokenInterface}};

use crate::state::Market_Place;
use crate::error::ErrorCode;


#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        seeds = [b"marketplace".as_ref() , name.as_bytes()],
        bump,
        space = Market_Place::INIT_SPACE,
    )]
    pub market_place: Account<'info, Market_Place>,

    #[account(
        seeds = [b"treasury".as_ref() , market_place.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards" , market_place.key().as_ref()],
        bump,
        mint::authority = market_place,
        mint::decimals = 6,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_market_place(&mut self, name: String , fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(!name.is_empty() && name.len() <= 4+32 , ErrorCode::NamingError);

        self.market_place.set_inner(Market_Place {
            admin: self.admin.key(),
            fee,
            name,
            bump: bumps.market_place,
            treasury_bump: bumps.treasury,
            reward_bump: bumps.reward_mint,
        });

        Ok(())
    }
}
