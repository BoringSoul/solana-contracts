#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
pub mod tokens;
pub mod nft;

use tokens::*;
use nft::*;

// declare_id!("FQcgiPK79ZFAtBriM2MjVGeDYVHu9zDdSwmGqD1CaEoS");
declare_id!("DvYFdnuamvwsxXDaHYAzfLSZ8AbDQd9BY2x71j6s87cs");
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

    pub fn create_nft(
        ctx: Context<NFTToken>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
        supply_no: u64,
        // assets: Vec<Asset>
    ) -> Result<()> {
        nft::mint::mint(ctx, token_title, token_symbol, token_uri, supply_no)
        // nft::mint::mint(ctx, token_title, token_symbol, token_uri, supply_no, assets)
    }
}
