use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};
use dotenv::dotenv;
use solana_program::pubkey::Pubkey as ProgramPubkey;
use std::{env, str::FromStr};

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
    #[account(
      address = get_mint_address()
    )]
    pub pay_mint: InterfaceAccount<'info, Mint>,
    #[account(
      init_if_needed,
      payer = owner, 
      associated_token::mint = pay_mint,
      associated_token::authority = owner,
    )]
    pub owner_pay_mint_ata: InterfaceAccount<'info, TokenAccount>,
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
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTask<'info> {
    pub fn create_task(
        &mut self,
        title: String,
        description: String,
        pay: u64,
        deadline: i64,
        bumps: CreateTaskBumps,
    ) -> Result<()> {
        // Check payment >= $20
        require_gte!(pay, 20);
        // Transfer to the config vault
        // Transfer to the task vault
        self.task.set_inner(Task {
            title,
            description,
            deadline,
            pay,
            submissions: Vec::new(),
            owner: self.owner.key(),
            task_vault_bump: bumps.task_vault,
            task_bump: bumps.task,
        });
        Ok(())
    }
}

fn get_mint_address() -> ProgramPubkey {
  dotenv().ok();
  let mint_address = env::var("PAY_MINT_ADDRESS").expect("PAY_MINT_ADDRESS must be set");
  ProgramPubkey::from_str(&mint_address).expect("Invalid MINT_ADDRESS")
}