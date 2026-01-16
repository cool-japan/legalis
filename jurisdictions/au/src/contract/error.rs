//! Australian Contract Law Errors

use thiserror::Error;

/// Contract law error
#[derive(Debug, Error)]
pub enum ContractError {
    /// Formation analysis error
    #[error("Contract formation analysis failed: {0}")]
    FormationFailed(String),

    /// Invalid term type
    #[error("Invalid term type: {0}")]
    InvalidTermType(String),

    /// Breach analysis error
    #[error("Breach analysis failed: {0}")]
    BreachAnalysisFailed(String),

    /// Damages calculation error
    #[error("Damages calculation failed: {0}")]
    DamagesCalculationFailed(String),

    /// ACL analysis error
    #[error("ACL analysis failed: {0}")]
    ACLAnalysisFailed(String),

    /// Consumer status error
    #[error("Consumer status determination failed: {0}")]
    ConsumerStatusFailed(String),

    /// Unfair terms analysis error
    #[error("Unfair terms analysis failed: {0}")]
    UnfairTermsAnalysisFailed(String),

    /// Missing required facts
    #[error("Missing required facts: {0}")]
    MissingFacts(String),

    /// Invalid vitiating factor
    #[error("Invalid vitiating factor: {0}")]
    InvalidVitiatingFactor(String),
}

/// Result type for contract operations
pub type ContractResult<T> = Result<T, ContractError>;
