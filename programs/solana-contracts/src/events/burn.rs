use anchor_lang::prelude::*;

#[event]
pub struct BurnEvent {
    pub asset_account: Pubkey,
    pub mint_account: Pubkey,
    pub token_account: Pubkey
}