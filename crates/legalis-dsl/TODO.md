# legalis-dsl TODO

## Status Summary

Version: 0.2.0 | Status: Stable | Tests: 737 Passing | Warnings: 0

All v0.1.1-v0.1.3 features complete (Grammar Extensions, Advanced Parsing, Type System). Module system (v0.1.4+) and LSP enhancements complete. AI-Assisted Authoring (v0.2.1) complete. Multi-Language DSL (v0.2.2) complete with support for Japanese, German, French, and Chinese. Advanced Type Inference (v0.2.4) complete (Hindley-Milner / Algorithm W). Contract DSL (v0.2.5), Regulatory DSL (v0.2.6) and the full Test DSL (v0.2.7 — `@test`/`@property`/`@coverage`/`@snapshot`/`@mock`) extensions complete. Formal Specification Export (v0.2.9) complete — Coq, Lean 4, TLA+, Alloy and SMT-LIB (OxiZ) backends.

---

## COMPLETED (2026-06-14 — domain syntax + refactorings + extensibility)

Implemented three full roadmap sections as new, self-contained module trees
(every file < 2000 lines, pure Rust, no new deps, no `scirs2`, additive only —
the base grammar and all existing public APIs are unchanged, so every existing
valid DSL still parses identically and all prior tests pass).

- **Automated Refactoring (v0.3.3)** — `src/refactor/` (`mod`, `normalize`,
  `extract`, `inline`, `merge`, `split`). Pure AST→AST transforms, each returning
  the transformed AST plus a structured `RefactorReport`/`RefactorChange`.
  - `normalize_condition_structure` — deterministic, idempotent negation normal
    form (push negations via De Morgan + double-negation elimination,
    `NOT(IN_RANGE)`↔`NOT_IN_RANGE` folding, flatten nested AND/OR, dedupe, stable
    structural ordering) + statute/document variants.
  - `extract_condition` (auto-most-frequent or targeted) replaces occurrences
    with a collision-free `HAS <ref_key>` placeholder; `inline_condition` /
    `inline_named_conditions` is its exact inverse (`inline(extract(doc)) == doc`,
    verified by round-trip tests).
  - `merge_similar_statutes` factors common conjuncts and OR-s the remainders
    across statutes sharing a structural signature (distributivity-preserving).
  - `split_complex_statute` decomposes by-effect and by-disjunction with unique
    id assignment (union-preserving).
- **Domain-Specific Language Variants (v0.3.2)** — `src/domains/` (`mod` +
  `tax`, `criminal`, `environmental`, `financial`, `healthcare`). A `LegalDomain`
  trait (keywords/operators/`parse_condition`/`validate_statute`/`vocabulary`),
  `DomainRegistry` + `builtin_registry()`, `DomainDiagnostic`/`DomainSeverity`,
  and opt-in tagging via a plain `DEFAULT domain "<name>"` (`domain_tag` /
  `tag_statute` / `is_tagged_with`). Each domain lowers bespoke syntax (tax
  brackets/rates/thresholds, mens rea/actus reus/penalty ranges, emission
  limits/reporting periods, capital/liquidity ratios, consent/data-category/
  retention) into ordinary `ConditionNode`s and validates domain invariants.
  Strictly additive (domain keywords lex as plain identifiers).
- **Grammar Extension Framework (v0.3.4)** — `src/extensibility/` (`mod`,
  `syntax`, `operators`, `literals`, `plugin`, `compat`). `ExtensibleParser`
  consults, in order: a version-aware `CompatibilityLayer` (quote/comment-aware
  deprecation rewrites with `SyntaxVersion` gating + removal errors),
  user-defined `SyntaxExtensionRegistry` productions/keywords, trait-based
  `ParserPlugin`s, then the core grammar. Plus an `OperatorTable`
  (precedence-climbing parser with its own symbol lexer, evaluable `ExprNode`)
  and a `LiteralRegistry` of `CustomLiteral`s (money/percent/duration →
  `LiteralValue` → `ConditionValue`).
