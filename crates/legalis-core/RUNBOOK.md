# Legalis-Core Runbook

This runbook provides step-by-step guides for common legal modeling scenarios using legalis-core.

## Table of Contents

1. [Modeling Tax Credits](#modeling-tax-credits)
2. [Implementing Eligibility Checking](#implementing-eligibility-checking)
3. [Building Decision Trees](#building-decision-trees)
4. [Working with Legal Transactions](#working-with-legal-transactions)
5. [Case Law Analysis](#case-law-analysis)
6. [Temporal Legal Rules](#temporal-legal-rules)
7. [Conflict Resolution](#conflict-resolution)

---

## Modeling Tax Credits

### Scenario
Model a tax credit for low-income individuals that grants $1,000 if:
- Age >= 18
- Income < $50,000
- Resident of the US

### Implementation

```rust
use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp, TemporalValidity};
use chrono::NaiveDate;

// Create the tax credit statute
let tax_credit = Statute::new(
    "tax-credit-2025",
    "Low Income Tax Credit",
    Effect::new(EffectType::MonetaryTransfer, "Tax credit of $1,000")
        .with_parameter("amount", "1000")
        .with_parameter("currency", "USD")
)
.with_precondition(Condition::Age {
    operator: ComparisonOp::GreaterOrEqual,
    value: 18
})
.with_precondition(Condition::Income {
    operator: ComparisonOp::LessThan,
    value: 50000
})
.with_precondition(Condition::Geographic {
    region_type: crate::RegionType::Country,
    region_id: "US".to_string()
})
.with_temporal_validity(
    TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
)
.with_jurisdiction("US")
.with_version(1);

// Validate the statute
let errors = tax_credit.validate();
assert!(errors.is_empty(), "Statute has validation errors: {:?}", errors);
```

**Key Points:**
- Use `MonetaryTransfer` effect type for financial effects
- Add parameters for amount and currency
- Combine multiple preconditions with `with_precondition`
- Set temporal validity for time-bound rules
- Always validate before use

---

## Implementing Eligibility Checking

### Scenario
Check if applicants are eligible for multiple benefits and return results.

### Implementation

```rust
use legalis_core::workflows::{EligibilityChecker, WorkflowContext};
use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};

// Define multiple benefit programs
let benefits = vec![
    Statute::new(
        "senior-discount",
        "Senior Citizen Discount",
        Effect::new(EffectType::Grant, "10% discount on public transit")
    )
    .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),

    Statute::new(
        "student-discount",
        "Student Discount",
        Effect::new(EffectType::Grant, "50% discount on public transit")
    )
    .with_precondition(Condition::age(ComparisonOp::LessThan, 25))
    .with_precondition(Condition::HasAttribute {
        attribute_name: "student_status".to_string()
    }),
];

// Create a workflow context for an applicant
let mut context = WorkflowContext::new();
context.set_age(22);
context.set_attribute("student_status", "enrolled");

// Check eligibility
let checker = EligibilityChecker::new(benefits);
let results = checker.check_eligibility(&context);

// Process results
for result in results {
    if result.is_eligible {
        println!("Eligible for: {}", result.statute_id);
        if let Some(effect) = result.effect {
            println!("  Benefit: {}", effect.description);
        }
    } else {
        println!("Not eligible for: {}", result.statute_id);
        if let Some(reason) = result.reason {
            println!("  Reason: {}", reason);
        }
    }
}
```

**Key Points:**
- Use `WorkflowContext` to hold applicant data
- `EligibilityChecker` evaluates all statutes automatically
- Check `is_eligible` field in results
- Access `reason` for ineligibility explanations

---

## Building Decision Trees

### Scenario
Create a decision tree for determining the appropriate legal entity type for a business.

### Implementation

```rust
use legalis_core::workflows::{DecisionNode, WorkflowContext};
use legalis_core::{Condition, ComparisonOp, EffectType};

// Build the decision tree
let entity_selector = DecisionNode::condition(
    Condition::custom("revenue_check", "annual_revenue < 100000"),
    // If low revenue (< $100k)
    DecisionNode::condition(
        Condition::custom("owner_check", "single_owner"),
        DecisionNode::outcome("Sole Proprietorship", Some(EffectType::Grant)),
        DecisionNode::outcome("Partnership", Some(EffectType::Grant))
    ),
    // If high revenue (>= $100k)
    DecisionNode::condition(
        Condition::custom("liability_check", "wants_limited_liability"),
        DecisionNode::outcome("LLC or Corporation", Some(EffectType::Grant)),
        DecisionNode::outcome("General Partnership", Some(EffectType::Grant))
    )
);

// Evaluate for a specific case
let mut context = WorkflowContext::new();
context.set_attribute("annual_revenue", "75000");
context.set_attribute("single_owner", "true");

match entity_selector.evaluate(&context) {
    Ok(recommendation) => println!("Recommended: {}", recommendation),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Key Points:**
- Nest `DecisionNode::condition` for complex logic
- Use `DecisionNode::outcome` for leaf nodes
- Custom conditions can encode domain-specific logic
- Decision trees provide human-readable reasoning paths

---

## Working with Legal Transactions

### Scenario
Update multiple statutes atomically with validation and rollback support.

### Implementation

```rust
use legalis_core::transactions::{TransactionBuilder, BatchProcessor};
use legalis_core::{Statute, Effect, EffectType};

// Create a transaction for updating tax laws
let mut txn = TransactionBuilder::new()
    .with_description("2025 Tax Law Updates")
    .with_metadata("author", "Legislative Committee")
    .with_metadata("session", "2025-Q1")
    .build();

// Add multiple operations
let old_tax = Statute::new("tax-1", "Old Tax", Effect::grant("Old credit"));
let new_tax = Statute::new("tax-1", "New Tax", Effect::grant("New credit"));

txn.add_statute(Statute::new("tax-2", "Additional Tax", Effect::grant("New deduction")));
txn.update_statute(old_tax, new_tax);
txn.remove_statute("tax-obsolete");

// Validate and commit
match txn.commit() {
    Ok(result) => println!("Transaction committed successfully: {:?}", result),
    Err(error) => {
        println!("Transaction failed: {:?}", error);
        txn.rollback();
    }
}

// Batch processing multiple transactions
let mut batch = BatchProcessor::new();
batch.add_transaction(txn);
// Add more transactions...

let results = batch.execute();
for (i, result) in results.iter().enumerate() {
    println!("Transaction {}: {:?}", i, result);
}
```

**Key Points:**
- Use `TransactionBuilder` for metadata and description
- Group related changes in a single transaction
- Validation happens automatically before commit
- Use `BatchProcessor` for multiple independent transactions
- Rollback support ensures data consistency

---

## Case Law Analysis

### Scenario
Build a case law database and find relevant precedents.

### Implementation

```rust
use legalis_core::case_law::{CaseDatabase, Case, CaseRule, Court, PrecedentWeight};
use chrono::NaiveDate;

// Create a case database
let mut db = CaseDatabase::new();

// Add landmark cases
let brown_v_board = Case::new(
    "brown-v-board-1954",
    "Brown v. Board of Education",
    Court::SupremeCourt,
    NaiveDate::from_ymd_opt(1954, 5, 17).unwrap()
)
.with_rule(CaseRule {
    summary: "Separate educational facilities are inherently unequal".to_string(),
    conditions: vec![],
    outcome: "Violation of Equal Protection Clause".to_string(),
})
.with_facts("Segregated public schools for black and white students".to_string())
.with_holding("Racial segregation in public schools is unconstitutional".to_string());

db.add_case(brown_v_board);

// Query the database
let query = db.query()
    .court(Court::SupremeCourt)
    .year_range(1950, 1960)
    .search_holding("segregation");

for case in query.execute() {
    println!("Found: {} ({})", case.name, case.decision_date);
}

// Find similar cases
let similar = db.find_similar_cases("brown-v-board-1954", 0.3);
for result in similar {
    println!("Similar case: {} (score: {:.2})", result.case_id, result.score);
    println!("  Reason: {}", result.reason);
}

// Track precedent relationships
db.add_precedent(
    "later-case-id",
    "brown-v-board-1954",
    PrecedentWeight::Binding,
    "Followed the principle of equal protection"
);
```

**Key Points:**
- Use `CaseDatabase` for organizing case law
- `CaseQuery` provides fluent search interface
- Full-text search across facts, holdings, and ratio
- Similarity search finds analogous cases
- Track precedent relationships with weights

---

## Temporal Legal Rules

### Scenario
Model laws that change over time with amendments and sunset clauses.

### Implementation

```rust
use legalis_core::{Statute, Effect, EffectType, TemporalValidity};
use chrono::{NaiveDate, Utc};

// Original law with sunset clause
let original_law = Statute::new(
    "privacy-act-2020",
    "Data Privacy Act",
    Effect::new(EffectType::Obligation, "Must obtain user consent")
)
.with_temporal_validity(
    TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
        .with_enacted_at(Utc::now())
)
.with_version(1);

// Check if law is currently in effect
let check_date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
if original_law.temporal_validity.is_in_effect(check_date) {
    println!("Law is in effect on {}", check_date);
}

// Amended version
let amended_law = Statute::new(
    "privacy-act-2020",
    "Data Privacy Act (Amended)",
    Effect::new(EffectType::Obligation, "Must obtain explicit user consent")
)
.with_temporal_validity(
    TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2023, 6, 1).unwrap())
        .with_amended_at(Utc::now())
)
.with_version(2);

// Compare versions
let diff = original_law.diff(&amended_law);
println!("Changes between versions:");
for change in diff.changes {
    println!("  - {:?}", change);
}
```

**Key Points:**
- Use `TemporalValidity` for all time-related metadata
- Set expiry dates for sunset clauses
- Track enactment and amendment timestamps
- Use `diff()` to compare statute versions
- Increment version numbers for amendments

---

## Conflict Resolution

### Scenario
Detect and resolve conflicts between competing statutes.

### Implementation

```rust
use legalis_core::{
    Statute, Effect, EffectType, TemporalValidity,
    StatuteConflictAnalyzer, ConflictResolution
};
use chrono::NaiveDate;

// Federal law (higher authority)
let federal_law = Statute::new(
    "federal-minimum-wage",
    "Federal Minimum Wage Law",
    Effect::new(EffectType::Obligation, "Minimum wage of $7.25/hour")
)
.with_jurisdiction("US")
.with_temporal_validity(
    TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2009, 7, 24).unwrap())
);

// State law (lower authority)
let state_law = Statute::new(
    "state-minimum-wage",
    "State Minimum Wage Law",
    Effect::new(EffectType::Obligation, "Minimum wage of $15/hour")
)
.with_jurisdiction("US-CA")
.with_temporal_validity(
    TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
);

// Analyze conflict
let resolution = StatuteConflictAnalyzer::resolve(&federal_law, &state_law);

match resolution {
    ConflictResolution::NoConflict => {
        println!("No conflict detected - both laws can coexist");
    }
    ConflictResolution::FirstPrevails(reason) => {
        println!("Federal law prevails: {}", reason);
    }
    ConflictResolution::SecondPrevails(reason) => {
        println!("State law prevails: {}", reason);
    }
    ConflictResolution::Unresolvable(reason) => {
        println!("Cannot automatically resolve: {}", reason);
    }
}

// Detect contradictions in a set of statutes
let statutes = vec![federal_law, state_law];
let contradictions = StatuteConflictAnalyzer::detect_contradictions(&statutes);

for contradiction in contradictions {
    println!("Contradiction found:");
    println!("  Type: {:?}", contradiction.contradiction_type);
    println!("  Statutes: {} vs {}", contradiction.statute1_id, contradiction.statute2_id);
}
```

**Key Points:**
- Use `StatuteConflictAnalyzer` for automated conflict detection
- Hierarchy (lex superior): Federal > State > Local
- Temporal (lex posterior): Newer laws prevail over older
- Specificity (lex specialis): More specific prevails over general
- Handle `Unresolvable` conflicts manually

---

## Best Practices

### 1. Always Validate Statutes
```rust
let statute = /* ... */;
let errors = statute.validate();
if !errors.is_empty() {
    for error in errors {
        eprintln!("Validation error: {}", error);
    }
}
```

### 2. Use Type-Safe Effects
```rust
use legalis_core::typed_effects::{MonetaryEffect, MonetaryType};

let tax = MonetaryEffect::new(MonetaryType::Tax, 1000)
    .with_currency("USD")
    .with_frequency("annual");
```

### 3. Leverage Const Collections for Performance
```rust
use legalis_core::const_collections::ConditionSet;

// Stack-allocated for small, known-size collections
let mut conditions = ConditionSet::<5>::new();
conditions.push(Condition::age(ComparisonOp::GreaterOrEqual, 18)).unwrap();
```

### 4. Document Legal Intent
```rust
let statute = Statute::new(
    "clear-id",
    "Descriptive Title",
    Effect::new(EffectType::Grant, "Clear description of effect")
)
.with_discretion_logic("Explain when judicial discretion applies");
```

### 5. Use Transactions for Data Integrity
```rust
let mut txn = Transaction::new();
// Add all related changes
txn.commit().expect("Transaction should succeed");
```

---

## Troubleshooting

### Issue: Condition Never Evaluates to True
**Solution:** Check that context has required attributes:
```rust
let mut context = WorkflowContext::new();
context.set_age(25);  // Don't forget to set all required attributes
```

### Issue: Validation Errors on Valid Statutes
**Solution:** Ensure all required fields are set:
```rust
let statute = Statute::new(id, title, effect)  // Required
    .with_jurisdiction("US")                    // Often required
    .with_version(1);                          // Recommended
```

### Issue: Transaction Fails to Commit
**Solution:** Check validation errors:
```rust
match txn.commit() {
    Err(TransactionResult::ValidationFailed { errors }) => {
        for error in errors {
            println!("Fix this: {}", error);
        }
    }
    _ => {}
}
```

---

## Additional Resources

- [API Documentation](https://docs.rs/legalis-core)
- [Examples Directory](../examples/)
- [Performance Guide](./PERFORMANCE.md)
- [Architecture Decisions](./ADR.md)
