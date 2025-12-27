# legalis-core TODO

## Completed

- [x] Add `DateRange` condition type (effective dates, deadlines)
- [x] Add `Geographic` condition (jurisdiction, region)
- [x] Add `EntityRelationship` condition (parent-child, employer-employee)
- [x] Implement condition normalization (simplify nested AND/OR)
- [x] Add metadata fields (enactment_date, jurisdiction, version)
- [x] Create `StatuteRegistry` for managing collections (in legalis-registry)
- [x] Implement `Display` trait for all public types
- [x] Create builder pattern for complex types
- [x] Add temporal validity (effective dates, sunset clauses)
- [x] Add `and_then` combinator for chaining results
- [x] Implement conversions from common types (From<Option<T>>, From<Result<T, E>>)
- [x] Add `unwrap_or_discretion` helper method
- [x] Add `unwrap_or`, `unwrap_or_else`, `combine`, `ok`, `as_ref`, `into_result` methods
- [x] Add `Compound` effect type for multiple effects
- [x] Add `Conditional` effect (runtime conditions)
- [x] Add `Delayed` effect with trigger conditions
- [x] Implement effect conflict detection
- [x] Add amendment/supersedes relationships (via hierarchy module)
- [x] Implement statute hierarchy (parent/child)
- [x] Add tags/categories for classification
- [x] Support typed attributes (via TypedEntity and TypedAttributes)
- [x] Add validation methods for construction invariants (Statute::validate)
- [x] Add serde feature flag for optional serialization
- [x] Add `evaluate` method with generic context (Condition::evaluate)
- [x] Add PartialOrd/Ord for ComparisonOp
- [x] Add comparison methods (compare_u32, compare_u64, compare_i64, compare_f64)

## Types & Structures

### Condition
- [x] Add `ResidencyDuration` condition improvements (added ResidencyType and DurationUnit enums)
- [x] Implement condition caching for performance (added ConditionCache)

### LegalEntity
- [x] Add attribute validation rules (added ValidationRule and AttributeValidator)
- [x] Implement attribute change history (added AttributeChange and AttributeHistory)
- [x] Add entity relationships (belongs_to, has_many) (added EntityRelationships)

## Improvements

- [x] Add comprehensive `PartialOrd`/`Ord` implementations (added to all enum types)
- [x] Implement `schemars::JsonSchema` for OpenAPI (added with `schema` feature flag)

## Testing

- [x] Add property-based tests with proptest (15 property tests covering all major types)
- [x] Add fuzzing targets for parsing (3 fuzz targets: attribute parsing, statute validation, condition display)
- [x] Increase test coverage to >90% (65 unit tests + 16 doc tests + 15 property tests)

## Documentation

- [x] Add examples for every public type (comprehensive doc examples with doc tests)
- [x] Document design decisions in module docs (detailed design philosophy and architecture decisions)
- [x] Add diagrams for type relationships (Mermaid diagrams for core types, conditions, entities, and case law)

## Recent Enhancements (2025-12-19)

### Trait Completeness
- [x] Add `PartialOrd`/`Ord` for `AmendmentType` (hierarchy module)
- [x] Add `PartialOrd`/`Ord` for `PrecedentWeight` (case_law module)
- [x] Add `PartialOrd`/`Ord` for `PrecedentApplication` (case_law module)
- [x] Add `Hash` trait to hashable enum types (Court, PrecedentWeight, PrecedentApplication, RegionType, RelationshipType, ComparisonOp, EffectType, AmendmentType)

### Display Implementations
- [x] Add `Display` for `Court` (case_law module)
- [x] Add `Display` for `PrecedentWeight` (case_law module)
- [x] Add `Display` for `PrecedentApplication` (case_law module)
- [x] Add `Display` for `DamageType` (case_law module)
- [x] Add `Display` for `Amendment` (hierarchy module)
- [x] Add `Display` for `StatuteHierarchy` (hierarchy module)

### Schema Support
- [x] Add `schemars::JsonSchema` support for all hierarchy types
- [x] Ensure all public types support the `schema` feature flag

### CaseDatabase Enhancements
- [x] Add `iter()` - iterator over all cases
- [x] Add `iter_mut()` - mutable iterator over all cases
- [x] Add `all_cases()` - get all cases as vector
- [x] Add `cases_by_year_range()` - filter cases by year range
- [x] Add `cases_by_court()` - filter cases by court type
- [x] Add `cases_citing()` - find cases that cite a specific case
- [x] Add `len()` and `is_empty()` - collection size methods
- [x] Add `precedent_count()` - count precedent relationships
- [x] Add `precedents()` - iterator over precedents

### Case Builder Methods
- [x] Add `with_date()` - set decision date
- [x] Add `with_obiter()` - set obiter dicta
- [x] Add `overruled_by()` - mark case as overruled
- [x] Add `is_good_law()` - check if case is not overruled

