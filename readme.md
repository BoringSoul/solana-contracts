1. source .env
2. 替换target/types/solana-contracts.ts 中第8行的address为 programId 对应的address
3. pnpm install
4. 跑这个命令 pnpm ts-mocha -p ./tsconfig.json tests/mint/wrap_and_mint.ts