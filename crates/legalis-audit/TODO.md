# legalis-audit TODO

## Completed

### Core Features
- [x] Audit record structure with UUID, timestamp, actor
- [x] Hash chain integrity for tamper detection
- [x] Decision context and result recording
- [x] In-memory audit trail storage
- [x] Basic integrity verification

### Storage System
- [x] Storage abstraction trait (AuditStorage)
- [x] In-memory storage backend (MemoryStorage)
- [x] JSONL file-based storage backend with persistence
- [x] SQLite storage backend with full indexing and transactions
- [x] Encrypted storage wrapper with AES-256-GCM
- [x] Flexible storage backend selection

### Query System
- [x] QueryBuilder with builder pattern
- [x] Statute ID filtering
- [x] Subject ID filtering
- [x] Event type filtering
- [x] Actor filtering (System, User, External with roles)
- [x] Date range queries
- [x] Pagination support (limit/offset)

### Export Functionality
- [x] CSV export
- [x] JSON export
- [x] JSON-LD export with semantic web compatibility
- [x] Excel export with formatting (.xlsx)
- [x] PDF report generation with compliance summary
- [x] HTML report generation with responsive design

### Analysis & Reporting
- [x] Decision pattern analysis
- [x] Anomaly detection (volume spikes, override patterns)
- [x] Distribution reports (statute, actor, event type, result)
- [x] Temporal distribution analysis
- [x] Compliance summary reports

### Decision Replay
- [x] Point-in-time reconstruction
- [x] Subject history tracking
- [x] Statute history tracking
- [x] Timeline comparison
- [x] What-if analysis

### Integrity & Security
- [x] Hash chain integrity verification
- [x] Merkle tree for efficient O(log n) verification
- [x] Merkle proof generation and validation
- [x] Batch verification support
- [x] AES-256-GCM encryption at rest
- [x] Secure key management and derivation
- [x] Record compression with DEFLATE (multiple levels)

### GDPR Compliance
- [x] Data subject access requests (Article 15)
- [x] Right to explanation (Article 22)
- [x] Retention policies with exemptions
- [x] Erasure analysis (right to be forgotten)

### Testing
- [x] Comprehensive unit tests for all modules (58 tests)
- [x] Storage backend tests (memory, JSONL, SQLite, encrypted)
- [x] Query builder tests
- [x] Export functionality tests (CSV, JSON, JSON-LD, Excel, PDF, HTML)
- [x] Integrity verification tests
- [x] Analysis tests
- [x] Replay tests
- [x] Retention policy tests
- [x] Encryption/decryption tests
- [x] Merkle tree tests
- [x] Compression tests (multiple levels, batch operations)
- [x] Zero warnings policy maintained (cargo clippy clean)

## Storage

### Backends
- [x] Add file-based JSON/JSONL storage
- [x] Add SQLite storage backend with full indexing
- [x] Implement PostgreSQL storage
- [ ] Support S3-compatible object storage
- [ ] Implement append-only log storage

### Features
- [x] Add retention policy support (with exemptions)
- [x] Add storage encryption at rest (AES-256-GCM)
- [x] Implement record compression (DEFLATE with multiple compression levels)
- [x] Create archival functionality (with compression, time-based policies, and integrity verification)
- [ ] Support log rotation

## Integrity

- [x] Add Merkle tree for efficient verification
- [ ] Implement witness signatures (external notarization)
- [ ] Add timestamping authority integration
- [ ] Create blockchain anchoring option
- [ ] Implement multi-party verification

## Querying

- [x] Add flexible query API (QueryBuilder with builder pattern)
- [x] Implement date range queries
- [x] Add actor/subject filtering
- [x] Create statute-based filtering
- [x] Support complex query expressions (via QueryBuilder)

## Reporting

### Formats
- [x] Generate PDF audit reports with compliance summary
- [x] Create CSV exports
- [x] Create Excel exports with formatting
- [x] Implement HTML report generation with responsive design
- [x] Add JSON-LD audit trail export
- [x] Add JSON export

### Analysis
- [x] Add decision pattern analysis (DecisionAnalyzer)
- [x] Implement anomaly detection (volume spikes, override patterns)
- [x] Create decision distribution reports (by statute, actor, event type, result)
- [x] Add trend analysis over time (temporal distribution)
- [x] Generate compliance summary reports (ComplianceReport)

