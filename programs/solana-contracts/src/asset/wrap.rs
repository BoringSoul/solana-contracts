#![allow(clippy::result_large_err)]

use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::Metadata,
    token::{Mint, Token, TokenAccount}
};
use solana_program::{program::invoke, system_instruction};
// use anchor_spl::token::Mint;
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

    // // Create new mint account, NFTs have 0 decimals
    // #[account(
    //     init,
    //     payer = owner,
    //     mint::decimals = 0,
    //     mint::authority = authority.key(),
    //     mint::freeze_authority = authority.key(),
    // )]
    // pub mint_account: Account<'info, Mint>,

    // // Create associated token account, if needed
    // // This is the account that will hold the NFT
    // #[account(
    //     init_if_needed,
    //     payer = owner,
    //     associated_token::mint = mint_account,
    //     associated_token::authority = authority,
    // )]
    // pub associated_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info,System>,
    // pub token_program: Program<'info, Token>,
    // pub token_metadata_program: Program<'info, Metadata>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn wrap(ctx: Context<WrapContext>, 
    assets: Vec<Asset>) -> Result<AssetInfo> {
    let clock = Clock::get()?;
    let data = AssetInfo {
        owner: ctx.accounts.owner.key(),
        supply_no: ctx.accounts.asset_manager.current_supply,
        assets,
        start_time: clock.unix_timestamp,
        mint_account: Pubkey::default(),
        token_account:Pubkey::default(),
    };
    ctx.accounts.asset.set_inner(data.clone());
    ctx.accounts.asset_manager.current_supply += 1;
    // transfer(ctx.accounts.owner.to_account_info(), ctx.accounts.authority.to_account_info(),  1 * 10_000_000_000)?;
    msg!("currentSupply:{:?}", ctx.accounts.asset_manager.current_supply);
    msg!("assetInfo:{:?}, assetKey:{:?}", data, ctx.accounts.asset.key());
    Ok(data)
}

pub fn transfer<'info>(sender:AccountInfo<'info>, receiver:AccountInfo<'info>,  amount:u64) ->Result<()> {
    invoke(&system_instruction::transfer(sender.key, receiver.key, amount), &[sender.clone(), receiver.clone()])?;
    Ok(())
}
