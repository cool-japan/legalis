# legalis-registry TODO

## Completed

- [x] In-memory statute storage with versioning
- [x] Tag-based organization
- [x] Jurisdiction indexing
- [x] Basic CRUD operations
- [x] Version history tracking

## Storage

- [x] Add SQLite backend
- [x] Add PostgreSQL backend
- [x] Implement connection pooling
- [x] Add backup and restore functionality
- [x] Support for transactions (database transactions via sqlx)

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
- [x] Import/export in Akoma Ntoso format
- [x] Statute dependency tracking (enhanced with dependency graphs)
- [x] Transaction support for batch operations

## API

- [x] Add async API variants
- [x] Implement streaming for large result sets
- [x] Add pagination support
- [x] Create GraphQL interface

## Testing

- [x] Add integration tests with real databases (via storage backend tests)
- [x] Add performance benchmarks
- [x] Test concurrent access patterns
