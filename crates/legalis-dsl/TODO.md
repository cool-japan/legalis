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
- [x] Add error recovery for partial parsing
- [x] Create error spans for IDE integration
- [x] Add warning system for deprecated syntax

## Tooling

- [x] Create syntax highlighting definitions (VSCode, vim)
- [x] Add LSP (Language Server Protocol) support
- [x] Add REPL for interactive parsing
- [x] Create schema/grammar documentation generator

## Performance

- [x] Benchmark parser performance
- [x] Optimize tokenizer for large documents
- [x] Add incremental parsing support
- [x] Implement parse result caching

## Testing

- [x] Add corpus of real-world legal document examples
- [x] Add fuzzing for parser robustness
- [x] Test error message quality
- [x] Add benchmark suite

## LSP Enhancements (2025-12-20)

### Advanced Features
- [x] Add code actions for quick fixes (auto-fix deprecated syntax)
- [x] Add document formatting support using AST pretty-printer
- [x] Implement AST formatter for LegalDocument nodes
- [x] Add integration tests for LSP features

### Code Quality
- [x] Thread-safe LSP backend with async/await
- [x] Support for workspace edits in code actions
- [x] Full document formatting with TextEdit generation

## Advanced Analysis & Tooling (2025-12-25)

### Semantic Validation
- [x] Add semantic validation module for statute verification
- [x] Implement circular dependency detection in REQUIRES clauses
- [x] Add undefined reference checking for statute IDs
- [x] Validate numeric ranges (min < max in BETWEEN conditions)
- [x] Check for self-references in REQUIRES and SUPERSEDES
- [x] Validate amendment targets exist
- [x] Add completeness checker for required fields

### Query API
- [x] Implement fluent query API for filtering statutes
- [x] Add jurisdiction, version, and date filtering
- [x] Support filtering by title patterns
- [x] Add REQUIRES/SUPERSEDES relationship queries
- [x] Implement condition search within statutes
- [x] Add count() and exists() helper methods

### Visualization
- [x] Create tree-view formatter for statute structure
- [x] Add color-coded terminal output support
- [x] Hierarchical display of conditions, effects, and metadata
- [x] Support for imports, amendments, and exceptions visualization