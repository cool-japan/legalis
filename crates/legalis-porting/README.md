# legalis-porting

Cross-jurisdiction statute porting for Legalis-RS.

## Overview

`legalis-porting` enables translation and adaptation of legal statutes between different jurisdictions, handling cultural and legal system differences.

## Features

### Core Porting
- **Cross-Jurisdiction Translation**: Port statutes between legal systems
- **Cultural Adaptation**: Inject jurisdiction-specific parameters
- **Compatibility Reports**: Assess porting feasibility and generate detailed reports
- **Change Tracking**: Document all adaptations made during porting
- **Partial Porting**: Port specific sections of statutes
- **Reverse Porting**: Analyze porting from target back to source

### Intelligence & Validation
- **AI-Assisted Adaptation**: Generate cultural adaptation suggestions using LLM
- **Conflict Detection**: Identify conflicts with target jurisdiction laws
- **Semantic Preservation**: Validate that legal meaning is preserved
- **Risk Assessment**: Evaluate risks in ported statutes
- **Similar Statute Finding**: Find equivalent statutes across jurisdictions
- **Automatic Term Replacement**: Replace legal terms with local equivalents
- **Context-Aware Parameter Adjustment**: Adjust values based on context

### Workflow & Compliance
- **Legal Expert Review Workflow**: Submit ported statutes for expert review
- **Automated Compliance Checking**: Check compliance with target regulations
- **Porting Workflow Management**: Track multi-step porting processes
- **Version Control**: Manage versioned ported statutes

### Bilateral Cooperation
- **Bilateral Legal Agreement Templates**: Create agreements between jurisdictions
- **Regulatory Equivalence Mapping**: Map equivalent regulations
- **Batch Porting**: Port multiple statutes efficiently

## Usage

```rust
use legalis_porting::{PortingEngine, PortingOptions};
use legalis_i18n::{Jurisdiction, Locale, CulturalParams, LegalSystem};
use legalis_core::{Statute, Effect, EffectType};

// Create source and target jurisdictions
let us_jurisdiction = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
    .with_legal_system(LegalSystem::CommonLaw)
    .with_cultural_params(CulturalParams::for_country("US"));

let jp_jurisdiction = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
    .with_legal_system(LegalSystem::CivilLaw)
    .with_cultural_params(CulturalParams::japan());

// Create porting engine
let engine = PortingEngine::new(us_jurisdiction, jp_jurisdiction);

// Create a statute
let statute = Statute::new(
    "adult-rights",
    "Adult Rights Act",
    Effect::new(EffectType::Grant, "Full legal capacity"),
);

// Configure porting options
let options = PortingOptions {
    apply_cultural_params: true,
    translate_terms: true,
    generate_report: true,
    ..Default::default()
};

// Port the statute
let ported = engine.port_statute(&statute, &options)?;

// Review changes
for change in &ported.changes {
    println!("  - {:?}: {}", change.change_type, change.description);
}

// Generate compatibility report
let report = engine.generate_report(&[statute]);
println!("Compatibility score: {:.1}%", report.compatibility_score * 100.0);
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
