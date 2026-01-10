//! Error types for tort claim validation

use thiserror::Error;

/// Errors that can occur during tort claim validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// No intent or negligence established (故意・過失なし)
    #[error("No intent or negligence established (故意・過失の立証なし)")]
    NoIntentOrNegligence,

    /// No causal link between act and damage (因果関係なし)
    #[error("No causal link established between tortious act and damages (因果関係の立証なし)")]
    NoCausalLink,

    /// No damage proven (損害の立証なし)
    #[error("No damages proven (損害の立証なし)")]
    NoDamage,

    /// No protected interest infringed (権利侵害なし)
    #[error("No legally protected interest infringed (権利侵害の立証なし)")]
    NoInfringement,

    /// Insufficient evidence for negligence (過失立証不足)
    #[error("Insufficient evidence of negligence: {0}")]
    InsufficientEvidence(String),

    /// Lack of responsibility capacity (責任能力なし)
    #[error("Tortfeasor lacks responsibility capacity (age < 12 or mental incapacity)")]
    NoResponsibilityCapacity,

    /// Article 709 liability not established (Article 710 precondition)
    #[error(
        "Article 709 liability not established as precondition for Article 710 (709条責任成立なし)"
    )]
    Article709NotEstablished,

    /// Not during business execution (Article 715)
    #[error("Tort did not occur during business execution (事業執行について該当なし)")]
    NotDuringBusinessExecution,

    /// Employment relationship missing (Article 715)
    #[error("No valid employment relationship established (使用関係なし)")]
    NoEmploymentRelationship,

    /// Independent contractor (Article 715)
    #[error("Tortfeasor is independent contractor; Article 715 does not apply (独立請負人)")]
    IndependentContractor,

    /// Multiple validation failures
    #[error("Multiple validation failures: {0:?}")]
    Multiple(Vec<ValidationError>),
}

/// Errors related to building tort claims
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TortClaimError {
    /// Validation error
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid field value
    #[error("Invalid value for field {field}: {reason}")]
    InvalidValue { field: String, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ValidationError::NoIntentOrNegligence;
        assert!(err.to_string().contains("故意・過失"));
    }

    #[test]
    fn test_tort_claim_error_from_validation_error() {
        let validation_err = ValidationError::NoDamage;
        let tort_err: TortClaimError = validation_err.into();

        assert!(matches!(tort_err, TortClaimError::Validation(_)));
    }
}
