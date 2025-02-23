use anchor_lang::prelude::*;

#[event]
pub struct TaskCompleted {
    pub task: Pubkey,
    pub description: String,
    pub submission: String,
    pub difficulty: u8,
    pub developer: Pubkey,
    pub task_owner: Pubkey,
    pub closed_at: i64,
}
