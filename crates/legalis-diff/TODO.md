# legalis-diff TODO

## Status Summary

Version: 0.5.9 | Status: Stable | Tests: Passing (862 nextest + 308 doctests) | Warnings: 0

All v0.1.x series features through v0.1.9 (Integration) are complete including advanced analytics (v0.2.5) and testing/quality (v0.2.9). Natural Language Processing (v0.2.1) with multi-language support, Collaborative Features (v0.2.4), and Cloud Integration (v0.2.3) including cloud storage backends (S3, Azure, GCS) and distributed diff computation are now complete. Legal-Domain Aware Diffing (v0.3.1) with legislative history, Compliance-Focused Diffing (v0.3.4), Collaborative Diff Review (v0.3.2), and Version Control Integration (v0.3.3) are now complete. Machine Learning Integration (v0.2.0), Time-Travel Diffing (v0.3.5), Cross-Jurisdiction Diffing (v0.3.6), Enterprise Diff Management (v0.3.7), and Machine-Readable Diff Formats (v0.3.8) are now complete. AI-Powered Diff Analysis (v0.3.0) with LLM-based semantic explanations, intent detection, automatic categorization, impact prediction, and AI-assisted merge conflict resolution is now complete. GPU acceleration for large diffs (v0.2.7) and Quantum-Ready Diff Algorithms (v0.3.9) including quantum-inspired similarity, quantum fingerprinting, quantum-safe signing, hybrid classical-quantum computation, and quantum random sampling are now complete. Real-time Diff Streaming (v0.5.1) with WebSocket support, live collaborative editing, incremental streaming, server-sent events, and real-time conflict resolution is now complete. Advanced Caching & Memoization (v0.5.2) with Redis integration, Memcached support, cache invalidation strategies, smart preloading, and multi-level cache hierarchies is now complete. Machine Learning Model Integration (v0.5.3) with custom model training, transfer learning, automated retraining pipeline, model versioning and rollback, and A/B testing is now complete. Blockchain & Distributed Ledger (v0.5.4) — immutable Merkle/proof-of-work diff recording, a gas-metered smart-contract engine driving review workflows, distributed consensus (PoA/PoS/BFT) for diff verification, a metered token ledger for paid API access, and ERC-721-style NFTs for important diffs — is now complete (pure Rust; live public-chain settlement deferred behind the `ChainAnchor` trait). Advanced Visualization (v0.5.5) — a navigable 3-D `Scene3d` diff graph with deterministic force-directed/sphere/layered/grid layouts, VR/AR scene export (A-Frame/X3D/glTF-JSON), interactive graph navigation, real-time collaborative view sessions (shared camera/cursors/annotations with last-writer-wins convergence), and a custom visualization-plugin registry — is now complete (pure Rust; live GPU/WebXR rasterisation deferred behind the `SceneRenderer` trait). Enterprise Features (v0.5.6) — single sign-on (SAML 2.0 / OIDC with real HMAC-SHA256 token validation), LDAP/Active-Directory support (DN parsing, RFC 4515 filters, bind/scoped-search, transitive nested-group resolution), advanced hierarchical RBAC (role inheritance, resource-pattern + ABAC-conditioned permissions, deny-override, group→role mapping), compliance reporting (SOC 2 / GDPR / HIPAA control catalogues + scored reports), and tamper-evident hash-chained enterprise audit logs with retention policies and legal holds — is now complete, implemented as the pure-Rust `governance` module (live IdP-over-HTTP and networked LDAP deferred behind the `SsoProvider` / `DirectoryService` traits). Mobile & Edge Computing (v0.5.7) — a mobile SDK facade (synchronous JSON-in/JSON-out boundary over the diff engine, ready for a UniFFI/cbindgen Swift/Kotlin wrapper), a latency/work-budgeted edge diff engine with a deterministic node scheduler, offline-first diff computation (local snapshot store + append-only replayable operation queue + durable persistence), a Progressive Web App generator (real W3C manifest + Service Worker + offline diff-viewer shell), and cross-platform synchronization (vector-clock causality, delta sync and convergent conflict resolution) — is now complete, implemented as the pure-Rust `mobile` module (the native language binding is deferred behind the `MobileBridge` trait and PWA hosting/browser runtime is external; both have pure-Rust backends/generators). Advanced Analytics & Insights (v0.5.8) — OLS trend-extrapolation predictive analytics, robust (z-score / MAD modified-z) anomaly detection over configurable change metrics, risk-trajectory change-impact forecasting, explainable factor-weighted risk-assessment automation, and a render-agnostic analytics-dashboard data model with JSON export — is now complete as the pure-Rust `analytics` module. Interoperability & Standards (v0.5.9) legal-XML formats — Akoma Ntoso (OASIS LegalDocML act/meta/body/section/article), OASIS LegalRuleML (prescriptive/constitutive statements with obligation/permission/prohibition deontic operators), and CEN MetaLex (FRBR work/expression interchange with a recursive fragment hierarchy) — are now complete as the pure-Rust `legal_xml` module (parsing via the workspace `quick-xml` reader), each round-tripping both the XML document and a `Statute`; ISO/IEC 27001 (organizational ISMS certification) and the umbrella "W3C Web Standards integration" item are deferred as having no actionable code target.

