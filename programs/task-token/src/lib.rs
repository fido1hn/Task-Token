use anchor_lang::prelude::*;

pub mod context;
pub use context::*;

pub mod state;
pub use state::*;

declare_id!("6irtasT64kUUv3558PXTcg3BUWLgWXjx2efQJXMEz2UE");

#[program]
pub mod task_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        ctx.accounts.initialize(fee, ctx.bumps)
    }

    pub fn create_task(
        ctx: Context<CreateTask>,
        title: String,
        description: String,
        pay: u64,
        deadline: i64,
    ) -> Result<()> {
        ctx.accounts
            .create_task(title, description, pay, deadline, ctx.bumps)
    }
}
