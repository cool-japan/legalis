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

## Recent Enhancements (2025-12-19)

### Session 1: Builder Methods & Doctests
- [x] Fixed doctests (changed `len()` to `count()`)
- [x] Added comprehensive builder methods for `StatuteEntry`:
  - `with_expiry_date()` - Set expiry date
  - `with_amends()` - Set parent statute for amendments
  - `with_supersedes()` - Add superseded statutes
  - `with_metadata()` - Add metadata key-value pairs
  - `with_jurisdiction()` - Override jurisdiction
- [x] Added comprehensive test for all builder methods

### Session 2: Pagination & Utility Methods
- [x] Enhanced `Pagination` with convenience methods:
  - `first()` - Create pagination for first page
  - `next()` / `prev()` - Navigate between pages
  - `with_page()` / `with_per_page()` - Builder methods
- [x] Enhanced `PagedResult<T>` with navigation helpers:
  - `has_next()` / `has_prev()` - Check for adjacent pages
  - `is_empty()` / `len()` - Check result size
  - `first_item_number()` / `last_item_number()` - Global item indices
  - `next_page()` / `prev_page()` - Get pagination for adjacent pages
- [x] Added utility methods to `StatuteRegistry`:
  - `all_statute_ids()` - Get all statute IDs
  - `contains()` - Check if statute exists
  - `get_many()` - Batch get multiple statutes
  - `latest_version()` - Get latest version number
  - `statistics()` - Get comprehensive registry statistics
- [x] Added `RegistryStatistics` struct with detailed metrics
- [x] Added `Hash` derive to `StatuteStatus` for HashMap usage
- [x] All tests passing (75 tests, +4 new tests)
- [x] Zero warnings policy maintained (clippy, build, tests)