- Tests: +86 (24 refactor, 32 domains, 30 extensibility) incl. extract/inline
  round-trip, normalize idempotence/order-independence, merge/split semantics,
  per-domain parse+validate, operator precedence/associativity, literal parsing,
  plugin/production dispatch, compat normalization/removal, and printer
  round-trips. Crate total 651 → 737. `cargo clippy -p legalis-dsl --all-targets
  -- -D warnings` clean; `cargo test -p legalis-dsl --doc` passes.

---

## COMPLETED (2026-06-14 — parser REQUIRES fix + parser_impl split)

- Fixed the `REQUIRES`/`SUPERSEDES` ID-list parser over-consuming (and silently dropping) the following clause; split the over-limit `parser_impl.rs` into `parser_impl/{mod,document,conditions,clauses,statute}.rs` (all < 2000 lines). Added regression tests `test_requires_does_not_drop_following_clause` and `test_requires_roundtrip_preserves_following_clause`.

---

## Completed 2026-06-14 — Formal Specification Export complete (v0.2.9)

Implemented all five formal-methods export backends as a new, self-contained
`src/formal/` module (kept out of the over-limit `parser_impl.rs`). The
semantically tricky AST → logic translation is done **once** in a shared
intermediate representation; the five backends only pretty-print it.

- **Shared lowering** (`formal/mod.rs`) — a `Formula` IR (`Compare`/`Range`/
  `BoolField`/`Like`/`Matches`/`And`/`Or`/`Not`/`Const`) plus a `FieldRegistry`
  that infers the typed entity record (`Int`/`Bool`/`Str`/`Date`, dates
  normalised to `YYYYMMDD`) from every condition. `DocumentSpec`/`StatuteSpec`
  carry the precondition, exception carve-outs (`precond ∧ ¬exc`), `REQUIRES`
  dependencies (topologically ordered so dependencies are defined first) and
  effects. `HAS k` lowers to a dedicated boolean `has_<k>` field; conflicting
  effects (grant/revoke or obligation/prohibition on the same label) are
  detected for consistency obligations.
- **Coq** (`CoqExporter`) — `Record Entity`, `Definition applies_<id> : Prop`,
  `Inductive LegalEffect`, satisfiability + consistency `Conjecture`s.
- **Lean 4** (`Lean4Exporter`) — `structure Entity`, `def applies_<id> : Prop`,
  `inductive LegalEffect`, namespace-wrapped `theorem … := by sorry` obligations.
- **TLA+** (`TlaExporter`) — record-set `Entity`, `Applies<Id>(e)` operators,
  `Effects<Id>` sequences, satisfiability/consistency `THEOREM`s.
- **Alloy** (`AlloyExporter`) — `sig Entity`, `pred applies<Id>[e]`, `run`/
  `check` analysis commands over a bounded `Int` scope.
- **SMT-LIB / OxiZ** (`SmtLibExporter`) — `declare-datatypes` entity,
  `define-fun applies_<id>`, native string theory for `LIKE`
  (`str.contains`/`str.prefixof`/`str.suffixof`), uninterpreted `str_matches`
  for regex, and `(push)`/`(check-sat)`/`(pop)` proof obligations.
- Common `FormalExporter` trait (`export`/`target`/`file_extension` +
  `export_statute`), re-exported at the crate root. Additive only; reuses the
  existing `ast` types without duplication. Pure Rust, scirs2-free, no new deps.
- Tests: +29 (20 unit in `formal/tests.rs` — IR lowering, field-registry merge,
  date/identifier helpers, conflict detection, dependency ordering, per-backend
  golden substrings, idempotence; 9 integration in `tests/formal.rs` — parse →
  export for the whole corpus, `REQUIRES` reference + ordering, `BETWEEN` +
  exception lowering, `LIKE`/`MATCHES`, consistency obligations, configuration).
  Crate total 620 → 649. `cargo clippy -p legalis-dsl --all-targets -- -D
  warnings` clean.

Deferred sibling roadmap items (annotated inline below) all require external/UI
tooling — Visual DSL Editor (v0.2.3), IDE Integration (v0.2.8), Arabic RTL
(v0.2.2), LLM-powered completion (v0.2.1), coverage-guided fuzzing and the VS
Code extension — and are intentionally left unchecked as out of pure-Rust scope.

---

