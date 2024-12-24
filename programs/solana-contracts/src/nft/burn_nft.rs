#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            self, mpl_token_metadata::{self}, Metadata
        },
        token::{burn, close_account, Burn, CloseAccount, Mint, Token, TokenAccount},
    },
    solana_program::{program::invoke, system_instruction},
};

#[derive(Accounts)]
pub struct BurnNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
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
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BurnNft<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {

        let token_metadata_program = self.token_metadata_program.key();
        let mint_account = self.mint_account.key();
        let metadata_seeds = &[
            b"metadata",
            token_metadata_program.as_ref(),
            mint_account.as_ref()
        ];
        let (_metadata_pda, _bump) =
            Pubkey::find_program_address(metadata_seeds, &self.token_metadata_program.key());
        let metadata_seeds: &[&[u8]] = &[b"metadata",
            token_metadata_program.as_ref(),
            mint_account.as_ref(), 
            &[_bump]];

        let master_edition_seeds = &[
            b"metadata",
            token_metadata_program.as_ref(),
            mint_account.as_ref(),
            b"edition",
        ];
        let (_master_edition_pda, _bump) =
            Pubkey::find_program_address(master_edition_seeds, &self.token_metadata_program.key());
        let master_edition_seeds = &[b"metadata", 
        token_metadata_program.as_ref(),
         mint_account.as_ref(), 
         b"edition",
        &[_bump]];
    

        // 将所有的 signer_seeds 放在一个数组中
        let signer_seeds: &[&[&[u8]]] = &[metadata_seeds, master_edition_seeds];

        let ix = mpl_token_metadata::instructions::BurnNft {
            collection_metadata: None,
            master_edition_account: self.edition_account.key(),
            metadata: self.metadata_account.key(),
            mint: self.mint_account.key(),
            owner: self.signer.key(),
            spl_token_program: self.token_program.key(),
            token_account: self.token_account.key(),
        }
        .instruction();
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &ToAccountInfos::to_account_infos(self),
            signer_seeds
        )?;
        Ok(())
    }
}