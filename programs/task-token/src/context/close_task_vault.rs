use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, CloseAccount, TokenAccount, TokenInterface};

use crate::state::{Config, TaskVaultInfo};

#[derive(Accounts)]
pub struct CloseTaskVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      seeds = [b"config", config.admin.key().as_ref()],
      bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,
    // task vault
    #[account(
      mut,
      seeds = [b"task_vault", task_vault_info.task.to_bytes().as_ref()],
      bump = task_vault_info.task_vault_bump,
    )]
    pub task_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      mut,
      seeds = [b"task_vault_info", task_vault_info.task.to_bytes().as_ref()],
      bump = task_vault_info.task_vault_info_bump,
      close = signer,
    )]
    pub task_vault_info: Box<Account<'info, TaskVaultInfo>>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CloseTaskVault<'info> {
    pub fn close_task_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.task_vault.to_account_info(),
            destination: self.signer.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let binding = self.config.admin.key();
        let seeds = &[b"config", binding.as_ref()];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