## Completed 2026-06-14 — Test DSL Extension complete (v0.2.7)

Finished the remaining *Test DSL* constructs, all introduced by a leading `@`
directive and building on the existing inline `@test` runner. Implemented as two
new, self-contained files (kept out of the over-limit `parser_impl.rs`), reusing
the existing condition/value/expectation vocabulary; every construct round-trips
through the printer and runs against parsed `legalis_core::Statute`s.

- **Property-based specifications** — `@property "..." FOR s { FORALL v IN lo TO
  hi | ( v1, .. ) [GIVEN ..] [USING ..] EXPECT .. [CASES n] }`. The engine
  enumerates the domain cross-product exhaustively when it fits the budget and
  otherwise samples deterministically via an in-module SplitMix64 seeded from the
  property name (no `rand`/external fuzzer); failures report a *shrunk*
  counterexample (locally-minimal per variable).
- **Coverage annotations** — `@coverage REQUIRE statutes|outcomes <op> n% [FOR
  s]` with `>=`/`>`/`==`; measures statute coverage (targeted by ≥1 case) and
  branch-outcome coverage (both a satisfied and an unsatisfied evaluation seen),
  over both `@test` cases and generated property cases.
- **Snapshot assertions** — `@snapshot "..." FOR s EXPECT "<sig>" | RECORD`,
  pinning a statute's structural signature (effect keyword + a stable FNV-1a
  digest of its canonical pretty-print); `RECORD` blesses the current value.
- **Mock entities** — `@mock id { k = v, .. }` reusable fixtures pulled into a
  `@test`/`@property` with `USING id` (precedence mock < `GIVEN` < `FORALL`).

- New modules: `src/testspec.rs` (AST + deterministic engine + runners:
  `MockEntityNode`, `PropertyDomain`/`PropertyVar`/`PropertySpecNode`,
  `CoverageMetric`/`CoverageComparator`/`CoverageRequirementNode`,
  `SnapshotMode`/`SnapshotAssertionNode`, `TestSpecDocument`/`TestSpecReport`,
  `run_property_cases`/`compute_coverage`/`check_coverage`/`run_snapshots`/
  `run_test_cases_with_mocks`/`statute_signature`) and `src/testspec_parser.rs`
  (`Directive` dispatcher + `parse_test_spec_document`/`run_test_spec`).
- Additive only: `TestCaseNode` gained `#[serde(default)] uses`; `@test` gained a
  `USING` clause; `contract_parser` lexing helpers + sub-parsers widened to
  `pub(crate)`; the case-outcome match was extracted to a shared
  `contract::evaluate_case_outcome`. `parse_contract_document` keeps only `@test`
  via the shared directive dispatcher (still errors on unknown `@foo`).
- `printer.rs` — `format_test_spec_document` + `format_mock`/`format_property`/
  `format_coverage`/`format_snapshot` and a shared `format_expectation`.
- Tests: +33 (16 unit in `testspec.rs`, 17 integration in `tests/testspec.rs`
  incl. full round-trip, exhaustive/sampled/shrinking properties, coverage
  pass/fail, snapshot match/mismatch/record, mock override, located errors);
  crate total 587 → 620. `cargo clippy -p legalis-dsl --all-targets -- -D
  warnings` clean.

---

## Completed 2026-06-14 — Advanced Type Inference (v0.2.4)

Added a genuine Hindley–Milner type-inference engine as a new, additive
`typeinfer` submodule (kept fully separate from the lightweight
`type_checker`, which performs only non-polymorphic compatibility checks).
All five advanced-type-system features are implemented as real algorithms,
not stubs:

- **Hindley–Milner inference** — Algorithm W over a typed `Term` IR with
  substitution composition, the occurs-check, instantiation and
  let-generalization (`InferenceEngine::infer` / `infer_scheme`).
- **Algebraic data types** — `DataDecl`/`Constructor`/`DataEnv` (sum + product,
  parameterised), constructor schemes folded into Algorithm W, and
  exhaustiveness checking via Maranget's usefulness algorithm
  (specialization + default matrices).
- **Polymorphic condition functions** — `TypeScheme` quantification over both
  type and row variables, with principal-type generalization.
