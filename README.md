# Legalis-RS

**The Architecture of Generative Jurisprudence**

*Governance as Code, Justice as Narrative*

[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)

## Overview

Legalis-RS is a Rust framework for parsing, analyzing, and simulating legal statutes. It transforms natural language legal documents into structured, machine-verifiable code while preserving the essential distinction between:

- **Deterministic Logic (Code)**: Computationally derivable legal outcomes (age requirements, income thresholds, deadlines)
- **Judicial Discretion (Narrative)**: Areas requiring human interpretation and judgment

This separation is the philosophical core of Legalis-RS - it explicitly marks where AI-assisted legal processing must yield to human judgment, serving as a safeguard against algorithmic overreach in legal systems.

## Core Philosophy

```
"Not everything should be computable."
```

The `LegalResult<T>` type embodies this principle:

```rust
pub enum LegalResult<T> {
    Deterministic(T),           // Automated processing possible
    JudicialDiscretion { ... }, // Human judgment required
    Void { reason: String },    // Logical inconsistency detected
}
```

## Workspace Structure

```
legalis-rs/
├── crates/
│   ├── # Core Layer
│   ├── legalis-core/      # Core types, traits, state management
│   ├── legalis-dsl/       # Domain Specific Language parser
│   ├── legalis-registry/  # Statute registry with version control
│   ├── # Intelligence Layer
│   ├── legalis-llm/       # LLM integration (OpenAI, Anthropic, etc.)
│   ├── legalis-verifier/  # Formal verification (SMT solver)
│   ├── # Simulation & Analysis Layer
│   ├── legalis-sim/       # Simulation engine (ECS-like)
│   ├── legalis-diff/      # Statute diffing and change detection
│   ├── # Internationalization & Porting Layer
│   ├── legalis-i18n/      # Multi-language/jurisdiction support
│   ├── legalis-porting/   # Cross-jurisdiction law transfer
│   ├── # Output Layer
│   ├── legalis-viz/       # Visualization (decision trees, flowcharts)
│   ├── legalis-chain/     # Smart contract export (Solidity, WASM, Ink!)
│   ├── # Infrastructure Layer
│   ├── legalis-audit/     # Audit trail and decision logging
│   ├── legalis-api/       # REST API server
│   └── legalis-cli/       # Command-line interface
├── legalis.md             # Full specification document
├── Cargo.toml             # Workspace configuration
└── README.md
```

## Crates

### Core Layer
| Crate | Description |
|-------|-------------|
| `legalis-core` | Core type definitions: `LegalResult`, `Statute`, `Condition`, `Effect` |
| `legalis-dsl` | Parser for the Legal DSL syntax, converting text to structured AST |
| `legalis-registry` | Central statute registry with version control and tagging |

### Intelligence Layer
| Crate | Description |
|-------|-------------|
| `legalis-llm` | LLM provider abstraction (OpenAI, Anthropic) with law compiler |
| `legalis-verifier` | Static analysis for logical contradictions and constitutional conflicts |

### Simulation & Analysis Layer
| Crate | Description |
|-------|-------------|
| `legalis-sim` | Async simulation engine for testing laws against populations |
| `legalis-diff` | Statute diffing, change detection, and impact analysis |

### Internationalization & Porting Layer
| Crate | Description |
|-------|-------------|
| `legalis-i18n` | Multi-language support, locale handling, jurisdiction registry |
| `legalis-porting` | Cross-jurisdiction law transfer with cultural adaptation (Soft ODA) |

### Output Layer
| Crate | Description |
|-------|-------------|
| `legalis-viz` | Visualization: decision trees, flowcharts, dependency graphs |
| `legalis-chain` | Smart contract generation (Solidity, WASM, Ink!) |

### Infrastructure Layer
| Crate | Description |
|-------|-------------|
| `legalis-audit` | Audit trail with tamper-proof decision logging |
| `legalis-api` | REST API server for external integrations |
| `legalis-cli` | Command-line tool for parsing, verification, and export |

## Quick Start

### Prerequisites

- Rust 1.85+ (Edition 2024)
- Cargo

### Building

```bash
# Clone the repository
git clone https://github.com/legalis-rs/legalis
cd legalis

# Build all crates
cargo build

# Run tests
cargo test

# Check for issues
cargo clippy
```

### Basic Usage

```rust
use legalis_core::{Statute, Condition, Effect, EffectType, ComparisonOp};
use legalis_dsl::LegalDslParser;

// Parse a statute from DSL
let parser = LegalDslParser::new();
let statute = parser.parse_statute(r#"
    STATUTE adult-rights: "Adult Rights Act" {
        WHEN AGE >= 18
        THEN GRANT "Full legal capacity"
    }
"#)?;

// Or build programmatically
let statute = Statute::new(
    "voting-rights",
    "Voting Rights Act",
    Effect::new(EffectType::Grant, "Right to vote in elections"),
)
.with_precondition(Condition::Age {
    operator: ComparisonOp::GreaterOrEqual,
    value: 18,
});
```

### Running Simulations

```rust
use legalis_sim::{SimEngine, PopulationBuilder};

// Create a test population
let population = PopulationBuilder::new()
    .generate_random(1000)
    .build();

// Run simulation
let engine = SimEngine::new(vec![statute], population);
let metrics = engine.run_simulation().await;

println!("{}", metrics.summary());
```

### Verifying Statutes

```rust
use legalis_verifier::StatuteVerifier;

let verifier = StatuteVerifier::new();
let result = verifier.verify(&statutes);

if !result.passed {
    for error in result.errors {
        eprintln!("Verification error: {}", error);
    }
}
```

## Use Cases

### Phase 1: The Visualizer
Transform complex municipal ordinances into decision trees, highlighting ambiguous "gray zones" requiring human interpretation.

### Phase 2: The Debugger (Legislative DX)
Detect logical contradictions in draft legislation before enactment - treating legal bugs as compile errors.

### Phase 3: Soft ODA (Legal System Export)
Port legal frameworks across jurisdictions while adapting to local cultural parameters.

### Phase 4: The Hybrid Court
Automate `Deterministic` cases (small claims, administrative procedures) while routing `JudicialDiscretion` cases to human judges.

## LLM Integration

Legalis-RS provides pluggable LLM support through the `LLMProvider` trait:

```rust
use legalis_llm::{OpenAiClient, AnthropicClient, LawCompiler};

// Use OpenAI
let client = OpenAiClient::new("your-api-key", "gpt-4");
let compiler = LawCompiler::new(client);
let statute = compiler.compile("Any person aged 18 or older may vote.").await?;

// Or Anthropic
let client = AnthropicClient::new("your-api-key", "claude-3-opus");
```

## Smart Contract Export

Generate blockchain-deployable contracts from verified statutes:

```rust
use legalis_chain::{ContractGenerator, TargetPlatform};

let generator = ContractGenerator::new(TargetPlatform::Solidity);
let contract = generator.generate(&statute)?;

println!("{}", contract.source);
```

## Architecture Decisions

1. **No External Orchestrator Dependency**: Uses Rust's native async (Tokio) instead of external task queues
2. **Vendor-Agnostic LLM Layer**: Trait-based abstraction allows swapping providers without code changes
3. **Explicit Discretion Markers**: The type system enforces acknowledgment of human judgment requirements
4. **SMT Solver Integration**: Planned Z3 integration for formal verification of legal consistency

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please read the contribution guidelines before submitting pull requests.

## Acknowledgments

This project draws inspiration from legal informatics research and the growing field of computational law. The goal is not to replace human judgment in law, but to clarify where such judgment is necessary.

---

*"Code is Law" - but Law must preserve space for human narrative.*
