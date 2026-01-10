//! e-Gov Electronic Filing Error Types
//!
//! Provides error types for e-Gov XML/JSON parsing, validation, and filing operations.

use thiserror::Error;

/// Application status for state transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ApplicationStatus {
    /// Draft status (下書き)
    Draft,
    /// Submitted to government (提出済み)
    Submitted,
    /// Under review by agency (審査中)
    UnderReview,
    /// Accepted by agency (受理)
    Accepted,
    /// Rejected by agency (却下)
    Rejected,
    /// Requires revision (補正要求)
    RequiresRevision,
    /// Approved (承認)
    Approved,
    /// Withdrawn by applicant (取下げ)
    Withdrawn,
}

/// e-Gov filing and parsing errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum EgovError {
    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlParse(String),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonParse(String),

    /// XML serialization error
    #[error("XML serialization error: {0}")]
    XmlSerialize(String),

    /// JSON serialization error
    #[error("JSON serialization error: {0}")]
    JsonSerialize(String),

    /// Schema validation failed
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    /// Missing required field
    #[error("Missing required field: {field} (フィールド {field} は必須です)")]
    MissingRequiredField { field: String },

    /// Invalid field value
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidFieldValue { field: String, reason: String },

    /// Invalid status transition
    #[error("Invalid status transition: {from:?} -> {to:?} (無効な状態遷移)")]
    InvalidStatusTransition {
        from: ApplicationStatus,
        to: ApplicationStatus,
    },

    /// Unsupported format
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Invalid application ID format
    #[error("Invalid application ID: {0}")]
    InvalidApplicationId(String),

    /// Attachment error
    #[error("Attachment error: {0}")]
    AttachmentError(String),

    /// File size exceeds limit
    #[error("File size {actual_bytes} bytes exceeds limit of {limit_bytes} bytes")]
    FileSizeExceeded {
        actual_bytes: usize,
        limit_bytes: usize,
    },

    /// Unsupported file type
    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),

    /// Format conversion error
    #[error("Format conversion error: {from} to {to}: {reason}")]
    ConversionError {
        from: String,
        to: String,
        reason: String,
    },

    /// Validation failed with multiple errors
    #[error("Validation failed: {count} error(s)")]
    ValidationFailed { count: usize },

    /// Generic error
    #[error("e-Gov error: {0}")]
    Other(String),
}

/// Result type for e-Gov operations
pub type Result<T> = std::result::Result<T, EgovError>;

impl EgovError {
    /// Check if the error is related to parsing
    pub fn is_parse_error(&self) -> bool {
        matches!(
            self,
            EgovError::XmlParse(_) | EgovError::JsonParse(_) | EgovError::EncodingError(_)
        )
    }

    /// Check if the error is related to validation
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            EgovError::SchemaValidation(_)
                | EgovError::MissingRequiredField { .. }
                | EgovError::InvalidFieldValue { .. }
                | EgovError::ValidationFailed { .. }
        )
    }

    /// Check if the error is related to status transitions
    pub fn is_status_error(&self) -> bool {
        matches!(self, EgovError::InvalidStatusTransition { .. })
    }

    /// Check if the error is related to attachments
    pub fn is_attachment_error(&self) -> bool {
        matches!(
            self,
            EgovError::AttachmentError(_)
                | EgovError::FileSizeExceeded { .. }
                | EgovError::UnsupportedFileType(_)
        )
    }
}

