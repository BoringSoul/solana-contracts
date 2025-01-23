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

  const supplyNo = new anchor.BN(3);
 
  const assetCollection = new PublicKey("initCollection的地址");
  console.log(`assetCollection: ${assetCollection}`);
  //MjadGGZfdowLjJbhf2xXUmGuydYtRV2AtCzbfzA9RCs
  const [assetAddress] = PublicKey.findProgramAddressSync([Buffer.from('asset'), assetCollection.toBuffer(), supplyNo.toBuffer("le", 8)], program.programId);
  console.log(`assetAddress: ${assetAddress}`);

  console.log(`payer: ${payer.publicKey}`);
  console.log(`  assetAddress : ${assetAddress}`);

  it('mint!', async () => {
    const metadata = {
      name: 'Homer NFT',
      symbol: 'HOMR',
      uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
    };
    const transactionSignature = await program.methods
      .mintNft(metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        assetCollection: assetCollection,
        asset: assetAddress,
      })
      .signers([payer])
      .rpc({ skipPreflight: true });
  });

});
