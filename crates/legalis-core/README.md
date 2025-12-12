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

## Usage

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

// Create an entity
let mut citizen = BasicEntity::new();
citizen.set_attribute("age", "25".to_string());
citizen.set_attribute("citizenship", "JP".to_string());
```

## Design Philosophy

The explicit separation of `Deterministic` and `JudicialDiscretion` outcomes is intentional. It serves as a safeguard against "AI theocracy" - the system explicitly acknowledges areas where human judgment is irreplaceable and must not be automated away.

## License

MIT OR Apache-2.0
