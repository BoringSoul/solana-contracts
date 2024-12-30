#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*, anchor_spl::{
        metadata::{Metadata, burn_nft, BurnNft},
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

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BurnNftContext<'info> {
    pub fn burn_nft(&mut self) -> Result<()> {

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


        // // Step 1: Burn the token
        // burn(
        //     CpiContext::new(
        //         self.token_program.to_account_info(),
        //         Burn {
        //             mint: self.mint_account.to_account_info(),
        //             from: self.token_account.to_account_info(),
        //             authority: self.owner.to_account_info(),
        //         },
        //     ),
        //     1,
        // )?;

        // // Step 2: Close the Token Account
        // close_account(CpiContext::new(
        //     self.token_program.to_account_info(),
        //     CloseAccount {
        //         account: self.token_account.to_account_info(),
        //         destination: self.owner.to_account_info(),
        //         authority: self.owner.to_account_info(),
        //     },
        // ))?;
        // msg!("metadata_account = {:?}",self.metadata_account.to_account_info());
        // close_account(CpiContext::new(
        //     self.token_metadata_program.to_account_info(),
        //     CloseAccount {
        //         account: self.metadata_account.to_account_info(),
        //         destination: self.owner.to_account_info(),
        //         authority: self.owner.to_account_info(),
        //     },
        // ))?;
        // msg!("edition_account = {:?}",self.edition_account.to_account_info());
        // close_account(CpiContext::new(
        //     self.token_metadata_program.to_account_info(),
        //     CloseAccount {
        //         account: self.edition_account.to_account_info(),
        //         destination: self.owner.to_account_info(),
        //         authority: self.owner.to_account_info(),
        //     },
        // ))?;
        // Ok(())
    }
}