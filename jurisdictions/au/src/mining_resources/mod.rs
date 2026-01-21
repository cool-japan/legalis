//! Australian Mining and Resources Law
//!
//! Comprehensive implementation of Australian mining legislation across states and territories.
//!
//! ## Key Legislation
//!
//! ### State/Territory Mining Acts
//!
//! - Mining Act 1978 (WA) - Western Australia
//! - Mineral Resources Act 1989 (Qld) - Queensland
//! - Mining Act 1992 (NSW) - New South Wales
//! - Mineral Resources (Sustainable Development) Act 1990 (Vic) - Victoria
//! - Mining Act 1971 (SA) - South Australia
//! - Mineral Resources Development Act 1995 (Tas) - Tasmania
//! - Mineral Titles Act 2010 (NT) - Northern Territory
//!
//! ### Commonwealth Legislation
//!
//! - Native Title Act 1993 (Cth) - Future act processes for mining
//! - Environment Protection and Biodiversity Conservation Act 1999 (Cth)
//! - Aboriginal and Torres Strait Islander Heritage Protection Act 1984 (Cth)
//!
//! ## Mining Tenure Types
//!
//! | Tenure | Purpose | Typical Term |
//! |--------|---------|--------------|
//! | Exploration Licence | Exploration activities | 5 years |
//! | Mining Lease | Production/extraction | 21 years |
//! | Retention Licence | Hold identified resource | 5 years |
//! | General Purpose Lease | Infrastructure | 21 years |
//!
//! ## Native Title
//!
//! Mining on native title land requires compliance with the "future act" regime:
//! - Right to Negotiate (s.31 NTA)
//! - Indigenous Land Use Agreements (ILUA)
//! - Expedited procedure (for exploration with low impact)
//!
//! ## Environmental Requirements
//!
//! - EPBC Act referral for Matters of National Environmental Significance (MNES)
//! - State environmental assessment/EIS
//! - Environmental authority/licence for operations
//! - Progressive rehabilitation obligations
//! - Mine closure planning and financial assurance
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::mining_resources::*;
//!
//! // Validate a mining tenure
//! let validation = validate_tenure(&tenure, current_date);
//! if !validation.is_valid {
//!     for issue in &validation.issues {
//!         println!("Issue: {} - {}", issue.category, issue.description);
//!     }
//! }
//!
//! // Calculate royalty
//! let royalty = calculate_royalty(
//!     MineralType::Gold,
//!     MiningJurisdiction::WA,
//!     quantity,
//!     unit_value,
//! );
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{MiningError, Result};
pub use types::{
    EnvironmentalApproval, EnvironmentalApprovalType, HeritageSignificance, HeritageSite,
    HeritageSiteType, MineClosurePlan, MineralType, MiningJurisdiction, MiningProject,
    MiningTenure, NativeTitleStatus, ProductionCapacity, ProductionUnit, ProjectPhase,
    RoyaltyCalculation, RoyaltyType, TenureHolder, TenureStatus, TenureType,
};
pub use validator::{
    ClosurePlanValidationResult, EiaRequirement, EpbcTrigger, ExplorationActivity,
    ExplorationActivityType, ExplorationValidationResult, IssueCategory, IssueSeverity,
    StateAssessmentLevel, TenureValidationResult, ValidationIssue, calculate_royalty,
    determine_eia_requirements, validate_closure_plan, validate_exploration_programme,
    validate_tenure,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Native Title Act 1993 (mining provisions) statute
pub fn create_native_title_act_mining() -> Statute {
    Statute::new(
        "AU-NTA-1993-MINING",
        "Native Title Act 1993 (Cth) - Mining Future Act Provisions",
        Effect::new(
            EffectType::Obligation,
            "Establishes future act regime for mining activities on native title land, \
             including right to negotiate and ILUA processes",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create EPBC Act 1999 statute (mining focus)
pub fn create_epbc_act_mining() -> Statute {
    Statute::new(
        "AU-EPBC-1999-MINING",
        "Environment Protection and Biodiversity Conservation Act 1999 (Cth) - Mining Provisions",
        Effect::new(
            EffectType::Prohibition,
            "Prohibits mining actions that significantly impact Matters of National \
             Environmental Significance without Commonwealth approval",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create state Mining Act statute
pub fn create_mining_act(jurisdiction: MiningJurisdiction) -> Statute {
    let act_name = jurisdiction.mining_act();
    let code = format!("AU-{:?}-MINING", jurisdiction);

    Statute::new(
        &code,
        act_name,
        Effect::new(
            EffectType::Grant,
            "Establishes framework for granting mining tenures, regulating exploration \
             and mining operations, and imposing royalty obligations",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_native_title_mining() {
        let statute = create_native_title_act_mining();
        assert!(statute.id.contains("NTA"));
        assert!(statute.title.contains("Native Title"));
    }

    #[test]
    fn test_create_epbc_mining() {
        let statute = create_epbc_act_mining();
        assert!(statute.id.contains("EPBC"));
        assert!(statute.title.contains("Environment Protection"));
    }

    #[test]
    fn test_create_state_mining_acts() {
        let wa = create_mining_act(MiningJurisdiction::WA);
        assert!(wa.title.contains("Mining Act 1978"));

        let qld = create_mining_act(MiningJurisdiction::QLD);
        assert!(qld.title.contains("Mineral Resources Act"));
    }
}
