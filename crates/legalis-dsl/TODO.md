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
- [x] Add EXCEPTION clause for handling edge cases
- [x] Support DEFAULT clause for missing attributes
- [x] Add AMENDMENT clause for version tracking
- [x] Add BETWEEN operator for ranges
- [x] Add IN operator for set membership
- [x] Add date comparison conditions
- [x] Add string pattern matching (LIKE)
- [x] Implement AST visitor pattern

## Parser Features

### Grammar Extensions
- [x] Add WHEN clause for temporal conditions
- [x] Add UNLESS clause for negative conditions
- [x] Add REQUIRES clause for dependencies

### Advanced Conditions
- [x] Add numeric range conditions with custom operators
- [x] Add regex pattern matching support
- [x] Add set operations (UNION, INTERSECT, DIFFERENCE) - AST and parsing infrastructure

## AST Improvements

- [x] Add AST transformation utilities
- [x] Add AST serialization (to JSON)
- [x] Add AST serialization to YAML
- [x] Add AST optimization passes (flatten, deduplicate, simplify, normalize)
- [x] Implement AST diffing for change detection

## Error Handling

- [x] Improve error messages with suggestions
- [x] Add "did you mean?" suggestions for typos (Levenshtein distance)
- [x] Create SyntaxError with context (expected vs found)
- [x] Create UndefinedReference error with hints
- [ ] Add error recovery for partial parsing
- [x] Create error spans for IDE integration
- [x] Add warning system for deprecated syntax

## Tooling

- [x] Create syntax highlighting definitions (VSCode, vim)
- [ ] Add LSP (Language Server Protocol) support
- [x] Add REPL for interactive parsing
- [ ] Create schema/grammar documentation generator

## Performance

- [ ] Benchmark parser performance
- [ ] Optimize tokenizer for large documents
- [ ] Add incremental parsing support
- [ ] Implement parse result caching

## Testing

- [ ] Add corpus of real-world legal document examples
- [x] Add fuzzing for parser robustness
- [ ] Test error message quality
- [x] Add benchmark suite
