//! Banking & Financial Services Law (ກົດໝາຍທະນາຄານແລະການບໍລິການທາງການເງິນ)
//!
//! This module implements Lao banking and financial services law provisions, including:
//! - **Commercial Bank Law 2006** (Law No. 03/NA, amended 2018)
//! - **Bank of Lao PDR Law 2018** (Law No. 50/NA)
//! - **AML/CFT Law 2014** (Law No. 50/NA)
//! - **Payment Systems Decree**
//!
//! ## Banking System Overview
//!
//! The Bank of Lao PDR (BOL - ທະນາຄານແຫ່ງ ສປປ ລາວ) serves as the central bank
//! and primary regulator of the Lao banking sector. All commercial banks must
//! obtain licenses from BOL and comply with capital adequacy, prudential
//! regulations, and AML/CFT requirements.
//!
//! ## Bank Types in Lao PDR
//!
//! ### State-Owned Banks (ທະນາຄານລັດ)
//! - **BCEL** (Banque pour le Commerce Exterieur Lao)
//! - **LDB** (Lao Development Bank)
//! - **APB** (Agricultural Promotion Bank)
//! - **Nayoby Bank** (Policy Bank)
//!
//! ### Joint Venture Banks (ທະນາຄານຮ່ວມທຶນ)
//! - Lao-Viet Bank (joint with Vietnam)
//! - BIC Bank (joint with various partners)
//!
//! ### Foreign Bank Branches (ສາຂາທະນາຄານຕ່າງປະເທດ)
//! - Thai banks (Bangkok Bank, Siam Commercial Bank, etc.)
//! - Vietnamese banks (BIDV, Vietinbank)
//! - Chinese banks (ICBC, Bank of China)
//!
//! ### Microfinance Institutions (ສະຖາບັນການເງິນຈຸລະພາກ)
//! - Deposit-taking MFIs
//! - Non-deposit MFIs
//! - Credit cooperatives
//! - Village funds
//!
//! ## Minimum Capital Requirements
//!
//! | Bank Type | Minimum Capital (LAK) |
//! |-----------|----------------------|
//! | Commercial Bank | 300 billion |
//! | Foreign Bank Branch | 50 billion |
//! | Deposit-taking MFI | 10 billion |
//! | Non-deposit MFI | 500 million |
//!
//! ## Capital Adequacy Requirements (Basel III)
//!
//! Lao banks must comply with Basel III capital adequacy requirements:
//!
//! - **Capital Adequacy Ratio (CAR)**: Minimum 8%
//! - **Tier 1 Capital Ratio**: Minimum 6%
//! - **Common Equity Tier 1 (CET1)**: Minimum 4.5%
//! - **Leverage Ratio**: Minimum 3%
//!
//! ## Prudential Regulations
//!
//! ### Large Exposure Limits
//! - **Single borrower limit**: 25% of capital
//! - **Related party lending limit**: 15% of capital
//!
//! ### Liquidity Requirements
//! - **Liquidity Coverage Ratio (LCR)**: Minimum 100%
//! - **Net Stable Funding Ratio (NSFR)**: Minimum 100%
//!
//! ## Deposit Protection
//!
//! The Deposit Protection Fund (DPF) provides insurance for eligible deposits:
//!
//! - **Coverage limit**: 50 million LAK per depositor per bank
//! - **Covered deposits**: Savings, current accounts, fixed deposits (LAK)
//! - **Excluded deposits**: Foreign currency deposits, interbank deposits
//!
//! ## Anti-Money Laundering (AML/CFT)
//!
//! Key AML requirements under Law No. 50/NA (2014):
//!
//! - **Customer Due Diligence (CDD)**: Identity verification, address verification
//! - **Enhanced Due Diligence (EDD)**: Required for PEPs and high-risk customers
//! - **STR Reporting**: Within 24 hours to FIU
//! - **Record Keeping**: Minimum 5 years
//! - **Sanctions Screening**: Against UN and domestic lists
//!
//! ## Interest Rate Regulations
//!
//! - **BOL Reference Rate**: Set by central bank monetary policy
//! - **Maximum Lending Rate**: Generally BOL rate + spread (approx. 15-18%)
//! - **Deposit Rate Floors**: May be set by BOL
//! - **Usury Prevention**: Rates exceeding certain thresholds prohibited
//!
//! ## Payment Systems
//!
//! BOL oversees payment systems including:
//!
//! - **RTGS** (Real-Time Gross Settlement): For large value payments
//! - **ACH** (Automated Clearing House): For retail payments
//! - **Mobile Banking**: Regulated by BOL
//! - **QR Payments**: LAOQR standard
//!
//! ## Foreign Exchange Regulations
//!
//! - **Exchange Rate Management**: Managed float against USD basket
//! - **Foreign Currency Accounts**: Permitted with restrictions
//! - **Capital Controls**: Some restrictions on capital flows
//! - **BOL Reference Rate**: Published daily
//!
//! ## Example Usage
//!
//! ```rust
//! use legalis_la::banking_law::{
//!     BankType, BankingLicense, LicenseStatus, BankingActivity,
//!     CapitalAdequacyReport, Tier1Capital, Tier2Capital, RiskWeightedAssets,
//!     validate_minimum_capital, validate_capital_adequacy,
//!     MIN_CAPITAL_COMMERCIAL_BANK_LAK, MIN_CAPITAL_ADEQUACY_RATIO_PERCENT,
//! };
//! use chrono::Utc;
//!
//! // Validate minimum capital for a new bank
//! let bank_type = BankType::StateOwned;
//! let capital = 350_000_000_000; // 350 billion LAK
//! assert!(validate_minimum_capital(&bank_type, capital).is_ok());
//!
//! // Create a banking license
//! let license = BankingLicense {
//!     license_number: "BOL-2024-001".to_string(),
//!     bank_name_lao: "ທະນາຄານທົດສອບ".to_string(),
//!     bank_name_eng: "Test Bank".to_string(),
//!     bank_type: BankType::PrivateDomestic,
//!     status: LicenseStatus::Active,
//!     issued_at: Utc::now(),
//!     expires_at: Utc::now() + chrono::Duration::days(365 * 5),
//!     licensed_activities: vec![
//!         BankingActivity::AcceptDeposits,
//!         BankingActivity::MakeLoans,
//!         BankingActivity::ForeignExchange,
//!     ],
//!     registered_capital_lak: 400_000_000_000,
//!     paid_up_capital_lak: 400_000_000_000,
//!     head_office_address: "Vientiane Capital".to_string(),
//! };
//! assert!(license.is_valid());
//! ```
//!
//! ## AML/CFT Validation Example
//!
//! ```rust
//! use legalis_la::banking_law::{
//!     CustomerDueDiligence, CustomerType, CDDLevel, PEPStatus, RiskRating,
//!     validate_cdd, validate_aml_compliance,
//! };
//! use chrono::Utc;
//!
//! let cdd = CustomerDueDiligence {
//!     customer_id: "C001".to_string(),
//!     customer_name: "Lao Customer".to_string(),
//!     customer_type: CustomerType::Individual,
//!     cdd_level: CDDLevel::Standard,
//!     identity_verified: true,
//!     address_verified: true,
//!     source_of_funds_documented: true,
//!     pep_status: PEPStatus::NotPEP,
//!     risk_rating: RiskRating::Low,
//!     last_review_date: Utc::now(),
//!     next_review_date: Utc::now() + chrono::Duration::days(365),
//! };
//!
//! assert!(validate_cdd(&cdd).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export error types
pub use error::{BankingLawError, Result};

