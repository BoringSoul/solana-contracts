use crate::asset::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::Metadata, 
    token::{transfer, Mint, Token, TokenAccount, Transfer}};



#[derive(Accounts)]
pub struct UnstakeContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

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
        seeds = [b"stake", 
        stake.asset_account.as_ref()],
        bump,
        close = owner
    )]
    pub stake: Account<'info, StakeInfo>,

    #[account(
        mut,
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &stake.asset_no.to_le_bytes()],
        bump,
    )]
    pub asset: Account<'info, AssetInfo>,


    #[account(
        mut,
        seeds = [b"mint", 
        asset_manager.key().as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump
    )]
    pub mint_account: Box<Account<'info, Mint>>,


    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = owner
    )]
    pub owner_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = authority
    )]
    pub authority_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl <'info> UnstakeContext<'info> {

    pub fn unstake(&mut self, 
        stake_no: u64
    ) -> Result<()> {
        msg!("self.stake_no = {:?}, input stake_no = {:?}", self.stake.stake_no, stake_no);
        assert_eq!(self.stake.stake_no, stake_no);
        // assert_eq!(self.stake.owner, self.owner.key());
        // assert_eq!(self.asset.owner, self.authority.key());

        self.asset.owner = self.owner.key();
        self.asset.token_account = self.owner_token_account.key();
        
        transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    to: self.owner_token_account.to_account_info(),
                    from: self.authority_token_account.to_account_info(),
                    authority: self.authority.to_account_info(),
                },
            ),
            1
        )?;
        Ok(())
    }
    
}