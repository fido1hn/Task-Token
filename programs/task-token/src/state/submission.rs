use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Submission {
    pub task: Pubkey,
    pub developer: Pubkey,
    #[max_len(50)]
    pub submission_link: String,
    pub submitted_at: i64,
    pub bump: u8,
}
