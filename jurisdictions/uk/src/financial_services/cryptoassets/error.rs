//! Cryptoassets Errors (FSMA 2023, FCA PS19/22, MLR 2017 Reg 14A)

use thiserror::Error;

/// Errors related to UK cryptoassets regulation
#[derive(Debug, Clone, Error, PartialEq)]
pub enum CryptoassetsError {
    // ============================================================================
    // Classification Errors (FCA PS19/22)
    // ============================================================================
    /// Security token not authorized
    #[error(
        "Security token '{token_name}' offered without FCA authorization. FCA PS19/22 Guidance states cryptoassets meeting definition of 'specified investment' under RAO 2001 (shares, debentures, units in collective investment scheme) require full FCA authorization under FSMA 2000 s.19. Offering security tokens without authorization: criminal offence, up to 2 years imprisonment and/or unlimited fine."
    )]
    SecurityTokenNotAuthorized {
        /// Token name
        token_name: String,
    },

    /// Prospectus required but not published
    #[error(
        "Prospectus not published for public offer of security token '{token_name}'. UK Prospectus Regulation requires prospectus approved by FCA when securities offered to public or admitted to trading on regulated market (unless exemption applies). Prospectus must contain: (a) information about issuer, (b) financial information, (c) risk factors, (d) terms of securities. Offering without prospectus: up to 2 years imprisonment and/or unlimited fine."
    )]
    ProspectusRequired {
        /// Token name
        token_name: String,
    },

    /// E-money token not authorized as EMI
    #[error(
        "E-money token '{token_name}' issued without e-money institution authorization. FCA PS19/22 states tokens meeting e-money definition (EMR 2011 Regulation 2) require EMI or credit institution authorization. E-money characteristics: (a) electronically stored monetary value, (b) issued on receipt of funds, (c) accepted by third parties, (d) redeemable at par. Issuing e-money without authorization: criminal offence under FSMA 2000 s.19."
    )]
    EMoneyTokenNotAuthorized {
        /// Token name
        token_name: String,
    },

    // ============================================================================
    // Stablecoin Errors (FSMA 2023 Part 5)
    // ============================================================================
    /// Stablecoin not fully reserved (< 1:1 backing)
    #[error(
        "Stablecoin '{token_name}' not fully reserved. Backing ratio: {backing_ratio:.2}% (minimum 100% required). FSMA 2023 Part 5 stablecoin regime requires fiat-backed stablecoins to maintain at least 1:1 reserve backing in high-quality liquid assets (cash, central bank reserves, short-term government bonds). Reserves must be segregated from issuer's own assets and held at authorized credit institution or Bank of England."
    )]
    StablecoinNotFullyReserved {
        /// Token name
        token_name: String,

        /// Backing ratio (percentage)
        backing_ratio: f64,
    },

    /// Stablecoin reserves not segregated
    #[error(
        "Stablecoin '{token_name}' reserves not segregated from issuer's assets. FSMA 2023 requires reserves backing stablecoins to be segregated and held on trust for token holders. Segregation protects token holders if issuer becomes insolvent. Reserves must not be used for issuer's business activities."
    )]
    StablecoinReservesNotSegregated {
        /// Token name
        token_name: String,
    },

    /// Stablecoin not audited
    #[error(
        "Stablecoin '{token_name}' reserves not independently audited. FSMA 2023 requires regular independent audits of reserve assets by qualified auditor. Audit must verify: (a) reserve assets exist, (b) reserves match tokens in circulation, (c) assets are of type and quality claimed. Audit reports must be published to token holders."
    )]
    StablecoinNotAudited {
        /// Token name
        token_name: String,
    },

    /// No redemption rights
    #[error(
        "Stablecoin '{token_name}' does not provide redemption rights. FSMA 2023 requires fiat-backed stablecoins to provide token holders with enforceable claim to underlying reserves. Token holders must be able to redeem tokens for fiat currency at par value (1:1) without excessive delay. Lack of redemption rights: token may not qualify as e-money or regulated stablecoin."
    )]
    NoRedemptionRights {
        /// Token name
        token_name: String,
    },

    /// Algorithmic stablecoin (high risk, may be banned)
    #[error(
        "Algorithmic stablecoin '{token_name}' presents significant consumer harm risk. FCA has warned algorithmic stablecoins (no reserve backing, rely on algorithms/arbitrage to maintain peg) are high-risk and prone to collapse (see TerraUSD/Luna collapse May 2022). UK may ban/heavily restrict algorithmic stablecoins under FSMA 2023. Investors should be aware of total loss risk."
    )]
    AlgorithmicStablecoin {
        /// Token name
        token_name: String,
    },

    // ============================================================================
    // Financial Promotion Errors (FSMA s.21, effective Oct 8, 2023)
    // ============================================================================
    /// Cryptoasset promotion not approved
    #[error(
        "Cryptoasset promotion not approved by FCA-authorized person. From October 8, 2023, FSMA 2000 s.21 financial promotions regime applies to cryptoassets. Promotions must be: (a) made by authorized person, OR (b) content approved by authorized person, OR (c) exemption applies. Promotion date: {promotion_date}. Unapproved promotions: up to 2 years imprisonment and/or unlimited fine."
    )]
    PromotionNotApproved {
        /// Promotion date
        promotion_date: String,
    },

    /// No risk warning in promotion
    #[error(
        "Cryptoasset promotion does not include required risk warnings. FCA Policy Statement PS23/6 (June 2023) requires cryptoasset promotions to include clear, prominent risk warnings: 'Don't invest unless you're prepared to lose all the money you invest. This is a high-risk investment and you are unlikely to be protected if something goes wrong.' Warning must be as prominent as any information about benefits."
    )]
    NoRiskWarning {
        /// Promotion content
        promotion_content: String,
    },

    /// Promotion not fair, clear and not misleading
    #[error(
        "Cryptoasset promotion fails 'fair, clear and not misleading' test. COBS 4.2.1R requires financial promotions to be fair, clear and not misleading. Common issues: (a) exaggerated returns, (b) downplaying risks, (c) unclear fee structures, (d) celebrity endorsements without disclosure, (e) fear of missing out (FOMO) tactics. FCA can ban firms from approving promotions and impose unlimited fines."
    )]
    PromotionMisleading {
        /// Promotion content
        promotion_content: String,

        /// Reason misleading
        reason: String,
    },

    // ============================================================================
    // AML/CTF Errors (MLR 2017 Reg 14A)
    // ============================================================================
    /// Cryptoasset exchange provider not registered with FCA
    #[error(
        "Cryptoasset exchange provider '{provider_name}' not registered with FCA. MLR 2017 Regulation 14A (inserted January 2020) requires cryptoasset exchange providers and custodian wallet providers to register with FCA. Registration requires: (a) fit and proper test, (b) AML/CTF systems and controls, (c) compliance officer, (d) appropriate systems for monitoring. Operating without registration: criminal offence, up to 2 years imprisonment and/or unlimited fine."
    )]
    ExchangeProviderNotRegistered {
        /// Provider name
        provider_name: String,
    },

    /// CDD not performed for crypto customer
    #[error(
        "Customer Due Diligence not performed for cryptoasset customer '{customer_name}'. MLR 2017 applies to cryptoasset exchange providers. Standard CDD required for all customers: (a) identify customer, (b) verify identity using reliable, independent source, (c) assess and verify purpose and nature of business relationship. Enhanced Due Diligence required for high-risk customers (PEPs, high-risk countries, large transactions)."
    )]
    CddNotPerformed {
        /// Customer name
        customer_name: String,
    },

    /// Travel Rule violated (transfer ≥£1,000)
    #[error(
        "Travel Rule violated for cryptoasset transfer of £{amount_gbp:.2}. MLR 2017 Regulation 14A requires cryptoasset exchange providers to obtain and transmit information on originator and beneficiary for transfers ≥£1,000 (FATF Recommendation 16). Information required: (a) originator name, (b) originator account/wallet address, (c) beneficiary name, (d) beneficiary account/wallet address. Travel Rule ensures crypto transfers traceable like bank transfers."
    )]
    TravelRuleViolation {
        /// Transfer amount in GBP
        amount_gbp: f64,
    },

    /// Sanctions screening not performed
    #[error(
        "Sanctions screening not performed for cryptoasset transaction involving '{entity_name}'. Cryptoasset exchange providers must screen customers and transactions against UK sanctions lists (OFSI), UN sanctions, and other relevant lists. Crypto commonly used for sanctions evasion. Firms must have automated sanctions screening systems. Breaching financial sanctions: up to 7 years imprisonment and/or unlimited fine."
    )]
    SanctionsScreeningNotPerformed {
        /// Entity name
        entity_name: String,
    },

    // ============================================================================
    // Tax Evasion / Money Laundering Risks
    // ============================================================================
    /// Suspected tax evasion via crypto
    #[error(
        "Suspected tax evasion via cryptoasset transaction involving '{customer_name}'. HMRC treats cryptoassets as property for tax purposes. Gains from cryptoassets subject to Capital Gains Tax. Income from crypto (mining, staking) subject to Income Tax. Common red flags: (a) large unexplained crypto purchases, (b) P2P trades avoiding exchanges, (c) use of privacy coins (Monero, Zcash), (d) offshore exchanges. Cryptoasset firms must file Suspicious Activity Reports (SARs) to NCA."
    )]
    SuspectedTaxEvasion {
        /// Customer name
        customer_name: String,
    },

    // ============================================================================
    // Market Abuse (UK MAR applies to security tokens)
    // ============================================================================
    /// Insider dealing in security token
    #[error(
        "Suspected insider dealing in security token '{token_name}'. UK MAR applies to security tokens traded on UK trading venues or for which prospectus published. Insider dealing: using inside information (specific, precise, non-public, price-sensitive) to trade. Example: Token issuer employees trading before major announcement. Criminal offence: up to 7 years imprisonment. FCA civil penalty: unlimited fines."
    )]
    InsiderDealingSecurityToken {
        /// Token name
        token_name: String,

        /// Transaction details
        details: String,
    },

    /// Market manipulation in cryptoasset
    #[error(
        "Suspected market manipulation in cryptoasset '{token_name}': {manipulation_type}. Common crypto manipulation tactics: (a) pump and dump schemes (coordinated buying then selling), (b) wash trading (self-dealing to inflate volume), (c) spoofing (fake orders to move price), (d) rug pulls (developer abandons project after raising funds). For security tokens: UK MAR applies, criminal offence. For utility tokens: common law fraud, FCA consumer protection powers."
    )]
    MarketManipulation {
        /// Token name
        token_name: String,

        /// Type of manipulation
        manipulation_type: String,
    },

    // ============================================================================
    // General Validation Errors
    // ============================================================================
    /// Cryptoassets validation error
    #[error("Cryptoassets validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// Multiple cryptoassets errors
    #[error("Multiple cryptoassets errors detected: {count} errors")]
    MultipleErrors {
        /// Number of errors
        count: usize,

        /// Error details
        errors: Vec<String>,
    },
}

