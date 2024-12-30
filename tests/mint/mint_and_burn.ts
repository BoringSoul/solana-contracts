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
  const payer = provider.wallet as anchor.Wallet;

  // Derive the PDA for the user's account.
  // const [assetInfoAccountAddr] = PublicKey.findProgramAddressSync([Buffer.from('ASSET_INFO'), payer.publicKey.toBuffer()], program.programId);
  const assetInfoAccountAddr = new Keypair();

  const mintKeypair = new Keypair();

  const associatedTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);
  
  const supply_no = new anchor.BN(1);
  const assets = [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}];

  console.log(` Payer : ${payer.publicKey}`);
  console.log(`programID = ${program.programId}}`)
  // console.log(`  assetInfoAccountAddr : ${assetInfoAccountAddr}`);

  const metadata = {
    name: 'Homer NFT',
    symbol: 'HOMR',
    uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
  };
              
  it('mint!', async () => {
    // Generate a keypair to use as the address of our mint account
    console.log(`  mintKeypair : ${mintKeypair.publicKey}`);

    // Derive the associated token address account for the mint and payer.
    console.log(` Token Account Address is  : ${associatedTokenAccountAddress}`);
    const transactionSignature = await program.methods
      .mintNft(metadata.name, metadata.symbol, metadata.uri, assetInfoAccountAddr.publicKey)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        associatedTokenAccount: associatedTokenAccountAddress
        // assetAccount: assetAccountKeypaire.publicKey,
      })
      .signers([mintKeypair
        // assetAccountKeypaire
      ])
      .rpc({ skipPreflight: true });

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });    

  it('burn', async () => {
    // Generate a keypair to use as the address of our mint account
    console.log(`  mintKeypair : ${mintKeypair.publicKey}`);

    console.log(` Token Account Address is  : ${associatedTokenAccountAddress}`);
    const transactionSignature = await program.methods
      .burnNft()
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        tokenAccount: associatedTokenAccountAddress,
      })
    .rpc({ skipPreflight: true });

    console.log('Success!');
    console.log(` Burn Transaction Signature: ${transactionSignature}`);
  });

});