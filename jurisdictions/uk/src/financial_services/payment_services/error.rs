//! Payment Services Errors (Payment Services Regulations 2017 - PSD2)

use thiserror::Error;

/// Errors related to UK payment services regulation (PSD2)
#[derive(Debug, Clone, Error, PartialEq)]
pub enum PaymentServicesError {
    // ============================================================================
    // Authorization Errors (PSR 2017 Part 2)
    // ============================================================================
    /// Payment institution not authorized by FCA
    #[error(
        "Payment institution '{institution_name}' is not authorized by the FCA. PSR 2017 Regulation 6 prohibits carrying on payment services business unless authorized as payment institution, registered as small payment institution, or exempt. Penalty: up to 2 years imprisonment and/or unlimited fine."
    )]
    NotAuthorized {
        /// Institution name
        institution_name: String,
    },

    /// Small Payment Institution exceeds volume threshold
    #[error(
        "Small Payment Institution '{institution_name}' exceeds monthly payment volume threshold. PSR 2017 Regulation 13(2) limits SPIs to average monthly payment transactions of €3 million over preceding 12 months. Institution must apply for full Authorized Payment Institution status."
    )]
    SpiVolumeExceeded {
        /// Institution name
        institution_name: String,

        /// Average monthly volume in EUR
        average_monthly_volume_eur: f64,
    },

    // ============================================================================
    // Strong Customer Authentication Errors (PSR 2017 Reg 67-68)
    // ============================================================================
    /// Strong Customer Authentication not performed
    #[error(
        "Strong Customer Authentication not performed for transaction '{transaction_id}' of £{amount_gbp:.2}. PSR 2017 Regulation 67 requires payment service providers to apply SCA when payer: (a) accesses payment account online, (b) initiates electronic payment transaction, (c) carries out any action through remote channel which may imply risk of payment fraud or other abuses."
    )]
    ScaNotPerformed {
        /// Transaction identifier
        transaction_id: String,

        /// Transaction amount in GBP
        amount_gbp: f64,
    },

    /// SCA non-compliant (insufficient factors)
    #[error(
        "Strong Customer Authentication non-compliant for user '{user_id}'. PSR 2017 Regulation 68(1) requires authentication based on two or more elements from different categories: (a) knowledge (something only user knows), (b) possession (something only user possesses), (c) inherence (something user is). Only {factors_present} factor(s) present."
    )]
    ScaNonCompliant {
        /// User identifier
        user_id: String,

        /// Number of factors present
        factors_present: usize,
    },

    /// SCA exemption not applicable
    #[error(
        "SCA exemption '{exemption_type}' not applicable for transaction '{transaction_id}'. {reason}. PSR 2017 Regulation 68 allows exemptions only in specific circumstances (low value payments, recurring transactions, trusted beneficiaries, etc.)."
    )]
    ScaExemptionNotApplicable {
        /// Transaction identifier
        transaction_id: String,

        /// Exemption type claimed
        exemption_type: String,

        /// Reason exemption not applicable
        reason: String,
    },

    // ============================================================================
    // Open Banking Errors (CMA Order 2017)
    // ============================================================================
    /// Open Banking consent not obtained
    #[error(
        "Open Banking consent not obtained from user '{user_id}' for {provider_type} access. PSR 2017 requires explicit consent for Third Party Providers to access payment accounts. Consent must specify: (a) type of payment information accessed, (b) accounts designated, (c) frequency of access."
    )]
    ConsentNotObtained {
        /// User identifier
        user_id: String,

        /// Provider type (AISP or PISP)
        provider_type: String,
    },

    /// Open Banking consent expired
    #[error(
        "Open Banking consent '{consent_id}' expired on {expiry_date}. PSR 2017 Regulation 67(3) limits AIS consent to maximum 90 days. User must provide fresh consent for continued account access."
    )]
    ConsentExpired {
        /// Consent identifier
        consent_id: String,

        /// Expiry date
        expiry_date: String,
    },

    /// Open Banking consent revoked
    #[error(
        "Open Banking consent '{consent_id}' has been revoked by user. PSR 2017 Regulation 69 gives users right to withdraw consent at any time. TPP must immediately cease accessing payment account."
    )]
    ConsentRevoked {
        /// Consent identifier
        consent_id: String,
    },

    /// Exceeds consent permissions
    #[error(
        "Third Party Provider attempted to access '{attempted_access}' which exceeds granted permissions for consent '{consent_id}'. Granted permissions: {granted_permissions}. PSR 2017 requires TPP access limited to explicitly consented services only."
    )]
    ExceedsConsentPermissions {
        /// Consent identifier
        consent_id: String,

        /// Attempted access type
        attempted_access: String,

        /// Permissions actually granted
        granted_permissions: String,
    },

    // ============================================================================
    // Safeguarding Errors (PSR 2017 Reg 20-22)
    // ============================================================================
    /// Client funds not safeguarded
    #[error(
        "Client funds of £{amount_gbp:.2} not safeguarded for '{institution_name}'. PSR 2017 Regulation 20 requires payment institutions to safeguard funds received from payment service users by: (a) segregation in separate account, or (b) insurance or comparable guarantee. Relevant funds must be safeguarded by end of business day following receipt."
    )]
    ClientFundsNotSafeguarded {
        /// Institution name
        institution_name: String,

        /// Amount not safeguarded in GBP
        amount_gbp: f64,
    },

    /// Client funds not segregated
    #[error(
        "Client funds of £{amount_gbp:.2} not segregated from firm's own funds. PSR 2017 Regulation 20(1)(a) requires funds safeguarded by segregation to be held in separate account that is clearly designated for safeguarding purposes and distinguished from any account used to hold funds belonging to payment institution."
    )]
    ClientFundsNotSegregated {
        /// Amount not segregated in GBP
        amount_gbp: f64,
    },

    /// Daily reconciliation not performed
    #[error(
        "Daily reconciliation of client funds not performed for '{institution_name}'. Last reconciliation: {last_reconciliation_date}. PSR 2017 Regulation 21 requires payment institutions to maintain records showing amounts safeguarded and perform daily reconciliation to ensure accuracy."
    )]
    DailyReconciliationNotPerformed {
        /// Institution name
        institution_name: String,

        /// Last reconciliation date
        last_reconciliation_date: String,
    },

    /// Safeguarding account not designated
    #[error(
        "Safeguarding account not properly designated for '{institution_name}'. PSR 2017 Regulation 20(1)(a) requires account to be clearly designated for safeguarding purposes. Account must be held at credit institution or Bank of England."
    )]
    SafeguardingAccountNotDesignated {
        /// Institution name
        institution_name: String,
    },

    // ============================================================================
    // Payment Execution Errors (PSR 2017 Reg 85-93)
    // ============================================================================
    /// Payment execution time exceeded
    #[error(
        "Payment execution time exceeded for transaction '{transaction_id}'. Payment initiated on {initiation_date}, should be received by {deadline_date}. PSR 2017 Regulation 85 requires payment orders executed by end of next business day (or for paper-initiated payments, by end of second business day) after receipt."
    )]
    ExecutionTimeExceeded {
        /// Transaction identifier
        transaction_id: String,

        /// Payment initiation date
        initiation_date: String,

        /// Deadline for receipt
        deadline_date: String,
    },

    /// Unauthorized payment transaction
    #[error(
        "Unauthorized payment transaction '{transaction_id}' of £{amount_gbp:.2}. PSR 2017 Regulation 77 makes payment service provider liable for unauthorized payment transactions unless payer acted fraudulently or failed with gross negligence to comply with security measures. Payer entitled to immediate refund."
    )]
    UnauthorizedTransaction {
        /// Transaction identifier
        transaction_id: String,

        /// Transaction amount in GBP
        amount_gbp: f64,
    },

    // ============================================================================
    // Information Requirements Errors (PSR 2017 Part 6)
    // ============================================================================
    /// Framework contract terms not provided
    #[error(
        "Framework contract terms not provided to payment service user '{user_id}'. PSR 2017 Regulation 44 requires payment service provider to provide or make available terms and conditions of framework contract on paper or durable medium before user bound by contract. Must include: fees, interest rates, execution times, dispute resolution procedures."
    )]
    FrameworkContractTermsNotProvided {
        /// User identifier
        user_id: String,
    },

    /// Transaction information not provided
    #[error(
        "Transaction information not provided for payment '{transaction_id}'. PSR 2017 Regulations 52-53 require payer's payment service provider to provide without delay after execution: (a) reference enabling identification, (b) amount in currency used, (c) amount of charges, (d) exchange rate if applicable, (e) debit value date."
    )]
    TransactionInfoNotProvided {
        /// Transaction identifier
        transaction_id: String,
    },

    // ============================================================================
    // General Validation Errors
    // ============================================================================
    /// Payment services validation error
    #[error("Payment services validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// Multiple payment services errors
    #[error("Multiple payment services errors detected: {count} errors")]
    MultipleErrors {
        /// Number of errors
        count: usize,

        /// Error details
        errors: Vec<String>,
    },
}

