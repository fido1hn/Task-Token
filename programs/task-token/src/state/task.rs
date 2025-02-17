use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Task {
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub description: String,
    pub deadline: u64,
    #[max_len(100)]
    pub submissions: Vec<Pubkey>,
    pub owner: Pubkey,
}
