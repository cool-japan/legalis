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

## Roadmap for 0.1.0 Series

### New Format Support (v0.1.1)
- [ ] Add OASIS LegalCite import/export
- [ ] Add CEN MetaLex support
- [ ] Add MPEG-21 REL (Rights Expression Language)
- [ ] Add Creative Commons license format
- [ ] Add SPDX license expression format

### Conversion Quality (v0.1.2)
- [ ] Add semantic loss quantification (0-100%)
- [ ] Add structure preservation scoring
- [ ] Add metadata completeness analysis
- [ ] Add round-trip fidelity testing
- [ ] Add conversion confidence calibration

### Schema Support (v0.1.3)
- [ ] Add XML Schema validation during import
- [ ] Add JSON Schema validation for outputs
- [ ] Add custom schema extension points
- [ ] Add schema migration utilities
- [ ] Add schema compatibility checking

### Format Detection (v0.1.4)
- [ ] Add automatic format detection
- [ ] Add encoding detection (UTF-8, UTF-16, etc.)
- [ ] Add format version detection
- [ ] Add mixed format handling
- [ ] Add format recommendation based on content

### Batch Processing (v0.1.5)
- [ ] Add directory-based batch conversion
- [ ] Add watch mode for continuous conversion
- [ ] Add parallel multi-format export
- [ ] Add conversion pipeline configuration
- [ ] Add resume capability for interrupted conversions

### Error Handling (v0.1.6)
- [ ] Add graceful degradation for unsupported features
- [ ] Add partial conversion with warnings
- [ ] Add error recovery strategies
- [ ] Add interactive error resolution
- [ ] Add error pattern analysis

### Transformation Pipeline (v0.1.7)
- [ ] Add custom transformation hooks
- [ ] Add pre/post processing plugins
- [ ] Add content normalization rules
- [ ] Add identifier mapping tables
- [ ] Add conditional transformation logic

### Performance (v0.1.8)
- [ ] Add lazy parsing for large documents
- [ ] Add memory-mapped file support
- [ ] Add conversion result caching
- [ ] Add incremental re-conversion
- [ ] Add parallel parsing with work stealing

### Integration (v0.1.9)
- [ ] Add CLI tool for standalone conversion
- [ ] Add REST API for conversion service
- [ ] Add webhook notifications for conversions
- [ ] Add conversion metrics and logging
- [ ] Add integration with document management systems
