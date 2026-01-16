//! Australian Contract Law Types
//!
//! Core types for Australian contract law including formation,
//! terms, breach, and remedies.

use serde::{Deserialize, Serialize};

use crate::common::{AustralianCase, Court, LegalArea, StateTerritory};

// ============================================================================
// Contract Formation
// ============================================================================

/// Contract formation element
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormationElement {
    /// Agreement (offer and acceptance)
    Agreement,
    /// Consideration (or deed)
    Consideration,
    /// Intention to create legal relations
    IntentionToCreateLegalRelations,
    /// Capacity to contract
    Capacity,
    /// Certainty of terms
    Certainty,
    /// Legality of purpose
    Legality,
}

/// Type of offer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferType {
    /// Offer to specific person
    Specific,
    /// Offer to class of persons
    ToClass,
    /// Offer to the world (Carlill v Carbolic Smoke Ball)
    ToTheWorld,
    /// Counter-offer
    CounterOffer,
    /// Standing offer
    Standing,
}

/// Mode of acceptance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcceptanceMode {
    /// Express acceptance
    Express,
    /// Acceptance by conduct
    Conduct,
    /// Silence (generally not acceptance)
    Silence,
    /// Postal acceptance
    Postal,
    /// Instantaneous communication
    Instantaneous,
}

/// Consideration requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsiderationType {
    /// Executory consideration (promise for promise)
    Executory,
    /// Executed consideration (act for promise)
    Executed,
    /// Past consideration (generally not valid)
    Past,
    /// Deed (no consideration required)
    Deed,
}

// ============================================================================
// Contract Terms
// ============================================================================

/// Type of contractual term
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermType {
    /// Express term
    Express,
    /// Term implied in fact
    ImpliedInFact,
    /// Term implied by statute
    ImpliedByStatute,
    /// Term implied by custom
    ImpliedByCustom,
    /// Term implied by common law
    ImpliedByCommonLaw,
}

/// Classification of term by importance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermClassification {
    /// Condition - essential term
    Condition,
    /// Warranty - non-essential term
    Warranty,
    /// Intermediate/innominate term (Hong Kong Fir)
    Intermediate,
}

/// Exclusion clause analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExclusionClauseStatus {
    /// Incorporated into contract
    Incorporated,
    /// Not incorporated (insufficient notice)
    NotIncorporated,
    /// Incorporated but not covering breach
    NotCovering,
    /// Void under ACL
    VoidUnderACL,
    /// Valid and effective
    Valid,
}

// ============================================================================
// Australian Consumer Law
// ============================================================================

/// Consumer guarantee type under ACL
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumerGuarantee {
    /// s.54 - Acceptable quality
    AcceptableQuality,
    /// s.55 - Fit for disclosed purpose
    FitForPurpose,
    /// s.56 - Matches description
    MatchesDescription,
    /// s.57 - Matches sample/demo model
    MatchesSample,
    /// s.58 - Repairs and spare parts available
    RepairsAndParts,
    /// s.59 - Express warranties honored
    ExpressWarranties,
    /// s.60 - Due care and skill (services)
    DueCareAndSkill,
    /// s.61 - Fit for purpose (services)
    ServicesFitForPurpose,
    /// s.62 - Reasonable time (services)
    ReasonableTime,
    /// s.63 - Title guarantee
    Title,
    /// s.64 - Undisturbed possession
    UndisturbedPossession,
    /// s.65 - Free from encumbrances
    FreeFromEncumbrances,
}

impl ConsumerGuarantee {
    /// Get the ACL section reference
    pub fn section(&self) -> u32 {
        match self {
            Self::AcceptableQuality => 54,
            Self::FitForPurpose => 55,
            Self::MatchesDescription => 56,
            Self::MatchesSample => 57,
            Self::RepairsAndParts => 58,
            Self::ExpressWarranties => 59,
            Self::DueCareAndSkill => 60,
            Self::ServicesFitForPurpose => 61,
            Self::ReasonableTime => 62,
            Self::Title => 63,
            Self::UndisturbedPossession => 64,
            Self::FreeFromEncumbrances => 65,
        }
    }
}

/// Unfair contract term type (ACL)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnfairTermType {
    /// Permits one party but not other to avoid/limit obligations
    AsymmetricObligations,
    /// Permits one party but not other to terminate
    AsymmetricTermination,
    /// Penalizes one party but not other for breach
    AsymmetricPenalties,
    /// Permits one party to vary terms without consent
    UnilateralVariation,
    /// Permits one party to vary characteristics unilaterally
    UnilateralCharacterChange,
    /// Allows one party to determine price unilaterally
    UnilateralPriceDetermination,
    /// Limits consumer's right to sue
    LimitsLegalAction,
    /// Limits evidence consumer can use
    LimitsEvidence,
    /// Imposes evidential burden on consumer
    EvidentialBurden,
    /// Other unfair term
    Other(String),
}

/// ACL prohibited conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProhibitedConduct {
    /// s.18 - Misleading or deceptive conduct
    MisleadingDeceptive,
    /// s.29 - False or misleading representations
    FalseRepresentations,
    /// s.30 - False representations about land
    FalseRepresentationsLand,
    /// s.31 - Misleading conduct about employment
    MisleadingEmployment,
    /// s.32 - Offering rebates etc without intent
    FalseRebates,
    /// s.33 - Misleading conduct about business
    MisleadingBusiness,
    /// s.34 - Bait advertising
    BaitAdvertising,
    /// s.35 - Accepting payment without ability to supply
    AcceptPaymentNoSupply,
    /// s.50 - Unconscionable conduct (general)
    UnconscionableGeneral,
    /// s.51 - Unconscionable conduct in trade/commerce
    UnconscionableTradeCommerce,
}

