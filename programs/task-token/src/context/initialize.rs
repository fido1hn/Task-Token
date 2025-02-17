use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(fee:u16)]
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
      mut,
      seeds = [b"config", config.key().as_ref()],
      bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
      init,
      payer = admin,
      mint::decimals = 6,
      mint::authority = config,
      mint::freeze_authority = config,
      seeds = [b"rewards", config.key().as_ref()],
      bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
