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

### GDPR Compliance
- [x] Data subject access requests (Article 15)
- [x] Right to explanation (Article 22)
- [x] Retention policies with exemptions
- [x] Erasure analysis (right to be forgotten)

### Testing
- [x] Comprehensive unit tests for all modules (30 tests)
- [x] Storage backend tests
- [x] Query builder tests
- [x] Export functionality tests
- [x] Integrity verification tests
- [x] Analysis tests
- [x] Replay tests
- [x] Retention policy tests
- [x] Zero warnings policy maintained

## Storage

### Backends
- [x] Add file-based JSON/JSONL storage
- [ ] Add SQLite storage backend (trait defined, implementation pending)
- [ ] Implement PostgreSQL storage
- [ ] Support S3-compatible object storage
- [ ] Implement append-only log storage

### Features
- [x] Add retention policy support (with exemptions)
- [ ] Add storage encryption at rest
- [ ] Implement record compression
- [ ] Create archival functionality
- [ ] Support log rotation

## Integrity

- [ ] Add Merkle tree for efficient verification
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
- [ ] Generate PDF audit reports
- [x] Create CSV exports
- [ ] Create Excel exports
- [ ] Implement HTML report generation
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
- [ ] Add audit log export for regulators
- [ ] Implement data minimization options

## Decision Replay

- [x] Add decision replay capability (DecisionReplayer)
- [x] Implement point-in-time reconstruction
- [x] Create what-if analysis on historical data
- [x] Add decision comparison tools (timeline comparison)
- [x] Add subject history tracking
- [x] Add statute history tracking

## Integration

- [ ] Add webhook notifications for new records
- [ ] Implement SIEM integration (syslog, CEF)
- [ ] Create Elasticsearch export
- [ ] Add OpenTelemetry tracing integration

## Performance

- [ ] Add async write batching
- [ ] Implement read caching
- [ ] Optimize hash chain verification
- [ ] Add background integrity checking

## Testing

- [x] Add tamper detection tests (verify_integrity tests)
- [ ] Create high-volume insertion benchmarks
- [ ] Test concurrent access patterns
- [x] Add storage backend tests (memory and JSONL)
