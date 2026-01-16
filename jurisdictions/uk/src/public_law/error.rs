//! UK Public Law - Error Types
//!
//! This module provides error types for UK public law analysis,
//! covering judicial review, human rights, and constitutional matters.

// Allow missing docs on enum variant struct fields
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Primary error type for UK public law operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PublicLawError {
    /// Error in judicial review analysis
    JudicialReview(JudicialReviewError),
    /// Error in human rights analysis
    HumanRights(HumanRightsError),
    /// Error in constitutional analysis
    Constitutional(ConstitutionalError),
    /// Invalid input data
    InvalidInput(String),
    /// Missing required information
    MissingInformation(String),
    /// Internal analysis error
    InternalError(String),
}

impl fmt::Display for PublicLawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JudicialReview(e) => write!(f, "Judicial review error: {e}"),
            Self::HumanRights(e) => write!(f, "Human rights error: {e}"),
            Self::Constitutional(e) => write!(f, "Constitutional error: {e}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::MissingInformation(msg) => write!(f, "Missing information: {msg}"),
            Self::InternalError(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for PublicLawError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::JudicialReview(e) => Some(e),
            Self::HumanRights(e) => Some(e),
            Self::Constitutional(e) => Some(e),
            _ => None,
        }
    }
}

// ============================================================================
// Judicial Review Errors
// ============================================================================

/// Errors in judicial review analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JudicialReviewError {
    /// Decision not amenable to judicial review
    NotAmenable { reason: String },
    /// No standing
    NoStanding { reason: String },
    /// Out of time
    OutOfTime { days_late: u32, reason: String },
    /// No arguable ground
    NoArguableGround { analysis: String },
    /// Remedy not available
    RemedyNotAvailable { remedy: String, reason: String },
    /// Alternative remedy must be pursued
    AlternativeRemedy { remedy: String },
    /// Permission refused
    PermissionRefused { reason: String },
    /// Academic/moot
    Academic { reason: String },
    /// Ouster clause issue
    OusterClause { statute: String, provision: String },
}

impl fmt::Display for JudicialReviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAmenable { reason } => {
                write!(f, "Decision not amenable to judicial review: {reason}")
            }
            Self::NoStanding { reason } => write!(f, "No standing: {reason}"),
            Self::OutOfTime { days_late, reason } => {
                write!(f, "Out of time by {days_late} days: {reason}")
            }
            Self::NoArguableGround { analysis } => {
                write!(f, "No arguable ground: {analysis}")
            }
            Self::RemedyNotAvailable { remedy, reason } => {
                write!(f, "Remedy {remedy} not available: {reason}")
            }
            Self::AlternativeRemedy { remedy } => {
                write!(f, "Must pursue alternative remedy: {remedy}")
            }
            Self::PermissionRefused { reason } => {
                write!(f, "Permission refused: {reason}")
            }
            Self::Academic { reason } => write!(f, "Claim academic: {reason}"),
            Self::OusterClause { statute, provision } => {
                write!(f, "Ouster clause in {statute} {provision}")
            }
        }
    }
}

impl std::error::Error for JudicialReviewError {}

// ============================================================================
// Human Rights Errors
// ============================================================================

/// Errors in human rights analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HumanRightsError {
    /// Not a public authority
    NotPublicAuthority { reason: String },
    /// No interference with right
    NoInterference { article: String, reason: String },
    /// Interference justified
    InterferenceJustified { justification: String },
    /// No victim status
    NoVictimStatus { reason: String },
    /// Article not applicable
    ArticleNotApplicable { article: String, reason: String },
    /// Out of HRA time limit (1 year)
    OutOfHraTime { days_late: u32 },
    /// Derogation in force
    DerogationInForce { article: String, derogation: String },
    /// Reservation applies
    ReservationApplies { reservation: String },
}

