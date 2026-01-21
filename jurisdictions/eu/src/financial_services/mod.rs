//! EU Financial Services Regulation
//!
//! This module provides modeling of key EU financial services regulations:
//! - MiFID II (Markets in Financial Instruments Directive II)
//! - PSD2 (Payment Services Directive 2)
//!
//! ## MiFID II (Directive 2014/65/EU)
//!
//! MiFID II regulates investment services and activities in the European Economic Area.
//!
//! ### Investment Services (Annex I Section A)
//!
//! 1. Reception and transmission of orders
//! 2. Execution of orders on behalf of clients
//! 3. Dealing on own account
//! 4. Portfolio management
//! 5. Investment advice
//! 6. Underwriting and placing of financial instruments
//! 7. Operation of multilateral trading facility (MTF)
//! 8. Operation of organized trading facility (OTF)
//!
//! ### Client Categorization (Article 4)
//!
//! Clients are categorized based on knowledge, experience, and financial situation:
//!
//! - **Retail Clients**: Highest level of protection
//! - **Professional Clients**: Reduced protection (elective or per se)
//! - **Eligible Counterparties**: Minimal protection
//!
//! ### Key Conduct of Business Rules (Articles 24-25)
//!
//! 1. **Act in best interest**: Act honestly, fairly, professionally
//! 2. **Information**: Fair, clear, not misleading
//! 3. **Suitability assessment**: For investment advice and portfolio management
//!    - Knowledge and experience
//!    - Financial situation
//!    - Investment objectives
//! 4. **Appropriateness assessment**: For other services
//!    - Knowledge and experience
//! 5. **Best execution** (Article 27): Execute orders on most favorable terms
//!
//! ### Product Governance (Article 16)
//!
//! Manufacturers and distributors must:
//! - Define target market for each product
//! - Ensure distribution strategy consistent with target market
//! - Monitor and review products
//!
//! ### Transaction Reporting (Article 26)
//!
//! Investment firms must report complete and accurate details of transactions
//! to competent authorities within one trading day.
//!
//! ## PSD2 (Directive 2015/2366/EU)
//!
//! PSD2 modernizes the EU payments landscape, enabling open banking and
//! third-party payment service providers.
//!
//! ### Payment Services (Annex I)
//!
//! 1. Cash deposits and withdrawals
//! 2. Payment transactions (credit transfer, direct debit, card payments)
//! 3. Issuing payment instruments
//! 4. Acquiring of payment transactions
//! 5. Money remittance
//! 6. **Payment initiation services (PIS)** - NEW in PSD2
//! 7. **Account information services (AIS)** - NEW in PSD2
//!
//! ### Strong Customer Authentication (SCA) - Article 97
//!
//! Payment service providers must apply SCA when payer:
//! - Accesses payment account online
//! - Initiates electronic payment transaction
//! - Carries out action through remote channel with risk of fraud
//!
//! **SCA Requirements**:
//! - At least 2 elements from different categories:
//!   1. Knowledge (password, PIN)
//!   2. Possession (card, mobile device)
//!   3. Inherence (fingerprint, face recognition)
//! - Elements must be independent (compromise of one doesn't compromise others)
//! - Dynamic linking for payment transactions
//!
//! **SCA Exemptions** (RTS on SCA):
//! - Low-value transactions (< €30)
//! - Recurring transactions
//! - Trusted beneficiaries
//! - Low-risk transactions (based on transaction monitoring)
//!
//! ### Open Banking - Article 67
//!
//! Account servicing payment service providers (ASPSPs) must provide
//! payment initiation and account information service providers with:
//!
//! - Access to payment accounts
//! - Same level of security as used by ASPSP
//! - Dedicated interface (API) for third-party providers
//! - Testing facilities
//! - Performance statistics
//!
//! ### Third-Party Providers (TPPs)
//!
//! PSD2 creates two new types of regulated payment service providers:
//!
//! 1. **Payment Initiation Service Providers (PISPs)**: Initiate payments on behalf of users
//! 2. **Account Information Service Providers (AISPs)**: Aggregate account information
//!
//! Both require:
//! - Authorization by competent authority
//! - Access to payment accounts
//! - Strong customer authentication
//!
//! ### Passporting Rights
//!
//! Authorized firms can provide services across EU Member States:
//! - Freedom of establishment (branches)
//! - Freedom to provide services (cross-border)
//!
//! ## Example Usage
//!
//! ### MiFID II - Client Categorization
//!
//! ```rust
//! use legalis_eu::financial_services::*;
//!
//! let retail_client = ClientCategory::Retail;
//! let professional_client = ClientCategory::Professional {
//!     elective: true, // Client requested professional treatment
//! };
//!
//! // Professional clients can opt-in to retail protection
//! ```
//!
//! ### MiFID II - Best Execution
//!
//! ```rust
//! use legalis_eu::financial_services::*;
//!
//! let best_execution = BestExecutionPolicy {
//!     execution_factors: vec![
//!         ExecutionFactor::Price,
//!         ExecutionFactor::Costs,
//!         ExecutionFactor::Speed,
//!         ExecutionFactor::Likelihood,
//!     ],
//!     execution_venues: vec![
//!         "Exchange A".to_string(),
//!         "MTF B".to_string(),
//!     ],
//!     factor_weighting: "Price has highest priority for retail clients".to_string(),
//!     methodology: "Systematic review of execution quality".to_string(),
//!     monitoring_process: "Monthly review of execution quality data".to_string(),
//! };
//! ```
//!
//! ### PSD2 - Strong Customer Authentication
//!
//! ```rust
//! use legalis_eu::financial_services::*;
//!
//! let sca = StrongCustomerAuthentication {
//!     authentication_elements: vec![
//!         AuthenticationElement::Knowledge,  // PIN
//!         AuthenticationElement::Possession, // Mobile device
//!     ],
//!     dynamic_linking: true, // Required for payments
//!     exemption: None,
//! };
//!
//! // Validates 2+ elements from different categories
//! assert!(sca.authentication_elements.len() >= 2);
//! ```
//!
//! ### PSD2 - Payment Initiation Provider
//!
//! ```rust
//! use legalis_eu::financial_services::*;
//! use legalis_eu::MemberState;
//!
//! let pisp = PaymentInitiationProvider {
//!     name: "PayTech Ltd".to_string(),
//!     authorized: true,
//!     home_member_state: MemberState::Germany,
//!     passporting: vec![MemberState::France, MemberState::Netherlands],
//!     api_access: true, // Access to bank APIs
//! };
//! ```
//!
//! ## Penalties
//!
//! - **MiFID II**: National competent authorities determine sanctions
//! - **PSD2**: Effective, proportionate, and dissuasive administrative sanctions

pub mod error;
pub mod types;

// Re-exports
pub use error::FinancialServicesError;
pub use types::{
    AccountInformationProvider, AuthenticationElement, BestExecutionPolicy, BuySellIndicator,
    ClientAssessment, ClientCategory, ConductOfBusiness, ExecutionFactor, FinancialInstrument,
    FinancialSituation, InducementPolicy, InformationQuality, InvestmentObjective,
    InvestmentService, KnowledgeLevel, OpenBankingApi, Passport, PaymentInitiationProvider,
    PaymentService, PaymentType, ProductGovernance, RiskTolerance, ScaExemption,
    StrongCustomerAuthentication, TargetMarket, ThirdPartyProvider, TransactionReport,
};
