#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
pub mod asset;
pub mod nft;
pub mod errors;
pub mod events;

use nft::*;
use asset::*;

pub use events::wrap::*;
pub use events::mint::*;
pub use events::burn::*;
pub use events::stake::*;
pub use events::unstake::*;

declare_id!("E6qJKiFiq2f23Qf9Zx4cuqJTeJasi5XVrV1wvge6BZjc");
#[program]
pub mod solana_contracts {

    use super::*;
   


    pub fn init_asset_collection(ctx: Context<InitAssetCollectionContext>, limit:u64, contract_uri:String, start_time:i64, end_time:i64, wrap_fee:u64, unwrap_fee:u64) -> Result<()> {
        ctx.accounts.init(limit, contract_uri, start_time, end_time, wrap_fee, unwrap_fee)
    }


    pub fn wrap_asset(
        ctx:Context<WrapContext>,
        assets: Vec<asset::Asset>
    ) -> Result<String>{
        asset::wrap(ctx, assets)
    }

    pub fn unwrap_asset(
        ctx:Context<UnwrapContext>,
        supply_no:u64
    ) -> Result<()>{
        ctx.accounts.unwrap(supply_no)
    }

    pub fn mint_nft(
        ctx: Context<MintNft>,
        token_title: String,
        token_symbol: String,
        token_uri: String
    ) -> Result<()> {
        ctx.accounts.mint(token_title, token_symbol, token_uri)
    }
    

    pub fn burn_nft(
        ctx: Context<BurnNftContext>,
        supply_no:u64
    ) -> Result<()> {
        ctx.accounts.burn_nft(supply_no)
    }

    pub fn stake(ctx:Context<StakeContext>, stake_no:u64) -> Result<StakeInfo>{
        ctx.accounts.stake(stake_no)
    }

    pub fn unstake(ctx:Context<UnstakeContext>, stake_no:u64) -> Result<()>{
        ctx.accounts.unstake(stake_no)
    }
}