---

## Completed

- [x] Structural diff between statutes
- [x] Change categorization (added/removed/modified)
- [x] Impact assessment with severity levels
- [x] Basic change reports

## Features

- [x] Semantic diff (understanding meaning changes)
- [x] Cross-statute impact analysis
- [x] Amendment chain visualization
- [x] Diff output in multiple formats (JSON, HTML, Markdown)
- [x] Side-by-side comparison view

## Advanced Analysis

- [x] Detect logically equivalent changes
- [x] Identify breaking vs non-breaking changes
- [x] Track condition relaxation/tightening
- [x] Analyze effect scope changes

## Merge Support

- [x] Add three-way merge for concurrent amendments
- [x] Implement conflict detection and resolution
- [x] Support merge strategies (ours, theirs, union)

## Visualization

- [x] Generate visual diff reports
- [x] Timeline visualization for amendments
- [x] Blame-style annotation for change tracking

## Integration

- [x] Git-style diff interface
- [x] Hook into version control systems
- [x] Create diff templates for common patterns

## Testing

- [x] Add comprehensive diff test cases
- [x] Test edge cases (empty statutes, identical statutes)
- [x] Benchmark diff performance on large statutes

## Performance & Optimization

- [x] Implement diff caching and memoization
- [x] Add incremental diff support
- [x] Create batch diff computation
- [x] Optimize for repeated diffs

## Advanced Algorithms

- [x] Implement Myers diff algorithm
- [x] Implement Patience diff algorithm
- [x] Add edit distance calculation
- [x] Support for advanced diff operations

## Statistical Analysis

- [x] Add statistical analysis of changes
- [x] Implement change pattern detection
- [x] Create aggregate statistics across multiple diffs
- [x] Generate statistical summaries and reports

## Enhanced Error Handling

- [x] Add specific error variants for different scenarios
- [x] Implement version conflict detection
- [x] Add merge conflict error types
- [x] Support serialization error handling

## Fuzzy Matching & Similarity

- [x] Implement Levenshtein distance calculation
- [x] Add similarity scoring between changes
- [x] Find similar changes across multiple diffs
- [x] Group similar changes by pattern
- [x] Support configurable similarity thresholds

## Change Recommendation System

- [x] Implement recommendation generation based on patterns
- [x] Add priority levels (Low, Medium, High, Critical)
- [x] Support multiple recommendation categories
- [x] Provide confidence scores for recommendations
- [x] Filter and sort recommendations
- [x] Detect common pitfalls in amendments
- [x] Analyze historical patterns for suggestions

## Enhanced Summarization

- [x] Add detailed summary with confidence scores
- [x] Provide change detection confidence metrics
- [x] Include impact assessment confidence
- [x] Generate analytical insights
- [x] Break down changes by type (added/removed/modified)

## Partial Comparison Support

- [x] Compare only preconditions between statutes
- [x] Compare only effects between statutes
- [x] Support targeted diff operations
- [x] Reduce computational overhead for partial comparisons

## Parallel Processing

- [x] Implement parallel diff computation using rayon
- [x] Add batch diff operations for multiple statute pairs
- [x] Support parallel sequence diffing
- [x] Add parallel processing for multiple sequences

## Rollback Analysis

- [x] Generate rollback diffs (reverse changes)
- [x] Analyze rollback feasibility and complexity
- [x] Identify rollback risks and issues
- [x] Provide rollback recommendations
- [x] Support rollback chain generation

## Change Validation

- [x] Validate diff completeness and consistency
- [x] Detect inconsistent change data
- [x] Verify impact assessment accuracy
- [x] Check for duplicate changes
- [x] Provide validation scores and warnings

## Export Formats

- [x] CSV export for spreadsheet analysis
- [x] Batch CSV export for multiple diffs
- [x] Proper CSV escaping for special characters

## Integrated Analysis & Batch Operations

- [x] Parallel batch validation using rayon
- [x] Batch validation summaries with aggregate statistics
- [x] Failed statute tracking in batch operations
- [x] Average validation score calculation
- [x] Integration of validation with parallel processing

## Performance Benchmarks

- [x] Parallel diff pair benchmarks
- [x] Batch validation benchmarks
- [x] Parallel validation benchmarks
- [x] Rollback generation benchmarks
- [x] Rollback analysis benchmarks
- [x] Parallel rollback generation benchmarks
- [x] Parallel rollback analysis benchmarks
- [x] Rollback statistics computation benchmarks
- [x] Parallel rollback validation benchmarks

