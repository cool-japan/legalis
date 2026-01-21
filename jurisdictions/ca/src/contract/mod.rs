//! Canada Contract Law
//!
//! Contract law analysis for Canadian common law provinces and Quebec civil law.
//!
//! # Overview
//!
//! This module covers contract law in Canada's bijural system:
//!
//! - **Common Law Provinces**: Ontario, BC, Alberta, etc. follow English common law
//!   principles modified by Canadian case law
//! - **Quebec**: Follows civil law tradition under the Civil Code of Quebec (CCQ)
//!
//! # Key Differences
//!
//! ## Formation
//!
//! | Element | Common Law | Quebec Civil Law |
//! |---------|------------|------------------|
//! | Consideration | Required | Not required (cause) |
//! | Consent | Objective test | Subjective consent (CCQ 1386) |
//! | Form | Generally informal | Some contracts require writing |
//!
//! ## Key Canadian Cases
//!
//! - **Bhasin v Hrynew \[2014\] SCC 71**: Duty of honest contractual performance
//! - **CM Callow v Zollinger \[2020\] SCC 45**: Expanded good faith duty
//! - **Tercon v BC \[2010\] SCC 4**: Three-step exclusion clause framework
//! - **Hunter Engineering v Syncrude \[1989\] SCC 21**: Fundamental breach
//!
//! # Usage
//!
//! ```rust,ignore
//! use legalis_ca::contract::{FormationAnalyzer, FormationFacts, BreachAnalyzer};
//! use legalis_ca::common::Province;
//!
//! // Analyze formation in Ontario (common law)
//! let facts = FormationFacts { province: Province::Ontario, /* ... */ };
//! let result = FormationAnalyzer::analyze(&facts);
//!
//! // Quebec civil law - no consideration required
//! let qc_facts = FormationFacts { province: Province::Quebec, /* ... */ };
//! let qc_result = FormationAnalyzer::analyze(&qc_facts);
//! assert!(qc_result.quebec_law_applies);
//! ```

#![allow(missing_docs)]

pub mod breach;
pub mod error;
pub mod formation;
pub mod types;

// Re-export types
pub use types::{
    // Formation
    Acceptance,
    // Breach
    BreachType,
    CcqConcept,
    CcqContractType,
    CommunicationMethod,
    Consideration,
    // Cases
    ContractArea,
    ContractCase,
    ContractRemedy,
    // Terms
    ContractTerm,
    DamagesCalculation,
    // Vitiating factors
    DuressType,
    ExclusionClause,
    FormationElement,
    MisrepresentationType,
    MistakeType,
    Offer,
    TermClassification,
    TermType,
    VitiatingFactor,
};

// Re-export formation
pub use formation::{
    CapacityIssue, CapacityStatus, ContractContext, FormationAnalyzer, FormationFacts,
    FormationResult, IntentionEvidence, LegalityStatus, OfferAnalyzer, OfferClassificationFacts,
    OfferClassificationResult, OfferContext,
};

// Re-export breach
pub use breach::{
    BreachAnalyzer, BreachFacts, BreachResult, DamagesAnalyzer, DamagesFacts, DamagesResult,
};

// Re-export error
pub use error::{ContractError, ContractResult};

// ============================================================================
// Legalis Core Integration
// ============================================================================

use legalis_core::{Effect, EffectType, Statute};

