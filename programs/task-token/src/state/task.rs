use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Task {
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub description: String,
    pub difficulty: u8,
    pub deadline: i64,
    pub pay: u64,
    pub owner: Pubkey,
    pub task_bump: u8,
}