## Parallel Rollback Operations

- [x] Parallel rollback diff generation
- [x] Parallel rollback feasibility analysis
- [x] Batch rollback operations with rayon
- [x] Performance optimization for large-scale rollback processing

## Rollback Statistics

- [x] Aggregate statistics across multiple rollback analyses
- [x] Complexity distribution tracking
- [x] Risk distribution analysis
- [x] Average recommendations calculation
- [x] Feasibility metrics

## Rollback Validation

- [x] Validate rollback diffs against forward diffs
- [x] Ensure proper value reversal
- [x] Target consistency checking
- [x] Parallel rollback validation
- [x] Integration with existing validation framework

## Parallel Export Operations

- [x] Parallel export to multiple formats
- [x] Batch export with format selection
- [x] Export to all formats simultaneously
- [x] Single diff multi-format export
- [x] ExportFormat enum for type-safe format selection

## Roadmap for 0.1.0 Series

### Semantic Diff Improvements (v0.1.1)
- [x] Add semantic equivalence detection (same meaning, different syntax)
- [x] Add intent-preserving refactoring detection
- [x] Add condition relaxation/tightening metrics
- [x] Add effect scope change quantification
- [x] Add breaking change classification

### Advanced Merge (v0.1.2)
- [x] Add semantic merge for compatible changes
- [x] Add conflict resolution suggestions
- [x] Add merge preview with impact assessment
- [x] Add interactive merge mode
- [x] Add merge history tracking

### Change Analysis (v0.1.3)
- [x] Add change impact scoring (0-100 scale)
- [x] Add stakeholder impact analysis
- [x] Add regulatory compliance impact
- [x] Add backward compatibility scoring
- [x] Add migration effort estimation

### Visualization Enhancements (v0.1.4)
- [x] Add interactive HTML diff viewer
- [x] Add syntax-highlighted diff output
- [x] Add inline annotations for change explanations
- [x] Add diff animation for presentations
- [x] Add three-way diff visualization

### Pattern Recognition (v0.1.5)
- [x] Add common amendment pattern library
- [x] Add pattern-based change suggestions
- [x] Add anti-pattern detection
- [x] Add best practice recommendations
- [x] Add historical pattern learning

### Audit Trail (v0.1.7)
- [x] Add change attribution (who changed what)
- [x] Add change justification tracking
- [x] Add approval workflow integration
- [x] Add change lifecycle tracking (proposed → approved → enacted)
- [x] Add rollback planning from diffs

### Performance (v0.1.7)
- [x] Add streaming diff for large statutes
- [x] Add incremental diff updates
- [x] Add diff result caching (already implemented in optimization module)
- [x] Add memory-efficient diff algorithms

### Export Formats (v0.1.8)
- [x] Add Word track-changes format
- [x] Add PDF with highlighted changes
- [x] Add LaTeX redline format
- [x] Add unified diff format (patch files)
- [x] Add structured changelog (CHANGELOG.md)

### Integration (v0.1.9)
- [x] Add Git integration for version control (already implemented in git/vcs modules)
- [x] Add GitHub/GitLab PR diff integration
- [x] Add notification webhooks for changes
- [x] Add diff-based CI/CD triggers
- [x] Add diff API for external tools

## Roadmap for 0.2.0 Series

### Machine Learning Integration (v0.2.0)
- [x] Add ML-based change classification
- [x] Implement pattern learning from historical diffs
- [x] Add anomaly detection for unusual changes
- [x] Create predictive models for change impact
- [x] Add automated change categorization

### Natural Language Processing (v0.2.1)
- [x] Generate natural language summaries of changes
- [x] Add semantic similarity using NLP techniques
- [x] Implement intent extraction from changes
- [x] Create automated change explanations
- [x] Add multi-language support for summaries

### Advanced Visualization (v0.2.2)
- [x] Add interactive web-based diff explorer
- [x] Create dependency graphs for changes
- [x] Implement heatmaps for change frequency
- [x] Add temporal visualization of amendments
- [x] Create customizable diff dashboards

### Cloud Integration (v0.2.3)
- [x] Add cloud storage backends (S3, Azure, GCS)
- [x] Implement distributed diff computation
- [x] Add webhook integration with cloud services
- [x] Create REST API for diff operations
- [x] Add authentication and authorization

### Collaborative Features (v0.2.4)
- [x] Add real-time collaborative diff review
- [x] Implement change commenting system
- [x] Add approval workflows
- [x] Create change voting mechanisms
- [x] Add conflict resolution collaboration

### Advanced Analytics (v0.2.5)
- [x] Add time-series analysis of changes
- [x] Implement change velocity metrics
- [x] Create compliance drift detection
- [x] Add risk scoring over time
- [x] Generate trend reports

