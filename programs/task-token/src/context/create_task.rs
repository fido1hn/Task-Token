use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};
use dotenv::dotenv;
use solana_program::pubkey::Pubkey as ProgramPubkey;
use std::{env, str::FromStr};

use crate::{errors::CustomError, state::{Config, Task}};

#[derive(Accounts)]
#[instruction(title: String)]
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
      seeds = [b"task", title.as_bytes(), owner.key().as_ref()],
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
        difficulty: u8,
        bumps: CreateTaskBumps,
    ) -> Result<()> {
        // Validate difficulty
        require!(difficulty <= 2, CustomError::InvalidDifficulty);
        // Check payment >= $20
        require_gte!(pay, 20);

        // Transfer to the config vault
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer { from: self.owner.to_account_info(), to: self.fee_vault.to_account_info() };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let listing_fee = 30_000_000;
        transfer(cpi_ctx, listing_fee)?;

        // Transfer to the task vault
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked { from: self.owner_pay_mint_ata.to_account_info(), mint: self.pay_mint.to_account_info(), to: self.task_vault.to_account_info(), authority: self.owner.to_account_info() };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, pay, 6)?;

        self.task.set_inner(Task {
            title,
            description,
            deadline,
            pay,
            difficulty,
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