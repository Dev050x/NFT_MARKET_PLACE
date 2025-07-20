use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{Mint, TokenAccount, TokenInterface ,  TransferChecked , transfer_checked},
};

use crate::{state::Listing, Market_Place};

#[derive(Accounts)]
pub struct List<'info> {
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
        init,
        payer = maker,
        seeds = [b"marketplace".as_ref() , maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        seeds = [b"marketplace".as_ref() , market_place.name.as_bytes()],
        bump = market_place.bump,
    )]
    pub market_place: Account<'info, Market_Place>,
    #[account(
        init,
        payer = maker,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,

    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> List<'info> {
    pub fn list(&mut self, price: u64, bumps: ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            mint: self.maker_mint.key(),
            bump: bumps.listing,
            price,
        });
        Ok(())
    }
    pub fn deposit_nft(&mut self) -> Result<()> {

        let cpiContext = CpiContext::new(self.token_program.to_account_info(), TransferChecked{
            from:self.maker_mint_ata.to_account_info(),
            to:self.vault.to_account_info(),
            mint:self.maker_mint.to_account_info(),
            authority:self.maker.to_account_info(),
        });

        transfer_checked(cpiContext, 1, 6)?;

        Ok(())
    }
}
