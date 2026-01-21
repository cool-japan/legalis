//! Error types for Australian Privacy Law
//!
//! This module provides comprehensive error types with statutory references
//! for privacy law violations and compliance failures.
//!
//! ## Statutory References
//!
//! Errors reference specific Australian Privacy Principles (APPs) and
//! sections of the Privacy Act 1988.
//!
//! ## Penalty Structure (Post-2022 Amendment)
//!
//! ### Serious/Repeated Interferences
//! - Individuals: Up to $2.5 million
//! - Bodies corporate: Greater of:
//!   - $50 million, OR
//!   - 3x benefit obtained, OR
//!   - 30% of adjusted turnover
//!
//! ### Other Contraventions
//! - Civil penalties vary by provision

use thiserror::Error;

/// Result type for privacy law operations
pub type Result<T> = std::result::Result<T, PrivacyError>;

/// Comprehensive error type for Australian privacy law
#[derive(Debug, Error)]
pub enum PrivacyError {
    /// APP 1 - Open and transparent management
    #[error("APP 1 Violation: {description} - Privacy policy requirement not met")]
    App1Violation {
        /// Description of violation
        description: String,
        /// Missing elements
        missing_elements: Vec<String>,
    },

    /// APP 2 - Anonymity and pseudonymity
    #[error("APP 2 Violation: {description} - Failed to offer anonymity/pseudonymity option")]
    App2Violation {
        /// Description
        description: String,
    },

    /// APP 3 - Collection of solicited personal information
    #[error("APP 3 Violation: {description} - Unlawful collection of {information_type}")]
    App3Violation {
        /// Description
        description: String,
        /// Type of information
        information_type: String,
        /// Whether sensitive information
        is_sensitive: bool,
    },

    /// APP 4 - Unsolicited personal information
    #[error("APP 4 Violation: Unsolicited {information_type} not properly handled")]
    App4Violation {
        /// Type of information
        information_type: String,
        /// Whether information should have been destroyed
        should_destroy: bool,
    },

    /// APP 5 - Notification of collection
    #[error("APP 5 Violation: {description} - Collection notice requirements not met")]
    App5Violation {
        /// Description
        description: String,
        /// Missing notice elements
        missing_elements: Vec<String>,
    },

    /// APP 6 - Use or disclosure
    #[error("APP 6 Violation: Unauthorised use/disclosure - {description}")]
    App6Violation {
        /// Description
        description: String,
        /// Primary purpose
        primary_purpose: String,
        /// Secondary purpose (if applicable)
        secondary_purpose: Option<String>,
    },

    /// APP 7 - Direct marketing
    #[error("APP 7 Violation: {description} - Direct marketing without consent")]
    App7Violation {
        /// Description
        description: String,
        /// Whether opt-out was provided
        opt_out_provided: bool,
    },

    /// APP 8 - Cross-border disclosure
    #[error("APP 8 Violation: Cross-border disclosure to {country} without adequate protection")]
    App8Violation {
        /// Destination country
        country: String,
        /// Description
        description: String,
    },

    /// APP 9 - Government identifiers
    #[error("APP 9 Violation: Improper use of government identifier - {description}")]
    App9Violation {
        /// Description
        description: String,
        /// Identifier type (e.g., TFN, Medicare)
        identifier_type: String,
    },

    /// APP 10 - Quality of personal information
    #[error("APP 10 Violation: Information quality issues - {description}")]
    App10Violation {
        /// Description
        description: String,
    },

    /// APP 11 - Security of personal information
    #[error("APP 11 Violation: {description} - Security measures inadequate")]
    App11Violation {
        /// Description
        description: String,
        /// Security gaps identified
        security_gaps: Vec<String>,
    },

    /// APP 12 - Access to personal information
    #[error("APP 12 Violation: Access request improperly handled - {description}")]
    App12Violation {
        /// Description
        description: String,
        /// Request ID
        request_id: Option<String>,
        /// Days overdue
        days_overdue: Option<i64>,
    },

    /// APP 13 - Correction of personal information
    #[error("APP 13 Violation: Correction request improperly handled - {description}")]
    App13Violation {
        /// Description
        description: String,
        /// Request ID
        request_id: Option<String>,
    },

    /// Notifiable Data Breach - failure to notify
    #[error("NDB Violation: Eligible breach not notified - {description}")]
    BreachNotificationFailure {
        /// Description
        description: String,
        /// Breach ID
        breach_id: Option<String>,
        /// Days since breach discovered
        days_since_discovery: Option<i64>,
    },

    /// Notifiable Data Breach - late notification
    #[error("NDB Violation: Breach notification late - {days_late} days beyond assessment period")]
    LateBreachNotification {
        /// Breach ID
        breach_id: String,
        /// Days late
        days_late: i64,
    },

    /// Invalid consent
    #[error("Consent Violation: {description} - Consent invalid for {purpose}")]
    InvalidConsent {
        /// Description
        description: String,
        /// Purpose
        purpose: String,
        /// Reason invalid
        reason: ConsentInvalidReason,
    },

    /// Credit reporting violation
    #[error("Credit Reporting Violation (Part IIIA): {description}")]
    CreditReportingViolation {
        /// Description
        description: String,
        /// Section reference
        section: String,
    },

