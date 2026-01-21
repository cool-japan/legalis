# Legalis-RS

**The Architecture of Generative Jurisprudence**

*Governance as Code, Justice as Narrative*

[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.1.3-brightgreen.svg)](RELEASE-0.1.3.md)
[![Crates](https://img.shields.io/badge/crates-41-blue.svg)](#crates)
[![Jurisdictions](https://img.shields.io/badge/jurisdictions-18%20operational-green.svg)](#jurisdictions)
[![Tests](https://img.shields.io/badge/tests-13083%20passing-success.svg)](#crates)
[![Files](https://img.shields.io/badge/rust%20files-1651-orange.svg)](#workspace-structure)
[![Code](https://img.shields.io/badge/lines-863k-informational.svg)](#workspace-structure)

## Overview

Legalis-RS is a Rust framework for parsing, analyzing, and simulating legal statutes across **multiple jurisdictions**. It transforms natural language legal documents into structured, machine-verifiable code while preserving the essential distinction between:

- **Deterministic Logic (Code)**: Computationally derivable legal outcomes (age requirements, income thresholds, deadlines)
- **Judicial Discretion (Narrative)**: Areas requiring human interpretation and judgment

This separation is the philosophical core of Legalis-RS - it explicitly marks where AI-assisted legal processing must yield to human judgment, serving as a safeguard against algorithmic overreach in legal systems.

### Supported Legal Systems

**🌍 Global Coverage**: 18 operational jurisdictions spanning **Civil Law**, **Common Law**, **Socialist**, and **Supranational** legal traditions:

- 🇦🇪 **UAE** - Civil Law + Islamic Law (Federal Law, Commercial, Labor, PDPL, Free Zones)
- 🇦🇺 **Australia** - Common Law + Statutory (Torrens system, Fair Work Act, Consumer Law, Privacy, Immigration)
- 🇧🇷 **Brazil** - Civil Law (Civil Code, Consumer Protection, LGPD, CLT Labor Law)
- 🇨🇦 **Canada** - Common Law + Civil Law (Quebec) (Charter of Rights, Aboriginal rights, Federal-provincial)
- 🇨🇳 **China** - Socialist Civil Law (Civil Code, Contract, Corporate, Data Protection, Labor)
- 🇩🇪 **Germany** - Civil Law (BGB, GmbHG, HGB, Grundgesetz)
- 🇪🇺 **European Union** - Supranational Law (GDPR, Competition Law, Treaties, 11 languages)
- 🇫🇷 **France** - Civil Law (Code civil, Code du travail, 35-hour work week)
- 🇮🇩 **Indonesia** - Civil Law (Civil Code, Investment, Labor, Tax)
- 🇮🇳 **India** - Common Law + Statutory (Constitution, IPC/BNS, DPDP, Consumer Protection, Corporate)
- 🇯🇵 **Japan** - Civil Law + Asian Tradition (Minpō, Labor, IP Law, e-Gov integration, 16+ domains)
- 🇱🇦 **Lao PDR (Laos)** - Civil Law + Socialist Tradition (Civil Code 2020, Japanese/French influences, ODA)
- 🇸🇬 **Singapore** - Common Law + Statutory (Companies, Employment, PDPA, Banking, Payment Services)
- 🇹🇭 **Thailand** - Civil Law (Civil Code, Labor, Investment, Data Protection)
- 🇬🇧 **United Kingdom** - Common Law (Employment Rights, Consumer Rights, Financial Services)
- 🇺🇸 **United States** - Common Law (Restatement of Torts, 51 jurisdictions, Choice of Law)
- 🇻🇳 **Vietnam** - Socialist Civil Law (Civil Code, Investment, Labor, Cybersecurity)
- 🇿🇦 **South Africa** - Mixed Law (Companies Act, LRA/BCEA Labor, POPIA, BBBEE)

**Total**: 1,651 Rust files (~863k LoC), 41 workspace crates, **13,083 tests passing** across all jurisdictions.

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
│   ├── ae/                # UAE: Federal Law, Commercial, Labor, PDPL, Free Zones
│   ├── au/                # Australia: Torrens, Fair Work, Consumer, Privacy, Immigration
│   ├── br/                # Brazil: Civil Code, Consumer, LGPD, CLT Labor
│   ├── ca/                # Canada: Charter, Aboriginal rights, Federal-provincial
│   ├── cn/                # China: Civil Code, Contract, Corporate, Data Protection
│   ├── de/                # Germany: BGB, GmbHG, HGB, Grundgesetz
│   ├── eu/                # EU: GDPR (11 languages), Competition, Treaties
│   ├── fr/                # France: Code civil, Code du travail (35h week)
│   ├── id/                # Indonesia: Civil Code, Investment, Labor, Tax
│   ├── in/                # India: Constitution, IPC/BNS, DPDP, Consumer, Corporate
│   ├── jp/                # Japan: Minpō, e-Gov, APPI, 16+ domains
│   ├── la/                # Lao PDR: Civil Code 2020, Japanese/French influences
│   ├── sg/                # Singapore: Banking, Payments, PDPA, CPF
│   ├── th/                # Thailand: Civil Code, Labor, Investment, Data Protection
│   ├── uk/                # UK: Employment, Consumer, Financial Services
│   ├── us/                # USA: Restatement, 51 jurisdictions, Choice of Law
│   ├── vn/                # Vietnam: Civil Code, Investment, Labor, Cybersecurity
│   └── za/                # South Africa: Companies Act, LRA/BCEA, POPIA, BBBEE
├── examples/
│   ├── jp-constitution-3d/ # 3D visualization of Japanese Constitution
│   └── welfare-benefits/   # Welfare benefits eligibility system
├── legalis.md             # Full specification document
├── Cargo.toml             # Workspace configuration
└── README.md
```

## Code Metrics

**Project Scale (v0.1.3)**:

| Metric | Count | Details |
|--------|-------|---------|
| **Total Lines of Code** | 929,539 | Rust (863k), Python (8.6k), Markdown (50k), TypeScript (1.7k) |
| **Rust Code Lines** | 863,282 | 719,506 executable + 26,553 comments + 117,223 blanks |
| **Documentation Lines** | 150,360 | Inline Rust documentation (/// and //!) |
| **Rust Files** | 1,651 | Across 65 workspace crates |
| **Test Functions** | 13,083 | Unit tests + async tests + property tests |
| **Workspace Crates** | 65 | 17 core + 18 jurisdictions + 30 examples |
| **Jurisdictions** | 18 | AE, AU, BR, CA, CN, DE, EU, FR, ID, IN, JP, LA, SG, TH, UK, US, VN, ZA |
| **Supported Languages** | 60+ | Multi-language i18n support |
| **Documentation Ratio** | 17.4% | Comments to code ratio (industry average: 10-20%) |
| **Zero Warnings** | ✅ | Strict clippy compliance enforced |

**Testing Infrastructure**:
- **Unit Tests**: 13,083 passing tests
- **Integration Tests**: 22+ test files
- **Benchmark Suites**: 13 Criterion benchmarks
- **Property-Based Tests**: 10+ tests across multiple crates
- **Fuzzing Targets**: 3 libFuzzer-based fuzz targets
- **Doc Tests**: 343 passing

**Edition**: Rust 2024 | **MSRV**: 1.86

## Crates

All 65 workspace crates (17 core + 18 jurisdictions + 30 examples) compile cleanly with **NO WARNINGS** - strict quality policy enforced.

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
| `legalis-verifier` | 0.2.9 | 392 | Static analysis with OxiZ SMT solver (Pure Rust), temporal logic (LTL/CTL), formal methods (Coq, Lean 4), distributed verification. |

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

Legalis-RS now includes comprehensive support for **10 major jurisdictions** (all fully implemented):

| Jurisdiction | Status | Files | Tests | Description |
|--------------|--------|-------|-------|-------------|
| **`au`** (Australia) | ✅ Complete | 39 | 168 | **Common Law + Statutory**: Commonwealth Constitution with implied rights, Contract Law (ACL), Corporate Law (Corporations Act), Criminal Law, Employment Law (Fair Work Act), Family Law, Property Law (Torrens system, Native Title), Tort Law (defamation, negligence). |
| **`ca`** (Canada) | ✅ Complete | 47 | 238 | **Common Law + Civil Law (Quebec)**: Canadian Charter of Rights and Freedoms, Federal-provincial division of powers, Contract Law, Corporate Law (oppression remedy), Criminal Code, Employment Law, Family Law (child/spousal support), Property Law (Aboriginal rights), Tort Law (occupiers' liability). |
| **`de`** (Germany) | ✅ Complete | 76 | 318 | **Civil Law System**: BGB 5-book structure (Erbrecht, Familienrecht, Sachenrecht, Schuldrecht, Unerlaubte Handlungen §823-826), GmbHG, HGB, AKTG, Arbeitsrecht, Grundgesetz. **22 examples**, 2 test suites. |
| **`eu`** (European Union) | ✅ Complete | 80 | 240 | **Supranational Law**: **GDPR (196 tests, 11 languages)**, Consumer Rights Directive, Competition Law (Articles 101-102 TFEU), Charter of Fundamental Rights, Four Freedoms, Treaty Framework. EUR-Lex/CELEX citation system. **25 examples**. |
| **`fr`** (France) | ✅ Complete | 76 | 545 | **Civil Law System**: Code civil (Napoleonic Code 1804, 2016 reform), Code de commerce (SA/SARL/SAS), **Code du travail (35-hour work week, SMIC)**, Constitution de 1958. **11 domains (highest coverage), 69.7% documentation ratio (industry-leading), 524 tests passing**. |
| **`jp`** (Japan) | ✅ Complete | 119 | 440 | **Civil Law + Asian Tradition**: **16+ domains (most extensive)** - 民法 (Minpō 709-715), 商法/会社法, 労働基準法, 知的財産法, 消費者保護法, 個人情報保護法 (APPI), 建設業法, 行政手続法 (**e-Gov integration**), 環境法. **398 tests, 27,600+ lines, 7 test suites, 10 examples**. Era system (元号), contract risk analysis. |
| **`la`** (Lao PDR) | ✅ Complete | 8 | 49 | **Civil Law + Socialist Tradition**: **Civil Code 2020 (Law No. 66/NA, 1087 articles, 6 books)** - General Provisions, Property, Obligations, Family, Inheritance. **Japanese/French influences**, JICA ODA legal assistance documentation, comparative law analysis (比較法学), legal transplantation research. **Bilingual (Lao/English)**, ODA program evaluation. |
| **`sg`** (Singapore) | ✅ Complete | 56 | 211 | **Common Law + Statutory**: Companies Act (ACRA), Employment Act (CPF), PDPA 2012 (DPO, DNC), Consumer Protection, IP Laws, **Banking Act (Basel III CAR)**, **Payment Services Act 2019 (DPT/Crypto, 7 service types)**. **2 test suites, 10 examples, trilingual (EN/中文/Melayu)**. |
| **`uk`** (United Kingdom) | ✅ Complete | 127 | 646 | **Common Law**: Employment Rights Act 1996 (unfair dismissal, redundancy), UK GDPR + DPA 2018, Consumer Rights Act 2015 (tiered remedies), Contract Law (common law precedents), Companies Act 2006, Financial Services (AML/CFT, MiFID2, Payment Services). **Most RS files (127), 10-11 domains**. |
| **`us`** (United States) | ✅ Complete | 98 | 473 | **Common Law System**: Restatement of Torts (ALI §158, §46, §402A), **51 jurisdictions** (50 states + DC) with state-specific variations, Choice of Law (5 approaches), Uniform Acts (UCC/UPA), Federal-State boundary analysis, Professional licensing (UBE, IMLC, NCARB), Tax variations, Cannabis/Privacy tracking. **436 tests, 18,700+ lines**. |

**Total**: 1,344 Rust files (726 jurisdiction files), 43 workspace crates, **3,328 jurisdiction tests** (~198k LoC across 10 operational jurisdictions)

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

# Build all crates (default features: includes REST + gRPC APIs)
cargo build

# Build without gRPC (minimal dependencies)
cargo build --no-default-features

# Run tests
cargo test

# Check for issues
cargo clippy
```

#### Building with SMT Solver (Optional)

The `legalis-verifier` crate has an optional `smt-solver` feature for rigorous formal verification using **OxiZ** (Pure Rust SMT solver):

```bash
# Build with SMT solver (Pure Rust - no external dependencies)
cargo build --features smt-solver

# Build with all features
cargo build --all-features

# Run tests with all features
cargo nextest run --all-features
```

**Note**: OxiZ is a Pure Rust SMT solver, so no external libraries or environment variables are needed!

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
4. **SMT Solver Integration**: Uses OxiZ (Pure Rust) for formal verification of legal consistency
   - **Achieved**: 100% Pure Rust implementation with no external C/C++ dependencies
5. **Object Storage**: S3-compatible storage support for audit trails
   - **Note**: MinIO can be replaced with rs3gw (Pure Rust S3-compatible gateway) from the COOLJAPAN Ecosystem

## Documentation

### Release Notes
- **[v0.1.3](RELEASE-0.1.3.md)** (January 21, 2026) - Global Expansion: 11 new jurisdictions (AE, AU, BR, CN, ID, IN, TH, VN, ZA + more)
- **[v0.1.2](RELEASE-0.1.2.md)** (January 15, 2026) - Code Quality: Clippy Compliance
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
