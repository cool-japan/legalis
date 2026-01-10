//! Payment Services Act 2019 - Singapore Payment Regulation
//!
//! This module provides comprehensive type-safe modeling of Singapore's payment services
//! regulations under the Payment Services Act 2019 (PSA), which came into force on
//! 28 January 2020, replacing the Money-Changing and Remittance Businesses Act (MCRBA)
//! and the Payment Systems (Oversight) Act (PS(O)A).
//!
//! # Overview
//!
//! The Payment Services Act 2019 regulates all payment service providers in Singapore,
//! including traditional money changers, remittance businesses, and modern fintech
//! companies offering e-wallets, digital payment tokens (cryptocurrencies), and other
//! payment innovations.
//!
//! ## Seven Types of Payment Services (PSA s. 3)
//!
//! 1. **Account Issuance Service**
//!    - Issuing payment accounts (e.g., e-wallets, prepaid accounts)
//!    - Examples: GrabPay, Touch 'n Go, YouTrip
//!
//! 2. **Domestic Money Transfer Service**
//!    - Transferring money within Singapore
//!    - Examples: PayNow, FAST transfers, peer-to-peer payments
//!
//! 3. **Cross-Border Money Transfer Service**
//!    - International remittances and payments
//!    - Examples: Western Union, Remitly, Wise (formerly TransferWise)
//!
//! 4. **Merchant Acquisition Service**
//!    - Enabling merchants to accept card/digital payments
//!    - Examples: Stripe, PayPal merchant services, payment gateways
//!
//! 5. **E-Money Issuance Service**
//!    - Issuing stored value facilities (prepaid cards, e-wallets)
//!    - Examples: EZ-Link, NETS FlashPay
//!
//! 6. **Digital Payment Token (DPT) Service**
//!    - Cryptocurrency exchange, wallet custody, token dealing
//!    - Examples: Binance, Coinbase, Crypto.com
//!    - **Note**: This brought cryptocurrency under MAS regulation (2020)
//!
//! 7. **Money-Changing Service**
//!    - Foreign currency exchange
//!    - Examples: Currency exchange booths, banks
//!
//! ## License Tiers
//!
//! Three tiers of licensing based on business scale and services:
//!
//! | License Type | Requirements | Services Allowed | Volume Limit |
//! |--------------|--------------|------------------|--------------|
//! | **Money-Changing License** | Simplified requirements | Money-changing only | No limit |
//! | **Standard Payment Institution (SPI)** | Standard compliance | Single service type | ≤ SGD 3M/month |
//! | **Major Payment Institution (MPI)** | Enhanced compliance | Multiple services OR high volume | > SGD 3M/month |
//!
//! ## Safeguarding Requirements (PSA s. 23)
//!
//! Payment institutions must safeguard customer funds for certain services:
//!
//! ### Services Requiring Safeguarding
//! - E-money issuance: **110%** of outstanding e-money
//! - Account issuance: **100%** of account balances
//! - Domestic money transfer: **100%** of outstanding float
//!
//! ### Safeguarding Methods
//! 1. **Trust account** with a licensed bank in Singapore
//! 2. **Statutory deposit** with MAS
//! 3. **Insurance or guarantee** arrangement (with MAS approval)
//!
//! ## Digital Payment Token (DPT) Regulation
//!
//! Singapore was one of the first countries to comprehensively regulate cryptocurrency
//! services. DPT service providers must:
//!
//! 1. **Obtain MAS License**: Standard or Major Payment Institution license
//! 2. **Implement AML/CFT**: Enhanced due diligence, transaction monitoring
//! 3. **Technology Risk Management**: Cybersecurity, custody safeguards
//! 4. **Consumer Protection**: Disclosure of risks, fair terms
//!
//! ## AML/CFT Requirements (PSA s. 20)
//!
//! All payment service providers must:
//! - Appoint an AML/CFT compliance officer
//! - Conduct Customer Due Diligence (CDD)
//! - Implement transaction monitoring
//! - Report suspicious transactions to STRO
//! - Enhanced verification for accounts > SGD 5,000 (PSA Notice PSN02)
//!
//! # Usage Examples
//!
//! ## Creating a Payment Service Provider
//!
//! ```rust
//! use legalis_sg::payment::*;
//! use chrono::Utc;
//!
//! // Create an e-wallet provider
//! let provider = PaymentServiceProviderBuilder::new()
//!     .uen("202012345A".to_string())
//!     .name("Singapore E-Wallet Pte Ltd".to_string())
//!     .license_type(PaymentLicenseType::StandardPaymentInstitution)
//!     .license_date(Utc::now())
//!     .add_service(PaymentServiceType::EMoneyIssuance)
//!     .add_service(PaymentServiceType::AccountIssuance)
//!     .monthly_volume_sgd(200_000_000) // SGD 2M/month
//!     .float_outstanding_sgd(50_000_000) // SGD 500k outstanding
//!     .safeguarding_enabled(true)
//!     .has_aml_officer(true)
//!     .build()
//!     .expect("Failed to build provider");
//!
//! // Validate compliance
//! let report = validate_payment_provider(&provider).expect("Validation failed");
//! if report.is_compliant {
//!     println!("Provider is compliant");
//! } else {
//!     for error in &report.errors {
//!         println!("Error: {}", error);
//!     }
//! }
//! ```
//!
//! ## Validating Safeguarding Arrangement
//!
//! ```rust
//! use legalis_sg::payment::*;
//! use chrono::Utc;
//!
//! let arrangement = SafeguardingArrangement {
//!     arrangement_type: SafeguardingType::TrustAccount,
//!     institution_name: "DBS Bank Ltd".to_string(),
//!     reference: "TRUST-ACC-123456".to_string(),
//!     amount_safeguarded_sgd: 55_000_000, // SGD 550k (110% of SGD 500k)
//!     established_date: Utc::now(),
//!     last_verified: Utc::now(),
//! };
//!
//! // For e-money issuance, 110% safeguarding required
//! let required = calculate_required_safeguarding(
//!     50_000_000, // SGD 500k float
//!     &PaymentServiceType::EMoneyIssuance,
//! );
//! println!("Required safeguarding: SGD {:.2}", required as f64 / 100.0);
//! // Output: Required safeguarding: SGD 550000.00
//! ```
//!
//! ## DPT (Cryptocurrency) Service Validation
//!
//! ```rust
//! use legalis_sg::payment::*;
//! use chrono::Utc;
//!
//! let crypto_exchange = PaymentServiceProviderBuilder::new()
//!     .uen("202098765B".to_string())
//!     .name("Singapore Crypto Exchange Pte Ltd".to_string())
//!     .license_type(PaymentLicenseType::MajorPaymentInstitution)
//!     .license_date(Utc::now())
//!     .add_service(PaymentServiceType::DigitalPaymentToken)
//!     .add_dpt_service(DptServiceType::Exchange)
//!     .add_dpt_service(DptServiceType::Custody)
//!     .has_aml_officer(true)
//!     .build()
//!     .expect("Failed to build crypto exchange");
//!
//! // Validate DPT-specific compliance
//! match validate_dpt_service(&crypto_exchange) {
//!     Ok(warnings) => {
//!         println!("DPT service compliant");
//!         for warning in warnings {
//!             println!("Warning: {}", warning);
//!         }
//!     }
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! ## Customer Account Verification
//!
//! ```rust
//! use legalis_sg::payment::*;
//! use chrono::Utc;
//!
//! let customer_account = CustomerPaymentAccount {
//!     account_id: "EWALLET-123456".to_string(),
//!     customer_name: "Alice Tan".to_string(),
//!     customer_id: "S9876543A".to_string(),
//!     account_type: PaymentAccountType::EWallet,
//!     balance_sgd: 800_000, // SGD 8,000 (exceeds SGD 5,000 threshold)
//!     opened_date: Utc::now(),
//!     kyc_completed: true,
//!     risk_category: RiskCategory::Medium,
//!     is_verified: true, // Enhanced verification completed
//! };
//!
//! // Validate account compliance
//! match validate_customer_account(&customer_account) {
//!     Ok(warnings) => {
//!         println!("✅ Customer account compliant");
//!         for warning in warnings {
//!             println!("⚠️  {}", warning);
//!         }
//!     }
//!     Err(e) => println!("❌ {}", e),
//! }
//! # Ok::<(), legalis_sg::payment::PaymentError>(())
//! ```
//!
//! # Regulatory Framework
//!
//! ## Key Statutes
//!
//! - **Payment Services Act 2019**: <https://sso.agc.gov.sg/Act/PSA2019>
//!
//! ## MAS Notices
//!
//! - **PSA Notice PSN02**: Customer Due Diligence for DPT Services
//! - **PSA Notice PSN03**: Technology Risk Management
//! - **PSA Notice PSN04**: Prevention of Money Laundering and Countering the Financing of Terrorism
//!
//! ## Regulatory Timeline
//!
//! - **28 Jan 2020**: PSA 2019 came into force
//! - **28 Jul 2020**: Exemption period ended - all providers must be licensed
//! - **DPT regulation**: Brought cryptocurrency exchanges under MAS supervision
//!
//! # Comparison with Other Jurisdictions
//!
//! | Aspect | Singapore PSA | EU PSD2 | US State MTL |
//! |--------|---------------|---------|--------------|
//! | **Crypto** | Regulated under DPT | Excluded | Varies by state |
//! | **Safeguarding** | 100-110% | 100% | Varies |
//! | **License Tiers** | 3 tiers | Single tier | Varies |
//! | **Cross-Border** | Single license | EU passporting | No passporting |
//!
//! # See Also
//!
//! - [`banking`](../banking/index.html) - Banking Act (Cap. 19)
//! - [`pdpa`](../pdpa/index.html) - Data protection for payment data
//!
//! # References
//!
//! - MAS Payment Services Regulation: <https://www.mas.gov.sg/regulation/payments>
//! - List of Licensed Payment Institutions: <https://eservices.mas.gov.sg/fid>
//! - PSA Act Overview: <https://www.mas.gov.sg/regulation/acts/payment-services-act-2019>

pub mod error;
pub mod types;
pub mod validator;

// Re-export key types for convenience
pub use error::{ErrorSeverity, PaymentError, Result};
pub use types::{
    CustomerPaymentAccount, DigitalPaymentToken, DptServiceType, LicenseStatus, PaymentAccountType,
    PaymentLicenseType, PaymentServiceProvider, PaymentServiceProviderBuilder, PaymentServiceType,
    PaymentTransaction, RiskCategory, SafeguardingArrangement, SafeguardingType,
};
pub use validator::{
    AmlStatus, LicenseStatusReport, PaymentProviderValidationReport, SafeguardingStatus,
    assess_safeguarding_status, calculate_required_safeguarding, validate_customer_account,
    validate_dpt_service, validate_payment_provider, validate_safeguarding, validate_transaction,
};
