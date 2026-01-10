//! Payment Services Module (Payment Services Regulations 2017 - PSD2)
//!
//! Comprehensive implementation of UK payment services regulation (EU PSD2).
//!
//! # Key Legislation
//!
//! ## Payment Services Regulations 2017 (PSR 2017)
//!
//! UK implementation of EU Payment Services Directive 2 (PSD2).
//!
//! ### Regulation 4: Authorization Types
//!
//! Six types of payment service providers:
//!
//! 1. **Authorized Payment Institution (API)**: Full PSD2 authorization from FCA
//! 2. **Small Payment Institution (SPI)**: Average monthly payment volume < €3 million
//! 3. **Electronic Money Institution (EMI)**: Authorized under EMR 2011
//! 4. **Credit Institution**: Bank authorized under CRR
//! 5. **Account Information Service Provider (AISP)**: Read-only access to accounts
//! 6. **Payment Initiation Service Provider (PISP)**: Initiate payments on behalf of users
//!
//! ### Regulation 67-68: Strong Customer Authentication (SCA)
//!
//! SCA required when payment service user:
//! - **Accesses payment account online**
//! - **Initiates electronic payment transaction**
//! - **Carries out any remote action** with risk of fraud or abuse
//!
//! SCA requires **two independent elements** from different categories:
//! 1. **Knowledge**: Something only user knows (password, PIN, security question)
//! 2. **Possession**: Something only user possesses (mobile device, token, smart card)
//! 3. **Inherence**: Something user is (fingerprint, face, voice, iris)
//!
//! ### Regulation 20-22: Safeguarding of Client Funds
//!
//! Payment institutions must safeguard funds received from payment service users by:
//!
//! **Option 1: Segregation** (Reg 20(1)(a))
//! - Funds held in separate account
//! - Clearly designated for safeguarding
//! - Distinguished from firm's own funds
//! - Held at credit institution or Bank of England
//!
//! **Option 2: Insurance/Guarantee** (Reg 20(1)(b))
//! - Funds protected by insurance policy
//! - Or comparable guarantee
//!
//! **Daily Reconciliation** (Reg 21): Required for both methods
//!
//! ## Open Banking (CMA Order 2017)
//!
//! Competition and Markets Authority ordered nine largest UK banks to implement
//! Open Banking standards by January 2018.
//!
//! ### Account Information Services (AIS)
//! - TPPs can read account information with user consent
//! - Consent valid for **maximum 90 days**
//! - User can revoke consent at any time
//!
//! ### Payment Initiation Services (PIS)
//! - TPPs can initiate payments with user consent
//! - One-off consent per payment
//! - No access to account information beyond payment confirmation

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{PaymentServicesError, Result};
pub use types::{
    AuthorizationType, ClientFundsSafeguarding, ConsentStatus, InherenceFactor, KnowledgeFactor,
    OpenBankingConsent, OpenBankingProviderType, PaymentTransaction, Permission, PossessionFactor,
    SafeguardingAccount, SafeguardingMethod, ScaExemption, StrongCustomerAuthentication,
};
pub use validator::{validate_open_banking_consent, validate_safeguarding, validate_sca};
