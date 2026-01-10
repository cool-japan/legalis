//! Error types for contract breach validation

use thiserror::Error;

/// Errors that can occur during contract breach claim validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContractLiabilityError {
    /// One of five necessary conditions missing for Article 415
    #[error("Necessary condition missing: {0}")]
    NecessaryConditionMissing(String),

    /// Missing required field during builder construction
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid field value
    #[error("Invalid value for field {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    /// Insufficient evidence for a requirement
    #[error("Insufficient evidence: {0}")]
    InsufficientEvidence(String),

    /// Multiple validation failures
    #[error("Multiple validation failures: {0:?}")]
    Multiple(Vec<ContractLiabilityError>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ContractLiabilityError::NecessaryConditionMissing("債務の存在".to_string());
        assert!(err.to_string().contains("Necessary condition"));
    }

    #[test]
    fn test_missing_field_error() {
        let err = ContractLiabilityError::MissingField("obligation".to_string());
        assert!(err.to_string().contains("obligation"));
    }
}
