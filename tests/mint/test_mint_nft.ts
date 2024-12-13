import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { ASSOCIATED_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/utils/token';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
} from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import type { SolanaContracts } from '../target/types/solana_contracts';

describe('extension_nft', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaContracts as Program<SolanaContracts>;
  const payer = provider.wallet as anchor.Wallet;

  it('开始铸造NFT!', async () => {
    const balance = await anchor.getProvider().connection.getBalance(payer.publicKey);

    if (balance < 1e8) {
      const res = await anchor.getProvider().connection.requestAirdrop(payer.publicKey, 1e9);
      await anchor.getProvider().connection.confirmTransaction(res, 'confirmed');
    }

    const mint = new Keypair();
    const assetInfoAccountAddr = new Keypair();
    console.log('铸币公钥', mint.publicKey.toBase58());

    const destinationTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      payer.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    const metadata = {
      name: 'Homer NFT',
      symbol: 'HOMR',
      uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
      supply_no: new anchor.BN(1),
      assets: [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}]
    };

    getOrCreateAssociatedTokenAccount;
    const tx = await program.methods
      .mintNft(metadata.name, metadata.symbol, metadata.uri, metadata.supply_no, metadata.assets)
      .accounts({
        signer: payer.publicKey,
        tokenAccount: destinationTokenAccount,
        mint: mint.publicKey,
        assetInfo: assetInfoAccountAddr.publicKey,
      })
      .signers([mint])
      .rpc();

    console.log('铸币交易TX', tx);
    await anchor.getProvider().connection.confirmTransaction(tx, 'confirmed');
  });
});
