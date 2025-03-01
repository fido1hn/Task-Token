use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, CloseAccount, TokenAccount, TokenInterface};

use crate::state::Task;

#[derive(Accounts)]
pub struct CloseTaskAccountVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      mut,
      seeds = [b"task", task.title.as_bytes(), task.owner.key().as_ref()],
      bump = task.task_bump,
      close = signer
    )]
    pub task: Box<Account<'info, Task>>,
    // task vault
    #[account(mut)]
    pub task_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CloseTaskAccountVault<'info> {
    pub fn close_task_vault(&mut self) -> Result<()> {
        require_eq!(self.signer.key(), self.task.owner);

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.task_vault.to_account_info(),
            destination: self.signer.to_account_info(),
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

        close_account(cpi_ctx)?;

        Ok(())
    }
}
