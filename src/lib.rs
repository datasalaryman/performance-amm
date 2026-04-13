#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

declare_id!("AaURiCefSvaygMUc8tc2yTyBpBYakWJeB7uGxEMWWfm2");

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub payer: &'info mut Signer,
    pub system_program: &'info Program<System>,
}

impl<'info> Initialize<'info> {
    #[inline(always)]
    pub fn initialize(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}

#[program]
mod performance_amm {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>, pair: String<32>) -> Result<(), ProgramError> {
        log(pair);
        ctx.accounts.initialize()
    }
}

#[cfg(test)]
mod tests;
