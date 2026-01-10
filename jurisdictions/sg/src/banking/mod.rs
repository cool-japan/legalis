//! Banking Act (Cap. 19) - Singapore Banking Regulation
//!
//! This module provides comprehensive type-safe modeling of Singapore's banking regulations,
//! including the Banking Act (Cap. 19) and key MAS Notices:
//! - **MAS Notice 637**: Risk Based Capital Adequacy Requirements (Basel III)
//! - **MAS Notice 626**: Prevention of Money Laundering and Countering the Financing of Terrorism
//!
//! # Overview
//!
//! Singapore's banking sector is regulated by the **Monetary Authority of Singapore (MAS)**,
//! which oversees all banking institutions operating in the country. The framework includes:
//!
//! ## Banking Licenses
//!
//! Three types of banking licenses issued under Banking Act s. 4:
//!
//! 1. **Full Bank License**
//!    - Can accept deposits of any size
//!    - Full range of banking services
//!    - Examples: DBS, OCBC, UOB (local banks), HSBC, Citibank (foreign banks)
//!
//! 2. **Wholesale Bank License**
//!    - Minimum deposit: SGD 250,000
//!    - Serves corporate and high-net-worth clients
//!    - Restricted branch network
//!
//! 3. **Merchant Bank License**
//!    - Investment banking and corporate finance
//!    - **Cannot** accept retail deposits (s. 28)
//!    - Examples: Goldman Sachs, Morgan Stanley Singapore offices
//!
//! ## Basel III Capital Adequacy (MAS Notice 637)
//!
//! Singapore adopted Basel III standards with these minimum requirements:
//!
//! | Capital Type | Minimum Ratio | Description |
//! |--------------|---------------|-------------|
//! | **CET1** (Common Equity Tier 1) | 6.5% | Highest quality capital (includes 2.5% conservation buffer) |
//! | **Tier 1** (CET1 + AT1) | 8.0% | Core capital |
//! | **Total CAR** (Tier 1 + Tier 2) | 10.0% | Total capital adequacy ratio |
//!
//! ### Capital Components
//!
//! - **CET1**: Common shares, retained earnings, other comprehensive income
//! - **AT1**: Additional Tier 1 - perpetual preferred stock, contingent convertible bonds
//! - **Tier 2**: Subordinated debt (≥5 year maturity), hybrid instruments
//! - **RWA**: Risk-Weighted Assets - assets weighted by credit, market, operational risk
//!
//! ## AML/CFT Compliance (MAS Notice 626)
//!
//! Banks must implement robust Anti-Money Laundering (AML) and Countering the Financing of
//! Terrorism (CFT) controls:
//!
//! ### Customer Due Diligence (CDD)
//!
//! - **Standard CDD**: For all customers (identity verification, purpose of account)
//! - **Enhanced Due Diligence (EDD)**: Required for:
//!   - High-risk customers
//!   - Politically Exposed Persons (PEPs)
//!   - Cross-border correspondent banking
//!
//! ### CDD Review Frequency
//!
//! - **High Risk / PEPs**: Annual review
//! - **Medium Risk**: Every 2 years
//! - **Low Risk**: Every 3 years
//!
//! ### Reporting Obligations
//!
//! 1. **Suspicious Transaction Reports (STR)**: File with STRO "as soon as reasonably practicable"
//!    (generally 5-10 business days)
//! 2. **Cash Transaction Reports (CTR)**: Report cash transactions ≥ SGD 20,000
//! 3. **Cross-Border Movement**: Report physical currency movements ≥ SGD 20,000
//!
//! # Usage Examples
//!
//! ## Creating a Bank with Capital Adequacy
//!
//! ```rust
//! use legalis_sg::banking::*;
//! use chrono::Utc;
//!
//! // Create capital adequacy data
//! let capital = CapitalAdequacy {
//!     cet1_capital_sgd: 1_500_000_000_00,      // SGD 15M
//!     at1_capital_sgd: 300_000_000_00,         // SGD 3M
//!     tier2_capital_sgd: 500_000_000_00,       // SGD 5M
//!     risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
//!     calculation_date: Utc::now(),
//! };
//!
//! // CET1: 15M / 100M = 15% ✓ (> 6.5%)
//! // Tier 1: 18M / 100M = 18% ✓ (> 8.0%)
//! // Total: 23M / 100M = 23% ✓ (> 10.0%)
//! assert!(capital.meets_regulatory_minimum());
//!
//! // Create bank
//! let bank = Bank::new(
//!     "197700001E".to_string(),
//!     "Singapore Commercial Bank Ltd".to_string(),
//!     BankLicenseType::FullBank,
//!     Utc::now(),
//!     true, // Locally incorporated
//!     "Singapore".to_string(),
//!     capital,
//! );
//!
//! // Validate
//! let report = validate_bank(&bank)?;
//! if report.is_compliant {
//!     println!("✅ Bank is fully compliant");
//!     println!("CET1: {:.2}%", report.capital_status.cet1_ratio);
//! }
//! # Ok::<(), legalis_sg::banking::BankingError>(())
//! ```
//!
//! ## Customer Due Diligence (CDD)
//!
//! ```rust
//! use legalis_sg::banking::*;
//! use chrono::Utc;
//!
//! let customer = CustomerAccount {
//!     account_number: "ACC123456".to_string(),
//!     customer_name: "John Tan".to_string(),
//!     customer_id: "S1234567A".to_string(),
//!     risk_category: CustomerRiskCategory::High,
//!     account_opened: Utc::now(),
//!     last_cdd_review: Utc::now(),
//!     edd_performed: true, // Required for high risk
//!     source_of_funds_verified: true,
//!     beneficial_owner_identified: true,
//!     balance_sgd: 100_000_00, // SGD 1,000
//! };
//!
//! match validate_customer_account(&customer) {
//!     Ok(warnings) => {
//!         println!("✅ Customer account compliant");
//!         for warning in warnings {
//!             println!("⚠️  {}", warning);
//!         }
//!     }
//!     Err(e) => println!("❌ {}", e),
//! }
//! # Ok::<(), legalis_sg::banking::BankingError>(())
//! ```
//!
//! ## Suspicious Transaction Reporting
//!
//! ```rust
//! use legalis_sg::banking::*;
//! use chrono::Utc;
//!
//! let str_report = SuspiciousTransactionReport {
//!     reference_number: "STR2024001234".to_string(),
//!     account_number: "ACC123456".to_string(),
//!     customer_name: "Suspicious Entity Pte Ltd".to_string(),
//!     transaction_amount_sgd: 500_000_00, // SGD 5,000
//!     transaction_date: Utc::now(),
//!     filing_date: Utc::now(),
//!     suspicion_description: "Structured transactions to avoid reporting threshold".to_string(),
//!     transaction_proceeded: false,
//! };
//!
//! if str_report.filed_timely() {
//!     println!("✅ STR filed within recommended timeframe");
//! }
//! ```
//!
//! # Regulatory Framework
//!
//! ## Key Statutes
//!
//! - **Banking Act (Cap. 19)**: <https://sso.agc.gov.sg/Act/BA1970>
//! - **MAS Act (Cap. 186)**: <https://sso.agc.gov.sg/Act/MASA1970>
//!
//! ## MAS Notices
//!
//! - **MAS Notice 637**: Risk Based Capital Adequacy Requirements
//! - **MAS Notice 626**: Prevention of Money Laundering and Countering the Financing of Terrorism
//! - **MAS Notice 610**: Submission of Statistics and Returns
//!
//! ## International Standards
//!
//! - **Basel III**: Capital adequacy framework adopted by MAS
//! - **FATF Recommendations**: Financial Action Task Force standards for AML/CFT
//!
//! # See Also
//!
//! - [`payment`](../payment/index.html) - Payment Services Act 2019
//! - [`pdpa`](../pdpa/index.html) - Personal Data Protection Act (data privacy in banking)
//!
//! # References
//!
//! - MAS Banking Regulation: <https://www.mas.gov.sg/regulation/banking>
//! - Basel III Framework: <https://www.bis.org/bcbs/basel3.htm>
//! - FATF Recommendations: <https://www.fatf-gafi.org/>

pub mod error;
pub mod types;
pub mod validator;

// Re-export key types for convenience
pub use error::{BankingError, ErrorSeverity, Result};
pub use types::{
    Bank, BankBuilder, BankLicenseType, CapitalAdequacy, CapitalType, CashTransactionReport,
    CashTransactionType, ComplianceOfficer, CustomerAccount, CustomerRiskCategory, LicenseStatus,
    SuspiciousTransactionReport,
};
pub use validator::{
    AmlComplianceStatus, BankValidationReport, CapitalAdequacyStatus, assess_aml_compliance,
    calculate_capital_shortfall, calculate_required_capital, validate_bank,
    validate_cash_transaction, validate_customer_account, validate_merchant_bank_activities,
    validate_str_filing, validate_wholesale_deposit,
};
