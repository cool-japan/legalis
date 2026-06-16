# legalis-interop TODO

## Status Summary

Version: 0.3.0 | Status: Stable | Tests: 700 passing (717 with all features) | Warnings: 0

All v0.1.x, v0.2.x, and v0.3.0 series features complete, plus the v0.3.2 AI-Native Formats batch, the v0.3.3 Quantum-Safe Format Migration batch, and the v0.3.4 Cross-Reality Legal Formats batch. Supports 51+ legal formats including Catala, Stipula, L4, Akoma Ntoso, LegalRuleML, LKIF, BPMN, DMN, CMMN, RuleML, SBVR, OpenLaw, Cicero, CommonForm, Clause.io, ContractExpress, FORMEX, NIEM, FinReg, XBRL, RegML, MiFID II, Basel III, SAP Legal Module, Salesforce Contract, DocuSign, MS Word Legal, PDF Legal, Solidity, Vyper, Cadence, and Move, plus AI-native formats (LLM-native, embedding, neural-document, attention-markup, semantic-chunk), a long-term preservation archive (BagIt-like container with quantum-resistant fixity, post-quantum hash-based signatures, migration metadata, and a cryptographic-agility layer), and cross-reality formats (VR/AR annotation, 3D document, holographic display, spatial markup, metaverse-native). Universal Legal Format (ULF) v1.0.0 with format negotiation, versioning, and compatibility layers now available as canonical interchange format for lossless conversions.

---

## Completed

- [x] Catala AST parser → legalis_core::Statute
- [x] legalis_core::Statute → Catala output
- [x] Support for Catala's literate programming style
- [x] Handle Catala's scope and context model
- [x] Stipula contract parser → legalis_core::Statute
- [x] legalis_core::Statute → Stipula output
- [x] Map party/asset model to legal entities
- [x] L4 parser → legalis_core::Statute
- [x] legalis_core::Statute → L4 output
- [x] Support for deontic logic (MUST, MAY, SHANT)
- [x] Handle rule-based reasoning model
- [x] Akoma Ntoso XML import/export
- [x] CLI integration (import, convert commands)
- [x] Bidirectional conversion with loss reporting
- [x] Metadata mapping between formats
- [x] Round-trip conversion tests
- [x] Conversion confidence scoring

## Format Support

### Catala
- [x] Preserve legal article references during conversion
- [x] Support for Catala's exception handling
- [x] Handle scope inheritance

### Stipula
- [x] Convert state machines to condition logic
- [x] Support for temporal obligations
- [x] Handle asset transfer semantics

### L4
- [x] Convert decision tables
- [x] Support for L4's temporal operators
- [x] Handle L4's default logic

### Standard Formats
- [x] LegalRuleML import/export
- [x] LKIF (Legal Knowledge Interchange Format)
- [x] LegalDocML support

## Quality Assurance

- [x] Semantic preservation validation
- [x] Coverage reports for format features
- [x] Batch conversion support
- [x] Diff-aware incremental conversion

## Performance

- [x] Optimize large document conversion (via caching and incremental conversion)
- [x] Add conversion caching
- [x] Parallel conversion support (optional feature)

## Testing

- [x] Add comprehensive test suites per format (99 tests total)
- [x] Test edge cases and error handling (27 edge case tests)
- [x] Benchmark conversion performance (criterion benchmarks)

## Advanced Features (New)

- [x] Streaming API for processing large documents without full memory load
  - [x] StreamingImporter for chunked reading
  - [x] StreamingExporter for batched writing
  - [x] StreamingConverter for end-to-end streaming
  - [x] 7 comprehensive tests
- [x] Async conversion APIs with tokio support (optional `async` feature)
  - [x] AsyncConverter with file-based operations
  - [x] Concurrent batch processing
  - [x] 5 comprehensive tests
- [x] Performance optimizations module
  - [x] String interning for memory efficiency
  - [x] Pre-compiled regex cache for common patterns
  - [x] Whitespace normalization utilities
  - [x] Identifier conversion utilities (CamelCase <-> snake_case)
  - [x] 18 comprehensive tests
- [x] Enhanced converter with integrated optimizations
  - [x] EnhancedConverter combining all optimizations
  - [x] Conversion statistics tracking
  - [x] Source analysis capabilities
  - [x] 9 comprehensive tests
