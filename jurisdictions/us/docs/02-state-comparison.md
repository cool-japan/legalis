# State Comparison Guide

**Analyzing legal variations across all 51 US jurisdictions**

## Overview

One of the unique features of Legalis-US is the ability to systematically compare how different states handle the same legal issue. This is essential for:

- **Legal research**: Finding persuasive precedent from similar jurisdictions
- **Multi-state practice**: Understanding different legal regimes
- **Comparative law**: Identifying majority vs. minority rules
- **Policy analysis**: Tracking interstate legal trends

---

## Quick Start

```rust
use legalis_us::states::{
    comparator::StateLawComparator,
    types::LegalTopic
};

let comparator = StateLawComparator::new();

// Compare comparative negligence rules across 5 states
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &["CA", "NY", "TX", "NC", "FL"],
);

// View results
println!("Majority rule: {:?}", comparison.majority_rule);
println!("Total states: {}", comparison.total_states);

// Get states following majority rule
for state in comparison.majority_states() {
    println!("✅ {}: Follows majority rule", state.name);
}

// Get states following minority rules
for state in comparison.minority_states() {
    println!("⚠️ {}: Follows minority rule", state.name);
}
```

---

## Comparative Negligence: A Complete Example

### The Legal Issue

When plaintiff and defendant are both at fault, how is liability allocated?

**Four Different Rules Used in US:**

1. **Pure Comparative Negligence** (13 states)
   - Plaintiff recovers damages reduced by their fault percentage
   - No bar to recovery even if plaintiff is 99% at fault
   - Example: $100k damages, plaintiff 80% at fault → recovers $20k

2. **Modified Comparative Negligence - 51% Bar** (24 states)
   - Plaintiff recovers if their fault ≤ 50%
   - Complete bar if plaintiff > 50% at fault
   - Example: $100k damages, plaintiff 50% at fault → recovers $50k
   - Example: $100k damages, plaintiff 51% at fault → recovers $0

3. **Modified Comparative Negligence - 50% Bar** (9 states)
   - Plaintiff recovers if their fault < 50%
   - Complete bar if plaintiff ≥ 50% at fault
   - Example: $100k damages, plaintiff 49% at fault → recovers $51k
   - Example: $100k damages, plaintiff 50% at fault → recovers $0

4. **Contributory Negligence** (5 states) ⚠️ MINORITY RULE
   - Complete bar to recovery if plaintiff has ANY fault
   - Even 1% fault = $0 recovery
   - Harsh rule from 19th century, abandoned by most states

### State-by-State Breakdown

```rust
// Compare all 51 jurisdictions
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &[
        // All 50 states + DC
        "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA",
        "HI", "ID", "IL", "IN", "IA", "KS", "KY", "LA", "ME", "MD",
        "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ",
        "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC",
        "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC"
    ],
);

// Generate markdown report
let report = comparator.generate_report(&comparison);
println!("{}", report);
```

**Output:**
```markdown
# Comparative Negligence Comparison

## Majority Rule: Modified Comparative Negligence 51%
**States**: 24 (47%)

## Breakdown by Rule:

### Pure Comparative Negligence (13 states)
- Alaska, Arizona, California, Florida, Kentucky, Louisiana
- Michigan, Mississippi, Missouri, New Mexico, New York
- Rhode Island, Washington

### Modified Comparative Negligence 51% (24 states)
- Colorado, Connecticut, Delaware, Hawaii, Idaho, Illinois, Indiana, Iowa
- Maine, Massachusetts, Minnesota, Montana, Nebraska, Nevada, New Hampshire
- New Jersey, North Dakota, Ohio, Oklahoma, Oregon, Pennsylvania, South Carolina
- Texas, Vermont, West Virginia, Wisconsin, Wyoming

### Modified Comparative Negligence 50% (9 states)
- Arkansas, Georgia, Kansas, Nebraska, North Dakota, South Dakota
- Tennessee, Utah, West Virginia

### Contributory Negligence (5 states) ⚠️ MINORITY
- Alabama, District of Columbia, Maryland, North Carolina, Virginia
```

---

## Similarity Scoring

### How It Works

The comparator calculates **similarity scores** between state pairs:

- **1.0**: Identical rules
- **0.5**: Different rules
- **0.0**: Missing data for comparison

