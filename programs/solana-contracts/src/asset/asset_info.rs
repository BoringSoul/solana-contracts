use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AssetInfo {

    pub bump:u8,
    pub user:Pubkey,
    pub supply_no: u64,
    #[max_len(5)]
    pub assets: Vec<Asset>
}

#[derive(InitSpace, Debug)]
#[account]
pub struct Asset {
    pub token_address: Pubkey,
    pub amount: u128
}