- [x] Rich error messages with context
  - [x] ContextualError with line/column information
  - [x] Source code snippets in error messages
  - [x] Format-specific error suggestions
  - [x] SourceLocation helper for error positioning
  - [x] 11 comprehensive tests
- [x] Comprehensive benchmarking suite
  - [x] Benchmarks for streaming operations
  - [x] Benchmarks for enhanced converter
  - [x] Benchmarks for optimization utilities
  - [x] 18 total benchmark functions

## Summary

**Total Test Coverage**: 502 tests (with all features enabled)
- Default features: 502 tests passing
- Async feature: 502 tests
- Batch feature: 502 tests
- All features: 502 tests
- **5 new format support modules added (v0.1.1)**
  - OASIS LegalCite
  - CEN MetaLex
  - MPEG-21 REL
  - Creative Commons
  - SPDX
- **Quality metrics module added (v0.1.2)**
  - Semantic loss quantification (0-100%)
  - Structure preservation scoring
  - Metadata completeness analysis
  - Round-trip fidelity testing
  - Conversion confidence calibration
  - 9 comprehensive tests
- **Schema validation module added (v0.1.3)**
  - XML Schema (XSD) validation
  - JSON Schema validation
  - Custom schema extension points
  - Schema migration utilities
  - Schema compatibility checking
  - 5 comprehensive tests
- **Format detection module added (v0.1.4)**
  - Automatic format detection with confidence scoring
  - Encoding detection (UTF-8, UTF-16, ASCII, Latin-1)
  - Format version detection
  - Mixed format handling
  - Content-based format recommendation
  - 8 comprehensive tests
- **Batch processing module added (v0.1.5)**
  - Directory-based batch conversion with file pattern matching
  - Watch mode for continuous conversion (file system monitoring)
  - Conversion pipeline configuration (multi-step conversions)
  - Resume capability for interrupted conversions (checkpointing)
  - Parallel batch processing with configurable concurrency
  - YAML configuration file support
  - Progress tracking and reporting
  - 9 comprehensive tests
- **Advanced error handling module added (v0.1.6)**
  - Graceful degradation for unsupported features
  - Partial conversion with detailed warnings
  - Configurable error recovery strategies (Skip, UseDefault, TryAlternative, AskUser, Abort)
  - Interactive error resolution with callbacks
  - Error pattern analysis with smart suggestions
  - ResilientConverter for fault-tolerant conversions
  - DetailedError with context, location, and severity
  - ErrorPatternAnalyzer for detecting common issues
  - 13 comprehensive tests
- **Transformation pipeline module added (v0.1.7)**
  - Custom transformation hooks for modifying statutes during conversion
  - Pre-processing plugins for source text manipulation
  - Post-processing plugins for output text refinement
  - Content normalization rules (whitespace, quotes, comments, case, regex)
  - Identifier mapping tables for renaming identifiers between formats
  - Conditional transformation logic with complex condition support
  - TransformationPipeline with builder pattern
  - TransformationSupport trait for LegalConverter integration
  - 19 comprehensive tests
- **Performance enhancements module added (v0.1.8)**
  - Lazy parsing for large documents with configurable chunk size
  - Memory-mapped file support for efficient large file handling
  - Persistent conversion cache with LRU eviction
  - Incremental re-conversion to avoid redundant work
  - Parallel parsing with work stealing (rayon-based)
  - HighPerformanceConverter combining all optimizations
  - LazyParser, MmapFileReader, PersistentCache, IncrementalConverter
  - ParallelParser for multi-core utilization (parallel feature)
  - 14 comprehensive tests
- **Integration modules added (v0.1.9)**
  - CLI tool for standalone conversion with command-line interface
  - REST API types and handlers for conversion service
  - Webhook notification system for conversion events
  - Comprehensive metrics and logging for conversion tracking
  - Document Management System (DMS) integration with file-based provider
  - 44 comprehensive tests (9 metrics, 8 CLI, 10 webhooks, 8 DMS, 9 REST API)
- **New format support modules added (v0.2.0)**
  - BPMN (Business Process Model and Notation) - OMG standard
  - DMN (Decision Model and Notation) - OMG standard
  - CMMN (Case Management Model and Notation) - OMG standard
  - RuleML (Rule Markup Language)
  - SBVR (Semantics of Business Vocabulary and Business Rules) - OMG standard
  - 9 comprehensive tests (5 BPMN, 1 DMN, 1 CMMN, 1 RuleML, 1 SBVR)