### Extensibility (v0.2.6)
- [x] Add plugin system for custom analyzers
- [x] Create DSL for custom diff rules
- [x] Implement custom export format plugins
- [x] Add scripting support (Rhai)
- [x] Create extension API

### Performance Optimization (v0.2.7)
- [x] Implement GPU acceleration for large diffs
- [x] Add incremental compilation for diff cache
- [x] Create adaptive algorithms based on input
- [x] Optimize memory usage for streaming
- [x] Add SIMD optimizations

### Security Features (v0.2.8)
- [x] Add cryptographic signing of diffs
- [x] Implement tamper detection
- [x] Add encryption for sensitive changes
- [x] Create audit trail integrity verification
- [x] Add access control for diff operations

### Testing & Quality (v0.2.9)
- [x] Add property-based testing with proptest
- [x] Implement fuzzing for diff algorithms
- [x] Create mutation testing
- [x] Add performance regression tests
- [x] Generate coverage reports

## Roadmap for 0.3.0 Series (Next-Gen Features)

### AI-Powered Diff Analysis (v0.3.0)
- [x] Add LLM-based semantic diff explanation
- [x] Implement intent detection for changes
- [x] Add automatic change categorization
- [x] Create impact prediction from diffs
- [x] Add AI-assisted merge conflict resolution

### Legal-Domain Aware Diffing (v0.3.1)
- [x] Add article/section structure awareness
- [x] Implement citation reference tracking
- [x] Add defined term propagation analysis
- [x] Create cross-reference impact detection
- [x] Add legislative history integration

### Collaborative Diff Review (v0.3.2)
- [x] Add real-time collaborative diff viewing
- [x] Implement commenting and annotation system
- [x] Add approval workflow integration
- [x] Create change request management
- [x] Add stakeholder notification system

### Version Control Integration (v0.3.3)
- [x] Add native Git integration for statutes
- [x] Implement Git LFS for large statute sets
- [x] Add branch comparison for statute variants
- [x] Create pull request diff integration
- [x] Add blame analysis for statute history

### Compliance-Focused Diffing (v0.3.4)
- [x] Add regulatory change impact assessment
- [x] Implement breaking change detection
- [x] Add backward compatibility analysis
- [x] Create compliance gap identification
- [x] Add enforcement date tracking

### Time-Travel Diffing (v0.3.5)
- [x] Add point-in-time statute reconstruction
- [x] Implement temporal diff queries
- [x] Add effective date-aware comparisons
- [x] Create sunset clause tracking
- [x] Add amendment chain visualization

### Cross-Jurisdiction Diffing (v0.3.6)
- [x] Add equivalent statute matching
- [x] Implement jurisdiction-aware normalization
- [x] Add multilingual diff alignment
- [x] Create harmonization gap detection
- [x] Add treaty comparison support

### Enterprise Diff Management (v0.3.7)
- [x] Add diff archiving and retention
- [x] Implement diff search and discovery
- [x] Add audit trail for diff operations
- [x] Create diff analytics dashboard
- [x] Add role-based diff access control

### Machine-Readable Diff Formats (v0.3.8)
- [x] Add OpenAPI diff specification
- [x] Implement JSON Patch (RFC 6902) support
- [x] Add JSON Merge Patch (RFC 7386) support
- [x] Create GraphQL schema diff
- [x] Add protobuf diff serialization

### Quantum-Ready Diff Algorithms (v0.3.9)
- [x] Add quantum-inspired similarity algorithms
- [x] Implement quantum fingerprinting for large statutes
- [x] Add quantum-safe diff signing
- [x] Create hybrid classical-quantum diff computation
- [x] Add quantum random sampling for large comparisons

## Roadmap for 0.4.0 Enhancements (Completed)

### Integration & Workflow (v0.4.0)
- [x] Comprehensive diff pipeline combining all features
- [x] Smart diff analyzer with automatic strategy selection
- [x] Distributed analysis workflow
- [x] Integration examples for real-world use cases
- [x] Enriched diff results with AI analysis
- [x] Batch processing optimizations

### Performance Benchmarks (v0.4.0)
- [x] Cloud storage operation benchmarks
- [x] Distributed computation benchmarks
- [x] GPU batch processing benchmarks
- [x] LLM analysis benchmarks
- [x] Quantum algorithm benchmarks
- [x] Comprehensive pipeline benchmarks

## Roadmap for 0.5.0 Series (Future)

### Real-time Diff Streaming (v0.5.1)
- [x] WebSocket-based real-time diff updates
- [x] Live collaborative editing with diff tracking
- [x] Incremental diff streaming for large documents
- [x] Server-sent events for diff notifications
- [x] Real-time conflict resolution

