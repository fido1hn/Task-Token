use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    admin: Pubkey,
    fee: u16, // basis point
    config_bump: u8,
    mint_bump: u8,
    vault_bump: u8,
}