    /// Entity not covered
    #[error("Entity Exemption: {entity_name} - {exemption_reason}")]
    EntityExemption {
        /// Entity name
        entity_name: String,
        /// Exemption reason
        exemption_reason: String,
    },

    /// Validation error
    #[error("Validation Error: {field} - {message}")]
    ValidationError {
        /// Field
        field: String,
        /// Message
        message: String,
    },

    /// General privacy error
    #[error("Privacy Error: {message}")]
    General {
        /// Message
        message: String,
    },
}

/// Reason consent is invalid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsentInvalidReason {
    /// Consent was withdrawn
    Withdrawn,
    /// Consent has expired
    Expired,
    /// Wrong consent method for information type
    WrongMethod,
    /// Consent not specific enough
    NotSpecific,
    /// Consent obtained through deception
    Deceptive,
    /// Individual lacks capacity
    LacksCapacity,
}

impl PrivacyError {
    /// Create APP 3 violation for sensitive information
    pub fn sensitive_collection_without_consent(info_type: impl Into<String>) -> Self {
        PrivacyError::App3Violation {
            description: "Sensitive information collected without express consent".into(),
            information_type: info_type.into(),
            is_sensitive: true,
        }
    }

    /// Create APP 6 violation for secondary purpose
    pub fn unauthorised_secondary_use(
        primary: impl Into<String>,
        secondary: impl Into<String>,
    ) -> Self {
        PrivacyError::App6Violation {
            description: "Use for secondary purpose without consent or exception".into(),
            primary_purpose: primary.into(),
            secondary_purpose: Some(secondary.into()),
        }
    }

    /// Create APP 11 violation for security breach
    pub fn security_breach(gaps: Vec<String>) -> Self {
        PrivacyError::App11Violation {
            description: "Security measures inadequate to protect personal information".into(),
            security_gaps: gaps,
        }
    }

    /// Create NDB violation
    pub fn breach_not_notified(breach_id: impl Into<String>, days: i64) -> Self {
        PrivacyError::BreachNotificationFailure {
            description: "Eligible data breach not notified within assessment period".into(),
            breach_id: Some(breach_id.into()),
            days_since_discovery: Some(days),
        }
    }

    /// Get the relevant APP number if applicable
    pub fn app_number(&self) -> Option<u8> {
        match self {
            PrivacyError::App1Violation { .. } => Some(1),
            PrivacyError::App2Violation { .. } => Some(2),
            PrivacyError::App3Violation { .. } => Some(3),
            PrivacyError::App4Violation { .. } => Some(4),
            PrivacyError::App5Violation { .. } => Some(5),
            PrivacyError::App6Violation { .. } => Some(6),
            PrivacyError::App7Violation { .. } => Some(7),
            PrivacyError::App8Violation { .. } => Some(8),
            PrivacyError::App9Violation { .. } => Some(9),
            PrivacyError::App10Violation { .. } => Some(10),
            PrivacyError::App11Violation { .. } => Some(11),
            PrivacyError::App12Violation { .. } => Some(12),
            PrivacyError::App13Violation { .. } => Some(13),
            _ => None,
        }
    }

    /// Check if this is a serious/repeated interference
    pub fn is_serious_interference(&self) -> bool {
        matches!(
            self,
            PrivacyError::App3Violation {
                is_sensitive: true,
                ..
            } | PrivacyError::App11Violation { .. }
                | PrivacyError::BreachNotificationFailure { .. }
        )
    }

    /// Get maximum penalty description
    pub fn max_penalty_description(&self) -> String {
        if self.is_serious_interference() {
            "Serious interference: Individuals up to $2.5M; \
             Bodies corporate up to $50M, 3x benefit, or 30% turnover"
                .to_string()
        } else {
            "Civil penalty applicable - amount varies by provision".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_number() {
        let err = PrivacyError::App3Violation {
            description: "Test".into(),
            information_type: "Health".into(),
            is_sensitive: true,
        };
        assert_eq!(err.app_number(), Some(3));
    }

    #[test]
    fn test_serious_interference() {
        let err = PrivacyError::sensitive_collection_without_consent("Health");
        assert!(err.is_serious_interference());

        let err = PrivacyError::App2Violation {
            description: "Test".into(),
        };
        assert!(!err.is_serious_interference());
    }

    #[test]
    fn test_security_breach_error() {
        let err =
            PrivacyError::security_breach(vec!["No encryption".into(), "Weak passwords".into()]);

        match err {
            PrivacyError::App11Violation { security_gaps, .. } => {
                assert_eq!(security_gaps.len(), 2);
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_breach_notification_error() {
        let err = PrivacyError::breach_not_notified("breach-001", 45);

        match err {
            PrivacyError::BreachNotificationFailure {
                days_since_discovery,
                ..
            } => {
                assert_eq!(days_since_discovery, Some(45));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_penalty_description() {
        let serious = PrivacyError::security_breach(vec!["Test".into()]);
        assert!(serious.max_penalty_description().contains("$50M"));

        let minor = PrivacyError::App2Violation {
            description: "Test".into(),
        };
        assert!(minor.max_penalty_description().contains("Civil penalty"));
    }
}
