//! MiFID II Errors (UK MiFID II, COBS, MAR)

use thiserror::Error;

/// Errors related to UK MiFID II regulation
#[derive(Debug, Clone, Error, PartialEq)]
pub enum Mifid2Error {
    // ============================================================================
    // Transaction Reporting Errors (MiFID II Article 26, FCA SUP 17)
    // ============================================================================
    /// Transaction not reported to FCA within T+1 deadline
    #[error(
        "Transaction '{transaction_id}' not reported to FCA within T+1 deadline. Executed on {transaction_date}, deadline: {deadline_date}. MiFID II Article 26(1) requires investment firms to report transactions in financial instruments to competent authority by end of following working day (T+1). FCA SUP 17 implements transaction reporting requirements."
    )]
    TransactionNotReported {
        /// Transaction identifier
        transaction_id: String,

        /// Transaction date
        transaction_date: String,

        /// Reporting deadline (T+1)
        deadline_date: String,
    },

    /// Transaction report incomplete (missing required fields)
    #[error(
        "Transaction report '{report_id}' incomplete. Missing required field: {missing_field}. MiFID II RTS 22 specifies 65 fields required for transaction reporting including: instrument identification (ISIN), buyer/seller identification (LEI), quantity, price, trading venue (MIC code), execution timestamp."
    )]
    TransactionReportIncomplete {
        /// Report identifier
        report_id: String,

        /// Missing field
        missing_field: String,
    },

    /// Invalid LEI (Legal Entity Identifier)
    #[error(
        "Invalid Legal Entity Identifier (LEI): '{lei}'. MiFID II transaction reporting requires 20-character LEI code issued by GLEIF (Global Legal Entity Identifier Foundation) for identifying counterparties. Format: [A-Z0-9]{{20}}."
    )]
    InvalidLei {
        /// Invalid LEI
        lei: String,
    },

    // ============================================================================
    // Product Governance Errors (COBS 16A, MiFID II Article 16(3))
    // ============================================================================
    /// Product not approved by product approval committee
    #[error(
        "Product '{product_name}' not approved by product approval committee. COBS 16A.1.5R requires manufacturers to establish, implement and review product approval process. Product governance arrangements must ensure products are designed to meet needs of identified target market and distribution strategy compatible with target market."
    )]
    ProductNotApproved {
        /// Product name
        product_name: String,
    },

    /// Target market not defined
    #[error(
        "Target market not defined for product '{product_name}'. COBS 16A.1.4R requires manufacturers to identify target market for each product. Target market must specify: (a) type of clients, (b) knowledge and experience, (c) financial situation with focus on ability to bear losses, (d) risk tolerance and compatibility, (e) objectives and needs."
    )]
    TargetMarketNotDefined {
        /// Product name
        product_name: String,
    },

    /// Product sold outside target market
    #[error(
        "Product '{product_name}' sold to client outside target market. Client profile: {client_profile}. Target market: {target_market}. COBS 16A.2.1R requires distributors to obtain target market information from manufacturers and ensure distribution strategy compatible with target market. Sales outside target market require documented justification."
    )]
    SoldOutsideTargetMarket {
        /// Product name
        product_name: String,

        /// Client profile
        client_profile: String,

        /// Target market definition
        target_market: String,
    },

    /// Distributors not notified of target market
    #[error(
        "Distributors not notified of target market for product '{product_name}'. COBS 16A.1.9R requires manufacturers to make available to distributors all appropriate information on financial instrument and product approval process, including identified target market. Information must enable distributors to understand and recommend/sell product to appropriate target market."
    )]
    DistributorsNotNotified {
        /// Product name
        product_name: String,
    },

    // ============================================================================
    // Research Unbundling Errors (MiFID II Article 24(8), COBS 2.3B)
    // ============================================================================
    /// Research payment bundled with execution
    #[error(
        "Research payment of £{amount_gbp:.2} bundled with execution services for '{research_provider}'. MiFID II Article 24(8) prohibits firms managing portfolios from accepting/receiving fees, commissions or monetary benefits from third parties in relation to investment services to clients (inducements ban). Investment research must be paid from firm's own resources or from separate research payment account funded by specific research charge to client."
    )]
    ResearchPaymentBundled {
        /// Research provider
        research_provider: String,

        /// Payment amount in GBP
        amount_gbp: f64,
    },

    /// Research budget not approved
    #[error(
        "Research budget not approved for payment of £{amount_gbp:.2} to '{research_provider}'. COBS 2.3B.5R requires firms to set and review total research budget at least annually. Research budget must be: (a) based on reasonable assessment of need for third-party research, (b) controlled by firm not portfolio managers, (c) not linked to volume/value of transactions."
    )]
    ResearchBudgetNotApproved {
        /// Research provider
        research_provider: String,

        /// Payment amount in GBP
        amount_gbp: f64,
    },

    /// Research payments not disclosed to clients
    #[error(
        "Research payments not disclosed to clients. COBS 2.3B.9R requires firms to provide clients with summary information about research payment account at least annually, including: (a) total costs paid from account, (b) proportion of research charges paid by each client/group of clients, (c) amount budgeted for research payments for coming year."
    )]
    ResearchPaymentsNotDisclosed {
        /// Firm name
        firm_name: String,
    },

    // ============================================================================
    // Best Execution Errors (COBS 11.2)
    // ============================================================================
    /// Best execution policy not established
    #[error(
        "Best execution policy not established for firm '{firm_name}'. COBS 11.2A.2R requires firms to establish and implement execution policy. Policy must include: (a) venues where firm places significant reliance, (b) factors affecting choice of venue, (c) how policy achieves best possible result. Firms must obtain prior consent from clients to execution policy."
    )]
    BestExecutionPolicyNotEstablished {
        /// Firm name
        firm_name: String,
    },

    /// Top 5 execution venues report not published
    #[error(
        "Top 5 execution venues report not published for period {period_start} to {period_end}. COBS 11.2A.28R requires investment firms to publish annually for each class of financial instruments: (a) top five execution venues by trading volume, (b) quality of execution obtained. Report must be published on firm's website by 30 April following year."
    )]
    Top5ReportNotPublished {
        /// Period start
        period_start: String,

        /// Period end
        period_end: String,
    },

    /// Best execution factors not considered
    #[error(
        "Best execution factors not sufficiently considered for order '{order_id}'. COBS 11.2A.7R requires firms when executing orders to take all sufficient steps to obtain best possible result taking into account: (a) price, (b) costs, (c) speed, (d) likelihood of execution and settlement, (e) size, (f) nature, (g) any other consideration relevant to execution."
    )]
    BestExecutionFactorsNotConsidered {
        /// Order identifier
        order_id: String,

        /// Factors not considered
        missing_factors: String,
    },

    // ============================================================================
    // Market Abuse Errors (UK MAR)
    // ============================================================================
    /// Suspicious transaction not reported to FCA
    #[error(
        "Suspicious transaction '{transaction_id}' not reported to FCA. UK MAR Article 16(2) requires persons professionally arranging or executing transactions to notify FCA without delay where they reasonably suspect transaction might constitute insider dealing or market manipulation. Firms must establish effective arrangements, systems and procedures to detect and report suspicious orders and transactions."
    )]
    SuspiciousTransactionNotReported {
        /// Transaction identifier
        transaction_id: String,

        /// Reason for suspicion
        reason: String,
    },

    /// Insider list not maintained
    #[error(
        "Insider list not maintained for inside information event '{event_description}'. UK MAR Article 18(1) requires issuers or any person acting on their behalf/account to draw up list of persons with access to inside information. Insider list must be promptly updated and submitted to FCA upon request. Failure to maintain insider list: administrative sanctions up to £5 million or 3% of annual turnover."
    )]
    InsiderListNotMaintained {
        /// Event description
        event_description: String,
    },

    // ============================================================================
    // Client Categorization Errors (COBS 3)
    // ============================================================================
    /// Client not categorized
    #[error(
        "Client '{client_name}' not categorized. COBS 3.2.1R requires firms to categorize clients as retail client, professional client, or eligible counterparty. Categorization determines level of regulatory protection: retail clients receive highest protection, eligible counterparties receive minimal protection. Firms must notify clients of their categorization."
    )]
    ClientNotCategorized {
        /// Client name
        client_name: String,
    },

    /// Retail client not given opt-up warning
    #[error(
        "Retail client '{client_name}' not given opt-up warning before treatment as professional client. COBS 3.5.3R requires firms requesting client be treated as professional client to give clear written warning of protections and compensation rights client may lose. Client must state in writing they are aware of consequences."
    )]
    OptUpWarningNotGiven {
        /// Client name
        client_name: String,
    },

    // ============================================================================
    // General Validation Errors
    // ============================================================================
    /// MiFID II validation error
    #[error("MiFID II validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// Multiple MiFID II errors
    #[error("Multiple MiFID II errors detected: {count} errors")]
    MultipleErrors {
        /// Number of errors
        count: usize,

        /// Error details
        errors: Vec<String>,
    },
}

