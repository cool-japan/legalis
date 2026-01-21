# Legalis-RS TODO

## Project Status Summary (v0.1.3)

All 65 crates (17 core + 18 jurisdictions + 30 examples) compile cleanly with no warnings. The project follows a "NO WARNINGS POLICY".

**Statistics**: 1,651 Rust files | 863k LoC | 13,083 tests passing | 18 jurisdictions

### Core Crates (16)

| Crate | Version | Status | Tests |
|-------|---------|--------|-------|
| legalis-core | 0.3.0 | Stable | 631 passing |
| legalis-dsl | 0.2.0 | Stable | 453 passing |
| legalis-registry | 0.2.9 | Stable | 680 passing |
| legalis-llm | 0.4.8 | Stable | 543 passing |
| legalis-verifier | 0.2.9 | Stable | 392 passing |
| legalis-sim | 0.2.9 | Stable | 643 passing |
| legalis-diff | 0.4.0 | Stable | Passing |
| legalis-i18n | 0.3.4 | Stable | 584 passing |
| legalis-porting | 0.3.0 | Stable | 298 passing |
| legalis-viz | 0.2.0 | Stable | 453 passing |
| legalis-chain | 0.3.7 | Stable | Passing |
| legalis-lod | 0.3.9 | Stable | 799 passing |
| legalis-audit | 0.2.4 | Stable | 529 passing |
| legalis-interop | 0.2.8 | Stable | 465 passing |
| legalis-api | 0.2.3 | Stable | 200 passing |
| legalis (CLI) | 0.2.3 | Stable | Passing |

### Jurisdiction Crates (18)

| Crate | Version | Status | Tests |
|-------|---------|--------|-------|
| legalis-ae | 0.1.3 | Stable | Passing |
| legalis-au | 0.1.3 | Stable | Passing |
| legalis-br | 0.1.3 | Stable | Passing |
| legalis-ca | 0.1.3 | Stable | Passing |
| legalis-cn | 0.1.3 | Stable | Passing |
| legalis-de | 0.1.3 | Stable | Passing |
| legalis-eu | 0.1.3 | Stable | Passing |
| legalis-fr | 0.1.3 | Stable | Passing |
| legalis-id | 0.1.3 | Stable | Passing |
| legalis-in | 0.1.3 | Stable | Passing |
| legalis-jp | 0.1.3 | Stable | Passing |
| legalis-la | 0.1.3 | Stable | Passing |
| legalis-sg | 0.1.3 | Stable | Passing |
| legalis-th | 0.1.3 | Stable | Passing |
| legalis-uk | 0.1.3 | Stable | Passing |
| legalis-us | 0.1.3 | Stable | Passing |
| legalis-vn | 0.1.3 | Stable | Passing |
| legalis-za | 0.1.3 | Stable | Passing |

### Jurisdiction Integration Improvements

The jurisdiction crates now utilize `legalis-core`, `legalis-verifier`, `legalis-i18n`, and `legalis-audit` for consistent API and rule engine integration.

**Current framework usage (updated 2026-01-11):**
| Jurisdiction | legalis-core | legalis-verifier | legalis-i18n | legalis-audit | legalis-sim | legalis-interop | legalis-lod | Status |
|--------------|--------------|------------------|--------------|---------------|-------------|-----------------|-------------|--------|
| legalis-fr | 26+ uses | ✅ verifier.rs | ✅ | - | - | ✅ Catala | - | Full integration |
| legalis-us | 11+ uses | ✅ verifier.rs | - | - | ✅ FLSA sim | - | - | Full integration |
| legalis-eu | 9+ uses | ✅ verifier.rs | ✅ | ✅ audit.rs | ✅ GDPR sim | - | ✅ EUR-Lex | Full integration |
| legalis-jp | 5+ uses | ✅ verifier.rs | ✅ dates.rs | - | ✅ labor sim | - | - | Full integration |
| legalis-de | 4+ uses | ✅ verifier.rs | ✅ dates.rs | - | ✅ Arbeit sim | - | - | Full integration |
| legalis-sg | 3+ uses | ✅ verifier.rs | ✅ dates.rs | - | - | ✅ L4 | - | Full integration |
| legalis-uk | 3+ uses | ✅ verifier.rs | ✅ dates.rs | - | ✅ NLW sim | - | ✅ leg.gov.uk | Full integration |
| legalis-ca | 5+ uses | ✅ verifier.rs | - | - | - | - | - | Full integration |
| legalis-au | 5+ uses | ✅ verifier.rs | - | - | - | - | - | Full integration |

