#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*, anchor_spl::{
        metadata::Metadata,
        token::{burn, close_account, Burn, CloseAccount, Mint, Token, TokenAccount},
    }
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
        seeds::program = token_metadata_program.key()
    )]
    pub edition_account:  UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BurnNft<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {

        // Step 1: Burn the token
        burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                Burn {
                    mint: self.mint_account.to_account_info(),
                    from: self.token_account.to_account_info(),
                    authority: self.signer.to_account_info(),
                },
            ),
            1,
        )?;

        // Step 2: Close the Token Account
        close_account(CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.token_account.to_account_info(),
                destination: self.signer.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        ))?;
        close_account(CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CloseAccount {
                account: self.metadata_account.to_account_info(),
                destination: self.signer.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        ))?;
        close_account(CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CloseAccount {
                account: self.edition_account.to_account_info(),
                destination: self.signer.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        ))?;
        Ok(())
    }
}