### Advanced Caching & Memoization (v0.5.2)
- [x] Redis integration for distributed caching
- [x] Memcached support for high-performance caching
- [x] Cache invalidation strategies
- [x] Smart cache preloading
- [x] Multi-level cache hierarchies

### Machine Learning Model Integration (v0.5.3)
- [x] Custom ML model training from diff history
- [x] Transfer learning for domain-specific diffs
- [x] Automated model retraining pipeline
- [x] Model versioning and rollback
- [x] A/B testing for ML predictions

### Blockchain & Distributed Ledger (v0.5.4)
- [x] Immutable diff recording on blockchain
- [x] Smart contract integration for automated workflows
- [x] Distributed consensus for diff verification
- [x] Cryptocurrency integration for paid API access
- [x] NFT generation for important diffs

**Completed 2026-06-14** — Implemented as the pure-Rust `blockchain` module
(`src/blockchain/`): `mod.rs`, `ledger.rs`, `contract.rs`, `consensus.rs`,
`token.rs`, `nft.rs`. Real SHA-256 hashing throughout (existing workspace `sha2`
+ `hex` deps).

- **Immutable diff recording** (`ledger.rs`): `DiffLedger`, `Block`,
  `DiffRecord`, `MerkleTree`/`MerkleProof` — an append-only chain that commits
  diff records via a binary Merkle tree, links blocks by hash, seals them with
  proof-of-work, validates the whole chain (header/PoW/Merkle/linkage/content)
  and produces compact inclusion proofs.
- **Smart contracts** (`contract.rs`): `SmartContract`/`Clause`/`Trigger`
  (composable predicate AST) /`Action`, executed by a deterministic,
  gas-metered `ContractEngine` that drives a `LegalWorkflow` lifecycle state
  machine (proposed → review → approved → enacted, plus freeze/reject).
- **Distributed consensus** (`consensus.rs`): `ConsensusEngine`/`Validator`/
  `Vote`/`Proposal`/`ConsensusOutcome` — PoA round-robin proposer selection,
  stake-weighted deterministic PoS election, and a BFT 2/3 voting tally with
  equivocation detection and `f = (n-1)/3` fault tolerance.
- **Token ledger for paid API** (`token.rs`): `TokenLedger`/`TokenTransaction`/
  `PricingTable`/`ApiOperation`/`UsageReport` — integer-exact, overflow-checked
  accounting with key-derived address ownership, nonce replay protection,
  supply conservation, a deterministic state root, and metered pay-per-call
  pricing.
- **NFTs for important diffs** (`nft.rs`): `NftRegistry`/`DiffNft`/`NftMetadata`/
  `ProvenanceEntry` — ERC-721-style minting (deterministic token ids, duplicate
  rejection), ownership transfer/burn with a tamper-evident provenance
  hash-chain, token-URI serialization, and an `is_mint_worthy` heuristic.
- **Deferred external binding**: live public-chain settlement (Ethereum/Bitcoin
  RPC) is abstracted behind the `ChainAnchor` trait with `InMemoryAnchor` and
  `FileAnchor` backends provided; a networked backend can be added without
  changing callers.

Tests: 84 new `#[test]`s + 3 doctests, all green. Crate suite 529 → 613 tests
(nextest), 285 doctests; `cargo clippy -p legalis-diff --all-targets -- -D
warnings` clean. Dependencies: enabled existing workspace `sha2` and `hex` for
this crate (no new workspace dependencies); added blockchain-specific
`DiffError` variants (`ChainIntegrity`, `InvalidTransaction`,
`InsufficientBalance`, `ConsensusFailure`, `ContractError`, `NftError`).

### Advanced Visualization (v0.5.5)
- [x] 3D diff visualization for complex relationships
- [x] VR/AR support for immersive diff exploration
- [x] Interactive graph-based diff navigation
- [x] Real-time collaborative visualization
- [x] Custom visualization plugins

**Completed 2026-06-14** — Implemented as the pure-Rust `immersive` module
(`src/immersive/`): `mod.rs`, `scene.rs`, `layout.rs`, `xr.rs`, `navigation.rs`,
`collab.rs`, `plugin.rs`. Distinct from the existing 2-D `visual`/`advanced_visual`
modules (DOT/SVG/HTML) and from `collaborative_review` (comment/vote workflows).

- **3D diff visualization** (`scene.rs`): `Scene3d`, `SceneNode`, `SceneEdge`,
  `NodeKind`/`EdgeKind` — a navigable 3-D scene graph built from one or many
  `StatuteDiff`s (statute root + change/target-group/impact nodes with typed
  `Contains`/`Grouped`/`Related`/`Impacts` edges), plus deterministic seeded
  positions and JSON round-trip. Core 3-D math (`Vec3` with operator traits,
  `Color`, `Camera`, `Viewport`, `BoundingBox`) lives in `mod.rs`.
