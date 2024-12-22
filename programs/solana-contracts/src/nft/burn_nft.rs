#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*, 
    anchor_spl::{
        metadata::{
            mpl_token_metadata::{self,},
            MetadataAccount
        }, 
        token::{ burn, Burn, Mint, Token, TokenAccount}
    }
};
#[derive(Accounts)]
pub struct BurnNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: Validate address by deriving pda
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account(
        constraint = {
            if let Some(collection) = &metadata_account.collection {
                collection.key == asset_info.key()
            } else {
                false
            }
        },
    )]
    pub asset_info: Account<'info, crate::asset::AssetInfo>,
    pub token_metadata_program: Program<'info, anchor_spl::metadata::Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

impl<'info> BurnNft<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {
        self.asset_info.close(self.signer.to_account_info())?;
        burn(CpiContext::new(self.token_program.to_account_info(), 
        Burn {
                mint:self.mint_account.to_account_info(),
                from:self.token_account.to_account_info(),
                authority:self.signer.to_account_info(),
            }), 
            1)
    }

    fn _find_metadata_account_key(&mut self) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"metadata",
                mpl_token_metadata::ID.as_ref(),
                self.mint_account.key().as_ref(),
            ],
        &mpl_token_metadata::ID
        )
    }

    // fn _get_metadata_account(&mut self, metadata_account_key: &Pubkey) -> Result<Metadata> {
    //     // 查找元数据账户信息
    //     // let metadata_account_info = self.remaining_accounts
    //     //     .iter()
    //     //     .find(|account| account.key == metadata_account_key)
    //     //     .ok_or_else(|| error!(crate::errors::ProgramErrorCode::MetadataAccountNotFound))?;
    //     let metadata_data = self.metadata_account.try_borrow_data()?;
    //     Metadata::safe_deserialize(&metadata_data)
    //     .map_err(|_| error!(crate::errors::ProgramErrorCode::MetadataDeserializeError))
    // }
}