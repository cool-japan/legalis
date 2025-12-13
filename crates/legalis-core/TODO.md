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

## In Progress

- [ ] Add `evaluate` method with generic context

## Types & Structures

### LegalResult
- [ ] Add `and_then` combinator for chaining results
- [ ] Implement `TryFrom` conversions from common types
- [ ] Add `unwrap_or_discretion` helper method
- [ ] Create `LegalResultExt` extension trait

### Condition
- [ ] Add `ResidencyDuration` condition improvements
- [ ] Implement condition caching for performance

### Effect
- [ ] Add `Compound` effect type for multiple effects
- [ ] Add `Conditional` effect (runtime conditions)
- [ ] Add `Delayed` effect with trigger conditions
- [ ] Implement effect conflict detection

### Statute
- [ ] Add amendment/supersedes relationships
- [ ] Implement statute hierarchy (parent/child)
- [ ] Add tags/categories for classification

### LegalEntity
- [ ] Support typed attributes (not just String)
- [ ] Add attribute validation rules
- [ ] Implement attribute change history
- [ ] Add entity relationships (belongs_to, has_many)

## Improvements

- [ ] Add comprehensive `PartialOrd`/`Ord` implementations
- [ ] Add validation methods for construction invariants
- [ ] Implement `schemars::JsonSchema` for OpenAPI
- [ ] Add serde feature flag for optional serialization

## Testing

- [ ] Add property-based tests with proptest
- [ ] Add fuzzing targets for parsing
- [ ] Increase test coverage to >90%

## Documentation

- [ ] Add examples for every public type
- [ ] Document design decisions in module docs
- [ ] Add diagrams for type relationships
