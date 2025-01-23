use {
    crate::asset::*,
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken, 
        metadata::Metadata, 
        token::{Mint, Token, TokenAccount}
    },
};

#[derive(Accounts)]
pub struct StakeAccountContext<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"asset", 
        asset.collection_id.as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump,
    )]
    pub asset: Account<'info, AssetInfo>,

    #[account(
        mut,
        seeds = [b"mint", asset.key().as_ref()],
        bump
    )]
    pub mint_account: Box<Account<'info, Mint>>,


    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_account,
        associated_token::authority = authority
    )]
    pub stake_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl <'info> StakeAccountContext<'info> {

    pub fn init_stake_account(&mut self) -> Result<()> {
        Ok(())
    }
    
}