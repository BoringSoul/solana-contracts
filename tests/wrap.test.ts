import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import {Keypair ,PublicKey } from '@solana/web3.js';
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    getOrCreateAssociatedTokenAccount,
  } from '@solana/spl-token';
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
  const supply_no = new anchor.BN(1);
  const assets = [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}];

  console.log(` 支付方Address : ${payer.publicKey}`);
  console.log(`  资产信息Address : ${assetInfoAccountAddr}`);

  it('打包资产', async () => {
    await program.methods
      .wrapAsset(supply_no, assets)
      .accounts({
        user: payer.publicKey,
        assetInfo: assetInfoAccountAddr.publicKey,
      })
      .signers([assetInfoAccountAddr])
      .rpc();

    // Fetch the account data
    const assetInfoAccount = await program.account.userState.fetch(assetInfoAccountAddr.publicKey);
    assert.equal(assetInfoAccount.user.toBase58(), payer.publicKey.toBase58());
    assert.equal(assetInfoAccount.assets, assets);
    assert.equal(assetInfoAccount.supplyNo, supply_no);
  });

  it('开始铸造NFT!', async () => {
    const balance = await anchor.getProvider().connection.getBalance(payer.publicKey);

    if (balance < 1e8) {
      const res = await anchor.getProvider().connection.requestAirdrop(payer.publicKey, 1e9);
      await anchor.getProvider().connection.confirmTransaction(res, 'confirmed');
    }

    const mint = new Keypair();
    console.log('铸币账号的公钥', mint.publicKey.toBase58());

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
      assets: [{"amount": new anchor.BN(10000), "tokenAddress": payer.publicKey}],
      assets_account: assetInfoAccountAddr
    };

    getOrCreateAssociatedTokenAccount;
    const tx = await program.methods
      .mintNft(metadata.name, metadata.symbol, metadata.uri, assetInfoAccountAddr.publicKey)
      .accounts({
        signer: payer.publicKey,
        tokenAccount: destinationTokenAccount,
        mint: mint.publicKey,
        // assetInfo: assetInfoAccountAddr.publicKey,
      })
      .signers([mint])
      .rpc();

    console.log('铸币成功, TX', tx);
    await anchor.getProvider().connection.confirmTransaction(tx, 'confirmed');
  });
});
