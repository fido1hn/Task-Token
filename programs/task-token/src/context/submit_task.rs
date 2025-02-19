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
    pub fn submit_task(&mut self, link: String, bumps: SubmitTaskBumps) -> Result<()> {
        // Create a submission account
        self.submission.set_inner(Submission {
            task: self.task.key(),
            developer: self.developer.key(),
            submission_link: link,
            bump: bumps.submission,
        });
        // Add submission account pubkey to the task submission vector field
        self.task
            .submissions
            .extend_from_slice(&[self.submission.key()]);

        Ok(())
    }
}
