# legalis-diff

Statute diffing and change detection for Legalis-RS.

## Overview

`legalis-diff` provides tools for comparing statute versions, detecting changes, and assessing the impact of legislative amendments.

## Features

- **Structural Diff**: Compare statute structure (conditions, effects, metadata)
- **Change Categorization**: Classify changes as added, removed, or modified
- **Impact Assessment**: Evaluate severity of changes (minor, moderate, major, breaking)
- **Change Reports**: Generate human-readable diff reports

## Usage

```rust
use legalis_diff::{StatuteDiff, DiffReport, ChangeType, ImpactLevel};
use legalis_core::Statute;

// Compare two statute versions
let old_statute = /* ... */;
let new_statute = /* ... */;

let diff = StatuteDiff::compare(&old_statute, &new_statute);

// Check for changes
if diff.has_changes() {
    println!("Changes detected:");

    for change in diff.changes() {
        match change.change_type {
            ChangeType::Added => println!("  + {}", change.description),
            ChangeType::Removed => println!("  - {}", change.description),
            ChangeType::Modified => println!("  ~ {}", change.description),
        }
    }
}

// Get impact assessment
let impact = diff.assess_impact();
match impact.level {
    ImpactLevel::Minor => println!("Minor changes"),
    ImpactLevel::Moderate => println!("Moderate changes"),
    ImpactLevel::Major => println!("Major changes - review required"),
    ImpactLevel::Breaking => println!("Breaking changes - full review required"),
}
```

## Change Types

| Type | Description |
|------|-------------|
| `Added` | New element added |
| `Removed` | Element removed |
| `Modified` | Element changed |

## Impact Levels

| Level | Description |
|-------|-------------|
| `Minor` | Cosmetic or documentation changes |
| `Moderate` | Changes to discretionary elements |
| `Major` | Changes to conditions or effects |
| `Breaking` | Fundamental structural changes |

## CLI Integration

```bash
# Compare two statute files
legalis diff old.legalis new.legalis

# Output as JSON
legalis diff old.legalis new.legalis --format json
```

## License

MIT OR Apache-2.0
