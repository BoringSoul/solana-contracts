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

  const payerSeed = [234,37,215,94,223,96,108,63,168,0,208,27,19,14,86,26,228,154,190,92,67,13,109,209,77,230,28,105,48,29,71,205,228,169,214,51,84,76,34,252,7,27,72,165,6,13,224,232,140,146,238,12,66,14,71,250,95,198,53,145,113,61,114,147];
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
