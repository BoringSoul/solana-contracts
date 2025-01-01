import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';
import { mintTo } from '@solana/spl-token';

describe('wrap asset', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;
  
  const payerSeed = [15,196,183,124,198,253,42,117,98,229,215,248,91,58,182,242,105,152,238,88,250,19,9,186,69,29,241,101,192,77,18,24,80,247,51,149,154,154,38,15,255,34,35,219,129,249,156,166,225,131,8,134,104,133,249,58,192,5,180,204,69,155,239,146];
  const payer = Keypair.fromSecretKey(new Uint8Array(payerSeed));

  const mintKeypair = new Keypair();
  console.log(` mint(SecretKey) : ${mintKeypair.secretKey}`);
  const assets = [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}];

  const authoritySeed  = [124,247,111,86,69,22,243,227,110,29,54,161,239,132,170,253,72,105,113,100,66,59,213,229,22,66,62,68,5,241,98,168,164,129,238,19,203,65,76,173,153,230,208,0,254,62,123,163,8,44,142,208,150,74,245,209,159,211,123,137,100,76,84,97];
  const authority = Keypair.fromSecretKey(new Uint8Array(authoritySeed));

  const [assetInfoAccountAddr] = PublicKey.findProgramAddressSync([Buffer.from('asset_manager'), authority.publicKey.toBuffer()], program.programId);
  
  it('WrapAssest', async () => {
    await program.methods
      .wrapAsset(assets)
      .accounts({
        owner: payer.publicKey,
        authority: authority.publicKey,
        mint_account: mintKeypair.publicKey,
        asset_manager: assetInfoAccountAddr,
      })
      .signers([authority])
      .rpc();
    });
//     // Fetch the account data
//     const assetInfoAccount = await program.account.userState.fetch(assetInfoAccountAddr.publicKey);
//     assert.equal(assetInfoAccount.user.toBase58(), payer.publicKey.toBase58());
//     assert.equal(assetInfoAccount.assets, assets);
//     assert.equal(assetInfoAccount.supplyNo, supply_no);
//   });

});
