# legalis-audit

Audit trail and decision logging for Legalis-RS.

## Overview

`legalis-audit` provides comprehensive audit logging for legal decisions, enabling accountability, compliance reporting, and decision replay for legal systems.

## Features

- **Immutable Audit Trail**: Hash-chained records for tamper detection
- **Decision Recording**: Full context capture for each decision
- **Compliance Reports**: Generate reports for regulatory requirements
- **Query Interface**: Search by statute, subject, time range
- **Integrity Verification**: Detect tampering or corruption

## Usage

```rust
use legalis_audit::{
    AuditTrail, AuditRecord, EventType, Actor,
    DecisionContext, DecisionResult,
};
use uuid::Uuid;

// Create audit trail
let trail = AuditTrail::new();

// Record a decision
let record = AuditRecord::new(
    EventType::AutomaticDecision,
    Actor::System { component: "eligibility-engine".to_string() },
    "adult-rights".to_string(),
    subject_id,
    DecisionContext::default(),
    DecisionResult::Deterministic {
        effect_applied: "Full legal capacity".to_string(),
        parameters: HashMap::new(),
    },
    None,
);

trail.record(record)?;

// Query records
let records = trail.query_by_statute("adult-rights")?;
let recent = trail.query_by_time_range(start, end)?;

// Verify integrity
match trail.verify_integrity() {
    Ok(true) => println!("Audit trail intact"),
    Ok(false) => println!("Integrity check failed"),
    Err(e) => println!("Tamper detected: {:?}", e),
}

// Generate compliance report
let report = trail.generate_compliance_report()?;
```

## Event Types

| Type | Description |
|------|-------------|
| `AutomaticDecision` | System-made deterministic decision |
| `DiscretionaryReview` | Decision requiring human review |
| `HumanOverride` | Human override of automatic decision |
| `Appeal` | Appeal or review request |
| `StatuteModified` | Statute was changed |
| `SimulationRun` | Simulation execution |

## Decision Results

| Result | Description |
|--------|-------------|
| `Deterministic` | Automatic decision with effect |
| `RequiresDiscretion` | Needs human judgment |
| `Void` | Invalid due to logical error |
| `Overridden` | Human override applied |

## Integrity Features

- **Hash Chain**: Each record includes hash of previous record
- **Record Hashing**: Individual record integrity verification
- **Tamper Detection**: Automatic detection of modifications
- **Chain Validation**: Full trail verification

## CLI Integration

```bash
# Generate audit report
legalis audit input.legalis --output report.json

# Verify audit trail integrity
legalis audit --verify trail.json
```

## License

MIT OR Apache-2.0
