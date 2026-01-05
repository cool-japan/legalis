# legalis-dsl TODO

## Status Summary

Version: 0.2.0 | Status: Stable | Tests: 453 Passing | Warnings: 0

All v0.1.1-v0.1.3 features complete (Grammar Extensions, Advanced Parsing, Type System). Module system (v0.1.4+) and LSP enhancements complete. AI-Assisted Authoring (v0.2.1) features significantly advanced.

---

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
- [x] Data flow analysis for statute dependencies
- [x] Taint analysis for security-sensitive attributes
- [x] Dead code detection (unreachable effects)
- [x] Value range analysis for numeric conditions
- [x] Consistency checking across related statutes

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
- [x] Create PDF export using LaTeX
- [x] Add Markdown documentation generator
- [x] Generate statute dependency diagrams
- [x] Create cross-reference tables
- [x] Add search index generation

### Metadata & Extraction (2025-12-27)
- [x] Extract jurisdiction hierarchy
- [x] Build temporal version history
- [x] Generate compliance matrices
- [x] Extract entity relationships
- [x] Create audit trail from amendments

### Testing & Quality
- [x] Add property-based testing with proptest
- [x] Create mutation testing framework
- [ ] Add coverage-guided fuzzing (requires cargo-fuzz integration)
- [x] Implement snapshot testing for AST
- [x] Create test case generators

### Integration & Tooling (2025-12-27)
- [x] Add GitHub Actions workflow support
- [x] Create pre-commit hooks for validation
- [ ] Add VS Code extension improvements
- [x] Implement watch mode for continuous validation
- [x] Create diff tool for statute comparison

## Roadmap for 0.1.0 Series

### Grammar Extensions (v0.1.1)
- [x] Add `DELEGATE` clause - delegation of authority to other statutes
- [x] Add `PRIORITY` clause - explicit priority ordering for conflicts
- [x] Add `SCOPE` clause - define applicable entity types
- [x] Add `CONSTRAINT` clause - invariant expressions that must hold
- [x] Add string interpolation in effect descriptions `${variable}`

### Advanced Parsing (v0.1.2)
- [x] Add Unicode identifier support (Japanese, Chinese legal terms)
- [x] Add heredoc syntax for multi-line strings
- [x] Add escape sequences in string literals
- [x] Add scientific notation for numeric literals
- [x] Add binary/hex/octal numeric literals for technical codes

### Type System (v0.1.3)
- [x] Add type annotations for condition values `age: Integer` (Type enum extended)
- [x] Add type inference for unannotated conditions (already supported)
- [x] Add type checking for comparison operators (already supported)
- [x] Add enum types for constrained values `status: Active | Inactive`
- [x] Add type aliases `type Currency = Decimal`

### Module System (v0.1.4) ✅ COMPLETE
- [x] Add namespace support `namespace tax.income.2024` - AST structures added
- [x] Add wildcard imports `IMPORT tax.income.*` - ImportKind enum added
- [x] Add selective imports `IMPORT { credit, deduction} FROM tax.income` - ImportKind::Selective added
- [x] Add re-exports for public API curation - ExportNode added
- [x] Add private/public visibility modifiers - Visibility enum added
- [x] Created module_system.rs with core types (ImportKind, Visibility, NamespaceNode, ExportNode)
- [x] Updated Token enum with module keywords (NAMESPACE, FROM, PUBLIC, PRIVATE, EXPORT, Star)
- [x] Updated tokenizer to recognize new keywords
- [x] Updated AST structures (LegalDocument gains namespace/exports, StatuteNode gains visibility, ImportNode gains kind)
- [x] Updated parser to create structures with new fields
- [x] Parser implementation for namespace declarations (`parse_namespace`)
- [x] Parser implementation for export declarations (`parse_export`) - supports wildcard, selective, and re-export
- [x] Parser implementation for wildcard imports (`IMPORT path.*`)
- [x] Parser implementation for selective imports (`IMPORT { items } FROM path`)
- [x] Parser implementation for visibility modifiers (`PUBLIC STATUTE` / `PRIVATE STATUTE`)
- [x] Core library builds successfully with all module system features

### Macro System (v0.1.5) ✅ COMPLETE
- [x] Add macro definition syntax `MACRO benefit_eligibility($age, $income)`
- [x] Add macro expansion with hygiene
- [x] Add variadic macro parameters
- [x] Add conditional macro expansion `#IF`, `#ELSE`
- [x] Add built-in macros for common patterns

### Error Recovery (v0.1.6) ✅ COMPLETE
- [x] Add panic mode recovery for syntax errors
- [x] Add missing delimiter insertion
- [x] Add typo correction with Levenshtein distance
- [x] Add contextual error messages based on parser state
- [x] Add multi-error reporting per parse

### LSP Enhancements (v0.1.7) ✅ COMPLETE
- [x] Add semantic tokens for syntax highlighting
- [x] Add inlay hints for inferred types
- [x] Add code lens for statute references count
- [x] Add signature help for condition constructors
- [x] Add document symbols hierarchy

