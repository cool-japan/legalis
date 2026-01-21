//! Australian Consumer Law - Enforcement and Product Safety
//!
//! Comprehensive implementation of ACL enforcement mechanisms and product safety.
//! This module complements the ACL provisions in the contract module.
//!
//! ## Key Legislation
//!
//! - Competition and Consumer Act 2010 (Cth)
//! - Australian Consumer Law (Schedule 2)
//! - Competition and Consumer (Country of Origin) Information Standard 2016
//!
//! ## Product Safety (Part 3-3)
//!
//! ### Mandatory Safety Standards
//!
//! | Product Category | Key Requirements |
//! |------------------|------------------|
//! | Children's toys | Small parts test, toxicity limits |
//! | Baby products | Stability, entrapment hazards |
//! | Electrical goods | Wiring, insulation standards |
//! | Cosmetics | Ingredient restrictions |
//! | Swimming aids | Buoyancy, labeling |
//!
//! ### Mandatory Injury Reporting (s.131)
//!
//! Suppliers must report within **2 days** if they become aware:
//! - Death associated with use of consumer goods
//! - Serious injury/illness requiring medical treatment
//!
//! ## ACCC Enforcement Powers
//!
//! ### Enforcement Hierarchy
//!
//! | Action | Typical Use |
//! |--------|-------------|
//! | Infringement notice | Minor/first offences |
//! | s.87B Undertaking | Negotiated resolution |
//! | Civil penalty | Serious contraventions |
//! | Criminal prosecution | Deliberate/systemic conduct |
//! | Injunction | Ongoing harmful conduct |
//!
//! ### Penalty Calculation
//!
//! | Recipient Type | Multiplier | Example (2,500 units) |
//! |----------------|------------|------------------------|
//! | Individual | 1x | $782,500 |
//! | Body corporate | 5x | $3,912,500 |
//! | Listed corporation | 10x | $7,825,000 |
//!
//! ## Unsolicited Consumer Agreements
//!
//! ### Permitted Contact Times
//!
//! - Monday-Friday: 9am-6pm
//! - Saturday: 9am-5pm
//! - Sunday/Public Holidays: Not permitted
//!
//! ### Cooling-off Period
//!
//! 10 business days from agreement date (s.82)
//!
//! ## Country of Origin
//!
//! ### Safe Harbour Defences
//!
//! | Claim | Requirements |
//! |-------|--------------|
//! | "Made in Australia" | Substantially transformed + 50% costs |
//! | "Product of Australia" | 100% Australian + transformed |
//! | "Grown in Australia" | 100% grown in Australia |
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::consumer_law::*;
//!
//! // Assess product safety
//! let safety = assess_product_safety(
//!     "Widget",
//!     ProductCategory::ChildrensToys,
//!     true,   // Has safety standard
//!     true,   // Meets standard
//!     false,  // No known hazard
//!     None,
//!     0,      // No injury reports
//! );
//!
//! // Calculate penalty
//! let penalty = calculate_penalty(
//!     AclContravention::FalseRepresentations,
//!     RecipientType::BodyCorporate,
//!     0,
//! );
//!
//! // Validate country of origin claim
//! let claim_result = validate_country_of_origin_claim(&claim);
//! if !claim_result.safe_harbour_available {
//!     println!("Issues: {:?}", claim_result.issues);
//! }
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{ConsumerLawError, Result};
pub use types::{
    AclContravention,
    // Country of Origin
    CountryOfOriginClaim,
    CountryOfOriginClaimDetails,
    CourtUndertaking,
    // ACCC Enforcement
    EnforcementActionType,
    InfringementNotice,
    InfringementNoticeStatus,
    InjuryReport,
    InjuryType,
    // Lay-by
    LayByAgreement,
    LayByStatus,
    PermittedContactTime,
    ProductCategory,
    ProductRecall,
    // Product Safety
    ProductSafetyStatus,
    RecallType,
    RecipientType,
    SafetyStandard,
    // Unsolicited Agreements
    UnsolicitedAgreementType,
    UnsolicitedConsumerAgreement,
};
pub use validator::{
    CountryOfOriginAssessment,
    InfringementNoticeAssessment,
    LayByComplianceResult,
    ProductSafetyAssessment,
    UnsolicitedAgreementComplianceResult,
    assess_infringement_notice,
    // Product Safety
    assess_product_safety,
    calculate_cooling_off_end_date,
    // ACCC Enforcement
    calculate_penalty,
    // Country of Origin
    validate_country_of_origin_claim,
    validate_injury_report_timing,
    // Lay-by
    validate_layby_agreement,
    // Unsolicited Agreements
    validate_unsolicited_agreement,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Competition and Consumer Act 2010 (product safety provisions) statute
pub fn create_product_safety_statute() -> Statute {
    Statute::new(
        "AU-ACL-PRODUCT-SAFETY",
        "Australian Consumer Law Part 3-3 (Product Safety)",
        Effect::new(
            EffectType::Prohibition,
            "Prohibits supply of unsafe goods and goods not complying with \
             mandatory safety standards; requires mandatory injury reporting",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create ACCC enforcement provisions statute
pub fn create_accc_enforcement_statute() -> Statute {
    Statute::new(
        "AU-CCA-ENFORCEMENT",
        "Competition and Consumer Act 2010 - ACCC Enforcement Provisions",
        Effect::new(
            EffectType::Grant,
            "Grants ACCC powers to investigate contraventions, issue infringement \
             notices, accept undertakings, and seek civil penalties and orders",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create unsolicited consumer agreements statute
pub fn create_unsolicited_agreements_statute() -> Statute {
    Statute::new(
        "AU-ACL-UNSOLICITED",
        "Australian Consumer Law Part 3-2 Division 2 (Unsolicited Consumer Agreements)",
        Effect::new(
            EffectType::Obligation,
            "Imposes requirements for door-to-door sales and telemarketing, \
             including cooling-off rights and permitted contact times",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create country of origin statute
pub fn create_country_of_origin_statute() -> Statute {
    Statute::new(
        "AU-ACL-COUNTRY-ORIGIN",
        "Australian Consumer Law Part 5-3 (Country of Origin)",
        Effect::new(
            EffectType::Obligation,
            "Establishes requirements for country of origin claims including \
             'Made in Australia' safe harbour defences",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_product_safety_statute() {
        let statute = create_product_safety_statute();
        assert!(statute.id.contains("PRODUCT-SAFETY"));
        assert!(statute.title.contains("Product Safety"));
    }

    #[test]
    fn test_create_accc_enforcement_statute() {
        let statute = create_accc_enforcement_statute();
        assert!(statute.id.contains("ENFORCEMENT"));
        assert!(statute.title.contains("ACCC"));
    }

    #[test]
    fn test_create_unsolicited_agreements_statute() {
        let statute = create_unsolicited_agreements_statute();
        assert!(statute.id.contains("UNSOLICITED"));
    }

    #[test]
    fn test_create_country_of_origin_statute() {
        let statute = create_country_of_origin_statute();
        assert!(statute.id.contains("COUNTRY-ORIGIN"));
    }
}
