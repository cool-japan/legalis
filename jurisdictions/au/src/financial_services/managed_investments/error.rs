//! Managed Investments Errors

use thiserror::Error;

/// Errors related to managed investment schemes
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ManagedInvestmentsError {
    /// Scheme not registered
    #[error(
        "Managed investment scheme '{scheme_name}' is not registered with ASIC. \
         See Corporations Act 2001 s.601ED."
    )]
    SchemeNotRegistered { scheme_name: String },

    /// RE requirements not met
    #[error(
        "Responsible entity '{re_name}' does not meet requirements: {reason}. \
         See Corporations Act 2001 s.601FC."
    )]
    ReRequirementsNotMet { re_name: String, reason: String },

    /// Compliance plan deficient
    #[error("Compliance plan deficient: {deficiency}. See Corporations Act 2001 s.601HA.")]
    CompliancePlanDeficient { deficiency: String },

    /// Compliance plan not lodged
    #[error("Compliance plan not lodged with ASIC. See Corporations Act 2001 s.601HA.")]
    CompliancePlanNotLodged,

    /// No compliance committee
    #[error(
        "No compliance committee established (required if no external directors). \
         See Corporations Act 2001 s.601JA."
    )]
    NoComplianceCommittee,

    /// Compliance committee composition
    #[error(
        "Compliance committee does not have majority external members. \
         See Corporations Act 2001 s.601JB."
    )]
    ComplianceCommitteeComposition,

    /// Validation error
    #[error("Managed investments validation error: {message}")]
    ValidationError { message: String },
}

/// Result type
pub type Result<T> = std::result::Result<T, ManagedInvestmentsError>;
