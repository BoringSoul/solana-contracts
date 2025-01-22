#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
// use anchor_spl::token::Mint;  // 暂未使用的代币铸造导入
use crate::asset::*;

/// 解包资产的上下文结构体，定义了执行unwrap指令所需的所有账户
#[derive(Accounts)]
pub struct UnwrapContext<'info> {
    /// 交易发起者账户，必须签名，可变
    /// 这个账户将接收被关闭账户的租金返还
    #[account(mut)]
    pub owner: Signer<'info>,

    /// 权限账户，必须签名，可变
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// 资产管理器账户，使用PDA派生
    /// seeds为："asset_manager" 和 authority的公钥
    #[account(
        mut,
        seeds = [b"asset_manager", authority.key().as_ref()],
        bump,
    )]
    pub asset_manager: Account<'info, AssetManager>,

    /// 要解包的资产账户，使用PDA派生
    /// seeds为："asset"、asset_manager的地址和supply序号
    /// close = owner 表示账户将被关闭，租金返还给owner
    #[account(
        mut,
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &asset.supply_no.to_le_bytes()],
        bump,
        close = owner,
    )]
    pub asset: Account<'info, AssetInfo>
}

/// UnwrapContext的实现
impl<'info> UnwrapContext<'info> {
    /// 解包资产的主要逻辑实现
    /// 参数：
    /// - supply_no: 要解包的资产供应序号
    /// 返回：操作结果
    pub fn unwrap(&mut self, supply_no:u64) -> Result<()> {
        // 验证解包操作的发起者是资产的所有者
        assert_eq!(self.owner.key(), self.asset.owner);
        // 验证供应序号匹配
        assert_eq!(supply_no, self.asset.supply_no);
        Ok(())
    }
}
