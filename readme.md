## 项目结构
这是一个基于 Anchor 框架的 Solana 智能合约项目，主要实现了资产管理、NFT 铸造等功能。

## 配置文件
```
[toolchain]

[features]
resolution = true
skip-lint = false

[programs.localnet]
solana_contracts = "AbCFJLirBCwCJUprnGXBgSuMEtVQQiQYmEDsmsVAfFSB"

[programs.devnet]
solana_contracts = "AbCFJLirBCwCJUprnGXBgSuMEtVQQiQYmEDsmsVAfFSB"

[programs.mainnet]
solana_contracts = "AbCFJLirBCwCJUprnGXBgSuMEtVQQiQYmEDsmsVAfFSB"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "devnet"
wallet = '~/.config/solana/id.json'

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

## 核心功能
1. 资产操作
包含以下操作:
- 初始化资产管理器
- 打包资产(wrap)
- 解包资产(unwrap)
- 质押(stake)
- 解质押(unstake)
2. NFT 相关
- 铸造 NFT
- 销毁 NFT

## 使用说明
``` bash
# 1. 设置环境变量
source .env

# 2. 替换程序 ID
# 修改 target/types/solana-contracts.ts 中第8行的 address

# 3. 安装依赖
pnpm install

# 4. 运行测试
pnpm ts-mocha -p ./tsconfig.json tests/各个test脚本
```

## 测试用例
项目包含多个测试文件，用于测试不同功能：
- 完整流程测试 (Wrap -> Mint -> Stake -> Unstake)
- 资产管理器初始化测试
- NFT 铸造测试
- 质押和解质押测试

## 注意事项
- 确保已安装 Solana 工具链
- 确保有足够的 SOL 用于测试
- 测试前确认网络配置(devnet/testnet/mainnet)
- 注意保管私钥安全