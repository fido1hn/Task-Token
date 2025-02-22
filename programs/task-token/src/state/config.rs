use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub payment_mint: Pubkey,
    pub fee: u16, // basis point
    pub config_bump: u8,
    pub mint_bump: u8,
    pub vault_bump: u8,
}
