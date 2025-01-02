#![allow(clippy::result_large_err)]

use {
    crate::{Asset, AssetInfo, AssetManager}, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    }
    
};

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
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
    )]
    pub asset: Box<Account<'info, AssetInfo>>,


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

    // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = payer,
        seeds = [b"mint", 
        asset_manager.key().as_ref(),
        &asset_manager.current_supply.to_le_bytes()],
        bump,
        mint::decimals = 0,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key(),
    )]
    pub mint_account: Box<Account<'info, Mint>>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer
    )]
    pub associated_token_account: Box<Account<'info, TokenAccount>>,



    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MintNft<'info> {
    pub fn mint(&mut self, 
        nft_name:String, 
        nft_symbol:String, 
        nft_uri:String
    ) -> Result<()> {

        self.asset.token_account = self.associated_token_account.key();
        self.asset.mint_account = self.mint_account.key();

        // msg!("Minting Token");
        // Cross Program Invocation (CPI)
        // Invoking the mint_to instruction on the token program
        mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint_account.to_account_info(),
                    to: self.associated_token_account.to_account_info(),
                    authority: self.authority.to_account_info(),
                },
            ),
            1,
        )?;
        
        // msg!("Creating metadata account");
        // Cross Program Invocation (CPI)
        // Invoking the create_metadata_account_v3 instruction on the token metadata program
        create_metadata_accounts_v3(
            CpiContext::new(
                self.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: self.metadata_account.to_account_info(),
                    mint: self.mint_account.to_account_info(),
                    mint_authority: self.authority.to_account_info(),
                    update_authority: self.authority.to_account_info(),
                    payer: self.payer.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
            ),
            DataV2 {
                name: nft_name,
                symbol: nft_symbol,
                uri: nft_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            false, // Is mutable
            false,  // Update authority is signer
            None,  // Collection details
        )?;

        // msg!("Creating master edition account");
        // Cross Program Invocation (CPI)
        // Invoking the create_master_edition_v3 instruction on the token metadata program
        create_master_edition_v3(
            CpiContext::new(
                self.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: self.edition_account.to_account_info(),
                    mint: self.mint_account.to_account_info(),
                    update_authority: self.authority.to_account_info(),
                    mint_authority: self.authority.to_account_info(),
                    payer: self.payer.to_account_info(),
                    metadata: self.metadata_account.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
            ),
            None, // Max Supply
        )?;

        Ok(())
    }
}