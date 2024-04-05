use anchor_lang::prelude::*;

declare_id!("BP7EaKhfYcioQB6oVQDcSn5fbDTne5AoHYnk6gpnMyxU");

#[program]
pub mod solanasc {
    use anchor_lang::solana_program::vote::error;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, message: String) -> Result<()> {
        msg!("You sent {}", a);
        msg!("You said {}", message);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn limit_range(ctx: Context<Nextalize>, a: u64) -> Result<()> {
        if a > 100 {
            return err!(MyError::AisTooBig);
        }

        if a < 10 {
            return err!(MyError::AisTooSmall);
        }

        msg!("Result = {}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Nextalize {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
}
