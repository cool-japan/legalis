//! Canada Criminal Law - Error Types
//!
//! Error types for Canadian criminal law analysis.

use thiserror::Error;

/// Criminal law error
#[derive(Debug, Error)]
pub enum CriminalError {
    /// Missing information
    #[error("Missing criminal law information: {0}")]
    MissingInformation(String),

    /// Invalid offence
    #[error("Invalid offence: {0}")]
    InvalidOffence(String),

    /// Defence not available
    #[error("Defence not available for this offence: {defence} for {offence}")]
    DefenceNotAvailable {
        /// Defence type
        defence: String,
        /// Offence
        offence: String,
    },

    /// Elements not established
    #[error("Elements not established: {element} for {offence}")]
    ElementsNotEstablished {
        /// Element type
        element: String,
        /// Offence
        offence: String,
    },

    /// Sentencing error
    #[error("Sentencing error: {0}")]
    SentencingError(String),

    /// Charter violation
    #[error("Charter violation in criminal process: {right}")]
    CharterViolation {
        /// Charter right
        right: String,
    },

    /// Procedure error
    #[error("Criminal procedure error: {0}")]
    ProcedureError(String),

    /// Evidence error
    #[error("Evidence error: {0}")]
    EvidenceError(String),
}

/// Result type for criminal law operations
pub type CriminalResult<T> = Result<T, CriminalError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CriminalError::MissingInformation("mens rea".to_string());
        assert!(error.to_string().contains("mens rea"));
    }

    #[test]
    fn test_defence_not_available() {
        let error = CriminalError::DefenceNotAvailable {
            defence: "duress".to_string(),
            offence: "murder".to_string(),
        };
        assert!(error.to_string().contains("duress"));
        assert!(error.to_string().contains("murder"));
    }

    #[test]
    fn test_charter_violation() {
        let error = CriminalError::CharterViolation {
            right: "s.10(b) right to counsel".to_string(),
        };
        assert!(error.to_string().contains("s.10(b)"));
    }
}
