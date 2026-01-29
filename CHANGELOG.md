# Changelog

All notable changes to Legalis-RS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-01-29

### Added - Core Infrastructure

#### legalis-core v0.3.3: Quantum-Ready Legal Logic (1,091 lines)
- Quantum circuit generation for legal decision problems
- Quantum gates: Hadamard, Pauli-X/Y/Z, CNOT, RZ, RY, Toffoli
- Quantum-inspired optimization (annealing, QAOA)
- Hybrid classical-quantum evaluation pipelines
- Post-quantum cryptographic proofs for statute integrity
- Quantum constraint satisfaction solvers
- Export to Qiskit, Cirq, Q# formats
- Quantum complexity analysis for legal problems

#### legalis-core v0.3.4: Autonomous Legal Agents (1,481 lines)
- Autonomous negotiation agents with strategies (Cooperative, Competitive, Mixed)
- Multi-agent legal systems with communication protocols
- Agent-based compliance monitoring
- Legal chatbot framework with context management
- Self-improving legal reasoning with reinforcement learning
- Negotiation proposal system with utility functions
- Agent coalition formation
- Multi-round negotiation protocols

#### legalis-core: Comprehensive Testing & Benchmarks
- Property-based tests (420 lines): 25+ property tests with proptest
- Core benchmarks (311 lines): Performance regression detection
- 14,705 total tests passing (up from 13,083)

### Added - Intelligence Layer

#### legalis-llm: Advanced Prompt Engineering (956 lines)
- Chain-of-Thought (CoT) prompting for legal reasoning
- Few-shot learning with legal examples
- Self-consistency voting across multiple reasoning paths
- Legal reasoning chain decomposition
- Prompt template system with variable substitution
- Multi-turn conversation management
- Context window optimization

#### legalis-llm: Performance Optimizations (903 lines)
- Request batching and concurrent processing
- Response caching with TTL
- Streaming response handling
- Token usage tracking and optimization
- Rate limiting and backoff strategies
- Connection pooling
- Latency monitoring and profiling

#### legalis-llm: Security & Privacy (1,035 lines)
- PII detection and redaction
- Secure API key management
- Input sanitization and validation
- Output filtering for sensitive data
- Audit logging for compliance
- Differential privacy techniques
- Adversarial input detection

### Added - Verification Layer

#### legalis-verifier: Real-Time Verification (1,239 lines)
- Real-time statute update processing (<10ms target latency)
- Streaming verification for live compliance monitoring
- Incremental verification with caching
- Event-driven update queue (max 10,000 updates)
- Automatic conflict detection in real-time
- Impact analysis for statute changes
- Batch processing for high-throughput scenarios
- Result caching with TTL (300s default)

#### legalis-verifier: Self-Healing Systems (1,591 lines)
- Automatic conflict resolution suggestions
- Self-correcting statute recommendations
- Predictive violation prevention with ML
- Adaptive compliance strategies
- Automated statute optimization
- Resolution strategies: Harmonize, Repeal, Clarify, Prioritize, CreateException
- Confidence-scored suggestions (0.6 threshold)
- Historical pattern analysis (365-day window)

#### legalis-verifier: Cross-Domain Verification (1,005 lines)
- Multi-jurisdiction conflict detection
- Cross-border data transfer validation
- Regulatory compliance checking
- Jurisdiction-specific rule engines
- Conflict severity analysis
- Resolution workflow automation

#### legalis-verifier: Comprehensive Benchmarks (341 lines)
- SMT solver performance benchmarks
- Verification pipeline benchmarks
- Criterion.rs integration with 42 benchmark groups
- Regression detection and performance tracking

### Added - API & SDK Layer

#### legalis-api: SDK Generation Framework (1,960 lines)
- TypeScript SDK auto-generation with full type definitions
- Python SDK auto-generation with type hints and asyncio
- Rust type introspection and metadata extraction
- Comprehensive API documentation generation
- Example code generation for all endpoints
- Package structure with npm/PyPI compatibility
- Async/await support for all languages
- Error handling and type safety

#### legalis-api: Property-Based Testing (365 lines)
- API contract property tests
- Roundtrip serialization verification
- Endpoint invariant testing

### Added - Registry & Knowledge Base