## Compliance

- [x] Add GDPR compliance features (retention module)
- [x] Implement data subject access requests (GDPR Article 15)
- [x] Create right-to-explanation support (GDPR Article 22)
- [x] Add erasure analysis (right to be forgotten)
- [x] Implement retention policies with exemptions
- [x] Add audit log export for regulators (StandardCSV, DetailedJSON, XML, GDPR, SOX, HIPAA formats)
- [x] Implement data minimization options (Redact, Pseudonymize, Remove strategies with auto-policies)

## Decision Replay

- [x] Add decision replay capability (DecisionReplayer)
- [x] Implement point-in-time reconstruction
- [x] Create what-if analysis on historical data
- [x] Add decision comparison tools (timeline comparison)
- [x] Add subject history tracking
- [x] Add statute history tracking

## Integration

- [x] Add webhook notifications for new records (async, retry support, event filtering)
- [ ] Implement SIEM integration (syslog, CEF)
- [ ] Create Elasticsearch export
- [ ] Add OpenTelemetry tracing integration

## Performance

- [x] Add async write batching (configurable batch size and delay, tokio-based)
- [x] Implement read caching (LRU cache with TTL, statistics, configurable size)
- [ ] Optimize hash chain verification
- [x] Add background integrity checking (periodic daemon, manual triggers, error reporting)

## Testing

- [x] Add tamper detection tests (verify_integrity tests)
- [x] Create high-volume insertion benchmarks (criterion-based, 10-10000 records)
- [x] Test concurrent access patterns (concurrent writes, reads/writes, queries, high contention)
- [x] Add storage backend tests (memory, JSONL, SQLite, encrypted, cached)

## Roadmap for 0.1.0 Series

### Storage Enhancements (v0.1.1)
- [ ] Add S3-compatible object storage backend
- [ ] Add append-only log storage for forensic analysis
- [ ] Add partitioned storage by date/jurisdiction
- [ ] Add automatic tier migration (hot → warm → cold)
- [ ] Add storage compression with configurable algorithms

### Advanced Querying (v0.1.2)
- [ ] Add full-text search across decision context
- [ ] Add aggregate queries (count by statute, by outcome)
- [ ] Add time-series queries for trend analysis
- [ ] Add join queries across multiple audit trails
- [ ] Add query plan explanation

### Integrity Features (v0.1.3)
- [ ] Add witness signatures (external notarization)
- [ ] Add timestamping authority integration (RFC 3161)
- [ ] Add blockchain anchoring (Bitcoin, Ethereum)
- [ ] Add multi-party verification (threshold signatures)
- [ ] Add tamper-evident sealed audit logs

### Compliance Extensions (v0.1.4)
- [ ] Add CCPA compliance features
- [ ] Add HIPAA audit requirements
- [ ] Add SOX compliance reporting
- [ ] Add ISO 27001 audit trail requirements
- [ ] Add configurable retention policies per regulation

### Analysis & Intelligence (v0.1.5)
- [ ] Add ML-based anomaly detection
- [ ] Add decision clustering analysis
- [ ] Add bias detection in decisions
- [ ] Add outcome prediction based on patterns
- [ ] Add what-if analysis for historical decisions

### Integration (v0.1.6)
- [ ] Add webhook notifications for new records
- [ ] Add SIEM integration (syslog, CEF, LEEF)
- [ ] Add Elasticsearch export for analytics
- [ ] Add OpenTelemetry tracing integration
- [ ] Add Slack/Teams notifications for anomalies

### Reporting Enhancements (v0.1.7)
- [ ] Add scheduled report generation
- [ ] Add custom report templates
- [ ] Add report delivery (email, S3, webhook)
- [ ] Add interactive HTML reports with filters
- [ ] Add comparison reports (month-over-month)

### Performance (v0.1.8)
- [ ] Add async write batching
- [ ] Add read caching with invalidation
- [ ] Add parallel integrity verification
- [ ] Add background integrity checking daemon
- [ ] Add bloom filter for quick record existence checks

### Forensic Features (v0.1.9)
- [ ] Add chain-of-custody tracking
- [ ] Add digital evidence packaging
- [ ] Add court-admissible export format
- [ ] Add timeline reconstruction tools
- [ ] Add decision lineage visualization
