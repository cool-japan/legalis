//! Administrative Procedure Act Error Types
//!
//! Error types for administrative procedure operations, including violations
//! of the Administrative Procedure Act (行政手続法) and Electronic Signatures Act (電子署名法).

use thiserror::Error;

/// Administrative Procedure Act errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AdministrativeError {
    /// Missing required field (Article reference included)
    #[error("Missing required field: {field} (行政手続法第{article}条)")]
    MissingRequiredField { field: String, article: String },

    /// Invalid electronic certificate
    #[error("Invalid certificate: {reason} (電子署名法)")]
    InvalidCertificate { reason: String },

    /// Certificate expired or not yet valid
    #[error("Certificate validity period error: {reason}")]
    CertificateValidityError { reason: String },

    /// Unsupported signature algorithm
    #[error("Unsupported signature algorithm: {algorithm}")]
    UnsupportedSignatureAlgorithm { algorithm: String },

    /// Processing period exceeds limit
    #[error("Processing period exceeds limit: {actual} days > {limit} days (行政手続法第7条)")]
    ProcessingPeriodExceeded { actual: u32, limit: u32 },

    /// Missing reason statement (Article 5 violation)
    #[error("Reason statement required for disposition (行政手続法第5条)")]
    MissingReasonStatement,

    /// Invalid procedure type
    #[error("Invalid procedure type: {0}")]
    InvalidProcedureType(String),

    /// e-Gov filing error
    #[error("Filing error: {0}")]
    FilingError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Invalid status transition
    #[error("Invalid status transition: cannot change from {from:?} to {to:?}")]
    InvalidStatusTransition { from: String, to: String },

    /// Document format error
    #[error("Document format error: {0}")]
    DocumentFormatError(String),

    /// Prior notification period insufficient (Article 6)
    #[error(
        "Prior notification period insufficient: {actual} days < {required} days (行政手続法第6条)"
    )]
    InsufficientNotificationPeriod { actual: u32, required: u32 },

    /// Generic error
    #[error("Administrative procedure error: {0}")]
    Other(String),
}

/// Result type for administrative procedure operations
pub type Result<T> = std::result::Result<T, AdministrativeError>;

impl AdministrativeError {
    /// Check if the error is related to certificates
    pub fn is_certificate_error(&self) -> bool {
        matches!(
            self,
            AdministrativeError::InvalidCertificate { .. }
                | AdministrativeError::CertificateValidityError { .. }
                | AdministrativeError::UnsupportedSignatureAlgorithm { .. }
        )
    }

    /// Check if the error is related to validation
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            AdministrativeError::MissingRequiredField { .. }
                | AdministrativeError::Validation(_)
                | AdministrativeError::MissingReasonStatement
        )
    }

    /// Check if the error is related to procedural requirements
    pub fn is_procedural_error(&self) -> bool {
        matches!(
            self,
            AdministrativeError::ProcessingPeriodExceeded { .. }
                | AdministrativeError::InsufficientNotificationPeriod { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AdministrativeError::MissingRequiredField {
            field: "applicant_name".to_string(),
            article: "5".to_string(),
        };
        assert!(err.to_string().contains("applicant_name"));
        assert!(err.to_string().contains("行政手続法"));
    }

    #[test]
    fn test_certificate_error_check() {
        let err = AdministrativeError::InvalidCertificate {
            reason: "expired".to_string(),
        };
        assert!(err.is_certificate_error());
        assert!(!err.is_validation_error());
    }

    #[test]
    fn test_validation_error_check() {
        let err = AdministrativeError::MissingReasonStatement;
        assert!(err.is_validation_error());
        assert!(!err.is_certificate_error());
    }

    #[test]
    fn test_procedural_error_check() {
        let err = AdministrativeError::ProcessingPeriodExceeded {
            actual: 120,
            limit: 90,
        };
        assert!(err.is_procedural_error());
        assert!(!err.is_validation_error());
    }

    #[test]
    fn test_error_equality() {
        let err1 = AdministrativeError::MissingReasonStatement;
        let err2 = AdministrativeError::MissingReasonStatement;
        assert_eq!(err1, err2);
    }
}
