use anchor_lang::prelude::*;

declare_id!("2euLYQyMsH2WYZ2DgVXfAUZMRmpG179v69M5RcBCuCGf");

#[program]
pub mod ancient {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
