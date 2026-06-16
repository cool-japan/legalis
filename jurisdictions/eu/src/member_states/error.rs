//! Error types for member-state GDPR implementation modeling.

use thiserror::Error;

/// Errors arising from member-state GDPR implementation modeling and validation.
///
/// These errors cover construction and validation of national GDPR implementations
/// (supervisory authorities, age of digital consent, national derogations) as well
/// as directive-transposition tracking.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MemberStateError {
    /// The requested member state has no national implementation registered.
    #[error("No national GDPR implementation registered for member state: {0}")]
    NoImplementation(String),

    /// Age of digital consent is outside the range permitted by Article 8(1) GDPR.
    ///
    /// Article 8(1) GDPR permits member states to lower the default of 16 years,
    /// but not below 13 years.
    #[error(
        "Age of digital consent {age} is invalid under Article 8(1) GDPR (must be between 13 and 16)"
    )]
    InvalidAgeOfConsent {
        /// The invalid age that was supplied.
        age: u8,
    },

    /// A required field was missing when constructing a national implementation.
    #[error("Missing required field for national implementation: {0}")]
    MissingField(String),

    /// The transposition record refers to a directive that is not an EU directive.
    #[error("Invalid transposition: {reason}")]
    InvalidTransposition {
        /// Human-readable explanation of why the transposition record is invalid.
        reason: String,
    },

    /// A national derogation references a GDPR opening clause that does not exist.
    #[error("Unknown GDPR opening clause referenced by derogation: {0}")]
    UnknownOpeningClause(String),

    /// Generic validation failure with a descriptive reason.
    #[error("Member-state validation failed: {reason}")]
    ValidationFailed {
        /// Human-readable validation failure reason.
        reason: String,
    },
}

impl MemberStateError {
    /// Construct a [`MemberStateError::NoImplementation`] error.
    pub fn no_implementation(state: impl Into<String>) -> Self {
        Self::NoImplementation(state.into())
    }

    /// Construct a [`MemberStateError::MissingField`] error.
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// Construct a [`MemberStateError::InvalidTransposition`] error.
    pub fn invalid_transposition(reason: impl Into<String>) -> Self {
        Self::InvalidTransposition {
            reason: reason.into(),
        }
    }

    /// Construct a [`MemberStateError::ValidationFailed`] error.
    pub fn validation_failed(reason: impl Into<String>) -> Self {
        Self::ValidationFailed {
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_implementation_display() {
        let err = MemberStateError::no_implementation("Luxembourg");
        assert_eq!(
            err.to_string(),
            "No national GDPR implementation registered for member state: Luxembourg"
        );
    }

    #[test]
    fn test_invalid_age_display() {
        let err = MemberStateError::InvalidAgeOfConsent { age: 12 };
        assert!(err.to_string().contains("12"));
        assert!(err.to_string().contains("Article 8(1)"));
    }

    #[test]
    fn test_constructors() {
        assert_eq!(
            MemberStateError::missing_field("authority"),
            MemberStateError::MissingField("authority".to_string())
        );
        assert!(matches!(
            MemberStateError::invalid_transposition("not a directive"),
            MemberStateError::InvalidTransposition { .. }
        ));
        assert!(matches!(
            MemberStateError::validation_failed("bad"),
            MemberStateError::ValidationFailed { .. }
        ));
    }
}