- **Type classes** — `Eq`/`Ord`/`Numeric`/`Matchable` with superclasses,
  context-bearing instances (`Eq a => Eq (List a)`), THIH-style context
  reduction (`to_hnf`/`reduce`), ambiguity detection, and dictionary/`Evidence`
  derivations.
- **Row polymorphism** — extensible record rows with row variables; unique-label
  row unification; conditions lower onto a shared open *entity* record (each
  attribute reference is a row-polymorphic field selection) and effect
  parameters are typed records checked with `effect_satisfies`.

- New module: `src/typeinfer/` — `mod.rs` (prelude, condition lowering,
  statute/document/effect entry points), `types.rs` (`MonoType`/`Row`/`Pred`/
  `QualType`/`TypeScheme`), `subst.rs`, `unify.rs` (incl. row unification),
  `term.rs` (typed IR), `adt.rs`, `classes.rs`, `infer.rs` (Algorithm W),
  `error.rs`, `tests.rs`.
- `lib.rs` — module wiring + re-exports (`InferenceEngine`, `MonoType`,
  `TypeScheme`, `EntityTyping`, `DocumentTyping`, `TypeInferError`).
- Integrates with existing `ConditionNode`/`EffectNode`/`StatuteNode`/
  `LegalDocument` without duplicating `type_checker`.
- Tests: +38 unit tests (`typeinfer::tests`) covering unification, the
  occurs-check, row unification, let-generalization, instantiation, ADTs +
  exhaustiveness, type-class solving/evidence/ambiguity, row polymorphism and
  AST-level condition/statute/document inference; crate total 549 → 587.
  `cargo clippy -p legalis-dsl --all-targets -- -D warnings` clean.

---

## Completed 2026-06-14 — Contract / Compliance / Inline-Test extensions (v0.2.5–v0.2.7)

Implemented the `CONTRACT` block, the regulatory clauses, and inline `@test`
cases as an additive, self-contained parse tree (kept separate from
`StatuteNode`/`LegalDocument`, which have 80+/107+ literal construction sites, to
avoid invasive churn). All constructs round-trip through `parse_contract_document`
→ `format_contract_document` → `parse_contract_document`.

- Modules touched/added:
  - `contract.rs` (new) — AST nodes, clause-template library, `@test` runner.
  - `contract_parser.rs` (new) — `impl LegalDslParser` grammar on spanned tokens
    (line/column errors); reuses `parse_condition_node` for `WHEN`.
  - `printer.rs` — `format_contract`/`format_contract_document`/`format_test_case`.
  - `ast.rs` + `tokenizer.rs` — new tokens (`Contract`, `Party`, `Right`,
    `Performance`, `Clause`, `Compliance`, `Penalty`, `Report`, `Inspect`,
    `Deadline`, `Timeline`, `At`) and `@`/keyword lexing.
  - `lib.rs` — module wiring + re-exports; `parser_impl.rs` —
    `parse_condition_node` made `pub(crate)`.
- New AST nodes: `ContractDocument`, `ContractNode`, `PartyNode`/`PartyRole`,
  `ClauseNode`/`ClauseTemplate`, `ObligationNode`, `RightNode`/`RightKind`,
  `PerformanceBlock`, `ComplianceRequirementNode`, `PenaltyNode`,
  `ReportNode`/`ReportFrequency`, `InspectionNode`, `DeadlineNode`,
  `TimelineNode`, `TestCaseNode`/`TestBinding`/`TestValue`/`TestExpectation`/
  `ExpectedEffect`, `TestRunReport`/`TestCaseResult`.
- `@test` runner (`run_test_cases` / `LegalDslParser::run_embedded_tests`)
  evaluates embedded cases against parsed `Statute`s via
  `legalis_core::Condition::evaluate` + `AttributeBasedContext`.
- Tests: +29 (8 unit in `contract.rs`, 21 integration in `tests/contract.rs`);
  crate total 520 → 549. `cargo clippy --all-targets -- -D warnings` clean.

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
- [ ] Add VS Code extension improvements (requires VS Code/TS extension tooling — out of pure-Rust scope)
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

