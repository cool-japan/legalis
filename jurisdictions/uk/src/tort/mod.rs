//! UK Tort Law Module
//!
//! This module provides comprehensive coverage of UK tort law, including:
//!
//! ## Negligence
//! - **Duty of care**: Caparo three-stage test (foreseeability, proximity, fair just reasonable)
//! - **Breach of duty**: Reasonable person standard, Bolam/Bolitho for professionals
//! - **Causation**: But-for test, material contribution, Fairchild exception
//! - **Remoteness**: Wagon Mound foreseeability test
//! - **Psychiatric injury**: Primary/secondary victims, Alcock control mechanisms
//! - **Pure economic loss**: Hedley Byrne, Murphy v Brentwood
//!
//! ## Occupiers' Liability
//! - **OLA 1957**: Common duty of care to visitors
//! - **OLA 1984**: Duty to non-visitors (trespassers)
//! - Special considerations for children, skilled visitors, independent contractors
//!
//! ## Nuisance
//! - **Private nuisance**: Unlawful interference with land
//! - **Public nuisance**: Interference with public rights
//! - **Rylands v Fletcher**: Strict liability for dangerous things
//!
//! ## Defamation
//! - **Defamation Act 2013**: Serious harm requirement
//! - **Defences**: Truth (s.2), honest opinion (s.3), public interest (s.4)
//! - **Privilege**: Absolute and qualified
//!
//! ## Economic Torts
//! - **Inducing breach of contract**: OBG v Allan requirements
//! - **Causing loss by unlawful means**: OBG narrow approach
//! - **Conspiracy**: Lawful and unlawful means conspiracy
//!
//! # Key Cases
//!
//! - Donoghue v Stevenson [1932] AC 562 (neighbour principle)
//! - Caparo Industries v Dickman [1990] 2 AC 605 (three-stage test)
//! - Bolam v Friern Hospital [1957] 1 WLR 582 (professional standard)
//! - Bolitho v City & Hackney HA [1998] AC 232 (logical Bolam)
//! - Alcock v Chief Constable [1992] 1 AC 310 (psychiatric injury)
//! - Hedley Byrne v Heller [1964] AC 465 (negligent misstatement)
//! - Murphy v Brentwood [1991] 1 AC 398 (pure economic loss)
//! - Wheat v E Lacon [1966] AC 552 (occupier definition)
//! - Tomlinson v Congleton [2003] UKHL 47 (obvious risks)
//! - Hunter v Canary Wharf [1997] AC 655 (nuisance standing)
//! - Transco v Stockport [2004] 2 AC 1 (Rylands v Fletcher)
//! - Lachaux v Independent Print [2019] UKSC 27 (serious harm)
//! - OBG Ltd v Allan [2008] 1 AC 1 (economic torts)
//!
//! # Example
//!
//! ```ignore
//! use legalis_uk::tort::{
//!     negligence::{CaparoAnalyzer, CaseContext},
//!     types::{HarmType, ProfessionalCapacity},
//! };
//!
//! // Analyze duty of care using Caparo test
//! let context = CaseContext {
//!     harm_type: HarmType::PhysicalInjury,
//!     novel_claim: false,
//!     relationship: "employer-employee".to_string(),
//!     professional_context: None,
//!     facts: "Worker injured on job".to_string(),
//! };
//!
//! let analyzer = CaparoAnalyzer::new(context);
//! // Established employer-employee duty category applies
//! ```

pub mod defamation;
pub mod economic;
pub mod error;
pub mod negligence;
pub mod nuisance;
pub mod occupiers;
pub mod types;

// Re-export core types
pub use types::{
    AlcockControl,
    Apportionment,
    BolamTest,
    BreachEvidence,
    BreachFactor,
    BreachOfDuty,
    CausationAnalysis,
    ChildStandard,
    CloseTie,
    CommonPractice,
    ContributoryNegligence,
    CostLevel,
    Damage,
    DamageType,
    DefenceEffect,
    DefenceType,
    // Negligence types
    DutyOfCareAnalysis,
    EconomicLossClaimType,
    EconomicLossType,
    EstablishedDutyCategory,
    EvidenceStrength,
    EvidenceType,
    ExTurpiCausa,
    ExtendedHedleyByrne,
    FactualCausation,
    FairJustReasonable,
    Foreseeability,
    HarmGravity,
    HarmType,
    HedleyByrneAnalysis,
    IllegalityType,
    InjurySeverity,
    InterveningAct,
    InterveningActType,
    LegalCausation,
    LimitationAnalysis,
    LimitationClaimType,
    LimitationPeriod,
    LossOfChance,
    LossOfChanceCase,
    MaterialContribution,
    MaterialIncrease,
    NegligenceDefence,
    PartyRole,
    PartyType,
    PolicyConsideration,
    ProfessionalCapacity,

    Proximity,
    ProximityTimeSpace,
    ProximityType,
    PsychiatricHarmType,
    PsychiatricInjuryAnalysis,
    PsychiatricVictimType,
    PureEconomicLossAnalysis,
    ReasonablePersonTest,
    Relationship,
    ResIpsaEffect,
    ResIpsaLoquitur,
    RiskLevel,
    SocialUtility,
    StandardOfCare,
    StandardType,
    TortParty,
    // General tort types
    TortType,
    Volenti,
    VolentiExclusion,
};

