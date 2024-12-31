#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
// use anchor_spl::token::Mint;
use crate::asset::*;

#[derive(Accounts)]
pub struct UnwrapContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    

    #[account(
        mut,
        close = owner
    )]
    pub asset: Account<'info, AssetInfo>
}

impl<'info> UnwrapContext<'info> {
    pub fn unwrap(&mut self) -> Result<()> {
        Ok(())
    }
}
