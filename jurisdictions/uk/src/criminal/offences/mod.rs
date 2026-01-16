//! UK Criminal Offences
//!
//! This module provides comprehensive analysis of criminal offences
//! under English law.
//!
//! # Modules
//!
//! - [`homicide`] - Murder, manslaughter, corporate manslaughter
//! - [`assault`] - Non-fatal offences against the person
//! - [`theft`] - Theft Act 1968 offences
//! - [`fraud`] - Fraud Act 2006 offences

pub mod assault;
pub mod fraud;
pub mod homicide;
pub mod theft;

// Re-export key types from each module
pub use assault::{
    ABHFacts, ABHHarm, ABHHarmType, ABHMensRea, AggravatedAssaultAnalyzer, AggravatedAssaultResult,
    ApprehensionDetails, AssaultBatteryAnalyzer, AssaultBatteryResult, AssaultConduct,
    AssaultFacts, AssaultMensRea, AssaultOrBattery, BatteryFacts, BatteryMensRea, CausationDetails,
    ConsentActivityType, ConsentFacts, DiseaseKnowledge, DiseaseSeverity,
    DiseaseTransmissionAnalyzer, DiseaseTransmissionFacts, DiseaseTransmissionResult, ForceDetails,
    ForceType, InflictionDetails, NonFatalOffence, Section18Facts, Section18Intent, Section20Facts,
    Section20Harm, Section20HarmType, Section20MensRea, SelfDefenceFacts, abh_offence,
    battery_offence, common_assault_offence, section18_offence, section20_offence,
};

pub use fraud::{
    AbuseAnalysisResult, AbuseDetails, AbuseOfPositionAnalyzer, AbuseOfPositionFacts,
    AbuseOfPositionResult, AbuseType, DisclosureDuty, DutyAnalysis, DutySource,
    FailingToDiscloseAnalyzer, FailingToDiscloseFacts, FailingToDiscloseResult, FailureAnalysis,
    FalseRepresentationAnalyzer, FalseRepresentationFacts, FalseRepresentationResult,
    FraudDishonestyFacts, FraudType, GainLossIntent, IntentAnalysis, ObtainingServicesAnalyzer,
    ObtainingServicesFacts, ObtainingServicesResult, PaymentIntent, PositionAnalysis,
    PositionDetails, PositionType, RelatedFraudOffence, RepresentationAnalysis,
    RepresentationDetails, RepresentationType, ServiceDetails, UndisclosedInformation,
    abuse_of_position_offence, failing_to_disclose_offence, false_representation_offence,
    obtaining_services_offence,
};

pub use homicide::{
    ActusReusAnalysisResult, BreachOfDutyFacts, CausationFacts, ConductType,
    CorporateManslaughterAnalyzer, CorporateManslaughterFacts, CorporateManslaughterResult,
    CorporateManslaughterSentencing, DangerousnessAssessment, DangerousnessFinding, DeathDetails,
    DefendantConduct, DiminishedImpairments, DiminishedResponsibilityFacts, DutyOfCareFacts,
    GrossNegligenceAssessment, GrossNegligenceManslaughterAnalyzer,
    GrossNegligenceManslaughterFacts, GrossNegligenceManslaughterResult, HomicideVerdict,
    IntentEvidence, InterveningActDetails, InvoluntaryManslaughterType, LossOfControlFacts,
    ManagementFailureDetails, MurderAnalysisResult, MurderAnalyzer, MurderFacts, MurderSentencing,
    NormalToleranceAssessment, ObliqueIntentEvidence, OmissionDutySource, OrganizationDetails,
    OrganizationType, PartialDefenceFacts, PremediationEvidence, QualifyingTrigger,
    Schedule21StartingPoint, SeniorManagementInvolvement, SuicidePactFacts, ThinSkullDetails,
    UnlawfulActDetails, UnlawfulActFinding, UnlawfulActManslaughterAnalyzer,
    UnlawfulActManslaughterFacts, UnlawfulActManslaughterResult, VictimDetails,
    VictimVulnerability, WeaponType, WeaponUsed, corporate_manslaughter_offence,
    gross_negligence_manslaughter_offence, murder_offence, unlawful_act_manslaughter_offence,
    voluntary_manslaughter_offence,
};

pub use theft::{
    AppropriationAnalysis, AppropriationFacts, AppropriationType, BelongingAnalysis,
    BelongingFacts, BelongingType, BuildingFacts, BuildingType, BurglaryAnalysisResult,
    BurglaryAnalyzer, BurglaryFacts, BurglaryIntent, BurglarySection, BurglarySubOffence,
    DeemedIntention, EntryFacts, EntryType, ForceAnalysis, HandledGoodsDetails,
    HandlingAnalysisResult, HandlingAnalyzer, HandlingBenefit, HandlingFacts, HandlingKnowledge,
    HandlingType, IntentionDepriveAnalysis, IntentionDepriveFacts, LandException, PropertyAnalysis,
    PropertyFacts, PropertyOffence, PropertyType, RobberyAnalysisResult, RobberyAnalyzer,
    RobberyFacts, RobberyForceFacts, SpecialBelongingSituation, TheftAnalysisResult, TheftAnalyzer,
    TheftDishonestyFacts, TheftFacts, burglary_offence, handling_offence, robbery_offence,
    theft_offence,
};
