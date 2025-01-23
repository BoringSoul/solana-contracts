use {
    crate::{asset::*, events::StakeEvent},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken, 
        metadata::Metadata, 
        token::{transfer, Mint, Token, TokenAccount, Transfer}
    },
};

#[derive(Accounts)]
pub struct StakeContext<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: Not Validated
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

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
    pub stake_account: Box<Account<'info, TokenAccount>>,

    
    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"stake", asset.key().as_ref()],
        bump,
        space = 8 + StakeInfo::INIT_SPACE
    )]
    pub stake: Account<'info, StakeInfo>,

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
        self.asset.token_account = self.stake_account.key();

        transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.owner_token_account.to_account_info(),
                    to: self.stake_account.to_account_info(),
                    authority: self.owner.to_account_info(),
                },
            ),
            1
        )?;
        emit!(StakeEvent {
            asset_account: self.asset.key(),
            stake_account: self.stake.key(),
            mint_account: self.mint_account.key(),
            owner_account: self.owner.key(),
            authority_account: self.authority.key(),
            owner_token_account: self.owner_token_account.key(),
            staker_token_account: self.stake_account.key(),
        });
        Ok(self.stake.clone().into_inner())
    }
    
}