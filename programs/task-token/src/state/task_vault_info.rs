use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TaskVaultInfo {
    pub task: Pubkey,
    pub task_vault_bump: u8,
    pub task_vault_info_bump: u8,
}
