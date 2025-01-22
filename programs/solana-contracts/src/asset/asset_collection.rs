
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct AssetCollection {
    //总供应量
    pub next_supply_no: u64,
    //限制供应量
    pub limit: u64,
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
pub struct InitAssetCollectionContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + std::mem::size_of::<AssetCollection>()
    )]
    pub asset_collection: Account<'info, AssetCollection>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitAssetCollectionContext<'info> {
    pub fn init(&mut self, limit:u64, contract_uri:String, start_time:i64, end_time:i64, wrap_fee:u64, unwrap_fee:u64) -> Result<()> {
        self.asset_collection.set_inner(AssetCollection {
            next_supply_no: 1,
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
