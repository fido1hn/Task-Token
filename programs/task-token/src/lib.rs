use anchor_lang::prelude::*;

declare_id!("6irtasT64kUUv3558PXTcg3BUWLgWXjx2efQJXMEz2UE");

#[program]
pub mod task_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
