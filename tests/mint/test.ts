import * as anchor from '@coral-xyz/anchor';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair } from '@solana/web3.js';
import type { SolanaContracts } from '../../target/types/solana_contracts';
import { BN } from 'bn.js';

describe('NFT Minter', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.SolanaContracts as anchor.Program<SolanaContracts>;

  // The metadata for our NFT
  const metadata = {
    name: 'Homer NFT',
    symbol: 'HOMR',
    uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
    supply_no: new BN(1),
    assets: [{"amount": new BN(10000), "token_address": 'thisisthattokenaddress'}]
  };

  it('Create an NFT!', async () => {
    // Generate a keypair to use as the address of our mint account
    const mintKeypair = new Keypair();

    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);
    console.log(`AssosiateTokenAccount: ${associatedTokenAccountAddress}`);
    const transactionSignature = await program.methods
      .createNft(metadata.name, metadata.symbol, metadata.uri, metadata.supply_no, metadata.assets)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        associatedTokenAccount: associatedTokenAccountAddress,
      })
      .signers([mintKeypair])
      .rpc({ skipPreflight: true });

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
