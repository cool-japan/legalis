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

## New Enhancements (2025-12-26)

### Serialization & I/O
- [x] Add TOML serialization/deserialization support
- [x] Add TOML format validation

### Graph Generation
- [x] Create dependency graph generator (REQUIRES/SUPERSEDES relationships)
- [x] Add GraphViz DOT format output
- [x] Add Mermaid diagram format output
- [x] Generate visual statute dependency graphs
- [x] Detect cycles in dependency graphs

### Analytics & Metrics
- [x] Create statistics module for statute analysis
- [x] Add complexity metrics (condition depth, effect count)
- [x] Count statutes, conditions, effects by type
- [x] Generate reports on statute relationships
- [x] Add dependency analysis (independent/leaf statutes)

### Import Resolution
- [x] Add import path validation
- [x] Implement import path resolution
- [x] Support relative and absolute import paths
- [x] Detect circular import dependencies
- [x] Add document caching for import resolution

## Advanced Features (2025-12-26)

### AST Transformation Pipeline
- [x] Create composable transformation pipeline
- [x] Add transformation combinators (sequence, parallel, conditional)
- [x] Implement reversible transformations with undo support
- [x] Add transformation validation and verification
- [x] Create preset transformation recipes for common patterns

### Code Generation Framework
- [x] Design pluggable code generator architecture
- [x] Implement SQL DDL/DQL generator for statute rules
- [x] Add Prolog predicate generator
- [x] Create Python function generator
- [ ] Support custom templates for code generation
- [x] Add roundtrip testing for generated code

### Advanced Semantic Analysis
- [x] Type inference and checking for condition values
- [ ] Data flow analysis for statute dependencies
- [ ] Taint analysis for security-sensitive attributes
- [x] Dead code detection (unreachable effects)
- [x] Value range analysis for numeric conditions
- [ ] Consistency checking across related statutes

### Performance & Profiling
- [ ] Add detailed performance profiling utilities
- [ ] Implement parse time breakdown by component
- [ ] Create memory usage profiler
- [ ] Add benchmark comparison tools
- [ ] Optimize hot paths identified by profiling

### Statute Templates & Macros
- [ ] Design template/macro system for common patterns
- [ ] Support parameterized statute templates
- [ ] Add template expansion and instantiation
- [ ] Create standard template library
- [ ] Implement template validation

### Documentation Generation
- [ ] Generate HTML documentation from AST
- [ ] Create PDF export using LaTeX
- [x] Add Markdown documentation generator
- [x] Generate statute dependency diagrams
- [x] Create cross-reference tables
- [ ] Add search index generation

### Metadata & Extraction
- [ ] Extract jurisdiction hierarchy
- [ ] Build temporal version history
- [ ] Generate compliance matrices
- [ ] Extract entity relationships
- [ ] Create audit trail from amendments

### Testing & Quality
- [x] Add property-based testing with proptest
- [ ] Create mutation testing framework
- [ ] Add coverage-guided fuzzing
- [x] Implement snapshot testing for AST
- [x] Create test case generators

### Integration & Tooling
- [ ] Add GitHub Actions workflow support
- [ ] Create pre-commit hooks for validation
- [ ] Add VS Code extension improvements
- [ ] Implement watch mode for continuous validation
- [ ] Create diff tool for statute comparison