- **Contract format support modules added (v0.2.1)**
  - OpenLaw - Protocol for creating and executing legal agreements
  - Cicero - Accord Project smart legal contract templates (CiceroMark)
  - CommonForm - Format for legal forms and contracts (JSON-based)
  - Clause.io - Contract automation platform templates
  - ContractExpress - Document automation platform
  - 23 comprehensive tests (4 OpenLaw, 4 Cicero, 5 CommonForm, 5 Clause.io, 5 ContractExpress)
- **Legal XML Standards support modules added (v0.2.2)**
  - FORMEX - EU Official Journal format for European Union publications
  - NIEM - National Information Exchange Model for U.S. government data exchange
  - Enhanced LegalDocML support (already implemented)
  - CEN MetaLex support (already implemented)
  - 8 comprehensive tests (4 FORMEX, 4 NIEM)
- **Regulatory format support modules added (v0.2.3)**
  - FinReg - Financial Regulatory format for compliance rules
  - XBRL - eXtensible Business Reporting Language for financial reporting
  - RegML - Regulation Markup Language for regulatory provisions
  - MiFID II - Markets in Financial Instruments Directive II reporting
  - Basel III - International banking regulatory framework
  - 19 comprehensive tests (4 FinReg, 4 XBRL, 4 RegML, 3 MiFID II, 4 Basel III)
- **AI Format Converters module added (v0.2.4)**
  - LLM-assisted format detection with confidence scoring
  - AI-powered lossy conversion recovery
  - Semantic structure inference for unstructured legal text
  - Format migration suggestions with reasoning
  - Automated format documentation generator
  - 9 comprehensive tests
- **Streaming Conversion v2 module added (v0.2.5)**
  - Chunked conversion for large files with configurable chunk size
  - Parallel format processing for simultaneous multi-format conversion
  - Incremental conversion updates with modification tracking
  - Resumable conversion jobs with checkpoint/restore capabilities
  - Progress tracking with time estimation and throughput metrics
  - ChunkedConverter, ParallelFormatProcessor, IncrementalUpdater, ResumableJob, ProgressTracker
  - 18 comprehensive tests
- **Round-Trip Fidelity module added (v0.2.6)**
  - Lossless round-trip verification with detailed analysis
  - Multi-dimensional fidelity scoring (structure, metadata, semantic, syntax)
  - Conversion delta tracking to identify changes
  - Format capability matrices for understanding format limitations
  - Automatic fallback strategies for unsupported features
  - Format recommendation system based on capabilities
  - FidelityAnalyzer, FidelityScore, ConversionDelta, FormatCapabilityMatrix, FallbackConfig
  - 24 comprehensive tests
- **Format Validation module added (v0.2.7)**
  - Schema validation for XML and JSON formats (well-formedness checking)
  - Semantic validation rules for legal content (statute structure, completeness)
  - Cross-format consistency checking for comparing conversions
  - Custom validation plugin support for extensibility
  - Detailed validation reports with severity levels (Error, Warning, Info)
  - Validation statistics tracking for quality metrics
  - ValidationIssue, ValidationReport, FormatValidator, SchemaValidator, SemanticValidator, ConsistencyChecker
  - 24 comprehensive tests
- **Enterprise integration format support modules added (v0.2.8 - Complete)**
  - SAP Legal Module - Enterprise legal management system integration
  - Salesforce Contract - Salesforce CPQ contract management format
  - DocuSign - Electronic signature and digital transaction platform
  - MS Word Legal - Microsoft Word legal add-in format support
  - PDF Legal - Adobe PDF legal annotations and form fields
  - Contract terms, obligations, clauses, and signing workflows
  - Document metadata, responsible parties, routing order, and annotations
  - Form fields, signatures, legal categories, and conditional logic
  - 20 comprehensive tests (4 per format × 5 formats)
