# Legalis-RS TODO

## Project-Wide

### High Priority
- [x] Create a CLI tool for common operations (legalis-cli)
- [x] Add REST API server (legalis-api)
- [x] Add visualization support (legalis-viz)
- [x] Create statute registry for managing collections (legalis-registry)
- [x] Add internationalization support (legalis-i18n)
- [x] Implement cross-jurisdiction porting (legalis-porting)
- [x] Add audit trail and decision logging (legalis-audit)
- [x] Add statute diffing and change detection (legalis-diff)
- [x] Add comprehensive integration tests across all crates
- [x] Set up CI/CD pipeline (GitHub Actions with test, clippy, fmt, docs, bench jobs)
- [x] Add benchmarks for simulation engine performance (legalis-sim)
- [x] Add Linked Open Data (RDF/TTL/JSON-LD) export (legalis-lod)
- [x] Create example applications demonstrating real-world use cases (welfare-benefits example)

### Medium Priority
- [ ] Add documentation comments (rustdoc) to all public APIs
- [x] Add support for loading statutes from files (YAML, TOML, JSON) - in CLI
- [x] Implement serialization/deserialization roundtrip tests
- [x] Add tracing/logging infrastructure (tracing crate integrated)
- [ ] Create Docker container for easy deployment
- [ ] Add OpenTelemetry integration for observability

### Low Priority
- [ ] Create web-based UI frontend for legalis-api
- [ ] Add GraphQL support to legalis-api
- [ ] Create VS Code extension for Legal DSL syntax highlighting
- [ ] Add Jupyter notebook integration for legal analysis

---

## Core Layer

### legalis-core

#### Features
- [x] Add metadata support for statutes (enactment date, jurisdiction, version)
- [x] Add more condition types (date ranges, geographic regions, entity relationships)
- [x] Implement condition optimization (simplification, normalization, CNF conversion)
- [ ] Add hierarchical statute relationships (parent/child, amendments)
- [x] Create statute builder pattern with fluent API
- [x] Add temporal validity (effective dates, sunset clauses)

#### Improvements
- [ ] Make `LegalEntity` trait more flexible with dynamic attribute types
- [ ] Add validation methods for `Statute` construction
- [x] Implement `Display` trait for all types for better debugging
- [ ] Add serde feature flag for optional serialization

### legalis-dsl

#### Features
- [x] Add support for multi-statute documents (parse_statutes method)
- [x] Implement AND/OR/NOT operators in condition parsing
- [x] Add support for nested conditions with parentheses (precedence-correct parsing)
- [x] Create pretty-printer for AST back to DSL format (legalis-dsl printer module, CLI format command)
- [x] Add line/column tracking for better error messages (SourceLocation struct, SpannedToken)
- [ ] Add LSP (Language Server Protocol) support

