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
│   ├── # Interoperability Layer
│   ├── legalis-interop/   # Import/export: Catala, Stipula, L4 formats
│   ├── # Output Layer
│   ├── legalis-viz/       # Visualization (decision trees, flowcharts)
│   ├── legalis-chain/     # Smart contract export (Solidity, WASM, Ink!)
│   ├── legalis-lod/       # Linked Open Data (RDF/TTL) export
│   ├── # Infrastructure Layer
│   ├── legalis-audit/     # Audit trail and decision logging
│   ├── legalis-api/       # REST API server
│   └── legalis-cli/       # Command-line interface
├── jurisdictions/
│   └── jp/                # Japanese legal system implementation
├── examples/
│   ├── jp-constitution-3d/ # 3D visualization of Japanese Constitution
│   └── welfare-benefits/   # Welfare benefits eligibility system
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

### Interoperability Layer
| Crate | Description |
|-------|-------------|
| `legalis-interop` | Import/export for Catala, Stipula, L4 legal DSL formats |

### Output Layer
| Crate | Description |
|-------|-------------|
| `legalis-viz` | Visualization: decision trees, flowcharts, dependency graphs |
| `legalis-chain` | Smart contract generation (Solidity, WASM, Ink!) |
| `legalis-lod` | Linked Open Data (RDF/TTL) export for semantic web integration |

### Infrastructure Layer
| Crate | Description |
|-------|-------------|
| `legalis-audit` | Audit trail with tamper-proof decision logging |
| `legalis-api` | REST API server for external integrations |
| `legalis-cli` | Command-line tool for parsing, verification, and export |

### Jurisdictions
| Jurisdiction | Description |
|--------------|-------------|
| `jp` | Japanese legal system implementation with localization support |

### Examples
| Example | Description |
|---------|-------------|
| `jp-constitution-3d` | 3D visualization of the Japanese Constitution demonstrating multi-dimensional legal relationships |
| `welfare-benefits` | Welfare benefits eligibility determination system showcasing rule-based processing |

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

## Legal DSL Interoperability

Legalis-RS can import from and export to other legal DSL formats:

```rust
use legalis_interop::{LegalConverter, LegalFormat};

let converter = LegalConverter::new();

// Auto-detect and import from Catala
let catala_source = r#"
declaration scope AdultRights:
  context input content integer
"#;
let (statutes, report) = converter.auto_import(catala_source)?;

// Export to L4 format
let (l4_output, _) = converter.export(&statutes, LegalFormat::L4)?;

// Direct format conversion
let (stipula_output, _) = converter.convert(
    catala_source,
    LegalFormat::Catala,
    LegalFormat::Stipula
)?;
```

### Supported Formats

| Format | Origin | Features |
|--------|--------|----------|
| **Catala** | Inria, France | Literate programming, scope-based, strong typing |
| **Stipula** | U. Bologna, Italy | Smart contracts, party/asset model, state machines |
| **L4** | Singapore | Deontic logic (MUST/MAY/SHANT), rule-based reasoning |
| **Akoma Ntoso** | OASIS Standard | XML legislative documents, semantic markup |

## Linked Open Data Export

Export statutes to RDF/TTL format for semantic web integration:

```rust
use legalis_lod::LodExporter;

let exporter = LodExporter::new();
let ttl_output = exporter.export_to_turtle(&statutes)?;

// Or export to RDF/XML
let rdf_output = exporter.export_to_rdf_xml(&statutes)?;
```

This enables integration with knowledge graphs and semantic web systems, allowing legal data to be linked with other open data sources.

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
