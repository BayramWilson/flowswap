use anchor_lang::prelude::*;

declare_id!("EyfmRhE4D7vEEQGkqW1hAstWC185ZUWr3ERGPv4g37Q2");

#[program]
pub mod flowswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
