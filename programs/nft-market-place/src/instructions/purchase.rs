use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{ state::Listing, Market_Place};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        init_if_needed,
        payer= taker,
        associated_token::authority = taker,
        associated_token::mint = maker_mint
    )]
    pub taker_mint_ata: InterfaceAccount<'info, TokenAccount>,
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
    #[account(
        seeds = [b"treasury".as_ref() , market_place.key().as_ref()],
        bump = market_place.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        let market_place_fee = (self.market_place.fee as u64)
            .checked_mul(self.listing.price)
            .unwrap()
            .checked_div(10000_u64)
            .unwrap();

        let cpiContext1 = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.taker.to_account_info(),
                to: self.maker.to_account_info(),
            },
        );
        transfer(
            cpiContext1,
            self.listing.price.checked_sub(market_place_fee).unwrap(),
        )?;

        let cpiContext2 = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.taker.to_account_info(),
                to: self.treasury.to_account_info(),
            },
        );
        transfer(cpiContext2, market_place_fee)?;
        Ok(())
    }


    pub fn send_nft(&mut self) -> Result<()>{

        let seeds = &[
            &self.market_place.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpiContext = CpiContext::new_with_signer(self.token_program.to_account_info(), TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.maker_mint.to_account_info(),
            to:self.taker_mint_ata.to_account_info(),
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