- **Layout** (`layout.rs`): `apply_layout` with four deterministic algorithms —
  Fruchterman–Reingold force-directed (cooling + centring gravity, O(n²)
  repulsion / edge attraction), Fibonacci-sphere (golden-angle), BFS-`Layered`,
  and a cubic `Grid` — all `rand`-free (seeded via FNV-1a; SciRS2 policy).
- **VR/AR export** (`xr.rs`): `export_scene` to `XrFormat::AFrame` (WebXR HTML),
  `X3d` (ISO XML), and `GltfJson` (glTF-like manifest), with XML/HTML escaping.
- **Interactive navigation** (`navigation.rs`): `SceneNavigator` — focus,
  neighbour expansion/collapse, BFS shortest-path, breadth-first ordering,
  level-of-detail buckets and a back/forward focus history.
- **Real-time collaborative visualization** (`collab.rs`): `VizSession` with a
  shared `CameraState`, per-participant `PresenceCursor`s, `SceneAnnotation`s and
  a sequence-ordered `VizEvent` log; concurrent updates reconciled by
  last-writer-wins on monotonic per-resource sequences (replicas converge to an
  identical SHA-256 `state_digest`).
- **Custom plugins** (`plugin.rs`): `VisualizationPlugin` trait + `VizPluginRegistry`
  (reusing `plugins::PluginMetadata`), with built-in `WireframeJsonPlugin` and an
  orthographic-projection `AsciiScatterPlugin`.
- **Deferred external binding**: live GPU (WebGL/WebGPU) / WebXR rasterisation is
  abstracted behind the `SceneRenderer` trait with pure-Rust `JsonSceneRenderer`
  (depth-sorted draw-call `RenderManifest`) and `NullSceneRenderer` backends; a
  networked/graphics backend can be added without changing callers.

Tests: 67 new `#[test]`s + 4 doctests, all green. Crate suite 613 → 680 tests
(nextest), 285 → 289 doctests; `cargo clippy -p legalis-diff --all-targets -- -D
warnings` clean. Dependencies: none added (reused existing `serde`/`serde_json`/
`sha2`/`hex`); added one additive `DiffError::Visualization(String)` variant.

### Enterprise Features (v0.5.6)
- [x] Single sign-on (SSO) integration
- [x] LDAP/Active Directory support
- [x] Advanced role-based access control (RBAC)
- [x] Compliance reporting (SOC 2, GDPR, HIPAA)
- [x] Enterprise audit logs with retention policies

**Completed 2026-06-14** — Implemented as the pure-Rust `governance` module
(`src/governance/`): `mod.rs`, `sso.rs`, `directory.rs`, `rbac.rs`,
`compliance_report.rs`, `audit_log.rs`. Deliberately distinct from the existing
`enterprise` (diff archiving / basic role checks), `security` (signing /
encryption) and `compliance` (regulatory *change* impact) modules — here the
subject is the *system's* identity, authorization and compliance posture. Real
SHA-256 / HMAC-SHA256 throughout (existing workspace `sha2` + `hex` deps).

- **SSO integration** (`sso.rs`): `SsoProvider` trait + `InMemoryIdentityProvider`,
  `SamlAssertion`, `OidcIdToken`, `SsoToken`, `SsoSession` — SAML 2.0 / OIDC token
  modelling with real **HMAC-SHA256 (HS256)** signing/verification (RFC 2104,
  RFC 4231-tested), plus issuer / audience / validity-window (clock-skew) checks
  that map a validated token to a `Principal`.
- **LDAP / Active-Directory** (`directory.rs`): `DirectoryService` trait +
  `InMemoryDirectory`, `DistinguishedName` parsing, `DirectoryEntry` multi-valued
  attributes, an RFC 4515 `LdapFilter` parser/matcher (presence / equality /
  substring / `&`|`!`), password `bind`, scoped `search` (Base/OneLevel/Subtree)
  and **transitive** nested-group resolution (`memberOf`/`member`).
- **Advanced RBAC** (`rbac.rs`): `RbacEngine` with hierarchical `Role`
  inheritance (cycle-safe), resource-pattern `Permission`s (`ResourcePattern`
  globs), ABAC `Condition`s, deny-override resolution, default-deny, and
  group→role mapping driven by `Principal` groups; `authorize` returns
  `AccessDenied`.
- **Compliance reporting** (`compliance_report.rs`): `ComplianceFramework`
  (SOC 2 / GDPR / HIPAA) built-in `Control` catalogues, `ComplianceAssessment`,
  scored `ComplianceReport` (`generate_report`, JSON + Markdown) with findings /
  recommendations, and a `SecurityPosture` capability mapping that auto-populates
  controls (tying RBAC / audit-log / encryption posture to controls).
