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
- [ ] Preserve legal article references during conversion
- [ ] Support for Catala's exception handling
- [ ] Handle scope inheritance

### Stipula
- [ ] Convert state machines to condition logic
- [ ] Support for temporal obligations
- [ ] Handle asset transfer semantics

### L4
- [ ] Convert decision tables
- [ ] Support for L4's temporal operators
- [ ] Handle L4's default logic

### Standard Formats
- [ ] LegalRuleML import/export
- [ ] LKIF (Legal Knowledge Interchange Format)
- [ ] LegalDocML support

## Quality Assurance

- [ ] Semantic preservation validation
- [ ] Coverage reports for format features
- [ ] Batch conversion support
- [ ] Diff-aware incremental conversion

## Performance

- [ ] Optimize large document conversion
- [ ] Add conversion caching
- [ ] Parallel conversion support

## Testing

- [ ] Add comprehensive test suites per format
- [ ] Test edge cases and error handling
- [ ] Benchmark conversion performance
