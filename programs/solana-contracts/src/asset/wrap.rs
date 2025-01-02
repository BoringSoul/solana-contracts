#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use crate::asset::*;

#[derive(Accounts)]
pub struct WrapContext<'info> {
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
        init,
        payer = owner,
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &asset_manager.current_supply.to_le_bytes()],
        bump,
        space = 8 + AssetInfo::INIT_SPACE
    )]
    pub asset: Account<'info, AssetInfo>,

    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn wrap(ctx: Context<WrapContext>, 
    assets: Vec<Asset>) -> Result<AssetInfo> {
    let clock = Clock::get()?;
    let data = AssetInfo {
        owner: ctx.accounts.owner.key(),
        supply_no: ctx.accounts.asset_manager.current_supply + 1,
        assets,
        start_time: clock.unix_timestamp,
        mint_account: Pubkey::default(),
        token_account:Pubkey::default(),
    };
    ctx.accounts.asset.set_inner(data.clone());
    ctx.accounts.asset_manager.current_supply += 1;
    // transfer(ctx.accounts.owner.to_account_info(), ctx.accounts.authority.to_account_info(),  1 * 10_000_000_000)?;
    msg!("assetInfo:{:?}, assetKey:{:?}", data, ctx.accounts.asset.key());
    Ok(data)
}

// pub fn transfer<'info>(sender:AccountInfo<'info>, receiver:AccountInfo<'info>,  amount:u64) ->Result<()> {
//     invoke(&system_instruction::transfer(sender.key, receiver.key, amount), &[sender.clone(), receiver.clone()])?;
//     Ok(())
// }