- **Blockchain format support modules added (v0.2.9 - Complete)**
  - Solidity - Ethereum smart contract language with NatSpec documentation
  - Vyper - Pythonic Ethereum smart contract language with decorators
  - Cadence - Flow blockchain resource-oriented programming language
  - Move - Aptos/Sui blockchain smart contract language with resource types
  - Blockchain documentation generator - Generate Markdown, HTML, JSON, NatSpec docs
  - Contract functions, modifiers, state variables, events, and access control
  - Resource ownership semantics, capabilities, and abilities
  - Smart contract legal annotation extraction and mapping
  - 37 comprehensive tests (4 Solidity + 5 Vyper + 5 Cadence + 5 Move + 5 blockchain docs + 13 integration)
- **Zero compiler warnings**
- **Zero clippy warnings (lib build)**
- **Clean release build**

## Roadmap for 0.1.0 Series

### New Format Support (v0.1.1) - COMPLETED
- [x] Add OASIS LegalCite import/export
- [x] Add CEN MetaLex support
- [x] Add MPEG-21 REL (Rights Expression Language)
- [x] Add Creative Commons license format
- [x] Add SPDX license expression format

### Conversion Quality (v0.1.2) - COMPLETED
- [x] Add semantic loss quantification (0-100%)
- [x] Add structure preservation scoring
- [x] Add metadata completeness analysis
- [x] Add round-trip fidelity testing
- [x] Add conversion confidence calibration

### Schema Support (v0.1.3) - COMPLETED
- [x] Add XML Schema validation during import
- [x] Add JSON Schema validation for outputs
- [x] Add custom schema extension points
- [x] Add schema migration utilities
- [x] Add schema compatibility checking

### Format Detection (v0.1.4) - COMPLETED
- [x] Add automatic format detection
- [x] Add encoding detection (UTF-8, UTF-16, etc.)
- [x] Add format version detection
- [x] Add mixed format handling
- [x] Add format recommendation based on content

### Batch Processing (v0.1.5) - COMPLETED
- [x] Add directory-based batch conversion
- [x] Add watch mode for continuous conversion
- [x] Add parallel multi-format export
- [x] Add conversion pipeline configuration
- [x] Add resume capability for interrupted conversions

### Error Handling (v0.1.6) - COMPLETED
- [x] Add graceful degradation for unsupported features
- [x] Add partial conversion with warnings
- [x] Add error recovery strategies
- [x] Add interactive error resolution
- [x] Add error pattern analysis

### Transformation Pipeline (v0.1.7) - COMPLETED
- [x] Add custom transformation hooks
- [x] Add pre/post processing plugins
- [x] Add content normalization rules
- [x] Add identifier mapping tables
- [x] Add conditional transformation logic

### Performance (v0.1.8) - COMPLETED
- [x] Add lazy parsing for large documents
- [x] Add memory-mapped file support
- [x] Add conversion result caching
- [x] Add incremental re-conversion
- [x] Add parallel parsing with work stealing

### Integration (v0.1.9) - COMPLETED
- [x] Add CLI tool for standalone conversion
- [x] Add REST API for conversion service
- [x] Add webhook notifications for conversions
- [x] Add conversion metrics and logging
- [x] Add integration with document management systems

## Roadmap for 0.2.0 Series

### New Format Support (v0.2.0) - COMPLETED
- [x] Add BPMN (Business Process Model) support
- [x] Implement DMN (Decision Model and Notation)
- [x] Add CMMN (Case Management Model)
- [x] Create RuleML bidirectional conversion
- [x] Add SBVR (Semantics of Business Vocabulary)

### Contract Formats (v0.2.1) - COMPLETED
- [x] Add OpenLaw format support
- [x] Implement Accord Project Cicero format
- [x] Add CommonForm format support
- [x] Create Clause.io template format
- [x] Add ContractExpress conversion

### Legal XML Standards (v0.2.2) - COMPLETED
- [x] Add LegalDocML (Akoma Ntoso 3.0) full support (already implemented)
- [x] Implement MetaLex conversion (already implemented)
- [x] Add CEN MetaLex format (already implemented)
- [x] Create FORMEX (EU Official Journal) support
- [x] Add NIEM (National Information Exchange) format

### Regulatory Formats (v0.2.3) ✅
- [x] Add FinReg (Financial Regulatory) format
- [x] Implement XBRL (eXtensible Business Reporting)
- [x] Add RegML (Regulation Markup Language)
- [x] Create MiFID II reporting format
- [x] Add Basel III compliance format

