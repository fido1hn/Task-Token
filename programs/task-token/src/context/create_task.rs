use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
// use solana_program::pubkey::Pubkey;
// use std::str::FromStr;

use crate::state::{Config, Task};

#[derive(Accounts)]
#[instruction(task_id: u16)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
      seeds = [b"config", config.admin.as_ref()],
      bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    pub pay_mint: InterfaceAccount<'info, Mint>,
    #[account(
      init,
      payer = owner,
      space = 8 + Task::INIT_SPACE,
      seeds = [b"task", task_id.to_le_bytes().as_ref(), owner.key().as_ref()],
      bump
    )]
    pub task: Account<'info, Task>,
    #[account(
      init,
      payer = owner,
      seeds = [b"task_vault", task.key().as_ref()],
      bump,
      token::mint = pay_mint,
      token::authority = config,
    )]
    pub task_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      seeds = [b"config", config.key().as_ref()],
      bump = config.vault_bump
    )]
    pub fee_vault: SystemAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
