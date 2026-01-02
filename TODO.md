# Legalis-RS TODO

## Project Status Summary

All 16 crates compile cleanly with no warnings. The project follows a "NO WARNINGS POLICY".

| Crate | Version | Status |
|-------|---------|--------|
| legalis-core | 0.2.0 | Stable |
| legalis-dsl | 0.2.0 | Stable |
| legalis-registry | 0.2.0 | Stable |
| legalis-llm | 0.2.0 | Stable |
| legalis-verifier | 0.2.0 | Stable |
| legalis-sim | 0.2.0 | Stable |
| legalis-diff | 0.2.0 | Stable |
| legalis-i18n | 0.2.0 | Stable |
| legalis-porting | 0.2.0 | Stable |
| legalis-viz | 0.2.0 | Stable |
| legalis-chain | 0.2.0 | Stable |
| legalis-lod | 0.2.0 | Stable |
| legalis-audit | 0.2.0 | Stable |
| legalis-interop | 0.2.0 | Stable |
| legalis-api | 0.2.0 | Stable |
| legalis-cli | 0.2.0 | Stable |

---

## Project-Wide

### Completed
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
- [x] Create example applications demonstrating real-world use cases
- [x] Add support for loading statutes from files (YAML, TOML, JSON)
- [x] Implement serialization/deserialization roundtrip tests
- [x] Add tracing/logging infrastructure (tracing crate integrated)
- [x] Create Docker container for easy deployment
- [x] Add OpenTelemetry integration for observability (in legalis-audit)

### In Progress
- [ ] Add documentation comments (rustdoc) to all public APIs

### Backlog
- [ ] Create web-based UI frontend for legalis-api
- [ ] Add GraphQL support to legalis-api
- [ ] Create VS Code extension for Legal DSL syntax highlighting
- [ ] Add Jupyter notebook integration for legal analysis

---

## Core Layer

### legalis-core
- [x] Add metadata support for statutes (enactment date, jurisdiction, version)
- [x] Add more condition types (date ranges, geographic regions, entity relationships)
- [x] Implement condition optimization (simplification, normalization, CNF conversion)
- [x] Create statute builder pattern with fluent API
- [x] Add temporal validity (effective dates, sunset clauses)
- [x] Make `LegalEntity` trait more flexible with dynamic attribute types
- [x] Add validation methods for `Statute` construction
- [x] Implement `Display` trait for all types
- [x] Add serde feature flag for optional serialization
- [ ] Add hierarchical statute relationships (parent/child, amendments)

### legalis-dsl
- [x] Add support for multi-statute documents
- [x] Implement AND/OR/NOT operators in condition parsing
- [x] Add support for nested conditions with parentheses
- [x] Create pretty-printer for AST back to DSL format
- [x] Add line/column tracking for better error messages
- [x] Add IMPORT statement for referencing other statutes
- [x] Add EXCEPTION clauses
- [x] Add EFFECTIVE_DATE and EXPIRY_DATE clauses
- [x] Support for comments in DSL
- [x] Add AMENDMENT clause for version tracking
- [x] Add JURISDICTION clause for locale binding
- [ ] Add LSP (Language Server Protocol) support

### legalis-registry
- [x] In-memory statute storage with versioning
- [x] Tag-based organization
- [x] Jurisdiction indexing
- [ ] Persistent storage backend (SQLite, PostgreSQL)
- [ ] Full-text search across statutes
- [ ] Event sourcing for complete change history
- [ ] Multi-tenant support for isolated registries

---

## Intelligence Layer

### legalis-llm
- [x] OpenAI provider implementation
- [x] Anthropic provider implementation
- [ ] Add Google Gemini provider implementation
- [ ] Add local LLM support (Ollama, llama.cpp)
- [ ] Add Azure OpenAI provider
- [ ] Implement provider fallback chain
- [ ] Add streaming response support
- [ ] Implement response caching layer
- [ ] Add token usage tracking and cost estimation

### legalis-verifier
- [x] Integrate Z3 SMT solver for proper constraint solving
- [x] Implement satisfiability checking for conditions
- [x] Basic conflict detection between statutes
- [x] Logical contradiction checking
- [x] Add complexity metrics for statutes
- [ ] Add temporal logic verification (LTL/CTL)
- [ ] Create proof generation for verification results
- [ ] Add model checking for statute interactions

---

## Simulation & Analysis Layer

### legalis-sim
- [x] Population-based simulation
- [x] Async execution with Tokio
- [x] Outcome metrics collection
- [ ] Add temporal simulation (time-based state changes)
- [ ] Implement agent behavior models (compliance, evasion)
- [ ] Create Monte Carlo simulation mode
- [ ] Implement parallel batch processing for large populations

### legalis-diff
- [x] Structural diff between statutes
- [x] Change categorization (added/removed/modified)
- [x] Impact assessment with severity levels
- [ ] Semantic diff (understanding meaning changes)
- [ ] Cross-statute impact analysis
- [ ] Amendment chain visualization

---

## Internationalization & Porting Layer

### legalis-i18n
- [x] Locale support with language/country/region
- [x] Jurisdiction registry with legal system types
- [x] Cultural parameters (age of majority, religious observances)
- [x] Legal dictionary for term translation
- [x] Translation manager for multi-language support
- [ ] ICU message format support
- [ ] Date/time localization for legal deadlines

