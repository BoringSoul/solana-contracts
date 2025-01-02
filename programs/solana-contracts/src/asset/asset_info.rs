use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct AssetInfo {
    pub owner:Pubkey,
    pub supply_no: u64,
    #[max_len(5)]
    pub assets: Vec<Asset>,
    pub start_time: i64,
    pub mint_account: Pubkey,
    pub token_account: Pubkey,
}

#[derive(InitSpace, Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Asset {
    pub token_address: Pubkey,
    pub amount: u128
}

impl AssetInfo {
    pub const KEY_LEN: usize = 32; // Pubkey 长度为 32 字节

    pub fn key_len() -> usize {
        Self::KEY_LEN
    }
}