#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("escSmDEY26evSYow7Nio1WtkNFneo95DTq83P4myqer");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<MakeAccounts>, input: MakeInput) -> Result<()> {
        _make(ctx, input)
    }

    pub fn take(ctx: Context<TakeAccounts>) -> Result<()> {
        _take(ctx)
    }

    pub fn refund(ctx: Context<RefundAccounts>) -> Result<()> {
        _refund(ctx)
    }
}
