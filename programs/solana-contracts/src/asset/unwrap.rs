#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
// use anchor_spl::token::Mint;
use crate::asset::*;

#[derive(Accounts)]
pub struct UnwrapContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"asset_manager", authority.key().as_ref()],
        bump,
    )]
    pub asset_manager: Account<'info, AssetManager>,


    #[account(
        mut,
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump,
        close = owner,
    )]
    pub asset: Account<'info, AssetInfo>
}

impl<'info> UnwrapContext<'info> {
    pub fn unwrap(&mut self, supply_no:u64) -> Result<()> {
        assert_eq!(self.owner.key(), self.asset.owner);
        assert_eq!(supply_no, self.asset.supply_no);
        Ok(())
    }
}