### legalis-porting
- [x] Cross-jurisdiction statute translation
- [x] Cultural parameter injection
- [x] Compatibility report generation
- [x] Change tracking for ported statutes
- [x] AI-assisted cultural adaptation suggestions
- [x] Bilateral legal agreement templates
- [x] Regulatory equivalence mapping
- [x] Conflict detection with target jurisdiction laws
- [x] Partial porting for statute sections
- [x] Reverse porting (target to source comparison)
- [x] Comprehensive roadmap through v0.3.4 (see crate TODO.md)

---

## Output Layer

### legalis-viz
- [x] Mermaid flowchart generation
- [x] GraphViz DOT format
- [x] D3.js interactive visualization
- [x] PlantUML sequence diagrams
- [x] ASCII art for terminal output
- [x] SVG/PNG direct rendering
- [x] Decision tree visualization
- [x] Discretion zone highlighting
- [x] Interactive web-based visualization
- [x] Statute dependency graphs
- [x] Timeline visualization for temporal statutes
- [x] 3D visualization with VR/AR support
- [x] Accessibility (WCAG 2.1 AA compliance)
- [x] Framework wrappers (React, Vue, Angular, WordPress)
- [x] Comprehensive roadmap through v0.3.4 (see crate TODO.md)

### legalis-chain
- [x] Solidity output support
- [x] WASM target support
- [x] Ink! (Substrate) target support
- [x] Vyper output support
- [x] Move (Aptos/Sui) target
- [x] Cairo (StarkNet) target
- [x] CosmWasm target
- [x] TON (FunC) target
- [x] Teal (Algorand) target
- [x] ERC-20, ERC-721, ERC-1155 token generation
- [x] DAO & governance contracts
- [x] Cross-chain bridge contracts
- [x] Security analysis and audit report generation
- [x] Comprehensive roadmap through v0.3.9 (see crate TODO.md)

---

## Infrastructure Layer

### legalis-api
- [x] REST API with Axum
- [x] CRUD operations for statutes
- [x] Verification endpoints
- [x] Health check endpoint
- [x] OpenAPI/Swagger documentation
- [x] Authentication (JWT, API keys)
- [x] Authorization (RBAC + ReBAC)
- [x] Rate limiting
- [ ] Add GraphQL API support
- [ ] Implement WebSocket for real-time updates
- [ ] Add response caching (Redis)

### legalis-cli
- [x] Parse, verify, viz, export, serve, init commands
- [x] Diff, port, simulate, audit, complexity commands
- [x] Shell completion generation
- [x] Lint, format, watch, repl, test, publish commands
- [x] Configuration file support (legalis.toml)
- [x] Progress bars and spinners
- [x] Interactive tutorials
- [x] Plugin system (basic)
- [x] Explain, trace, benchmark, migrate, graph commands
- [x] Registry push/pull/sync/login commands
- [ ] Full plugin ecosystem with marketplace
- [ ] AI-powered CLI features
- [ ] Full-featured TUI dashboard

### legalis-audit
- [x] Audit trail with decision logging
- [x] Hash chain integrity verification
- [x] Merkle tree for efficient verification
- [x] SQLite, PostgreSQL storage backends
- [x] Encrypted storage (AES-256-GCM)
- [x] GDPR, SOX, HIPAA, CCPA, ISO 27001 compliance
- [x] Blockchain anchoring (Bitcoin, Ethereum)
- [x] SIEM integration (Syslog, CEF, LEEF)
- [x] Elasticsearch export
- [x] OpenTelemetry tracing integration
- [x] Comprehensive roadmap through v0.3.4 (see crate TODO.md)

---

## Interoperability Layer

### legalis-interop
- [x] Catala import/export
- [x] Stipula import/export
- [x] L4 import/export
- [x] Akoma Ntoso XML import/export
- [x] LegalRuleML, LKIF, LegalDocML support
- [x] BPMN, DMN, CMMN, RuleML, SBVR support
- [x] OASIS LegalCite, CEN MetaLex, MPEG-21 REL
- [x] Creative Commons, SPDX license formats
- [x] Streaming API for large documents
- [x] Async conversion APIs
- [x] Transformation pipeline with hooks
- [x] Comprehensive roadmap through v0.3.4 (see crate TODO.md)

---

## Linked Open Data Layer

### legalis-lod
- [x] RDF serialization (Turtle, N-Triples, RDF/XML, JSON-LD)
- [x] ELI vocabulary support
- [x] Dublin Core metadata
- [x] SPARQL query generation
- [x] SHACL shape generation
- [x] EUR-Lex, legislation.gov.uk integration
- [x] Knowledge graph construction
- [x] OWL 2 RL reasoning
- [x] RDF-star support
- [x] Graph algorithms (shortest path, centrality, components)
- [x] Comprehensive roadmap through v0.3.9 (see crate TODO.md)

---

## Documentation

- [ ] Write architecture decision records (ADRs)
- [ ] Create user guide with tutorials
- [ ] Add API reference documentation (rustdoc)
- [ ] Create contributor's guide
- [ ] Write deployment guide
- [x] Add example applications for each use case
- [ ] Create video tutorials

---

## Research & Future Directions

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
