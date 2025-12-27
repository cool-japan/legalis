# Architectural Decision Records (ADR)

This document records the significant architectural decisions made in the design of legalis-core, including context, considered alternatives, and rationale.

## Table of Contents

1. [ADR-001: Use Algebraic Data Types for Conditions](#adr-001-use-algebraic-data-types-for-conditions)
2. [ADR-002: Trait-Based Legal Entities](#adr-002-trait-based-legal-entities)
3. [ADR-003: HashMap for Effect Parameters](#adr-003-hashmap-for-effect-parameters)
4. [ADR-004: Separate Deterministic and Discretionary Logic](#adr-004-separate-deterministic-and-discretionary-logic)
5. [ADR-005: Short-Circuit Evaluation](#adr-005-short-circuit-evaluation)
6. [ADR-006: Const Generics for Collections](#adr-006-const-generics-for-collections)
7. [ADR-007: Optional Serde Feature](#adr-007-optional-serde-feature)
8. [ADR-008: Validation Over Panics](#adr-008-validation-over-panics)

---

## ADR-001: Use Algebraic Data Types for Conditions

### Status
**Accepted** - 2025-12-15

### Context
We needed a way to represent legal conditions that could be:
- Simple (Age >= 18)
- Compound (AND, OR, NOT combinations)
- Extensible for domain-specific checks
- Type-safe at compile time
- Easy to pattern match

### Decision
Use Rust enums (ADTs) with recursive composition for the `Condition` type:

```rust
pub enum Condition {
    Age { operator: ComparisonOp, value: u32 },
    Income { operator: ComparisonOp, value: u64 },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    // ...
}
```

### Alternatives Considered

#### 1. Trait-Based Approach
```rust
trait Condition {
    fn evaluate(&self, entity: &dyn LegalEntity) -> bool;
}
```

**Pros:**
- More flexible for custom conditions
- Open for extension without modifying core

**Cons:**
- Requires dynamic dispatch (slower)
- Harder to serialize/deserialize
- No exhaustiveness checking
- More complex for users

#### 2. Functional Approach
```rust
type Condition = Box<dyn Fn(&dyn LegalEntity) -> bool>;
```

**Pros:**
- Very flexible
- Composable with standard combinators

**Cons:**
- Cannot serialize
- No introspection
- Difficult to debug
- Poor error messages

### Rationale

**Why ADTs Won:**
1. **Pattern matching exhaustiveness**: Compiler ensures all cases are handled
2. **Performance**: No dynamic dispatch, all conditions are direct enum variants
3. **Serializability**: Can be serialized with serde
4. **Introspection**: Can inspect condition structure
5. **Simplicity**: Clear mental model for users

**Trade-offs Accepted:**
- Less flexible than trait-based (but `Custom` variant provides escape hatch)
- Requires modifying enum to add new condition types (mitigated by `define_custom_condition!` macro)

### Consequences

**Positive:**
- Fast evaluation (no virtual dispatch)
- Easy to understand and use
- Excellent compiler support
- Can be displayed, serialized, and analyzed

**Negative:**
- Adding new condition types requires modifying the enum
- Larger enum size (but mitigated with `Box` for nested conditions)

---

## ADR-002: Trait-Based Legal Entities

### Status
**Accepted** - 2025-12-15

### Context
Legal entities (people, organizations) need to:
- Store attributes (name, age, income, etc.)
- Support different backends (in-memory, database)
- Allow custom implementations
- Integrate with existing systems

### Decision
Define `LegalEntity` as a trait that can be implemented by any type:

```rust
pub trait LegalEntity {
    fn get_attribute(&self, key: &str) -> Option<&str>;
    fn set_attribute(&mut self, key: &str, value: String);
    // ...
}
```

Provide two built-in implementations:
1. `BasicEntity`: Simple HashMap-based
2. `TypedEntity`: Type-safe with compile-time checking

### Alternatives Considered

#### 1. Concrete Struct
```rust
pub struct LegalEntity {
    attributes: HashMap<String, String>,
}
```

**Pros:**
- Simple and straightforward
- No dynamic dispatch

**Cons:**
- Cannot integrate with existing systems
- One-size-fits-all approach
- Cannot add custom behavior

#### 2. Generic Parameter
```rust
pub struct Statute<E> {
    preconditions: Vec<Condition<E>>,
    // ...
}
```

**Pros:**
- Type-safe
- No dynamic dispatch

**Cons:**
- Viral generics throughout the codebase
- Cannot mix entity types
- More complex API

### Rationale

**Why Trait Won:**
1. **Flexibility**: Users can implement for their own types
2. **Integration**: Works with existing database models
3. **Simplicity**: Clean API without generics everywhere
4. **Extensibility**: Can add domain-specific behavior

**Trade-offs Accepted:**
- Slight performance overhead from trait method calls
- Requires dynamic dispatch in some cases

### Consequences

**Positive:**
- Easy integration with existing systems
- Users can implement custom entity types
- Clean API without generics pollution

**Negative:**
- Some dynamic dispatch overhead (minimal in practice)
- Trait object limitations (e.g., not object-safe for some operations)

---

## ADR-003: HashMap for Effect Parameters

### Status
**Accepted** - 2025-12-15

### Context
Legal effects need to carry additional information:
- Tax amount and currency
- Duration and renewal terms
- Specific requirements

Different effects have different parameters, and we need a flexible approach.

### Decision
Use `HashMap<String, String>` for effect parameters:

```rust
pub struct Effect {
    pub effect_type: EffectType,
    pub description: String,
    pub parameters: HashMap<String, String>,
}
```

Also provide `typed_effects` module for compile-time safety when needed.

### Alternatives Considered

#### 1. Enum Variants
```rust
pub enum Effect {
    Grant { resource: String, duration: Option<u32> },
    MonetaryTransfer { amount: i64, currency: String },
    // ...
}
```

**Pros:**
- Type-safe
- Pattern matching

**Cons:**
- Rigid structure
- Breaking changes when adding parameters
- Cannot represent arbitrary effects

#### 2. Trait-Based Parameters
```rust
trait EffectParameter {
    fn as_any(&self) -> &dyn Any;
}
```

**Pros:**
- Type-safe
- Extensible

**Cons:**
- Complex for users
- Difficult to serialize
- Requires downcasting

### Rationale

**Why HashMap Won:**
1. **Flexibility**: Can represent any effect parameters
2. **Extensibility**: Add parameters without breaking changes
3. **Simplicity**: Easy to understand and use
4. **Serializability**: Works well with serde

**Hybrid Approach:**
- Base `Effect` uses HashMap for maximum flexibility
- `typed_effects` module provides type-safe wrappers for common cases
- Users can choose based on their needs

### Consequences

**Positive:**
- Very flexible and extensible
- Easy to add new parameter types
- Simple serialization

**Negative:**
- No compile-time type checking for parameters
- Need to parse strings to get typed values
- Potential for typos in parameter names

**Mitigation:**
- `typed_effects` module for type safety when needed
- Validation methods to check required parameters

---

## ADR-004: Separate Deterministic and Discretionary Logic

### Status
**Accepted** - 2025-12-15

### Context
Legal rules have two fundamentally different types:
1. **Deterministic**: Mechanically derivable (age >= 18, income < $50k)
2. **Discretionary**: Requires human judgment ("good cause", "public interest")

Conflating these can lead to false confidence in automated legal systems.

### Decision
Explicitly distinguish between deterministic and discretionary outcomes:

```rust
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { reasoning: String },
    Void { conflict: String },
}
```

And in statutes:
```rust
pub struct Statute {
    pub preconditions: Vec<Condition>,       // Deterministic checks
    pub discretion_logic: Option<String>,    // Human judgment required
    // ...
}
```

### Alternatives Considered

#### 1. Boolean Result
```rust
pub fn evaluate(&self) -> bool
```

**Pros:**
- Simple
- Fast

**Cons:**
- No distinction between deterministic and discretionary
- Cannot represent uncertainty
- Dangerous for legal applications

#### 2. Confidence Score
```rust
pub fn evaluate(&self) -> (bool, f64)  // result, confidence
```

**Pros:**
- Represents uncertainty

**Cons:**
- False precision
- Still doesn't capture discretionary nature
- Can mislead users about automation capabilities

### Rationale

**Why Explicit Distinction Won:**
1. **Safety**: Prevents false confidence in automation
2. **Clarity**: Makes it clear when human judgment is required
3. **Ethics**: Preserves human agency in legal interpretation
4. **Correctness**: Accurately models legal reality

This aligns with the principle: **"Not Everything Should Be Computable"**

### Consequences

**Positive:**
- Clear when automated decision is appropriate
- Preserves human judgment in law
- Prevents "AI theocracy"
- More honest about system capabilities

**Negative:**
- More complex than simple boolean
- Requires users to handle discretion cases

---

## ADR-005: Short-Circuit Evaluation

### Status
**Accepted** - 2025-12-20

### Context
Compound conditions (AND, OR) can be expensive to evaluate, especially with:
- Deep nesting
- Expensive sub-conditions (database queries, external API calls)
- Large batches of evaluations

### Decision
Implement short-circuit evaluation with lazy semantics:

```rust
// AND: If left is false, don't evaluate right
if let Condition::And(left, right) = condition {
    if !left.evaluate(ctx)? {
        return Ok(false);  // Short-circuit
    }
    return right.evaluate(ctx);
}

// OR: If left is true, don't evaluate right
if let Condition::Or(left, right) = condition {
    if left.evaluate(ctx)? {
        return Ok(true);  // Short-circuit
    }
    return right.evaluate(ctx);
}
```

### Alternatives Considered

#### 1. Eager Evaluation
```rust
let left_result = left.evaluate(ctx)?;
let right_result = right.evaluate(ctx)?;
return Ok(left_result && right_result);
```

**Pros:**
- Simpler to implement
- Can collect all errors

**Cons:**
- Wastes computation
- Slower for large condition trees
- May trigger unnecessary side effects

#### 2. Parallel Evaluation
```rust
let (left_result, right_result) = rayon::join(
    || left.evaluate(ctx),
    || right.evaluate(ctx)
);
```

**Pros:**
- Faster on multi-core systems
- Utilizes all CPU cores

**Cons:**
- Wastes computation for short-circuit cases
- More complex
- Requires thread safety

### Rationale

**Why Short-Circuit Won:**
1. **Performance**: Avoids unnecessary evaluation
2. **Correctness**: Matches boolean logic semantics
3. **Efficiency**: Especially beneficial for expensive checks
4. **Predictability**: Users can optimize by ordering conditions

**Hybrid Approach:**
- Default: Short-circuit evaluation
- Optional: Parallel evaluation with `parallel` feature flag

### Consequences

**Positive:**
- Fast evaluation for most cases
- Users can optimize by reordering conditions
- Matches intuition from other languages

**Negative:**
- Cannot collect all validation errors in one pass
- Order-dependent (but this allows optimization)

**Best Practice:**
```rust
// Put cheap checks first
Condition::And(
    cheap_check,      // Fast boolean check
    expensive_check   // Only runs if cheap_check passes
)
```

---

## ADR-006: Const Generics for Collections

### Status
**Accepted** - 2025-12-26

### Context
Many legal scenarios involve small, fixed-size collections:
- Statutes with 3-5 preconditions
- Registries with dozens of statutes
- Lookups with known bounds

Heap allocation overhead can be significant for these cases.

### Decision
Provide const generic collections alongside standard Vec/HashMap:

```rust
pub struct ConditionSet<const N: usize> {
    items: [Option<Condition>; N],
    len: usize,
}

pub struct StatuteArray<const N: usize> { /* ... */ }
pub struct FastLookup<const N: usize> { /* ... */ }
```

### Alternatives Considered

#### 1. Only Vec/HashMap
**Pros:**
- Simpler API
- One way to do things

**Cons:**
- Always heap allocation
- Poor cache locality for small collections
- Overhead for performance-critical code

#### 2. SmallVec-style (hybrid)
```rust
pub struct HybridVec<T, const N: usize> {
    // Inline storage for small, heap for large
}
```

**Pros:**
- Best of both worlds
- Automatic selection

**Cons:**
- More complex implementation
- Larger type size
- Still has heap code path

### Rationale

**Why Const Generics Won:**
1. **Performance**: Zero heap allocations for small collections
2. **Predictability**: Known memory usage at compile time
3. **Cache locality**: Better CPU cache utilization
4. **Control**: User chooses based on needs

**Design Principle:**
- Provide both options
- Let users choose based on use case
- Document trade-offs clearly

### Consequences

**Positive:**
- Excellent performance for small collections
- Deterministic memory usage
- Better embedded systems support

**Negative:**
- Two ways to do things (complexity)
- Cannot grow beyond fixed size
- Need to choose size at compile time

**Guidelines:**
- Use const collections for known small sizes (< 10 items)
- Use Vec for dynamic or large collections
- Use FastLookup for frequent ID lookups

---

## ADR-007: Optional Serde Feature

### Status
**Accepted** - 2025-12-16

### Context
Serialization is crucial for:
- Persisting statutes to disk/database
- API communication
- Configuration files

But not all users need it, and serde adds dependencies.

### Decision
Make serde support optional with a feature flag:

```toml
[features]
default = ["serde"]
serde = ["dep:serde", "dep:serde_json", "uuid/serde", "chrono/serde"]
```

```rust
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Statute { /* ... */ }
```

### Alternatives Considered

#### 1. Always Include Serde
**Pros:**
- Simple - everyone has serialization
- No conditional compilation

**Cons:**
- Forced dependency
- Larger binary size
- Slower compile times

#### 2. Separate Crate
```rust
// legalis-core-serde
impl Serialize for Statute { /* ... */ }
```

**Pros:**
- Clean separation
- No conditional compilation

**Cons:**
- More crates to maintain
- More complex for users
- Duplicated type definitions

### Rationale

**Why Optional Feature Won:**
1. **Flexibility**: Users opt in to serialization
2. **Performance**: Smaller binaries when not needed
3. **Standard**: Follows Rust ecosystem conventions
4. **Simplicity**: Single crate, conditional compilation

### Consequences

**Positive:**
- Users can opt out of serde
- Smaller binaries for non-serialization use cases
- Faster compile times when disabled

**Negative:**
- Conditional compilation complexity
- Need to test with/without feature
- More complex documentation

---

## ADR-008: Validation Over Panics

### Status
**Accepted** - 2025-12-16

### Context
Invalid statutes can occur due to:
- User input errors
- Programmatic bugs
- Data corruption

We need to handle these gracefully.

### Decision
Prefer validation methods that return errors over runtime panics:

```rust
impl Statute {
    pub fn validate(&self) -> Vec<ValidationError> {
        // Returns list of errors
    }

    pub fn validated(self) -> Result<Self, Vec<ValidationError>> {
        let errors = self.validate();
        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }
}
```

### Alternatives Considered

#### 1. Panic on Invalid State
```rust
pub fn new(id: &str, title: &str, effect: Effect) -> Statute {
    assert!(!id.is_empty(), "ID cannot be empty");
    // ...
}
```

**Pros:**
- Simple
- Catches errors immediately

**Cons:**
- Crashes program
- Cannot recover
- Poor for library code

#### 2. Result from Constructor
```rust
pub fn new(id: &str, title: &str, effect: Effect)
    -> Result<Statute, ValidationError>
{
    // ...
}
```

**Pros:**
- Early validation
- Type-safe

**Cons:**
- Awkward API
- Cannot build incrementally
- Hard to accumulate errors

### Rationale

**Why Validation Methods Won:**
1. **Flexibility**: Can build incrementally, validate when ready
2. **Error Accumulation**: Can report all errors at once
3. **Library Friendly**: Doesn't crash user's program
4. **Recovery**: Allows programmatic fixing

**Builder Pattern Compatibility:**
```rust
let statute = Statute::new(id, title, effect)
    .with_precondition(cond1)
    .with_precondition(cond2);

// Validate when ready
let errors = statute.validate();
```

### Consequences

**Positive:**
- Can build statutes incrementally
- Collect all validation errors at once
- Users decide how to handle errors
- Better for library code

**Negative:**
- Can create invalid statutes temporarily
- Need to remember to validate
- More complex than panic

**Mitigation:**
- Provide `validated()` for Result-based workflow
- Document validation requirements clearly
- Consider typestate pattern for compile-time guarantees (implemented in `TypedStatuteBuilder`)

---

## Decision-Making Process

### How Decisions Are Made

1. **Identify Need**: Clear problem or design question
2. **Research**: Explore alternatives, look at similar systems
3. **Prototype**: Test approaches with code
4. **Evaluate**: Consider trade-offs
5. **Document**: Record in ADR
6. **Implement**: Build with chosen approach
7. **Review**: Validate decision with usage

### When to Create an ADR

Create an ADR when:
- Significant architectural choice
- Multiple viable alternatives
- Decision impacts API design
- Trade-offs need explanation
- Future maintainers need context

### Changing Decisions

Decisions can be:
- **Accepted**: Current decision
- **Deprecated**: Still present but not recommended
- **Superseded**: Replaced by new ADR

To change a decision:
1. Create new ADR explaining why
2. Mark old ADR as superseded
3. Implement migration path
4. Update documentation

---

## Future ADRs

Planned architectural decisions to document:

- **ADR-009**: Async evaluation strategy
- **ADR-010**: Database backend abstraction
- **ADR-011**: Multi-language support approach
- **ADR-012**: API versioning strategy
- **ADR-013**: Plugin system design

---

## References

- [Architecture Decision Records](https://adr.github.io/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Zero-Cost Abstractions](https://boats.gitlab.io/blog/post/zero-cost-abstractions/)
- [Parse, Don't Validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
