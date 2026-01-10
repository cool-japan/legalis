//! Financial Services Errors (FSMA 2000, FCA Rules)

#![allow(missing_docs)]

use thiserror::Error;

/// Errors related to UK financial services regulation
#[derive(Debug, Clone, Error, PartialEq)]
pub enum FinancialServicesError {
    // ============================================================================
    // Authorization Errors (FSMA 2000)
    // ============================================================================
    /// Firm not authorized by FCA
    #[error(
        "Firm '{firm_name}' is not authorized by the FCA to conduct regulated activities. See FSMA 2000 s.19 (General prohibition). It is a criminal offence to carry on regulated activities without FCA authorization."
    )]
    NotAuthorized { firm_name: String },

    /// Authorization suspended
    #[error(
        "Firm authorization is suspended. Cannot conduct regulated activities while suspension is in effect. See FSMA 2000 s.45 (Variation etc. on FCA's own initiative)."
    )]
    AuthorizationSuspended,

    /// Activity not permitted under authorization
    #[error(
        "Activity '{activity}' is not permitted under current FCA authorization. Firm is only authorized for: {permitted_activities}. See FSMA 2000 Part 4A (Permission to carry on regulated activities)."
    )]
    ActivityNotPermitted {
        activity: String,
        permitted_activities: String,
    },

    // ============================================================================
    // FCA Principles for Businesses (PRIN)
    // ============================================================================
    /// Breach of Principle 1: Integrity
    #[error(
        "Breach of FCA Principle 1 (Integrity): {details}. A firm must conduct its business with integrity."
    )]
    BreachIntegrity { details: String },

    /// Breach of Principle 2: Skill, care and diligence
    #[error(
        "Breach of FCA Principle 2 (Skill, care and diligence): {details}. A firm must conduct its business with due skill, care and diligence."
    )]
    BreachSkillCare { details: String },

    /// Breach of Principle 6: Customers' interests
    #[error(
        "Breach of FCA Principle 6 (Customers' interests): {details}. A firm must pay due regard to the interests of its customers and treat them fairly."
    )]
    BreachCustomersInterests { details: String },

    /// Breach of Principle 7: Communications with clients
    #[error(
        "Breach of FCA Principle 7 (Communications): {details}. A firm must pay due regard to the information needs of its clients, and communicate information to them in a way which is clear, fair and not misleading."
    )]
    BreachCommunications { details: String },

    /// Breach of Principle 8: Conflicts of interest
    #[error(
        "Breach of FCA Principle 8 (Conflicts of interest): {details}. A firm must manage conflicts of interest fairly, both between itself and its customers and between a customer and another client."
    )]
    BreachConflictsOfInterest { details: String },

    /// Breach of Principle 10: Clients' assets
    #[error(
        "Breach of FCA Principle 10 (Clients' assets): {details}. A firm must arrange adequate protection for clients' assets when it is responsible for them."
    )]
    BreachClientAssets { details: String },

    /// Breach of Principle 11: Relations with regulators
    #[error(
        "Breach of FCA Principle 11 (Relations with regulators): {details}. A firm must deal with its regulators in an open and cooperative way, and must disclose to the FCA appropriately anything relating to the firm of which the FCA would reasonably expect notice."
    )]
    BreachRelationsWithRegulators { details: String },

    // ============================================================================
    // Client Categorization (COBS 3)
    // ============================================================================
    /// Incorrect client categorization
    #[error(
        "Client '{client_name}' incorrectly categorized as {actual_category:?}. Should be {correct_category:?}. See COBS 3 (Client categorization). Incorrect categorization may result in inadequate protection."
    )]
    IncorrectClientCategorization {
        client_name: String,
        actual_category: String,
        correct_category: String,
    },

    // ============================================================================
    // Suitability and Appropriateness (COBS 9, 10)
    // ============================================================================
    /// Unsuitable investment recommendation
    #[error(
        "Investment recommendation is UNSUITABLE for client: {reason}. See COBS 9 (Suitability). When providing personal recommendations or managing investments, firm must ensure suitability based on: (a) knowledge and experience, (b) financial situation, (c) investment objectives."
    )]
    UnsuitableRecommendation { reason: String },

    /// Insufficient information for suitability assessment
    #[error(
        "Insufficient information obtained to assess suitability. Missing: {missing_information}. See COBS 9.2 (Assessing suitability). Firm must obtain necessary information regarding client's knowledge/experience, financial situation, and investment objectives."
    )]
    InsufficientInformationForSuitability { missing_information: String },

    /// Inappropriate product
    #[error(
        "Product is inappropriate for client given their knowledge and experience: {reason}. See COBS 10 (Appropriateness). Firm must assess whether client has necessary knowledge and experience to understand risks."
    )]
    InappropriateProduct { reason: String },

    // ============================================================================
    // Client Assets (CASS)
    // ============================================================================
    /// Client money not segregated
    #[error(
        "Client money of £{amount} not properly segregated. See CASS 7 (Client money rules). Client money must be segregated from firm's own money and held in designated client bank accounts."
    )]
    ClientMoneyNotSegregated { amount: f64 },

    /// Client assets not protected
    #[error(
        "Client assets worth £{value} not adequately protected. {reason}. See CASS 6 (Custody rules). Firm must make adequate arrangements for safeguarding of client custody assets."
    )]
    ClientAssetsNotProtected { value: f64, reason: String },

    /// Missing CASS reconciliation
    #[error(
        "Daily client money reconciliation not performed for {days_missing} days. Last reconciliation: {last_date}. See CASS 7.15 (Internal client money reconciliation). Firms must perform daily reconciliation of client money."
    )]
    MissingCassReconciliation {
        days_missing: u32,
        last_date: String,
    },

    // ============================================================================
    // Financial Promotions (FSMA s.21)
    // ============================================================================
    /// Unapproved financial promotion
    #[error(
        "Financial promotion not approved by authorized person. See FSMA 2000 s.21 (Restrictions on financial promotion). Financial promotions must be approved by FCA-authorized person or fall within exemption."
    )]
    UnapprovedFinancialPromotion,

    /// Misleading financial promotion
    #[error(
        "Financial promotion is misleading: {reason}. See COBS 4 (Communicating with clients, including financial promotions). All communications must be fair, clear and not misleading."
    )]
    MisleadingPromotion { reason: String },

    /// Missing risk warning
    #[error(
        "Financial promotion missing required risk warning for {product_type}. See COBS 4.2 (Fair, clear and not misleading communications). Risk warnings must be prominent and adequate for product risk."
    )]
    MissingRiskWarning { product_type: String },

    // ============================================================================
    // Market Abuse (UK MAR)
    // ============================================================================
    /// Market abuse detected
    #[error(
        "Suspected market abuse: {abuse_type:?}. {details}. See Market Abuse Regulation (UK MAR). Market abuse is a criminal offence. Must be reported to FCA immediately."
    )]
    MarketAbuse { abuse_type: String, details: String },

    /// Failure to report suspicious transaction
    #[error(
        "Suspicious transaction not reported to FCA. See UK MAR Article 16 (Reporting of suspicious transactions and orders). Firms must report suspicious transactions without delay."
    )]
    FailureToReportSuspiciousTransaction,

    // ============================================================================
    // Best Execution (COBS 11)
    // ============================================================================
    /// Failure to achieve best execution
    #[error(
        "Failed to take all sufficient steps to obtain best execution: {reason}. See COBS 11.2 (Best execution). Firm must take all sufficient steps to obtain best possible result for clients."
    )]
    FailedBestExecution { reason: String },

    /// No best execution policy
    #[error(
        "Best execution policy not established or not provided to clients. See COBS 11.2.12 (Best execution policy). Firms must establish and implement best execution policy and provide summary to clients."
    )]
    NoBestExecutionPolicy,

    // ============================================================================
    // Senior Managers Regime (SM&CR)
    // ============================================================================
    /// Senior manager not approved
    #[error(
        "Senior manager '{name}' performing SMF{function_number} without FCA approval. See SM&CR. Senior managers must be approved by FCA before performing senior management functions."
    )]
    SeniorManagerNotApproved { name: String, function_number: u32 },

    /// Missing statement of responsibilities
    #[error(
        "Senior manager '{name}' missing statement of responsibilities. See SM&CR. Each senior manager must have clear statement of responsibilities describing their role."
    )]
    MissingStatementOfResponsibilities { name: String },

    // ============================================================================
    // General Errors
    // ============================================================================
    /// Validation error
    #[error("Financial services validation error: {message}")]
    ValidationError { message: String },

    /// Multiple errors
    #[error("Multiple financial services errors: {errors:?}")]
    MultipleErrors { errors: Vec<String> },
}

