use anchor_lang::prelude::*;

#[account]
pub struct Task {
    pub title: String,
    pub description: String,
    pub deadline: u16,
    pub submissions: Vec<Pubkey>,
    pub owner: Pubkey,
}
