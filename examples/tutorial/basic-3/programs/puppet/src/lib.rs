#![feature(proc_macro_hygiene)]

use anchor_lang::prelude::*;

#[program]
mod puppet {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> ProgramResult {
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub puppet: ProgramAccount<'info, Puppet>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: ProgramAccount<'info, Puppet>,
}

#[account]
pub struct Puppet {
    pub data: u64,
}
