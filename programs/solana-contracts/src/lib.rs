#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
pub mod asset;
pub mod tokens;
pub mod nft;
pub mod errors;
use tokens::*;
use nft::*;
use asset::*;

// declare_id!("FQcgiPK79ZFAtBriM2MjVGeDYVHu9zDdSwmGqD1CaEoS");
declare_id!("7mQjTQKBv9QzW1xGyqHs4ECDzWRfWp8hMiyq1fTC3MEc");
#[program]
pub mod solana_contracts {

    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, token_title, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        tokens::mint::mint_token(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        tokens::transfer::transfer_tokens(ctx, amount)
    }

    pub fn mint_nft(
        ctx: Context<MintNft>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
        assets_address: String
    ) -> Result<()> {
        nft::mint_nft::mint_nft(ctx, token_title, token_symbol, token_uri, assets_address)
    }

    pub fn wrap_asset(
        ctx:Context<WrapContext>,
        supply_no:u64,
        assets: Vec<asset::Asset>
    ) -> Result<()>{
        asset::wrap(ctx, supply_no, assets)
    }
}
