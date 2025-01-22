#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use crate::asset::*;
use crate::events::wrap::WrapEvent;

/// 包装资产的上下文结构体，定义了执行wrap指令所需的所有账户
#[derive(Accounts)]
pub struct WrapContext<'info> {
    /// 交易发起者账户，必须签名，可变
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

    /// 新建的资产账户，使用PDA派生
    /// seeds为："asset"、asset_manager的地址和当前supply序号
    /// 由owner支付创建费用
    /// 分配空间为：8(discriminator) + AssetInfo结构体空间
    #[account(
        init,
        payer = owner,
        seeds = [b"asset", 
        asset_manager.key().as_ref(),
        &asset_manager.current_supply_no.to_le_bytes()],
        bump,
        space = 8 + AssetInfo::INIT_SPACE
    )]
    pub asset: Account<'info, AssetInfo>,

    /// 系统程序，用于创建账户
    pub system_program: Program<'info,System>,
    /// 租金账户，用于检查账户余额是否足够支付租金
    pub rent: Sysvar<'info, Rent>,
}

/// 包装资产的主要逻辑实现
/// 参数：
/// - ctx: 包含所有必要账户的上下文
/// - assets: 要包装的资产列表
/// 返回：新创建的资产账户的公钥字符串
pub fn wrap(ctx: Context<WrapContext>, 
    assets: Vec<Asset>) -> Result<String> {
    // 确保当前supply数量未超过限制
    assert!(ctx.accounts.asset_manager.current_supply_no <= ctx.accounts.asset_manager.limit);
    
    // 获取当前时间戳
    let clock = Clock::get()?;
    
    // 创建新的资产信息
    let data = AssetInfo {
        owner: ctx.accounts.owner.key(),          // 设置所有者
        supply_no: ctx.accounts.asset_manager.current_supply_no,  // 设置供应序号
        assets,                                   // 设置资产列表
        start_time: clock.unix_timestamp,         // 设置开始时间
        mint_account: Pubkey::default(),         // 设置默认mint账户
        token_account: Pubkey::default(),        // 设置默认token账户
    };
    
    // 将数据写入资产账户
    ctx.accounts.asset.set_inner(data.clone());
    
    // 增加资产管理器的供应计数
    ctx.accounts.asset_manager.current_supply_no += 1;
    
    // 发出包装事件
    emit!(WrapEvent {
        asset_info: data.clone(),
        asset_account: ctx.accounts.asset.key(),
        owner: ctx.accounts.owner.key(),
    });
    
    // 返回新创建的资产账户的公钥
    Ok(ctx.accounts.asset.key().to_string())
}

// 转账功能（已注释掉）
// pub fn transfer<'info>(sender:AccountInfo<'info>, receiver:AccountInfo<'info>,  amount:u64) ->Result<()> {
//     invoke(&system_instruction::transfer(sender.key, receiver.key, amount), &[sender.clone(), receiver.clone()])?;
//     Ok(())
// }
