
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';

// 定义 Asset 结构
class Asset {
    constructor(data) {
        this.id = new PublicKey(data.id); // 假设 Asset 结构中有一个 id 字段
        this.value = data.value; // 假设 Asset 结构中有一个 value 字段
    }
}

// 定义 AssetInfo 结构
class AssetInfo {
    constructor(data) {
        this.owner = new PublicKey(data.owner);
        this.supply_no = data.supply_no;
        this.assets = data.assets;
        this.start_time = data.start_time;
        this.mint_account = new PublicKey(data.mint_account);
        this.token_account = new PublicKey(data.token_account);
    }
}

// 解析 AssetInfo
function parseAssetInfo(buffer) {
    // 解析数据
    const owner = buffer.slice(0, 32); // Pubkey 是 32 字节
    const supply_no = buffer.readBigUInt64LE(32); // u64 从第 32 字节开始


    

    // 解析 assets
    const assetsLength = buffer.readUInt8(32); // 假设 assets 的长度在第 32字节
    const assets = [];
    let offset = 32; // 从第 113 字节开始解析 assets
    for (let i = 0; i < assetsLength; i++) {
        const assetId = buffer.slice(offset, offset+32); // 假设 Asset 有一个 id 字段
        const assetValue = buffer.readBigUInt64LE(offset +33, offset + 18); // 假设 Asset 有一个 value 字段
        assets.push(new Asset({ id: assetId, value: assetValue }));
        offset += 68; // 假设每个 Asset 占用 2 字节
    }

    const start_time = buffer.readBigInt64LE(40); // i64 从第 40 字节开始
    const mint_account = buffer.slice(48, 80); // Pubkey 是 32 字节
    const token_account = buffer.slice(80, 112); // Pubkey 是 32 字节

    return new AssetInfo({
        owner: owner,
        supply_no: supply_no,
        assets: assets,
        start_time: start_time,
        mint_account: mint_account,
        token_account: token_account,
    });
}

// 获取并解析数据
function getAssetInfo(base64String) {
    const buffer = Buffer.from(base64String, 'base64');
    const assetInfo = parseAssetInfo(buffer);
    console.log('资产信息:', assetInfo);
}

// 示例 Base64 字符串
const base64String = 'ZhPiTIgIOiBnndAwkocBqGU4K5RBVhEi24nSdTrbaLQBAAAAAAAAAAEAAABmE+JMiAg6IGed0DCShwGoZTgrlEFWESLbidJ1OttotBAnAAAAAAAAAAAAAAAAAABcbndnAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=';
getAssetInfo(base64String);