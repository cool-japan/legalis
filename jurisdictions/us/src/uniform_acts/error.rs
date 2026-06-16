//! Errors produced by the uniform-act validators.
//!
//! The adoption *trackers* in this module are infallible (they answer queries
//! with `Option`/`bool`). The *validators*, however, check a concrete fact
//! pattern against a model act's substantive requirements and therefore need a
//! dedicated error type. Each variant cites the controlling section of the
//! relevant uniform act.

use thiserror::Error;

/// Errors raised when validating facts against a uniform act's requirements.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UniformActError {
    /// One or more requirements for the creation of a trust under
    /// Uniform Trust Code § 402 were not satisfied.
    #[error("Uniform Trust Code § 402 (creation of trust) not satisfied: {0}")]
    TrustCreation(String),

    /// A will failed the execution formalities of Uniform Probate Code
    /// §§ 2-501 / 2-502.
    #[error("Uniform Probate Code §§ 2-501/2-502 (will execution) not satisfied: {0}")]
    WillExecution(String),

    /// A limited liability company failed the formation requirements of the
    /// Revised Uniform Limited Liability Company Act § 201 (and § 108 / § 113).
    #[error("Uniform Limited Liability Company Act § 201 (formation) not satisfied: {0}")]
    LlcFormation(String),

    /// An arbitration agreement failed the validity requirements of the
    /// Revised Uniform Arbitration Act § 6.
    #[error("Revised Uniform Arbitration Act § 6 (validity) not satisfied: {0}")]
    ArbitrationAgreement(String),

    /// An electronic record or signature fell outside the legal-recognition
    /// rule of the Uniform Electronic Transactions Act §§ 5 / 7.
    #[error("Uniform Electronic Transactions Act §§ 5/7 (legal recognition) not satisfied: {0}")]
    ElectronicTransaction(String),
}

/// Result alias for uniform-act validation operations.
pub type Result<T> = std::result::Result<T, UniformActError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_cites_section() {
        let err = UniformActError::TrustCreation("no definite beneficiary".to_string());
        assert!(err.to_string().contains("§ 402"));

        let err = UniformActError::WillExecution("only one witness".to_string());
        assert!(err.to_string().contains("2-502"));

        let err = UniformActError::ArbitrationAgreement("not in a record".to_string());
        assert!(err.to_string().contains("§ 6"));

        let err = UniformActError::ElectronicTransaction("testamentary".to_string());
        assert!(err.to_string().contains("Electronic Transactions Act"));
    }
}