// Re-export error types
pub use error::TortError;

// Re-export negligence analysis
pub use negligence::{
    BreachAnalyzer, BreachFacts, CaparoAnalyzer, CaseContext, ContributoryNegligenceFacts,
    DefenceAnalyzer, EconomicLossFacts, ForeseeabilityFacts, NegligenceClaimAnalysis, PolicyFacts,
    ProximityFacts, PsychiatricInjuryAnalyzer, PsychiatricInjuryFacts, PureEconomicLossAnalyzer,
    ResIpsaFacts, VolentiFacts,
};

// Re-export occupiers' liability types
pub use occupiers::{
    AwarenessAnalysis, ChildConsiderations, CommonDutyAnalysis, ControlDegree, CostOfProtection,
    DangerType, EntrantStatus, ExclusionClause, IndependentContractorAnalysis, NonVisitorType,
    OLA1957Analysis, OLA1957Facts, OLA1984Analysis, OLA1984Defence, OLA1984DefenceType,
    OLA1984Facts, OLADefence, OLADefenceType, OccupationBasis, Occupier, OccupierAnalysis,
    OccupiersLiabilityAnalyzer, PremisesDanger, PremisesInfo, PremisesType,
    PresenceKnowledgeAnalysis, ReasonableProtectionAnalysis, RiskSeverity, Section1_3Analysis,
    Section1_4Analysis, SkilledVisitorAnalysis, VisitorAnalysis, VolentiFacts1957, Warning,
    WarningAnalysis, WarningType,
};

// Re-export nuisance types
pub use nuisance::{
    DangerousThing, DefendantLiabilityAnalysis, DefendantRole, DefendantUtility, EscapeAnalysis,
    InterferenceAnalysis, InterferenceDuration, InterferenceSeverity, InterferenceType,
    LandInterest, LocalityCharacter, MaliceAnalysis, MischiefAnalysis, NonNaturalFactor,
    NonNaturalUseAnalysis, NuisanceAnalyzer, NuisanceDefence, NuisanceDefenceType, NuisanceRemedy,
    NuisanceRemedyType, NuisanceType, PrivateNuisanceAnalysis, PrivateNuisanceFacts,
    PublicNuisanceAnalysis, PublicNuisanceFacts, PublicNuisanceTest, PublicRight,
    ReasonablenessAnalysis, RylandsDamageType, RylandsDefence, RylandsDefenceType, RylandsFacts,
    RylandsStanding, RylandsVFletcherAnalysis, SensitivityAnalysis, SensitivityEffect,
    SpecialDamageAnalysis, StandingAnalysis, ThingBroughtAnalysis, TimeSensitivity, UtilityLevel,
};

// Re-export defamation types
pub use defamation::{
    ChaseLevel, ClaimantAnalysis, ClaimantType, DefamationAnalyzer, DefamationClaimAnalysis,
    DefamationDefence, DefamationDefenceType, DefamationFacts, DefamationRemedy,
    DefamationRemedyType, DefamationType, DefenceAnalysis,
    EvidenceStrength as DefamationEvidenceStrength, FinancialLossAnalysis, GenericDefenceAnalysis,
    HarmEvidence, HarmEvidenceType, HarmType as DefamationHarmType, HonestOpinionAnalysis,
    HonestOpinionFacts, InnuendoType, InuendoMeaning, MeaningAnalysis, PrivilegeAnalysis,
    PrivilegeFacts, PrivilegeType, PublicInterestAnalysis, PublicInterestFactor,
    PublicInterestFacts, PublicationAnalysis, PublicationExtent, PublicationMedium, PublisherRole,
    SeriousHarmAnalysis, StatementAnalysis, StatementType, TruthAnalysis, TruthDefenceFacts,
    WebsiteOperatorAnalysis, WebsiteOperatorFacts,
};

// Re-export economic tort types
pub use economic::{
    AgreementAnalysis, BreachAnalysis as EconomicBreachAnalysis, ConspiracyAnalysis,
    ConspiracyFacts, ConspiracyIntention, ConspiracyType, ContractAnalysis, ContractType,
    DamageAnalysis as ConspiracyDamageAnalysis, DealingAnalysis, EconomicDefenceType,
    EconomicLossType as EconomicTortLossType, EconomicTortAnalyzer, EconomicTortDefence,
    EconomicTortType, InducementAnalysis, InducementMethod, InducingBreachAnalysis,
    InducingBreachFacts, IntentionAnalysis, JustificationAnalysis, JustificationFacts,
    KnowledgeAnalysis, KnowledgeType, LossAnalysis, UnlawfulMeansAnalysis, UnlawfulMeansDetail,
    UnlawfulMeansFact, UnlawfulMeansFacts, UnlawfulMeansIntention, UnlawfulMeansType,
};
