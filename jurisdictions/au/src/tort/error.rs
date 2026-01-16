//! Australian Tort Law Errors

use thiserror::Error;

/// Tort law error
#[derive(Debug, Error)]
pub enum TortError {
    /// Duty of care analysis error
    #[error("Duty of care analysis failed: {0}")]
    DutyAnalysisFailed(String),

    /// Breach analysis error
    #[error("Breach analysis failed: {0}")]
    BreachAnalysisFailed(String),

    /// Causation analysis error
    #[error("Causation analysis failed: {0}")]
    CausationAnalysisFailed(String),

    /// Damages calculation error
    #[error("Damages calculation failed: {0}")]
    DamagesCalculationFailed(String),

    /// Defamation analysis error
    #[error("Defamation analysis failed: {0}")]
    DefamationAnalysisFailed(String),

    /// Nuisance analysis error
    #[error("Nuisance analysis failed: {0}")]
    NuisanceAnalysisFailed(String),

    /// Invalid state/territory
    #[error("Invalid state or territory: {0}")]
    InvalidState(String),

    /// Missing required facts
    #[error("Missing required facts: {0}")]
    MissingFacts(String),

    /// CLA defence error
    #[error("CLA defence analysis failed: {0}")]
    CLADefenceFailed(String),
}

/// Result type for tort operations
pub type TortResult<T> = Result<T, TortError>;