### Optimization (v0.1.8) ✅ COMPLETE
- [x] Add condition hoisting (move invariant conditions up)
- [x] Add common subexpression elimination
- [x] Add dead condition elimination
- [x] Add condition reordering for short-circuit optimization
- [x] Add constant folding for static expressions

### Code Generation Targets (v0.1.9) ✅ COMPLETE
- [x] Add TypeScript/JavaScript generator
- [x] Add Go generator
- [x] Add Rust generator (for embedding)
- [x] Add Java generator
- [x] Add C# generator

## Roadmap for 0.2.0 Series

### Language Server Protocol 2.0 (v0.2.0) ✅ COMPLETE
- [x] Add workspace symbol search across multiple files
- [x] Add call hierarchy for statute references
- [x] Add type hierarchy for condition inheritance
- [x] Add linked editing ranges for rename refactoring
- [x] Add selection range providers for smart selection

### AI-Assisted Authoring (v0.2.1)
- [x] Add intelligent completion suggestions (CompletionProvider)
- [ ] Add LLM-powered statute completion (requires external LLM integration)
- [ ] Add natural language to DSL translation (parser-based approach possible)
- [x] Add semantic error explanation in plain language (ErrorExplainer)
- [ ] Add auto-fix suggestions from AI analysis (requires pattern matching system)
- [x] Add DSL to natural language documentation generation (NLGenerator)

### Multi-Language DSL (v0.2.2)
- [ ] Add Japanese statute syntax variant (日本法令DSL)
- [ ] Add German statute syntax variant (Deutsche Rechtssprache)
- [ ] Add French statute syntax variant (Syntaxe juridique française)
- [ ] Add Chinese statute syntax variant (中文法规语法)
- [ ] Add Arabic statute syntax variant (with RTL support)

### Visual DSL Editor (v0.2.3)
- [ ] Add block-based visual programming interface
- [ ] Add drag-and-drop condition builder
- [ ] Add visual flow diagram for statute logic
- [ ] Add real-time DSL text synchronization
- [ ] Add export to SVG/PNG for documentation

### Advanced Type Inference (v0.2.4)
- [ ] Add Hindley-Milner type inference for conditions
- [ ] Add algebraic data type support
- [ ] Add polymorphic condition functions
- [ ] Add type classes for condition behaviors
- [ ] Add row polymorphism for effect parameters

### Contract DSL Extension (v0.2.5)
- [ ] Add contract-specific syntax extensions
- [ ] Add clause templates for common provisions
- [ ] Add obligation/right relationship modeling
- [ ] Add party definition syntax
- [ ] Add performance condition blocks

### Regulatory DSL Extension (v0.2.6)
- [ ] Add compliance requirement syntax
- [ ] Add penalty structure definitions
- [ ] Add reporting obligation blocks
- [ ] Add inspection/audit requirement syntax
- [ ] Add deadline and timeline specifications

### Test DSL Extension (v0.2.7)
- [ ] Add inline test case syntax `@test`
- [ ] Add property-based test specifications
- [ ] Add coverage requirement annotations
- [ ] Add snapshot assertion syntax
- [ ] Add mock entity definitions

### IDE Integration Enhancements (v0.2.8)
- [ ] Add JetBrains plugin (IntelliJ, CLion)
- [ ] Add Neovim/Vim plugin with TreeSitter
- [ ] Add Emacs major mode
- [ ] Add Zed editor extension
- [ ] Add web-based Monaco editor support

### Formal Specification Export (v0.2.9)
- [ ] Add Coq export for proof assistants
- [ ] Add Lean 4 export for theorem proving
- [ ] Add TLA+ export for model checking
- [ ] Add Alloy export for constraint analysis
- [ ] Add Z3 SMT-LIB direct export

## Roadmap for 0.3.0 Series (Next-Gen Features)

### Natural Language Understanding (v0.3.0)
- [ ] Add NLU parser for legislative text
- [ ] Add automatic DSL extraction from laws
- [ ] Add entity recognition for legal concepts
- [ ] Add coreference resolution for statute references
- [ ] Add semantic role labeling for conditions

### Collaborative Editing (v0.3.1)
- [ ] Add real-time collaborative DSL editing
- [ ] Add operational transformation for conflicts
- [ ] Add presence awareness (cursor positions)
- [ ] Add commenting and review system
- [ ] Add version branching for draft statutes

### Domain-Specific Language Variants (v0.3.2)
- [ ] Add tax law specialized syntax
- [ ] Add criminal law specialized syntax
- [ ] Add environmental regulation syntax
- [ ] Add financial services regulation syntax
- [ ] Add healthcare compliance syntax

### Automated Refactoring (v0.3.3)
- [ ] Add extract condition refactoring
- [ ] Add inline condition refactoring
- [ ] Add merge similar statutes refactoring
- [ ] Add split complex statute refactoring
- [ ] Add normalize condition structure refactoring

### Grammar Extension Framework (v0.3.4)
- [ ] Add user-defined syntax extensions
- [ ] Add domain-specific operator definitions
- [ ] Add custom literal syntax
- [ ] Add pluggable parser modules
- [ ] Add syntax backward compatibility layers