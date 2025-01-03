import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { min } from 'bn.js';

describe('Wrap -> Mint -> Stake-> Unstake', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;
  
  const payerSeed = [137,208,207,116,77,151,229,89,133,115,90,11,15,150,170,146,200,189,204,252,244,79,250,34,105,231,32,148,110,252,2,170,102,19,226,76,136,8,58,32,103,157,208,48,146,135,1,168,101,56,43,148,65,86,17,34,219,137,210,117,58,219,104,180];
  const payer = Keypair.fromSecretKey(new Uint8Array(payerSeed));
  console.log(`payer: ${payer.publicKey}`);

  // const mintKeypair = Keypair.fromSecretKey(new Uint8Array([6,228,116,176,184,251,15,87,10,226,14,84,30,119,2,141,147,191,50,161,150,176,158,131,98,77,157,108,87,169,250,207,38,110,92,181,25,227,103,240,156,104,148,209,32,181,189,58,248,161,239,37,232,156,124,158,130,104,211,183,120,123,105,69]));
 

  const authoritySeed  = [124,247,111,86,69,22,243,227,110,29,54,161,239,132,170,253,72,105,113,100,66,59,213,229,22,66,62,68,5,241,98,168,164,129,238,19,203,65,76,173,153,230,208,0,254,62,123,163,8,44,142,208,150,74,245,209,159,211,123,137,100,76,84,97];
  const authority = Keypair.fromSecretKey(new Uint8Array(authoritySeed));
  console.log(`authority: ${authority.publicKey}`);

  const [assetManagerAddress] = PublicKey.findProgramAddressSync([Buffer.from('asset_manager'), authority.publicKey.toBuffer()], program.programId);
  console.log(`assetManagerAddress: ${assetManagerAddress}`);

  const supplyNo = new anchor.BN(1);
  //MjadGGZfdowLjJbhf2xXUmGuydYtRV2AtCzbfzA9RCs
  const [assetAddress ] = PublicKey.findProgramAddressSync([Buffer.from('asset'), assetManagerAddress.toBuffer(), supplyNo.toBuffer("le", 8)], program.programId);
  console.log(`assetAddress: ${assetAddress}`);
  
  const stakeNo = new anchor.BN(1);

  it('WrapAssest', async () => {
    const assets = [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}];
    await program.methods
      .wrapAsset(assets)
      .accounts({
        owner: payer.publicKey,
        authority: authority.publicKey,
        assetManager: assetManagerAddress,
      })
      .signers([payer, authority])
      .rpc();
  });

   
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
        authority: authority.publicKey,
        assetManager: assetManagerAddress,
        asset: assetAddress,
      })
      .signers([payer, 
        authority
      ])
      .rpc({ skipPreflight: true });
  });

  it('Stake', async () => {
    await program.methods
      .stake(stakeNo)
      .accounts({
        owner: payer.publicKey,
        authority: authority.publicKey,
        assetManager: assetManagerAddress,
        asset: assetAddress
      })
      .signers([payer, authority])
      .rpc();
    });
  it('Unstake', async () => {
    const [stakeAddress ] = PublicKey.findProgramAddressSync([Buffer.from('stake'), assetAddress.toBuffer()], program.programId);
    console.log(`stakeAddress: ${stakeAddress}`);
    await program.methods
    .unstake(stakeNo)
    .accounts({
      owner: payer.publicKey,
      authority: authority.publicKey,
      assetManager: assetManagerAddress,
      stake:stakeAddress
      // mintAccount: mintKeypair.publicKey,
      // associatedTokenAccount:associatedTokenAccountAddress
    })
    .signers([payer, authority])
    .rpc();
  });

});
