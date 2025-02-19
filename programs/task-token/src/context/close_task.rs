use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseTask<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
