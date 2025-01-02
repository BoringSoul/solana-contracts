#![allow(clippy::result_large_err)]

use {
    crate::{AssetInfo, AssetManager}, anchor_lang::prelude::*, anchor_spl::{
        metadata::{burn_nft, BurnNft, Metadata},
        token::{burn, close_account, Burn, CloseAccount, Mint, Token, TokenAccount},
    }
};

#[derive(Accounts)]
pub struct BurnNftContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

     /// CHECK: Validate address by deriving pda
     #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub edition_account:  UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"asset_manager", authority.key().as_ref()],
        bump,
    )]
    pub asset_manager: Account<'info, AssetManager>,


    #[account(
        mut,
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump,
        close = owner,
    )]
    pub asset: Account<'info, AssetInfo>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BurnNftContext<'info> {
    pub fn burn_nft(&mut self, supply_no:u64) -> Result<()> {

        burn_nft(CpiContext::new(
            self.token_metadata_program.to_account_info(), 
            BurnNft {
                edition:  self.edition_account.to_account_info(),
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint_account.to_account_info(),
                owner: self.owner.to_account_info(),
                spl_token: self.token_program.to_account_info(),
                token: self.token_account.to_account_info(),
            }
        ), None)
    }
}