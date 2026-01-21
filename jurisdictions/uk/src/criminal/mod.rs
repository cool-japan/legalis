//! UK Criminal Law Module
//!
//! This module provides comprehensive coverage of English criminal law,
//! including offences, defences, sentencing, and procedure.
//!
//! # Modules
//!
//! - `types` - Core criminal law type definitions
//! - `error` - Error types for criminal law operations
//! - `offences` - Criminal offences (homicide, assault, theft, fraud)
//! - `defences` - Criminal defences (self-defence, duress, insanity)
//! - `sentencing` - Sentencing guidelines and dangerous offenders
//! - `procedure` - Criminal procedure (PACE, arrest, detention)
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_uk::criminal::{
//!     offences::{MurderAnalyzer, MurderFacts},
//!     defences::{SelfDefenceAnalyzer, SelfDefenceFacts},
//! };
//!
//! // Analyze a murder case
//! let facts = MurderFacts { /* ... */ };
//! let result = MurderAnalyzer::analyze(&facts)?;
//! ```

pub mod defences;
pub mod error;
pub mod offences;
pub mod procedure;
pub mod sentencing;
pub mod types;

// Re-export error types
pub use error::{
    ActusReusError, CausationError, CriminalError, CriminalResult, DefenceError, MensReaError,
    OffenceAnalysisError, PartyLiabilityError, ProcedureError, SentencingError,
};

// Re-export core types
pub use types::{
    ActType,
    // Actus reus
    ActusReusElement,
    AggravatingFactor,
    AppealRoute,
    // Case law
    CaseCitation,
    // Causation
    CausationAnalysis,
    CommunityOrder,
    CommunityRequirement,
    // Procedure
    CriminalStage,
    CulpabilityAssessment,
    CulpabilityCategory,
    CustodialSentence,
    CustodyType,
    // Defences
    DefenceCategory,
    DefenceEffect,
    DefenceResult,
    DefenceType,
    DepartureAnalysis,
    DirectIntentionFacts,
    DischargeType,
    DishonestyAnalysis,
    ExtendedSentence,
    FactualCausation,
    FineDetails,
    GuiltyPleaReduction,
    HarmAssessment,
    HarmCategory,
    InterveningAct,
    InterveningActType,
    InvoluntaryCause,
    JointEnterpriseAnalysis,
    LegalCausation,
    LifeSentenceType,
    MaximumSentence,
    MensReaAnalysis,
    // Mens rea
    MensReaType,
    MitigatingFactor,
    ObliqueIntentionFacts,
    // Offence classification
    Offence,
    OffenceBuilder,
    OffenceCategory,
    OffenceClassification,
    OffenceSeverity,
    OmissionDuty,
    // Parties
    PartyRole,
    PleaStage,
    RecklessnessAnalysis,
    SecondaryParticipation,
    SentenceRange,
    // Sentencing types
    SentenceType,
    // Sentencing guidelines
    SentencingGuideline,
    StatutorySource,
    SuspendedSentence,
    ThinSkullAnalysis,
    Verdict,
    VoluntarinessAnalysis,
    WithdrawalAnalysis,
};

// Re-export offence analyzers and types
pub use offences::{
    ABHFacts,
    AbuseOfPositionAnalyzer,
    AbuseOfPositionFacts,
    AbuseOfPositionResult,
    AggravatedAssaultAnalyzer,
    AggravatedAssaultResult,
    // Assault
    AssaultBatteryAnalyzer,
    AssaultBatteryResult,
    AssaultFacts,
    BatteryFacts,
    BurglaryAnalysisResult,
    BurglaryAnalyzer,
    BurglaryFacts,
    CorporateManslaughterAnalyzer,
    CorporateManslaughterFacts,
    CorporateManslaughterResult,
    DiseaseTransmissionAnalyzer,
    DiseaseTransmissionFacts,
    DiseaseTransmissionResult,
    FailingToDiscloseAnalyzer,
    FailingToDiscloseFacts,
    FailingToDiscloseResult,
    // Fraud
    FalseRepresentationAnalyzer,
    FalseRepresentationFacts,
    FalseRepresentationResult,
    FraudType,
    GrossNegligenceManslaughterAnalyzer,
    GrossNegligenceManslaughterFacts,
    GrossNegligenceManslaughterResult,
    HandlingAnalysisResult,
    HandlingAnalyzer,
    HandlingFacts,
    HomicideVerdict,
    MurderAnalysisResult,
    // Homicide
    MurderAnalyzer,
    MurderFacts,
    MurderSentencing,
    NonFatalOffence,
    ObtainingServicesAnalyzer,
    ObtainingServicesFacts,
    ObtainingServicesResult,
    PropertyOffence,
    RobberyAnalysisResult,
    RobberyAnalyzer,
    RobberyFacts,
    Schedule21StartingPoint,
    Section18Facts,
    Section20Facts,
    TheftAnalysisResult,
    // Theft
    TheftAnalyzer,
    TheftFacts,
    UnlawfulActManslaughterAnalyzer,
    UnlawfulActManslaughterFacts,
    UnlawfulActManslaughterResult,
    abh_offence,
    abuse_of_position_offence,
    battery_offence,
    burglary_offence,
    common_assault_offence,
    corporate_manslaughter_offence,
    failing_to_disclose_offence,
    false_representation_offence,
    gross_negligence_manslaughter_offence,
    handling_offence,
    // Offence definitions
    murder_offence,
    obtaining_services_offence,
    robbery_offence,
    section18_offence,
    section20_offence,
    theft_offence,
    unlawful_act_manslaughter_offence,
    voluntary_manslaughter_offence,
};

// Re-export defence analyzers and types
pub use defences::{
    AutomatismAnalyzer,
    AutomatismCause,
    AutomatismFacts,
    AutomatismResult,
    AutomatismType,
    ConsentDefenceAnalyzer,
    ConsentDefenceFacts,
    ConsentDefenceResult,
    ConsentHarmLevel,
    DuressAnalyzer,
    DuressFacts,
    DuressResult,
    DuressTestFindings,
    DuressType,
    InfanticideAnalyzer,
    InfanticideFacts,
    InfanticideResult,
    // Insanity
    InsanityAnalyzer,
    InsanityDisposal,
    InsanityFacts,
    InsanityResult,
    InsanityVerdict,
    IntoxicationAnalyzer,
    IntoxicationFacts,
    IntoxicationResult,
    IntoxicationType,
    McNaghtenLimbs,
    McNaghtenLimbsResult,
    MindDiseaseCause,
    MistakeAnalyzer,
    MistakeFacts,
    MistakeResult,
    MistakeType,
    OffenceIntentType as IntoxicationOffenceType,
    PritchardCriteria,
    // General defences
    SelfDefenceAnalyzer,
    SelfDefenceFacts,
    SelfDefenceResult,
    SelfDefenceType,
    UnfitnessAnalyzer,
    UnfitnessRecommendation,
    UnfitnessResult,
    UnfitnessToPlead,
    analyze_defence,
    insanity_to_defence_result,
};

// Re-export sentencing types
pub use sentencing::{
    CulpabilityFactor, DangerousOffenderAnalyzer, DangerousOffenderFacts, DangerousOffenderResult,
    ExtendedSentenceAssessment, FactorWeight, GuiltyPleaFacts, HarmFactor, LifeSentenceAssessment,
    NineStepAnalysis, OffenceCategory as SentencingCategory, PreviousConviction, RemandTime,
    SentenceRecommendation, SentencingAnalysisResult, SentencingFacts, SentencingGuidelineAnalyzer,
    TotalityFacts,
};

// Re-export procedure types
pub use procedure::{
    AdmissibilityRisk,
    AllocationFactor,
    AppropriateAdult,
    ArrestAnalysisResult,
    // Arrest
    ArrestAnalyzer,
    ArrestFacts,
    ArrestGrounds,
    ArrestingOfficer,
    CautionStatus,
    DetentionAnalysisResult,
    // Detention
    DetentionAnalyzer,
    DetentionDuration,
    DetentionExtension,
    DetentionFacts,
    DetentionReview,
    DetentionRights,
    ExclusionGround,
    ExtensionAuthority,
    InterviewAnalysisResult,
    // Interview
    InterviewAnalyzer,
    InterviewConduct,
    InterviewFacts,
    LegalRepresentation,
    // Mode of trial
    ModeOfTrialAnalyzer,
    ModeOfTrialFacts,
    ModeOfTrialResult,
    ModePreference,
    NecessityCriteria,
    NecessityCriterion,
    RecordingStatus,
    ReviewOutcome,
    RightStatus,
    RiskLevel,
    SpecialWarning,
    TrialVenue,
    UnlawfulArrestConsequences,
};
