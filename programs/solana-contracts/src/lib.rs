
#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
pub mod manager;
pub mod asset;
pub mod nft;
pub mod errors;

use manager::*;
use nft::*;
use asset::*;

declare_id!("2HjXiq4PGKKXTdBsdQe9CxRr5mCHa8N87wUWauZqbZfi");
#[program]
pub mod solana_contracts {


    use super::*;
    pub fn mint_nft(
        ctx: Context<MintNft>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
        asset_info_key: Pubkey
        // supply_no:u64,
        // assets: Vec<asset::Asset>
    ) -> Result<()> {
        ctx.accounts.mint(token_title, token_symbol, token_uri, 
            asset_info_key,
            // supply_no,
            // assets
        )
    }


    pub fn init_asset_manager(ctx: Context<InitContext>, limit:u128, contract_uri:String, start_time:i64, end_time:i64, wrap_fee:u64, unwrap_fee:u64) -> Result<()> {
        ctx.accounts.init(limit, contract_uri, start_time, end_time, wrap_fee, unwrap_fee)
    }


    pub fn update_asset(ctx: Context<UpdateAssetManagerContext>) -> Result<AssetManager> {
        ctx.accounts.add_asset()
    }

    pub fn burn_nft(
        ctx: Context<BurnNft>,
    ) -> Result<()> {
        ctx.accounts.burn_nft()
    }

    pub fn wrap_asset(
        ctx:Context<WrapContext>,
        supply_no:u64,
        assets: Vec<asset::Asset>
    ) -> Result<()>{
        asset::wrap(ctx, supply_no, assets)
    }

    pub fn unwrap_asset(
        ctx:Context<UnwrapContext>
    ) -> Result<()>{
        ctx.accounts.unwrap()
    }
}
