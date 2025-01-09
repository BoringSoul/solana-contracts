use anchor_lang::prelude::*;

#[event]
pub struct UnstakeEvent {
    pub asset_account: Pubkey,
    pub stake_account: Pubkey,
    pub mint_account: Pubkey,
    pub owner_account: Pubkey,
    pub authority_account: Pubkey,
    pub owner_token_account: Pubkey,
    pub staker_token_account: Pubkey,
}