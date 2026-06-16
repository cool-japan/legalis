# Tutorial: Building a Complete Legal System

This tutorial builds a small but complete legal system end to end with
Legalis-RS: we define statutes, attach conditions, evaluate them against
entities, verify the rule set for logical defects, and finally simulate the rules
against a population to measure their impact. By the end you will have touched
every core crate — `legalis-core`, `legalis-dsl`, `legalis-verifier`, and
`legalis-sim` — and understand how they compose.

The example domain is a **small municipal benefits program**: a few welfare-style
rules with age, income, and attribute conditions, including one rule that
deliberately leaves room for human judgment.

> Prefer to read working code first? The repository's `examples/welfare-benefits`
> crate is a fuller version of exactly this pipeline; run it with
> `cargo run -p welfare-benefits`.

## What you will build

```
DSL / Rust  ──►  Statutes  ──►  Verification  ──►  Evaluation  ──►  Simulation
 (authoring)     (the model)    (find defects)     (per entity)     (population)
```

## Prerequisites

A working Rust toolchain and a clone of the repository (see the
[user guide](user-guide.md#1-installation-and-building)). Create a new binary
crate to follow along, and add the engine crates you need:

```toml
# Cargo.toml (of your own crate)
[dependencies]
legalis-core = "0.1"
legalis-dsl = "0.1"
legalis-verifier = "0.1"
legalis-sim = "0.1"
tokio = { version = "1", features = ["full"] }
```

(Inside this workspace you would instead use `legalis-core.workspace = true`,
etc., per the [workspace policy](adr/0002-cargo-workspace-dependency-policy.md).)

---

## Step 1 — Define statutes

There are two ways to author a statute: in the **Legal DSL** or directly in
**Rust**. We will use both so you can see the equivalence.

### 1a. In the DSL

The DSL reads close to legal prose. Put these three rules in a string (or a
`.legalis` file):

```legalis
STATUTE basic-welfare: "Basic Welfare Assistance" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN INCOME <= 30000
    THEN GRANT "Monthly welfare payment of $500"

    DISCRETION "Case workers may adjust based on local cost of living"
}

STATUTE senior-pension: "Senior Citizens Pension Supplement" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 65 AND INCOME <= 50000
    THEN GRANT "Monthly pension supplement of $300"
}

STATUTE child-support: "Child Support Benefit" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS dependent-children AND INCOME <= 60000
    THEN GRANT "Per-child monthly benefit of $200"
}
```

Notice `basic-welfare` carries a `DISCRETION` clause: part of its outcome is
explicitly *not* mechanical. That distinction is central to the whole system (see
[ADR-0004](adr/0004-legalresult-deterministic-vs-discretion.md)).

Parse the document into the core model with `LegalDslParser`:

```rust
use legalis_dsl::LegalDslParser;

const STATUTES_SRC: &str = include_str!("benefits.legalis");

fn load_statutes() -> Result<Vec<legalis_core::Statute>, Box<dyn std::error::Error>> {
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(STATUTES_SRC)?;
    Ok(statutes)
}
```

### 1b. The same rule in Rust

Anything you can write in the DSL you can build programmatically. Here is
`senior-pension` constructed with the fluent builder:

```rust
use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};

fn senior_pension() -> Statute {
    Statute::new(
        "senior-pension",
        "Senior Citizens Pension Supplement",
        Effect::new(EffectType::Grant, "Monthly pension supplement of $300"),
    )
    .with_precondition(
        Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 65 }
            .and(Condition::Income { operator: ComparisonOp::LessOrEqual, value: 50000 }),
    )
    .with_jurisdiction("US")
    .with_version(1)
}
```

The `.and(..)` combinator builds a `Condition::And(..)` out of two leaf
conditions — exactly what `WHEN AGE >= 65 AND INCOME <= 50000` produces.

---

## Step 2 — Understand the conditions

A `Statute` holds its preconditions in a `Vec<Condition>` (the `preconditions`
field). `Condition` is an enum: concrete leaves plus recursive `And`/`Or`/`Not`.
The leaves we used:

| DSL | `Condition` variant |
|-----|---------------------|
| `INCOME <= 30000` | `Income { operator: ComparisonOp::LessOrEqual, value: 30000 }` |
| `AGE >= 65` | `Age { operator: ComparisonOp::GreaterOrEqual, value: 65 }` |
| `HAS dependent-children` | `HasAttribute { key: "dependent-children".into() }` |
| `A AND B` | `And(Box<A>, Box<B>)` |

Conditions are evaluated against an *entity*. The simplest entity type is
`BasicEntity`, a string-keyed attribute bag:

```rust
use legalis_core::{BasicEntity, LegalEntity};

fn make_citizen(name: &str, age: u32, income: u64) -> BasicEntity {
    let mut e = BasicEntity::new();
    e.set_attribute("name", name.to_string());
    e.set_attribute("age", age.to_string());
    e.set_attribute("income", income.to_string());
    e
}
```

A direct, readable way to check eligibility is to pattern-match the conditions —
this is precisely what the welfare-benefits example does:

```rust
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};

fn is_eligible(entity: &BasicEntity, statute: &Statute) -> bool {
    statute.preconditions.iter().all(|c| eval(entity, c))
}

fn eval(entity: &BasicEntity, condition: &Condition) -> bool {
    match condition {
        Condition::Age { operator, value } => {
            match entity.get_attribute("age").and_then(|s| s.parse::<u32>().ok()) {
                Some(age) => cmp_u32(age, *operator, *value),
                None => false,
            }
        }
        Condition::Income { operator, value } => {
            match entity.get_attribute("income").and_then(|s| s.parse::<u64>().ok()) {
                Some(income) => cmp_u64(income, *operator, *value),
                None => false,
            }
        }
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => eval(entity, l) && eval(entity, r),
        Condition::Or(l, r) => eval(entity, l) || eval(entity, r),
        Condition::Not(inner) => !eval(entity, inner),
        _ => true, // other condition kinds: handle as your domain requires
    }
}

fn cmp_u32(a: u32, op: ComparisonOp, b: u32) -> bool {
    match op {
        ComparisonOp::GreaterOrEqual => a >= b,
        ComparisonOp::GreaterThan => a > b,
        ComparisonOp::LessOrEqual => a <= b,
        ComparisonOp::LessThan => a < b,
        ComparisonOp::Equal => a == b,
        ComparisonOp::NotEqual => a != b,
    }
}

fn cmp_u64(a: u64, op: ComparisonOp, b: u64) -> bool {
    match op {
        ComparisonOp::GreaterOrEqual => a >= b,
        ComparisonOp::GreaterThan => a > b,
        ComparisonOp::LessOrEqual => a <= b,
        ComparisonOp::LessThan => a < b,
        ComparisonOp::Equal => a == b,
        ComparisonOp::NotEqual => a != b,
    }
}
```

> `legalis-core` also provides an `EvaluationContext` trait and a built-in
> `Condition::evaluate(..)` with short-circuiting if you want to evaluate against
> a richer, typed context instead of hand-writing the match. The explicit match
> above is shown because it makes the semantics obvious and mirrors the shipped
> example.

---

## Step 3 — Verify the rule set

Before trusting the rules, run them through the verifier to catch logical
defects: contradictions, conflicting effects, dead/unreachable statutes,
circular references, and redundant conditions.

```rust
use legalis_verifier::StatuteVerifier;

fn verify(statutes: &[legalis_core::Statute]) {
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(statutes);

    if result.passed {
        println!("✓ all statutes passed verification");
    } else {
        println!("✗ verification failed:");
        for error in &result.errors {
            println!("  error: {error:?}");
        }
    }
    for warning in &result.warnings {
        println!("  warning: {warning}");
    }
    for suggestion in &result.suggestions {
        println!("  suggestion: {suggestion}");
    }
}
```

`verify` returns a `VerificationResult` whose `passed` flag, `errors`,
`warnings`, and `suggestions` you can act on — for example, failing a CI build
when `passed` is false. For deeper, satisfiability-level checking (e.g. proving a
precondition can never be true), build with `--features smt-solver`, which enables
the pure-Rust OxiZ solver. See
[ADR-0006](adr/0006-static-verification-and-conflict-detection.md).

To try this out, introduce a deliberate conflict — two statutes that grant and
revoke the same thing under the same condition — and watch the verifier flag it.

---

## Step 4 — Evaluate individual cases

With verified rules and an evaluation function, decide concrete cases:

```rust
fn main_eval() -> Result<(), Box<dyn std::error::Error>> {
    let statutes = load_statutes()?;
    verify(&statutes);

    let citizens = [
        make_citizen("Alice", 72, 35_000),
        make_citizen("Bob",   35, 25_000),
        make_citizen("Carol", 28, 22_000),
    ];

    for citizen in &citizens {
        let name = citizen.get_attribute("name").unwrap_or_default();
        println!("== {name} ==");
        for statute in &statutes {
            if is_eligible(citizen, statute) {
                println!("  [+] {} — ELIGIBLE: {}", statute.id, statute.effect.description);
            } else {
                println!("  [-] {} — not eligible", statute.id);
            }
        }
    }
    Ok(())
}
```

Where a statute carries a `DISCRETION` clause (like `basic-welfare`), eligibility
tells you the *deterministic* part — that the applicant meets the mechanical
threshold — while the discretionary adjustment remains a human decision. In a
fuller pipeline you would represent the outcome as a `LegalResult<T>` so that the
`JudicialDiscretion` case is carried through the system rather than silently
flattened.

---

## Step 5 — Simulate against a population

Individual decisions are useful; *aggregate* behavior is where policy questions
get answered. The simulator applies your statutes to a whole population and
reports how outcomes break down — crucially, **how many are deterministic versus
discretionary versus void**.

```rust
use legalis_sim::{PopulationBuilder, SimEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let statutes = load_statutes()?;
    verify(&statutes);

    // Generate a synthetic population of 1,000 entities
    let population = PopulationBuilder::new().generate_random(1000).build();

    // Apply the rules
    let engine = SimEngine::new(statutes, population);
    let metrics = engine.run_simulation().await; // async: the engine uses Tokio

    println!("total applications:    {}", metrics.total_applications);
    println!("deterministic outcomes: {}", metrics.deterministic_count);
    println!("discretionary outcomes: {}", metrics.discretion_count);
    println!("void outcomes:          {}", metrics.void_count);
    Ok(())
}
```

`run_simulation` is `async`, so call it from an `async` context
(`#[tokio::main]` here). The `SimulationMetrics` it returns separates outcomes
into the three `LegalResult` categories. A high `discretion_count` is a signal,
not a bug: it tells you how much of this policy genuinely requires human judgment
and therefore cannot be fully automated.

---

## Putting it together

The full pipeline you just built is:

1. **Author** statutes in the DSL (or Rust) — `legalis-dsl`, `legalis-core`.
2. **Verify** the rule set for logical defects — `legalis-verifier`.
3. **Evaluate** conditions against entities — `legalis-core`.
4. **Simulate** the rules against a population and measure deterministic vs.
   discretionary impact — `legalis-sim`.

From here you can extend in any direction:

- **Visualize** a statute as a decision tree (`legalis-viz`, or
  `legalis viz --viz-format mermaid`).
- **Diff** two versions of a rule and assess impact (`legalis-diff`,
  `legalis diff`).
- **Export** to a smart contract, RDF, or another legal DSL (`legalis-chain`,
  `legalis-lod`, `legalis-interop`).
- **Audit** every decision with an integrity-checked trail (`legalis-audit`).
- **Port** the rules to another jurisdiction with cultural adaptation
  (`legalis-porting`).

See the [user guide](user-guide.md) for the command-line equivalents of each of
these steps, and the [comparison with other frameworks](comparison-with-other-frameworks.md)
for how this approach relates to Catala, OpenFisca, rule engines, and the legal
markup standards.