### AI Format Converters (v0.2.4) ✅
- [x] Add LLM-assisted format detection
- [x] Implement AI-powered lossy conversion recovery
- [x] Add semantic structure inference
- [x] Create format migration suggestions
- [x] Add automated format documentation

### Streaming Conversion (v0.2.5) ✅
- [x] Add chunked conversion for large files
- [x] Implement parallel format processing
- [x] Add incremental conversion updates
- [x] Create resumable conversion jobs
- [x] Add progress reporting and estimation

### Round-Trip Fidelity (v0.2.6) ✅
- [x] Add lossless round-trip verification
- [x] Implement fidelity scoring
- [x] Add conversion delta tracking
- [x] Create format capability matrices
- [x] Add automatic fallback strategies

### Format Validation (v0.2.7) ✅
- [x] Add schema validation for all formats
- [x] Implement semantic validation rules
- [x] Add cross-format consistency checking
- [x] Create custom validation plugins
- [x] Add validation report generation

### Enterprise Integration (v0.2.8) ✅
- [x] Add SAP legal module integration
- [x] Implement Salesforce contract format
- [x] Add DocuSign envelope conversion
- [x] Create Microsoft Word legal add-in format
- [x] Add Adobe PDF legal annotations

### Blockchain Format Support (v0.2.9) ✅
- [x] Add Solidity contract to legal format
- [x] Implement Cadence (Flow) conversion
- [x] Add Move (Aptos/Sui) legal mapping
- [x] Create Vyper legal annotation extraction
- [x] Add smart contract documentation generation
- **37 comprehensive tests** (4 Solidity + 5 Vyper + 5 Cadence + 5 Move + 18 blockchain docs)

## Roadmap for 0.3.0 Series (Next-Gen Features)

### Universal Legal Format (v0.3.0) ✅ Complete
- [x] Define universal legal interchange format (ULF v1.0.0)
- [x] Implement canonical form representation (UniversalLegalDocument with provisions, metadata, provenance)
- [x] Add format negotiation protocol (FormatNegotiator with compatibility scoring)
- [x] Create format evolution versioning (UlfVersion, VersionMigrator, UlfFeatures)
- [x] Add backward/forward compatibility layers (version validation, migration warnings)
- **Implementation Details**:
  - Created `universal_format.rs` module with 1289 lines
  - ULF v1.0.0 supports temporal validity, cross-references, provenance, extensions, and document structure
  - Format negotiator analyzes 40+ format compatibility with confidence scoring
  - Version system supports semantic versioning with migration framework
  - Added 21 comprehensive tests (6 ULF, 7 negotiation, 8 versioning)
  - All conversions can now use ULF as lossless intermediate format

### Real-Time Format Translation (v0.3.1) ✅ Complete
- [x] Add live document format translation
- [x] Implement streaming conversion APIs
- [x] Add collaborative format editing
- [x] Create real-time format synchronization
- [x] Add multi-format document views

## COMPLETED (2026-06-14 — real-time/collaborative conversion engines)

All five v0.3.1 items implemented as **pure-Rust, offline** engines in a new
`realtime/` module (the "real-time"/"collaborative" naming describes the
incremental, low-latency, conflict-free-convergence *capability*, not a
requirement for live networking — every engine is transport-agnostic and runs
with no server). Built only on stable `legalis-core` + the existing workspace
deps (`serde`, `serde_json`, `sha2`); no new dependency, no `scirs2`.

- **Shared backbone** (`realtime/mod.rs`): `CanonicalDocument` — an ordered,
  id-keyed collection of `DocumentRegion`s (one per `Statute`) with an
  O(log n) index; content-addressed `region_fingerprint` (domain-separated
  SHA-256 over canonical JSON) drives change detection; `RegionDelta`
  (added/updated/removed/reordered) diffs two snapshots; `DocumentChange`
  (Insert/Update/Remove/Move) is the serializable edit unit; `FormatPair`,
  `ChangeKind`. 9 tests.
- **Live document format translation** (`realtime/live_translate.rs`):
  `LiveTranslator` — delta-driven incremental translator with a per-region
  export cache keyed by fingerprint. `apply_change`/`apply_changes` re-translate
  *only* the regions that actually changed (revert-in-batch ⇒ zero work);
  `from_source` seeds from imported text; segmented output is reassembled in
  document order. Property test: incremental output == from-scratch
  translation. 7 tests.