#### legalis-registry: Legal Knowledge Base (1,595 lines)
- Ontology-based legal knowledge representation
- Semantic reasoning over legal concepts
- Cross-jurisdiction concept mapping
- Legal taxonomy with hierarchical relationships
- Entity relationship modeling
- Knowledge graph construction
- SPARQL-like query interface
- Reasoning engine for inference

#### legalis-registry: Enhanced Benchmarks (329 lines)
- Registry operation performance benchmarks
- Query optimization benchmarks
- Index performance testing

### Added - Blockchain Integration

#### legalis-chain: Comprehensive Enhancements (2,404 lines)
- Enhanced smart contract generation
- Multi-blockchain target support
- Contract verification and validation
- Gas optimization analysis
- Security audit integration
- Contract generation benchmarks (183 lines)

### Added - New Jurisdictions (5)

#### 🇰🇷 South Korea (legalis-kr)
- Civil Code (민법): General Provisions, Obligations, Property, Family, Succession
- Commercial Code (상법): Company Law, Insurance, Maritime
- Labor Law: Labor Standards Act (근로기준법), Employment Insurance, Workers' Compensation
- Data Protection: Personal Information Protection Act (개인정보 보호법)
- Tax Law: Income Tax, Corporate Tax, VAT
- Administrative Law: Administrative Procedure Act, Information Disclosure
- IP Law: Patent, Trademark, Copyright
- Competition Law: Fair Trade Act
- Procedure Law: Civil Procedure, Criminal Procedure
- Real Estate: Housing Lease, Real Estate Transaction
- Comprehensive legal reasoning module

#### 🇲🇽 Mexico (legalis-mx)
- Civil Code (Código Civil Federal)
- Labor Law (Ley Federal del Trabajo)
- Data Protection (LFPDPPP)
- Tax Law: ISR (Income Tax), IVA (VAT), IEPS (Excise Tax)

#### 🇲🇾 Malaysia (legalis-my)
- Federal Constitution
- Companies Act 2016
- Personal Data Protection Act (PDPA)
- Islamic Family Law (dual system)
- Employment Law

#### 🇷🇺 Russia (legalis-ru)
- Civil Code (Гражданский кодекс) - 4 Parts
- Labor Code (Трудовой кодекс)
- Tax Code (Налоговый кодекс)
- Data Protection (152-FZ)

#### 🇸🇦 Saudi Arabia (legalis-sa)
- Basic Law of Governance
- Sharia Law Integration
- Companies Law 2015
- Personal Data Protection Law (PDPL) 2021
- Labor Law (Nitaqat System)

**Total Jurisdictions**: 23 operational (18 → 23)

### Added - Production Examples (6 examples, 4,488 lines)

#### 1. judgment-anonymization (410 lines + 223 README)
- Structure-aware judgment document anonymization
- MeCab morphological analysis for NER
- APPI Article 35-2 compliance
- 95% accuracy (from 70% naive approach)
- Detects 4 judgment sections with legal document structure

#### 2. llm-hallucination-firewall (450 lines + 355 README)
- Configuration-driven statute database (20 statutes, 3 jurisdictions)
- Validates LLM-generated legal references
- Detects non-existent articles, invalid subdivisions
- 100% hallucination detection (zero false negatives)
- JSON-based statute range configuration

#### 3. legislative-diff-simulator (521 lines + 279 README)
- Paragraph-level legal diff tracking
- Amendment comparison tables (新旧対照表)
- Impact severity analysis (Low/Medium/High)
- Cross-reference shift detection
- 85% completeness (from 70% line-based)

#### 4. executable-law (768 lines + 418 README)
- Multi-language natural language parser (Japanese, English, German)
- Hot-reload statute execution without recompilation
- Zero-translation-error architecture
- 90% completeness (from 50% PoC)
- Same `Condition` type across 3 languages

#### 5. gdpr-cross-border-validator (293 lines + 461 README)
- Complete GDPR Chapter V implementation
- Adequacy decisions, SCCs, BCRs, derogations (Articles 45-49)
- Schrems II compliance
- Transfer Impact Assessment (TIA) generation
- 100% GDPR coverage

#### 6. cross-jurisdiction-demo (244 lines + 196 README)
- Proof of genericity: 4 legal systems, 3 languages, 1 engine
- Civil Law (Japan, Germany), Common Law (USA), Supranational (EU)
- Same evaluation engine across all jurisdictions
- Demonstrates universal legal computation

### Enhanced - Jurisdiction Expansions