### Error Recovery (v0.1.7) ✅ COMPLETE
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
- [ ] Add LLM-powered statute completion (requires external LLM integration — out of pure-Rust scope)
- [x] Add natural language to DSL translation (parser-based approach - NLTranslator)
- [x] Add semantic error explanation in plain language (ErrorExplainer)
- [x] Add auto-fix suggestions from AI analysis (AutoFixer with pattern matching)
- [x] Add DSL to natural language documentation generation (NLGenerator)

### Multi-Language DSL (v0.2.2) ✅ COMPLETE
- [x] Add Japanese statute syntax variant (日本法令DSL)
- [x] Add German statute syntax variant (Deutsche Rechtssprache)
- [x] Add French statute syntax variant (Syntaxe juridique française)
- [x] Add Chinese statute syntax variant (中文法规语法)
- [x] Multi-language keyword mapping system (KeywordMapping, MultiLangTranslator)
- [x] Language detection and automatic translation
- [x] Example generators for each supported language
- [ ] Add Arabic statute syntax variant (with RTL support) — RTL editor/shaping needs UI tooling, not a parser feature

### Visual DSL Editor (v0.2.3)
_Deferred: all require a GUI/graphics runtime, not a pure-Rust language feature._
- [ ] Add block-based visual programming interface — needs a GUI toolkit / web canvas
- [ ] Add drag-and-drop condition builder — needs an interactive front-end
- [ ] Add visual flow diagram for statute logic — needs a diagram renderer (out of scope here)
- [ ] Add real-time DSL text synchronization — needs an editor host
- [ ] Add export to SVG/PNG for documentation — needs an image/vector renderer

### Advanced Type Inference (v0.2.4) ✅ COMPLETE
- [x] Add Hindley-Milner type inference for conditions — Algorithm W with unification, occurs-check, let-generalization (`typeinfer::InferenceEngine`)
- [x] Add algebraic data type support — sum/product `DataDecl`s, constructor schemes, exhaustiveness-checked `match` (Maranget usefulness)
- [x] Add polymorphic condition functions — parametric polymorphism via `TypeScheme`s + let-generalization (e.g. `id : forall a. a -> a`)
- [x] Add type classes for condition behaviors — `Eq`/`Ord`/`Numeric`/`Matchable` with context reduction + dictionary/`Evidence` resolution
- [x] Add row polymorphism for effect parameters — extensible records with row variables (`Row`, `effect_satisfies`)

### Contract DSL Extension (v0.2.5) ✅ COMPLETE
- [x] Add contract-specific syntax extensions - `CONTRACT id: "title" { ... }` block (ContractNode)
- [x] Add clause templates for common provisions - `CLAUSE id FROM template: "text"` + ClauseTemplate library (`common_clause_templates`)
- [x] Add obligation/right relationship modeling - `OBLIGATION id BY x TO y` + `RIGHT id OF z CLAIM ... CORRELATIVE oblig` (Hohfeldian correlativity)
- [x] Add party definition syntax - `PARTY id: "name" ROLE role` (PartyNode, PartyRole)
- [x] Add performance condition blocks - `PERFORMANCE id { DESC .. WHEN .. DUE .. }` (PerformanceBlock)

### Regulatory DSL Extension (v0.2.6) ✅ COMPLETE
- [x] Add compliance requirement syntax - `COMPLIANCE id: "desc" STANDARD "x" WHEN ..` (ComplianceRequirementNode)
- [x] Add penalty structure definitions - `PENALTY id: "desc" AMOUNT n cur PER unit FOR oblig WHEN ..` (PenaltyNode)
- [x] Add reporting obligation blocks - `REPORT id: "desc" EVERY freq TO recipient DUE ..` (ReportNode, ReportFrequency)
- [x] Add inspection/audit requirement syntax - `INSPECT|AUDIT id: "desc" BY authority EVERY freq WHEN ..` (InspectionNode)
- [x] Add deadline and timeline specifications - `DEADLINE id: "date" "desc"` + `TIMELINE id { DEADLINE .. }` (DeadlineNode, TimelineNode)

