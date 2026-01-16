//! Australian Tort Law Types
//!
//! Core types for Australian tort law including negligence,
//! Civil Liability Act reforms, and defamation.

use serde::{Deserialize, Serialize};

use crate::common::{AustralianCase, Court, LegalArea, StateTerritory};

// ============================================================================
// Duty of Care
// ============================================================================

/// Duty of care category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DutyCategory {
    /// Recognized category (established duty)
    Recognized(RecognizedDuty),
    /// Novel duty (requires salient features analysis)
    Novel,
}

/// Recognized duty categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecognizedDuty {
    /// Manufacturer to ultimate consumer (Donoghue v Stevenson)
    ManufacturerConsumer,
    /// Road user to road user
    RoadUser,
    /// Employer to employee
    EmployerEmployee,
    /// Occupier to visitor (under OL statutes)
    OccupierVisitor,
    /// Doctor/professional to patient/client
    Professional,
    /// Teacher/school to student
    TeacherStudent,
    /// Parent to child
    ParentChild,
    /// Bailor to bailee
    BailorBailee,
    /// Solicitor to client
    SolicitorClient,
    /// Financial advisor to client
    FinancialAdvisor,
}

/// Salient features for novel duty (Sullivan v Moody)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SalientFeature {
    /// Foreseeability of harm
    Foreseeability,
    /// Nature of harm
    NatureOfHarm,
    /// Vulnerability of plaintiff
    Vulnerability,
    /// Control over risk
    ControlOverRisk,
    /// Assumption of responsibility
    AssumptionOfResponsibility,
    /// Knowledge of likelihood of harm
    KnowledgeOfLikelihood,
    /// Proximity of parties
    Proximity,
    /// Indeterminacy concerns
    IndeterminacyConcerns,
    /// Coherence with existing law
    CoherenceWithLaw,
    /// Conflicting duties
    ConflictingDuties,
    /// Nature of defendant's activity
    NatureOfActivity,
}

// ============================================================================
// Breach of Duty
// ============================================================================

/// Standard of care factors (Civil Liability Acts)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardOfCareFactor {
    /// Probability of harm (CLA s.9(a))
    ProbabilityOfHarm,
    /// Likely seriousness of harm (CLA s.9(b))
    SeriousnessOfHarm,
    /// Burden of precautions (CLA s.9(c))
    BurdenOfPrecautions,
    /// Social utility of activity (CLA s.9(d))
    SocialUtility,
}

/// Obvious risk (CLA provisions)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObviousRisk {
    /// Whether risk was obvious
    pub obvious: bool,
    /// Whether risk was inherent
    pub inherent: bool,
    /// Whether recreational activity
    pub recreational_activity: bool,
    /// Description of risk
    pub description: String,
}

// ============================================================================
// Causation
// ============================================================================

/// Causation test
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausationTest {
    /// Factual causation (CLA s.5D(1)(a))
    FactualCausation,
    /// Scope of liability (CLA s.5D(1)(b))
    ScopeOfLiability,
}

/// Novus actus interveniens type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NovusActus {
    /// Third party act
    ThirdPartyAct,
    /// Claimant's own act
    ClaimantAct,
    /// Natural event
    NaturalEvent,
}

// ============================================================================
// Damages
// ============================================================================

/// Type of damages in tort
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortDamagesType {
    /// Economic loss
    EconomicLoss,
    /// Non-economic loss (pain and suffering)
    NonEconomicLoss,
    /// Medical expenses
    MedicalExpenses,
    /// Loss of earning capacity
    LossOfEarningCapacity,
    /// Gratuitous care
    GratuitousCare,
    /// Future care needs
    FutureCare,
    /// Property damage
    PropertyDamage,
}

/// Damages caps (Civil Liability Acts)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesCaps {
    /// Non-economic loss cap
    pub non_economic_loss_cap: Option<f64>,
    /// Loss of earning capacity cap
    pub earning_capacity_cap: Option<f64>,
    /// Gratuitous care threshold
    pub gratuitous_care_threshold_hours: Option<u32>,
    /// State applying
    pub state: StateTerritory,
}

impl DamagesCaps {
    /// Get default caps for a state
    pub fn for_state(state: StateTerritory) -> Self {
        // Caps vary by state - these are indicative
        match state {
            StateTerritory::NewSouthWales => Self {
                non_economic_loss_cap: Some(687_000.0),           // Indexed
                earning_capacity_cap: Some(4_377.0 * 52.0 * 3.0), // ~3x average earnings
                gratuitous_care_threshold_hours: Some(6),
                state,
            },
            StateTerritory::Victoria => Self {
                non_economic_loss_cap: Some(650_000.0),
                earning_capacity_cap: None, // Different approach
                gratuitous_care_threshold_hours: Some(6),
                state,
            },
            _ => Self {
                non_economic_loss_cap: Some(600_000.0),
                earning_capacity_cap: None,
                gratuitous_care_threshold_hours: Some(6),
                state,
            },
        }
    }
}

// ============================================================================
// Civil Liability Act Reforms
// ============================================================================

