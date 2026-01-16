//! UK Land Law - Error Types
//!
//! This module provides error types for UK land law analysis,
//! covering estates, interests, registration, and conveyancing.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Primary error type for UK land law operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LandLawError {
    /// Error in estate analysis
    Estate(EstateError),
    /// Error in interest/easement analysis
    Interest(InterestError),
    /// Error in registration
    Registration(RegistrationError),
    /// Error in conveyancing
    Conveyancing(ConveyancingError),
    /// Error in mortgage analysis
    Mortgage(MortgageError),
    /// Invalid input data
    InvalidInput(String),
    /// Missing required information
    MissingInformation(String),
    /// Internal analysis error
    InternalError(String),
}

impl fmt::Display for LandLawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Estate(e) => write!(f, "Estate error: {e}"),
            Self::Interest(e) => write!(f, "Interest error: {e}"),
            Self::Registration(e) => write!(f, "Registration error: {e}"),
            Self::Conveyancing(e) => write!(f, "Conveyancing error: {e}"),
            Self::Mortgage(e) => write!(f, "Mortgage error: {e}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::MissingInformation(msg) => write!(f, "Missing information: {msg}"),
            Self::InternalError(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for LandLawError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Estate(e) => Some(e),
            Self::Interest(e) => Some(e),
            Self::Registration(e) => Some(e),
            Self::Conveyancing(e) => Some(e),
            Self::Mortgage(e) => Some(e),
            _ => None,
        }
    }
}

// ============================================================================
// Estate Errors
// ============================================================================

/// Errors in estate analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EstateError {
    /// Invalid lease term (must be certain)
    UncertainTerm { reason: String },
    /// Lease exceeds maximum permissible term
    ExcessiveTerm { years: u32, maximum: u32 },
    /// Missing exclusive possession (licence not lease)
    NoExclusivePossession { reason: String },
    /// Invalid freehold arrangement
    InvalidFreehold { reason: String },
    /// Co-ownership issue
    CoOwnershipIssue { issue: String },
    /// Forfeiture issue
    Forfeiture { ground: String },
    /// LTA 1954 issue
    Lta1954Issue { issue: String },
}

impl fmt::Display for EstateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UncertainTerm { reason } => {
                write!(f, "Uncertain term (Lace v Chantler): {reason}")
            }
            Self::ExcessiveTerm { years, maximum } => {
                write!(f, "Lease term {years} years exceeds maximum {maximum}")
            }
            Self::NoExclusivePossession { reason } => {
                write!(f, "No exclusive possession (Street v Mountford): {reason}")
            }
            Self::InvalidFreehold { reason } => {
                write!(f, "Invalid freehold: {reason}")
            }
            Self::CoOwnershipIssue { issue } => {
                write!(f, "Co-ownership issue: {issue}")
            }
            Self::Forfeiture { ground } => {
                write!(f, "Forfeiture ground: {ground}")
            }
            Self::Lta1954Issue { issue } => {
                write!(f, "LTA 1954 issue: {issue}")
            }
        }
    }
}

impl std::error::Error for EstateError {}

// ============================================================================
// Interest Errors
// ============================================================================

/// Errors in interest/easement analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterestError {
    /// Easement requirements not met (Re Ellenborough Park)
    EasementRequirementsFailed { missing: String },
    /// No dominant tenement
    NoDominantTenement,
    /// Same owner of both tenements
    CommonOwnership,
    /// Does not accommodate dominant tenement
    NoAccommodation { reason: String },
    /// Not capable of forming subject matter of grant
    NotCapableOfGrant { reason: String },
    /// Prescription period not satisfied
    PrescriptionFailed {
        years_needed: u32,
        years_proved: u32,
    },
    /// Covenant burden does not run
    BurdenDoesNotRun { reason: String },
    /// Covenant benefit does not run
    BenefitDoesNotRun { reason: String },
    /// Not a restrictive covenant
    NotRestrictive,
    /// No building scheme
    NoBuildingScheme { missing_element: String },
}

