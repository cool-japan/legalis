//! United States Jurisdiction Support for Legalis-RS
//!
//! This module provides Common Law tort support for the United States legal system,
//! including:
//!
//! - **Restatement of Torts** (ALI) - Synthesized principles from case law
//! - **Famous tort cases** - Landmark precedents (Palsgraf, Donoghue, etc.)
//! - **Stare decisis** - Case law precedent system
//! - **State-specific variations** - Different legal rules across 50 states
//! - **Choice of Law** - Determining which state's law applies to multi-state disputes
//! - **Uniform Acts** - Tracking UCC and other uniform law adoption across states
//! - **Federal-State Relations** - Preemption analysis and Commerce Clause
//! - **Professional Licensing** - Attorney, physician, and architect licensing across states
//! - **Tax Law Variations** - Income, sales, and corporate tax across 51 jurisdictions
//!
//! ## Common Law vs Civil Law
//!
//! The US legal system (inherited from English Common Law) differs fundamentally
//! from Civil Law systems (Japan, Germany, France) in how legal rules develop:
//!
//! ### Civil Law Approach (大陸法)
//!
//! ```text
//! Legislature
//!     ↓
//! Code/Statute (e.g., 民法709条, BGB §823, Code civil 1240)
//!     ↓
//! Courts apply statute to cases
//! ```
//!
//! ### Common Law Approach (英米法)
//!
//! ```text
//! Case 1 → Precedent A
//!     ↓
//! Case 2 cites Case 1 → Refines Precedent A
//!     ↓
//! Case 3 distinguishes → Exception to Precedent A
//!     ↓
//! Restatement synthesizes → § X: Rule A (non-binding)
//!     ↓
//! Case 4 adopts Restatement § X
//! ```
//!
//! ## Key Differences
//!
//! | Feature | Civil Law | Common Law |
//! |---------|-----------|------------|
//! | Primary Source | Statutes/Codes | Cases/Precedents |
//! | Court Role | Apply code | Make law |
//! | Reasoning | Deductive (code → case) | Analogical (case → case) |
//! | Binding Force | Statute text | Prior holdings (stare decisis) |
//! | Flexibility | Low (legislature must amend) | High (courts distinguish) |
//!
//! ## Why This Matters for Legalis-RS
//!
//! Civil Law modeling uses `Statute` objects (e.g., 民法709条).
//! Common Law modeling uses `Case` objects with `precedent_weight()`.
//!
//! The same tort concept appears differently:
//! - **Civil Law**: Article 709 (statute) → "intent OR negligence"
//! - **Common Law**: Palsgraf (case) → "duty to foreseeable plaintiff"
//!
//! We need both modeling approaches in Legalis-RS.

// ===== Common Law Foundation =====
pub mod cases;
pub mod restatement;

// ===== State-Specific Features (Phase 1) =====
pub mod states;

// ===== Choice of Law (Phase 1D) =====
pub mod choice_of_law;

// ===== Uniform Acts (Phase 1E) =====
pub mod uniform_acts;

// ===== Federal-State Boundary (Phase 1F) =====
pub mod federal;

// ===== Professional Licensing (Phase 3) =====
pub mod professional_licensing;

// ===== Tax Law Variations (Phase 4) =====
pub mod tax;

// ===== Legislative Tracking (Phase 5) =====
pub mod legislative;

// ===== Legal Reasoning Engine (Phase 6) =====
pub mod reasoning;

// ===== Phase 1B: Jurisdiction Expansion =====
// Securities Law (Securities Act 1933, Securities Exchange Act 1934)
pub mod securities;

// Bankruptcy Law (Bankruptcy Code Title 11)
pub mod bankruptcy;

// Immigration Law (Immigration and Nationality Act)
pub mod immigration;

// Antitrust Law (Sherman Act, Clayton Act, FTC Act)
pub mod antitrust;

// ===== Re-exports for Convenience =====

// Common Law cases
pub use cases::{donoghue_v_stevenson, garratt_v_dailey, palsgraf_v_long_island, vosburg_v_putney};

// Restatement sections
pub use restatement::{
    battery_as_statute, iied_as_statute, products_liability_as_statute, section_46_iied,
    section_158_battery, section_402a_products_liability,
};

// State-specific types and utilities
pub use states::{
    registry::{CourtStructure, GeographicRegion, StateMetadata, StateRegistry},
    types::{
        CaseReference, CauseOfAction, DamagesType, LegalTopic, LegalTradition, StateId,
        StateLawVariation, StateRule, StatuteReference,
    },
};

// Choice of law
pub use choice_of_law::{
    analyzer::{ChoiceOfLawApproach, ChoiceOfLawResult, USChoiceOfLawAnalyzer},
    factors::{ContactingFactor, USChoiceOfLawFactors},
    restatement_first::{RestatementFirst, RestatementFirstResult, RestatementFirstRule},
    restatement_second::{RestatementSecond, RestatementSecondResult, Section, Section6Factor},
};

