//! AML/CTF Errors (AUSTRAC)

use thiserror::Error;

/// Errors related to AML/CTF compliance
#[derive(Debug, Clone, Error, PartialEq)]
pub enum AmlCtfError {
    /// Customer identification incomplete
    #[error(
        "Customer identification incomplete for '{customer_name}'. Missing: {missing}. \
         See AML/CTF Act 2006 s.31."
    )]
    IncompleteIdentification {
        customer_name: String,
        missing: String,
    },

    /// Identity not verified
    #[error(
        "Identity not verified for '{customer_name}'. \
         Must verify using reliable documents. See AML/CTF Rules Chapter 4."
    )]
    IdentityNotVerified { customer_name: String },

    /// Insufficient documentation
    #[error(
        "Insufficient identity documents for '{customer_name}'. \
         Provided: {documents_provided}. Required: {documents_required}. \
         See AML/CTF Rules Chapter 4."
    )]
    InsufficientDocuments {
        customer_name: String,
        documents_provided: String,
        documents_required: String,
    },

    /// Beneficial owner not identified
    #[error(
        "Beneficial owners not identified for entity '{entity_name}'. \
         Must identify all beneficial owners with >25% ownership or control. \
         See AML/CTF Act 2006 s.36."
    )]
    BeneficialOwnerNotIdentified { entity_name: String },

    /// PEP not screened
    #[error(
        "PEP screening not completed for '{customer_name}'. \
         Must determine if customer is a Politically Exposed Person. \
         See AML/CTF Rules Chapter 15."
    )]
    PepNotScreened { customer_name: String },

    /// EDD required but not applied
    #[error(
        "Enhanced due diligence required for '{customer_name}' but not applied. \
         Reason: {reason}. See AML/CTF Rules Chapter 15."
    )]
    EddRequired {
        customer_name: String,
        reason: String,
    },

    /// EDD incomplete
    #[error(
        "Enhanced due diligence incomplete for '{customer_name}'. \
         Missing: {missing}. See AML/CTF Rules Chapter 15."
    )]
    EddIncomplete {
        customer_name: String,
        missing: String,
    },

    /// No AML/CTF program
    #[error(
        "No AML/CTF program in place. Reporting entities must have Part A and Part B programs. \
         See AML/CTF Act 2006 Part 7."
    )]
    NoAmlCtfProgram,

    /// Program deficient
    #[error("AML/CTF program deficient: {deficiency}. See AML/CTF Rules Chapter 8.")]
    ProgramDeficient { deficiency: String },

    /// No MLRO appointed
    #[error(
        "No AML/CTF Compliance Officer (MLRO) appointed. \
         See AML/CTF Rules Chapter 8.4."
    )]
    NoMlro,

    /// Not registered with AUSTRAC
    #[error(
        "Not registered with AUSTRAC as a reporting entity. \
         See AML/CTF Act 2006 s.14K."
    )]
    NotRegistered,

    /// SMR not submitted
    #[error(
        "Suspicious matter report not submitted to AUSTRAC. \
         Subject: {subject}. Must submit within {days} business days. \
         See AML/CTF Act 2006 s.41-43."
    )]
    SmrNotSubmitted { subject: String, days: u32 },

    /// SMR late
    #[error(
        "Suspicious matter report submitted late. Required within 24 hours for terrorism financing \
         or 3 business days for other matters. See AML/CTF Act 2006 s.41."
    )]
    SmrLate,

    /// TTR not submitted
    #[error(
        "Threshold transaction report not submitted for ${amount} transaction. \
         Must report cash transactions $10,000+. See AML/CTF Act 2006 s.43."
    )]
    TtrNotSubmitted { amount: f64 },

    /// IFTI not submitted
    #[error(
        "International funds transfer instruction not submitted. \
         All international transfers must be reported. See AML/CTF Act 2006 s.45."
    )]
    IftiNotSubmitted,

    /// Sanctions match
    #[error(
        "Customer '{customer_name}' matches sanctions list: {list_name}. \
         Cannot proceed with transaction. See AML/CTF Act 2006."
    )]
    SanctionsMatch {
        customer_name: String,
        list_name: String,
    },

    /// Prohibited customer
    #[error(
        "Cannot onboard customer '{customer_name}'. Risk rating: Prohibited. \
         Reason: {reason}."
    )]
    ProhibitedCustomer {
        customer_name: String,
        reason: String,
    },

    /// Validation error
    #[error("AML/CTF validation error: {message}")]
    ValidationError { message: String },
}

/// Result type
pub type Result<T> = std::result::Result<T, AmlCtfError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let error = AmlCtfError::IncompleteIdentification {
            customer_name: "John Smith".to_string(),
            missing: "Date of birth".to_string(),
        };
        assert!(error.to_string().contains("AML/CTF Act 2006"));
        assert!(error.to_string().contains("s.31"));

        let error = AmlCtfError::NoMlro;
        assert!(error.to_string().contains("AML/CTF Compliance Officer"));
    }

    #[test]
    fn test_smr_error() {
        let error = AmlCtfError::SmrNotSubmitted {
            subject: "Suspicious Customer".to_string(),
            days: 3,
        };
        assert!(error.to_string().contains("Suspicious matter report"));
        assert!(error.to_string().contains("s.41"));
    }
}
