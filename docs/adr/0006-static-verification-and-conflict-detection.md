# ADR-0006: Static verification with SMT-backed conflict detection

## Status

Accepted

## Context

One of the project's stated use cases is "the debugger for legislation": catching
logical defects in draft rules *before* they are enacted, the way a compiler
catches type errors before a program ships. That requires the ability to reason
about a set of statutes statically and answer questions such as:

- Are two statutes in conflict (the same situation triggers a `Grant` in one and
  a `Revoke` in another)?
- Is a precondition unsatisfiable, making a statute dead code?
- Are there circular references between statutes?
- Does a draft violate a stated constitutional principle?

Some of these questions are shallow structural checks; others (satisfiability of
arbitrary numeric/logical conditions) need a real constraint solver.

## Decision

`legalis-verifier` provides a `StatuteVerifier` that runs a battery of static
checks over a slice of statutes:

```rust
use legalis_verifier::StatuteVerifier;

let verifier = StatuteVerifier::new();
let result = verifier.verify(&statutes);
if !result.passed {
    for error in &result.errors {
        eprintln!("verification error: {error:?}");
    }
}
```

The verifier returns a `VerificationResult` with `passed`, `errors`, `warnings`,
and `suggestions`, and supports incremental, single-statute, budgeted, and
parallel verification variants. Checks include circular-reference detection,
dead/unreachable statute detection, contradiction and conflicting-effect
detection, redundant-condition detection, and constitutional-principle compliance.

For the questions that require true constraint solving (condition
satisfiability), the verifier integrates an SMT solver — **OxiZ**
(`oxiz-solver` / `oxiz-core`), a pure-Rust solver — behind the **`smt-solver`**
feature flag. Per ADR-0001, this keeps the heavy solver optional and avoids
linking a native solver such as Z3. Additional features gate temporal logic and
report formats (`parallel`, `pdf`, `watch`).

## Consequences

**Benefits**

- Logical defects in a rule set become reportable errors and warnings, enabling a
  "legislative DX" workflow and CI gating (`legalis verify --strict`).
- The default build provides fast structural checks with zero external
  dependencies; teams that need rigorous satisfiability opt into `smt-solver`.
- Verification results are structured (severity-aware, serializable to JSON, and
  available as SARIF/HTML), so they integrate with existing developer tooling.

**Trade-offs / risks accepted**

- Static verification of natural-language-derived rules is necessarily partial:
  it reasons about the *modelled* conditions, not the full meaning of the law.
  Discretionary content (ADR-0004) is explicitly outside what can be proven.
- The SMT path inherits the maturity/performance characteristics of a pure-Rust
  solver; it is feature-gated precisely so the core does not depend on it.
- Some checks are heuristic (e.g. subsumption/redundancy) and are surfaced as
  warnings/suggestions rather than hard errors.
