use anchor_lang::prelude::*;

declare_id!("EYQ9vZ45TLe2grjwm3nF1znSC3fVgi19JdGCZBKFTXpz");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, mint_a_amount: u64, mint_b_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, mint_a_amount, mint_b_amount, &ctx.bumps)?;
        ctx.accounts.deposit()?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }
}