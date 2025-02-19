use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use dotenv::dotenv;
use solana_program::pubkey::Pubkey as ProgramPubkey;
use std::{env, str::FromStr};

use crate::state::{Config, Submission, Task};

#[derive(Accounts)]
pub struct CloseTask<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      seeds = [b"config", config.admin.key().as_ref()],
      bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    #[account(
      seeds = [b"task", task.title.as_bytes(), task.owner.key().as_ref()],
      bump
    )]
    pub task: Account<'info, Task>,
    #[account(
      seeds = [b"submission", submission.developer.as_ref(), task.key().as_ref()],
      bump
    )]
    pub submission: Account<'info, Submission>,
    // developer pay ata
    pub developer_pay_ata: InterfaceAccount<'info, TokenAccount>,
    // developer rewards ata
    pub developer_task_token_ata: InterfaceAccount<'info, TokenAccount>,
    // rewards mint
    #[account(
      seeds = [b"rewards", config.key().as_ref()],
      bump = config.mint_bump
    )]
    pub task_token_mint: InterfaceAccount<'info, Mint>,
    // payment mint
    #[account(address = get_mint_address())]
    pub pay_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

fn get_mint_address() -> ProgramPubkey {
    dotenv().ok();
    let mint_address = env::var("PAY_MINT_ADDRESS").expect("PAY_MINT_ADDRESS must be set");
    ProgramPubkey::from_str(&mint_address).expect("Invalid MINT_ADDRESS")
}
