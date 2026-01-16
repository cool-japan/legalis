//! Canada Tort Law
//!
//! Tort law analysis for Canadian common law provinces and Quebec civil law.
//!
//! # Overview
//!
//! Canadian tort law follows common law principles derived from England,
//! with significant development through Canadian jurisprudence:
//!
//! - **Duty of Care**: Anns/Cooper two-stage test (Cooper v Hobart [2001])
//! - **Causation**: But-for test with material contribution exception (Clements [2012])
//! - **Damages**: Non-pecuniary cap (trilogy cases, indexed from 1978)
//!
//! ## Quebec Civil Law
//!
//! Quebec uses civil liability under art. 1457-1481 CCQ, based on fault,
//! damage, and causal link, rather than common law categories.
//!
//! # Key Canadian Cases
//!
//! - **Donoghue v Stevenson** [1932] (neighbour principle - applied in Canada)
//! - **Cooper v Hobart** [2001] SCC 79 (refined Anns test)
//! - **Childs v Desormeaux** [2006] SCC 18 (social host liability)
//! - **Clements v Clements** [2012] SCC 32 (material contribution)
//! - **Mustapha v Culligan** [2008] SCC 27 (psychological harm)
//! - **Andrews v Grand & Toy** [1978] SCC (non-pecuniary cap)
//! - **Grant v Torstar** [2009] SCC 61 (responsible communication)
//!
//! # Usage
//!
//! ```rust,ignore
//! use legalis_ca::tort::{
//!     DutyOfCareAnalyzer, DutyOfCareFacts, NegligenceAnalyzer,
//!     DefamationAnalyzer, OccupiersLiabilityAnalyzer,
//! };
//!
//! // Duty of care analysis
//! let duty_facts = DutyOfCareFacts { /* ... */ };
//! let result = DutyOfCareAnalyzer::analyze(&duty_facts);
//!
//! // Full negligence analysis
//! let negligence_facts = NegligenceFacts { /* ... */ };
//! let result = NegligenceAnalyzer::analyze(&negligence_facts);
//! ```

#![allow(missing_docs)]

pub mod defamation;
pub mod error;
pub mod negligence;
pub mod occupiers;
pub mod types;

// Re-export types
pub use types::{
    // Standard of care
    BreachFactor,
    // Causation
    CausationTest,
    // Occupiers' liability
    CommonLawEntrantStatus,
    // Defamation
    DefamationDefence,
    DefamationType,
    // Duty of care
    DutyOfCareStage,
    HazardType,
    InterveningCause,
    // Defences
    NegligenceDefence,
    // Damages
    NonPecuniaryCap,
    // Nuisance
    NuisanceFactor,
    NuisanceType,
    OlaDuty,
    OlaStatute,
    PolicyNegation,
    ProximityFactor,
    RecognizedDutyCategory,
    RemotenessTest,
    ResponsibleCommunicationFactors,
    StandardOfCare,
    // Cases
    TortArea,
    TortCase,
    TortDamages,
};

// Re-export negligence
pub use negligence::{
    CausationAnalyzer, CausationFacts, CausationResult, DamagesFacts as NegligenceDamagesFacts,
    DamagesResult as NegligenceDamagesResult, DutyOfCareAnalyzer, DutyOfCareFacts,
    DutyOfCareResult, NegligenceAnalyzer, NegligenceFacts, NegligenceResult, RemotenessAnalyzer,
    RemotenessFacts, RemotenessResult, StandardOfCareAnalyzer, StandardOfCareFacts,
    StandardOfCareResult, TortDamagesAnalyzer,
};

// Re-export occupiers' liability
pub use occupiers::{
    ApplicableLaw, EntrantStatus, EntryPurpose, HazardDescription, OccupierStatus,
    OccupiersLiabilityAnalyzer, OccupiersLiabilityFacts, OccupiersLiabilityResult, OlaDefence,
};

// Re-export defamation
pub use defamation::{
    DamagesAssessment, DamagesType, DefamationAnalyzer, DefamationDefenceClaim, DefamationElements,
    DefamationFacts, DefamationResult, DefenceAnalysis, PublicationMedium, PublicationReach,
    StatementContext,
};

// Re-export error
pub use error::{TortError, TortResult};

// ============================================================================
// Legalis Core Integration
// ============================================================================

use legalis_core::{Effect, EffectType, Statute};

/// Create Occupiers' Liability Act statute for a province
pub fn create_ola_statute(province: &crate::common::Province) -> Option<Statute> {
    use crate::common::Province;

    let (id, title) = match province {
        Province::Ontario => ("ON_OLA", "Occupiers' Liability Act, RSO 1990, c O.2"),
        Province::BritishColumbia => ("BC_OLA", "Occupiers Liability Act, RSBC 1996, c 337"),
        Province::Alberta => ("AB_OLA", "Occupiers' Liability Act, RSA 2000, c O-4"),
        Province::Manitoba => ("MB_OLA", "Occupiers' Liability Act, CCSM c O8"),
        _ => return None, // Other provinces use common law
    };

    Some(
        Statute::new(
            id,
            title,
            Effect::new(
                EffectType::Obligation,
                "Occupier owes common duty of care to all visitors",
            ),
        )
        .with_jurisdiction(province.abbreviation()),
    )
}

/// Create Quebec civil liability statute (art.1457 CCQ)
pub fn create_ccq_civil_liability() -> Statute {
    Statute::new(
        "CCQ_art1457",
        "Civil Code of Quebec - art.1457 - Civil Liability",
        Effect::new(
            EffectType::Obligation,
            "Every person has duty to abide by rules of conduct imposed by circumstances, \
             usage or law so as not to cause injury to another",
        ),
    )
    .with_jurisdiction("QC")
}

/// Create general negligence statute framework
pub fn create_negligence_statute(jurisdiction: &str) -> Statute {
    Statute::new(
        format!("{}_NEGLIGENCE", jurisdiction),
        "Common Law Negligence Framework",
        Effect::new(
            EffectType::Obligation,
            "Duty to take reasonable care to avoid acts or omissions that could \
             reasonably foreseeably harm another (Donoghue v Stevenson)",
        ),
    )
    .with_jurisdiction(jurisdiction)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Province;

    #[test]
    fn test_ola_ontario() {
        let statute = create_ola_statute(&Province::Ontario);
        assert!(statute.is_some());
        assert!(statute.as_ref().is_some_and(|s| s.id.contains("ON")));
    }

    #[test]
    fn test_ola_bc() {
        let statute = create_ola_statute(&Province::BritishColumbia);
        assert!(statute.is_some());
    }

    #[test]
    fn test_ola_quebec_none() {
        let statute = create_ola_statute(&Province::Quebec);
        assert!(statute.is_none()); // Quebec uses civil code
    }

    #[test]
    fn test_ccq_civil_liability() {
        let statute = create_ccq_civil_liability();
        assert!(statute.id.contains("CCQ"));
        assert!(statute.title.contains("1457"));
    }

    #[test]
    fn test_negligence_statute() {
        let statute = create_negligence_statute("ON");
        assert!(statute.title.contains("Negligence"));
    }
}
