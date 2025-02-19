use anchor_lang::prelude::*;

use crate::{state::Task, Submission};

#[derive(Accounts)]
#[instruction(task_id: u16)]
pub struct SubmitTask<'info> {
    #[account(mut)]
    pub developer: Signer<'info>,
    #[account(
      seeds = [b"task", task_id.to_le_bytes().as_ref(), task.owner.key().as_ref()],
      bump
    )]
    pub task: Account<'info, Task>,
    #[account(
      init,
      payer = developer,
      space = 8 + Submission::INIT_SPACE,
      seeds = [b"submission", developer.key().as_ref(), task.key().as_ref()],
      bump
    )]
    pub submission: Account<'info, Submission>,
    pub system_program: Program<'info, System>,
}

impl<'info> SubmitTask<'info> {
    pub fn submit_task(&mut self) -> Result<()> {
        // Create a submission account
        // Add submission account pubkey to the task submission field vector

        Ok(())
    }
}
