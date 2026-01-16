//! Canada Corporate Law Module
//!
//! This module provides comprehensive modeling of Canadian corporate law,
//! including the CBCA and provincial corporations acts.
//!
//! ## Key Areas
//!
//! - **Incorporation**: Federal (CBCA) and provincial incorporation
//! - **Director Duties**: s.122 CBCA duties (care, fiduciary)
//! - **Shareholder Remedies**: Oppression (s.241), derivative action (s.239)
//! - **Fundamental Changes**: Amalgamation, arrangement, continuance
//! - **Securities**: Prospectus exemptions, reporting issuers
//!
//! ## Director Duties Framework
//!
//! Under CBCA s.122:
//! - **Fiduciary Duty** (s.122(1)(a)): Act honestly and in good faith with a
//!   view to the best interests of the corporation
//! - **Duty of Care** (s.122(1)(b)): Exercise care, diligence, and skill that
//!   a reasonably prudent person would exercise
//!
//! ## Key Cases
//!
//! - **BCE Inc v 1976 Debentureholders** [2008] SCC 69: Stakeholder interests
//! - **Peoples v Wise** [2004] SCC 68: Business judgment rule
//! - **Kosmopoulos v Constitution Insurance** [1987] SCC: Corporate veil
//! - **Ebrahimi v Westbourne Galleries** [1973]: Quasi-partnership

mod directors;
mod error;
mod oppression;
mod types;

pub use directors::{
    BusinessJudgmentFactors, ConflictDetails, ConflictNature, DecisionContext, DecisionMakerType,
    DirectorDutyAnalyzer, DirectorDutyFacts, DirectorDutyResult, ImpactNature, ImpactSeverity,
    InformationLevel, StakeholderAnalysis, StakeholderImpact,
};
pub use error::{CorporateError, CorporateResult};
pub use oppression::{
    AllegedConduct, CompensationType, ComplainantInfo, ConductType, ExitOffer, ExpectationSource,
    ExpectationStrength, OppressionAnalyzer, OppressionContext, OppressionFacts, OppressionRemedy,
    OppressionResult, ReasonableExpectation, SharePurchaser, ValuationBasis,
};
pub use types::{
    AmalgamationType, ApprovalRequirement, BusinessJudgmentElement, ComplainantType,
    ContinuanceDirection, CorporateArea, CorporateCase, CorporateStatus, CorporateType,
    DerivativeRequirement, DirectorDisqualification, DirectorDuty, DirectorQualification,
    DutyBreach, FiduciaryBreachType, FundamentalChange, IncorporationJurisdiction,
    OppressionConduct, OppressionElement, ProspectusExemption, ReportingIssuerStatus, SecurityType,
    ShareClass, ShareRight, ShareStructure, ShareholderRemedy, StakeholderInterest,
    StakeholderType,
};

// Re-export legalis-core types
pub use legalis_core::{Effect, EffectType, Statute};

