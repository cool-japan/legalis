# Changelog

All notable changes to Legalis-RS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
