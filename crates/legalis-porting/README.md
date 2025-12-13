# legalis-porting

Cross-jurisdiction statute porting for Legalis-RS.

## Overview

`legalis-porting` enables translation and adaptation of legal statutes between different jurisdictions, handling cultural and legal system differences.

## Features

- **Cross-Jurisdiction Translation**: Port statutes between legal systems
- **Cultural Adaptation**: Inject jurisdiction-specific parameters
- **Compatibility Reports**: Assess porting feasibility
- **Change Tracking**: Document all adaptations made

## Usage

```rust
use legalis_porting::{PortingEngine, PortingConfig, CompatibilityReport};
use legalis_i18n::JurisdictionRegistry;
use legalis_core::Statute;

// Create porting engine
let registry = JurisdictionRegistry::default();
let engine = PortingEngine::new(registry);

// Port a statute from US to Japan
let us_statute = /* ... */;
let config = PortingConfig::new("US", "JP");

let result = engine.port(&us_statute, &config)?;

// Check compatibility
println!("Compatibility score: {}%", result.compatibility_score);

// Review changes
for change in result.changes {
    println!("  - {}: {}", change.field, change.description);
}

// Get ported statute
let jp_statute = result.statute;
```

## Porting Process

1. **Analysis**: Examine source statute structure
2. **Compatibility Check**: Assess legal system compatibility
3. **Cultural Injection**: Apply target jurisdiction parameters
4. **Validation**: Verify ported statute validity
5. **Report Generation**: Document all changes

## Compatibility Levels

| Score | Level | Description |
|-------|-------|-------------|
| 90-100% | High | Direct port possible |
| 70-89% | Medium | Minor adaptations needed |
| 50-69% | Low | Significant changes required |
| <50% | Incompatible | Manual review required |

## CLI Integration

```bash
# Port statute to different jurisdiction
legalis port input.legalis --target JP

# Generate compatibility report
legalis port input.legalis --target JP --format report
```

## License

MIT OR Apache-2.0
