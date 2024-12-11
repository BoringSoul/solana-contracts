#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
// use anchor_spl::token::Mint;
use crate::asset::*;

#[derive(Accounts)]
pub struct WrapContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + AssetInfo::INIT_SPACE,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    pub system_program: Program<'info, System>,
}

pub fn wrap(ctx: Context<WrapContext>, 
    supply_no: u64,
    assets: Vec<Asset>) -> Result<()> {
    *ctx.accounts.asset_info = AssetInfo {
        user: ctx.accounts.user.key(),
        supply_no,
        assets
    };
    Ok(())
}
