//! UK Public Law - Core Types
//!
//! This module provides comprehensive type definitions for UK public law,
//! covering judicial review, human rights, and constitutional principles.
//!
//! # Legal Framework
//!
//! UK public law governs the relationship between the state and individuals:
//! - Judicial review of administrative action
//! - Human Rights Act 1998 (incorporating ECHR)
//! - Constitutional principles (parliamentary sovereignty, rule of law)
//! - Royal prerogative powers

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

// ============================================================================
// Public Body Types
// ============================================================================

/// Type of public body for judicial review purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicBodyType {
    /// Central government department
    GovernmentDepartment { name: String },
    /// Local authority
    LocalAuthority { name: String },
    /// Non-departmental public body (NDPB/quango)
    Ndpb { name: String },
    /// Regulatory body
    Regulator { name: String },
    /// Tribunal
    Tribunal { name: String },
    /// NHS body
    NhsBody { name: String },
    /// Police force
    Police { force: String },
    /// Court (for limited review purposes)
    Court { name: String },
    /// Body exercising public functions (s.6 HRA)
    HybridBody {
        name: String,
        public_function: String,
    },
}

/// Nature of the decision being challenged
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionNature {
    /// Statutory decision (exercising powers under statute)
    Statutory { statute: String, section: String },
    /// Prerogative decision
    Prerogative { power: PrerogativePower },
    /// Policy decision
    Policy { policy_area: String },
    /// Individual determination
    IndividualDetermination,
    /// Rule-making/secondary legislation
    RuleMaking { instrument_type: String },
    /// Guidance or code of practice
    Guidance,
    /// Contractual decision with public element
    ContractualPublic,
}

/// Royal prerogative power
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrerogativePower {
    /// Foreign affairs (conduct of international relations)
    ForeignAffairs,
    /// Defence and armed forces
    DefenceAndArmedForces,
    /// Treaty-making
    TreatyMaking,
    /// Appointment and dismissal (ministers, civil servants)
    AppointmentAndDismissal,
    /// Mercy and pardon
    MercyAndPardon,
    /// Honours and titles
    HonoursAndTitles,
    /// National security
    NationalSecurity,
    /// Royal assent
    RoyalAssent,
    /// Summoning and proroguing Parliament
    SummoningParliament,
    /// Passport issuance
    PassportIssuance,
    /// Crime and justice (prosecution, AG powers)
    CrimeAndJustice,
    /// Other prerogative
    Other(String),
}

// ============================================================================
// Judicial Review Grounds
// ============================================================================

/// Ground of judicial review
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroundOfReview {
    /// Illegality - error of law
    Illegality(IllegalityType),
    /// Irrationality/unreasonableness
    Irrationality(IrrationalityType),
    /// Procedural impropriety
    ProceduralImpropriety(ProceduralType),
    /// Proportionality (HRA cases)
    Proportionality,
    /// Legitimate expectation
    LegitimateExpectation(ExpectationType),
    /// Human rights violation
    HumanRightsViolation { article: EchrArticle },
}

/// Types of illegality
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IllegalityType {
    /// Acting without jurisdiction/ultra vires
    UltraVires,
    /// Error of law on face of record
    ErrorOfLaw,
    /// Fettering discretion
    FetteringDiscretion,
    /// Improper delegation
    ImproperDelegation,
    /// Relevant considerations - failure to consider
    FailureToConsider { factor: String },
    /// Relevant considerations - considering irrelevant
    IrrelevantConsideration { factor: String },
    /// Improper purpose
    ImproperPurpose { purpose: String },
    /// Jurisdictional error
    JurisdictionalError,
}

/// Types of irrationality
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrrationalityType {
    /// Wednesbury unreasonableness
    Wednesbury,
    /// Sub-Wednesbury (anxious scrutiny in rights cases)
    AnxiousScrutiny,
    /// Proportionality (stricter than Wednesbury)
    Proportionality,
    /// Inconsistency with previous decisions
    Inconsistency,
}

