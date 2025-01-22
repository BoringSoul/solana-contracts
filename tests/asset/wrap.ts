import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';

describe('打包资产', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;
  
  const payerSeed = [137,208,207,116,77,151,229,89,133,115,90,11,15,150,170,146,200,189,204,252,244,79,250,34,105,231,32,148,110,252,2,170,102,19,226,76,136,8,58,32,103,157,208,48,146,135,1,168,101,56,43,148,65,86,17,34,219,137,210,117,58,219,104,180];
  const payer = Keypair.fromSecretKey(new Uint8Array(payerSeed));
  console.log(`payer: ${payer.publicKey}`);

  const assetCollection = PublicKey.from("9ZRVL57MtSDk26k6oSy1Zbm1n1XXXFm7dePSgqeNr6Kq");

  it('打包', async () => {
  const assets = [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}];
    await program.methods
      .wrapAsset(assets)
      .accounts({
        owner: payer.publicKey,
        assetCollection: assetCollection,
      })
      .signers([payer])
      .rpc();
    });

});
