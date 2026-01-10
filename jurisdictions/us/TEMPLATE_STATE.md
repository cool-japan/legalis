# State Implementation Template

This template shows the correct pattern for implementing new state modules, based on Illinois (successfully implemented and tested).

## Template File Structure

```rust
//! [State Name] State Law
//!
//! [State Name] tort law features:
//! - [Comparative Negligence Rule]
//! - [Other notable features]
//! - [Geographic/legal significance]

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// [State Name] state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct [StateName]Law;

impl [StateName]Law {
    /// Get [State Name] state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("[XX]", "[State Name]", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::ModifiedComparative51, // or ModifiedComparative50, PureComparativeNegligence, ContributoryNegligence
        )
        .with_statute(
            StatuteReference::new("[Citation]")
                .with_title("[Title]")
                .with_year([Year]),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt([Year], [Month], [Day]).unwrap())
        .with_notes(
            "[Description of the rule, when adopted, key features]"
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::SeveralLiabilityOnly, // or JointAndSeveralLiability, ModifiedJointAndSeveral { threshold_percent: XX }
        )
        .with_statute(
            StatuteReference::new("[Citation]")
                .with_title("[Title]")
                .with_year([Year]),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt([Year], [Month], [Day]).unwrap())
        .with_notes(
            "[Description]"
        )
    }

    /// Get all state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![
            Self::comparative_negligence(),
            Self::joint_and_several_liability(),
            // Add other variations as needed
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_[state]_state_id() {
        let [state] = [StateName]Law::state_id();
        assert_eq!([state].code, "[XX]");
        assert_eq!([state].name, "[State Name]");
    }

    #[test]
    fn test_comparative_negligence() {
        let comp_neg = [StateName]Law::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::[RuleVariant]);
        assert_eq!(comp_neg.state.code, "[XX]");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_joint_several_liability() {
        let joint_several = [StateName]Law::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::[RuleVariant]);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = [StateName]Law::state_variations();
        assert!(variations.len() >= 2);
        assert!(variations
            .iter()
            .any(|v| v.topic == LegalTopic::ComparativeNegligence));
        assert!(variations
            .iter()
            .any(|v| v.topic == LegalTopic::JointAndSeveralLiability));
    }
}
```

## Key Points

### 1. StateId requires 3 arguments
```rust
StateId::new("IL", "Illinois", LegalTradition::CommonLaw)
```

### 2. StateLawVariation Builder Pattern
```rust
StateLawVariation::new(state, topic, rule)  // Constructor with 3 required args
    .with_statute(StatuteReference)          // Optional
    .with_adoption_date(NaiveDate)          // Optional
    .with_notes("...")                       // Optional
```

### 3. StateRule Variants Available

**Negligence:**
- `StateRule::PureComparativeNegligence` - CA, NY, FL, etc.
- `StateRule::ModifiedComparative50` - 12 states (plaintiff can recover if ≤50% at fault)
- `StateRule::ModifiedComparative51` - TX, IL, 21 states (plaintiff can recover if <51% at fault)
- `StateRule::ContributoryNegligence` - NC, VA, MD, DC, AL (complete bar)

**Joint Liability:**
- `StateRule::JointAndSeveralLiability` - Traditional rule
- `StateRule::SeveralLiabilityOnly` - Proportionate share only
- `StateRule::ModifiedJointAndSeveral { threshold_percent: 60 }` - Hybrid (struct variant)

**Damages:**
- `StateRule::DamagesCap { damage_type, cap_amount, conditions }` - Struct variant
- `StateRule::NoDamagesCap`

### 4. StatuteReference Usage
```rust
StatuteReference::new("[Citation]")      // Required
    .with_title("[Title]")              // Optional
    .with_year(YYYY)                    // Optional
```

**Methods available:**
- `new(citation)` - Constructor
- `with_title(title)` - Add title
- `with_year(year)` - Add year

**Methods NOT available:**
- ❌ `with_text()` - Not supported
- ❌ `with_effective_date()` - Not supported
- ❌ `with_jurisdiction()` - Not supported

Use NaiveDate for dates in `with_adoption_date()`.

### 5. Module Registration

After creating the file, update `src/states/mod.rs`:

```rust
// Add module declaration
pub mod [state_name];

// Add re-export
pub use [state_name]::[StateName]Law;
```

### 6. Testing

Each state module should have at least 4 tests:
1. State ID verification
2. Comparative negligence rule
3. Joint and several liability rule
4. State variations collection

Run tests with:
```bash
cargo nextest run --all-features
cargo clippy --all-features  # Ensure 0 warnings
```

## Example: Illinois (Reference Implementation)

See `src/states/illinois.rs` for the complete, working reference implementation.

## Tier 1 States to Implement (Priority Order)

1. **Pennsylvania (PA)** - Modified 51%, Fair Share Act, Third Circuit
2. **Ohio (OH)** - Modified 51%, tort reform, Midwest bellwether
3. **Georgia (GA)** - Modified 50%, Southern growth
4. **Massachusetts (MA)** - Modified 51%, New England leader
5. **Washington (WA)** - Pure comparative, Pacific Northwest
6. **Michigan (MI)** - Modified 51%, automotive industry
7. **New Jersey (NJ)** - Modified 51%, proximity to NY

## Tier 2 States (Regional Representatives)

- **North Carolina (NC)** - Contributory negligence (MINORITY RULE)
- **Virginia (VA)** - Contributory negligence (MINORITY RULE)
- **Maryland (MD)** - Contributory negligence (MINORITY RULE)
- **Alabama (AL)** - Contributory negligence (MINORITY RULE)
- **Tennessee (TN)** - Modified 50%
- **Arizona (AZ)** - Pure comparative
- **Colorado (CO)** - Modified 50%
- **Minnesota (MN)** - Modified 51%, Better Law approach for choice of law
- **Wisconsin (WI)** - Modified 51%
- **Missouri (MO)** - Pure comparative
- **Indiana (IN)** - Modified 51%

## Notes

- Louisiana is special (Civil Law) and is already implemented in Phase 1
- DC is also contributory negligence (5th jurisdiction with this minority rule)
- Most states are Modified 51% (21 states) or Modified 50% (12 states)
- Pure comparative is less common (13 states including CA, NY, FL)
- Contributory negligence is rare (only 5 jurisdictions)

## Implementation Speed

With this template:
- Simple state (2 variations): ~100-150 lines, 15 minutes
- Complex state (4+ variations): ~200-250 lines, 30 minutes
- All 44 remaining states: Estimated 20-25 hours of work

## Quality Standards

- ✅ Zero compiler warnings (cargo clippy)
- ✅ All tests passing (cargo nextest run)
- ✅ Accurate statutory citations
- ✅ Correct comparative negligence classification
- ✅ Implementation Policy: IMPLEMENT ALL features, don't simplify
