import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';

describe('wrap asset', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;

  const payerSeed = [137,208,207,116,77,151,229,89,133,115,90,11,15,150,170,146,200,189,204,252,244,79,250,34,105,231,32,148,110,252,2,170,102,19,226,76,136,8,58,32,103,157,208,48,146,135,1,168,101,56,43,148,65,86,17,34,219,137,210,117,58,219,104,180];
  const payer = Keypair.fromSecretKey(new Uint8Array(payerSeed));

  const authoritySeed  = [124,247,111,86,69,22,243,227,110,29,54,161,239,132,170,253,72,105,113,100,66,59,213,229,22,66,62,68,5,241,98,168,164,129,238,19,203,65,76,173,153,230,208,0,254,62,123,163,8,44,142,208,150,74,245,209,159,211,123,137,100,76,84,97];
  const authority = Keypair.fromSecretKey(new Uint8Array(authoritySeed));

  const supplyNo = new anchor.BN(1);
  const [assetInfoAccountAddr] = PublicKey.findProgramAddressSync([Buffer.from('asset_manager'), authority.publicKey.toBuffer()], program.programId);
  const [assetAddress ] = PublicKey.findProgramAddressSync([Buffer.from('asset'), assetInfoAccountAddr.toBuffer(), supplyNo.toBuffer("le", 8)], program.programId);
  

  console.log(`payer: ${payer.publicKey}`);
  console.log(`authority: ${authority.publicKey}`);
  console.log(`programID = ${program.programId}`)
  console.log(`  assetInfoAccountAddr : ${assetInfoAccountAddr}`);

  it('burn', async () => {
    // Generate a keypair to use as the address of our mint account
    const transactionSignature = await program.methods
      .burnNft()
      .accounts({
        owner: payer.publicKey,
        authority: authority.publicKey,
        assetManager: assetInfoAccountAddr,
        asset: assetAddress,
      })
    .signers([payer, authority])
    .rpc({ skipPreflight: true });
    console.log('Success!');
    console.log(` Burn Transaction Signature: ${transactionSignature}`);
  });

});
