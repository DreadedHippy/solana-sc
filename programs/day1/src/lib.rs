use anchor_lang::prelude::*;

declare_id!("6m3e17LB3YhXvzax97TNxGwUL6nRbFRKjGYgXWxXLPcD");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