**Completed integrations:**
- [x] **All jurisdictions**: reasoning/ module with Statute-based legal analysis
- [x] **All jurisdictions**: Constitutional/fundamental rights verification (verifier.rs)
- [x] **All jurisdictions**: Legal hierarchy checking (lex specialis, federal preemption, etc.)
- [x] **JP**: Japanese calendar (和暦) with holiday-aware working days calculation
- [x] **DE**: German federal state holidays (Bundesländer) with working days calculation
- [x] **SG**: Singapore calendar with multi-ethnic holiday support
- [x] **UK**: UK timezone (GMT/BST) with bank holiday calculation
- [x] **EU**: GDPR audit trail (Articles 15, 22, 5(2)) with legalis-audit integration
- [x] **US**: FLSA minimum wage/overtime impact simulation with legalis-sim
- [x] **EU**: GDPR fine/compliance impact simulation with legalis-sim
- [x] **FR**: Catala DSL integration with legalis-interop (Code Civil, Code Travail, CGI)
- [x] **SG**: L4 DSL integration with legalis-interop (Companies Act, Employment Act, PDPA)
- [x] **JP**: Labor law simulation with legalis-sim (minimum wage, work style reform, paid leave)
- [x] **DE**: Arbeitsrecht simulation with legalis-sim (Mindestlohn, ArbZG, BUrlG)
- [x] **UK**: Employment law simulation with legalis-sim (NLW, WTR, annual leave)
- [x] **EU**: EUR-Lex integration with legalis-lod (GDPR, TFEU, Consumer Rights RDF export)
- [x] **UK**: legislation.gov.uk integration with legalis-lod (ERA, NMWA, WTR RDF export)

**Recommended future improvements:**
- [ ] **All jurisdictions**: Integrate with `legalis-dsl` for rule definitions

**Benefits achieved:**
1. Unified rule engine evaluation across all jurisdictions
2. Constitutional/fundamental rights compliance checking
3. Jurisdiction-specific calendar and deadline calculations
4. GDPR-compliant audit trail for EU jurisdiction
5. Better interoperability with legalis-porting
6. Policy impact simulation for US (FLSA), EU (GDPR), JP (Labor Law), DE (Arbeitsrecht), UK (Employment)
7. Cross-DSL interoperability for FR (Catala) and SG (L4)
8. Linked Open Data export for EU (EUR-Lex) and UK (legislation.gov.uk)

---

## Project-Wide

### Completed
- [x] Create a CLI tool for common operations (legalis)
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
- [x] Create web-based UI frontend for legalis-api (completed - web-ui/public/index.html)
- [x] Add GraphQL support to legalis-api (implemented in graphql.rs)
- [x] Create VS Code extension for Legal DSL syntax highlighting (completed - vscode-extension/)
- [x] Add Jupyter notebook integration for legal analysis (completed - notebooks/legal_analysis_demo.ipynb)

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
- [x] Add hierarchical statute relationships (parent/child, amendments)
- [x] Implement case law database with precedents
- [x] Add TypedEntity with TypedAttributes system
- [x] Property-based testing with proptest (15 tests)
- [x] JsonSchema support with `schema` feature flag
- [x] PartialOrd/Ord/Hash for all enum types
- [x] Comprehensive roadmap through v0.3.0 (see crate TODO.md)

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
- [x] Add LSP (Language Server Protocol) support
- [x] Syntax highlighting (VSCode, vim)
- [x] REPL for interactive parsing
- [x] Error recovery and "did you mean?" suggestions
- [x] AST optimization passes (flatten, deduplicate, simplify)
- [x] AST diffing for change detection
- [x] Incremental parsing support
- [x] Fuzzing for parser robustness
- [x] Code actions and document formatting
- [x] AI-assisted authoring features (v0.2.1)
- [x] Comprehensive roadmap through v0.2.0 (see crate TODO.md)

### legalis-registry
- [x] In-memory statute storage with versioning
- [x] Tag-based organization
- [x] Jurisdiction indexing
- [x] Persistent storage backend (SQLite, PostgreSQL)
- [x] Full-text search across statutes
- [x] Event sourcing for complete change history
- [x] Multi-tenant support for isolated registries
- [x] GraphQL interface with async API
- [x] Distributed registry with Raft consensus (v0.2.0)
- [x] Vector search & embeddings with HNSW (v0.2.1)
- [x] Blockchain integration (Ethereum, Bitcoin) (v0.2.2)
- [x] Graph database backend with Neo4j (v0.2.3)
- [x] Multi-tenant architecture with isolation (v0.2.4)
- [x] AI-powered features (summaries, tagging, recommendations) (v0.2.5)
- [x] Event sourcing 2.0 with time-travel queries (v0.2.6)
- [x] Federation protocol for multi-registry (v0.2.7)
- [x] Real-time collaboration with WebSocket (v0.2.8)
- [x] Enterprise security (LDAP, SSO, HSM) (v0.2.9)
- [x] Comprehensive roadmap through v0.2.9 (see crate TODO.md)

