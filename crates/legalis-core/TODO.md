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

## Next Phase Enhancements (2025-12-25)

### Performance & Optimization
- [x] Add criterion benchmarks for critical operations (Statute::validate, builders, database queries)
- [ ] Implement lazy evaluation for compound conditions
- [ ] Add memoization cache for frequently evaluated conditions
- [ ] Optimize condition normalization algorithm
- [ ] Add parallel evaluation for independent And/Or conditions

### Advanced Legal Reasoning
- [x] Implement conflict resolution between statutes (temporal precedence, specificity, hierarchy)
  - Implements lex posterior (later law prevails)
  - Implements lex specialis (more specific law prevails)
  - Implements lex superior (higher authority prevails)
  - Automatic conflict detection and resolution
  - Resolves multiple statutes at a given date with precedence ordering
- [ ] Add subsumption checking (does statute A subsume statute B?)
- [ ] Implement legal entailment (what follows from a set of rules?)
- [ ] Add contradiction detection across statute sets
- [ ] Implement abductive reasoning (explain why an outcome occurred)

### Query & Search DSL
- [x] Fluent query builder for CaseDatabase (CaseQuery with chaining)
- [x] Add filter combinators (jurisdiction, court, year_range, date_range, etc.)
- [x] Add count() and first() query methods
- [x] Add not_overruled() and with_rule() filters
- [ ] Implement full-text search for case facts and holdings
- [ ] Add similarity search for analogical reasoning
- [ ] Create statute registry query DSL

### Enhanced Condition Types
- [x] Add `Duration` condition (time periods with units: days, weeks, months, years)
- [x] Add `Percentage` condition (ownership stakes, voting shares)
- [x] Add `SetMembership` condition (membership/exclusion with IN/NOT IN operators)
- [x] Add `Pattern` condition (regex matching for identifiers with =~/!~ operators)
- [ ] Add `Calculation` condition (derived values, formulas)

### Error Handling & Diagnostics
- [ ] Add structured error types with error codes
- [ ] Implement error recovery strategies
- [ ] Add diagnostic context to validation errors
- [ ] Create error reporting helpers with suggestions
- [ ] Add error severity levels (warning, error, critical)

### Integration & Utilities
- [ ] Add conversion helpers to/from common legal data formats (XML, JSON-LD)
- [ ] Implement statute diffing (show changes between versions)
- [ ] Add audit trail for condition evaluations
- [ ] Create workflow helpers for common legal processes
- [ ] Add transaction support for batch updates

### Type System Enhancements
- [ ] Add generic parameter constraints for strongly-typed effects
- [ ] Implement builder verification at compile time (typestate pattern)
- [ ] Add phantom types for jurisdiction-specific statutes
- [ ] Create macro for defining custom condition types
- [ ] Add const generics for array-based optimizations

### Documentation & Examples
- [ ] Add runbook for common legal scenarios
- [ ] Create tutorial for building a complete legal system
- [ ] Add comparison with other legal reasoning frameworks
- [ ] Document performance characteristics and complexity
- [ ] Add architectural decision records (ADRs)