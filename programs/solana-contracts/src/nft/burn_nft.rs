#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*, anchor_spl::{
        metadata::{Metadata, mpl_token_metadata::{self,}}, token::{ burn, Burn, Token, TokenAccount}
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
    pub remaining_accounts: Vec<AccountInfo<'info>>,
}

impl<'info> BurnNft<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {
        let (_metadata_account_key, _bump) = Self::_find_metadata_account_key(self);
        let metadata:Metadata = Self::_get_metadata_account(self, &_metadata_account_key)?;
        burn(CpiContext::new(self.token_program.to_account_info(), 
        Burn {
                mint:self.mint_account.to_account_info(),
                from:self.token_account.to_account_info(),
                authority:self.signer.to_account_info(),
            }), 
            1)
    }

    fn _find_metadata_account_key(&mut self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            self.mint_account.key().as_ref(),
        ],
        &mpl_token_metadata::ID)
    }

    fn _get_metadata_account(&mut self, metadata_account_key: &Pubkey) -> Result<Metadata> {
        // 查找元数据账户信息
        let metadata_account_info = self.remaining_accounts
            .iter()
            .find(|account| account.key == metadata_account_key)
            .ok_or_else(|| error!(crate::errors::ProgramErrorCode::MetadataAccountNotFound))?;
        let metadata_data = metadata_account_info.try_borrow_data()?;
        metadata_data. .map_err(|_| error!(crate::errors::ProgramErrorCode::InvalidMetadataAccount))
    }
}

// 新增 BurnNft 指令


