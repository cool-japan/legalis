//! UK Criminal Defences
//!
//! This module provides comprehensive analysis of criminal defences
//! available under English law.
//!
//! # Modules
//!
//! - [`general`] - General defences (self-defence, duress, intoxication, etc.)
//! - [`insanity`] - Insanity defence and related mental condition defences

pub mod general;
pub mod insanity;

// Re-export key types from general defences
pub use general::{
    AutomatismAnalyzer, AutomatismCause, AutomatismFacts, AutomatismResult, AutomatismType,
    ConsentActivityType, ConsentDefenceAnalyzer, ConsentDefenceFacts, ConsentDefenceResult,
    ConsentFraud, ConsentHarmLevel, ConsentValidity, DuressAnalyzer, DuressFacts, DuressResponse,
    DuressResult, DuressTestFindings, DuressType, HonestBeliefFacts, IntoxicationAnalyzer,
    IntoxicationEffect, IntoxicationFacts, IntoxicationLevel, IntoxicationResult, IntoxicationType,
    MistakeAnalyzer, MistakeDetails, MistakeFacts, MistakeResult, MistakeType, OffenceIntentType,
    ReasonablenessFacts, SelfDefenceAnalyzer, SelfDefenceFacts, SelfDefenceResult, SelfDefenceType,
    SpecialCircumstances, ThreatDetails, ThreatTarget, VoluntaryAssociation, analyze_defence,
};

// Re-export key types from insanity
pub use insanity::{
    BalanceDisturbanceReason, BalanceDisturbedFacts, CurrentMentalState, DefectOfReasonFacts,
    DiseaseOfMindFacts, FirstLimbFacts, InfanticideAnalyzer, InfanticideFacts, InfanticideResult,
    InsanityAnalyzer, InsanityDisposal, InsanityFacts, InsanityResult, InsanityVerdict,
    McNaghtenLimbs, McNaghtenLimbsResult, MentalStateSeverity, MindDiseaseCause, PritchardCriteria,
    PsychiatricEvidence, SecondLimbFacts, TreatmentStatus, UnfitnessAnalyzer,
    UnfitnessRecommendation, UnfitnessResult, UnfitnessToPlead, insanity_to_defence_result,
};
