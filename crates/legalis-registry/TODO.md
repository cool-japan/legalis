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
- [ ] Add backup and restore functionality
- [ ] Support for transactions

## Search

- [ ] Full-text search across statutes
- [ ] Fuzzy matching for statute IDs
- [ ] Search by condition types
- [ ] Search by effect types
- [ ] Advanced query language

## Performance

- [ ] Add caching layer (LRU cache)
- [ ] Implement optimistic concurrency control
- [ ] Add batch operations
- [ ] Implement lazy loading for large statutes

## Features

- [ ] Event sourcing for complete change history
- [ ] Webhook notifications for statute changes
- [ ] Multi-tenant support for isolated registries
- [ ] Import/export in Akoma Ntoso format
- [ ] Statute dependency tracking

## API

- [ ] Add async API variants
- [ ] Implement streaming for large result sets
- [ ] Add pagination support
- [ ] Create GraphQL interface

## Testing

- [ ] Add integration tests with real databases
- [ ] Add performance benchmarks
- [ ] Test concurrent access patterns