impl fmt::Display for InterestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EasementRequirementsFailed { missing } => {
                write!(f, "Re Ellenborough Park requirements not met: {missing}")
            }
            Self::NoDominantTenement => {
                write!(f, "No dominant tenement - easement in gross not permitted")
            }
            Self::CommonOwnership => {
                write!(
                    f,
                    "Common ownership - dominant and servient must have different owners"
                )
            }
            Self::NoAccommodation { reason } => {
                write!(f, "Does not accommodate dominant tenement: {reason}")
            }
            Self::NotCapableOfGrant { reason } => {
                write!(
                    f,
                    "Not capable of forming subject matter of grant: {reason}"
                )
            }
            Self::PrescriptionFailed {
                years_needed,
                years_proved,
            } => {
                write!(
                    f,
                    "Prescription failed: need {years_needed} years, only proved {years_proved}"
                )
            }
            Self::BurdenDoesNotRun { reason } => {
                write!(f, "Burden does not run with land: {reason}")
            }
            Self::BenefitDoesNotRun { reason } => {
                write!(f, "Benefit does not run with land: {reason}")
            }
            Self::NotRestrictive => {
                write!(
                    f,
                    "Not a restrictive covenant - positive covenants do not bind successors"
                )
            }
            Self::NoBuildingScheme { missing_element } => {
                write!(f, "No building scheme: {missing_element}")
            }
        }
    }
}

impl std::error::Error for InterestError {}

// ============================================================================
// Registration Errors
// ============================================================================

/// Errors in land registration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistrationError {
    /// First registration deadline missed
    FirstRegistrationMissed { days_late: u32 },
    /// Disposition not completed by registration
    DispositionNotCompleted { disposition: String },
    /// Priority search expired
    PrioritySearchExpired { days_expired: u32 },
    /// Overriding interest dispute
    OverridingInterestDispute { interest: String },
    /// Alteration of register refused
    AlterationRefused { reason: String },
    /// Indemnity claim
    IndemnityIssue { issue: String },
    /// Adverse possession claim
    AdversePossession { issue: String },
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstRegistrationMissed { days_late } => {
                write!(
                    f,
                    "First registration deadline missed by {days_late} days - legal estate reverts to grantor"
                )
            }
            Self::DispositionNotCompleted { disposition } => {
                write!(
                    f,
                    "Disposition not completed by registration: {disposition}"
                )
            }
            Self::PrioritySearchExpired { days_expired } => {
                write!(f, "Priority search expired {days_expired} days ago")
            }
            Self::OverridingInterestDispute { interest } => {
                write!(f, "Overriding interest dispute: {interest}")
            }
            Self::AlterationRefused { reason } => {
                write!(f, "Alteration of register refused: {reason}")
            }
            Self::IndemnityIssue { issue } => {
                write!(f, "Indemnity issue: {issue}")
            }
            Self::AdversePossession { issue } => {
                write!(f, "Adverse possession: {issue}")
            }
        }
    }
}

impl std::error::Error for RegistrationError {}

// ============================================================================
// Conveyancing Errors
// ============================================================================

/// Errors in conveyancing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConveyancingError {
    /// Contract not complying with s.2 LP(MP)A 1989
    ContractFormalities { defect: String },
    /// Deed not complying with s.1 LP(MP)A 1989
    DeedFormalities { defect: String },
    /// Requisition on title
    RequisitionOnTitle { issue: String },
    /// Search revealed issue
    SearchIssue { search_type: String, issue: String },
    /// Completion delayed
    CompletionDelayed { reason: String },
    /// Chain issue
    ChainIssue { issue: String },
    /// Stamp Duty Land Tax issue
    SdltIssue { issue: String },
}

impl fmt::Display for ConveyancingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContractFormalities { defect } => {
                write!(
                    f,
                    "Contract does not comply with LP(MP)A 1989 s.2: {defect}"
                )
            }
            Self::DeedFormalities { defect } => {
                write!(f, "Deed does not comply with LP(MP)A 1989 s.1: {defect}")
            }
            Self::RequisitionOnTitle { issue } => {
                write!(f, "Requisition on title: {issue}")
            }
            Self::SearchIssue { search_type, issue } => {
                write!(f, "{search_type} search revealed: {issue}")
            }
            Self::CompletionDelayed { reason } => {
                write!(f, "Completion delayed: {reason}")
            }
            Self::ChainIssue { issue } => {
                write!(f, "Chain issue: {issue}")
            }
            Self::SdltIssue { issue } => {
                write!(f, "SDLT issue: {issue}")
            }
        }
    }
}

