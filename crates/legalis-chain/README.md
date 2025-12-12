# legalis-chain

Smart contract export for Legalis-RS.

## Overview

This crate provides code generation capabilities to convert deterministic legal statutes into deployable smart contracts. It supports multiple blockchain platforms and ensures that only `Deterministic` (non-discretionary) statutes can be exported to immutable code.

## Supported Platforms

| Platform | Output |
|----------|--------|
| Solidity | Ethereum/EVM compatible contracts |
| RustWasm | WebAssembly modules with wasm-bindgen |
| Ink! | Substrate/Polkadot contracts |

## Usage

### Generate Solidity Contract

```rust
use legalis_chain::{ContractGenerator, TargetPlatform};
use legalis_core::{Statute, Condition, Effect, EffectType, ComparisonOp};

let statute = Statute::new(
    "adult-rights",
    "Adult Rights Act",
    Effect::new(EffectType::Grant, "Full legal capacity"),
)
.with_precondition(Condition::Age {
    operator: ComparisonOp::GreaterOrEqual,
    value: 18,
});

let generator = ContractGenerator::new(TargetPlatform::Solidity);
let contract = generator.generate(&statute)?;

println!("{}", contract.source);
// Output: Solidity contract with checkEligibility and applyEffect functions
```

### Generate WASM Module

```rust
let generator = ContractGenerator::new(TargetPlatform::RustWasm);
let contract = generator.generate(&statute)?;
// Output: Rust code with wasm_bindgen annotations
```

### Generate Ink! Contract

```rust
let generator = ContractGenerator::new(TargetPlatform::Ink);
let contract = generator.generate(&statute)?;
// Output: ink! contract for Substrate chains
```

### Batch Generation

```rust
let contracts: Vec<Result<GeneratedContract, ChainError>> =
    generator.generate_batch(&statutes);
```

## Generated Contract Structure

### Solidity Output

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title Adult Rights Act
/// @notice Auto-generated from Legalis-RS
contract AdultRights {
    address public owner;
    mapping(address => bool) public eligible;

    constructor() {
        owner = msg.sender;
    }

    /// @notice Check if an entity meets the preconditions
    function checkEligibility(uint256 age) public pure returns (bool) {
        require(age >= 18, "Age requirement not met");
        return true;
    }

    /// @notice Apply the legal effect
    function applyEffect(address beneficiary) public {
        require(msg.sender == owner, "Only owner can apply effects");
        eligible[beneficiary] = true;
    }
}
```

## Discretionary Statute Protection

Statutes with `discretion_logic` cannot be exported to smart contracts:

```rust
let discretionary_statute = Statute::new(...)
    .with_discretion("Consider special circumstances");

let result = generator.generate(&discretionary_statute);
assert!(matches!(result, Err(ChainError::DiscretionaryStatute(_))));
```

This is intentional - discretionary decisions require human judgment and cannot be encoded into immutable smart contract logic.

## Error Types

```rust
pub enum ChainError {
    DiscretionaryStatute(String),  // Cannot export discretionary statutes
    UnsupportedCondition(String),  // Condition type not supported
    UnsupportedEffect(String),     // Effect type not supported
    GenerationError(String),       // General generation error
}
```

## Generated Contract Type

```rust
pub struct GeneratedContract {
    pub name: String,              // Contract/module name
    pub source: String,            // Generated source code
    pub platform: TargetPlatform,  // Target platform
    pub abi: Option<String>,       // ABI (for Solidity)
}
```

## Condition Mapping

| legalis-core Condition | Solidity | Rust/WASM |
|------------------------|----------|-----------|
| Age comparison | `require(age >= N)` | `if !(age >= N) { return false; }` |
| Income comparison | `require(income <= N)` | `if !(income <= N) { return false; }` |
| AND | Multiple requires | Multiple if checks |
| OR | Combined require | Combined conditions |
| NOT | Negated require | Negated condition |

## License

MIT OR Apache-2.0
