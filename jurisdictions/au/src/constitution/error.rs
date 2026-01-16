//! Australian Constitutional Law Errors

use thiserror::Error;

/// Constitutional law error
#[derive(Debug, Error)]
pub enum ConstitutionalError {
    /// Invalid constitutional provision reference
    #[error("Invalid constitutional provision: {0}")]
    InvalidProvision(String),

    /// Power not found
    #[error("Commonwealth power not found: {0}")]
    PowerNotFound(String),

    /// Characterization failed
    #[error("Failed to characterize law: {0}")]
    CharacterizationFailed(String),

    /// Inconsistency analysis error
    #[error("Inconsistency analysis failed: {0}")]
    InconsistencyAnalysisFailed(String),

    /// Invalid implied rights analysis
    #[error("Implied rights analysis failed: {0}")]
    ImpliedRightsAnalysisFailed(String),

    /// Missing required facts
    #[error("Missing required facts for analysis: {0}")]
    MissingFacts(String),

    /// Invalid state/territory
    #[error("Invalid state or territory: {0}")]
    InvalidStateTerritory(String),
}

/// Result type for constitutional operations
pub type ConstitutionalResult<T> = Result<T, ConstitutionalError>;
