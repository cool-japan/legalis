# legalis-dsl TODO

## Parser Features

### Grammar Extensions
- [ ] Add IMPORT statement for cross-statute references
- [ ] Add EXCEPTION clause for handling edge cases
- [ ] Add EFFECTIVE_DATE and EXPIRY_DATE clauses
- [ ] Support DEFAULT clause for missing attributes
- [ ] Add JURISDICTION specifier

### Operators
- [ ] Implement AND/OR operators in condition parsing
- [ ] Add NOT operator support
- [ ] Support parentheses for grouping conditions
- [ ] Add BETWEEN operator for ranges
- [ ] Add IN operator for set membership

### Comments
- [ ] Add single-line comment support (//)
- [ ] Add multi-line comment support (/* */)
- [ ] Add documentation comments for metadata

## AST Improvements

- [ ] Add source location tracking (line, column)
- [ ] Implement AST visitor pattern
- [ ] Add AST transformation utilities
- [ ] Create pretty-printer (AST -> DSL text)
- [ ] Implement AST diffing for change detection

## Error Handling

- [ ] Add error recovery for partial parsing
- [ ] Improve error messages with suggestions
- [ ] Add "did you mean?" suggestions for typos
- [ ] Create error spans for IDE integration
- [ ] Add warning system for deprecated syntax

## Tooling

- [ ] Create syntax highlighting definitions (VSCode, vim)
- [ ] Add LSP (Language Server Protocol) support
- [ ] Create formatter/linter tool
- [ ] Add REPL for interactive parsing
- [ ] Create schema/grammar documentation generator

## Performance

- [ ] Benchmark parser performance
- [ ] Optimize tokenizer for large documents
- [ ] Add incremental parsing support
- [ ] Implement parse result caching

## Testing

- [ ] Add corpus of real-world legal document examples
- [ ] Add fuzzing for parser robustness
- [ ] Test error message quality
- [ ] Add benchmark suite