/// Result type for cryptoassets operations
pub type Result<T> = std::result::Result<T, CryptoassetsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_token_error_includes_fsma_reference() {
        let error = CryptoassetsError::SecurityTokenNotAuthorized {
            token_name: "ShareToken".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("FSMA 2000 s.19"));
        assert!(error_msg.contains("FCA PS19/22"));
    }

    #[test]
    fn test_stablecoin_error_includes_fsma_2023() {
        let error = CryptoassetsError::StablecoinNotFullyReserved {
            token_name: "USDT".to_string(),
            backing_ratio: 85.0,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("FSMA 2023"));
        assert!(error_msg.contains("1:1 reserve"));
    }

    #[test]
    fn test_promotion_error_includes_oct_8_2023_date() {
        let error = CryptoassetsError::PromotionNotApproved {
            promotion_date: "2024-01-01".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("October 8, 2023"));
        assert!(error_msg.contains("FSMA 2000 s.21"));
    }

    #[test]
    fn test_mlr_error_includes_regulation_14a() {
        let error = CryptoassetsError::ExchangeProviderNotRegistered {
            provider_name: "Crypto Exchange Ltd".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("MLR 2017 Regulation 14A"));
        assert!(error_msg.contains("register with FCA"));
    }

    #[test]
    fn test_travel_rule_error_includes_threshold() {
        let error = CryptoassetsError::TravelRuleViolation { amount_gbp: 5000.0 };
        let error_msg = error.to_string();
        assert!(error_msg.contains("£1,000"));
        assert!(error_msg.contains("FATF Recommendation 16"));
    }

    #[test]
    fn test_algorithmic_stablecoin_error_mentions_terrausd() {
        let error = CryptoassetsError::AlgorithmicStablecoin {
            token_name: "AlgoStable".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("algorithmic"));
        assert!(error_msg.contains("TerraUSD"));
    }
}