impl std::error::Error for ConveyancingError {}

// ============================================================================
// Mortgage Errors
// ============================================================================

/// Errors in mortgage analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MortgageError {
    /// Undue influence
    UndueInfluence { category: String },
    /// Misrepresentation to surety
    Misrepresentation,
    /// Priority dispute
    PriorityDispute { issue: String },
    /// Possession proceedings issue
    PossessionIssue { issue: String },
    /// Power of sale issue
    PowerOfSaleIssue { issue: String },
    /// Receiver appointment issue
    ReceiverIssue { issue: String },
}

impl fmt::Display for MortgageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndueInfluence { category } => {
                write!(f, "Undue influence ({category})")
            }
            Self::Misrepresentation => {
                write!(f, "Misrepresentation to surety")
            }
            Self::PriorityDispute { issue } => {
                write!(f, "Priority dispute: {issue}")
            }
            Self::PossessionIssue { issue } => {
                write!(f, "Possession issue: {issue}")
            }
            Self::PowerOfSaleIssue { issue } => {
                write!(f, "Power of sale issue: {issue}")
            }
            Self::ReceiverIssue { issue } => {
                write!(f, "Receiver issue: {issue}")
            }
        }
    }
}

impl std::error::Error for MortgageError {}

// ============================================================================
// Conversion Traits
// ============================================================================

impl From<EstateError> for LandLawError {
    fn from(err: EstateError) -> Self {
        Self::Estate(err)
    }
}

impl From<InterestError> for LandLawError {
    fn from(err: InterestError) -> Self {
        Self::Interest(err)
    }
}

impl From<RegistrationError> for LandLawError {
    fn from(err: RegistrationError) -> Self {
        Self::Registration(err)
    }
}

impl From<ConveyancingError> for LandLawError {
    fn from(err: ConveyancingError) -> Self {
        Self::Conveyancing(err)
    }
}

impl From<MortgageError> for LandLawError {
    fn from(err: MortgageError) -> Self {
        Self::Mortgage(err)
    }
}

// ============================================================================
// Result Type Alias
// ============================================================================

/// Result type for land law operations
pub type LandLawResult<T> = Result<T, LandLawError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_land_law_error_display() {
        let err = LandLawError::InvalidInput("test input".into());
        assert!(err.to_string().contains("Invalid input"));
    }

    #[test]
    fn test_estate_error() {
        let err = EstateError::UncertainTerm {
            reason: "No fixed end date".into(),
        };
        assert!(err.to_string().contains("Lace v Chantler"));
    }

    #[test]
    fn test_interest_error() {
        let err = InterestError::EasementRequirementsFailed {
            missing: "No dominant tenement".into(),
        };
        assert!(err.to_string().contains("Ellenborough"));
    }

    #[test]
    fn test_registration_error() {
        let err = RegistrationError::FirstRegistrationMissed { days_late: 30 };
        assert!(err.to_string().contains("30 days"));
        assert!(err.to_string().contains("reverts"));
    }

    #[test]
    fn test_conveyancing_error() {
        let err = ConveyancingError::ContractFormalities {
            defect: "Not signed by both parties".into(),
        };
        assert!(err.to_string().contains("LP(MP)A 1989"));
    }

    #[test]
    fn test_mortgage_error() {
        let err = MortgageError::UndueInfluence {
            category: "Class 2A - presumed".into(),
        };
        assert!(err.to_string().contains("Undue influence"));
    }

    #[test]
    fn test_error_conversion() {
        let estate_err = EstateError::NoExclusivePossession {
            reason: "Shared with others".into(),
        };
        let land_err: LandLawError = estate_err.into();
        assert!(matches!(land_err, LandLawError::Estate(_)));
    }

    #[test]
    fn test_error_source() {
        let inner = EstateError::Forfeiture {
            ground: "Non-payment of rent".into(),
        };
        let outer = LandLawError::Estate(inner);
        assert!(outer.source().is_some());
    }
}