```rust
// Compare California with other states
let ca_similarities = comparison.similarity_matrix[0]; // CA is first state

for (i, state) in states.iter().enumerate() {
    let similarity = ca_similarities[i];
    match similarity {
        1.0 => println!("{}: Identical to CA", state),
        0.5 => println!("{}: Different from CA", state),
        _ => println!("{}: No data", state),
    }
}
```

### Finding Similar Jurisdictions

```rust
// Find states most similar to New York for comparative negligence
let ny_variations = comparison.by_state.get(&StateId::from_code("NY"));

let similar_states = comparison.by_state.iter()
    .filter(|(state_id, variation)| {
        variation.rule == ny_variations.unwrap().rule
    })
    .map(|(state_id, _)| state_id.name.clone())
    .collect::<Vec<_>>();

println!("States with same rule as NY: {:?}", similar_states);
// Output: States with pure comparative negligence
```

**Use Case**: If researching NY law on comparative negligence, look for persuasive precedent from CA, FL, AK, AZ, KY, LA, MI, MS, MO, NM, RI, WA (all pure comparative negligence states).

---

## Advanced: Joint and Several Liability

### The Legal Issue

When multiple defendants are liable, how is responsibility allocated?

**Three Main Approaches:**

1. **Traditional Joint and Several Liability**
   - Each defendant liable for full amount
   - Plaintiff can collect entire judgment from any defendant
   - Defendants use contribution to reallocate among themselves

2. **Several Liability Only**
   - Each defendant liable only for their percentage
   - Plaintiff bears risk if one defendant is insolvent
   - "Tort reform" approach adopted by most states

3. **Modified Joint and Several**
   - Joint liability if defendant's fault exceeds threshold (e.g., 50%)
   - Several liability below threshold
   - Hybrid approach

```rust
// Compare joint and several liability rules
let comparison = comparator.compare_states(
    LegalTopic::JointAndSeveralLiability,
    &["CA", "TX", "GA", "NJ", "PA"],
);

// California: Several liability for non-economic damages only
// Texas: Modified joint & several (>50% = joint)
// Georgia: Traditional joint & several (retains old rule)
// New Jersey: Traditional joint & several
// Pennsylvania: Modified joint & several (>60% = joint)
```

### Interstate Patterns

**Trend**: Most states have **abolished or limited** traditional joint and several liability since 1980s "tort reform" movement.

**Traditional Joint & Several (Minority)**: Georgia, New Jersey, Rhode Island

**Several Only (Majority)**: 42 states

**Modified (Hybrid)**: Pennsylvania (60% threshold), Massachusetts (50%), Connecticut, Vermont

---

## Identifying Majority vs. Minority Rules

### Why It Matters

Courts often cite **majority rule** as persuasive when:
- State has no clear precedent on an issue
- State is considering adopting a new rule
- Showing trend in American law

**Minority rule** indicates:
- State may be outlier
- Rule may be outdated or in process of change
- Opportunity for reform advocacy

### Automatic Detection

```rust
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &all_states,
);

// Library automatically determines majority/minority
assert!(comparison.majority_rule.is_some());

match &comparison.majority_rule {
    Some(StateRule::ModifiedComparativeNegligence51) => {
        println!("Majority of states use 51% bar");
    }
    _ => {}
}

// Contributory negligence states are minority (only 5)
let contributory_states = comparison.minority_states();
assert_eq!(contributory_states.len(), 5);
```

---

## Regional Patterns

### Analyzing by Region

```rust
use legalis_us::states::registry::{StateRegistry, GeographicRegion};

let registry = StateRegistry::new();

// Get all Southern states
let southern_states: Vec<_> = registry.states_by_region(GeographicRegion::South)
    .iter()
    .map(|meta| meta.state_id.code.as_str())
    .collect();

// Compare just Southern states
let southern_comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &southern_states,
);

println!("Southern states comparison:");
println!("{}", comparator.generate_report(&southern_comparison));
```

**Regional Patterns Found:**

- **South**: More likely to retain contributory negligence (NC, VA, MD, DC, AL)
- **West**: Trend toward pure comparative negligence (CA, AK, AZ, NM, WA)
- **Midwest**: Mix of modified 51% and 50% bars

---

## Special Case: Louisiana

Louisiana is the **only Civil Law state** in the US (inherited from French colonial period).