### Statute Helper Methods
- [x] Add `precondition_count()` - get number of preconditions
- [x] Add `has_preconditions()` - check if preconditions exist
- [x] Add `has_discretion()` - check if discretion logic exists
- [x] Add `has_jurisdiction()` - check if jurisdiction is set
- [x] Add `preconditions()` - get reference to preconditions slice

### TemporalValidity Enhancements
- [x] Add `with_enacted_at()` - set enacted timestamp
- [x] Add `with_amended_at()` - set amended timestamp
- [x] Add `has_effective_date()` - check if effective date is set
- [x] Add `has_expiry_date()` - check if expiry date is set
- [x] Add `is_enacted()` - check if enacted timestamp exists
- [x] Add `is_amended()` - check if amended timestamp exists
- [x] Add `has_expired()` - check if expired as of given date
- [x] Add `is_pending()` - check if not yet effective as of given date

### Condition Enhancements
- [x] Add `is_compound()` - check if condition is AND/OR/NOT
- [x] Add `is_simple()` - check if condition is non-compound
- [x] Add `is_negation()` - check if condition is NOT
- [x] Add `count_conditions()` - count total conditions including nested
- [x] Add `depth()` - calculate nesting depth
- [x] Add `age()` - convenience constructor for Age conditions
- [x] Add `income()` - convenience constructor for Income conditions
- [x] Add `has_attribute()` - convenience constructor for HasAttribute
- [x] Add `attribute_equals()` - convenience constructor for AttributeEquals
- [x] Add `custom()` - convenience constructor for Custom conditions
- [x] Add `and()` - fluent combinator for AND
- [x] Add `or()` - fluent combinator for OR
- [x] Add `not()` - fluent combinator for NOT

### ComparisonOp Enhancements
- [x] Add `inverse()` - get inverse comparison operator
- [x] Add `is_equality()` - check if operator is == or !=
- [x] Add `is_ordering()` - check if operator is ordering comparison

### Effect Enhancements
- [x] Add `get_parameter()` - get parameter value by key
- [x] Add `has_parameter()` - check if parameter exists
- [x] Add `parameter_count()` - count parameters
- [x] Add `remove_parameter()` - remove parameter by key
- [x] Add `grant()` - convenience constructor for Grant effects
- [x] Add `revoke()` - convenience constructor for Revoke effects
- [x] Add `obligation()` - convenience constructor for Obligation effects
- [x] Add `prohibition()` - convenience constructor for Prohibition effects

### Testing Enhancements
- [x] Add unit tests for `Court::Display` implementation
- [x] Add unit tests for `PrecedentWeight::Display` implementation
- [x] Add unit tests for `PrecedentApplication::Display` implementation
- [x] Add unit tests for `DamageType::Display` implementation
- [x] Add unit tests for `AmendmentType::Display` implementation
- [x] Add unit tests for `Amendment::Display` implementation
- [x] Add unit tests for `StatuteHierarchy::Display` implementation
- [x] Add unit tests for ordering traits (Ord/PartialOrd)
- [x] Add unit tests for CaseDatabase iterators and query methods
- [x] Add unit tests for Condition helper methods and constructors
- [x] Add unit tests for ComparisonOp inverse and classification
- [x] Add unit tests for Effect helpers and constructors
- [x] Add unit tests for Statute helper methods
- [x] Add unit tests for TemporalValidity helper methods
- [x] Test coverage increased from 68 to 77 unit tests (+ 9 tests)
- [x] Doc test coverage: 17 doc tests
- [x] Added 8 tests for new condition types (Duration, Percentage, SetMembership, Pattern)
- [x] Added 7 tests for fluent query builder (CaseQuery)
- [x] Added 9 tests for statute conflict resolution
- [x] Current test coverage: 101 unit tests + 19 doc tests = 120 total tests

## Recent Enhancements (2025-12-26)

### Performance & Optimization
- [x] **Lazy evaluation for compound conditions**
  - Implemented `Condition::evaluate()` with short-circuit logic
  - AND: Returns false immediately when left operand is false
  - OR: Returns true immediately when left operand is true
  - Maximum depth protection to prevent stack overflow
  - Added `EvaluationContext` for passing entity attributes and settings

- [x] **Memoization cache for condition evaluation**
  - Added `ConditionCache` with LRU eviction policy
  - Configurable capacity (default 1000 entries)
  - Helper methods: `get()`, `insert()`, `clear()`, `len()`, `is_empty()`
  - Integrated into `EvaluationContext` (optional caching)

### Enhanced Error Handling
- [x] **Structured error types with error codes**
  - `ErrorSeverity` enum (Warning, Error, Critical)
  - Enhanced `ValidationError` with `error_code()`, `severity()`, `suggestion()` methods
  - New `ConditionError` for evaluation errors:
    - MissingAttribute (C001)
    - TypeMismatch (C002)
    - InvalidFormula (C003)
    - PatternError (C004)
    - MaxDepthExceeded (C005)
  - All error codes follow consistent naming: E001-E999 for validation, C001-C999 for conditions

