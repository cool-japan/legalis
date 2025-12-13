# legalis-verifier

Formal verification for Legalis-RS legal statutes.

## Overview

This crate provides static analysis and verification tools for detecting logical inconsistencies, circular references, and constitutional conflicts in legal statutes. It serves as a "compiler" for laws, catching bugs before they become enacted legislation.

## Features

- Circular reference detection
- Dead statute detection (unsatisfiable conditions)
- Constitutional compliance checking
- Logical contradiction detection between statutes
- Extensible principle checking system

## Usage

### Basic Verification

```rust
use legalis_verifier::{StatuteVerifier, verify_integrity};
use legalis_core::{Statute, Effect, EffectType};

let statutes = vec![
    Statute::new("statute-1", "First Law", Effect::new(EffectType::Grant, "Permission")),
    Statute::new("statute-2", "Second Law", Effect::new(EffectType::Revoke, "Permission")),
];

let verifier = StatuteVerifier::new();
let result = verifier.verify(&statutes);

if result.passed {
    println!("All statutes verified successfully!");
} else {
    for error in result.errors {
        eprintln!("Error: {}", error);
    }
}

// Warnings and suggestions
for warning in result.warnings {
    println!("Warning: {}", warning);
}
```

### Using verify_integrity Helper

```rust
let result = verify_integrity(&statutes)?;
println!("Passed: {}", result.passed);
```

## Verification Checks

### 1. Circular Reference Detection

Detects when statutes form dependency cycles:
```
Statute A references Statute B
Statute B references Statute A
```

### 2. Dead Statute Detection

Identifies statutes with contradictory conditions that can never be satisfied:
```
WHEN AGE >= 18 AND AGE < 18  // Always false
```

### 3. Constitutional Compliance

Checks against configurable constitutional principles:

```rust
let principles = vec![
    ConstitutionalPrinciple {
        id: "equality".to_string(),
        name: "Equal Protection".to_string(),
        description: "All persons are equal under the law".to_string(),
        check: PrincipleCheck::NoDiscrimination,
    },
];

let verifier = StatuteVerifier::with_principles(principles);
```

### 4. Logical Contradiction Detection

Finds statutes with conflicting effects on the same conditions.

## Error Types

```rust
pub enum VerificationError {
    CircularReference(String),
    DeadStatute { statute_id: String },
    ConstitutionalConflict { statute_id: String, principle: String },
    LogicalContradiction(String),
    Ambiguity(String),
}
```

## Verification Result

```rust
pub struct VerificationResult {
    pub passed: bool,
    pub errors: Vec<VerificationError>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

impl VerificationResult {
    pub fn pass() -> Self;
    pub fn fail(errors: Vec<VerificationError>) -> Self;
    pub fn with_warning(self, warning: impl Into<String>) -> Self;
    pub fn with_suggestion(self, suggestion: impl Into<String>) -> Self;
    pub fn merge(&mut self, other: VerificationResult);
}
```

## Constitutional Principles

```rust
pub struct ConstitutionalPrinciple {
    pub id: String,
    pub name: String,
    pub description: String,
    pub check: PrincipleCheck,
}

pub enum PrincipleCheck {
    NoDiscrimination,      // No discrimination based on protected attributes
    RequiresProcedure,     // Requires procedural safeguards
    NoRetroactivity,       // Must not be retroactive
    Custom(String),        // Custom check with description
}
```

## SMT Solver Integration

The verifier supports optional Z3 SMT solver integration for rigorous formal verification. Enable with the `z3-solver` feature:

```toml
[dependencies]
legalis-verifier = { version = "0.2", features = ["z3-solver"] }
```

### SMT-Based Verification Features

When `z3-solver` is enabled:

- **Satisfiability Checking**: Formally proves whether conditions can be satisfied
- **Tautology Verification**: Checks if conditions are always true
- **Contradiction Detection**: Rigorously proves when conditions contradict
- **Implication Checking**: Verifies logical implications (cond1 => cond2)
- **Counterexample Generation**: Provides concrete variable assignments

### Usage Example

```rust
#[cfg(feature = "z3-solver")]
{
    use legalis_verifier::{create_z3_context, SmtVerifier};
    use legalis_core::{Condition, ComparisonOp};

    let ctx = create_z3_context();
    let mut verifier = SmtVerifier::new(&ctx);

    let cond1 = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    };

    let cond2 = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    };

    // Check implication: Age >= 21 implies Age >= 18
    assert!(verifier.implies(&cond1, &cond2)?);

    // Get a model (counterexample)
    if let Some(model) = verifier.get_model(&cond1)? {
        println!("Example: age = {}", model["age"]);
    }
}
```

### Automatic Fallback

The verifier automatically falls back to heuristic checking if:
- The `z3-solver` feature is not enabled
- The SMT solver fails or times out

This ensures verification always works, with optional enhanced precision.

## Complexity Analysis

The verifier includes comprehensive complexity metrics for statutes:

```rust
use legalis_verifier::{analyze_complexity, complexity_report};

let metrics = analyze_complexity(&statute);
println!("Complexity Level: {}", metrics.complexity_level);
println!("Score: {}/100", metrics.complexity_score);

// Generate report for multiple statutes
let report = complexity_report(&statutes);
println!("{}", report);
```

### Complexity Metrics

- **condition_count**: Number of preconditions
- **condition_depth**: Maximum nesting level
- **logical_operator_count**: AND, OR, NOT operations
- **condition_type_count**: Number of distinct condition types
- **has_discretion**: Whether discretionary logic is present
- **cyclomatic_complexity**: Measure of code paths (1 + decisions)
- **complexity_score**: Overall score (0-100)
- **complexity_level**: Simple, Moderate, Complex, or Very Complex

### Complexity Levels

- **Simple (0-25)**: Few conditions, minimal complexity
- **Moderate (26-50)**: Some nesting or multiple conditions
- **Complex (51-75)**: Deep nesting or many logical operators
- **Very Complex (76-100)**: Consider simplification

## License

MIT OR Apache-2.0
