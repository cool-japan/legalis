# ADR-0003: The Statute / Condition / Effect core model

## Status

Accepted

## Context

Every layer of Legalis-RS — the DSL, the verifier, the simulator, the
jurisdiction crates, the exporters — needs a shared, precise representation of a
legal rule. That representation must be:

- **Composable**, because real statutes combine many sub-conditions with logical
  connectives, exceptions, and effects.
- **Serializable**, because rules are loaded from and saved to YAML/JSON/TOML and
  exported to other formats (Catala, L4, Akoma Ntoso, RDF, smart contracts).
- **Machine-checkable**, because the whole point of the project is to verify and
  simulate rules rather than just store their text.

## Decision

`legalis-core` defines a small, central model. A rule is a **`Statute`**, which
carries an identifier, a title, a single primary **`Effect`**, a list of
preconditions, optional temporal validity, jurisdiction, version, and optional
discretionary logic. Statutes are constructed with a fluent builder:

```rust
use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};

let statute = Statute::new(
        "voting-rights",
        "Voting Rights Act",
        Effect::new(EffectType::Grant, "Right to vote in elections"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("US")
    .with_version(1);
```

The two supporting types are algebraic data types:

- **`Condition`** is an `enum` with concrete leaf variants (`Age`, `Income`,
  `Geographic`, `EntityRelationship`, `DateRange`, `ResidencyDuration`,
  `HasAttribute`, `AttributeEquals`, `Duration`, `Percentage`, `SetMembership`,
  `Pattern`, `Custom`, and more) plus recursive combinators `And`, `Or`, and
  `Not`. Comparisons use a shared **`ComparisonOp`** enum
  (`Equal`, `NotEqual`, `GreaterThan`, `GreaterOrEqual`, `LessThan`,
  `LessOrEqual`).
- **`Effect`** pairs an **`EffectType`** (`Grant`, `Revoke`, `Obligation`,
  `Prohibition`, `MonetaryTransfer`, `StatusChange`, and the composite
  `Conditional` / `Delayed` / `Compound`) with a description and a flexible
  `HashMap<String, String>` of parameters.

Conditions are evaluated against an entity through the **`EvaluationContext`**
trait, which exposes typed accessors (`get_age`, `get_income`, `get_attribute`,
`check_geographic`, `check_relationship`, and so on). This keeps the rule model
independent of how facts are stored.

## Consequences

**Benefits**

- Using `enum`s for `Condition` and `EffectType` gives exhaustive
  pattern-matching: the compiler flags every place that must handle a new variant.
  This is visible throughout the codebase — for example, the welfare-benefits
  example pattern-matches on `Condition::Age`, `Condition::Income`,
  `Condition::And`, etc.
- The model serializes cleanly (behind a `serde` feature flag) and is the single
  interchange type that the DSL parser produces, the verifier consumes, the
  simulator applies, and every exporter reads.
- Recursive `And`/`Or`/`Not` composition mirrors how legal preconditions actually
  combine, so authoring and analysis stay close to the domain.

**Trade-offs / risks accepted**

- A central `Condition` enum grows over time; adding a variant is a breaking
  change that ripples through every exhaustive `match`. This is accepted as the
  price of compile-time completeness.
- `Effect` parameters are stringly-typed (`HashMap<String, String>`) for
  flexibility; stronger typing, where needed, is layered above the core rather
  than baked into it.

See `crates/legalis-core/ADR.md` for the crate-internal rationale behind these
data-structure choices (ADTs over a visitor pattern, `HashMap` for parameters,
etc.).
