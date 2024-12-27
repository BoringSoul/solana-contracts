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
  const payer = provider.wallet as anchor.Wallet;

  // Derive the PDA for the user's account.
  // const [assetInfoAccountAddr] = PublicKey.findProgramAddressSync([Buffer.from('ASSET_INFO'), payer.publicKey.toBuffer()], program.programId);
  const assetInfoAccountAddr = new Keypair();
  // const mintKeypair = new Keypair();

  console.log(` Payer : ${payer.publicKey}`);
  console.log(`  assetInfoAccountAddr : ${assetInfoAccountAddr}`);

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
        onwer: payer.publicKey
      })
      .rpc();
  });

  // it('Close Account', async () => {
  //   await program.methods
  //     .closeUser()
  //     .accounts({
  //       user: payer.publicKey,
  //       userAccount: userAccountAddress,
  //     })
  //     .rpc();

  //   // The account should no longer exist, returning null.
  //   const userAccount = await program.account.userState.fetchNullable(userAccountAddress);
  //   assert.equal(userAccount, null);
  // });
});
