use std::borrow::{Borrow, BorrowMut};

use anchor_lang::prelude::*;

use crate::Asset;

#[account]
#[derive(InitSpace)]
pub struct AssetManager {
    //总供应量
    pub total_supply: u128,
    //限制供应量
    pub limit: u128,
    //合约的uri
    #[max_len(100)]
    pub contract_uri: String,
    //打包费率
    pub wrap_fee: u64,
    //解包费率
    pub unwrap_fee: u64,
    // 开始时间
    pub start_time:i64,
    // 结束时间
    pub end_time:i64,
}

#[derive(Accounts)]
pub struct InitContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [b"asset_manager"],
        bump,
        space = 8 + std::mem::size_of::<AssetManager>()
    )]
    pub asset_manager: Account<'info, AssetManager>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitContext<'info> {
    pub fn init(&mut self, limit:u128, contract_uri:String, start_time:i64, end_time:i64, wrap_fee:u64, unwrap_fee:u64) -> Result<()> {
        self.asset_manager.set_inner(AssetManager {
            total_supply: 0,
            limit,
            contract_uri,
            start_time,
            end_time,
            wrap_fee,
            unwrap_fee,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateAssetManagerContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"asset_manager"],
        bump,
    )]
    pub asset_manager: Account<'info, AssetManager>,
}

impl<'info> UpdateAssetManagerContext<'info> {
    pub fn add_asset(&mut self) -> Result<AssetManager> {
        let supply_no = self.asset_manager.total_supply;
        self.asset_manager.total_supply = supply_no + 1;
        let data = AssetManager {
            total_supply: supply_no,
            limit: self.asset_manager.limit,
            contract_uri: self.asset_manager.contract_uri.clone(),
            start_time: self.asset_manager.start_time,
            end_time: self.asset_manager.end_time,
            wrap_fee: self.asset_manager.wrap_fee,
            unwrap_fee: self.asset_manager.unwrap_fee
        };
        Ok(data)
    }
}