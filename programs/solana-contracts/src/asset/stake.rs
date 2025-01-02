use crate::asset::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::Metadata, 
    token::{transfer, Mint, Token, TokenAccount, Transfer}
};

#[derive(Accounts)]
pub struct StakeContext<'info> {
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
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump,
    )]
    pub asset: Account<'info, AssetInfo>,


    #[account(
        seeds = [b"mint", 
        asset_manager.key().as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump
    )]
    pub mint_account: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = owner,
        seeds = [b"stake", 
        asset.key().as_ref()],
        bump,
        space = 8 + StakeInfo::INIT_SPACE
    )]
    pub stake: Account<'info, StakeInfo>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = owner
    )]
    pub owner_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = authority,
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

impl <'info> StakeContext<'info> {

    pub fn stake(&mut self, 
        stake_no: u64,
    ) -> Result<StakeInfo> {
        // assert_eq!(self.asset.owner, self.owner.key());
        let clock = Clock::get()?;
        self.stake.set_inner(StakeInfo {
            owner: self.owner.key(),
            stake_no,
            asset_no: self.asset.supply_no,
            start_time: clock.unix_timestamp,
            asset_account: self.asset.key()
        });

        
        self.asset.owner = self.authority.key();
        self.asset.token_account = self.authority_token_account.key();

        transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.owner_token_account.to_account_info(),
                    to: self.authority_token_account.to_account_info(),
                    authority: self.owner.to_account_info(),
                },
            ),
            1
        )?;
        Ok(self.stake.clone().into_inner())
    }
    
}