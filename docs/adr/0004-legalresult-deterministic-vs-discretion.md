# ADR-0004: `LegalResult` separates deterministic logic from judicial discretion

## Status

Accepted

## Context

The central philosophical claim of Legalis-RS is that **"not everything should be
computable."** Some legal determinations follow mechanically from the rules (is
the applicant at least 18? is income below the threshold?), while others require
human interpretation (was there "just cause"? what does "public welfare"
require?). A system that silently collapses the second kind into the first is
dangerous: it dresses up a value judgment as a calculation and removes the human
from the loop precisely where the human matters most.

The design must therefore make the boundary between "automatable" and "requires a
person" explicit and impossible to ignore by accident.

## Decision

Outcomes are represented by the **`LegalResult<T>`** enum in `legalis-core`, with
three variants:

```rust
pub enum LegalResult<T> {
    Deterministic(T),           // mechanically derivable outcome
    JudicialDiscretion { .. },  // requires human interpretation
    Void { reason: String },    // the rule itself is logically inconsistent
}
```

- `Deterministic(T)` carries an outcome that was computed from the rules.
- `JudicialDiscretion { .. }` records that a human judgment is required, carrying
  the issue, a context identifier, and an optional narrative hint rather than a
  fabricated answer.
- `Void { reason }` flags that the law as written is internally contradictory.

`LegalResult` is a monad-like type with combinators (`map`, `and_then`,
`unwrap_or_discretion`, …) so that deterministic computation can be chained while
discretion and voidness propagate through the pipeline instead of being lost. The
simulator's metrics count these three outcomes separately (`deterministic_count`,
`discretion_count`, `void_count`), so the proportion of cases that *cannot* be
automated is a first-class, measurable output.

## Consequences

**Benefits**

- The type system enforces acknowledgment of human judgment. Code that consumes a
  `LegalResult<T>` cannot get at the `T` without explicitly handling the
  `JudicialDiscretion` and `Void` cases.
- Simulations and audits can report exactly how much of a statute book is
  deterministic versus discretionary — turning the "where must a human decide?"
  question into a metric (see the simulation tutorial).
- It supports the intended "hybrid court" use case: deterministic matters can be
  routed to automation while discretionary matters are escalated to people.

**Trade-offs / risks accepted**

- Every consumer of an outcome pays an ergonomic cost: results are wrapped and
  must be unwrapped through explicit handling. This friction is intentional.
- Classifying a determination as deterministic vs. discretionary is itself a
  modeling decision made by the statute author; the type enforces that the
  decision is *made and recorded*, not that it is *correct*.
