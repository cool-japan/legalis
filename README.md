# Legalis-RS

**The Architecture of Generative Jurisprudence**

*Governance as Code, Justice as Narrative*

[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.1.1-brightgreen.svg)](RELEASE-0.1.1.md)
[![Crates](https://img.shields.io/badge/crates-23-blue.svg)](#crates)
[![Jurisdictions](https://img.shields.io/badge/jurisdictions-7%20operational-green.svg)](#jurisdictions)
[![Tests](https://img.shields.io/badge/tests-9580%20passing-success.svg)](#crates)
[![Files](https://img.shields.io/badge/rust%20files-1062-orange.svg)](#workspace-structure)

## Overview

Legalis-RS is a Rust framework for parsing, analyzing, and simulating legal statutes across **multiple jurisdictions**. It transforms natural language legal documents into structured, machine-verifiable code while preserving the essential distinction between:

- **Deterministic Logic (Code)**: Computationally derivable legal outcomes (age requirements, income thresholds, deadlines)
- **Judicial Discretion (Narrative)**: Areas requiring human interpretation and judgment

This separation is the philosophical core of Legalis-RS - it explicitly marks where AI-assisted legal processing must yield to human judgment, serving as a safeguard against algorithmic overreach in legal systems.

### Supported Legal Systems

**🌍 Global Coverage**: 7 operational jurisdictions spanning **Civil Law**, **Common Law**, and **Supranational** legal traditions:

- 🇩🇪 **Germany** - Civil Law (BGB, StGB, Grundgesetz)
- 🇪🇺 **European Union** - Supranational Law (GDPR, Competition Law, Treaties)
- 🇫🇷 **France** - Civil Law (Code civil, Code du travail, 35-hour work week)
- 🇯🇵 **Japan** - Civil Law + Asian Tradition (Minpō, Labor, IP Law, e-Gov integration)
- 🇸🇬 **Singapore** - Common Law + Statutory (Companies Act, Employment Act, PDPA, Banking)
- 🇬🇧 **United Kingdom** - Common Law (Employment Rights, Consumer Rights, Financial Services)
- 🇺🇸 **United States** - Common Law (Restatement of Torts, 51 jurisdictions, Choice of Law)

**Total**: 1,062 Rust files, 29 examples, 9,568 tests passing across all jurisdictions.

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
│   └── legalis/           # Command-line interface
├── jurisdictions/
│   ├── de/                # Germany (Civil Law): BGB, StGB, Grundgesetz
│   ├── eu/                # European Union: GDPR, Competition Law, Treaties
│   ├── fr/                # France (Civil Law): Code civil, Code du travail
│   ├── jp/                # Japan (Civil Law + Asian): Minpō, Labor, IP Law
│   ├── sg/                # Singapore (Common Law): Companies, Employment, Banking
│   ├── uk/                # United Kingdom (Common Law): Employment, Consumer, Financial
│   └── us/                # USA (Common Law): Restatement, 51 jurisdictions
├── examples/
│   ├── jp-constitution-3d/ # 3D visualization of Japanese Constitution
│   └── welfare-benefits/   # Welfare benefits eligibility system
├── legalis.md             # Full specification document
├── Cargo.toml             # Workspace configuration
└── README.md
```

## Crates

All 23 crates (16 core + 7 jurisdictions) compile cleanly with **NO WARNINGS** - strict quality policy enforced.

### Core Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-core` | 0.3.0 | 631 | Core type definitions: `LegalResult`, `Statute`, `Condition`, `Effect`. Case law database, TypedEntity system, hierarchical relationships. |
| `legalis-dsl` | 0.2.0 | 453 | Parser for the Legal DSL syntax with LSP support, REPL, error recovery, AST optimization. |
| `legalis-registry` | 0.2.9 | 680 | Central statute registry with distributed Raft consensus, vector search, blockchain anchoring, real-time collaboration. |

### Intelligence Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-llm` | 0.4.8 | 543 | LLM provider abstraction (OpenAI, Anthropic, Gemini, Ollama) with law compiler, federated learning, neuro-symbolic integration. |
| `legalis-verifier` | 0.2.9 | 392 | Static analysis with Z3 SMT solver, temporal logic (LTL/CTL), formal methods (Coq, Lean 4), distributed verification. |

### Simulation & Analysis Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-sim` | 0.2.9 | 643 | Async simulation engine with GPU acceleration, distributed execution, agent-based modeling, economic/healthcare/urban simulation. |
| `legalis-diff` | 0.4.0 | ✓ | Statute diffing with AI-powered analysis, ML integration, quantum-ready algorithms, time-travel diffing. |

### Internationalization & Porting Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-i18n` | 0.3.4 | 584 | Multi-language support (60+ languages), ICU message format, legal citation formatting (Bluebook, OSCOLA, etc.), RTL support. |
| `legalis-porting` | 0.3.0 | 298 | Cross-jurisdiction law transfer with cultural adaptation (Soft ODA), multi-hop porting chains, confidence scoring. |

### Interoperability Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-interop` | 0.2.8 | 465 | Import/export for Catala, Stipula, L4, Akoma Ntoso, LegalRuleML, BPMN, DMN formats. |

### Output Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-viz` | 0.2.0 | 453 | Visualization with VR/AR support, 3D/holographic display, AI-powered selection, legal history scrollytelling. |
| `legalis-chain` | 0.3.7 | ✓ | Smart contract generation (Solidity, WASM, Ink!, Move, Cairo, CosmWasm) with gas optimization, L2 support. |
| `legalis-lod` | 0.3.9 | 799 | Linked Open Data (RDF/TTL) export, SPARQL queries, OWL reasoning, triple store integration. |

### Infrastructure Layer
| Crate | Version | Tests | Description |
|-------|---------|-------|-------------|
| `legalis-audit` | 0.2.4 | 529 | Audit trail with blockchain anchoring, Merkle trees, SIEM integration, compliance frameworks (GDPR, SOX, ISO 27001). |
| `legalis-api` | 0.2.3 | 200 | REST + gRPC + GraphQL APIs with OAuth2/OIDC, WebSocket, SSE, API Gateway features. |
| `legalis` (CLI) | 0.2.3 | ✓ | Command-line tool with AI-powered features, interactive TUI, workflow automation, cloud integration. |

### Jurisdictions

Legalis-RS now includes comprehensive support for **7 major jurisdictions** (all fully implemented):

| Jurisdiction | Status | Files | Examples | Description |
|--------------|--------|-------|----------|-------------|
| **`de`** (Germany) | ✅ Complete | 42 | 22 | **Civil Law System**: BGB (Bürgerliches Gesetzbuch), StGB (Strafgesetzbuch), Grundgesetz. Comprehensive tort law (§823, §826), constitutional rights validation, employment law (Arbeitsrecht) with dismissal protection. |
| **`eu`** (European Union) | ✅ Complete | 35 | 17 | **Supranational Law**: GDPR (Articles 6-83), Consumer Rights Directive, Competition Law (Articles 101-102 TFEU), Charter of Fundamental Rights, Four Freedoms (goods, persons, services, capital). |
| **`fr`** (France) | ✅ Complete | 53 | 6 | **Civil Law System**: Code civil (Napoleonic Code 1804, 2016 reform), Code de commerce (SA/SARL/SAS), **Code du travail (35-hour work week)**, Constitution de 1958. 154 tests passing, 50+ articles. |
| **`jp`** (Japan) | ✅ Complete | 72 | 15 | **Civil Law + Asian Tradition**: 民法 (Minpō Articles 709-715), 商法/会社法 (Companies Act), 労働基準法 (Labor Standards), 知的財産法 (IP Law), 消費者保護法 (Consumer Protection). **176 tests, 13,400+ lines**. e-Gov XML parser, Wareki (和暦) calendar support. |
| **`us`** (United States) | ✅ Complete | 75 | 0 | **Common Law System**: Restatement of Torts (ALI §158, §46, §402A), **51 jurisdictions** (50 states + DC) with comparative/contributory negligence variations, Choice of Law (5 approaches), Uniform Acts (UCC/UPA), Federal-State boundary analysis, Professional licensing (UBE, IMLC, NCARB). **378 tests passing, 15,000+ lines**. |
| **`sg`** (Singapore) | ✅ Complete | 150 | 10 | **Common Law + Statutory**: Companies Act (Cap. 50), Employment Act (Cap. 91), PDPA 2012, Consumer Protection, **IP Laws** (Patents/Trademarks/Copyright/Designs), **Banking Act (Cap. 19)** Basel III CAR, **Payment Services Act 2019** DPT/Crypto. **150 tests, 14,800+ lines**. ACRA/UEN, CPF, MAS Notice 637/626, AML/CFT, Safeguarding, **trilingual errors** (EN/中文/Melayu). |
| **`uk`** (United Kingdom) | ✅ Complete | 50+ | 7 | **Common Law**: Employment Rights Act 1996, Working Time Regulations, Equality Act 2010, Consumer Rights Act 2015, Financial Services and Markets Act 2000, Companies Act 2006. Employment contracts, redundancy calculations, consumer remedies, FCA authorization. |

**Total**: 1,062 Rust source files, 67+ working examples across 7 operational jurisdictions

### Examples

#### Japan Examples
| Example | Description |
|---------|-------------|
| `jp-constitution-3d` | 3D visualization of the Japanese Constitution demonstrating multi-dimensional legal relationships |
| `welfare-benefits` | Welfare benefits eligibility determination system showcasing rule-based processing |
| `minpo-709-tort` | Japanese Civil Code Article 709 tort simulation |
| `comparative-tort-law` | Comparative tort law analysis across Japan, Germany, France, and USA |
| `jp-drone-regulation` | Aviation Act drone regulations: registration, flight categories, Level 4 |

#### International Examples
| Example | Country | Description |
|---------|---------|-------------|
| `eu-gdpr-compliance` | EU | GDPR data protection compliance checker (Articles 6-49) |
| `uk-employment-law` | UK | Employment Rights Act 1996, Working Time Regulations, Equality Act 2010 |
| `brazil-consumer-protection` | Brazil | Consumer Defense Code (CDC Lei 8.078/1990) |
| `india-rti-act` | India | Right to Information Act 2005 transparency law |
| `singapore-business` | Singapore | Companies Act, PDPA, Employment Act compliance |
| `australia-immigration` | Australia | Migration Act 1958 visa eligibility (189, 190, 500, etc.) |
| `canada-healthcare` | Canada | Canada Health Act and provincial health plans (OHIP, MSP, RAMQ) |
| `korea-labor-law` | South Korea | Labor Standards Act (근로기준법) worker protections |
| `mexico-tax-law` | Mexico | Codigo Fiscal de la Federacion (ISR, IVA, IEPS) |
| `thailand-business` | Thailand | Foreign Business Act, BOI Investment Promotion |

#### Advanced/Research Examples
| Example | Focus | Description |
|---------|-------|-------------|
| `soviet-law-history` | Historical | USSR 1922-1991 legal system reconstruction for basic research |
| `private-international-law` | PIL | Conflict of Laws: Japan PIL, Rome I/II, Hague Conventions |
| `laos-civil-code` | Soft ODA | Laos Civil Code 2020 - Japan's legal technical assistance case study |
| `religious-legal-systems` | Comparative | Canon Law, Islamic Finance, Jewish/Hindu Personal Law (academic) |

#### Technical Feature Examples
| Example | Crate | Description |
|---------|-------|-------------|
| `smart-contract-export` | legalis-chain | Export statutes to Solidity, WASM, Ink!, Move contracts |
| `legal-knowledge-graph` | legalis-lod | RDF/TTL, JSON-LD export with SPARQL generation |
| `statute-version-control` | legalis-registry | Version history, snapshots, backup/restore |
| `legal-dsl-interop` | legalis-interop | Catala, L4, Stipula DSL conversion |
| `multilingual-statute` | legalis-i18n | Multi-language display with citation formatting |

## Quick Start

### Prerequisites

- Rust 1.85+ (Edition 2024)
- Cargo

### Building

```bash
# Clone the repository
git clone https://github.com/cool-japan/legalis
cd legalis

# Build all crates (default features: includes REST + gRPC APIs, no Z3 required)
cargo build

# Build without gRPC (minimal dependencies)
cargo build --no-default-features

# Run tests
cargo test

# Check for issues
cargo clippy
```

#### Building with Z3 SMT Solver (Optional)

The `legalis-verifier` crate has an optional `z3-solver` feature for rigorous formal verification. To build with all features including Z3 support:

```bash
# Install Z3 (macOS)
brew install z3

# Install Z3 (Ubuntu/Debian)
sudo apt install libz3-dev

# Install Z3 (Fedora/RHEL)
sudo dnf install z3-devel

# Install Z3 (Arch Linux)
sudo pacman -S z3

# Setup Z3 environment variables (macOS/Linux) - REQUIRED for all-features builds
source setup-z3-env.sh

# Build with all features
cargo build --all-features

# Run tests with all features
cargo nextest run --all-features
```

**Important**: You MUST source the `setup-z3-env.sh` script in every shell session where you want to build with `--all-features`. The script automatically detects your platform and configures the necessary environment variables.

Alternatively, you can use [direnv](https://direnv.net/) with the included `.envrc` file for automatic environment setup whenever you `cd` into the project directory.

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
4. **SMT Solver Integration**: Currently uses Z3 (v0.12) for formal verification of legal consistency
   - **Note**: In v0.2.0, Z3 will be replaced with a Pure Rust SMT solver from the COOLJAPAN Ecosystem to achieve 100% Pure Rust implementation
5. **Object Storage**: S3-compatible storage support for audit trails
   - **Note**: MinIO can be replaced with rs3gw (Pure Rust S3-compatible gateway) from the COOLJAPAN Ecosystem

## Documentation

### Release Notes
- **[v0.1.1](RELEASE-0.1.1.md)** (January 10, 2026) - Jurisdiction Expansion: EU, Singapore, UK
- **[v0.1.0](RELEASE-0.1.0.md)** (January 5, 2026) - Genesis: Initial release

### Technical Papers
Detailed technical papers are available in multiple languages:

| Language | Document |
|----------|----------|
| 日本語 (Japanese) | [PAPER-JA.md](docs/PAPER-JA.md) |
| English | [PAPER-EN.md](docs/PAPER-EN.md) |
| Français (French) | [PAPER-FR.md](docs/PAPER-FR.md) |
| Deutsch (German) | [PAPER-DE.md](docs/PAPER-DE.md) |
| 中文 (Chinese) | [PAPER-ZH.md](docs/PAPER-ZH.md) |
| ไทย (Thai) | [PAPER-TH.md](docs/PAPER-TH.md) |
| ລາວ (Lao) | [PAPER-LO.md](docs/PAPER-LO.md) |
| Tiếng Việt (Vietnamese) | [PAPER-VI.md](docs/PAPER-VI.md) |
| Bahasa Indonesia | [PAPER-ID.md](docs/PAPER-ID.md) |
| Bahasa Melayu (Malay) | [PAPER-MS.md](docs/PAPER-MS.md) |
| العربية (Arabic) | [PAPER-AR.md](docs/PAPER-AR.md) |
| தமிழ் (Tamil) | [PAPER-TA.md](docs/PAPER-TA.md) |
| Eesti (Estonian) | [PAPER-ET.md](docs/PAPER-ET.md) |
| हिन्दी (Hindi) | [PAPER-HI.md](docs/PAPER-HI.md) |
| Español (Spanish) | [PAPER-ES.md](docs/PAPER-ES.md) |
| Português (Portuguese) | [PAPER-PT.md](docs/PAPER-PT.md) |

These papers provide comprehensive coverage of the system architecture, core technologies, design philosophy, and case studies.

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