- **Enterprise audit logs** (`audit_log.rs`): `EnterpriseAuditLog` — an
  append-only **SHA-256 hash-chained**, tamper-evident log (`verify_integrity`),
  `RetentionPolicy` (max-age / max-entries / min-retention floor / freeze),
  **legal holds**, verifiable prefix purge with checkpoint hashing, query
  filtering and CSV/JSON export.
- **Deferred external bindings**: live IdP-over-HTTP (SAML redirect/POST, OIDC
  discovery + JWKS + RSA) and live networked LDAP/AD are abstracted behind the
  `SsoProvider` and `DirectoryService` traits with pure-Rust in-memory backends
  provided; networked backends can be added without changing callers.

Tests: 49 new `#[test]`s + 6 doctests, all green. Crate suite 680 → 729 tests
(nextest), 289 → 295 doctests; `cargo clippy -p legalis-diff --all-targets -- -D
warnings` clean. Dependencies: none added (reused existing `serde`/`serde_json`/
`chrono`/`sha2`/`hex`); added three additive `DiffError` variants
(`AuthenticationFailed`, `AccessDenied`, `DirectoryError`).

### Mobile & Edge Computing (v0.5.7)
- [x] Mobile SDK for iOS and Android
- [x] Edge computing support for low-latency diffs
- [x] Offline-first diff computation
- [x] Progressive Web App (PWA) for diff viewing
- [x] Cross-platform synchronization

**Completed 2026-06-14** — Implemented as the pure-Rust `mobile` module
(`src/mobile/`): `mod.rs`, `sdk.rs`, `edge.rs`, `offline.rs`, `pwa.rs`,
`sync.rs`. Shared `DeviceClass` / `DeviceProfile` / `NetworkQuality` plus SHA-256
helpers (existing workspace `sha2` + `hex`) underpin the five sub-modules.

- **Mobile SDK** (`sdk.rs`): `MobileSdk` exposes a synchronous, panic-free
  JSON-in/JSON-out boundary (`handle_json`) over typed `MobileRequest` /
  `MobileResponse` enums (compute diff / summarize / detailed summary / breaking
  check / SDK info), the natural surface for a UniFFI/cbindgen Swift/Kotlin
  wrapper, plus secure-storage helpers (`persist_diff` / `load_diff`).
- **Edge low-latency diffing** (`edge.rs`): `EdgeDiffer` computes a diff under an
  explicit `EdgeBudget` (work-unit budget + optional wall-clock deadline) — change
  cap, value truncation to bound memory, byte-identical fast-path, early-exit
  `truncated` flag, and an O(1) `quick_severity` triage mode; `EdgeScheduler`
  deterministically places `EdgeJob`s onto the best `EdgeNode` by
  capacity/load/network, skipping unhealthy, offline or under-provisioned nodes.
- **Offline-first** (`offline.rs`): `OfflineEngine` keeps a local snapshot store
  and an append-only operation queue (`UpsertStatute` / `RecordDiff`), computes
  diffs optimistically without a network, supports the sync lifecycle
  (`pending` → `mark_synced` → `drain_synced`), reconstructs state from the log
  via `replay`, and persists to / restores from disk (`save_to_path` /
  `load_from_path`).
- **PWA generator** (`pwa.rs`): `PwaBundle::from_diff` emits a real W3C Web App
  Manifest (`PwaManifest`), a Service Worker implementing a chosen `CacheStrategy`
  (`ServiceWorkerConfig::to_javascript`) and an HTML-escaped offline diff-viewer
  shell, writable to disk (`write_to_dir`).
- **Cross-platform sync** (`sync.rs`): `SyncEngine` reconciles state across
  devices using `VectorClock` causality, `delta_since` delta sync and a convergent
  total-order resolution (Lamport-projection primary key + `ConflictResolution`
  tiebreak), so `sync_pair` drives two replicas to an identical `state_digest`;
  concurrent edits are recorded as `SyncConflict`s and tombstones replicate.
- **Deferred external bindings**: the *native* mobile language binding (UniFFI /
  cbindgen + Swift/Kotlin, Keychain / Android Keystore, APNs / FCM) is abstracted
  behind the `sdk::MobileBridge` trait with a complete pure-Rust
  `InMemoryBridge`; PWA *hosting / browser runtime* is external (the generator
  produces deployable, standards-compliant assets but does not serve them).

Tests: 47 new `#[test]`s + 6 doctests, all green. Crate suite 729 → 776 tests
(nextest), 295 → 301 doctests; `cargo clippy -p legalis-diff --all-targets -- -D
warnings` clean. Dependencies: none added (reused existing `serde`/`serde_json`/
`chrono`/`sha2`/`hex`); no new `DiffError` variants (reused
`SerializationError` / `IdMismatch`).

