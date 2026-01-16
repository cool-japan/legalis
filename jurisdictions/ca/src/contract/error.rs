//! Canada Contract Law - Error Types
//!
//! Error types for contract law analysis.

#![allow(missing_docs)]

use std::fmt;

use crate::common::Province;

/// Errors in contract law analysis
#[derive(Debug, Clone)]
pub enum ContractError {
    /// Invalid formation
    InvalidFormation {
        reason: String,
        elements_missing: Vec<String>,
    },
    /// Province not specified
    ProvinceRequired,
    /// Invalid province for operation
    InvalidProvince {
        province: Province,
        operation: String,
        reason: String,
    },
    /// Missing required facts
    MissingFacts { required: Vec<String> },
    /// Invalid consideration (common law only)
    InvalidConsideration { reason: String },
    /// Capacity issue
    CapacityIssue { party: String, issue: String },
    /// Illegality issue
    IllegalityIssue { reason: String },
    /// Quebec civil law error
    QuebecCivilLawError { article: String, message: String },
    /// Statute interpretation error
    StatuteInterpretation {
        statute: String,
        section: String,
        message: String,
    },
    /// Damages calculation error
    DamagesCalculation { reason: String },
    /// Remoteness analysis error
    RemotenessError { reason: String },
    /// Mitigation error
    MitigationError { reason: String },
    /// Exclusion clause analysis error
    ExclusionClauseError { reason: String },
    /// General analysis error
    AnalysisError { context: String, message: String },
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormation {
                reason,
                elements_missing,
            } => {
                write!(
                    f,
                    "Invalid contract formation: {}. Missing elements: {}",
                    reason,
                    elements_missing.join(", ")
                )
            }
            Self::ProvinceRequired => {
                write!(f, "Province must be specified for contract analysis")
            }
            Self::InvalidProvince {
                province,
                operation,
                reason,
            } => {
                write!(
                    f,
                    "Province {:?} invalid for {}: {}",
                    province, operation, reason
                )
            }
            Self::MissingFacts { required } => {
                write!(f, "Missing required facts: {}", required.join(", "))
            }
            Self::InvalidConsideration { reason } => {
                write!(f, "Invalid consideration: {}", reason)
            }
            Self::CapacityIssue { party, issue } => {
                write!(f, "Capacity issue for {}: {}", party, issue)
            }
            Self::IllegalityIssue { reason } => {
                write!(f, "Contract illegality: {}", reason)
            }
            Self::QuebecCivilLawError { article, message } => {
                write!(f, "Quebec civil law error (CCQ {}): {}", article, message)
            }
            Self::StatuteInterpretation {
                statute,
                section,
                message,
            } => {
                write!(
                    f,
                    "Statute interpretation error ({} s.{}): {}",
                    statute, section, message
                )
            }
            Self::DamagesCalculation { reason } => {
                write!(f, "Damages calculation error: {}", reason)
            }
            Self::RemotenessError { reason } => {
                write!(f, "Remoteness analysis error: {}", reason)
            }
            Self::MitigationError { reason } => {
                write!(f, "Mitigation analysis error: {}", reason)
            }
            Self::ExclusionClauseError { reason } => {
                write!(f, "Exclusion clause analysis error: {}", reason)
            }
            Self::AnalysisError { context, message } => {
                write!(f, "Contract analysis error in {}: {}", context, message)
            }
        }
    }
}

impl std::error::Error for ContractError {}

/// Result type for contract operations
pub type ContractResult<T> = Result<T, ContractError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_formation_error() {
        let err = ContractError::InvalidFormation {
            reason: "No acceptance".to_string(),
            elements_missing: vec!["Acceptance".to_string(), "Consideration".to_string()],
        };
        let msg = err.to_string();
        assert!(msg.contains("No acceptance"));
        assert!(msg.contains("Acceptance"));
    }

    #[test]
    fn test_quebec_error() {
        let err = ContractError::QuebecCivilLawError {
            article: "art. 1385".to_string(),
            message: "Consent vitiated".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("CCQ"));
        assert!(msg.contains("1385"));
    }

    #[test]
    fn test_damages_error() {
        let err = ContractError::DamagesCalculation {
            reason: "Cannot determine quantum".to_string(),
        };
        assert!(err.to_string().contains("quantum"));
    }

    #[test]
    fn test_province_error() {
        let err = ContractError::InvalidProvince {
            province: Province::Quebec,
            operation: "consideration analysis".to_string(),
            reason: "Quebec uses civil law".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Quebec"));
        assert!(msg.contains("civil law"));
    }
}