impl fmt::Display for HumanRightsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPublicAuthority { reason } => {
                write!(f, "Not a public authority: {reason}")
            }
            Self::NoInterference { article, reason } => {
                write!(f, "No interference with {article}: {reason}")
            }
            Self::InterferenceJustified { justification } => {
                write!(f, "Interference justified: {justification}")
            }
            Self::NoVictimStatus { reason } => {
                write!(f, "No victim status: {reason}")
            }
            Self::ArticleNotApplicable { article, reason } => {
                write!(f, "{article} not applicable: {reason}")
            }
            Self::OutOfHraTime { days_late } => {
                write!(f, "Out of HRA time limit by {days_late} days")
            }
            Self::DerogationInForce {
                article,
                derogation,
            } => {
                write!(f, "Derogation from {article}: {derogation}")
            }
            Self::ReservationApplies { reservation } => {
                write!(f, "Reservation applies: {reservation}")
            }
        }
    }
}

impl std::error::Error for HumanRightsError {}

// ============================================================================
// Constitutional Errors
// ============================================================================

/// Errors in constitutional analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstitutionalError {
    /// Parliament has sovereign power
    ParliamentarySovereignty { issue: String },
    /// Non-justiciable political question
    PoliticalQuestion { matter: String },
    /// Prerogative not reviewable
    PrerogativeNotReviewable { power: String, reason: String },
    /// Act of state doctrine
    ActOfState { context: String },
    /// Crown immunity
    CrownImmunity { context: String },
    /// Constitutional convention not enforceable
    ConventionNotEnforceable { convention: String },
}

impl fmt::Display for ConstitutionalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParliamentarySovereignty { issue } => {
                write!(f, "Parliamentary sovereignty prevents review: {issue}")
            }
            Self::PoliticalQuestion { matter } => {
                write!(f, "Non-justiciable political question: {matter}")
            }
            Self::PrerogativeNotReviewable { power, reason } => {
                write!(f, "Prerogative {power} not reviewable: {reason}")
            }
            Self::ActOfState { context } => {
                write!(f, "Act of state doctrine applies: {context}")
            }
            Self::CrownImmunity { context } => {
                write!(f, "Crown immunity applies: {context}")
            }
            Self::ConventionNotEnforceable { convention } => {
                write!(f, "Convention not legally enforceable: {convention}")
            }
        }
    }
}

impl std::error::Error for ConstitutionalError {}

// ============================================================================
// Conversion Traits
// ============================================================================

impl From<JudicialReviewError> for PublicLawError {
    fn from(err: JudicialReviewError) -> Self {
        Self::JudicialReview(err)
    }
}

impl From<HumanRightsError> for PublicLawError {
    fn from(err: HumanRightsError) -> Self {
        Self::HumanRights(err)
    }
}

impl From<ConstitutionalError> for PublicLawError {
    fn from(err: ConstitutionalError) -> Self {
        Self::Constitutional(err)
    }
}

// ============================================================================
// Result Type Alias
// ============================================================================

/// Result type for public law operations
pub type PublicLawResult<T> = Result<T, PublicLawError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_public_law_error_display() {
        let err = PublicLawError::InvalidInput("test input".into());
        assert!(err.to_string().contains("Invalid input"));
    }

    #[test]
    fn test_judicial_review_error() {
        let err = JudicialReviewError::NoStanding {
            reason: "Insufficient interest".into(),
        };
        assert!(err.to_string().contains("No standing"));
    }

    #[test]
    fn test_human_rights_error() {
        let err = HumanRightsError::NotPublicAuthority {
            reason: "Private company".into(),
        };
        assert!(err.to_string().contains("Not a public authority"));
    }

    #[test]
    fn test_constitutional_error() {
        let err = ConstitutionalError::PoliticalQuestion {
            matter: "Foreign policy".into(),
        };
        assert!(err.to_string().contains("political question"));
    }

    #[test]
    fn test_error_conversion() {
        let jr_err = JudicialReviewError::OutOfTime {
            days_late: 30,
            reason: "Delay".into(),
        };
        let public_err: PublicLawError = jr_err.into();
        assert!(matches!(public_err, PublicLawError::JudicialReview(_)));
    }

    #[test]
    fn test_error_source() {
        let inner = JudicialReviewError::Academic {
            reason: "No live issue".into(),
        };
        let outer = PublicLawError::JudicialReview(inner);
        assert!(outer.source().is_some());
    }
}
