import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { min } from 'bn.js';

describe('质押测试', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;
  
  const payerSeed = [137,208,207,116,77,151,229,89,133,115,90,11,15,150,170,146,200,189,204,252,244,79,250,34,105,231,32,148,110,252,2,170,102,19,226,76,136,8,58,32,103,157,208,48,146,135,1,168,101,56,43,148,65,86,17,34,219,137,210,117,58,219,104,180];
  const payer = Keypair.fromSecretKey(new Uint8Array(payerSeed));
  console.log(`payer: ${payer.publicKey}`);


  const authoritySeed  = [124,247,111,86,69,22,243,227,110,29,54,161,239,132,170,253,72,105,113,100,66,59,213,229,22,66,62,68,5,241,98,168,164,129,238,19,203,65,76,173,153,230,208,0,254,62,123,163,8,44,142,208,150,74,245,209,159,211,123,137,100,76,84,97];
  const authority = Keypair.fromSecretKey(new Uint8Array(authoritySeed));
  console.log(`authority: ${authority.publicKey}`);

  const supplyNo = new anchor.BN(4);
  const assetCollection = new PublicKey("B4Qn9uFmvdHb8YqEcbo7Dk41jvsSeiLDSBXjS4xuQDp9");
  console.log(`assetCollection: ${assetCollection}`);
  //MjadGGZfdowLjJbhf2xXUmGuydYtRV2AtCzbfzA9RCs
  const [assetAddress] = PublicKey.findProgramAddressSync([Buffer.from('asset'), assetCollection.toBuffer(), supplyNo.toBuffer("le", 8)], program.programId);
  console.log(`assetAddress: ${assetAddress}`);

  it('创建质押时的系统nft钱包账号(服务端)', async () => {
    await program.methods
    .initStakeAccount()
    .accounts({
      authority: authority.publicKey,
      asset:assetAddress
    })
    .signers([authority])
    .rpc();
  });

  it('开始质押', async () => {
    const stakeNo = new anchor.BN(1);
    await program.methods
    .stake(stakeNo)
    .accounts({
      owner: payer.publicKey,
      authority: authority.publicKey,
      asset:assetAddress
    })
    .signers([payer])
    .rpc();
  });

});
