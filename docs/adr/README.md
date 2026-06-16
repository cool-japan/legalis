# Architecture Decision Records (ADRs)

This directory records the significant architecture decisions that shape the
Legalis-RS workspace. Each record captures the **context** in which a choice was
made, the **decision** itself, and the **consequences** (both benefits and
trade-offs) that follow from it.

These workspace-level ADRs are deliberately broad: they explain *why the project
is structured the way it is*. For decisions internal to a single crate (for
example, the data-structure choices inside `legalis-core`), see that crate's own
notes such as `crates/legalis-core/ADR.md`.

## Format

Every ADR follows the classic Michael Nygard template:

- **Title** — short noun phrase describing the decision.
- **Status** — `Proposed`, `Accepted`, `Superseded`, or `Deprecated`.
- **Context** — the forces at play: requirements, constraints, prior art.
- **Decision** — what was decided, stated in active voice.
- **Consequences** — what becomes easier, what becomes harder, and the risks
  accepted.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-0001](0001-pure-rust-no-mandatory-c-dependencies.md) | Pure-Rust stack with no mandatory C/C++ dependencies | Accepted |
| [ADR-0002](0002-cargo-workspace-dependency-policy.md) | Single Cargo workspace with centralized dependency versions | Accepted |
| [ADR-0003](0003-statute-condition-effect-core-model.md) | The Statute / Condition / Effect core model | Accepted |
| [ADR-0004](0004-legalresult-deterministic-vs-discretion.md) | `LegalResult` separates deterministic logic from judicial discretion | Accepted |
| [ADR-0005](0005-dsl-as-first-class-authoring-language.md) | A dedicated DSL as a first-class legal authoring language | Accepted |
| [ADR-0006](0006-static-verification-and-conflict-detection.md) | Static verification with SMT-backed conflict detection | Accepted |
| [ADR-0007](0007-jurisdiction-crate-per-country.md) | One crate per jurisdiction | Accepted |
| [ADR-0008](0008-no-panic-error-handling-policy.md) | No-panic / no-unwrap error-handling policy in library code | Accepted |
| [ADR-0009](0009-offline-first-no-mandatory-services.md) | Offline-first design with no mandatory external services | Accepted |

## How to add a new ADR

1. Copy an existing record as a starting point and give it the next sequential
   number (`NNNN-short-title.md`).
2. Fill in Title, Status, Context, Decision, and Consequences.
3. Add a row to the index table above.
4. If the new ADR replaces an older one, set the old record's status to
   `Superseded by ADR-NNNN` and link the two.

ADRs are append-only history. Prefer adding a new record that supersedes an old
one over editing past decisions in place.
