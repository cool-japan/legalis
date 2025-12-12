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

## Future: SMT Solver Integration

The crate is designed to integrate with Z3 or similar SMT solvers for rigorous formal verification of legal consistency. This would enable:

- Satisfiability checking for complex condition combinations
- Proof generation for verification results
- Temporal logic verification (LTL/CTL)

## License

MIT OR Apache-2.0
