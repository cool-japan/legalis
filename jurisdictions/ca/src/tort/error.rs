//! Canada Tort Law - Error Types
//!
//! Error types for tort law analysis.

#![allow(missing_docs)]

use std::fmt;

use crate::common::Province;

/// Errors in tort law analysis
#[derive(Debug, Clone)]
pub enum TortError {
    /// Province required
    ProvinceRequired,
    /// Invalid province for operation
    InvalidProvince {
        province: Province,
        operation: String,
        reason: String,
    },
    /// Missing required facts
    MissingFacts { required: Vec<String> },
    /// Invalid duty of care analysis
    DutyOfCareError { reason: String },
    /// Standard of care error
    StandardOfCareError { reason: String },
    /// Causation analysis error
    CausationError { reason: String },
    /// Remoteness analysis error
    RemotenessError { reason: String },
    /// Damages calculation error
    DamagesError { reason: String },
    /// Occupiers' liability error
    OccupiersLiabilityError { reason: String },
    /// Defamation analysis error
    DefamationError { reason: String },
    /// Nuisance analysis error
    NuisanceError { reason: String },
    /// General analysis error
    AnalysisError { context: String, message: String },
}

impl fmt::Display for TortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProvinceRequired => {
                write!(f, "Province must be specified for tort analysis")
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
            Self::DutyOfCareError { reason } => {
                write!(f, "Duty of care analysis error: {}", reason)
            }
            Self::StandardOfCareError { reason } => {
                write!(f, "Standard of care analysis error: {}", reason)
            }
            Self::CausationError { reason } => {
                write!(f, "Causation analysis error: {}", reason)
            }
            Self::RemotenessError { reason } => {
                write!(f, "Remoteness analysis error: {}", reason)
            }
            Self::DamagesError { reason } => {
                write!(f, "Damages calculation error: {}", reason)
            }
            Self::OccupiersLiabilityError { reason } => {
                write!(f, "Occupiers' liability error: {}", reason)
            }
            Self::DefamationError { reason } => {
                write!(f, "Defamation analysis error: {}", reason)
            }
            Self::NuisanceError { reason } => {
                write!(f, "Nuisance analysis error: {}", reason)
            }
            Self::AnalysisError { context, message } => {
                write!(f, "Tort analysis error in {}: {}", context, message)
            }
        }
    }
}

impl std::error::Error for TortError {}

/// Result type for tort operations
pub type TortResult<T> = Result<T, TortError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_province_required_error() {
        let err = TortError::ProvinceRequired;
        assert!(err.to_string().contains("Province"));
    }

    #[test]
    fn test_duty_of_care_error() {
        let err = TortError::DutyOfCareError {
            reason: "No proximity established".to_string(),
        };
        assert!(err.to_string().contains("proximity"));
    }

    #[test]
    fn test_causation_error() {
        let err = TortError::CausationError {
            reason: "But-for test not satisfied".to_string(),
        };
        assert!(err.to_string().contains("But-for"));
    }

    #[test]
    fn test_defamation_error() {
        let err = TortError::DefamationError {
            reason: "Statement not defamatory".to_string(),
        };
        assert!(err.to_string().contains("defamatory"));
    }
}