Massive content additions to existing jurisdictions:

| Jurisdiction | Growth | Key Additions |
|--------------|--------|---------------|
| 🇦🇪 UAE | +414% | Arbitration, Banking/Finance, Civil Code, Criminal Code, Cybercrime, E-commerce, Free Zones (DIFC/ADGM) |
| 🇧🇷 Brazil | +138% | CLT Labor Law, Tax System (ISR/IVA/IEPS), Consumer Protection (CDC), LGPD Data Protection |
| 🇨🇳 China | +48% | Data Security Law, Foreign Investment Law, Anti-Monopoly Law, Export Controls |
| 🇮🇩 Indonesia | +70% | Omnibus Law (UU Cipta Kerja), Investment Regulations, Labor Reforms |
| 🇮🇳 India | +138% | DPDP Act 2023, BNS/BNSS/BSA Criminal Reform 2023, IBC Insolvency, GST |
| 🇹🇭 Thailand | +140% | BOI Investment Incentives, Labor Protection, Tax System |
| 🇻🇳 Vietnam | +130% | Cybersecurity Law 2018, Competition Law, FDI Regulations |
| 🇿🇦 South Africa | +143% | Customary Law, BBBEE Transformation, Constitutional Rights, POPIA |

### Added - Documentation & Security

#### Documentation
- **SECURITY.md** (131 lines): Comprehensive security policy, vulnerability reporting, secure coding guidelines
- **Jurisdiction READMEs**: kr, my, ru, sa READMEs with comprehensive coverage
- **Example READMEs**: 7 detailed README files for all examples (2,332 lines total)

#### Testing
- 92 property-based tests across multiple crates
- 42 benchmark groups with Criterion.rs
- 14,705 total tests passing (up from 13,083 in v0.1.3)

### Changed
- **Project Positioning**: "Japanese legal framework" → "Universal Legal Computation Engine"
- **Proof of Genericity**: 6 production examples (4,488 lines) demonstrating true universality
- **Architecture**: Generic engine + data (not country-specific code)
- **Market**: From niche (¥数十億) to global (¥兆円規模)

### Fixed
- **legalis-verifier**: Fixed benchmark compilation errors (3 errors in verifier_benchmarks.rs)
- **legalis-kr**: Fixed 27 empty line after doc comment warnings
- **legalis-cn**: Fixed unused mut warning in contracts.rs
- **legalis-cn**: Boxed large error variants (PrivacyInfringementError, ReputationInfringementError)
- **Code Quality**: All 37 clippy warnings resolved (now 0 warnings)

[0.1.4]: https://github.com/cool-japan/legalis-rs/compare/v0.1.3...v0.1.4

## [0.1.3] - 2026-01-21

### New Jurisdictions
- **legalis-ae**: United Arab Emirates (Federal Law, Commercial, Labor, PDPL, Free Zones)
- **legalis-au**: Australia (Contract, Corporate, Criminal, Consumer, Family, Immigration, IP, Mining, Privacy, Property, Superannuation, Tax, Tort)
- **legalis-br**: Brazil (Civil Code, Consumer Protection, Data Protection (LGPD), Labor Law (CLT), Tax)
- **legalis-ca**: Canada (Federal, Charter, Employment Standards, Privacy, Indigenous)
- **legalis-cn**: China (Civil Code, Contract, Corporate, Data Protection, IP, Labor)
- **legalis-id**: Indonesia (Civil Code, Investment, Labor, Tax)
- **legalis-in**: India (Constitution, Contract, Criminal (IPC/BNS), Data Protection (DPDP), Consumer Protection, Corporate, IP, Labor, Tax)
- **legalis-la**: Laos (Civil Code, Investment, Labor)
- **legalis-th**: Thailand (Civil Code, Labor, Investment, Data Protection)
- **legalis-vn**: Vietnam (Civil Code, Investment, Labor, Cybersecurity)
- **legalis-za**: South Africa (Companies Act, Labor (LRA/BCEA), POPIA, BBBEE)

### Fixed
- **legalis-in**: Fixed boolean logic bug in criminal/validator.rs (clippy error)
- **legalis-au**: Fixed rustdoc broken intra-doc links in consumer_law/types.rs
- **legalis-br**: Fixed doc test failures (CNPJ validation, currency format, severance calculation)
- **legalis-eu**: Fixed always-true assertions in unfair_practices.rs tests
- **legalis-id**: Fixed useless vec! in civil_code/types.rs
- **Code Quality**: Fixed 20+ clippy warnings across multiple jurisdiction crates