### Test DSL Extension (v0.2.7) ✅ COMPLETE
- [x] Add inline test case syntax `@test` - `@test "name" FOR statute { GIVEN k = v EXPECT GRANT|SATISFIED|NOT SATISFIED }` + `run_test_cases`/`run_embedded_tests` runner
- [x] Add property-based test specifications - `@property "name" FOR statute { FORALL v IN lo TO hi | ( .. ) [GIVEN ..] [USING ..] EXPECT .. [CASES n] }` with exhaustive/sampled (SplitMix64) generation and counterexample shrinking (`PropertySpecNode`, `run_property_cases`)
- [x] Add coverage requirement annotations - `@coverage REQUIRE statutes|outcomes >= n% [FOR statute]` measuring statute + branch-outcome coverage (`CoverageRequirementNode`, `compute_coverage`/`check_coverage`)
- [x] Add snapshot assertion syntax - `@snapshot "name" FOR statute EXPECT "<sig>" | RECORD` pinning a stable effect+FNV-1a structural signature (`SnapshotAssertionNode`, `statute_signature`, `run_snapshots`)
- [x] Add mock entity definitions - `@mock id { k = v, .. }` reusable fixtures pulled in with `USING` (precedence: mock < GIVEN < FORALL) (`MockEntityNode`, `run_test_cases_with_mocks`)

### IDE Integration Enhancements (v0.2.8)
_Deferred: each targets a specific external editor's plugin SDK, not a pure-Rust language/semantic feature._
- [ ] Add JetBrains plugin (IntelliJ, CLion) — needs the JetBrains plugin SDK
- [ ] Add Neovim/Vim plugin with TreeSitter — needs a TreeSitter grammar + editor host
- [ ] Add Emacs major mode — needs Emacs Lisp tooling
- [ ] Add Zed editor extension — needs the Zed extension API
- [ ] Add web-based Monaco editor support — needs the Monaco/web front-end

### Formal Specification Export (v0.2.9) ✅ COMPLETE
- [x] Add Coq export for proof assistants — `CoqExporter`: `Record Entity`, `Definition applies_<id> : Prop`, `Inductive LegalEffect`, satisfiability/consistency `Conjecture`s
- [x] Add Lean 4 export for theorem proving — `Lean4Exporter`: `structure Entity`, `def applies_<id> : Prop`, `inductive LegalEffect`, `theorem … := by sorry` obligations (namespace-wrapped)
- [x] Add TLA+ export for model checking — `TlaExporter`: record-set `Entity`, `Applies<Id>(e)` operators, `Effects<Id>` sequences, satisfiability/consistency `THEOREM`s
- [x] Add Alloy export for constraint analysis — `AlloyExporter`: `sig Entity`, `pred applies<Id>[e]`, `run`/`check` analysis commands (bounded `Int` scope)
- [x] Add OxiZ SMT-LIB direct export — `SmtLibExporter`: `declare-datatypes` entity, `define-fun applies_<id>`, native string theory for `LIKE` (`str.contains`/`str.prefixof`/`str.suffixof`), `(check-sat)` obligations

## Roadmap for 0.3.0 Series (Next-Gen Features)

### Natural Language Understanding (v0.3.0)
- [ ] Add NLU parser for legislative text — DEFERRED: needs NLP/ML models, out of pure-Rust parser scope
- [ ] Add automatic DSL extraction from laws — DEFERRED: needs NLP/ML models, out of pure-Rust parser scope
- [ ] Add entity recognition for legal concepts — DEFERRED: needs an NER/ML model
- [ ] Add coreference resolution for statute references — DEFERRED: needs an NLP coreference model
- [ ] Add semantic role labeling for conditions — DEFERRED: needs an SRL/ML model

### Collaborative Editing (v0.3.1)
- [ ] Add real-time collaborative DSL editing — DEFERRED: needs an editor host; legalis-interop already provides CRDT/OT engines
- [ ] Add operational transformation for conflicts — DEFERRED: legalis-interop already provides CRDT/OT engines; integration needs an editor host
- [ ] Add presence awareness (cursor positions) — DEFERRED: needs an editor host / live session
- [ ] Add commenting and review system — DEFERRED: needs an editor/review UI host
- [ ] Add version branching for draft statutes — DEFERRED: belongs to a VCS/storage layer, not the DSL parser

