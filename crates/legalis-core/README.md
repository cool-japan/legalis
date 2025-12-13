# legalis-core

Core types and traits for the Legalis-RS legal framework.

## Overview

This crate provides the foundational type definitions that represent legal concepts in the Legalis-RS system. The central philosophy is encoded in the `LegalResult<T>` type, which explicitly distinguishes between computationally derivable outcomes and those requiring human judgment.

## Key Types

### LegalResult<T>

The core algebraic data type representing legal judgment outcomes:

```rust
pub enum LegalResult<T> {
    // Automatically derivable (age checks, income limits, deadlines)
    Deterministic(T),

    // Requires human interpretation ("reasonable cause", "public welfare")
    JudicialDiscretion {
        issue: String,
        context_id: Uuid,
        narrative_hint: Option<String>,
    },

    // Logical inconsistency in the law itself
    Void { reason: String },
}
```

### Statute

Represents a legal article with preconditions and effects:

```rust
pub struct Statute {
    pub id: String,
    pub title: String,
    pub preconditions: Vec<Condition>,
    pub effect: Effect,
    pub discretion_logic: Option<String>,
}
```

### Condition

Represents legal preconditions:

```rust
pub enum Condition {
    Age { operator: ComparisonOp, value: u32 },
    Income { operator: ComparisonOp, value: u64 },
    HasAttribute { key: String },
    AttributeEquals { key: String, value: String },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    Custom { description: String },
}
```

### LegalEntity Trait

Interface for entities subject to legal rules:

```rust
pub trait LegalEntity: Send + Sync {
    fn id(&self) -> Uuid;
    fn get_attribute(&self, key: &str) -> Option<String>;
    fn set_attribute(&mut self, key: &str, value: String);
}
```

### Typed Attributes System

Legalis-Core provides a type-safe attribute system to replace error-prone string parsing with strongly-typed values:

```rust
use legalis_core::{TypedEntity, AttributeValue, AttributeError};
use chrono::NaiveDate;

// Create a typed entity
let mut person = TypedEntity::new();

// Set typed attributes (compile-time type safety)
person.set_u32("age", 25);
person.set_u64("income", 50000);
person.set_bool("is_citizen", true);
person.set_string("name", "Alice");
person.set_date("birth_date", NaiveDate::from_ymd_opt(1999, 1, 15).unwrap());
person.set_f64("tax_rate", 0.15);

// Get typed attributes (no parsing errors)
assert_eq!(person.get_u32("age").unwrap(), 25);
assert!(person.get_bool("is_citizen").unwrap());

// Type safety - this will return an error
assert!(person.get_u32("name").is_err()); // name is a String, not u32
```

**Supported Types:**
- `u32`, `u64`, `i64` - Integer values
- `bool` - Boolean flags
- `String` - Text values
- `NaiveDate` - Date values
- `f64` - Floating point numbers
- `Vec<String>` - String lists

**Backward Compatibility:**
`TypedEntity` implements `LegalEntity` trait and automatically parses string values:

```rust
let mut entity = TypedEntity::new();

// Old string-based code still works
entity.set_attribute("age", "30".to_string());

// New typed code can read the parsed value
assert_eq!(entity.get_u32("age").unwrap(), 30);

// String retrieval also works
assert_eq!(entity.get_attribute("age").unwrap(), "30");
```

## Usage

### Basic Entity (String-based)

```rust
use legalis_core::{Statute, Condition, Effect, EffectType, ComparisonOp, BasicEntity};

// Create a statute
let statute = Statute::new(
    "adult-voting-rights",
    "Adult Voting Rights",
    Effect::new(EffectType::Grant, "Right to vote in elections"),
)
.with_precondition(Condition::Age {
    operator: ComparisonOp::GreaterOrEqual,
    value: 18,
});

// Create an entity (string-based)
let mut citizen = BasicEntity::new();
citizen.set_attribute("age", "25".to_string());
citizen.set_attribute("citizenship", "JP".to_string());
```

### Typed Entity (Type-safe)

```rust
use legalis_core::{TypedEntity, Statute, Condition, Effect, EffectType, ComparisonOp};

// Create an entity (type-safe)
let mut citizen = TypedEntity::new();
citizen.set_u32("age", 25);                // No string conversion needed
citizen.set_string("citizenship", "JP");   // Explicit type

// Both BasicEntity and TypedEntity implement LegalEntity trait
// and work seamlessly with the rest of the framework
```

## Design Philosophy

The explicit separation of `Deterministic` and `JudicialDiscretion` outcomes is intentional. It serves as a safeguard against "AI theocracy" - the system explicitly acknowledges areas where human judgment is irreplaceable and must not be automated away.

## License

MIT OR Apache-2.0
