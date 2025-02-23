use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    errors::CustomError,
    state::{Config, Task},
};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
      seeds = [b"config", config.admin.as_ref()],
      bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
      init,
      payer = owner,
      space = 8 + Task::INIT_SPACE,
      seeds = [b"task", title.as_bytes(), owner.key().as_ref()],
      bump
    )]
    pub task: Box<Account<'info, Task>>,
    #[account(
      mut,
      seeds = [b"config", config.key().as_ref()],
      bump = config.vault_bump
    )]
    pub config_vault: SystemAccount<'info>,
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
        // Check difficulty
        require!(difficulty <= 2, CustomError::InvalidDifficulty);
        // Check payment >= $20
        require_gte!(pay, 20);

        // Collect listing fee
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.owner.to_account_info(),
            to: self.config_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let listing_fee = 30_000_000;
        transfer(cpi_ctx, listing_fee)?;

        self.task.set_inner(Task {
            title,
            description,
            deadline,
            pay,
            difficulty,
            owner: self.owner.key(),
            task_vault_bump: 0u8,
            task_bump: bumps.task,
        });
        Ok(())
    }
}
