#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

pub mod tokens;

use tokens::*;

declare_id!("FQcgiPK79ZFAtBriM2MjVGeDYVHu9zDdSwmGqD1CaEoS");

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
        mint::mint_token(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        transfer::transfer_tokens(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