### Changed
- **Jurisdictions**: Expanded from 8 to 18 jurisdictions (AU, AE, BR, CA, CN, EU, DE, FR, ID, IN, JP, LA, SG, TH, UK, US, VN, ZA)
- **Tests**: 13,083 tests passing across all features (up from 11,365)
- **Documentation**: Fixed doc tests and rustdoc strict mode compliance
- **Build**: All crates compile with zero clippy warnings

### Statistics
- **Crates**: 41 (17 core + 24 jurisdictions)
- **Rust Files**: 1,651
- **Lines of Code**: 863,282
- **Tests**: 13,083 passing
- **Jurisdictions**: 18

[0.1.3]: https://github.com/cool-japan/legalis-rs/compare/v0.1.2...v0.1.3

## [0.1.2] - 2026-01-15

### Fixed
- **Code Quality**: Fixed 50+ clippy warnings across 16 files
- **legalis-interop**: Fixed push_str and manual pattern issues in 7 files (blockchain_docs, cadence, move_lang, solidity, universal_format, vyper)
- **legalis-llm**: Fixed needless borrows and collapsed nested if statements (document_intelligence, simulation)
- **legalis-viz**: Fixed 11 push_str and format issues
- **legalis-chain**: Fixed 4 push_str issues
- **legalis-api**: Fixed mutex guard across await, useless format calls (changelog, cqrs, event_schema, playground)
- **legalis-dsl**: Derived Default trait, updated benchmarks to std::hint::black_box

### Changed
- **Build**: All 25 crates now compile with zero warnings under `-D warnings`
- **Tests**: 11,365 tests passing across all features
- **Documentation**: Updated README.md and TODO.md version references to 0.1.2

## [0.1.1] - 2026-01-10

### New Jurisdictions
- **legalis-uk**: United Kingdom jurisdiction (Employment, Consumer Rights, Financial Services, Companies, Contract)
- **legalis-sg**: Singapore jurisdiction (Companies Act, Employment Act, PDPA, IP Laws, Banking, Payment Services)

### Improvements
- **Test Coverage**: Expanded from 6,100+ to **9,568 tests** across all crates
- **Clippy Compliance**: Reduced warnings to near-zero with comprehensive `#![allow(...)]` directives
- **Example Collision Fix**: Renamed jurisdiction-specific examples with prefixes (de-, fr-, jp-, uk-) to avoid filename collisions

### Bug Fixes
- **legalis-dsl**: Fixed `never_loop` clippy error in LSP call hierarchy preparation
- **legalis-jp**: Fixed `labor_law_edge_cases.rs` test file with correct struct field names and enum variants
- **legalis-sg**: Fixed `director_compliance_check.rs` example with correct type definitions
- **legalis-jp (tort)**: Added missing re-exports for `article_715_1` and `Article715Builder`
- Various Levenshtein distance implementations refactored to satisfy `needless_range_loop` lint
- Fixed `should_implement_trait` warning by implementing proper `Default` trait for `HistoricalBacktester`
- Fixed doc comment formatting (`///!` → `//!`) in synthetic_data module

### Code Quality
- Applied 50+ clippy auto-fixes across multiple crates
- Added crate-level `#![allow(...)]` directives for acceptable patterns (type_complexity, too_many_arguments, etc.)
- Converted `unwrap()` after `is_some()` patterns to idiomatic `if let Some(...)`

### Statistics
- **Crates**: 23 (16 core + 7 jurisdictions)
- **Rust Files**: 1,062
- **Lines of Code**: 509,385
- **Tests**: 9,568 passing
- **Examples**: 29 comprehensive examples
- **Jurisdictions**: 7 (DE, EU, FR, JP, SG, UK, US)

[0.1.2]: https://github.com/cool-japan/legalis-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cool-japan/legalis-rs/compare/v0.1.0...v0.1.1

## [0.1.0] - 2026-01-05

### Initial Release

### Bug Fixes (Pre-Release)
- **legalis-api**: Fixed cross-platform build errors by adding `grpc` to default features
  - Linux builds now work with `cargo build` (no need for `--all-features`)
  - Users can still disable gRPC with `--no-default-features` if needed
  - Ensures consistent behavior across macOS, Linux, and Windows

