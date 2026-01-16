//! Canada Family Law - Error Types
//!
//! Error types for Canadian family law analysis.

use thiserror::Error;

/// Family law error
#[derive(Debug, Error)]
pub enum FamilyError {
    /// Missing information
    #[error("Missing family law information: {0}")]
    MissingInformation(String),

    /// Invalid jurisdiction
    #[error("Invalid family law jurisdiction: {0}")]
    InvalidJurisdiction(String),

    /// Divorce ground not established
    #[error("Divorce ground not established: {0}")]
    DivorceGroundNotEstablished(String),

    /// Separation period insufficient
    #[error("Separation period insufficient: {required} months required, {actual} provided")]
    SeparationInsufficient {
        /// Required months
        required: u32,
        /// Actual months
        actual: u32,
    },

    /// Child support calculation error
    #[error("Child support calculation error: {0}")]
    ChildSupportCalculationError(String),

    /// Spousal support calculation error
    #[error("Spousal support calculation error: {0}")]
    SpousalSupportCalculationError(String),

    /// Best interests analysis error
    #[error("Best interests analysis error: {0}")]
    BestInterestsError(String),

    /// Family violence concern
    #[error("Family violence concern identified: {0}")]
    FamilyViolenceConcern(String),

    /// Relocation analysis error
    #[error("Relocation analysis error: {0}")]
    RelocationError(String),

    /// Property division error
    #[error("Property division error: {0}")]
    PropertyDivisionError(String),
}

/// Result type for family law operations
pub type FamilyResult<T> = Result<T, FamilyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = FamilyError::MissingInformation("income data".to_string());
        assert!(error.to_string().contains("income data"));
    }

    #[test]
    fn test_separation_insufficient() {
        let error = FamilyError::SeparationInsufficient {
            required: 12,
            actual: 6,
        };
        assert!(error.to_string().contains("12"));
        assert!(error.to_string().contains("6"));
    }

    #[test]
    fn test_family_violence_concern() {
        let error = FamilyError::FamilyViolenceConcern("physical abuse allegations".to_string());
        assert!(error.to_string().contains("physical abuse"));
    }
}