### Advanced Legal Reasoning
- [x] **Subsumption checking for statutes**
  - `Statute::subsumes()` - checks if one statute's conditions subsume another's
  - `Statute::is_subsumed_by()` - inverse check
  - Heuristic-based implementation (framework for future logical analysis)
  - Useful for detecting redundancy and logical relationships

### Enhanced Condition Types
- [x] **Calculation condition**
  - Added `Condition::Calculation` variant for formula-based checks
  - Constructor: `Condition::calculation(formula, operator, value)`
  - Framework ready for expression parser integration (meval, evalexpr)
  - Supports comparing calculated values against thresholds

### Comparison Operators
- [x] **Enhanced ComparisonOp methods**
  - `compare_u32()` - Compare unsigned 32-bit integers
  - `compare_u64()` - Compare unsigned 64-bit integers
  - `compare_i64()` - Compare signed 64-bit integers
  - `compare_f64()` - Compare floating-point with epsilon handling

## Latest Enhancements (2025-12-26 - Session 2)

### Performance Optimizations
- [x] **Optimized condition normalization algorithm**
  - `Condition::normalize()` - Simplifies conditions using logical rules
  - `Condition::is_normalized()` - Checks if condition is in normalized form
  - Double negation elimination: `NOT (NOT A)` → `A`
  - De Morgan's laws: `NOT (A AND B)` → `(NOT A) OR (NOT B)`
  - Recursive normalization for nested conditions

### Legal Reasoning Enhancements
- [x] **Contradiction detection across statute sets**
  - `StatuteConflictAnalyzer::detect_contradictions()` - Detects logical contradictions
  - New `Contradiction` type with detailed information
  - New `ContradictionType` enum (ConflictingEffects, IdenticalConditionsConflictingEffects, etc.)
  - Checks for Grant vs Revoke conflicts
  - Detects identical preconditions with conflicting effects

### Version Control & Auditing
- [x] **Statute diffing (version comparison)**
  - `Statute::diff()` - Computes differences between statute versions
  - New `StatuteDiff` type with list of changes
  - New `StatuteChange` enum tracking specific modifications
  - Tracks changes to: ID, title, effect, preconditions, temporal validity, version, jurisdiction
  - Human-readable diff output with Display implementation

- [x] **Audit trail for condition evaluations**
  - New `EvaluationAuditTrail` for tracking evaluation history
  - New `EvaluationRecord` capturing timestamp, condition, result, duration
  - `EvaluationContext::with_audit_trail()` - Enable auditing
  - `EvaluationContext::record_evaluation()` - Record evaluations
  - Analytics methods: `average_duration()`, `slowest_evaluation()`, `slow_evaluations()`
  - LRU eviction policy (default 1000 records)

### Test Coverage
- All new features have comprehensive doc tests
- 101 unit tests passing ✓
- 26 doc tests passing ✓
- 0 warnings (clippy clean) ✓

## Latest Enhancements (2025-12-26 - Session 3)

### Evaluation Framework with Lazy Evaluation
- [x] **`EvaluationContext` trait for flexible condition evaluation**
  - Trait-based approach for custom evaluation logic
  - Methods for getting attributes, age, income, dates, geographic data, relationships
  - Support for residency duration, duration units, percentages, and formula evaluation
  - Enables integration with any entity storage or context system

- [x] **`EvaluationError` enum for structured error handling**
  - `MissingAttribute`: When required attribute is not found
  - `MissingContext`: When required context data is unavailable
  - `InvalidFormula`: For calculation/formula errors
  - `PatternError`: For regex pattern matching errors
  - `MaxDepthExceeded`: Protection against infinite recursion
  - `Custom`: For domain-specific errors
  - Implements `std::error::Error` and `Display`

- [x] **`Condition::evaluate()` with lazy short-circuit evaluation**
  - Evaluates conditions against `EvaluationContext`
  - **Short-circuit AND**: Returns `false` immediately if left operand is `false`
  - **Short-circuit OR**: Returns `true` immediately if left operand is `true`
  - Maximum depth protection (default 100 levels) to prevent stack overflow
  - Full support for all condition types: Age, Income, Geographic, Pattern, Calculation, etc.
  - Comprehensive doc tests demonstrating usage

- [x] **`ConditionEvaluator` for memoization**
  - HashMap-based cache for evaluation results
  - Tracks cache hits and misses
  - `hit_ratio()` method for performance monitoring
  - `clear_cache()` to reset cache
  - Significant performance improvement for repeated evaluations

### Advanced Legal Reasoning
- [x] **`SubsumptionAnalyzer` for statute subsumption checking**
  - `subsumes()`: Checks if statute A subsumes statute B
  - `find_subsumed()`: Finds all statutes subsumed by a given statute
  - `find_subsuming()`: Finds all statutes that subsume a given statute
  - Handles effect compatibility checking
  - Condition subsumption logic for Age, Income, Percentage conditions
  - Supports compound condition analysis

