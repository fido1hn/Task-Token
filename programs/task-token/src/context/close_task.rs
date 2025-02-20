use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};
use dotenv::dotenv;
use solana_program::pubkey::Pubkey as ProgramPubkey;
use std::{env, str::FromStr};

use crate::state::{Config, Submission, Task};

#[derive(Accounts)]
pub struct CloseTask<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub developer: SystemAccount<'info>,
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
    // task vault
    #[account(
      seeds = [b"task_vault", task.key().as_ref()],
      bump = task.task_bump
    )]
    pub task_vault: InterfaceAccount<'info, TokenAccount>,
    // developer pay ata
    #[account(
      init_if_needed,
      payer = signer,
      associated_token::mint = pay_mint,
      associated_token::authority = developer
    )]
    pub developer_pay_ata: InterfaceAccount<'info, TokenAccount>,
    // developer rewards ata
    #[account(
      init_if_needed,
      payer = signer,
      associated_token::mint = task_token_mint,
      associated_token::authority = developer
    )]
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

impl<'info> CloseTask<'info> {
    pub fn close_task(&mut self) -> Result<()> {
        // check project owner is signer
        require_eq!(self.signer.key(), self.task.owner.key());

        // send fee from vault to developer
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.task_vault.to_account_info(),
            mint: self.pay_mint.to_account_info(),
            to: self.developer_pay_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let binding = self.config.admin.key();
        let seeds = &[b"config", binding.as_ref()];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.task_vault.amount, 6)?;

        // Mint task tokens to developer
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.task_token_mint.to_account_info(),
            to: self.developer_task_token_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let binding = self.config.admin.key();
        let seeds = &[b"config", binding.as_ref()];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount);

        // emit the task completed event
        emit!(TaskCompleted {
            task: self.task.key(),
            description: self.task.description.to_string(),
            submission: self.submission.submission_link.to_string(),
            difficulty: self.task.difficulty,
            developer: self.developer.key(),
            task_owner: self.task.owner.key(),
            closed_at: Clock::get()?.unix_timestamp
        });

        // close all submission accounts
        Ok(())
    }
}

fn get_mint_address() -> ProgramPubkey {
    dotenv().ok();
    let mint_address = env::var("PAY_MINT_ADDRESS").expect("PAY_MINT_ADDRESS must be set");
    ProgramPubkey::from_str(&mint_address).expect("Invalid MINT_ADDRESS")
}

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
