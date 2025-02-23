use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::{Config, Submission};

#[derive(Accounts)]
pub struct CloseSubmission<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      seeds = [b"config", config.admin.key().as_ref()],
      bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    #[account(
      mut,
      seeds = [b"submission", signer.key().as_ref(), submission.task.key().as_ref()],
      bump = submission.bump,
      close = signer
    )]
    pub submission: Account<'info, Submission>,
    #[account(
      init_if_needed,
      payer = signer,
      associated_token::mint = task_token_mint,
      associated_token::authority = signer, // Developer's ATA for task tokens
    )]
    pub developer_task_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      seeds = [b"task_token", config.key().as_ref()],
      bump = config.mint_bump,
    )]
    pub task_token_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseSubmission<'info> {
    pub fn close_submission(&mut self) -> Result<()> {
        // Mint task tokens to the developer as an incentive
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

        let amount = 500_000; // Small incentive

        mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}
