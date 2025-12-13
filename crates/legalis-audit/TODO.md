# legalis-audit TODO

## Completed

- [x] Audit record structure with UUID, timestamp, actor
- [x] Hash chain integrity for tamper detection
- [x] Decision context and result recording
- [x] In-memory audit trail storage
- [x] Basic integrity verification

## Storage

### Backends
- [ ] Add SQLite storage backend
- [ ] Implement PostgreSQL storage
- [ ] Add file-based JSON/JSONL storage
- [ ] Support S3-compatible object storage
- [ ] Implement append-only log storage

### Features
- [ ] Add storage encryption at rest
- [ ] Implement record compression
- [ ] Add retention policy support
- [ ] Create archival functionality
- [ ] Support log rotation

## Integrity

- [ ] Add Merkle tree for efficient verification
- [ ] Implement witness signatures (external notarization)
- [ ] Add timestamping authority integration
- [ ] Create blockchain anchoring option
- [ ] Implement multi-party verification

## Querying

- [ ] Add flexible query API
- [ ] Implement date range queries
- [ ] Add actor/subject filtering
- [ ] Create statute-based filtering
- [ ] Support complex query expressions

## Reporting

### Formats
- [ ] Generate PDF audit reports
- [ ] Create Excel/CSV exports
- [ ] Implement HTML report generation
- [ ] Add JSON-LD audit trail export

### Analysis
- [ ] Add decision pattern analysis
- [ ] Implement anomaly detection
- [ ] Create decision distribution reports
- [ ] Add trend analysis over time
- [ ] Generate compliance summary reports

## Compliance

- [ ] Add GDPR compliance features
- [ ] Implement data subject access requests
- [ ] Create right-to-explanation support
- [ ] Add audit log export for regulators
- [ ] Implement data minimization options

## Decision Replay

- [ ] Add decision replay capability
- [ ] Implement point-in-time reconstruction
- [ ] Create what-if analysis on historical data
- [ ] Add decision comparison tools

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

- [ ] Add tamper detection tests
- [ ] Create high-volume insertion benchmarks
- [ ] Test concurrent access patterns
- [ ] Add storage backend tests
