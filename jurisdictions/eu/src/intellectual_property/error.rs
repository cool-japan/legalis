//! Error types for EU Intellectual Property law

use thiserror::Error;

/// Errors related to EU intellectual property law compliance and validation
#[derive(Debug, Error, Clone, PartialEq)]
pub enum IpError {
    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid trademark
    #[error("Invalid trademark: {reason}")]
    InvalidTrademark { reason: String },

    /// Invalid design
    #[error("Invalid design: {reason}")]
    InvalidDesign { reason: String },

    /// Copyright violation
    #[error("Copyright issue: {reason}")]
    CopyrightIssue { reason: String },

    /// Trade secret violation
    #[error("Trade secret issue: {reason}")]
    TradeSecretIssue { reason: String },

    /// Invalid Nice classification
    #[error("Invalid Nice class: {class}. Must be 1-45")]
    InvalidNiceClass { class: u8 },

    /// Distinctiveness requirement not met
    #[error("Mark lacks distinctiveness: {reason}")]
    LackOfDistinctiveness { reason: String },

    /// Similarity conflict
    #[error("Similarity conflict detected: {reason}")]
    SimilarityConflict { reason: String },

    /// Protection expired
    #[error("Protection expired: {reason}")]
    ProtectionExpired { reason: String },

    /// Invalid duration
    #[error("Invalid protection duration: {reason}")]
    InvalidDuration { reason: String },
}

impl IpError {
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    pub fn invalid_trademark(reason: impl Into<String>) -> Self {
        Self::InvalidTrademark {
            reason: reason.into(),
        }
    }

    pub fn invalid_design(reason: impl Into<String>) -> Self {
        Self::InvalidDesign {
            reason: reason.into(),
        }
    }

    pub fn copyright_issue(reason: impl Into<String>) -> Self {
        Self::CopyrightIssue {
            reason: reason.into(),
        }
    }
}
