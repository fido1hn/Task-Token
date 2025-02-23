use anchor_lang::prelude::*;

use crate::{state::Task, Submission};

#[derive(Accounts)]
pub struct SubmitTask<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      seeds = [b"task", task.title.as_bytes(), task.owner.key().as_ref()],
      bump
    )]
    pub task: Account<'info, Task>,
    #[account(
      init,
      payer = signer,
      space = 8 + Submission::INIT_SPACE,
      seeds = [b"submission", signer.key().as_ref(), task.key().as_ref()],
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
            developer: self.signer.key(),
            submission_link: link,
            bump: bumps.submission,
        });

        Ok(())
    }
}