/// Civil Liability Act defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CLADefence {
    /// Obvious risk - no duty to warn
    ObviousRiskNoWarning,
    /// Inherent risk
    InherentRisk,
    /// Voluntary assumption of risk (recreational)
    VoluntaryAssumptionRecreational,
    /// Dangerous recreational activity
    DangerousRecreationalActivity,
    /// Good Samaritan protection
    GoodSamaritan,
    /// Volunteer protection
    VolunteerProtection,
    /// Intoxication of plaintiff
    PlaintiffIntoxication,
    /// Illegal activity of plaintiff
    PlaintiffIllegalActivity,
    /// Mental harm - normal fortitude rule
    MentalHarmNormalFortitude,
}

/// Mental harm requirements (CLA provisions)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MentalHarmCategory {
    /// Consequential mental harm (with physical injury)
    Consequential,
    /// Pure mental harm
    Pure,
}

// ============================================================================
// Defamation
// ============================================================================

/// Defamation element
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationElement {
    /// Publication
    Publication,
    /// Identification of plaintiff
    Identification,
    /// Defamatory meaning
    DefamatoryMeaning,
}

/// Defamation defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationDefence {
    /// Justification (truth)
    Justification,
    /// Contextual truth
    ContextualTruth,
    /// Absolute privilege
    AbsolutePrivilege,
    /// Qualified privilege (common law)
    QualifiedPrivilegeCommonLaw,
    /// Qualified privilege (statutory)
    QualifiedPrivilegeStatutory,
    /// Honest opinion
    HonestOpinion,
    /// Innocent dissemination
    InnocentDissemination,
    /// Triviality
    Triviality,
    /// Public interest
    PublicInterest,
    /// Scientific/academic peer review
    ScientificPeerReview,
}

/// Uniform Defamation Law imputations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImputationType {
    /// Criminal conduct
    CriminalConduct,
    /// Incompetence
    Incompetence,
    /// Dishonesty
    Dishonesty,
    /// Immoral conduct
    ImmoralConduct,
    /// Financial irresponsibility
    FinancialIrresponsibility,
    /// Other defamatory imputation
    Other(String),
}

// ============================================================================
// Nuisance
// ============================================================================

/// Type of nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceType {
    /// Private nuisance
    Private,
    /// Public nuisance
    Public,
}

/// Private nuisance interference type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceInterference {
    /// Physical damage to land
    PhysicalDamage,
    /// Interference with enjoyment
    UseAndEnjoyment,
}

/// Nuisance defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceDefence {
    /// Statutory authority
    StatutoryAuthority,
    /// Prescription (20 years)
    Prescription,
    /// Consent
    Consent,
    /// Act of third party
    ThirdPartyAct,
    /// Act of God
    ActOfGod,
}

// ============================================================================
// Key Cases
// ============================================================================

impl AustralianCase {
    /// Sullivan v Moody - Novel duty analysis
    pub fn sullivan_v_moody_full() -> Self {
        Self::new(
            "Sullivan v Moody",
            2001,
            "(2001) 207 CLR 562",
            Court::HighCourt,
            LegalArea::Tort,
            "For novel duties, court examines salient features including coherence with existing principles",
        )
    }

    /// Rogers v Whitaker - Medical disclosure
    pub fn rogers_v_whitaker_full() -> Self {
        Self::new(
            "Rogers v Whitaker",
            1992,
            "(1992) 175 CLR 479",
            Court::HighCourt,
            LegalArea::Tort,
            "Duty to disclose material risks; rejected Bolam test for disclosure",
        )
    }

    /// Wyong Shire Council v Shirt - Breach
    pub fn wyong_v_shirt() -> Self {
        Self::new(
            "Wyong Shire Council v Shirt",
            1980,
            "(1980) 146 CLR 40",
            Court::HighCourt,
            LegalArea::Tort,
            "Breach: reasonable response to foreseeable risk, not far-fetched or fanciful",
        )
    }

    /// March v Stramare - Causation
    pub fn march_v_stramare() -> Self {
        Self::new(
            "March v E & MH Stramare Pty Ltd",
            1991,
            "(1991) 171 CLR 506",
            Court::HighCourt,
            LegalArea::Tort,
            "But-for test not exclusive; common sense approach to causation",
        )
    }

    /// Tame v NSW - Mental harm
    pub fn tame_v_nsw() -> Self {
        Self::new(
            "Tame v New South Wales",
            2002,
            "(2002) 211 CLR 317",
            Court::HighCourt,
            LegalArea::Tort,
            "Pure mental harm requires recognized psychiatric illness, reasonable foreseeability",
        )
    }

    /// Ipp Report - Tort reform
    pub fn ipp_report() -> Self {
        Self::new(
            "Review of the Law of Negligence Report (Ipp Report)",
            2002,
            "Commonwealth of Australia",
            Court::HighCourt, // Not a case but influential
            LegalArea::Tort,
            "Foundation for Civil Liability Act reforms across all states",
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damages_caps() {
        let caps = DamagesCaps::for_state(StateTerritory::NewSouthWales);
        assert!(caps.non_economic_loss_cap.is_some());
        assert_eq!(caps.gratuitous_care_threshold_hours, Some(6));
    }

    #[test]
    fn test_sullivan_v_moody() {
        let case = AustralianCase::sullivan_v_moody_full();
        assert_eq!(case.year, 2001);
        assert!(case.principle.contains("salient features"));
    }

    #[test]
    fn test_recognized_duties() {
        let duty = DutyCategory::Recognized(RecognizedDuty::ManufacturerConsumer);
        assert!(matches!(duty, DutyCategory::Recognized(_)));
    }
}
