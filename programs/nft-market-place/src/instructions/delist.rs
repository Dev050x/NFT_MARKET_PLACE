use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface ,  TransferChecked , transfer_checked , CloseAccount , close_account},
};

use crate::{state::Listing, Market_Place};


#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint
    )]
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        close=maker,
        has_one = maker,
        seeds = [market_place.key().as_ref(), maker_mint.key().as_ref()],
        bump=listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        seeds = [market_place.key().as_ref() , market_place.name.as_bytes()],
        bump = market_place.bump,
    )]
    pub market_place: Account<'info, Market_Place>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> Delist<'info>{
    
    pub fn withdraw_nft(&mut self ) -> Result<()>{

        let seeds = &[
            &self.market_place.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpiContext = CpiContext::new_with_signer(self.token_program.to_account_info(), TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.maker_mint.to_account_info(),
            to:self.maker_mint_ata.to_account_info(),
            authority:self.listing.to_account_info(),
        }, signer_seeds);

        transfer_checked(cpiContext, 1, 6)?;

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()>{

        let seeds = &[
            &self.market_place.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpiContext = CpiContext::new_with_signer(self.token_program.to_account_info(), CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.listing.to_account_info(),
        }, signer_seeds);

        close_account(cpiContext)?;

        Ok(())
    }

}