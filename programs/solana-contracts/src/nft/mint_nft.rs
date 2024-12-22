#![allow(clippy::result_large_err)]

use {
    crate::asset::asset_info::{Asset, AssetInfo}, 
    anchor_lang::{prelude::*, system_program}, 
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::{Collection, DataV2}, CreateMasterEditionV3, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    }
};

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
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

    // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    // #[account(
    //     seeds = [b"authority"],
    //     bump,
    // )]
    // /// CHECK: This is account is not initialized and is being used for signing purposes only
    // pub mint_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + AssetInfo::INIT_SPACE + AssetInfo::key_len(),
    )]
    pub asset_account: Account<'info, AssetInfo>,

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
        nft_uri:String, 
        supply_no: u64,
        assets: Vec<Asset>,
        // bumps: &MintNftBumps, 
    ) -> Result<()> {
        self.asset_account.assets = assets;
        self.asset_account.supply_no = supply_no;

        msg!("Create Data Account");
        self.create_data_account()?;

        msg!("Minting Token");
        // Cross Program Invocation (CPI)
        // Invoking the mint_to instruction on the token program
        mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint_account.to_account_info(),
                    to: self.associated_token_account.to_account_info(),
                    authority: self.payer.to_account_info(),
                },
            ),
            1,
        )?;

        // let seeds = &[
        //     &b"authority"[..], 
        //     &[bumps.mint_authority]
        // ];
        // let signer_seeds = &[&seeds[..]];

        // let cpi_program = self.token_program.to_account_info();
        // let cpi_accounts = MintTo {
        //     mint: self.mint_account.to_account_info(),
        //     to: self.associated_token_account.to_account_info(),
        //     authority: self.mint_authority.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        // mint_to(cpi_ctx, 1)?;
        
        msg!("Creating metadata account");
        // Cross Program Invocation (CPI)
        // Invoking the create_metadata_account_v3 instruction on the token metadata program
        create_metadata_accounts_v3(
            CpiContext::new(
                self.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: self.metadata_account.to_account_info(),
                    mint: self.mint_account.to_account_info(),
                    mint_authority: self.payer.to_account_info(),
                    update_authority: self.payer.to_account_info(),
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
                collection: Some(Collection {
                    verified: false,
                    key: self.asset_account.key(),
                }),
                uses: None,
            },
            false, // Is mutable
            true,  // Update authority is signer
            None,  // Collection details
        )?;

        msg!("Creating master edition account");
        // Cross Program Invocation (CPI)
        // Invoking the create_master_edition_v3 instruction on the token metadata program
        create_master_edition_v3(
            CpiContext::new(
                self.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: self.edition_account.to_account_info(),
                    mint: self.mint_account.to_account_info(),
                    update_authority: self.payer.to_account_info(),
                    mint_authority: self.payer.to_account_info(),
                    payer: self.payer.to_account_info(),
                    metadata: self.metadata_account.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
            ),
            None, // Max Supply
        )?;

        msg!("NFT minted successfully.");

        Ok(())
    }

    fn create_data_account(&mut self) -> Result<()> {

        let space = AssetInfo::INIT_SPACE + AssetInfo::key_len();
        let lamports_required = self.rent.minimum_balance(8 + AssetInfo::INIT_SPACE);

        msg!(
            "Create Mint and metadata account size and cost: {} lamports: {}",
            space as u64,
            lamports_required
        );

        system_program::create_account(
            CpiContext::new(
                self.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: self.payer.to_account_info(),
                    to: self.asset_account.to_account_info(),
                }
            ),
            lamports_required,
            space as u64,
            &self.payer.key()
        )
    }
}