impl ProhibitedConduct {
    /// Get the ACL section
    pub fn section(&self) -> u32 {
        match self {
            Self::MisleadingDeceptive => 18,
            Self::FalseRepresentations => 29,
            Self::FalseRepresentationsLand => 30,
            Self::MisleadingEmployment => 31,
            Self::FalseRebates => 32,
            Self::MisleadingBusiness => 33,
            Self::BaitAdvertising => 34,
            Self::AcceptPaymentNoSupply => 35,
            Self::UnconscionableGeneral => 50,
            Self::UnconscionableTradeCommerce => 51,
        }
    }
}

// ============================================================================
// Breach
// ============================================================================

/// Type of breach
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Breach of condition (repudiatory)
    Condition,
    /// Breach of warranty
    Warranty,
    /// Breach of intermediate term (depends on consequence)
    Intermediate,
    /// Anticipatory breach
    Anticipatory,
    /// Fundamental breach
    Fundamental,
    /// Renunciation
    Renunciation,
}

/// Remedy for breach
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractRemedy {
    /// Damages
    Damages,
    /// Specific performance
    SpecificPerformance,
    /// Injunction
    Injunction,
    /// Rescission
    Rescission,
    /// Rectification
    Rectification,
    /// Termination (for repudiatory breach)
    Termination,
    /// Quantum meruit
    QuantumMeruit,
    /// Account of profits
    AccountOfProfits,
}

/// Type of damages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesType {
    /// Expectation damages (put in position as if contract performed)
    Expectation,
    /// Reliance damages (put in pre-contract position)
    Reliance,
    /// Restitution
    Restitution,
    /// Nominal damages
    Nominal,
    /// Aggravated damages
    Aggravated,
    /// Consequential damages
    Consequential,
}

// ============================================================================
// Vitiating Factors
// ============================================================================

/// Vitiating factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitiatingFactor {
    /// Misrepresentation
    Misrepresentation(MisrepresentationType),
    /// Mistake
    Mistake(MistakeType),
    /// Duress
    Duress(DuressType),
    /// Undue influence
    UndueInfluence(UndueInfluenceType),
    /// Unconscionability
    Unconscionability,
    /// Illegality
    Illegality,
}

/// Type of misrepresentation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MisrepresentationType {
    /// Fraudulent misrepresentation
    Fraudulent,
    /// Negligent misrepresentation
    Negligent,
    /// Innocent misrepresentation
    Innocent,
}

/// Type of mistake
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistakeType {
    /// Common mistake (both parties same mistake)
    Common,
    /// Mutual mistake (parties at cross-purposes)
    Mutual,
    /// Unilateral mistake
    Unilateral,
    /// Mistake as to identity
    Identity,
    /// Non est factum
    NonEstFactum,
}

/// Type of duress
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuressType {
    /// Duress to the person
    Person,
    /// Duress to goods
    Goods,
    /// Economic duress
    Economic,
}

/// Type of undue influence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndueInfluenceType {
    /// Actual undue influence
    Actual,
    /// Presumed undue influence (Class 2A - automatic)
    PresumedAutomatic,
    /// Presumed undue influence (Class 2B - proven relationship)
    PresumedProven,
}

// ============================================================================
// Key Cases
// ============================================================================

impl AustralianCase {
    /// Waltons Stores v Maher - Estoppel
    pub fn waltons_stores() -> Self {
        Self::new(
            "Waltons Stores (Interstate) Ltd v Maher",
            1988,
            "(1988) 164 CLR 387",
            Court::HighCourt,
            LegalArea::Contract,
            "Proprietary estoppel can found cause of action in Australia; unconscionability key",
        )
    }

    /// Ermogenous v Greek Orthodox - Intention
    pub fn ermogenous() -> Self {
        Self::new(
            "Ermogenous v Greek Orthodox Community of SA Inc",
            2002,
            "(2002) 209 CLR 95",
            Court::HighCourt,
            LegalArea::Contract,
            "Intention to create legal relations is objective question",
        )
    }

    /// Hungry Jack's v Burger King - Good faith
    pub fn hungry_jacks() -> Self {
        Self::new(
            "Burger King Corporation v Hungry Jack's Pty Ltd",
            2001,
            "[2001] NSWCA 187",
            Court::StateSupremeCourt(StateTerritory::NewSouthWales),
            LegalArea::Contract,
            "Duty of good faith implied in certain commercial contracts",
        )
    }

    /// ACCC v CG Berbatis - Unconscionability
    pub fn berbatis() -> Self {
        Self::new(
            "ACCC v CG Berbatis Holdings Pty Ltd",
            2003,
            "(2003) 214 CLR 51",
            Court::HighCourt,
            LegalArea::Contract,
            "Unconscionability requires exploitation of special disadvantage",
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
    fn test_consumer_guarantee_sections() {
        assert_eq!(ConsumerGuarantee::AcceptableQuality.section(), 54);
        assert_eq!(ConsumerGuarantee::DueCareAndSkill.section(), 60);
    }

    #[test]
    fn test_prohibited_conduct_sections() {
        assert_eq!(ProhibitedConduct::MisleadingDeceptive.section(), 18);
        assert_eq!(ProhibitedConduct::UnconscionableTradeCommerce.section(), 51);
    }

    #[test]
    fn test_waltons_stores_case() {
        let case = AustralianCase::waltons_stores();
        assert_eq!(case.year, 1988);
        assert!(case.principle.contains("estoppel"));
    }
}