- **Streaming conversion APIs** (`realtime/streaming_convert.rs`):
  `StreamingConverter` — chunked, bounded-memory format→format conversion with
  an explicit state machine (`Idle → Streaming → Flushing → Done`/`Failed`).
  `feed_chunk` accepts arbitrary byte windows and emits converted chunks at
  record boundaries; the buffer is flushed at a threshold and force-split at a
  hard cap, so peak memory is bounded regardless of document size
  (`StreamMetrics::peak_buffer_bytes`). `convert_str_chunked` convenience.
  7 tests (incl. mid-stream memory-bound assertion and chunked-vs-whole
  equivalence).
- **Collaborative format editing** (`realtime/collab.rs`): `CrdtDocument` — a
  CRDT merge engine (RGA-ordered sequence + per-field last-writer-wins
  registers + tombstones) over Lamport `Dot`s `(counter, replica)`. Inserts,
  updates, deletes, and moves are commutative + idempotent, giving
  **conflict-free convergence**: any two replicas observing the same operation
  set compute the same document. `state_digest` proves convergence. 8 tests:
  concurrent-insert convergence under reversed delivery, LWW update resolution,
  monotone delete vs. concurrent update, 3-replica convergence under 4 delivery
  permutations, concurrent-move convergence, idempotent redelivery,
  snapshot/replay equivalence.
- **Real-time format synchronization** (`realtime/sync.rs`): `FormatSyncEngine`
  — keeps two *different-format* representations in sync by importing an edited
  side, diffing it, replaying the delta as CRDT ops on a shared `CrdtDocument`,
  and re-exporting *both* sides. Bidirectional (`Endpoint::A`/`B`);
  `apply_remote_ops`/`snapshot_ops` are the hooks a transport would use;
  `with_replica_id` keeps two engines' CRDT identities distinct. 6 tests incl.
  two-engine convergence via op relay under concurrent edits.
- **Multi-format document views** (`realtime/views.rs`): `MultiFormatView` —
  projects one canonical document into N simultaneous format views that stay
  mutually consistent; each view is refreshed incrementally (one re-render per
  changed region per view); views can be added/removed at runtime and a
  late-added view immediately matches the canonical state. Consistency property
  test: every view == a from-scratch export of the canonical document. 6 tests.

- **Wiring**: `pub mod realtime;` added to `lib.rs` (now ~1007 lines, well under
  2000). Additive and backward-compatible — no existing API changed, no new
  `LegalFormat` variant (these engines compose the existing converter), so the
  exhaustive matches in `coverage.rs`/`enhanced.rs` are untouched.
- **Files** (all < 2000 lines): `realtime/mod.rs` (626), `collab.rs` (635),
  `live_translate.rs` (464), `streaming_convert.rs` (524), `sync.rs` (409),
  `views.rs` (344).
- **Tests**: 43 new (`700` total, up from `657`). `cargo nextest run -p
  legalis-interop` → **700 passed, 0 failed**. `cargo clippy -p legalis-interop
  --all-targets [--all-features] -- -D warnings` → **clean** (zero warnings).
  No `unwrap`/`expect`/`panic!`/`todo!`/`unimplemented!`/`unreachable!` in
  non-test code; tests use `expect` only.
- **Deferred**: a live *network transport* (WebSocket/QUIC peer relay) for the
  collaborative and synchronisation engines — the convergence logic
  (CRDT + bidirectional propagation) is fully implemented and exercised offline;
  only the wire shipping of `CrdtOp`s between physically-separate hosts is left,
  because that requires a networking stack out of scope for an offline,
  dependency-light interop crate. `CrdtOp`/`DocumentChange` are already
  `Serialize`/`Deserialize`, so a transport can be layered on without engine
  changes.

