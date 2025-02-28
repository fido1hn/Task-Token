use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{
    events::TaskCompleted,
    state::{Config, Submission, Task},
};

#[derive(Accounts)]
pub struct PayDeveloper<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub developer: SystemAccount<'info>,
    #[account(
      seeds = [b"config", config.admin.key().as_ref()],
      bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
      seeds = [b"task", task.title.as_bytes(), task.owner.key().as_ref()],
      bump = task.task_bump,
    )]
    pub task: Box<Account<'info, Task>>,
    #[account(
      seeds = [b"submission", submission.developer.as_ref(), submission.task.as_ref()],
      bump = submission.bump,
    )]
    pub submission: Box<Account<'info, Submission>>,
    // task vault
    #[account(
      mut,
      associated_token::mint = payment_mint,
      associated_token::authority = task,
    )]
    pub task_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    // developer payment ata
    #[account(
      init_if_needed,
      payer = signer,
      associated_token::mint = payment_mint,
      associated_token::authority = developer
    )]
    pub developer_payment_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    // developer task token ata
    #[account(
      init_if_needed,
      payer = signer,
      associated_token::mint = task_token_mint,
      associated_token::authority = developer
    )]
    pub developer_task_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    // task token mint
    #[account(
      seeds = [b"task_token", config.key().as_ref()],
      bump = config.mint_bump
    )]
    pub task_token_mint: Box<InterfaceAccount<'info, Mint>>,
    // payment mint
    #[account(address = config.payment_mint)]
    pub payment_mint: Box<InterfaceAccount<'info, Mint>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Should only pay developer & mint task tokens
impl<'info> PayDeveloper<'info> {
    pub fn pay_developer(&mut self) -> Result<()> {
        // check project owner is signer
        require_eq!(self.signer.key(), self.task.owner.key());

        self.transfer_payment()?;
        self.mint_task_tokens()?;

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

        Ok(())
    }

    fn transfer_payment(&mut self) -> Result<()> {
        // send payment from vault to developer
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.task_vault.to_account_info(),
            mint: self.payment_mint.to_account_info(),
            to: self.developer_payment_ata.to_account_info(),
            authority: self.task.to_account_info(),
        };

        let seeds = &[
            b"task",
            self.task.title.as_bytes(),
            self.task.owner.as_ref(),
            &[self.task.task_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.task_vault.amount, 6)?;

        Ok(())
    }

    fn mint_task_tokens(&mut self) -> Result<()> {
        // Mint task tokens to developer
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.task_token_mint.to_account_info(),
            to: self.developer_task_token_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[
            b"config",
            self.config.admin.as_ref(),
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let amount = match self.task.difficulty {
            0 => 1_000_000,
            1 => 2_000_000,
            2 => 3_000_000,
            _ => 0, // handle other values
        };

        mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}