### Domain-Specific Language Variants (v0.3.2) ✅ COMPLETE
- [x] Add tax law specialized syntax — `TaxLawDomain` (`src/domains/tax.rs`): `BRACKET/RATE/THRESHOLD/TAXABLE_BASE/DEDUCTION/EXEMPTION`, lowers to `Between`/`Comparison`; validates rate∈[0,100], inverted/overlapping brackets
- [x] Add criminal law specialized syntax — `CriminalLawDomain` (`src/domains/criminal.rs`): `MENS_REA/ACTUS_REUS/ELEMENT/PENALTY_RANGE/OFFENSE`; validates recognized mens-rea levels, penalty-range coherence, actus-reus+mens-rea completeness
- [x] Add environmental regulation syntax — `EnvironmentalDomain` (`src/domains/environmental.rs`): `EMISSION_LIMIT/THRESHOLD/REPORTING_PERIOD/MONITORING`; validates non-negative limits and positive reporting periods
- [x] Add financial services regulation syntax — `FinancialServicesDomain` (`src/domains/financial.rs`): `CAPITAL_RATIO/LIQUIDITY_RATIO/LEVERAGE_RATIO/RATIO/REPORTING`; validates non-negative/plausible ratios and known Basel names
- [x] Add healthcare compliance syntax — `HealthcareDomain` (`src/domains/healthcare.rs`): `CONSENT/DATA_CATEGORY/RETENTION/PURPOSE`; validates consent levels, data categories, positive retention, weak-consent-for-protected-data
- [x] Domain registry + opt-in tagging — `DomainRegistry`/`builtin_registry()`, `LegalDomain` trait, `DomainDiagnostic`/`DomainSeverity`, `DEFAULT domain "<name>"` tagging (`domain_tag`/`tag_statute`/`is_tagged_with`); additive (domain keywords lex as plain identifiers)

### Automated Refactoring (v0.3.3) ✅ COMPLETE
- [x] Add extract condition refactoring — `extract_condition` (`src/refactor/extract.rs`): auto-most-frequent or targeted; replaces occurrences with a collision-free `HAS <ref_key>` placeholder + `ExtractedCondition`/report
- [x] Add inline condition refactoring — `inline_condition`/`inline_named_conditions` (`src/refactor/inline.rs`): exact inverse of extract (`inline(extract(doc)) == doc`), fixpoint substitution for nested refs
- [x] Add merge similar statutes refactoring — `merge_similar_statutes` (`src/refactor/merge.rs`): groups by structural signature, factors common conjuncts, OR-s remainders (distributivity-preserving)
- [x] Add split complex statute refactoring — `split_complex_statute` (`src/refactor/split.rs`): by-effect and by-disjunction decomposition with unique id assignment (union-preserving)
- [x] Add normalize condition structure refactoring — `normalize_condition_structure` (`src/refactor/normalize.rs`): deterministic, idempotent NNF (push negations, flatten, dedupe, stable order) + statute/document variants

### Grammar Extension Framework (v0.3.4) ✅ COMPLETE
- [x] Add user-defined syntax extensions — `SyntaxExtensionRegistry` (`src/extensibility/syntax.rs`): register custom keywords + grammar productions (trigger keyword → `ConditionNode` handler)
- [x] Add domain-specific operator definitions — `OperatorTable`/`OperatorDef` (`src/extensibility/operators.rs`): precedence + associativity (incl. prefix/non-assoc), precedence-climbing parser over a self-contained symbol lexer, evaluable `ExprNode`
- [x] Add custom literal syntax — `LiteralRegistry`/`CustomLiteral` (`src/extensibility/literals.rs`): pluggable literal forms (built-in money/percent/duration) producing typed `LiteralValue` → `ConditionValue`
- [x] Add pluggable parser modules — `ParserPlugin`/`PluginRegistry` (`src/extensibility/plugin.rs`): trait-based plugins the `ExtensibleParser` consults before the core grammar
- [x] Add syntax backward compatibility layers — `CompatibilityLayer`/`DeprecationRule`/`SyntaxVersion` (`src/extensibility/compat.rs`): version-aware, quote/comment-aware deprecation rewrites (warn when deprecated, error when removed); orchestrated by `ExtensibleParser`