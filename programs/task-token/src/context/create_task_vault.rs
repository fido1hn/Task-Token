use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::state::{Config, Task, TaskVaultInfo};

#[derive(Accounts)]
pub struct CreateTaskVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      seeds = [b"task", task.title.as_bytes(), task.owner.key().as_ref()],
      bump = task.task_bump
    )]
    pub task: Box<Account<'info, Task>>,
    #[account(
      seeds = [b"config", config.admin.as_ref()],
      bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(address = config.payment_mint)]
    pub payment_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub signer_payment_mint_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      init,
      payer = signer,
      seeds = [b"task_vault", task.key().as_ref()],
      bump,
      token::mint = payment_mint,
      token::authority = config,
    )]
    pub task_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      init,
      payer = signer,
      space = 8 + TaskVaultInfo::INIT_SPACE,
      seeds = [b"task_vault_info", task.key().as_ref()],
      bump
    )]
    pub task_vault_info: Account<'info, TaskVaultInfo>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTaskVault<'info> {
    pub fn create_task_vault(&mut self, bump: CreateTaskVaultBumps) -> Result<()> {
        // Save bump; Needs task account to get Task Vault bump
        self.task_vault_info.set_inner(TaskVaultInfo {
            task: self.task.key(),
            task_vault_bump: bump.task_vault,
            task_vault_info_bump: bump.task_vault_info,
        });

        // Transfer to the task vault
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.signer_payment_mint_ata.to_account_info(),
            mint: self.payment_mint.to_account_info(),
            to: self.task_vault.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, self.task.pay, 6)?;
        Ok(())
    }
}
