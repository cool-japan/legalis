//! Canada Constitutional Law - Error Types

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Error type for constitutional law operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstitutionalError {
    /// Charter right not recognized
    UnknownCharterRight { description: String },
    /// Invalid section reference
    InvalidSectionReference { section: String },
    /// Division of powers analysis error
    DivisionError { reason: String },
    /// Oakes test analysis error
    OakesTestError { stage: String, reason: String },
    /// Missing required information
    MissingInformation { field: String },
    /// Invalid jurisdiction
    InvalidJurisdiction { reason: String },
}

impl fmt::Display for ConstitutionalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCharterRight { description } => {
                write!(f, "Unknown Charter right: {description}")
            }
            Self::InvalidSectionReference { section } => {
                write!(f, "Invalid section reference: {section}")
            }
            Self::DivisionError { reason } => {
                write!(f, "Division of powers error: {reason}")
            }
            Self::OakesTestError { stage, reason } => {
                write!(f, "Oakes test error at {stage}: {reason}")
            }
            Self::MissingInformation { field } => {
                write!(f, "Missing required information: {field}")
            }
            Self::InvalidJurisdiction { reason } => {
                write!(f, "Invalid jurisdiction: {reason}")
            }
        }
    }
}

impl std::error::Error for ConstitutionalError {}

/// Result type for constitutional operations
pub type ConstitutionalResult<T> = Result<T, ConstitutionalError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ConstitutionalError::UnknownCharterRight {
            description: "s.99".to_string(),
        };
        assert!(err.to_string().contains("s.99"));
    }
}