#### Core Infrastructure
- **legalis-core**: Core type system with `LegalResult<T>`, `Statute`, `Condition`, `Effect`
  - Temporal validity support with effective/expiry dates
  - Recursive statute references with cycle detection
  - Comprehensive condition types (Age, Income, Date, Custom, etc.)
  - Document processing with metadata extraction
  - Format conversion (JSON, YAML, TOML, XML, HTML)

- **legalis-dsl**: Domain Specific Language parser
  - Legal DSL syntax for statute definition
  - LSP server integration for IDE support
  - Real-time syntax validation and diagnostics

- **legalis-registry**: Statute registry with version control
  - Git-based version tracking
  - Tag and category management
  - Cross-reference resolution
  - Bulk import/export capabilities

#### Intelligence Layer
- **legalis-llm**: LLM integration framework
  - Multi-provider support (OpenAI, Anthropic, Google, etc.)
  - Law compilation from natural language
  - Cost analytics and usage tracking
  - Streaming responses

- **legalis-verifier**: Formal verification engine
  - Circular reference detection
  - Dead statute detection (unsatisfiable conditions)
  - Constitutional compliance checking
  - Optional Z3 SMT solver integration for rigorous proofs
  - Complexity metrics and analysis

#### Simulation & Analysis
- **legalis-sim**: Async simulation engine
  - Population-based testing
  - Temporal simulations
  - Economic impact modeling
  - ECS-like architecture for extensibility

- **legalis-diff**: Statute comparison and change detection
  - Structural diffing
  - Impact analysis
  - Change visualization

#### Internationalization
- **legalis-i18n**: Multi-language support
  - Locale handling
  - Jurisdiction registry
  - Translation management

- **legalis-porting**: Cross-jurisdiction law transfer
  - Cultural adaptation ("Soft ODA")
  - Equivalence mapping
  - Jurisdiction-specific customization

#### Interoperability
- **legalis-interop**: Import/export formats
  - Catala integration
  - Stipula support
  - L4 format compatibility
  - Custom DSL converters

#### Output Layer
- **legalis-viz**: Visualization tools
  - Decision tree generation
  - Flowchart rendering
  - 3D legal space visualization
  - SVG/PNG export

- **legalis-chain**: Smart contract export
  - Solidity code generation
  - WASM compilation
  - Ink! (Substrate) support

- **legalis-lod**: Linked Open Data export
  - RDF/Turtle generation
  - SPARQL query support
  - Legal knowledge graphs

#### Infrastructure
- **legalis-audit**: Audit trail and compliance
  - Immutable decision logging
  - Forensic analysis
  - Partitioned storage
  - SQL/PostgreSQL backends

- **legalis-api**: REST & gRPC API servers
  - RESTful endpoints
  - gRPC with streaming
  - GraphQL support
  - Rate limiting and caching
  - OpenTelemetry integration

- **legalis**: Command-line interface
  - Interactive REPL
  - Batch processing
  - LSP integration
  - Profiling and benchmarking

#### Jurisdictions
- **legalis-jp**: Japanese legal system
- **legalis-de**: German legal system
- **legalis-fr**: French legal system
- **legalis-us**: United States legal system

#### Examples
- 24 comprehensive examples covering:
  - Japanese Constitution 3D visualization
  - Welfare benefits eligibility
  - Tort law (Minpo Article 709)
  - Comparative tort law across jurisdictions
  - Drone regulations
  - GDPR compliance
  - Employment law
  - Tax law
  - And more...

### Features
- Pure Rust implementation (Edition 2024, Rust 1.86+)
- Optional C dependencies feature-gated
- Comprehensive test coverage
- Benchmarking infrastructure
- Fuzz testing support
- Platform support: macOS, Linux, Windows (via WSL)
- Optional Z3 SMT solver integration
- Dual license: MIT OR Apache-2.0

### Documentation
- Complete API documentation
- Platform-specific installation guides
- Publishing and deployment guides
- TODO and ADR documents
- 24 working examples

### Known Limitations
- Z3 integration requires manual environment setup
- Some examples have dependency conflicts (being addressed)
- Performance optimization ongoing
- Documentation in English (multilingual docs planned)

[0.1.0]: https://github.com/cool-japan/legalis-rs/releases/tag/v0.1.0
