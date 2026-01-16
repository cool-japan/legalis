//! Australian Family Law
//!
//! Implementation of Family Law Act 1975 (Cth) including:
//! - Divorce (Part VI)
//! - Parenting orders (Part VII)
//! - Property and maintenance (Part VIII)
//! - Child support
//!
//! ## Key Legislation
//!
//! - Family Law Act 1975 (Cth)
//! - Child Support (Assessment) Act 1989 (Cth)
//!
//! ## Key Cases
//!
//! - Stanford v Stanford [2012] HCA 52 - Property approach
//! - Mallet v Mallet (1984) - Homemaker contributions

pub mod fla;
pub mod types;

// Re-export commonly used types
pub use fla::{
    DivorceAnalyzer, DivorceFacts, DivorceResult, ParentingAnalyzer, ParentingFacts,
    ParentingResult, PropertyAnalyzer, PropertyFacts, PropertyResult,
};
pub use types::{
    AdditionalConsideration, CareLevel, ContributionType, FamilyCase, FamilyViolenceType,
    FutureNeedsFactor, ParentalResponsibility, ParentingOrderType, PrimaryConsideration,
    ProtectionOrderType, RelationshipType, TimeArrangement,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Family Law Act 1975 statute
pub fn create_family_law_act() -> Statute {
    Statute::new(
        "AU-FLA-1975",
        "Family Law Act 1975 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Federal jurisdiction over divorce, parenting, and property division",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Child Support Assessment Act statute
pub fn create_child_support_act() -> Statute {
    Statute::new(
        "AU-CSA-1989",
        "Child Support (Assessment) Act 1989 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Child support assessment and collection",
        ),
    )
    .with_jurisdiction("AU")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_family_law_act() {
        let statute = create_family_law_act();
        assert!(statute.id.contains("FLA"));
    }

    #[test]
    fn test_divorce_analysis() {
        let facts = DivorceFacts {
            jurisdictional_connection: true,
            separated_12_months: true,
            ..Default::default()
        };

        let result = DivorceAnalyzer::analyze(&facts);
        assert!(result.eligible);
    }

    #[test]
    fn test_property_analysis() {
        let facts = PropertyFacts {
            total_assets: 500_000.0,
            total_liabilities: 100_000.0,
            ..Default::default()
        };

        let result = PropertyAnalyzer::analyze(&facts);
        assert_eq!(result.total_pool, 400_000.0);
    }
}
