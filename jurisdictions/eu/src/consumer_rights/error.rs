//! Error types for Consumer Rights Directive compliance

use thiserror::Error;

/// Errors for Consumer Rights Directive compliance validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConsumerRightsError {
    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Missing required information under Article 6
    #[error("Missing required information: {requirement}")]
    MissingInformation { requirement: String },

    /// Invalid contract date
    #[error("Invalid contract date: {reason}")]
    InvalidContractDate { reason: String },

    /// Withdrawal period expired
    #[error("Withdrawal period expired on {deadline}")]
    WithdrawalPeriodExpired { deadline: String },

    /// Withdrawal right does not apply (exception under Article 17)
    #[error("Withdrawal right does not apply: {exception}")]
    WithdrawalExceptionApplies { exception: String },

    /// Invalid withdrawal notice
    #[error("Invalid withdrawal notice: {reason}")]
    InvalidWithdrawalNotice { reason: String },

    /// Missing standard withdrawal form
    #[error("Trader must provide model withdrawal form (Annex I(B))")]
    MissingWithdrawalForm,

    /// Invalid price
    #[error("Invalid price: {reason}")]
    InvalidPrice { reason: String },

    /// Contract type mismatch
    #[error("Contract type mismatch: expected {expected}, got {actual}")]
    ContractTypeMismatch { expected: String, actual: String },

    /// Multiple violations
    #[error("Multiple Consumer Rights Directive violations: {0:?}")]
    MultipleViolations(Vec<String>),
}

impl ConsumerRightsError {
    /// Create error for missing field
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// Create error for missing information
    pub fn missing_information(requirement: impl Into<String>) -> Self {
        Self::MissingInformation {
            requirement: requirement.into(),
        }
    }

    /// Create error for invalid contract date
    pub fn invalid_contract_date(reason: impl Into<String>) -> Self {
        Self::InvalidContractDate {
            reason: reason.into(),
        }
    }

    /// Create error for withdrawal exception
    pub fn withdrawal_exception(exception: impl Into<String>) -> Self {
        Self::WithdrawalExceptionApplies {
            exception: exception.into(),
        }
    }

    /// Create error for invalid withdrawal notice
    pub fn invalid_withdrawal_notice(reason: impl Into<String>) -> Self {
        Self::InvalidWithdrawalNotice {
            reason: reason.into(),
        }
    }

    /// Create error for invalid price
    pub fn invalid_price(reason: impl Into<String>) -> Self {
        Self::InvalidPrice {
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ConsumerRightsError::missing_field("trader");
        assert_eq!(err.to_string(), "Missing required field: trader");

        let err2 = ConsumerRightsError::missing_information("price");
        assert!(err2.to_string().contains("Missing required information"));
    }

    #[test]
    fn test_error_construction() {
        let err = ConsumerRightsError::invalid_price("Negative price not allowed");
        assert!(err.to_string().contains("Invalid price"));

        let err2 = ConsumerRightsError::withdrawal_exception("Perishable goods");
        assert!(err2.to_string().contains("Withdrawal right does not apply"));
    }
}
