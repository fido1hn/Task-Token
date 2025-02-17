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
}