/// Result type for payment services operations
pub type Result<T> = std::result::Result<T, PaymentServicesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_contain_references() {
        // Test SCA error includes PSR 2017 Reg 67
        let error = PaymentServicesError::ScaNotPerformed {
            transaction_id: "TX001".to_string(),
            amount_gbp: 1000.0,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("PSR 2017"));
        assert!(error_msg.contains("Regulation 67"));

        // Test safeguarding error includes Reg 20
        let error = PaymentServicesError::ClientFundsNotSafeguarded {
            institution_name: "Test PI".to_string(),
            amount_gbp: 50000.0,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("PSR 2017"));
        assert!(error_msg.contains("Regulation 20"));

        // Test consent error
        let error = PaymentServicesError::ConsentExpired {
            consent_id: "CONSENT001".to_string(),
            expiry_date: "2024-04-01".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("PSR 2017"));
        assert!(error_msg.contains("90 days"));
    }

    #[test]
    fn test_sca_error_includes_factor_requirements() {
        let error = PaymentServicesError::ScaNonCompliant {
            user_id: "USER001".to_string(),
            factors_present: 1,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("two or more elements"));
        assert!(error_msg.contains("knowledge"));
        assert!(error_msg.contains("possession"));
        assert!(error_msg.contains("inherence"));
    }
}
