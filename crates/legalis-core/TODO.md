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
- [x] Increase test coverage to >90% (59 unit tests + 16 doc tests + 15 property tests)

## Documentation

- [x] Add examples for every public type (comprehensive doc examples with doc tests)
- [x] Document design decisions in module docs (detailed design philosophy and architecture decisions)
- [ ] Add diagrams for type relationships
