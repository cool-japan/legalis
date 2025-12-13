# legalis-dsl TODO

## Completed

- [x] Add IMPORT statement for cross-statute references
- [x] Add EFFECTIVE_DATE and EXPIRY_DATE clauses
- [x] Add JURISDICTION specifier
- [x] Add VERSION specifier
- [x] Implement AND/OR operators in condition parsing
- [x] Add NOT operator support
- [x] Support parentheses for grouping conditions
- [x] Add single-line comment support (//)
- [x] Add multi-line comment support (/* */)
- [x] Add source location tracking (line, column)
- [x] Create pretty-printer (AST -> DSL text)
- [x] Add multi-statute document parsing
- [x] Add HAS keyword for attribute checks

## Parser Features

### Grammar Extensions
- [ ] Add EXCEPTION clause for handling edge cases
- [ ] Support DEFAULT clause for missing attributes
- [ ] Add AMENDMENT clause for version tracking
- [ ] Add BETWEEN operator for ranges
- [ ] Add IN operator for set membership

### Advanced Conditions
- [ ] Add date comparison conditions
- [ ] Add string pattern matching (LIKE)
- [ ] Add numeric range conditions

## AST Improvements

- [ ] Implement AST visitor pattern
- [ ] Add AST transformation utilities
- [ ] Implement AST diffing for change detection
- [ ] Add AST optimization passes

## Error Handling

- [ ] Add error recovery for partial parsing
- [ ] Improve error messages with suggestions
- [ ] Add "did you mean?" suggestions for typos
- [ ] Create error spans for IDE integration
- [ ] Add warning system for deprecated syntax

## Tooling

- [ ] Create syntax highlighting definitions (VSCode, vim)
- [ ] Add LSP (Language Server Protocol) support
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
