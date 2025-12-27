# legalis-interop TODO

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

**Total Test Coverage**: 169 tests (with all features enabled)
- Default features: 164 tests
- Async feature: 169 tests
- All features: 169 tests
- **Zero compiler warnings**
- **Zero clippy warnings**
- **Clean release build**
