use anchor_lang::prelude::*;

declare_id!("iBnL6NYFBi3zy91EYCNqmeELN4XEaekLchFUFZEJvdP");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
