use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
      init,
      payer = admin,
      space = 8 + Config::INIT_SPACE,
      seeds = [b"config", admin.key().as_ref()],
      bump
    )]
    pub config: Account<'info, Config>,
    #[account(
      seeds = [b"config", config.key().as_ref()],
      bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
      init,
      payer = admin,
      seeds = [b"task_token", config.key().as_ref()],
      bump,
      mint::decimals = 6,
      mint::authority = config,
      mint::freeze_authority = config,
    )]
    pub task_token_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: This is the payment mint. We'll store its address in the config.
    pub payment_mint: AccountInfo<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, fee: u16, bumps: InitializeBumps) -> Result<()> {
        self.config.set_inner(Config {
            admin: self.admin.key(),
            payment_mint: self.payment_mint.key(),
            fee,
            config_bump: bumps.config,
            mint_bump: bumps.task_token_mint,
            vault_bump: bumps.vault,
        });
        Ok(())
    }
}