use crate::common::Province;

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Canada Business Corporations Act
pub fn create_cbca() -> Statute {
    Statute::new(
        "CBCA",
        "Canada Business Corporations Act, RSC 1985, c C-44",
        Effect::new(
            EffectType::Grant,
            "Federal business corporations statute. Covers incorporation, corporate governance, \
             director duties (s.122), shareholder remedies (oppression s.241, derivative s.239), \
             fundamental changes, and securities. Directors owe fiduciary duty to corporation \
             and duty of care (business judgment rule applies).",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Canada Not-for-profit Corporations Act
pub fn create_cnca() -> Statute {
    Statute::new(
        "CNCA",
        "Canada Not-for-profit Corporations Act, SC 2009, c 23",
        Effect::new(
            EffectType::Grant,
            "Federal not-for-profit corporations statute. Similar governance framework \
             to CBCA but adapted for non-share capital corporations. Director duties, \
             member remedies, and fundamental changes.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create provincial business corporations act
pub fn create_provincial_corporations_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("OBCA", "Business Corporations Act, RSO 1990, c B.16"),
        Province::BritishColumbia => ("BCBCA", "Business Corporations Act, SBC 2002, c 57"),
        Province::Alberta => ("ABCA", "Business Corporations Act, RSA 2000, c B-9"),
        Province::Quebec => ("QBCA", "Business Corporations Act, CQLR c S-31.1"),
        Province::Manitoba => ("MBCA", "The Corporations Act, CCSM c C225"),
        Province::Saskatchewan => ("SBCA", "The Business Corporations Act, SS 1978, c B-10"),
        Province::NovaScotia => ("NSCA", "Companies Act, RSNS 1989, c 81"),
        Province::NewBrunswick => ("NBBCA", "Business Corporations Act, SNB 1981, c B-9.1"),
        Province::NewfoundlandLabrador => ("NLCA", "Corporations Act, RSNL 1990, c C-36"),
        Province::PrinceEdwardIsland => ("PEICA", "Companies Act, RSPEI 1988, c C-14"),
        _ => ("BCA", "Business Corporations Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial business corporations statute. Similar structure to CBCA with \
             provincial variations. Covers incorporation, governance, director duties, \
             and shareholder remedies.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create securities act
pub fn create_securities_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("OSA", "Securities Act, RSO 1990, c S.5"),
        Province::BritishColumbia => ("BCSA", "Securities Act, RSBC 1996, c 418"),
        Province::Alberta => ("ASA", "Securities Act, RSA 2000, c S-4"),
        Province::Quebec => ("QSA", "Securities Act, CQLR c V-1.1"),
        _ => ("SA", "Securities Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Obligation,
            "Provincial securities legislation. Governs prospectus requirements, \
             continuous disclosure, insider trading, and take-over bids. \
             CSA harmonization through National Instruments.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Competition Act
pub fn create_competition_act() -> Statute {
    Statute::new(
        "CA",
        "Competition Act, RSC 1985, c C-34",
        Effect::new(
            EffectType::Prohibition,
            "Federal competition law. Prohibits anti-competitive practices including \
             conspiracies, abuse of dominance, deceptive marketing. Merger review \
             by Competition Bureau. Criminal and civil tracks.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Investment Canada Act
pub fn create_investment_canada_act() -> Statute {
    Statute::new(
        "ICA",
        "Investment Canada Act, RSC 1985, c 28 (1st Supp)",
        Effect::new(
            EffectType::Grant,
            "Federal foreign investment review statute. Reviews of investments by \
             non-Canadians for net benefit to Canada. National security review \
             for sensitive sectors.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create all corporate law statutes for a province
pub fn create_corporate_statutes(province: &Province) -> Vec<Statute> {
    vec![
        create_cbca(),
        create_cnca(),
        create_provincial_corporations_act(province),
        create_securities_act(province),
        create_competition_act(),
        create_investment_canada_act(),
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cbca() {
        let statute = create_cbca();
        assert!(statute.title.contains("Canada Business Corporations"));
    }

    #[test]
    fn test_create_cnca() {
        let statute = create_cnca();
        assert!(statute.title.contains("Not-for-profit"));
    }

    #[test]
    fn test_create_obca() {
        let statute = create_provincial_corporations_act(&Province::Ontario);
        assert!(statute.title.contains("Business Corporations Act"));
    }

    #[test]
    fn test_create_bcbca() {
        let statute = create_provincial_corporations_act(&Province::BritishColumbia);
        assert!(statute.title.contains("Business Corporations Act"));
    }

    #[test]
    fn test_create_securities_act() {
        let statute = create_securities_act(&Province::Ontario);
        assert!(statute.title.contains("Securities Act"));
    }

    #[test]
    fn test_create_competition_act() {
        let statute = create_competition_act();
        assert!(statute.title.contains("Competition Act"));
    }

    #[test]
    fn test_create_investment_canada_act() {
        let statute = create_investment_canada_act();
        assert!(statute.title.contains("Investment Canada"));
    }

    #[test]
    fn test_create_corporate_statutes() {
        let statutes = create_corporate_statutes(&Province::Ontario);
        assert!(statutes.len() >= 6);
    }
}
