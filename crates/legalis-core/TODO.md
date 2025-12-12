# legalis-core TODO

## Types & Structures

### LegalResult
- [ ] Add `and_then` combinator for chaining results
- [ ] Implement `TryFrom` conversions from common types
- [ ] Add `unwrap_or_discretion` helper method
- [ ] Create `LegalResultExt` extension trait for additional utilities

### Condition
- [ ] Add `DateRange` condition type (effective dates, deadlines)
- [ ] Add `Geographic` condition (jurisdiction, region)
- [ ] Add `EntityRelationship` condition (parent-child, employer-employee)
- [ ] Implement condition normalization (simplify nested AND/OR)
- [ ] Add `evaluate` method with generic context

### Effect
- [ ] Add `Compound` effect type for multiple simultaneous effects
- [ ] Add `Conditional` effect (effect depends on runtime conditions)
- [ ] Add `Delayed` effect with trigger conditions
- [ ] Implement effect conflict detection

### Statute
- [ ] Add metadata fields (enactment_date, jurisdiction, version)
- [ ] Add amendment/supersedes relationships
- [ ] Implement statute hierarchy (parent/child statutes)
- [ ] Add tags/categories for classification
- [ ] Create `StatuteRegistry` for managing collections

### LegalEntity
- [ ] Support typed attributes (not just String)
- [ ] Add attribute validation rules
- [ ] Implement attribute change history
- [ ] Add entity relationships (belongs_to, has_many)

## Improvements

- [ ] Implement `Display` trait for all public types
- [ ] Add comprehensive `PartialOrd`/`Ord` implementations
- [ ] Create builder pattern for all complex types
- [ ] Add validation methods for construction invariants
- [ ] Implement `schemars::JsonSchema` for OpenAPI generation

## Testing

- [ ] Add property-based tests with proptest
- [ ] Add serialization roundtrip tests
- [ ] Add fuzzing targets for parsing
- [ ] Increase test coverage to >90%

## Documentation

- [ ] Add examples for every public type
- [ ] Document design decisions in module docs
- [ ] Add diagrams for type relationships