---

## Intelligence Layer

### legalis-llm
- [x] OpenAI provider implementation
- [x] Anthropic provider implementation
- [x] Add Google Gemini provider implementation
- [x] Add local LLM support (Ollama, llama.cpp)
- [x] Add Azure OpenAI provider
- [x] Implement provider fallback chain
- [x] Add streaming response support
- [x] Implement response caching layer
- [x] Add token usage tracking and cost estimation
- [x] Mistral AI and HuggingFace providers
- [x] Semantic caching with disk persistence
- [x] Circuit breaker pattern with health checks
- [x] Prompt template system with versioning
- [x] A/B testing for prompts
- [x] Federated learning (v0.4.0)
- [x] Legal ontology integration (v0.4.1)
- [x] Causal reasoning (v0.4.2)
- [x] Adversarial robustness (v0.4.3)
- [x] Meta-prompting (v0.4.4)
- [x] Legal citation networks (v0.4.5)
- [x] Temporal legal reasoning (v0.4.6)
- [x] Cross-domain transfer learning (v0.4.7)
- [x] Neuro-symbolic integration (v0.4.8)
- [x] Comprehensive roadmap through v0.4.8 (see crate TODO.md)

### legalis-verifier
- [x] Integrate OxiZ SMT solver (Pure Rust) for proper constraint solving
- [x] Implement satisfiability checking for conditions
- [x] Basic conflict detection between statutes
- [x] Logical contradiction checking
- [x] Add complexity metrics for statutes
- [x] Add temporal logic verification (LTL/CTL)
- [x] Create proof generation for verification results
- [x] Add model checking for statute interactions
- [x] JSON, HTML, SARIF report formats
- [x] Constitutional principles checking
- [x] Multi-party verification (v0.2.1)
- [x] Probabilistic verification with Markov chains (v0.2.2)
- [x] Explainable verification (v0.2.3)
- [x] Privacy-preserving verification with ZK proofs (v0.2.4)
- [x] Incremental verification 2.0 (v0.2.5)
- [x] Formal methods integration (Coq, Lean 4, Isabelle) (v0.2.6)
- [x] Machine learning verification (v0.2.7)
- [x] Distributed verification (v0.2.8)
- [x] Certification framework (ISO 27001, SOC 2, GDPR) (v0.2.9)
- [x] Comprehensive roadmap through v0.2.9 (see crate TODO.md)

---

## Simulation & Analysis Layer

### legalis-sim
- [x] Population-based simulation
- [x] Async execution with Tokio
- [x] Outcome metrics collection
- [x] Add temporal simulation (time-based state changes)
- [x] Implement agent behavior models (compliance, evasion)
- [x] Create Monte Carlo simulation mode
- [x] Implement parallel batch processing for large populations
- [x] Agent lifecycle (birth, death, status changes)
- [x] Inter-agent relationships and hierarchies
- [x] Realistic demographic distributions
- [x] A/B testing and sensitivity analysis
- [x] Time-series analysis and projections
- [x] GPU acceleration (v0.2.0)
- [x] Distributed simulation (v0.2.1)
- [x] Agent-based modeling 2.0 (v0.2.2)
- [x] Real-time simulation (v0.2.3)
- [x] Synthetic data generation (v0.2.4)
- [x] Economic simulation extensions (v0.2.5)
- [x] Healthcare simulation (v0.2.6)
- [x] Environmental simulation (v0.2.7)
- [x] Urban simulation (v0.2.8)
- [x] Simulation validation framework (v0.2.9)
- [x] Comprehensive roadmap through v0.2.9 (see crate TODO.md)

### legalis-diff
- [x] Structural diff between statutes
- [x] Change categorization (added/removed/modified)
- [x] Impact assessment with severity levels
- [x] Semantic diff (understanding meaning changes)
- [x] Cross-statute impact analysis
- [x] Amendment chain visualization
- [x] Machine learning integration (v0.2.0)
- [x] Natural language processing with multi-language support (v0.2.1)
- [x] Cloud integration (S3, Azure, GCS) (v0.2.3)
- [x] Collaborative features (v0.2.4)
- [x] Advanced analytics (v0.2.5)
- [x] GPU acceleration for large diffs (v0.2.7)
- [x] AI-powered diff analysis (v0.3.0)
- [x] Legal-domain aware diffing (v0.3.1)
- [x] Collaborative diff review (v0.3.2)
- [x] Version control integration (v0.3.3)
- [x] Compliance-focused diffing (v0.3.4)
- [x] Time-travel diffing (v0.3.5)
- [x] Cross-jurisdiction diffing (v0.3.6)
- [x] Enterprise diff management (v0.3.7)
- [x] Machine-readable diff formats (v0.3.8)
- [x] Quantum-ready diff algorithms (v0.3.9)
- [x] Comprehensive roadmap through v0.4.0 (see crate TODO.md)

