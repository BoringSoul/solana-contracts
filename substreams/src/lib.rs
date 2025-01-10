mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::BurnEvent;
use pb::substreams::v1::program::MintEvent;
use pb::substreams::v1::program::StakeEvent;
use pb::substreams::v1::program::UnstakeEvent;
use pb::substreams::v1::program::WrapEvent;
use pb::substreams::v1::program::InitAssetManager;
use pb::substreams::v1::program::Stake;
use pb::substreams::v1::program::Unstake;
use pb::substreams::v1::program::WrapAsset;
use pb::substreams::v1::program::UnwrapAsset;


use pb::substreams::v1::program::AssetInfo;


use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "E6qJKiFiq2f23Qf9Zx4cuqJTeJasi5XVrV1wvge6BZjc";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut burn_event_list: Vec<BurnEvent> = Vec::new();
    let mut mint_event_list: Vec<MintEvent> = Vec::new();
    let mut stake_event_list: Vec<StakeEvent> = Vec::new();
    let mut unstake_event_list: Vec<UnstakeEvent> = Vec::new();
    let mut wrap_event_list: Vec<WrapEvent> = Vec::new();
    let mut init_asset_manager_list: Vec<InitAssetManager> = Vec::new();
    let mut stake_list: Vec<Stake> = Vec::new();
    let mut unstake_list: Vec<Unstake> = Vec::new();
    let mut wrap_asset_list: Vec<WrapAsset> = Vec::new();
    let mut unwrap_asset_list: Vec<UnwrapAsset> = Vec::new();

    blk.transactions().for_each(|transaction| {

        // ------------- EVENTS -------------
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();
        let programs_selector: ProgramsSelector = ProgramsSelector::new(&["*".to_string()]);
        let log_contexts = LogContext::parse_logs_basic(&meta.log_messages, &programs_selector);

        log_contexts
            .iter()
            .filter(|context| context.program_id == PROGRAM_ID)
            .for_each(|context| {
                context.data_logs.iter().for_each(|data| {
                    if let Ok(decoded) = BASE64_STANDARD.decode(data) {
                        let slice_u8: &mut &[u8] = &mut &decoded[..];
                        let slice_discriminator: [u8; 8] =
                            slice_u8[0..8].try_into().expect("error");

                        match slice_discriminator {
                            idl::idl::program::events::BurnEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::BurnEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    burn_event_list.push(BurnEvent {
                                        trx_hash: transaction.id(),
                                        asset_account: event.asset_account.to_string(),
                                        mint_account: event.mint_account.to_string(),
                                        token_account: event.token_account.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::MintEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::MintEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    mint_event_list.push(MintEvent {
                                        trx_hash: transaction.id(),
                                        asset_account: event.asset_account.to_string(),
                                        mint_account: event.mint_account.to_string(),
                                        token_account: event.token_account.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::StakeEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::StakeEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    stake_event_list.push(StakeEvent {
                                        trx_hash: transaction.id(),
                                        asset_account: event.asset_account.to_string(),
                                        stake_account: event.stake_account.to_string(),
                                        mint_account: event.mint_account.to_string(),
                                        owner_account: event.owner_account.to_string(),
                                        authority_account: event.authority_account.to_string(),
                                        owner_token_account: event.owner_token_account.to_string(),
                                        staker_token_account: event.staker_token_account.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::UnstakeEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UnstakeEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    unstake_event_list.push(UnstakeEvent {
                                        trx_hash: transaction.id(),
                                        asset_account: event.asset_account.to_string(),
                                        stake_account: event.stake_account.to_string(),
                                        mint_account: event.mint_account.to_string(),
                                        owner_account: event.owner_account.to_string(),
                                        authority_account: event.authority_account.to_string(),
                                        owner_token_account: event.owner_token_account.to_string(),
                                        staker_token_account: event.staker_token_account.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::WrapEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WrapEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    wrap_event_list.push(WrapEvent {
                                        trx_hash: transaction.id(),
                                        asset_info: Some(AssetInfo {
						owner: event.asset_info.owner.to_string(),supply_no: event.asset_info.supply_no,
					}),
                                        asset_account: event.asset_account.to_string(),
                                        owner: event.owner.to_string(),
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                });
            });// ------------- INSTRUCTIONS -------------
        transaction
        .walk_instructions()
        .into_iter()
        .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
        .for_each(|inst| {
            let slice_u8: &[u8] = &inst.data()[..];if slice_u8[0..8] == idl::idl::program::client::args::InitAssetManager::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitAssetManager::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    init_asset_manager_list.push(InitAssetManager {
                        trx_hash: transaction.id(),
                        limit: instruction.limit,
                        contract_uri: instruction.contract_uri,
                        start_time: instruction.start_time,
                        end_time: instruction.end_time,
                        wrap_fee: instruction.wrap_fee,
                        unwrap_fee: instruction.unwrap_fee,
                        acct_owner: accts[0].to_string(),
                        acct_asset_manager: accts[1].to_string(),
                    });
                }
            }if slice_u8[0..8] == idl::idl::program::client::args::Stake::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Stake::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    stake_list.push(Stake {
                        trx_hash: transaction.id(),
                        stake_no: instruction.stake_no,
                        acct_owner: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_asset_manager: accts[2].to_string(),
                        acct_asset: accts[3].to_string(),
                        acct_stake: accts[4].to_string(),
                        acct_mint_account: accts[5].to_string(),
                        acct_owner_token_account: accts[6].to_string(),
                        acct_authority_token_account: accts[7].to_string(),
                    });
                }
            }if slice_u8[0..8] == idl::idl::program::client::args::Unstake::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Unstake::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    unstake_list.push(Unstake {
                        trx_hash: transaction.id(),
                        stake_no: instruction.stake_no,
                        acct_owner: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_asset_manager: accts[2].to_string(),
                        acct_asset: accts[3].to_string(),
                        acct_stake: accts[4].to_string(),
                        acct_mint_account: accts[5].to_string(),
                        acct_owner_token_account: accts[6].to_string(),
                        acct_authority_token_account: accts[7].to_string(),
                    });
                }
            }if slice_u8[0..8] == idl::idl::program::client::args::WrapAsset::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::WrapAsset::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    wrap_asset_list.push(WrapAsset {
                        trx_hash: transaction.id(),
                        assets: instruction.assets.into_iter().map(|assets| ).collect(),
                        acct_owner: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_asset_manager: accts[2].to_string(),
                        acct_asset: accts[3].to_string(),
                    });
                }
            }if slice_u8[0..8] == idl::idl::program::client::args::UnwrapAsset::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UnwrapAsset::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    unwrap_asset_list.push(UnwrapAsset {
                        trx_hash: transaction.id(),
                        supply_no: instruction.supply_no,
                        acct_owner: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_asset_manager: accts[2].to_string(),
                        acct_asset: accts[3].to_string(),
                    });
                }
            }
        });
    });


    Data {
        burn_event_list,
        mint_event_list,
        stake_event_list,
        unstake_event_list,
        wrap_event_list,
        init_asset_manager_list,
        stake_list,
        unstake_list,
        wrap_asset_list,
        unwrap_asset_list,
    }
}



fn map_option_asset_info(value: Option<idl::idl::program::types::AssetInfo>) -> Option<AssetInfo> {
    match value {
        Some(asset_info) => {
            return Some(AssetInfo {
                owner: asset_info.owner.to_string(),
                supply_no: asset_info.supply_no,
            })
        },
        None => {
            return None;
        }
    }
}

