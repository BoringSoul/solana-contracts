
#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
pub mod asset;
pub mod nft;
pub mod errors;

use nft::*;
use asset::*;

declare_id!("J7NfYmxgsvEQKC2kZhLpizyLQ7ucXC6KHW3dtpf7rtPF");
#[program]
pub mod solana_contracts {

    use super::*;
    pub fn mint_nft(
        ctx: Context<MintNft>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
        // supply_no:u64,
        // assets: Vec<asset::Asset>
    ) -> Result<()> {
        ctx.accounts.mint(token_title, token_symbol, token_uri)
    }


    pub fn init_asset_manager(ctx: Context<InitContext>, limit:u64, contract_uri:String, start_time:i64, end_time:i64, wrap_fee:u64, unwrap_fee:u64) -> Result<()> {
        ctx.accounts.init(limit, contract_uri, start_time, end_time, wrap_fee, unwrap_fee)
    }


    pub fn update_supply_no(ctx: Context<UpdateSupplyNoContext>) -> Result<AssetManager> {
        ctx.accounts.update_supply_no()
    }

    pub fn burn_nft(
        ctx: Context<BurnNftContext>,
        supply_no:u64
    ) -> Result<()> {
        ctx.accounts.burn_nft(supply_no)
    }

    pub fn wrap_asset(
        ctx:Context<WrapContext>,
        assets: Vec<asset::Asset>
    ) -> Result<AssetInfo>{
        asset::wrap(ctx, assets)
    }

    pub fn unwrap_asset(
        ctx:Context<UnwrapContext>,
        supply_no:u64
    ) -> Result<()>{
        ctx.accounts.unwrap(supply_no)
    }

    pub fn stake(ctx:Context<StakeContext>, stake_no:u64) -> Result<StakeInfo>{
        ctx.accounts.stake(stake_no)
    }

    pub fn unstake(ctx:Context<UnstakeContext>, stake_no:u64) -> Result<()>{
        ctx.accounts.unstake(stake_no)
    }
}
