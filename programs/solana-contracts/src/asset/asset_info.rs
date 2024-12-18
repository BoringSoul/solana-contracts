use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AssetInfo {
    pub user:Pubkey,
    pub supply_no: u64,
    #[max_len(5)]
    pub assets: Vec<Asset>
}

#[derive(InitSpace, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Asset {
    pub token_address: Pubkey,
    pub amount: u128
}