# Legalis-RS User Guide

This guide walks you from a clean checkout to authoring, verifying, simulating,
and exporting legal rules with Legalis-RS. It is task-oriented: each section is a
short tutorial you can follow end to end.

If you want the *why* behind the design, read the
[Architecture Decision Records](adr/README.md). If you want the single biggest
worked example, see
[Building a Complete Legal System](tutorial-building-a-legal-system.md).

## Contents

1. [Installation and building](#1-installation-and-building)
2. [Core concepts](#2-core-concepts)
3. [Your first statute (the DSL)](#3-your-first-statute-the-dsl)
4. [A worked example: welfare benefits](#4-a-worked-example-welfare-benefits)
5. [Using the command-line tool](#5-using-the-command-line-tool)
6. [Verifying statutes](#6-verifying-statutes)
7. [Running simulations](#7-running-simulations)
8. [Exporting and interoperating](#8-exporting-and-interoperating)
9. [Where to go next](#9-where-to-go-next)

---

## 1. Installation and building

### Prerequisites

- Rust (Edition 2024 toolchain). The project targets a recent stable Rust.
- Cargo. No C/C++ toolchain, no system libraries, and no environment variables
  are required for the default build — the whole stack is pure Rust.

### Build

```bash
git clone https://github.com/cool-japan/legalis
cd legalis

# Build all crates with default features
cargo build

# Minimal build (drops optional gRPC, etc.)
cargo build --no-default-features

# Run the tests
cargo test
```

### Optional features

Heavier or environment-specific capabilities are behind feature flags so the
default build stays light and dependency-free:

```bash
# Rigorous formal verification via the pure-Rust OxiZ SMT solver
cargo build --features smt-solver

# Everything on
cargo build --all-features
```

| Feature | Crate | What it enables |
|---------|-------|-----------------|
| `smt-solver` | `legalis-verifier` | Satisfiability checking with the pure-Rust OxiZ solver |
| `parallel` | `legalis-verifier`, `legalis-core` | Multi-core evaluation/verification |
| `pdf` | `legalis-verifier` | PDF report output |
| `cuda` | `legalis-sim` | NVIDIA GPU-accelerated condition evaluation (falls back to CPU) |
| `grpc` | `legalis-api` | gRPC server (on by default for the API crate) |

### Install the CLI

The command-line tool's binary is named `legalis`:

```bash
cargo install --path crates/legalis-cli
legalis --help
```

You can also run it without installing: `cargo run -p legalis -- <args>`.

---

## 2. Core concepts

Legalis-RS revolves around a small set of types defined in `legalis-core`.

### Statute

A **`Statute`** is a single legal rule: an id, a title, one primary effect, a
list of preconditions, and optional metadata (jurisdiction, version, temporal
validity, discretionary logic). You build one with a fluent builder:

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

### Condition

A **`Condition`** is what must be true for the effect to apply. Conditions are an
enum with concrete leaf variants and recursive combinators:

- Leaves: `Age`, `Income`, `Geographic`, `EntityRelationship`, `DateRange`,
  `ResidencyDuration`, `HasAttribute`, `AttributeEquals`, `Duration`,
  `Percentage`, `SetMembership`, `Pattern`, `Custom`, and more.
- Combinators: `And(..)`, `Or(..)`, `Not(..)`.

Numeric comparisons use **`ComparisonOp`**: `Equal`, `NotEqual`, `GreaterThan`,
`GreaterOrEqual`, `LessThan`, `LessOrEqual`.

### Effect

An **`Effect`** is what happens when the conditions hold: an **`EffectType`**
(`Grant`, `Revoke`, `Obligation`, `Prohibition`, `MonetaryTransfer`,
`StatusChange`, plus composite `Conditional`/`Delayed`/`Compound`), a
description, and a parameter map.

### LegalResult: deterministic vs. discretion

The most important type is **`LegalResult<T>`**. It encodes the project's core
principle that *not everything should be computable*:

```rust
pub enum LegalResult<T> {
    Deterministic(T),           // computed mechanically from the rules
    JudicialDiscretion { .. },  // a human must decide
    Void { reason: String },    // the rule is internally contradictory
}
```

This makes the boundary between "automatable" and "needs a person" explicit and
impossible to ignore. See [ADR-0004](adr/0004-legalresult-deterministic-vs-discretion.md).

---

## 3. Your first statute (the DSL)

You can build statutes in Rust, but the **Legal DSL** is usually the friendlier
authoring surface. DSL files use the `.legalis` extension.

Create `adult-rights.legalis`:

```legalis
STATUTE adult-rights: "Adult Rights Act" {
    JURISDICTION "US"
    VERSION 1

    WHEN AGE >= 18
    THEN GRANT "Full legal capacity"
}
```

Conditions combine with `AND`, `OR`, and `NOT`, can be grouped with parentheses,
and support range/set/pattern operators:

```legalis
STATUTE tax-credit: "Tax Credit Eligibility" {
    JURISDICTION "US"
    VERSION 2
    REQUIRES base-income, residency

    WHEN income BETWEEN 20000 AND 100000
    WHEN AGE >= 25 AND AGE <= 65
    WHEN HAS dependents
    THEN GRANT "tax credit"
    THEN OBLIGATION "file tax return"

    EXCEPTION WHEN income > 90000 "High income exception"
    SUPERSEDES old-tax-credit
}
```

Key DSL elements:

| Element | Keywords |
|---------|----------|
| Declaration | `STATUTE <id>: "<title>" { ... }` |
| Conditions | `WHEN`, `UNLESS`, `AND`, `OR`, `NOT`, `HAS`, `BETWEEN ... AND ...`, `IN [...]`, `LIKE`, `MATCHES` |
| Operators | `>=`, `<=`, `>`, `<`, `==` / `=`, `!=` |
| Effects | `THEN GRANT` / `REVOKE` / `OBLIGATION` / `PROHIBITION`, `DISCRETION "..."` |
| Metadata | `JURISDICTION`, `VERSION`, `EFFECTIVE_DATE`, `EXPIRY_DATE` |
| Relationships | `REQUIRES`, `EXCEPTION`, `AMENDMENT`, `SUPERSEDES` |
| Modules | `IMPORT "path.legalis" [AS alias]` |

Parse it in Rust with `LegalDslParser`:

```rust
use legalis_dsl::LegalDslParser;

let parser = LegalDslParser::new();
let statutes = parser.parse_statutes(include_str!("adult-rights.legalis"))?;
for s in &statutes {
    println!("{} — {}", s.id, s.title);
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

`parse_statute` parses a single statute; `parse_statutes` parses a multi-statute
document. The inverse direction (model → DSL text) is the `DslPrinter`, whose
output style is controlled by `PrinterConfig` (`default`, `compact`, `verbose`).

---

## 4. A worked example: welfare benefits

The repository ships a complete example at `examples/welfare-benefits/`. It
parses a set of welfare statutes from DSL, verifies them, evaluates real
citizens, draws a decision tree, runs a population simulation, and writes an
audit trail. Run it with:

```bash
cargo run -p welfare-benefits
```

The statutes are authored inline as DSL. Here are two of them, verbatim from the
example:

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
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 65 AND INCOME <= 50000
    THEN GRANT "Monthly pension supplement of $300"
}
```

The example's `main` follows a pipeline that is representative of how the crates
fit together:

```rust
use legalis_core::{BasicEntity, LegalEntity};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

# const WELFARE_STATUTES: &str = "";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse statutes from DSL
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(WELFARE_STATUTES)?;

    // 2. Verify them for consistency
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    if result.passed {
        println!("All statutes passed verification");
    }

    // 3. (the example then evaluates individual citizens and records
    //     decisions in an audit trail)

    // 4. Run a population simulation
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;

    println!("Total applications: {}", metrics.total_applications);
    println!("Deterministic: {}", metrics.deterministic_count);
    println!("Discretionary: {}", metrics.discretion_count);
    Ok(())
}
```

Note the shape of the flow: **parse → verify → evaluate → simulate**, with an
audit trail recording each decision. That is the canonical Legalis-RS workflow.

> The example also constructs entities with `BasicEntity::new()` and
> `set_attribute("age", ...)` / `set_attribute("income", ...)`, then evaluates
> each `Condition` against them. `BasicEntity` is the simplest entity type;
> `TypedEntity` offers type-checked attribute access.

---

## 5. Using the command-line tool

The `legalis` binary exposes the whole workflow from the shell. The most common
commands:

```bash
# Parse / validate a DSL file
legalis parse  --input statutes/adult-rights.legalis
legalis validate --input "statutes/*.legalis" --strict

# Verify statutes for logical consistency (fail the build on warnings)
legalis verify --input "statutes/*.legalis" --strict

# Lint and format DSL files
legalis lint   --input "statutes/*.legalis" --fix
legalis format --input statutes/adult-rights.legalis --inplace --style default

# Visualize a statute as a decision tree / flowchart
legalis viz --input statutes/tax-credit.legalis --output tax-credit.mmd \
            --viz-format mermaid    # also: dot, ascii, box

# Diff two versions of a statute
legalis diff --old v1.legalis --new v2.legalis --diff-format markdown

# Explain a statute in natural language
legalis explain --input statutes/tax-credit.legalis --detail detailed

# Analyze complexity / generate an audit report
legalis complexity --input "statutes/*.legalis"
legalis audit --input "statutes/*.legalis" --output audit.txt
```

Scaffolding and project management:

```bash
legalis init my-legal-project           # new project (legalis.toml, layout)
legalis new --name my-rule --template basic --output my-rule.legalis
legalis doctor                          # diagnose the installation
legalis tutorial --topic introduction   # built-in interactive tutorials
```

Running a simulation or the API server straight from the CLI:

```bash
legalis simulate --input "statutes/*.legalis" --population 1000
legalis serve --host 127.0.0.1 --port 3000
```

Global flags apply to every command, including `-v/--verbose` (repeatable),
`-f/--format` (`text`, `json`, `yaml`, `toml`, `table`, `csv`, `html`),
`-q/--quiet`, `--config <file>`, and `--theme`.

Generate shell completions for your shell:

```bash
legalis completions bash > ~/.local/share/bash-completion/completions/legalis
```

> The CLI also has many advanced subcommands (`registry`, `port`, `import`,
> `convert`, `lod`, `graph`, `trace`, `benchmark`, `batch`, `workflow`, an
> interactive `dashboard`/`repl`, and an `offline` queue). Run
> `legalis <command> --help` for the exact flags of any of them.

---

## 6. Verifying statutes

Verification is how you treat "legal bugs as compile errors." From Rust:

```rust
use legalis_verifier::StatuteVerifier;

let verifier = StatuteVerifier::new();
let result = verifier.verify(&statutes);

if !result.passed {
    for error in &result.errors {
        eprintln!("error: {error:?}");
    }
}
for warning in &result.warnings {
    eprintln!("warning: {warning}");
}
```

`verify` returns a `VerificationResult` with `passed`, `errors`, `warnings`, and
`suggestions`. The verifier checks for circular references, dead/unreachable
statutes, contradictions and conflicting effects, redundant conditions, and
constitutional-principle compliance. For true satisfiability checking, build with
`--features smt-solver` (pure-Rust OxiZ solver — no external dependencies).

From the shell, `legalis verify --input "..." --strict` runs the same checks and
returns a non-zero exit code on failure, so it slots into CI.

---

## 7. Running simulations

Simulation answers "what happens when this rule meets a population?" — including
*how much* of the outcome is deterministic vs. discretionary.

```rust
use legalis_sim::{PopulationBuilder, SimEngine};

// Build a population of 1,000 random entities
let population = PopulationBuilder::new().generate_random(1000).build();

// Apply the statutes
let engine = SimEngine::new(statutes, population);
let metrics = engine.run_simulation().await; // run_simulation is async

println!("total applications: {}", metrics.total_applications);
println!("deterministic:      {}", metrics.deterministic_count);
println!("discretionary:      {}", metrics.discretion_count);
println!("void:               {}", metrics.void_count);
```

`run_simulation` is `async` (the engine uses Tokio), so call it from an
`async fn`/`#[tokio::main]`. The returned `SimulationMetrics` separates outcomes
into deterministic, discretionary, and void counts — a direct, measurable read on
where human judgment is required.

You can also drive a simulation from the CLI with
`legalis simulate --input "..." --population N`.

---

## 8. Exporting and interoperating

A verified statute set can be exported to many targets.

From the CLI:

```bash
# Serialize / smart-contract export
legalis export --input rule.legalis --output rule.json    --export-format json
legalis export --input rule.legalis --output rule.sol     --export-format solidity

# Linked Open Data (RDF)
legalis lod --input rule.legalis --output rule.ttl --rdf-format turtle

# Import from / convert between other legal DSLs
legalis import  --input scope.catala --from catala --output rule.json
legalis convert --input scope.catala --from catala --to l4 --output rule.l4
```

The interop layer (`legalis-interop`) handles import/export for Catala, Stipula,
L4, and Akoma Ntoso (among others); `legalis-chain` generates smart contracts
(Solidity, WASM, Ink!, and more); and `legalis-lod` emits RDF/Turtle/JSON-LD for
the semantic web. See the corresponding examples (`legal-dsl-interop`,
`smart-contract-export`, `legal-knowledge-graph`).

---

## 9. Where to go next

- **[Building a Complete Legal System](tutorial-building-a-legal-system.md)** —
  the full statutes → conditions → verification → simulation tutorial.
- **[Architecture Decision Records](adr/README.md)** — why the project is built
  the way it is.
- **[Comparison with other frameworks](comparison-with-other-frameworks.md)** —
  how Legalis-RS relates to Catala, OpenFisca, rule engines, and the
  Akoma Ntoso / LegalRuleML standards.
- **[Deployment guide](deployment.md)** — release builds, the API service, and
  feature flags in production.
- **[Contributing](../CONTRIBUTING.md)** — repo layout, coding policies, and how
  to add a new jurisdiction.
- The `examples/` directory — 35+ runnable examples across jurisdictions and
  features. Each has its own `README.md`.
