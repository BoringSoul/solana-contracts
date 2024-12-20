#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*, anchor_spl::{
        metadata::{mpl_token_metadata::types::DataV2, Metadata}, token::{ burn, Burn, Token, TokenAccount}
    }
};
#[derive(Accounts)]
pub struct BurnNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,
    // #[account(init_if_needed, seeds = [b"nft_authority".as_ref()], bump, space = 8, payer = signer)]
    // pub nft_authority: Account<'info, crate::mint_nft::NftAuthority>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BurnNft<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {
        let metadata = self.token_metadata_program.deserialize_data::<AccountInfo<'info>>();
        burn(CpiContext::new(self.token_program.to_account_info(), 
        Burn {
                mint:self.mint_account.to_account_info(),
                from:self.token_account.to_account_info(),
                authority:self.signer.to_account_info(),
            }), 
            1)
    }
}

// 新增 BurnNft 指令