/// Result type for MiFID II operations
pub type Result<T> = std::result::Result<T, Mifid2Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_reporting_error_includes_references() {
        let error = Mifid2Error::TransactionNotReported {
            transaction_id: "TX001".to_string(),
            transaction_date: "2024-01-01".to_string(),
            deadline_date: "2024-01-02".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("MiFID II Article 26"));
        assert!(error_msg.contains("T+1"));
        assert!(error_msg.contains("FCA SUP 17"));
    }

    #[test]
    fn test_product_governance_error_includes_cobs_reference() {
        let error = Mifid2Error::TargetMarketNotDefined {
            product_name: "High Risk Bond".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("COBS 16A"));
        assert!(error_msg.contains("target market"));
    }

    #[test]
    fn test_research_unbundling_error_includes_article_24() {
        let error = Mifid2Error::ResearchPaymentBundled {
            research_provider: "Research House Ltd".to_string(),
            amount_gbp: 10000.0,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("MiFID II Article 24(8)"));
        assert!(error_msg.contains("bundled"));
    }

    #[test]
    fn test_best_execution_error_includes_cobs_11() {
        let error = Mifid2Error::BestExecutionPolicyNotEstablished {
            firm_name: "Test Investment Firm".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("COBS 11.2A"));
        assert!(error_msg.contains("execution policy"));
    }

    #[test]
    fn test_market_abuse_error_includes_mar_reference() {
        let error = Mifid2Error::SuspiciousTransactionNotReported {
            transaction_id: "TX123".to_string(),
            reason: "Unusual volume before announcement".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("UK MAR Article 16"));
        assert!(error_msg.contains("suspicious"));
    }
}