/// Types of procedural impropriety
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProceduralType {
    /// Breach of natural justice - bias
    Bias(BiasType),
    /// Breach of natural justice - fair hearing
    FairHearing { breach: String },
    /// Breach of statutory procedure
    StatutoryProcedure { breach: String },
    /// Failure to give reasons
    FailureToGiveReasons,
    /// Failure to consult
    FailureToConsult,
}

/// Types of bias
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasType {
    /// Actual bias
    Actual,
    /// Apparent bias (fair-minded observer test)
    Apparent,
    /// Automatic disqualification (pecuniary interest, etc.)
    AutomaticDisqualification { reason: String },
}

/// Types of legitimate expectation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpectationType {
    /// Substantive expectation (rare)
    Substantive { benefit_expected: String },
    /// Procedural expectation
    Procedural { procedure_expected: String },
}

// ============================================================================
// Standing (Locus Standi)
// ============================================================================

/// Standing to bring judicial review
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandingType {
    /// Direct victim/affected person
    DirectVictim,
    /// Sufficient interest (s.31(3) Senior Courts Act 1981)
    SufficientInterest { basis: String },
    /// Public interest standing
    PublicInterest { organization: String },
    /// Representative standing
    Representative { representing: String },
    /// Third party intervener
    Intervener,
}

// ============================================================================
// Remedies
// ============================================================================

/// Judicial review remedy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JrRemedy {
    /// Quashing order (certiorari)
    QuashingOrder,
    /// Mandatory order (mandamus)
    MandatoryOrder { action_required: String },
    /// Prohibiting order (prohibition)
    ProhibitingOrder,
    /// Declaration
    Declaration { content: String },
    /// Injunction
    Injunction(InjunctionType),
    /// Damages (limited availability)
    Damages { basis: DamagesBasis },
}

/// Type of injunction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjunctionType {
    /// Interim injunction
    Interim,
    /// Final injunction
    Final,
    /// Mandatory injunction
    Mandatory { action: String },
    /// Prohibitory injunction
    Prohibitory { prohibition: String },
}

/// Basis for damages in judicial review
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesBasis {
    /// HRA s.8 damages (just satisfaction)
    HraDamages,
    /// EU law damages (Francovich - retained)
    EuLawDamages,
    /// Misfeasance in public office
    Misfeasance,
    /// Breach of statutory duty
    BreachOfStatutoryDuty,
}

// ============================================================================
// Human Rights Act 1998
// ============================================================================

/// ECHR Article
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EchrArticle {
    /// Article 2 - Right to life
    Article2,
    /// Article 3 - Prohibition of torture
    Article3,
    /// Article 4 - Prohibition of slavery
    Article4,
    /// Article 5 - Right to liberty
    Article5,
    /// Article 6 - Right to fair trial
    Article6,
    /// Article 7 - No punishment without law
    Article7,
    /// Article 8 - Right to private and family life
    Article8,
    /// Article 9 - Freedom of thought, conscience, religion
    Article9,
    /// Article 10 - Freedom of expression
    Article10,
    /// Article 11 - Freedom of assembly
    Article11,
    /// Article 12 - Right to marry
    Article12,
    /// Article 14 - Prohibition of discrimination
    Article14,
    /// Protocol 1 Article 1 - Protection of property
    Protocol1Article1,
    /// Protocol 1 Article 2 - Right to education
    Protocol1Article2,
    /// Protocol 1 Article 3 - Right to free elections
    Protocol1Article3,
}

impl EchrArticle {
    /// Whether the right is absolute (non-derogable)
    pub fn is_absolute(&self) -> bool {
        matches!(
            self,
            Self::Article2 | Self::Article3 | Self::Article4 | Self::Article7
        )
    }

    /// Whether the right is qualified (can be limited)
    pub fn is_qualified(&self) -> bool {
        matches!(
            self,
            Self::Article8 | Self::Article9 | Self::Article10 | Self::Article11
        )
    }

