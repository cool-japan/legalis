# legalis-registry TODO

## Completed

- [x] In-memory statute storage with versioning
- [x] Tag-based organization
- [x] Jurisdiction indexing
- [x] Basic CRUD operations
- [x] Version history tracking

## Storage

- [ ] Add SQLite backend
- [ ] Add PostgreSQL backend
- [ ] Implement connection pooling
- [x] Add backup and restore functionality
- [ ] Support for transactions

## Search

- [x] Full-text search across statutes
- [x] Fuzzy matching for statute IDs
- [x] Search by condition types
- [x] Search by effect types
- [x] Advanced query language

## Performance

- [x] Add caching layer (LRU cache)
- [x] Implement optimistic concurrency control
- [x] Add batch operations
- [x] Implement lazy loading for large statutes

## Features

- [x] Event sourcing for complete change history
- [x] Webhook notifications for statute changes
- [x] Multi-tenant support for isolated registries
- [ ] Import/export in Akoma Ntoso format
- [x] Statute dependency tracking (enhanced with dependency graphs)

## API

- [ ] Add async API variants
- [ ] Implement streaming for large result sets
- [x] Add pagination support
- [ ] Create GraphQL interface

## Testing

- [ ] Add integration tests with real databases
- [x] Add performance benchmarks
- [ ] Test concurrent access patterns
