//! Canada Employment Law - Error Types
//!
//! Error types for Canadian employment law analysis.

use thiserror::Error;

use crate::common::Province;

/// Employment law error
#[derive(Debug, Error)]
pub enum EmploymentError {
    /// Missing employment information
    #[error("Missing employment information: {0}")]
    MissingInformation(String),

    /// Invalid jurisdiction
    #[error("Invalid employment jurisdiction: {0}")]
    InvalidJurisdiction(String),

    /// Federal jurisdiction required
    #[error("Federal jurisdiction required for this industry: {0}")]
    FederalJurisdictionRequired(String),

    /// Provincial jurisdiction required
    #[error("Provincial jurisdiction required: {province}")]
    ProvincialJurisdictionRequired {
        /// Province required
        province: Province,
    },

    /// Employment status unclear
    #[error("Employment status unclear - unable to classify: {0}")]
    StatusUnclear(String),

    /// No valid termination grounds
    #[error("No valid termination grounds established: {0}")]
    NoTerminationGrounds(String),

    /// Just cause not established
    #[error("Just cause not established: {0}")]
    JustCauseNotEstablished(String),

    /// Insufficient progressive discipline
    #[error("Insufficient progressive discipline for termination: {0}")]
    InsufficientDiscipline(String),

    /// Human rights violation
    #[error("Potential human rights violation: {ground}")]
    HumanRightsViolation {
        /// Protected ground
        ground: String,
    },

    /// Accommodation failure
    #[error("Failure to accommodate: {0}")]
    AccommodationFailure(String),

    /// Invalid notice period
    #[error("Invalid notice period: {0}")]
    InvalidNoticePeriod(String),

    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),

    /// Statute not applicable
    #[error("Statute not applicable: {0}")]
    StatuteNotApplicable(String),

    /// Time limit exceeded
    #[error("Time limit exceeded for {action}: {days} days elapsed")]
    TimeLimitExceeded {
        /// Action that is time-limited
        action: String,
        /// Days elapsed
        days: u32,
    },
}

/// Result type for employment law operations
pub type EmploymentResult<T> = Result<T, EmploymentError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = EmploymentError::MissingInformation("salary data".to_string());
        assert!(error.to_string().contains("salary data"));
    }

    #[test]
    fn test_human_rights_error() {
        let error = EmploymentError::HumanRightsViolation {
            ground: "disability".to_string(),
        };
        assert!(error.to_string().contains("disability"));
    }

    #[test]
    fn test_time_limit_error() {
        let error = EmploymentError::TimeLimitExceeded {
            action: "filing complaint".to_string(),
            days: 365,
        };
        assert!(error.to_string().contains("365"));
    }
}
