import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';

describe('wrap asset', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;

  const authoritySeed  = [124,247,111,86,69,22,243,227,110,29,54,161,239,132,170,253,72,105,113,100,66,59,213,229,22,66,62,68,5,241,98,168,164,129,238,19,203,65,76,173,153,230,208,0,254,62,123,163,8,44,142,208,150,74,245,209,159,211,123,137,100,76,84,97];
  const authority = Keypair.fromSecretKey(new Uint8Array(authoritySeed));
  console.log(` authority : ${authority.publicKey}`);

  // Derive the PDA for the user's account.
  const [assetInfoAccountAddr] = PublicKey.findProgramAddressSync([Buffer.from('asset_manager'), authority.publicKey.toBuffer()], program.programId);
  // const assetInfoAccountAddr = new Keypair();
  // const mintKeypair = new Keypair();
  // const assetInfoAccount = Keypair.generate();
  console.log(`assetInfoAccountAddr : ${assetInfoAccountAddr}`);

  it('initAssetManager', async () => {
    await program.methods
      .initAssetManager(new anchor.BN(2000), 
      "baidu.com", 
      new anchor.BN(1735277947523), 
      new anchor.BN(1735278947523),
      new anchor.BN(3), 
      new anchor.BN(3),
    )
    .accounts({
      owner: authority.publicKey
    })
    .signers([authority])
    .rpc();
  });
});