```rust
let la_id = StateId::louisiana();
assert_eq!(la_id.legal_tradition, LegalTradition::CivilLaw);

// Louisiana uses different terminology
// "Delict" instead of "tort"
// "Obligations" instead of "contracts"
// Louisiana Civil Code instead of Common Law precedent
```

**Comparative Law Analysis:**

Louisiana's approach can be compared with:
- **France**: Code civil Article 1240 (similar Civil Law tradition)
- **Germany**: BGB §823 (another Civil Law system)
- **Japan**: Minpō Article 709 (adopted from German Civil Law)

---

## Practical Applications

### 1. Legal Research

**Problem**: Client injured in Colorado. Colorado has limited case law on specific negligence issue.

**Solution**: Find similar states with better-developed case law.

```rust
// Find states with same rule as Colorado
let co_variations = comparison.by_state.get(&StateId::from_code("CO"));
let similar_states = comparison.by_state.iter()
    .filter(|(_, v)| v.rule == co_variations.unwrap().rule)
    .collect::<Vec<_>>();

// Research case law from similar states for persuasive precedent
```

### 2. Multi-State Firm Strategy

**Problem**: Law firm with offices in 10 states needs to understand tort law variations.

**Solution**: Generate comparison report for all office locations.

```rust
let office_states = vec!["CA", "NY", "TX", "FL", "IL", "PA", "OH", "GA", "NC", "MA"];
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &office_states,
);

// Identify states requiring different legal strategies
let pure_comp = comparison.by_state.iter()
    .filter(|(_, v)| matches!(v.rule, StateRule::PureComparativeNegligence))
    .count();

let contributory = comparison.by_state.iter()
    .filter(|(_, v)| matches!(v.rule, StateRule::ContributoryNegligence))
    .count();

println!("Pure comparative: {} states", pure_comp);
println!("Contributory negligence: {} states (requires special attention)", contributory);
```

### 3. Policy Analysis

**Problem**: State legislature considering changing comparative negligence rule.

**Solution**: Analyze what other states have done.

```rust
// Show nationwide breakdown
let all_states = vec![/* all 51 */];
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &all_states,
);

let report = comparator.generate_report(&comparison);

// Report shows:
// - 13 states pure comparative (25%)
// - 24 states modified 51% (47%) ← MAJORITY
// - 9 states modified 50% (18%)
// - 5 states contributory (10%) ← MINORITY, declining
```

**Policy Insight**: Contributory negligence is dying out (only 5 states remain). Trend is toward modified comparative negligence.

---

## API Reference

### Core Functions

```rust
// Create comparator
let comparator = StateLawComparator::new();

// Compare specific states
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &["CA", "NY", "TX"]
);

// Find states with specific rule
let pure_comp_states = comparator.states_with_rule(
    LegalTopic::ComparativeNegligence,
    StateRule::PureComparativeNegligence
);

// Generate report
let report = comparator.generate_report(&comparison);
```

### StateComparison Type

```rust
pub struct StateComparison {
    pub majority_rule: Option<StateRule>,
    pub minority_rules: Vec<StateRule>,
    pub by_state: HashMap<StateId, StateLawVariation>,
    pub similarity_matrix: Vec<Vec<f64>>,
    pub total_states: usize,
}

// Helper methods
impl StateComparison {
    pub fn majority_states(&self) -> Vec<&StateId>;
    pub fn minority_states(&self) -> Vec<&StateId>;
    pub fn includes_state(&self, state_code: &str) -> bool;
}
```

---

## Available Legal Topics

Currently supported for comparison:

- `ComparativeNegligence`
- `JointAndSeveralLiability`
- `DramShopLiability`
- `GoodSamaritanProtection`
- `ProductsLiability`
- `EmotionalDistress`
- `PunitiveDamages`
- `MedicalMalpractice`

More topics coming in future versions.

---

## Best Practices

1. **Start broad, then narrow**: Compare all states first, then focus on relevant subset
2. **Check dates**: Laws change over time; verify current law
3. **Read cases**: Similarity in rules doesn't mean identical application
4. **Consider policy**: Understand WHY states differ, not just HOW
5. **Use regional analysis**: Geography often predicts legal approach

---

**Next**: [Professional Licensing Guide](03-professional-licensing.md)
