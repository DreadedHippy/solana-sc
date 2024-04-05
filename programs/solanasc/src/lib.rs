use anchor_lang::prelude::*;

declare_id!("EWUPVRLV3vutGzRFR37Aj79rTARCGvmBYhja9KjNMJ1s");

#[program]
pub mod solanasc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64) -> Result<()> {
        msg!("You sent {}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