### Advanced Analytics & Insights (v0.5.8)
- [x] Predictive analytics for future changes
- [x] Anomaly detection in diff patterns
- [x] Change impact forecasting
- [x] Risk assessment automation
- [x] Custom analytics dashboards

### Interoperability & Standards (v0.5.9)
- [ ] ISO/IEC 27001 compliance — DEFERRED: an organizational information-security-management-system certification (process/audit scope), not implementable as library code.
- [ ] W3C Web Standards integration — DEFERRED: no concrete spec named; "W3C Web Standards" is an umbrella term with no single actionable target for this crate (note: a real W3C Web App Manifest + Service Worker generator already ships in the `mobile::pwa` module under v0.5.7).
- [x] OASIS LegalRuleML support
- [x] Akoma Ntoso XML format support
- [x] CEN Metalex standard compliance

## COMPLETED (2026-06-14 — analytics + legal XML formats)

### Advanced Analytics & Insights (v0.5.8)
Implemented as the pure-Rust `analytics` module (no new external deps; depends
only on `legalis-core` and this crate's diff types). All analyses are
deterministic.

- **Predictive analytics for future changes** (`analytics/predictive.rs`):
  `forecast_change_volume` extrapolates a statute's change-volume history with an
  ordinary-least-squares `LinearModel` (`slope`/`intercept`/`r_squared`) fitted
  by `fit_linear_model`, projecting each future revision with a residual-sized
  interval (`ChangeForecast`/`ChangeProjection`). `series_from_diffs` builds the
  series from an ordered diff slice.
- **Anomaly detection in diff patterns** (`analytics/anomaly.rs`):
  `detect_anomalies` flags statistical outliers over a configurable metric
  (`AnomalyMetric`: change count / severity rank / kind diversity / removal
  count) using either a classic z-score or a robust MAD-based modified z-score,
  with the Iglewicz–Hoaglin mean-absolute-deviation fallback when the MAD
  collapses on near-constant histories (`AnomalyConfig`/`AnomalyReport`).
- **Change-impact forecasting** (`analytics/forecast.rs`): `forecast_impact`
  treats the per-revision risk score as a time series, projects it forward and
  classifies the `ImpactTrajectory` (escalating / stable / de-escalating)
  (`ImpactForecast`/`ImpactProjection`).
- **Risk-assessment automation** (`analytics/risk.rs`): `assess_risk` produces an
  explainable `[0,100]` score from weighted, retained `RiskFactor`s (severity,
  outcome, eligibility, discretion, saturating breadth) banded into a
  `RiskLevel` (`RiskAssessment`).
- **Custom analytics dashboards (data)** (`analytics/dashboard.rs`):
  `build_dashboard` assembles a render-agnostic `AnalyticsDashboard` of typed
  `DashboardWidget`s (scorecard / gauge / time-series / distribution / table)
  with JSON export/import — the dashboard *data*, not a GUI.

### Interoperability & Standards (v0.5.9) — legal XML formats
Implemented as the pure-Rust `legal_xml` module. Parsing uses the workspace
`quick-xml` reader (correct entity/attribute handling) via a shared
`XmlNode` tree (`legal_xml/xml_util.rs`); emission uses a small indented
`XmlBuilder` with XML 1.0 escaping (`legal_xml/writer.rs`). Each format models
the core element vocabulary of the real standard and round-trips both the
document itself and a `legalis_core::Statute` (machine-readable originals are
preserved inline so reconstruction is exact).

- **Akoma Ntoso / OASIS LegalDocML** (`legal_xml/akoma_ntoso.rs`):
  `AkomaNtosoDocument` models `akomaNtoso → act → meta`(FRBR
  Work/Expression/Manifestation identification) `/ body → section → article`
  (`num`/`heading`/`content`); `from_statute`/`to_statute`/`to_xml`/`from_xml`.
- **OASIS LegalRuleML** (`legal_xml/legalruleml.rs`): `LegalRuleMlDocument`
  models `lrml:LegalRuleML → Statements` of `PrescriptiveStatement`
  (`ruleml:Rule` if/then) and `ConstitutiveStatement` (facts), with deontic
  operators `Obligation`/`Permission`/`Prohibition` (`DeonticKind`) chosen from
  the statute's `EffectType`, and `ruleml:Atom`/`Rel`/`Ind` logical atoms.
- **CEN MetaLex** (`legal_xml/metalex.rs`): `MetalexDocument` models the
  FRBR `bibliographicWork` / `bibliographicExpression` interchange structure
  with a recursive typed `fragment` hierarchy (eligibility → condition,
  effect, discretion).

Deferred (no actionable code target): **ISO/IEC 27001** (organizational ISMS
certification) and **W3C Web Standards integration** (umbrella term, no concrete
spec; note a real W3C Web App Manifest + Service Worker generator already exists
in `mobile::pwa`).
