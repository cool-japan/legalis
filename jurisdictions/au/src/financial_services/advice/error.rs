//! Financial Advice Errors

use thiserror::Error;

/// Errors related to financial advice
#[derive(Debug, Clone, Error, PartialEq)]
pub enum AdviceError {
    /// Best interests duty breach
    #[error(
        "Breach of best interests duty for client '{client_name}'. \
         Details: {details}. See Corporations Act 2001 s.961B."
    )]
    BestInterestsDutyBreach {
        client_name: String,
        details: String,
    },

    /// Safe harbour step not completed
    #[error("Safe harbour step not completed: {step}. See Corporations Act 2001 s.961B(2).")]
    SafeHarbourStepMissing { step: String },

    /// Priority rule breach
    #[error(
        "Breach of priority rule: Adviser failed to prioritize client's interests. \
         Details: {details}. See Corporations Act 2001 s.961J."
    )]
    PriorityRuleBreach { details: String },

    /// FSG not provided
    #[error(
        "Financial Services Guide not provided to retail client. \
         See Corporations Act 2001 s.941A."
    )]
    FsgNotProvided,

    /// FSG deficient
    #[error("Financial Services Guide deficient: {deficiency}. See Corporations Act 2001 s.942B.")]
    FsgDeficient { deficiency: String },

    /// PDS not provided
    #[error(
        "Product Disclosure Statement not provided to retail client. \
         See Corporations Act 2001 s.1012A."
    )]
    PdsNotProvided,

    /// PDS deficient
    #[error(
        "Product Disclosure Statement deficient: {deficiency}. \
         See Corporations Act 2001 s.1013C."
    )]
    PdsDeficient { deficiency: String },

    /// SOA not provided
    #[error(
        "Statement of Advice not provided after personal advice. \
         See Corporations Act 2001 s.946A."
    )]
    SoaNotProvided,

    /// SOA deficient
    #[error("Statement of Advice deficient: {deficiency}. See Corporations Act 2001 s.947B.")]
    SoaDeficient { deficiency: String },

    /// Conflicted remuneration received
    #[error(
        "Conflicted remuneration received: {description}. Amount: ${amount}. \
         See Corporations Act 2001 s.963E."
    )]
    ConflictedRemuneration { description: String, amount: f64 },

    /// Validation error
    #[error("Advice validation error: {message}")]
    ValidationError { message: String },
}

/// Result type
pub type Result<T> = std::result::Result<T, AdviceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let error = AdviceError::BestInterestsDutyBreach {
            client_name: "John".to_string(),
            details: "Failed to investigate".to_string(),
        };
        assert!(error.to_string().contains("s.961B"));

        let error = AdviceError::SoaNotProvided;
        assert!(error.to_string().contains("s.946A"));
    }
}
