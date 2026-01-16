//! Australian Jurisdiction Support for Legalis-RS
//!
//! Comprehensive implementation of Australian law including:
//! - Constitutional law (Commonwealth powers, implied rights)
//! - Contract law (with Australian Consumer Law)
//! - Tort law (with Civil Liability Act reforms)
//! - Employment law (Fair Work Act)
//! - Criminal law (Commonwealth and state)
//! - Family law (Family Law Act 1975)
//! - Property law (Torrens, native title)
//! - Corporate law (Corporations Act 2001)
//!
//! ## Jurisdictional Structure
//!
//! Australia has a federal system with:
//! - Commonwealth (federal) law
//! - Six states (NSW, Vic, Qld, SA, WA, Tas)
//! - Two territories (NT, ACT)
//!
//! ## Key Legislation
//!
//! - Constitution (1901)
//! - Competition and Consumer Act 2010 (incl. ACL)
//! - Fair Work Act 2009
//! - Family Law Act 1975
//! - Corporations Act 2001
//! - Native Title Act 1993
//! - Civil Liability Acts (various states)
//!
//! ## Key Cases
//!
//! - Mabo v Queensland (No 2) (1992) - Native title
//! - Lange v ABC (1997) - Implied freedom
//! - Sullivan v Moody (2001) - Duty of care
//! - Work Choices (2006) - Corporations power

#![allow(missing_docs)]

pub mod common;
pub mod constitution;
pub mod contract;
pub mod corporate;
pub mod criminal;
pub mod employment;
pub mod family;
pub mod property;
pub mod reasoning;
pub mod tort;

// Re-export commonly used types
pub use common::{AustralianCalendar, AustralianCase, AustralianHoliday, Court, StateTerritory};
pub use constitution::{
    CharacterizationAnalyzer, CommonwealthPower, ConstitutionalProvision, ExpressRight,
    ImpliedRight, InconsistencyAnalyzer, PoliticalCommunicationAnalyzer,
};
pub use contract::{
    ConsumerAnalyzer, ConsumerGuarantee, FormationAnalyzer, GuaranteeAnalyzer,
    MisleadingConductAnalyzer, UnfairTermsAnalyzer,
};
pub use corporate::{DirectorsDutiesAnalyzer, DirectorsDuty, InsolventTradingAnalyzer};
pub use criminal::{
    Defence, FaultElement, OffenceAnalyzer, OffenceCategory, SentenceType, SentencingAnalyzer,
};
pub use employment::{
    CompensationCalculator, EligibilityAnalyzer, GeneralProtectionsAnalyzer,
    NationalEmploymentStandard, NesAnalyzer, UnfairDismissalAnalyzer,
};
pub use family::{DivorceAnalyzer, ParentingAnalyzer, PropertyAnalyzer as FamilyPropertyAnalyzer};
pub use property::{IndefeasibilityAnalyzer, NativeTitleAnalyzer, TorrensPrinciple};
pub use reasoning::{AustralianReasoningEngine, ConstitutionalVerifier};
pub use tort::{
    BreachAnalyzer as TortBreachAnalyzer, CausationAnalyzer, DefamationAnalyzer,
    DutyOfCareAnalyzer, NegligenceAnalyzer,
};

use legalis_core::Statute;

// ============================================================================
// Main Statute Builders
// ============================================================================

/// Create Australian Constitution statute
pub fn create_constitution() -> Statute {
    constitution::create_constitution_statute()
}

/// Create Australian Consumer Law statute
pub fn create_acl() -> Statute {
    contract::create_acl_statute()
}

/// Create Civil Liability Act for a state
pub fn create_cla(state: &StateTerritory) -> Statute {
    tort::create_civil_liability_act(state)
}

/// Create Fair Work Act statute
pub fn create_fair_work_act() -> Statute {
    employment::create_fair_work_act()
}

/// Create Criminal Code Act statute
pub fn create_criminal_code_act() -> Statute {
    criminal::create_criminal_code_act()
}

/// Create Family Law Act statute
pub fn create_family_law_act() -> Statute {
    family::create_family_law_act()
}

/// Create Native Title Act statute
pub fn create_native_title_act() -> Statute {
    property::create_native_title_act()
}

/// Create Corporations Act statute
pub fn create_corporations_act() -> Statute {
    corporate::create_corporations_act()
}

/// Create all major Australian statutes
pub fn create_major_statutes() -> Vec<Statute> {
    let mut statutes = vec![
        // Constitutional
        create_constitution(),
        // Criminal
        create_criminal_code_act(),
        criminal::create_crimes_act(),
        // Employment
        create_fair_work_act(),
        // Family
        create_family_law_act(),
        family::create_child_support_act(),
        // Property
        create_native_title_act(),
        // Corporate
        create_corporations_act(),
        corporate::create_asic_act(),
        // Consumer/Competition
        contract::create_acl_statute(),
        contract::create_cca_statute(),
    ];

    // Civil Liability Acts for each state
    for state in StateTerritory::all() {
        statutes.push(tort::create_civil_liability_act(state));
        statutes.push(tort::create_defamation_act(state));
        statutes.push(contract::create_sale_of_goods_statute(state));
    }

    statutes
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_constitution() {
        let statute = create_constitution();
        assert!(statute.id.contains("CONST"));
    }

    #[test]
    fn test_create_acl() {
        let statute = create_acl();
        assert!(statute.id.contains("ACL"));
    }

    #[test]
    fn test_create_major_statutes() {
        let statutes = create_major_statutes();
        // Constitution + ACL + CCA + (CLA + Defamation + SOG) * 8 states = 27
        assert!(statutes.len() >= 20);
    }

    #[test]
    fn test_state_territory_count() {
        assert_eq!(StateTerritory::all().len(), 8);
        assert_eq!(StateTerritory::states().len(), 6);
        assert_eq!(StateTerritory::territories().len(), 2);
    }
}
