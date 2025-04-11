#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
use instructions::*;

declare_id!("28rk9GftFsUSZNemyhpXPKStimdidUP8mMgTgJDGFT3p");

#[program]
pub mod amm {
    use super::*;

    pub fn initializeme(ctx: Context<Initialization>,
    seed:u64, fee:u16, authority:Option<Pubkey>) -> Result<()> {
        
        msg!("Greetings from: {:?}", ctx.program_id);
        
        ctx.accounts.init(seed,fee,authority,ctx.bumps)
        
    }

    pub fn deposit(ctx:Context<Deposit>, 
        amount:u64,
        max_x:u64,
        max_y:u64,
         )->Result<()>
         {

        ctx.accounts.deposit(amount,max_x, max_y)

    }
}