impl ApplicationStatus {
    /// Check if a status transition is valid
    pub fn can_transition_to(&self, target: ApplicationStatus) -> bool {
        use ApplicationStatus::*;

        match (self, target) {
            // Draft can transition to Submitted or Withdrawn
            (Draft, Submitted) | (Draft, Withdrawn) => true,

            // Submitted can transition to UnderReview, Rejected, or Withdrawn
            (Submitted, UnderReview) | (Submitted, Rejected) | (Submitted, Withdrawn) => true,

            // UnderReview can transition to Accepted, Rejected, RequiresRevision, or Withdrawn
            (UnderReview, Accepted)
            | (UnderReview, Rejected)
            | (UnderReview, RequiresRevision)
            | (UnderReview, Withdrawn) => true,

            // RequiresRevision can transition back to Submitted or Withdrawn
            (RequiresRevision, Submitted) | (RequiresRevision, Withdrawn) => true,

            // Accepted can transition to Approved
            (Accepted, Approved) => true,

            // Same status is always allowed (no-op)
            _ if self == &target => true,

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Get the display name in Japanese
    pub fn display_ja(&self) -> &'static str {
        match self {
            Self::Draft => "下書き",
            Self::Submitted => "提出済み",
            Self::UnderReview => "審査中",
            Self::Accepted => "受理",
            Self::Rejected => "却下",
            Self::RequiresRevision => "補正要求",
            Self::Approved => "承認",
            Self::Withdrawn => "取下げ",
        }
    }

    /// Get the display name in English
    pub fn display_en(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Submitted => "Submitted",
            Self::UnderReview => "Under Review",
            Self::Accepted => "Accepted",
            Self::Rejected => "Rejected",
            Self::RequiresRevision => "Requires Revision",
            Self::Approved => "Approved",
            Self::Withdrawn => "Withdrawn",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_status_transitions() {
        // Valid transitions
        assert!(ApplicationStatus::Draft.can_transition_to(ApplicationStatus::Submitted));
        assert!(ApplicationStatus::Submitted.can_transition_to(ApplicationStatus::UnderReview));
        assert!(ApplicationStatus::UnderReview.can_transition_to(ApplicationStatus::Accepted));
        assert!(ApplicationStatus::Accepted.can_transition_to(ApplicationStatus::Approved));

        // Invalid transitions
        assert!(!ApplicationStatus::Draft.can_transition_to(ApplicationStatus::Approved));
        assert!(!ApplicationStatus::Approved.can_transition_to(ApplicationStatus::Draft));
        assert!(!ApplicationStatus::Rejected.can_transition_to(ApplicationStatus::Approved));
    }

    #[test]
    fn test_status_withdrawal() {
        // Can withdraw from most states
        assert!(ApplicationStatus::Draft.can_transition_to(ApplicationStatus::Withdrawn));
        assert!(ApplicationStatus::Submitted.can_transition_to(ApplicationStatus::Withdrawn));
        assert!(ApplicationStatus::UnderReview.can_transition_to(ApplicationStatus::Withdrawn));
        assert!(
            ApplicationStatus::RequiresRevision.can_transition_to(ApplicationStatus::Withdrawn)
        );

        // Cannot withdraw from terminal states
        assert!(!ApplicationStatus::Approved.can_transition_to(ApplicationStatus::Withdrawn));
        assert!(!ApplicationStatus::Rejected.can_transition_to(ApplicationStatus::Withdrawn));
    }

    #[test]
    fn test_error_type_checks() {
        let parse_err = EgovError::XmlParse("test error".into());
        assert!(parse_err.is_parse_error());
        assert!(!parse_err.is_validation_error());

        let validation_err = EgovError::SchemaValidation("test".into());
        assert!(validation_err.is_validation_error());
        assert!(!validation_err.is_parse_error());

        let status_err = EgovError::InvalidStatusTransition {
            from: ApplicationStatus::Draft,
            to: ApplicationStatus::Approved,
        };
        assert!(status_err.is_status_error());
        assert!(!status_err.is_validation_error());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(ApplicationStatus::Draft.display_ja(), "下書き");
        assert_eq!(ApplicationStatus::Draft.display_en(), "Draft");

        assert_eq!(ApplicationStatus::Submitted.display_ja(), "提出済み");
        assert_eq!(ApplicationStatus::Submitted.display_en(), "Submitted");
    }

    #[test]
    fn test_error_display() {
        let err = EgovError::MissingRequiredField {
            field: "applicant_name".into(),
        };
        assert!(err.to_string().contains("applicant_name"));
        assert!(err.to_string().contains("必須"));

        let err2 = EgovError::InvalidStatusTransition {
            from: ApplicationStatus::Draft,
            to: ApplicationStatus::Approved,
        };
        assert!(err2.to_string().contains("Draft"));
        assert!(err2.to_string().contains("Approved"));
    }
}
