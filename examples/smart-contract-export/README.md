# smart-contract-export

This example demonstrates generating executable smart contracts from legal statutes using `legalis-chain`. Three deterministic statutes (adult rights, welfare eligibility, senior discount) are compiled to 24+ blockchain target platforms: EVM targets including Solidity, Vyper, ZkSync Era, Base, Scroll, and Linea; WASM targets including Rust/WASM, Ink! (Substrate), and CosmWasm; Move targets for Aptos and Sui; and others such as Cairo (StarkNet), Solana, NEAR, TON, and Algorand. Only statutes without judicial discretion clauses qualify for on-chain deployment.

## Usage

```sh
cargo run -p smart-contract-export --all-features
```

## What It Demonstrates

- Determinism check: only DISCRETION-free statutes are exportable to smart contracts
- Solidity and Vyper contract generation for EVM chains
- Ink! (Substrate/Polkadot) and CosmWasm WASM contract output
- Move module generation for Aptos and Sui
- Cairo contract generation for StarkNet
- 24+ `TargetPlatform` variants via `ContractGenerator`

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
