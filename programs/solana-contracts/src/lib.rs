
#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
pub mod asset;
pub mod tokens;
pub mod nft;
pub mod errors;
use nft::*;
use asset::*;

declare_id!("D8q5HBdGEkm9wVMVs75FP38D2BXD665f7xCmL9K2v5Dq");
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