/// Create Sale of Goods Act implied warranty statute (s.15)
pub fn create_sale_of_goods_act(province: &crate::common::Province) -> Statute {
    use crate::common::Province;

    let (id, title) = match province {
        Province::Ontario => (
            "ON_SGA_s15",
            "Sale of Goods Act, RSO 1990, c S.1 - Implied warranties",
        ),
        Province::BritishColumbia => (
            "BC_SGA_s18",
            "Sale of Goods Act, RSBC 1996, c 410 - Implied warranties",
        ),
        Province::Alberta => (
            "AB_SGA_s15",
            "Sale of Goods Act, RSA 2000, c S-2 - Implied warranties",
        ),
        Province::Manitoba => (
            "MB_SGA_s15",
            "Sale of Goods Act, CCSM c S10 - Implied warranties",
        ),
        Province::Saskatchewan => (
            "SK_SGA_s15",
            "Sale of Goods Act, RSS 1978, c S-1 - Implied warranties",
        ),
        Province::NovaScotia => (
            "NS_SGA_s15",
            "Sale of Goods Act, RSNS 1989, c 408 - Implied warranties",
        ),
        Province::NewBrunswick => (
            "NB_SGA_s15",
            "Sale of Goods Act, RSNB 2016, c 110 - Implied warranties",
        ),
        Province::PrinceEdwardIsland => (
            "PEI_SGA_s15",
            "Sale of Goods Act, RSPEI 1988, c S-1 - Implied warranties",
        ),
        Province::NewfoundlandLabrador => (
            "NL_SGA_s15",
            "Sale of Goods Act, RSNL 1990, c S-6 - Implied warranties",
        ),
        Province::NorthwestTerritories => (
            "NWT_SGA_s15",
            "Sale of Goods Act, RSNWT 1988, c S-2 - Implied warranties",
        ),
        Province::Yukon => (
            "YK_SGA_s15",
            "Sale of Goods Act, RSY 2002, c 198 - Implied warranties",
        ),
        Province::Nunavut => (
            "NU_SGA_s15",
            "Sale of Goods Act, RSNWT (Nu) 1988, c S-2 - Implied warranties",
        ),
        Province::Quebec => {
            // Quebec uses Civil Code, not Sale of Goods Act
            return create_ccq_obligations();
        }
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Goods sold in course of business carry implied conditions of merchantable quality and fitness for purpose",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Civil Code of Quebec (Book Five: Obligations) good faith statute (art.1375)
pub fn create_ccq_obligations() -> Statute {
    Statute::new(
        "CCQ_art1375",
        "Civil Code of Quebec - art.1375 - Good Faith",
        Effect::new(
            EffectType::Obligation,
            "Parties must conduct themselves in good faith at formation, performance, and termination of contract",
        ),
    )
    .with_jurisdiction("QC")
}

/// Create CCQ consent formation statute (art.1385-1386)
pub fn create_ccq_consent() -> Statute {
    Statute::new(
        "CCQ_art1385",
        "Civil Code of Quebec - art.1385 - Contract Formation",
        Effect::new(
            EffectType::Grant,
            "Contract formed by exchange of consents between persons having capacity to contract",
        ),
    )
    .with_jurisdiction("QC")
}

/// Create CCQ warranty of quality statute (art.1726)
pub fn create_ccq_warranty_quality() -> Statute {
    Statute::new(
        "CCQ_art1726",
        "Civil Code of Quebec - art.1726 - Warranty of Quality",
        Effect::new(
            EffectType::Obligation,
            "Seller warrants that property is free of latent defects which render it unfit for use or diminish its usefulness",
        ),
    )
    .with_jurisdiction("QC")
}

/// Create Consumer Protection Act unfair practices statute
pub fn create_consumer_protection_act(province: &crate::common::Province) -> Statute {
    use crate::common::Province;

    let (id, title) = match province {
        Province::Ontario => (
            "ON_CPA_unfair",
            "Consumer Protection Act, 2002, SO 2002, c 30 - Unfair Practices",
        ),
        Province::BritishColumbia => (
            "BC_BPCPA_unfair",
            "Business Practices and Consumer Protection Act, SBC 2004, c 2 - Unfair Practices",
        ),
        Province::Alberta => (
            "AB_CPA_unfair",
            "Consumer Protection Act, RSA 2000, c C-26.3 - Unfair Practices",
        ),
        Province::Quebec => (
            "QC_CPA_unfair",
            "Consumer Protection Act, CQLR c P-40.1 - Unfair Practices",
        ),
        Province::Manitoba => (
            "MB_CPA_unfair",
            "Consumer Protection Act, CCSM c C200 - Unfair Practices",
        ),
        Province::Saskatchewan => (
            "SK_CPBPA_unfair",
            "Consumer Protection and Business Practices Act, SS 2014, c C-30.2 - Unfair Practices",
        ),
        Province::NovaScotia => (
            "NS_CPA_unfair",
            "Consumer Protection Act, RSNS 1989, c 92 - Unfair Practices",
        ),
        Province::NewBrunswick => (
            "NB_CPWLA_unfair",
            "Consumer Product Warranty and Liability Act, SNB 1978, c C-18.1 - Unfair Practices",
        ),
        Province::PrinceEdwardIsland => (
            "PEI_CPA_unfair",
            "Consumer Protection Act, RSPEI 1988, c C-19 - Unfair Practices",
        ),
        Province::NewfoundlandLabrador => (
            "NL_CPBPA_unfair",
            "Consumer Protection and Business Practices Act, SNL 2009, c C-31.1 - Unfair Practices",
        ),
        Province::NorthwestTerritories => (
            "NWT_CPA_unfair",
            "Consumer Protection Act, RSNWT 1988, c C-17 - Unfair Practices",
        ),
        Province::Yukon => (
            "YK_CPA_unfair",
            "Consumer Protection Act, RSY 2002, c 40 - Unfair Practices",
        ),
        Province::Nunavut => (
            "NU_CPA_unfair",
            "Consumer Protection Act, RSNWT (Nu) 1988, c C-17 - Unfair Practices",
        ),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Prohibition,
            "Unfair practices in consumer transactions are prohibited",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create all contract-related statutes for a province
pub fn create_contract_statutes(province: &crate::common::Province) -> Vec<Statute> {
    let mut statutes = vec![
        create_sale_of_goods_act(province),
        create_consumer_protection_act(province),
    ];

    // Quebec has additional civil code provisions
    if province.is_civil_law() {
        statutes.push(create_ccq_consent());
        statutes.push(create_ccq_warranty_quality());
    }

    statutes
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Province;

    #[test]
    fn test_sale_of_goods_ontario() {
        let statute = create_sale_of_goods_act(&Province::Ontario);
        assert!(statute.id.contains("ON"));
        assert!(statute.title.contains("Sale of Goods"));
    }

    #[test]
    fn test_sale_of_goods_bc() {
        let statute = create_sale_of_goods_act(&Province::BritishColumbia);
        assert!(statute.id.contains("BC"));
    }

    #[test]
    fn test_quebec_uses_ccq() {
        let statute = create_sale_of_goods_act(&Province::Quebec);
        assert!(statute.title.contains("Civil Code"));
    }

    #[test]
    fn test_ccq_obligations() {
        let statute = create_ccq_obligations();
        assert!(statute.id.contains("CCQ"));
        assert!(statute.title.contains("Good Faith"));
    }

    #[test]
    fn test_consumer_protection_ontario() {
        let statute = create_consumer_protection_act(&Province::Ontario);
        assert!(statute.id.contains("CPA"));
        assert!(statute.title.contains("Unfair"));
    }

    #[test]
    fn test_consumer_protection_quebec() {
        let statute = create_consumer_protection_act(&Province::Quebec);
        assert!(statute.id.contains("QC"));
    }

    #[test]
    fn test_contract_statutes_ontario() {
        let statutes = create_contract_statutes(&Province::Ontario);
        assert_eq!(statutes.len(), 2); // SGA + CPA
    }

    #[test]
    fn test_contract_statutes_quebec() {
        let statutes = create_contract_statutes(&Province::Quebec);
        assert_eq!(statutes.len(), 4); // CCQ + CPA + Consent + Warranty
    }
}
