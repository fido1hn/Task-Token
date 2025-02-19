use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Task {
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub description: String,
    pub deadline: i64,
    #[max_len(100)]
    pub submissions: Vec<Pubkey>,
    pub pay: u64,
    pub owner: Pubkey,
    pub task_vault_bump: u8,
    pub task_bump: u8,
}