// Re-export constants
pub use types::{
    AML_RECORD_KEEPING_YEARS, DEPOSIT_INSURANCE_LIMIT_LAK, LICENSE_VALIDITY_YEARS,
    MAX_LENDING_RATE_PERCENT, MIN_CAPITAL_ADEQUACY_RATIO_PERCENT, MIN_CAPITAL_COMMERCIAL_BANK_LAK,
    MIN_CAPITAL_FOREIGN_BRANCH_LAK, MIN_CAPITAL_MFI_DEPOSIT_LAK, MIN_CAPITAL_MFI_NON_DEPOSIT_LAK,
    MIN_CET1_RATIO_PERCENT, MIN_LCR_PERCENT, MIN_LEVERAGE_RATIO_PERCENT, MIN_NSFR_PERCENT,
    MIN_TIER1_CAPITAL_RATIO_PERCENT, RELATED_PARTY_LIMIT_PERCENT, RESERVE_REQUIREMENT_PERCENT,
    SINGLE_BORROWER_LIMIT_PERCENT, STR_REPORTING_DEADLINE_HOURS,
};

// Re-export bank types
pub use types::{BankType, BankingActivity, BankingLicense, LicenseStatus, MicrofinanceType};

// Re-export capital adequacy types
pub use types::{
    AssetRiskWeight, CapitalAdequacyReport, RiskWeightedAssets, Tier1Capital, Tier2Capital,
};

// Re-export prudential types
pub use types::{BorrowerExposure, LargeExposureReport, LiquidityReport};

// Re-export deposit protection types
pub use types::{ClaimStatus, DepositInsuranceClaim, DepositType};

// Re-export foreign exchange types
pub use types::{FXTransactionType, ForeignExchangeTransaction};

// Re-export AML/CFT types
pub use types::{
    CDDLevel, CustomerDueDiligence, CustomerType, PEPStatus, RiskRating, STRStatus,
    SuspicionIndicator, SuspiciousTransactionReport,
};

// Re-export interest rate types
pub use types::{DepositRate, InterestRateStructure, LendingRate, LoanType};

// Re-export payment system types
pub use types::{
    BOLReportType, PaymentService, PaymentServiceLicense, RTGSStatus, RTGSTransaction,
};

// Re-export BOL supervision types
pub use types::FitAndProperAssessment;

// Re-export validators
pub use validator::{
    // Deposit protection validators
    calculate_insured_amount,
    // AML/CFT validators
    validate_aml_compliance,
    // BOL supervision validators
    validate_banking_compliance,
    // License validators
    validate_banking_license,
    validate_bol_reporting,
    // Capital adequacy validators
    validate_capital_adequacy,
    // Foreign exchange validators
    validate_capital_flow,
    validate_car,
    validate_cdd,
    validate_cdd_review,
    validate_deposit_claim,
    validate_deposit_insured,
    // Interest rate validators
    validate_deposit_rate,
    validate_exchange_rate,
    validate_fit_and_proper,
    validate_fx_account,
    // Prudential validators
    validate_large_exposures,
    validate_lcr,
    validate_lending_rate,
    validate_leverage_ratio,
    validate_license_for_activity,
    validate_liquidity,
    validate_mfi_capital,
    validate_minimum_capital,
    // Payment system validators
    validate_mobile_banking_compliance,
    validate_nsfr,
    validate_payment_provider,
    validate_pep_identification,
    validate_record_keeping,
    validate_related_party_limit,
    validate_reserve_requirement,
    validate_risk_weight,
    validate_rtgs_transaction,
    validate_sanctions_screening,
    validate_single_borrower_limit,
    validate_str_reporting,
    validate_tier1_ratio,
    validate_usury,
};
