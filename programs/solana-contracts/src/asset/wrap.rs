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
        space = AssetInfo::INIT_SPACE,
        seeds = [
            b"ASSET_INFO",
            user.key().as_ref(),
            //这里不知道怎么用assets里面的东东作为Seed
        ],
        bump
    )]
    pub asset_info_accout: Account<'info, AssetInfo>,
    pub system_program: Program<'info, System>,
}

pub fn wrap(ctx: Context<WrapContext>, 
    supply_no: u64,
    assets: Vec<Asset>) -> Result<()> {
    *ctx.accounts.asset_info_accout = AssetInfo {
        bump: ctx.bumps.asset_info_accout,
        user: ctx.accounts.user.key(),
        supply_no,
        assets
    };
    Ok(())
}
