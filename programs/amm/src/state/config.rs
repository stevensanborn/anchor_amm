use anchor_lang::prelude::*;

//main struct of the LP pool 
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed:u64,
    pub authority:Option<Pubkey>, //not locked into the liquidity pool , you can set the authority to someone else
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub fee:u16,
    pub locked:bool,//safety that the authority can lock the pool
    pub config_bump: u8, //multiple
    pub lp_bump:u8 //liquifity pool bump
}