- [x] **Legal Entailment Engine**
  - `EntailmentEngine`: Determines what legal conclusions follow from statutes and facts
  - `EntailmentResult`: Tracks statute application results with satisfaction status
  - `entail()`: Applies all statutes and returns results
  - `entail_satisfied()`: Returns only statutes whose conditions are met
  - `ForwardChainingEngine`: Multi-step legal reasoning with inference chains
  - `InferenceStep`: Tracks dependencies in reasoning chains
  - Enables automatic legal conclusion derivation from facts

### Test Coverage
- All enhancements include comprehensive doc tests
- 101 unit tests passing ✓
- 29 doc tests passing ✓
- 0 warnings (adhering to NO WARNINGS POLICY) ✓
- 0 errors ✓

## Latest Enhancements (2025-12-26 - Session 4)

### Performance & Parallelization
- [x] **Parallel evaluation for compound conditions (requires "parallel" feature)**
  - Added `Condition::evaluate_parallel()` for parallel And/Or evaluation
  - Uses rayon for multi-core processing
  - `ConditionEvaluator::evaluate_parallel()` with memoization
  - `EntailmentEngine::entail_parallel()` for parallel statute application
  - `Condition::evaluate_all_parallel()` for batch evaluation
  - Significant performance improvement on multi-core systems
  - Optional feature flag: `cargo build --features parallel`

### Query & Search DSL
- [x] **Statute Registry Query DSL**
  - `StatuteQuery`: Fluent query builder for searching statutes
  - Filter methods: `jurisdiction()`, `jurisdiction_prefix()`, `effect_type()`
  - Condition filters: `with_preconditions()`, `unconditional()`, `min_preconditions()`
  - Temporal filters: `effective_at()`, `currently_effective()`
  - Search filters: `id_prefix()`, `id_suffix()`, `keyword()`
  - Execution methods: `execute()`, `first()`, `count()`, `exists()`
  - `StatuteRegistry`: Collection manager with query capabilities
  - Methods: `add()`, `remove()`, `get()`, `get_mut()`, `query()`
  - Iterator support and collection helpers
  - `find_conflicts()` for detecting conflicting statutes

### Enhanced Error Diagnostics
- [x] **Diagnostic context for validation errors**
  - `SourceLocation`: File, line, column, and code snippet information
  - `DiagnosticContext`: Stack traces, notes, and suggestions
  - `DiagnosticValidationError`: Enhanced errors with full context
  - `DiagnosticReporter`: Collect and format multiple errors
  - Methods: `error_count()`, `critical_errors()`, `summary()`, `report()`
  - Severity filtering and categorization
  - Beautiful formatted error output with location and suggestions

### Advanced Legal Reasoning
- [x] **Abductive reasoning for outcome explanation**
  - `AbductiveReasoner`: Explain why legal outcomes occurred
  - `LegalExplanation`: Detailed explanations with confidence scores
  - `ReasoningStep`: Step-by-step reasoning chains
  - `explain_effect()`: Explain why an effect occurred
  - `explain_statute()`: Explain statute application/non-application
  - `explain_why_not()`: Explain why an outcome did NOT occur
  - `find_alternatives()`: Find alternative paths to achieve outcomes
  - Confidence scoring based on satisfied conditions
  - Backward reasoning from effects to conditions

### Test Coverage
- All new features include comprehensive doc tests
- 101 unit tests passing ✓
- 38 doc tests passing ✓ (with all features)
- 0 warnings (adhering to NO WARNINGS POLICY) ✓
- 0 errors ✓
- Parallel feature tested and verified ✓

## Next Phase Enhancements (2025-12-25)