// Uniform acts
pub use uniform_acts::{
    adoption_status::{AdoptionComparison, AdoptionStatus, UniformActComparator},
    ucc::{UCCAdoption, UCCArticle, UCCTracker, UCCVersion},
    upa::{PartnershipActVersion, UPAAdoption, UPATracker},
};

// Federal-state boundary
pub use federal::{
    commerce_clause::{CommerceClauseAnalysis, CommerceClauseResult, DormantCommerceClauseTest},
    preemption::{
        ConflictPreemptionType, FieldPreemptionAnalysis, PreemptionAnalysis, PreemptionResult,
        PreemptionType,
    },
};

// Professional licensing
pub use professional_licensing::{
    architect::{ArchitectLicensing, NCARBStatus, can_use_ncarb_certificate},
    bar_admission::{
        BarAdmissionRequirements, MultijurisdictionalPractice, ProHacViceRules, UBEStatus,
        bar_requirements, can_transfer_ube_score, ube_status,
    },
    medical::{
        IMLCStatus, PrescribingAuthority, TelemedicineRules, is_imlc_member, prescribing_authority,
        telemedicine_requirements,
    },
    types::{LicenseStatus, LicenseType, ProfessionalLicense, ReciprocityType},
};

// Tax law variations
pub use tax::{
    corporate_tax::{
        ApportionmentFormula, CorporateTaxInfo, TaxHavenStatus, apportionment_formula,
        corporate_tax_rate, is_tax_haven,
    },
    income_tax::{
        IncomeTaxStructure, IncomeTaxType, TaxBracket, has_state_income_tax, income_tax_structure,
        no_income_tax_states,
    },
    sales_tax::{
        NexusType, SalesTaxInfo, SalesTaxNexus, has_sales_tax, post_wayfair_nexus,
        state_sales_tax_rate,
    },
};

// Legislative tracking
pub use legislative::{
    constitutional::{
        ConstitutionalPrivacyRight, DirectDemocracyPowers, InitiativeReferendumStatus,
        StateConstitutionalProvisions, constitutional_privacy_right, has_initiative_referendum,
        state_constitutional_provisions,
    },
    policy_tracker::{
        CannabisStatus, DataPrivacyLaw, PolicyAdoptionTracker, RightToRepairStatus,
        cannabis_status, comprehensive_privacy_laws, has_comprehensive_privacy_law,
        right_to_repair_status, states_with_recreational_cannabis,
    },
};

// Legal reasoning engine
pub use reasoning::{
    ComplianceStatus, LegalAnalysis, LegalReasoningEngine, ReasoningError, ReasoningResult,
    ReasoningStep, RiskLevel, UsEvaluationContext, Violation, ViolationSeverity,
    all_federal_statutes, employment_statutes, tax_statutes,
};

// Securities law
pub use securities::{
    AccreditationBasis, AccreditedInvestor, BlueSkyCompliance, Exemption, HoweyTestAnalysis,
    Issuer, IssuerType, Offering, OfferingType, QualifiedInstitutionalBuyer, RegistrationStatus,
    RegulationDRule, SecuritiesError, Security, SecurityType, validate_accredited_investor,
    validate_howey_test, validate_registration, validate_regulation_d,
};

// Bankruptcy law
pub use bankruptcy::{
    BankruptcyCase, BankruptcyChapter, BankruptcyError, ClaimPriority, ClaimType, Creditor, Debtor,
    DebtorType, Discharge,
};

// Immigration law
pub use immigration::{
    GreenCardApplication, ImmigrationStatus, NaturalizationApplication, VisaCategory,
    validate_naturalization_5year,
};

// Antitrust law
pub use antitrust::{
    AntitrustViolation, CompetitiveConcern, ConcentrationLevel, HsrFiling, MarketPower,
    MergerAnalysis, MonopolyAnalysis,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restatement_functions_available() {
        // Verify Restatement sections are accessible
        let battery = section_158_battery();
        assert!(battery.name.contains("158"));

        let iied = section_46_iied();
        assert!(iied.name.contains("46"));

        let products = section_402a_products_liability();
        assert!(products.name.contains("402A"));
    }

    #[test]
    fn test_cases_available() {
        // Verify famous cases are accessible
        let palsgraf = palsgraf_v_long_island();
        assert_eq!(palsgraf.year, 1928);

        let donoghue = donoghue_v_stevenson();
        assert_eq!(donoghue.year, 1932);

        let garratt = garratt_v_dailey();
        assert_eq!(garratt.year, 1955);

        let vosburg = vosburg_v_putney();
        assert_eq!(vosburg.year, 1891);
    }

    #[test]
    fn test_statute_versions_available() {
        // Verify statute representations of Restatement sections
        let battery_statute = battery_as_statute();
        assert_eq!(
            battery_statute.jurisdiction,
            Some("US-RESTATEMENT".to_string())
        );

        let iied_statute = iied_as_statute();
        assert!(iied_statute.is_valid());

        let products_statute = products_liability_as_statute();
        assert_eq!(products_statute.version, 2); // Restatement (Second)
    }
}
