use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market_Place {
    pub admin: Pubkey,
    pub fee: u16,
    #[max_len(32)]
    pub name: String,
    pub bump: u8,
    pub treasury_bump: u8,
    pub reward_bump: u8,
}