### AI-Native Formats (v0.3.2) ✅ Complete
- [x] Add LLM-native legal format
- [x] Implement embedding-based format
- [x] Add neural legal document format
- [x] Create attention-aware markup
- [x] Add semantic chunk format
- **Implementation Details** (completed 2026-06-14):
  - New module `formats_nextgen/` (mod + 5 submodules), all pure-Rust, scirs2-free,
    deterministic, and dependency-free beyond serde/serde_json:
    - `llm_native.rs` — `LlmNativeDocument`/`LlmBlock` with salience scoring and
      token-budget-aware prompt ordering (`render_prompt`); JSON-with-provenance.
    - `embedding.rs` — `EmbeddingDocument`/`EmbeddingRecord`/`RetrievalHit` with a
      deterministic feature-hashing embedder and cosine-similarity `search` (no
      external model).
    - `neural.rs` — `NeuralDocument`/`NeuralNode`/`NeuralEdge`; node salience via
      weighted PageRank over a semantic-similarity adjacency, plus derivation edges.
    - `attention.rs` — `AttentionDocument`/`AttentionUnit`/`AttentionSpan`;
      role-tagged spans with a softmax (TF-IDF) attention distribution and
      cross-references; inline `⟦role|a=…⟧` markup.
    - `semantic_chunk.rs` — `ChunkDocument`/`SemanticChunk`/`SemanticChunkConfig`;
      overlap-controlled RAG chunking with stable content-addressed IDs.
  - `mod.rs` shares `StructuredStatute` (lossless provenance backbone),
    `HashingEmbedder`, `cosine_similarity`, `softmax`, condition/effect codecs,
    `render_statute_markdown`, and token estimation.
  - 5 new `LegalFormat` variants (`LlmNative`, `Embedding`, `NeuralDocument`,
    `AttentionMarkup`, `SemanticChunk`) with extensions, `from_extension`, and
    importer/exporter registration; schema-tagged JSON enables auto-detection.
  - All five formats implement `FormatImporter`/`FormatExporter` with
    round-trippable serialize/parse via embedded provenance.
  - 53 new tests (11 shared utils + 7 LLM-native + 6 embedding + 8 neural +
    6 attention + 8 semantic-chunk + 7 converter-level integration in
    `ai_native_tests.rs`). Zero clippy warnings, no unwrap/expect in non-test code.

### Quantum-Safe Format Migration (v0.3.3) ✅ Complete
- [x] Add post-quantum signed formats
- [x] Implement quantum-resistant checksums
- [x] Add long-term preservation formats
- [x] Create format archival strategies
- [x] Add cryptographic agility support
- **Implementation Details** (completed 2026-06-14):
  - New module `future_proof/` (mod + 4 submodules), pure-Rust and `scirs2`-free,
    deterministic and dependency-light (reuses the workspace's `sha2` crate,
    enabled via `sha2.workspace = true`; no new workspace dependency):
    - `mod.rs` — shared primitives: SHA-256/512/512-256 wrappers, a
      length-prefixed domain-separated `tagged_hash`, constant-time comparison,
      and hex codecs (`to_hex`/`from_hex`/`from_hex_array`).
    - `checksum.rs` — `ChecksumAlgorithm` (SHA-256, SHA-512, SHA-512/256,
      iterated SHA-512 hardening, and a SHA-512‖SHA-256 concatenation combiner)
      plus `Checksum` with constant-time `verify`, `quantum_preimage_bits`
      (Grover bound), and redundant `compute_set`/`verify_set` helpers.
    - `hash_sig.rs` — a self-contained **Lamport one-time signature** with a
      hash-committed public-key fingerprint (secret keys derived from a seed via
      a PRF, so no `rand` dependency), lifted to a many-time `MerkleSigner`
      (XMSS-style Merkle tree) with one-time-use enforcement and authentication
      paths. Documented as a hash-based OTS, **not** a standardized PQ scheme.
    - `agility.rs` — `AlgorithmRegistry` of digest/signature/KEM descriptors
      (classical & quantum security bits, life-cycle status) with `recommended`
      and `migration_target`; a versioned `CryptoEnvelope` with
      `is_quantum_resistant`/`weaknesses`/`upgraded` (in-place scheme upgrade);
      `CryptoSuite` presets; `SignatureScheme` (hash-based implemented; ML-DSA /
      SLH-DSA / ML-KEM **deferred** as `AlgorithmStatus::Planned` — no heavy
      lattice dependency added).
    - `archive.rs` — `PreservationArchive`, a self-describing, BagIt-like
      container with a manifest, lossless `StructuredStatute` payload (reused
      from `formats_nextgen`), redundant fixity checksums, `MigrationRecord`
      history, an optional post-quantum hash-based signature, and a crypto
      envelope. Includes `verify_fixity`/`sign`/`verify_signature`,
      `to_bagit_files`/`from_bagit_files`, and `ArchivalStrategy`
      (minimal/standard/maximum-security presets) + `ArchivalPlan` dry-run
      planning with quantum-resistance warnings.
  - 1 new `LegalFormat` variant (`PreservationArchive`, extension `lpa.json`)
    with `extension`/`from_extension` and importer/exporter registration; the
    importer verifies fixity and signatures on import; schema-tagged JSON enables
    auto-detection. `enhanced.rs` and `coverage.rs` exhaustive matches updated.
  - 39 new tests (7 shared primitives + 6 checksum + 8 hash-signature +
    6 agility + 12 archive/strategy/converter-integration; BagIt round-trip uses
    `std::env::temp_dir()`). Zero clippy warnings (`-D warnings`, all features),
    no unwrap/expect/panic in non-test code.