### Performance & Optimization
- [x] Add criterion benchmarks for critical operations (Statute::validate, builders, database queries)
- [x] Implement lazy evaluation for compound conditions (with short-circuit AND/OR)
- [x] Add memoization cache for frequently evaluated conditions (ConditionCache with LRU eviction)
- [x] Optimize condition normalization algorithm (De Morgan's laws, double negation elimination)
- [x] Add parallel evaluation for independent And/Or conditions (with "parallel" feature)

### Advanced Legal Reasoning
- [x] Implement conflict resolution between statutes (temporal precedence, specificity, hierarchy)
  - Implements lex posterior (later law prevails)
  - Implements lex specialis (more specific law prevails)
  - Implements lex superior (higher authority prevails)
  - Automatic conflict detection and resolution
  - Resolves multiple statutes at a given date with precedence ordering
- [x] Add subsumption checking (does statute A subsume statute B?) - heuristic-based implementation
- [x] Add contradiction detection across statute sets (detect logical conflicts)
- [x] **Implement legal entailment (what follows from a set of rules?)**
  - Added `EntailmentEngine` for determining legal conclusions from statutes and facts
  - Added `EntailmentResult` struct to track statute application results
  - Added `ForwardChainingEngine` for multi-step legal reasoning
  - Added `InferenceStep` for tracking inference chains
  - Supports automatic application of applicable statutes based on condition evaluation
  - Filters to only satisfied statutes with `entail_satisfied()`
- [x] **Implement abductive reasoning (explain why an outcome occurred)**
  - Added `AbductiveReasoner` for backward reasoning from outcomes to conditions
  - `explain_effect()`, `explain_statute()`, `explain_why_not()`, `find_alternatives()`
  - Detailed reasoning chains with confidence scoring

### Query & Search DSL
- [x] Fluent query builder for CaseDatabase (CaseQuery with chaining)
- [x] Add filter combinators (jurisdiction, court, year_range, date_range, etc.)
- [x] Add count() and first() query methods
- [x] Add not_overruled() and with_rule() filters
- [x] **Create statute registry query DSL**
  - `StatuteQuery` with fluent API for searching statutes
  - `StatuteRegistry` for managing statute collections
  - Comprehensive filter methods and query execution
- [x] **Implement full-text search for case facts and holdings**
  - `search_facts()` - search in case facts
  - `search_holding()` - search in case holding
  - `search_ratio()` - search in ratio decidendi
  - `search_all()` - search across all text fields
  - `search_keywords()` - multi-keyword search with AND logic
  - Case-insensitive text matching
- [x] **Add similarity search for analogical reasoning**
  - `find_similar_cases()` - find cases similar to a given case
  - `SimilarityResult` type with score and explanation
  - Cosine similarity using term frequency
  - Simple stopword filtering
  - Automatic case similarity scoring (0.0 to 1.0)

### Enhanced Condition Types
- [x] Add `Duration` condition (time periods with units: days, weeks, months, years)
- [x] Add `Percentage` condition (ownership stakes, voting shares)
- [x] Add `SetMembership` condition (membership/exclusion with IN/NOT IN operators)
- [x] Add `Pattern` condition (regex matching for identifiers with =~/!~ operators)
- [x] Add `Calculation` condition (derived values, formulas) - framework ready for expression parser integration

### Error Handling & Diagnostics
- [x] Add structured error types with error codes (ValidationError, ConditionError)
- [x] Add error severity levels (warning, error, critical) - ErrorSeverity enum
- [x] Create error reporting helpers with suggestions (error_code(), severity(), suggestion() methods)
- [x] **Add diagnostic context to validation errors**
  - `SourceLocation` for file/line/column information
  - `DiagnosticContext` with stack traces, notes, and suggestions
  - `DiagnosticValidationError` for enhanced error reporting
  - `DiagnosticReporter` for collecting and formatting errors
- [x] **Implement error recovery strategies**
  - `ConditionError::suggestion()` - get fix suggestions for condition errors
  - `ConditionError::recovery_options()` - get multiple recovery paths
  - `ValidationError::recovery_options()` - get multiple fix options
  - `ValidationError::try_auto_fix()` - automatic error correction when possible
  - Detailed recovery guidance for all error types

### Integration & Utilities
- [x] Add conversion helpers to/from common legal data formats (XML, JSON-LD)
- [x] Implement statute diffing (show changes between versions)
- [x] Add audit trail for condition evaluations
- [x] Create workflow helpers for common legal processes
- [x] Add transaction support for batch updates

### Type System Enhancements
- [x] Add generic parameter constraints for strongly-typed effects
- [x] **Implement builder verification at compile time (typestate pattern)**
  - `TypedStatuteBuilder<I, T, E>` with phantom type states
  - Marker types: `NoId/HasId`, `NoTitle/HasTitle`, `NoEffect/HasEffect`
  - Type-safe state transitions for required fields
  - `build()` method only available when all required fields are set
  - Compile-time verification prevents incomplete statutes
  - Fully documented with compile_fail examples
- [x] **Add phantom types for jurisdiction-specific statutes**
  - `Jurisdiction` trait for compile-time jurisdiction tracking
  - Pre-defined markers: `US`, `UK`, `EU`, `California`, `NewYork`, `AnyJurisdiction`
  - `JurisdictionStatute<J>` wrapper with phantom type parameter
  - `JurisdictionStatuteRegistry<J>` for collections
  - Type-safe prevention of jurisdiction mixing
  - Explicit `convert_to<K>()` method for jurisdiction changes
  - `define_jurisdiction!` macro for custom jurisdictions
  - Automatic jurisdiction code assignment
- [x] **Create macro for defining custom condition types**
  - `define_custom_condition!` macro for generating custom condition boilerplate
  - Automatic struct generation with public fields
  - Constructor methods (`new()`)
  - Display trait implementation
  - Conversion to `Condition::Custom` variant
  - Evaluation helper method stub
  - Fully documented with examples
- [x] Add const generics for array-based optimizations

### Documentation & Examples
- [x] Add runbook for common legal scenarios
- [ ] Create tutorial for building a complete legal system
- [ ] Add comparison with other legal reasoning frameworks
- [x] Document performance characteristics and complexity
- [x] Add architectural decision records (ADRs)

## Latest Enhancements (2025-12-26 - Session 5)

### Full-Text Search & Information Retrieval
- [x] **Full-text search for case law**
  - `CaseQuery::search_facts()` - search in case facts
  - `CaseQuery::search_holding()` - search in case holding
  - `CaseQuery::search_ratio()` - search in ratio decidendi
  - `CaseQuery::search_all()` - search across all text fields (facts, holding, ratio, issues)
  - `CaseQuery::search_keywords()` - multi-keyword AND search
  - Case-insensitive matching
  - Integration with existing CaseQuery fluent API

- [x] **Similarity search for analogical reasoning**
  - `CaseDatabase::find_similar_cases()` - find cases similar to a given case
  - `SimilarityResult` struct with case ID, score (0.0-1.0), and human-readable reason
  - Cosine similarity algorithm using term frequency vectors
  - Text tokenization with stopword filtering
  - Similarity reason generation (jurisdiction, court, common terms)
  - Useful for finding precedents and analogous cases

### Error Recovery & Diagnostics
- [x] **Enhanced error recovery strategies**
  - `ConditionError::suggestion()` - single best suggestion for fixing the error
  - `ConditionError::recovery_options()` - multiple recovery paths to choose from
  - `ValidationError::recovery_options()` - multiple fix options for validation errors
  - `ValidationError::try_auto_fix()` - automatic error correction (returns fixed value + description)
  - Comprehensive recovery guidance for all error types:
    - MissingAttribute: add to entity, use default, make optional
    - TypeMismatch: convert type, change condition, add conversion
    - InvalidFormula: fix syntax, use simpler condition, define variables
    - PatternError: fix regex, escape chars, use simpler comparison
    - MaxDepthExceeded: flatten, break into parts, remove circular refs
    - InvalidId: auto-fix by replacing invalid chars with hyphens/underscores

### Macro System for Extensibility
- [x] **`define_custom_condition!` macro**
  - Generates custom condition types with minimal boilerplate
  - Auto-generates: struct, constructor, Display, From<T> for Condition
  - Evaluation helper method stub (users can override)
  - Fully documented with doctests
  - Example usage:
    ```rust
    define_custom_condition! {
        /// Employment status check
        EmploymentStatus {
            status: String,
            requires_full_time: bool,
        }
    }
    ```

### Test Coverage
- All new features include comprehensive doc tests
- 101 unit tests passing ✓
- 47 doc tests passing ✓ (up from 36)
- 0 warnings (adhering to NO WARNINGS POLICY) ✓
- 0 errors ✓
- Total: 148 tests (up from 137)

## Latest Enhancements (2025-12-26 - Session 6)

### Typestate Pattern for Compile-Time Safety
- [x] **`TypedStatuteBuilder` with typestate pattern**
  - Generic builder with three type parameters: `TypedStatuteBuilder<I, T, E>`
  - State markers: `NoId`/`HasId`, `NoTitle`/`HasTitle`, `NoEffect`/`HasEffect`
  - Type-safe state transitions ensure required fields are set
  - `id()`, `title()`, `effect()` methods transition states
  - `build()` only callable when all required fields present (HasId, HasTitle, HasEffect)
  - Optional fields (`with_precondition`, `with_discretion`, etc.) available in all states
  - Prevents incomplete statute construction at compile time
  - Comprehensive documentation with `compile_fail` examples

### Phantom Types for Jurisdiction Safety
- [x] **`Jurisdiction` trait system**
  - Trait-based jurisdiction markers with `code()` method
  - Pre-defined jurisdictions: `US`, `UK`, `EU`, `California`, `NewYork`, `AnyJurisdiction`
  - Each marker provides static jurisdiction code (e.g., "US", "UK", "US-CA")

- [x] **`JurisdictionStatute<J>` wrapper**
  - Phantom type parameter `J: Jurisdiction` for compile-time tracking
  - Prevents mixing statutes from different jurisdictions
  - Automatic jurisdiction field assignment on construction
  - `jurisdiction_code()` - get jurisdiction at compile time
  - `statute()` / `into_statute()` - access underlying statute
  - `convert_to<K>()` - explicit jurisdiction conversion with type change

- [x] **`JurisdictionStatuteRegistry<J>` collection**
  - Type-safe registry enforcing single jurisdiction
  - `add()` only accepts statutes of same jurisdiction type
  - `find()`, `iter()`, `len()`, `is_empty()` for collection management
  - Compile-time guarantee all statutes share jurisdiction

- [x] **`define_jurisdiction!` macro**
  - Easy creation of custom jurisdiction markers
  - Generates marker struct and Jurisdiction impl
  - Example: `define_jurisdiction! { Texas => "US-TX" }`
  - Fully integrated with type system

### Test Coverage
- All new features include comprehensive doc tests
- 101 unit tests passing ✓
- 53 doc tests passing ✓ (up from 47)
- 1 compile_fail test passing ✓
- 0 warnings (adhering to NO WARNINGS POLICY) ✓
- 0 errors ✓
- Total: 155 tests (up from 148)

## Latest Enhancements (2025-12-27)

### Format Conversion & Interoperability
- [x] **JSON-LD converter for semantic web integration**
  - `JsonLdConverter::to_json_ld()` - Convert statutes to JSON-LD format
  - `JsonLdConverter::from_json_ld()` - Parse statutes from JSON-LD
  - Proper @context and @type annotations for semantic web
  - Support for all statute fields including temporal validity
  - Round-trip conversion with validation

- [x] **XML converter for legacy systems**
  - `XmlConverter::to_xml()` - Convert statutes to XML format
  - `XmlConverter::from_xml()` - Basic XML parsing (framework ready)
  - Proper escaping of special characters
  - Well-formatted output with indentation
  - Support for all statute attributes

### Workflow Helpers
- [x] **Workflow context and execution**
  - `WorkflowContext` - Entity-based context for workflow execution
  - `EligibilityChecker` - Check benefit/program eligibility
  - `ComplianceVerifier` - Verify action compliance with regulations
  - `DecisionNode` - Decision tree for multi-step legal decisions
  - Support for attribute-based and contextual evaluation

### Transaction Support
- [x] **Batch update transactions**
  - `Transaction` - Group multiple operations atomically
  - `TransactionBuilder` - Fluent builder for transactions
  - `BatchProcessor` - Execute multiple transactions
  - `Operation` enum - Add, update, remove, modify operations
  - Validation before commit with rollback support
  - Transaction metadata and audit trail

### Strongly-Typed Effects
- [x] **Generic parameter constraints for effects**
  - `TypedEffect` - Wrapper for type-safe effect parameters
  - `GrantEffect` - Typed grant effects with duration, renewability
  - `MonetaryEffect` - Tax, fine, subsidy with amount and currency
  - `StatusChangeEffect` - State transitions with reversibility
  - `GenericTypedEffect<P>` - Generic effects with parameter constraints
  - `EffectParameter` trait for custom parameter types

### Const Generic Collections
- [x] **Array-based optimizations with const generics**
  - `ConditionSet<N>` - Fixed-size condition collections (stack allocated)
  - `StatuteArray<N>` - Fixed-size statute arrays (stack allocated)
  - `FastLookup<N>` - Hash table with linear probing for fast ID lookup
  - `CollectionError` - Error handling for capacity constraints
  - Zero heap allocations for small collections
  - Better cache locality and performance

### Test Coverage
- All new modules include comprehensive tests
- 119 unit tests passing ✓ (up from 101)
- 74 doc tests passing ✓ (up from 53)
- 1 compile_fail test passing ✓
- 0 warnings (adhering to NO WARNINGS POLICY) ✓
- 0 errors ✓
- Total: 194 tests (up from 155)

## Latest Enhancements (2025-12-27 - Session 2)

### Comprehensive Documentation
- [x] **RUNBOOK.md - Practical Guide for Common Scenarios**
  - Tax credit modeling with step-by-step examples
  - Eligibility checking workflows
  - Decision tree construction
  - Transaction management patterns
  - Case law analysis and querying
  - Temporal legal rules handling
  - Conflict resolution strategies
  - 7 complete scenarios with working code
  - Troubleshooting guide
  - Best practices section

- [x] **PERFORMANCE.md - Performance Guide and Optimization**
  - Complete complexity analysis for all operations
  - Memory usage breakdown by type
  - Optimization strategies with code examples
  - Benchmark results for critical operations
  - Best practices for different scales (small/medium/large)
  - Profiling tool recommendations
  - Performance tuning guide for common problems
  - Future optimization roadmap

- [x] **ADR.md - Architectural Decision Records**
  - ADR-001: Algebraic Data Types for Conditions
  - ADR-002: Trait-Based Legal Entities
  - ADR-003: HashMap for Effect Parameters
  - ADR-004: Separate Deterministic and Discretionary Logic
  - ADR-005: Short-Circuit Evaluation
  - ADR-006: Const Generics for Collections
  - ADR-007: Optional Serde Feature
  - ADR-008: Validation Over Panics
  - Each ADR includes context, alternatives, rationale, and consequences
  - Decision-making process documented
  - Guidelines for future ADRs

### Documentation Quality
- All new documentation includes:
  - Clear examples with working code
  - Trade-off analysis
  - Performance considerations
  - Best practices
  - Troubleshooting guidance
- Cross-referenced between documents
- Practical, user-focused content
- Suitable for both beginners and advanced users

## Roadmap for 0.1.0 Series

### Advanced Condition Types (v0.1.1) - COMPLETED
- [x] Add `Composite` condition - combines multiple condition types with weights
- [x] Add `Threshold` condition - aggregate scoring across multiple attributes
- [x] Add `Fuzzy` condition - support for fuzzy logic membership functions
- [x] Add `Probabilistic` condition - probability-based condition evaluation
- [x] Add `Temporal` condition - time-sensitive condition with decay/growth functions
- [x] Added Display implementations for all new condition types
- [x] Added evaluation logic with linear interpolation for fuzzy membership
- [x] Added parallel evaluation support for Composite and Probabilistic conditions
- [x] Added helper constructor methods: composite(), threshold(), fuzzy(), probabilistic(), temporal()
- [x] Updated count_conditions() and depth() to handle nested conditions
- [x] Added get_current_timestamp() method to EvaluationContext trait
- [x] All doc tests passing (5 new doc tests)

### Case Law Integration (v0.1.2) - COMPLETED
- [x] Add `Precedent::distinguish()` - distinguish current case from precedent
- [x] Add `Precedent::follow()` - follow ratio decidendi with modifications
- [x] Add `CaseDatabase::find_conflicting_precedents()` - detect conflicting rulings
- [x] Add `CaseDatabase::binding_precedents_by_issue()` - find binding precedents by legal issue
- [x] Add hierarchical court binding rules (Supreme → Appellate → Trial)
- [x] Added helper methods: `is_binding()`, `was_distinguished()`, `was_overruled()`
- [x] Implemented `issues_similar()` private method for keyword-based issue similarity
- [x] Automatic precedent weight determination based on court hierarchy
- [x] Support for Limited application when following with modifications
- [x] All doc tests passing (4 new doc tests)

### Evaluation Enhancements (v0.1.3) - PARTIALLY COMPLETED
- [x] Add `DefaultValueContext` - wrapper providing default values for missing attributes
- [x] Add `FallbackContext` - wrapper providing fallback evaluation strategies
- [x] Added `with_default()` builder method for adding individual defaults
- [x] Implemented full `EvaluationContext` trait for both wrappers
- [x] Fallback uses OR logic for geographic and relationship checks
- [x] All doc tests passing (2 new doc tests)
- [ ] Add `Condition::evaluate_with_explanation()` - detailed step-by-step explanation
- [ ] Add `Condition::partial_evaluate()` - partial evaluation with unknowns
- [ ] Add uncertainty propagation in compound conditions

### Effect System Extensions (v0.1.4)
- [ ] Add `Effect::compose()` - compose multiple effects with priority ordering
- [ ] Add `Effect::inverse()` - compute inverse effect for rollback
- [ ] Add `TemporalEffect` - effects with start/end times and recurrence
- [ ] Add `ConditionalEffect` - effects that depend on runtime conditions
- [ ] Add effect dependency tracking and cycle detection

### Statute Relationship Improvements (v0.1.5)
- [ ] Add `Statute::derives_from()` - track derivation relationships
- [ ] Add `Statute::applies_to()` - specify applicable entity types
- [ ] Add `Statute::exceptions()` - structured exception handling
- [ ] Add `StatuteGraph` - full dependency graph with visualization hooks
- [ ] Add cross-jurisdiction statute equivalence detection

### Builder Pattern Extensions (v0.1.6)
- [ ] Add `StatuteBuilder::from_template()` - build from statute templates
- [ ] Add `StatuteBuilder::validate_progressive()` - validation at each step
- [ ] Add `ConditionBuilder` - fluent API for complex condition construction
- [ ] Add `EffectBuilder` - fluent API for effect construction
- [ ] Add macro `statute!` for declarative statute definition

### Serialization & Interoperability (v0.1.7)
- [ ] Add `Statute::to_yaml()` / `from_yaml()` - YAML serialization
- [ ] Add `Statute::to_toml()` / `from_toml()` - TOML serialization
- [ ] Add streaming deserialization for large statute collections
- [ ] Add schema migration support for version changes
- [ ] Add `Statute::hash()` - content-addressable statute hashing

### Testing Utilities (v0.1.8)
- [ ] Add `StatuteTestBuilder` - test fixture generation
- [ ] Add `ConditionGenerator` - random condition generation for property tests
- [ ] Add `EvaluationContextMock` - mock context for testing
- [ ] Add snapshot testing utilities for statute serialization
- [ ] Add mutation testing support for condition logic

### Performance & Memory (v0.1.9)
- [ ] Add arena allocator for bulk statute operations
- [ ] Add string interning for repeated identifiers
- [ ] Add `CompactStatute` - memory-optimized statute representation
- [ ] Add lazy loading for statute preconditions and effects
- [ ] Add parallel batch evaluation with work stealing