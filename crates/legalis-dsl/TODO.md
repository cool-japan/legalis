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
- [x] Support custom templates for code generation (via template system)
- [x] Add roundtrip testing for generated code

### Advanced Semantic Analysis
- [x] Type inference and checking for condition values
- [ ] Data flow analysis for statute dependencies
- [ ] Taint analysis for security-sensitive attributes
- [x] Dead code detection (unreachable effects)
- [x] Value range analysis for numeric conditions
- [ ] Consistency checking across related statutes

### Performance & Profiling (2025-12-27)
- [x] Add detailed performance profiling utilities
- [x] Implement parse time breakdown by component
- [x] Create memory usage profiler
- [x] Add benchmark comparison tools
- [x] Optimize hot paths identified by profiling

### Statute Templates & Macros (2025-12-27)
- [x] Design template/macro system for common patterns
- [x] Support parameterized statute templates
- [x] Add template expansion and instantiation
- [x] Create standard template library
- [x] Implement template validation

### Documentation Generation (2025-12-27)
- [x] Generate HTML documentation from AST
- [ ] Create PDF export using LaTeX
- [x] Add Markdown documentation generator
- [x] Generate statute dependency diagrams
- [x] Create cross-reference tables
- [ ] Add search index generation

### Metadata & Extraction (2025-12-27)
- [x] Extract jurisdiction hierarchy
- [x] Build temporal version history
- [x] Generate compliance matrices
- [x] Extract entity relationships
- [x] Create audit trail from amendments

### Testing & Quality
- [x] Add property-based testing with proptest
- [ ] Create mutation testing framework
- [ ] Add coverage-guided fuzzing
- [x] Implement snapshot testing for AST
- [x] Create test case generators

### Integration & Tooling (2025-12-27)
- [ ] Add GitHub Actions workflow support
- [ ] Create pre-commit hooks for validation
- [ ] Add VS Code extension improvements
- [x] Implement watch mode for continuous validation
- [x] Create diff tool for statute comparison

## Roadmap for 0.1.0 Series

### Grammar Extensions (v0.1.1)
- [ ] Add `DELEGATE` clause - delegation of authority to other statutes
- [ ] Add `PRIORITY` clause - explicit priority ordering for conflicts
- [ ] Add `SCOPE` clause - define applicable entity types
- [ ] Add `CONSTRAINT` clause - invariant expressions that must hold
- [ ] Add string interpolation in effect descriptions `${variable}`

### Advanced Parsing (v0.1.2)
- [ ] Add Unicode identifier support (Japanese, Chinese legal terms)
- [ ] Add heredoc syntax for multi-line strings
- [ ] Add escape sequences in string literals
- [ ] Add scientific notation for numeric literals
- [ ] Add binary/hex/octal numeric literals for technical codes

### Type System (v0.1.3)
- [ ] Add type annotations for condition values `age: Integer`
- [ ] Add type inference for unannotated conditions
- [ ] Add type checking for comparison operators
- [ ] Add enum types for constrained values `status: Active | Inactive`
- [ ] Add type aliases `type Currency = Decimal`

### Module System (v0.1.4)
- [ ] Add namespace support `namespace tax.income.2024`
- [ ] Add wildcard imports `IMPORT tax.income.*`
- [ ] Add selective imports `IMPORT { credit, deduction } FROM tax.income`
- [ ] Add re-exports for public API curation
- [ ] Add private/public visibility modifiers

### Macro System (v0.1.5)
- [ ] Add macro definition syntax `MACRO benefit_eligibility($age, $income)`
- [ ] Add macro expansion with hygiene
- [ ] Add variadic macro parameters
- [ ] Add conditional macro expansion `#IF`, `#ELSE`
- [ ] Add built-in macros for common patterns

### Error Recovery (v0.1.6)
- [ ] Add panic mode recovery for syntax errors
- [ ] Add missing delimiter insertion
- [ ] Add typo correction with Levenshtein distance
- [ ] Add contextual error messages based on parser state
- [ ] Add multi-error reporting per parse

### LSP Enhancements (v0.1.7)
- [ ] Add semantic tokens for syntax highlighting
- [ ] Add inlay hints for inferred types
- [ ] Add code lens for statute references count
- [ ] Add signature help for condition constructors
- [ ] Add document symbols hierarchy

### Optimization (v0.1.8)
- [ ] Add condition hoisting (move invariant conditions up)
- [ ] Add common subexpression elimination
- [ ] Add dead condition elimination
- [ ] Add condition reordering for short-circuit optimization
- [ ] Add constant folding for static expressions

### Code Generation Targets (v0.1.9)
- [ ] Add TypeScript/JavaScript generator
- [ ] Add Go generator
- [ ] Add Rust generator (for embedding)
- [ ] Add Java generator
- [ ] Add C# generator