### Cross-Reality Legal Formats (v0.3.4) ✅ Complete
- [x] Add VR/AR legal annotation format
- [x] Implement 3D legal document format
- [x] Add holographic legal display format
- [x] Create spatial legal markup
- [x] Add metaverse-native legal formats
- **Implementation Details** (completed 2026-06-14):
  - New module `cross_reality/` (mod + 5 submodules), pure-Rust, `scirs2`-free,
    deterministic, and dependency-free beyond serde/serde_json (no new workspace
    dependency). Reuses `formats_nextgen::StructuredStatute` as the lossless
    provenance backbone so every format round-trips the underlying `Statute` set.
    - `mod.rs` — shared spatial primitives: `Vec3`, `Quaternion` (axis-angle +
      Hamilton product), `Transform`, `Color` (+ hex), `Aabb`, `SpatialAnchor`
      with `AnchorKind` (world/marker/plane/geo/face/object), `SceneLayout`
      (grid/circle/helix/stack) with deterministic `layout_positions` /
      `layout_transform` and `face_target_yaw`, plus `effect_color`,
      `condition_salience`, `depth_parallax`, and `round3`.
    - `vr_ar.rs` — `VrArScene`/`AnnotationAnchor`; spatially-anchored, effect-
      coloured, salience-scaled, optionally-billboarded annotations with a
      Markdown body and visibility range; schema-tagged JSON.
    - `document_3d.rs` — `Scene3D`/`Node3D`/`SceneEdge`; a scene graph of statute
      panels with derivation edges and an `Aabb`, plus an X3D-like XML projection
      (`to_x3d`, XML-escaped, lossy visualisation view; JSON stays canonical).
    - `holographic.rs` — `HologramDisplay`/`DepthLayer`/`HologramElement` with
      `LightFieldParams`; salience-ordered depth-plane assignment, per-element
      ring-spiral placement, depth-derived parallax, and luminance.
    - `spatial_markup.rs` — `SpatialMarkupDocument`/`MarkupNode`; a compact,
      fully-parseable textual DSL (`#SLM/v1`) encoding per-node transforms,
      anchors, and the complete statute payload — lossless without a separate
      provenance blob (`to_markup`/`from_markup`).
    - `metaverse.rs` — `MetaverseScene`/`MetaverseEntity`/`Portal` with
      `WorldMetadata`, `EntityModel`/`EntityPrimitive`, effect-derived
      `InteractionVerb`s gated on preconditions, and lineage portals from
      derivation links; schema-tagged JSON.
  - 5 new `LegalFormat` variants (`VrArAnnotation`, `SpatialDocument3D`,
    `Holographic`, `SpatialMarkup`, `MetaverseLegal`) with `extension`,
    `from_extension`, importer/exporter registration, and full coverage in the
    exhaustive matches (`coverage.rs` analyzers, `enhanced.rs` normalization).
    Schema-tagged JSON (and the `#SLM` header) enable auto-detection.
  - 42 new tests (11 shared primitives + 4 VR/AR + 5 3D-document + 5 holographic
    + 6 spatial-markup + 5 metaverse + 6 converter-level integration in
    `lib.rs`), all including round-trip coverage. Zero clippy warnings
    (`-D warnings`, all features), no unwrap/expect/panic in non-test code.
- **Deferred**: `Create spatial legal markup`'s nested/hierarchical grouping and
  `metaverse-native`'s live avatar presence/networking are out of scope for an
  offline serializer; the data model is implemented and the live transport is
  left for a future real-time batch.