/// Result type for financial services operations
pub type Result<T> = std::result::Result<T, FinancialServicesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_contain_references() {
        let error = FinancialServicesError::NotAuthorized {
            firm_name: "Test Firm".to_string(),
        };
        assert!(error.to_string().contains("FSMA 2000"));
        assert!(error.to_string().contains("s.19"));

        let error = FinancialServicesError::UnsuitableRecommendation {
            reason: "Risk too high".to_string(),
        };
        assert!(error.to_string().contains("COBS 9"));
        assert!(error.to_string().contains("Suitability"));
    }

    #[test]
    fn test_principles_breach_errors() {
        let principles = vec![
            FinancialServicesError::BreachIntegrity {
                details: "test".to_string(),
            },
            FinancialServicesError::BreachSkillCare {
                details: "test".to_string(),
            },
            FinancialServicesError::BreachCustomersInterests {
                details: "test".to_string(),
            },
            FinancialServicesError::BreachCommunications {
                details: "test".to_string(),
            },
            FinancialServicesError::BreachConflictsOfInterest {
                details: "test".to_string(),
            },
            FinancialServicesError::BreachClientAssets {
                details: "test".to_string(),
            },
            FinancialServicesError::BreachRelationsWithRegulators {
                details: "test".to_string(),
            },
        ];

        // Check each principle error contains "Principle"
        for error in principles {
            assert!(error.to_string().contains("Principle"));
        }
    }
}