    /// Whether the right is limited (specific exceptions)
    pub fn is_limited(&self) -> bool {
        matches!(self, Self::Article5 | Self::Article6 | Self::Article12)
    }
}

/// Type of HRA duty
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HraDuty {
    /// Negative duty - not to interfere
    Negative,
    /// Positive duty - to take action to protect
    Positive,
    /// Procedural duty - to investigate
    Procedural,
}

/// HRA s.3 interpretation outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section3Outcome {
    /// Legislation can be read compatibly
    CompatibleReading { interpretation: String },
    /// Legislation cannot be read compatibly
    IncompatibleReading,
}

/// HRA s.4 declaration of incompatibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclarationOfIncompatibility {
    /// Provision declared incompatible
    pub provision: String,
    /// Article violated
    pub article: EchrArticle,
    /// Incompatibility identified
    pub incompatibility: String,
}

/// HRA s.6 public authority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section6Authority {
    /// Core public authority (always bound)
    Core { name: String },
    /// Hybrid/functional public authority (bound when exercising public functions)
    Hybrid {
        name: String,
        public_function: String,
    },
    /// Not a public authority
    NotPublicAuthority,
}

// ============================================================================
// Proportionality
// ============================================================================

/// Proportionality analysis framework
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProportionalityAnalysis {
    /// Legitimate aim
    pub legitimate_aim: LegitimateAim,
    /// Rational connection to aim
    pub rational_connection: bool,
    /// Necessary in democratic society
    pub necessary: bool,
    /// Fair balance struck
    pub fair_balance: bool,
    /// Overall proportionate
    pub proportionate: bool,
    /// Analysis reasoning
    pub reasoning: String,
}

/// Legitimate aims for rights interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegitimateAim {
    /// National security
    NationalSecurity,
    /// Public safety
    PublicSafety,
    /// Economic wellbeing
    EconomicWellbeing,
    /// Prevention of disorder or crime
    PreventionOfCrime,
    /// Protection of health
    ProtectionOfHealth,
    /// Protection of morals
    ProtectionOfMorals,
    /// Protection of rights of others
    ProtectionOfRightsOfOthers,
    /// Other aim
    Other { description: String },
}

// ============================================================================
// Constitutional Principles
// ============================================================================

/// Constitutional principle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalPrinciple {
    /// Parliamentary sovereignty
    ParliamentarySovereignty,
    /// Rule of law
    RuleOfLaw,
    /// Separation of powers
    SeparationOfPowers,
    /// Royal prerogative
    RoyalPrerogative,
    /// Principle of legality
    PrincipleOfLegality,
    /// Access to justice
    AccessToJustice,
    /// Constitutional statutes
    ConstitutionalStatute { statute: String },
}

/// Rule of law sub-principles (Dicey, Bingham)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleOfLawPrinciple {
    /// No punishment without law (nulla poena sine lege)
    Legality,
    /// Laws must be certain and predictable
    LegalCertainty,
    /// Equality before the law
    Equality,
    /// Access to justice and the courts
    AccessToJustice,
    /// Supremacy of law over arbitrary power
    SupremacyOfLaw,
    /// No punishment without law (Latin)
    NullumCrimenSineLege,
    /// Due process
    DueProcess,
    /// Government must act within its powers
    GovernmentWithinPowers,
    /// Laws must be accessible and clear
    AccessibleAndClear,
}

// ============================================================================
// Case Law Citations
// ============================================================================

/// Case citation for public law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicLawCitation {
    /// Case name
    pub name: String,
    /// Year
    pub year: u32,
    /// Report citation
    pub citation: String,
    /// Principle established
    pub principle: String,
}

impl PublicLawCitation {
    /// Create a new case citation
    pub fn new(
        name: impl Into<String>,
        year: u32,
        citation: impl Into<String>,
        principle: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            year,
            citation: citation.into(),
            principle: principle.into(),
        }
    }
}

