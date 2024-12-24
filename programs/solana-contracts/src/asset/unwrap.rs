#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
// use anchor_spl::token::Mint;
use crate::asset::*;

#[derive(Accounts)]
pub struct UnwrapContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub asset_info: Account<'info, AssetInfo>
}

impl<'info> UnwrapContext<'info> {
    pub fn unwrap(&mut self) -> Result<()> {
        self.asset_info.close(self.user.to_account_info())
    }
}
