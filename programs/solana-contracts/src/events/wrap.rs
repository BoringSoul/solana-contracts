use anchor_lang::prelude::*;
use crate::asset::AssetInfo;

#[event]
pub struct WrapEvent {
    pub asset_info: AssetInfo,
    pub asset_key: Pubkey,
}