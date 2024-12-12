import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';

describe('wrap asset', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;
  const payer = provider.wallet as anchor.Wallet;

  // Derive the PDA for the user's account.
  const [assetInfoAccountAddr] = PublicKey.findProgramAddressSync([Buffer.from('ASSET_INFO'), payer.publicKey.toBuffer()], program.programId);
  // const assetInfoAccountAddr = new Keypair();
  // const mintKeypair = new Keypair();
  const supply_no = new anchor.BN(1);
  const assets = [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}];

  it('WrapAssest', async () => {
    await program.methods
      .wrapAsset(supply_no, assets)
      .accounts({
        user: payer.publicKey,
        assetInfo: assetInfoAccountAddr,
      })
      .rpc();

    // Fetch the account data
    const assetInfoAccount = await program.account.userState.fetch(assetInfoAccountAddr);
    assert.equal(assetInfoAccount.user.toBase58(), payer.publicKey.toBase58());
    assert.equal(assetInfoAccount.assets, assets);
    assert.equal(assetInfoAccount.supplyNo, supply_no);
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