#### Grammar Extensions
- [x] Add IMPORT statement for referencing other statutes (parse_document, ImportNode, AS alias)
- [ ] Add EXCEPTION clauses
- [ ] Add EFFECTIVE_DATE and EXPIRY_DATE clauses
- [x] Support for comments in DSL (// and /* */)
- [ ] Add AMENDMENT clause for version tracking
- [ ] Add JURISDICTION clause for locale binding

### legalis-registry

#### Features
- [x] In-memory statute storage with versioning
- [x] Tag-based organization
- [x] Jurisdiction indexing
- [ ] Persistent storage backend (SQLite, PostgreSQL)
- [ ] Full-text search across statutes
- [ ] Event sourcing for complete change history
- [ ] Multi-tenant support for isolated registries
- [ ] Import/export in standard legal document formats (Akoma Ntoso)

#### Improvements
- [ ] Add caching layer for frequently accessed statutes
- [ ] Implement optimistic concurrency control
- [ ] Add webhook notifications for statute changes

---

## Intelligence Layer

### legalis-llm

#### Providers
- [x] OpenAI provider implementation
- [x] Anthropic provider implementation
- [ ] Add Google Gemini provider implementation
- [ ] Add local LLM support (Ollama, llama.cpp)
- [ ] Add Azure OpenAI provider
- [ ] Implement provider fallback chain

#### Features
- [ ] Add streaming response support
- [ ] Implement response caching layer
- [ ] Add token usage tracking and cost estimation
- [ ] Create prompt templates for different legal domains
- [ ] Add retry logic with exponential backoff
- [ ] Implement conversation context management
- [ ] Add structured output validation against expected schema
- [ ] Add rate limiting per provider

#### Law Compiler
- [ ] Add confidence scoring for compiled statutes
- [ ] Implement iterative refinement with verification feedback
- [ ] Create domain-specific prompt engineering for civil/common law

### legalis-verifier

#### Formal Verification
- [ ] Integrate Z3 SMT solver for proper constraint solving
- [ ] Implement satisfiability checking for conditions
- [ ] Add temporal logic verification (LTL/CTL)
- [ ] Create proof generation for verification results
- [ ] Add model checking for statute interactions

#### Analysis
- [x] Basic conflict detection between statutes
- [x] Logical contradiction checking
- [ ] Implement dead code detection (unreachable statute branches)
- [x] Add complexity metrics for statutes
- [ ] Create dependency graph analysis
- [ ] Implement semantic similarity detection between statutes

#### Constitutional Checks
- [ ] Add configurable constitutional principle definitions
- [ ] Implement jurisdictional rule sets
- [ ] Add precedent-based conflict detection
- [ ] Create constitutional consistency scoring

---

## Simulation & Analysis Layer

### legalis-sim

#### Features
- [x] Population-based simulation
- [x] Async execution with Tokio
- [x] Outcome metrics collection
- [ ] Add temporal simulation (time-based state changes)
- [ ] Implement agent behavior models (compliance, evasion)
- [ ] Add support for inter-agent relationships
- [ ] Add scenario scripting support
- [ ] Create Monte Carlo simulation mode

#### Performance
- [ ] Implement parallel batch processing for large populations
- [ ] Add memory-efficient streaming for very large simulations
- [ ] Create incremental simulation (only re-simulate changed statutes)
- [ ] Add SIMD optimizations for numeric conditions

#### Analysis
- [ ] Add statistical analysis tools (distributions, correlations)
- [ ] Implement sensitivity analysis
- [ ] Create comparative analysis between statute versions
- [ ] Add A/B testing support for statute variants

### legalis-diff

#### Features
- [x] Structural diff between statutes
- [x] Change categorization (added/removed/modified)
- [x] Impact assessment with severity levels
- [ ] Semantic diff (understanding meaning changes)
- [ ] Cross-statute impact analysis
- [ ] Amendment chain visualization
- [ ] Diff output in multiple formats (JSON, HTML, Markdown)

#### Improvements
- [ ] Add three-way merge for concurrent amendments
- [ ] Implement blame-style annotation for change tracking
- [ ] Create diff templates for common legislative patterns

---

## Internationalization & Porting Layer

### legalis-i18n

#### Features
- [x] Locale support with language/country/region
- [x] Jurisdiction registry with legal system types
- [x] Cultural parameters (age of majority, religious observances)
- [x] Legal dictionary for term translation
- [x] Translation manager for multi-language support
- [ ] ICU message format support
- [ ] Plural rules handling for different languages
- [ ] Date/time localization for legal deadlines

#### Content
- [ ] Add comprehensive legal term dictionaries (EN, JA, DE, FR, ES, ZH)
- [ ] Create jurisdiction-specific legal glossaries
- [ ] Add Latin legal term translations
- [ ] Create mapping between civil and common law concepts

### legalis-porting

#### Features
- [x] Cross-jurisdiction statute translation
- [x] Cultural parameter injection
- [x] Compatibility report generation
- [x] Change tracking for ported statutes
- [ ] AI-assisted cultural adaptation suggestions
- [ ] Bilateral legal agreement templates
- [ ] Regulatory equivalence mapping

#### Improvements
- [ ] Add conflict detection with target jurisdiction laws
- [ ] Implement partial porting for statute sections
- [ ] Create porting validation with local legal experts
- [ ] Add reverse porting (target to source comparison)

---

## Output Layer

### legalis-viz

#### Formats
- [x] Mermaid flowchart generation
- [x] GraphViz DOT format
- [ ] D3.js interactive visualization
- [ ] PlantUML sequence diagrams
- [x] ASCII art for terminal output (tree and box formats)
- [ ] SVG/PNG direct rendering

#### Features
- [x] Decision tree visualization
- [x] Discretion zone highlighting
- [ ] Interactive web-based visualization
- [ ] Statute dependency graphs
- [ ] Timeline visualization for temporal statutes
- [ ] Population distribution charts from simulations

#### Improvements
- [ ] Add customizable styling/theming
- [ ] Create accessibility-compliant output
- [ ] Add annotation support for judicial notes

### legalis-chain

#### Platforms
- [x] Solidity output support
- [x] WASM target support
- [x] Ink! (Substrate) target support
- [ ] Add Vyper output support
- [ ] Implement Move (Aptos/Sui) target
- [ ] Add Cairo (StarkNet) target
- [ ] Create CosmWasm target

#### Features
- [ ] Generate comprehensive test suites alongside contracts
- [ ] Add gas optimization hints
- [ ] Implement upgrade pattern generation (proxy contracts)
- [ ] Create multi-contract generation for complex statutes
- [ ] Add oracle integration for external data feeds

#### Security
- [ ] Add automated security checks (reentrancy, overflow)
- [ ] Generate formal verification annotations (Certora, Scribble)
- [ ] Create audit report templates
- [ ] Implement slither/mythril integration for analysis

---

## Infrastructure Layer

### legalis-api

#### Features
- [x] REST API with Axum
- [x] CRUD operations for statutes
- [x] Verification endpoints
- [x] Health check endpoint
- [ ] Add GraphQL API support
- [ ] Implement WebSocket for real-time updates
- [ ] Add OpenAPI/Swagger documentation
- [ ] Create API versioning strategy (v1, v2)

#### Security
- [ ] Add authentication (JWT, API keys)
- [ ] Implement authorization (RBAC)
- [ ] Add rate limiting
- [ ] Create audit logging for API calls

#### Performance
- [ ] Add response caching (Redis)
- [ ] Implement request batching
- [ ] Add gzip compression

### legalis-cli

#### Commands
- [x] parse - Parse DSL files
- [x] verify - Verify statutes
- [x] viz - Generate visualizations
- [x] export - Export to smart contracts
- [x] serve - Start API server
- [x] init - Initialize new project
- [x] Add diff - Compare statute versions
- [x] Add port - Port statutes between jurisdictions
- [x] Add simulate - Run population simulations
- [x] Add audit - Generate audit reports
- [x] Add complexity - Analyze statute complexity

#### Features
- [ ] Add interactive REPL mode
- [x] Create shell completions (bash, zsh, fish) via `legalis completions <shell>`
- [ ] Add progress indicators for long operations
- [ ] Implement watch mode for file changes

### legalis-audit

#### Features
- [x] Audit trail with decision logging
- [x] Hash chain integrity verification
- [x] Compliance report generation
- [x] Audit record tamper detection
- [ ] Export to legal compliance formats
- [ ] Add digital signature support
- [ ] Implement timestamping authority integration
- [ ] Create chain of custody tracking

#### Storage
- [ ] Add persistent storage backend
- [ ] Implement log rotation and archival
- [ ] Create searchable audit index
- [ ] Add backup and recovery procedures

---

## Interoperability Layer

### legalis-interop

The interoperability layer enables Legalis-RS to import from and export to other legal DSL formats,
making it a universal bridge between legal technology ecosystems.

#### Supported Formats

##### Catala (Inria, France)
- [x] Catala AST parser → legalis_core::Statute
- [x] legalis_core::Statute → Catala output
- [x] Support for Catala's literate programming style
- [ ] Preserve legal article references during conversion
- [x] Handle Catala's scope and context model

##### Stipula (University of Bologna)
- [x] Stipula contract parser → legalis_core::Statute
- [x] legalis_core::Statute → Stipula output
- [x] Map party/asset model to legal entities
- [ ] Convert state machines to condition logic
- [ ] Support for temporal obligations

##### SLL / L4 (Singapore)
- [x] L4 parser → legalis_core::Statute
- [x] legalis_core::Statute → L4 output
- [x] Support for deontic logic (MUST, MAY, SHANT)
- [x] Handle rule-based reasoning model
- [ ] Convert decision tables

##### Standard Formats
- [x] Akoma Ntoso XML import/export (OASIS standard)
- [ ] LegalRuleML import/export
- [ ] LKIF (Legal Knowledge Interchange Format)
- [ ] LegalDocML support

#### CLI Integration
- [x] `legalis import --from catala input.catala_en`
- [x] `legalis import --from stipula input.stipula`
- [x] `legalis import --from l4 input.l4`
- [x] `legalis convert` command for format-to-format conversion
- [x] Auto-detect format from file extension

#### Conversion Features
- [x] Bidirectional conversion with loss reporting
- [ ] Semantic preservation validation
- [x] Metadata mapping between formats
- [ ] Batch conversion for statute collections
- [ ] Diff-aware incremental conversion

#### Quality Assurance
- [x] Round-trip conversion tests
- [ ] Semantic equivalence verification
- [ ] Coverage reports for format features
- [x] Conversion confidence scoring

---

## Documentation

- [ ] Write architecture decision records (ADRs)
- [ ] Create user guide with tutorials
- [ ] Add API reference documentation (rustdoc)
- [ ] Create contributor's guide
- [ ] Write deployment guide
- [ ] Add example applications for each use case
- [ ] Create video tutorials

---

## Research & Future Directions

### Generative Jurisprudence
- [ ] Research formal methods for legal reasoning
- [ ] Explore natural language processing improvements for legal text
- [ ] Investigate cross-jurisdictional legal mapping standards
- [ ] Study blockchain-based legal registries and DAOs
- [ ] Explore AI-assisted legal drafting tools

### Phase Roadmap
- [ ] Phase 1: The Visualizer - Municipal ordinance visualization
- [ ] Phase 2: The Debugger - Legislative DX for draft legislation
- [ ] Phase 3: Soft ODA - Legal system export between jurisdictions
- [ ] Phase 4: The Hybrid Court - Deterministic case automation

### Academic & Industry Integration
- [ ] Partner with law schools for validation
- [ ] Collaborate with legal tech companies
- [ ] Publish research papers on computational law
- [ ] Engage with standards bodies (Akoma Ntoso, LegalRuleML)
