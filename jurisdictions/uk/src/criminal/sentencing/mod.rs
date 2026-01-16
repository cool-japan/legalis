//! UK Criminal Sentencing
//!
//! This module provides comprehensive analysis of sentencing under English law,
//! implementing Sentencing Council guidelines and dangerous offender provisions.
//!
//! # Modules
//!
//! - [`guidelines`] - Sentencing Council guidelines framework

pub mod guidelines;

// Re-export key types
pub use guidelines::{
    AggravatingAnalysis, ConvictionRelevance, CulpabilityFactor, DangerousOffenderAnalyzer,
    DangerousOffenderFacts, DangerousOffenderResult, ExtendedSentenceAssessment, FactorWeight,
    GuiltyPleaFacts, HarmFactor, LifeSentenceAssessment, MitigatingAnalysis, NineStepAnalysis,
    OffenceCategory, PreviousConviction, PreviousOffence, RemandTime, RiskAssessment,
    SentenceRecommendation, SentencingAnalysisResult, SentencingFacts, SentencingGuidelineAnalyzer,
    TotalityFacts,
};
