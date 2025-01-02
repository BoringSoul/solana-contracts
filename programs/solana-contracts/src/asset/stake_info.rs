use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct StakeInfo {
    pub owner:Pubkey,
    pub stake_no: u64,
    pub asset_no: u64,
    pub start_time: i64,
    pub asset_account: Pubkey
}


impl StakeInfo {
    pub const KEY_LEN: usize = 32; // Pubkey 长度为 32 字节

    pub fn key_len() -> usize {
        Self::KEY_LEN
    }
}