---

## Internationalization & Porting Layer

### legalis-i18n
- [x] Locale support with language/country/region
- [x] Jurisdiction registry with legal system types
- [x] Cultural parameters (age of majority, religious observances)
- [x] Legal dictionary for term translation
- [x] Translation manager for multi-language support
- [x] ICU message format support
- [x] Date/time localization for legal deadlines
- [x] Timezone support for international legal deadlines
- [x] Legal citation formatting (Bluebook, OSCOLA, AGLC, McGill, European, Japanese)
- [x] RTL (Right-to-Left) text support for Arabic/Hebrew
- [x] Deadline calculator with business days and timezone awareness
- [x] 60+ language support including emerging markets
- [x] AI-powered translation with LLM integration
- [x] Real-time interpretation and court proceeding support
- [x] Semantic cross-lingual search with embeddings
- [x] Regulatory harmonization (EU, UN treaties, international standards)
- [x] Historical legal language support (archaic terms, old calendars)
- [x] Comprehensive roadmap through v0.3.4 (see crate TODO.md)

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
- [x] Multi-hop porting chains
- [x] Porting templates and presets
- [x] Confidence scoring and risk assessment
- [x] Jurisdiction-specific validators
- [x] Historical porting analytics
- [x] Comprehensive roadmap through v0.3.0 (see crate TODO.md)

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
- [x] Live updates with WebSocket
- [x] PDF/PowerPoint export
- [x] Visual regression testing
- [x] VR statute exploration
- [x] AR legal document overlay
- [x] 360° panoramic timeline viewing
- [x] AI-powered automatic visualization selection
- [x] Live court proceeding visualization
- [x] Legal history scrollytelling
- [x] Looking Glass holographic display
- [x] 3D print export (STL/OBJ/3MF)
- [x] Comprehensive roadmap through v0.2.0 (see crate TODO.md)

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
- [x] Gas optimization for all targets
- [x] Formal verification integration
- [x] Multi-signature wallet generation
- [x] Timelock controller templates
- [x] Upgradeable contract patterns
- [x] Oracle integration templates
- [x] Layer 2 optimizations (Optimism, Arbitrum, zkSync)
- [x] Comprehensive roadmap through v0.3.7 (see crate TODO.md)

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
- [x] Add GraphQL API support
- [x] Implement WebSocket for real-time updates
- [x] Add response caching (Redis)
- [x] OAuth2/OIDC support (Keycloak, Auth0, Okta, Google, GitHub)
- [x] Server-Sent Events (SSE)
- [x] Batch operations
- [x] Async verification with polling
- [x] Streaming simulation results
- [x] gRPC support with reflection and health checks (v0.2.1)
- [x] GraphQL enhancements (APQ, batching, live queries) (v0.2.2)
- [x] API Gateway features (transformations, circuit breaker, load balancing) (v0.2.3)
- [x] Comprehensive roadmap through v0.2.3 (see crate TODO.md)
- [ ] SDK generation from OpenAPI

### legalis
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
- [x] Project management (new, add, update, clean, doctor)
- [x] Registry integration (search, install, list, outdated, uninstall)
- [x] Colored output with table formatting
- [x] Multiple output formats (JSON, YAML, TOML, CSV, HTML)
- [x] Environment variable overrides (LEGALIS_*)
- [x] Dry-run and interactive modes
- [x] AI-powered CLI features (v0.2.0)
- [x] Interactive TUI (v0.2.1)
- [x] Workflow automation (v0.2.2)
- [x] Cloud integration (v0.2.3)
- [x] Comprehensive roadmap through v0.2.3 (see crate TODO.md)
- [ ] Full plugin ecosystem with marketplace

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
- [x] Tamper detection and alerts
- [x] Retention policies and archiving
- [x] Advanced querying with filtering
- [x] PDF report generation
- [x] JSON/CSV export
- [x] Comprehensive roadmap through v0.2.4 (see crate TODO.md)

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
- [x] Format auto-detection
- [x] Validation during conversion
- [x] Lossy vs lossless mode
- [x] Batch conversion support
- [x] Progress reporting for large files
- [x] Comprehensive roadmap through v0.2.8 (see crate TODO.md)

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
- [x] Triple store integration (Virtuoso, Fuseki, GraphDB)
- [x] SKOS taxonomy support
- [x] Named graph management
- [x] Federated SPARQL queries
- [x] Inference rule customization
- [x] Graph visualization export
- [x] Performance optimization for large graphs
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