// ============================================================================
// Analysis Results
// ============================================================================

/// Result of judicial review analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JrAnalysisResult {
    /// Is the claim reviewable?
    pub reviewable: bool,
    /// Does claimant have standing?
    pub standing: bool,
    /// Is claim in time?
    pub in_time: bool,
    /// Grounds identified
    pub grounds: Vec<GroundOfReview>,
    /// Likelihood of success
    pub success_likelihood: SuccessLikelihood,
    /// Recommended remedies
    pub remedies: Vec<JrRemedy>,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Overall analysis
    pub analysis: String,
}

/// Likelihood of success
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccessLikelihood {
    /// Arguable (sufficient for permission)
    Arguable,
    /// Reasonable prospects
    ReasonableProspects,
    /// Good prospects
    GoodProspects,
    /// Strong case
    Strong,
    /// Weak/unarguable
    Weak,
}

/// Result of HRA analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HraAnalysisResult {
    /// Articles potentially engaged
    pub articles_engaged: Vec<EchrArticle>,
    /// Is respondent a public authority under s.6?
    pub public_authority: Section6Authority,
    /// Has there been an interference?
    pub interference: bool,
    /// Is interference justified?
    pub justified: Option<bool>,
    /// Proportionality analysis (if applicable)
    pub proportionality: Option<ProportionalityAnalysis>,
    /// Section 3 interpretation possible?
    pub section_3: Option<Section3Outcome>,
    /// Section 4 declaration appropriate?
    pub section_4: Option<DeclarationOfIncompatibility>,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Overall analysis
    pub analysis: String,
}

// ============================================================================
// Time Limits
// ============================================================================

/// Judicial review time limit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JrTimeLimit {
    /// General limit (3 months)
    pub general_limit_days: u32,
    /// Promptness requirement
    pub promptness_required: bool,
    /// Specific shorter limit (e.g., planning 6 weeks)
    pub specific_limit: Option<SpecificLimit>,
}

impl Default for JrTimeLimit {
    fn default() -> Self {
        Self {
            general_limit_days: 90, // 3 months
            promptness_required: true,
            specific_limit: None,
        }
    }
}

/// Specific time limits for certain decisions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecificLimit {
    /// Planning decisions (6 weeks)
    Planning { weeks: u32 },
    /// Procurement (30 days)
    Procurement { days: u32 },
    /// Other specific limit
    Other { days: u32, basis: String },
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echr_article_types() {
        assert!(EchrArticle::Article3.is_absolute());
        assert!(!EchrArticle::Article8.is_absolute());
        assert!(EchrArticle::Article8.is_qualified());
        assert!(EchrArticle::Article5.is_limited());
    }

    #[test]
    fn test_case_citation() {
        let case = PublicLawCitation::new(
            "Associated Provincial Picture Houses v Wednesbury Corporation",
            1948,
            "1 KB 223",
            "Unreasonableness standard for judicial review",
        );
        assert_eq!(case.year, 1948);
        assert!(case.principle.contains("Unreasonableness"));
    }

    #[test]
    fn test_jr_time_limit_default() {
        let limit = JrTimeLimit::default();
        assert_eq!(limit.general_limit_days, 90);
        assert!(limit.promptness_required);
    }

    #[test]
    fn test_ground_of_review() {
        let ground = GroundOfReview::Illegality(IllegalityType::UltraVires);
        assert!(matches!(ground, GroundOfReview::Illegality(_)));
    }

    #[test]
    fn test_proportionality_analysis() {
        let analysis = ProportionalityAnalysis {
            legitimate_aim: LegitimateAim::PublicSafety,
            rational_connection: true,
            necessary: true,
            fair_balance: true,
            proportionate: true,
            reasoning: "Measure is proportionate to aim".into(),
        };
        assert!(analysis.proportionate);
    }
}
