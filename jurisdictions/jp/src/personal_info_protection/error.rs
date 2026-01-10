//! Personal Information Protection Act Error Types
//!
//! Error types for Act on the Protection of Personal Information
//! (個人情報の保護に関する法律 Act No. 57 of 2003, amended 2020/2022).

use thiserror::Error;

/// Personal Information Protection Act errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AppiError {
    /// Purpose not specified at collection (Article 15)
    #[error("Purpose not specified at collection (個人情報保護法第15条)")]
    PurposeNotSpecified,

    /// Consent required for sensitive data (Article 17-2)
    #[error("Consent required for sensitive personal information (個人情報保護法第17条2項)")]
    ConsentRequiredForSensitiveData,

    /// Inadequate security measures (Article 20)
    #[error("Inadequate security measures (個人情報保護法第20条)")]
    InadequateSecurityMeasures,

    /// Third-party provision without consent or opt-out (Article 23)
    #[error("Third-party provision without consent or opt-out (個人情報保護法第23条)")]
    UnauthorizedThirdPartyProvision,

    /// Cross-border transfer without consent (Article 24)
    #[error("Cross-border transfer without consent or adequate protection (個人情報保護法第24条)")]
    UnauthorizedCrossBorderTransfer,

    /// Records not maintained (Article 25)
    #[error("Records of third-party provision not maintained (個人情報保護法第25条)")]
    RecordsNotMaintained,

    /// Data subject request not fulfilled (Articles 28-30)
    #[error("Data subject request not fulfilled: {request_type} (個人情報保護法第{article}条)")]
    RequestNotFulfilled {
        request_type: String,
        article: String,
    },

    /// High-risk AI system without mitigation measures
    #[error("High-risk AI system without adequate mitigation measures")]
    HighRiskAiWithoutMitigation,

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingRequiredField { field: String },

    /// Invalid data handling volume
    #[error("Invalid data handling volume declaration")]
    InvalidDataHandlingVolume,

    /// Purpose change without consent (Article 15-2)
    #[error("Purpose change requires consent (個人情報保護法第15条2項)")]
    PurposeChangeWithoutConsent,

    /// Improper acquisition (Article 17)
    #[error("Improper acquisition of personal information (個人情報保護法第17条)")]
    ImproperAcquisition,

    /// Data breach notification failure (Article 22-2)
    #[error("Data breach notification requirement not met (個人情報保護法第22条の2)")]
    BreachNotificationFailure,

    /// Anonymous processing failure (Article 36)
    #[error("Anonymous processing standards not met (個人情報保護法第36条)")]
    AnonymousProcessingFailure,

    /// Pseudonymous processing failure (Article 35-2)
    #[error("Pseudonymous processing standards not met (個人情報保護法第35条の2)")]
    PseudonymousProcessingFailure,

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Filing error
    #[error("Filing error: {0}")]
    FilingError(String),

    /// Other error
    #[error("{0}")]
    Other(String),
}

/// Result type for APPI operations
pub type Result<T> = std::result::Result<T, AppiError>;

impl AppiError {
    /// Check if error is related to consent requirements
    pub fn is_consent_error(&self) -> bool {
        matches!(
            self,
            Self::ConsentRequiredForSensitiveData
                | Self::UnauthorizedThirdPartyProvision
                | Self::UnauthorizedCrossBorderTransfer
                | Self::PurposeChangeWithoutConsent
        )
    }

    /// Check if error is related to security measures
    pub fn is_security_error(&self) -> bool {
        matches!(
            self,
            Self::InadequateSecurityMeasures | Self::BreachNotificationFailure
        )
    }

    /// Check if error is related to data subject rights
    pub fn is_data_subject_rights_error(&self) -> bool {
        matches!(self, Self::RequestNotFulfilled { .. })
    }

    /// Check if error is related to validation
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            Self::MissingRequiredField { .. } | Self::Validation(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppiError::ConsentRequiredForSensitiveData;
        let msg = err.to_string();
        assert!(msg.contains("個人情報保護法"));
        assert!(msg.contains("17条"));
    }

    #[test]
    fn test_consent_error_check() {
        let err = AppiError::UnauthorizedThirdPartyProvision;
        assert!(err.is_consent_error());
        assert!(!err.is_security_error());
    }

    #[test]
    fn test_security_error_check() {
        let err = AppiError::InadequateSecurityMeasures;
        assert!(err.is_security_error());
        assert!(!err.is_consent_error());
    }

    #[test]
    fn test_data_subject_rights_error() {
        let err = AppiError::RequestNotFulfilled {
            request_type: "Disclosure".to_string(),
            article: "28".to_string(),
        };
        assert!(err.is_data_subject_rights_error());
        let msg = err.to_string();
        assert!(msg.contains("Disclosure"));
        assert!(msg.contains("28"));
    }

    #[test]
    fn test_validation_error_check() {
        let err = AppiError::MissingRequiredField {
            field: "business_name".to_string(),
        };
        assert!(err.is_validation_error());
    }
}
