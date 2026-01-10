//! Financial Services Module (FSMA 2000, FCA Rules)
//!
//! Comprehensive implementation of UK financial services regulation.
//!
//! # Key Legislation
//!
//! ## Financial Services and Markets Act 2000 (FSMA 2000)
//!
//! FSMA 2000 is the primary UK legislation regulating financial services. It establishes:
//! - The regulatory framework for financial services
//! - FCA (Financial Conduct Authority) and PRA (Prudential Regulation Authority)
//! - The requirement for FCA authorization to conduct regulated activities
//! - The financial promotions regime
//!
//! ### Section 19: The General Prohibition
//!
//! **"No person may carry on a regulated activity in the United Kingdom, or purport to
//! do so, unless he is an authorised person or an exempt person."**
//!
//! Breach is a **criminal offence** punishable by up to 2 years imprisonment and/or unlimited fine.
//!
//! ### Regulated Activities
//!
//! Regulated activities are defined in the Regulated Activities Order 2001 (RAO), including:
//! - Accepting deposits
//! - Dealing in investments (as principal or agent)
//! - Arranging deals in investments
//! - Managing investments
//! - Safeguarding and administering investments
//! - Advising on investments
//! - Operating trading platforms (MTF, OTF)
//!
//! ## FCA Handbook
//!
//! The FCA Handbook contains the detailed rules and guidance for financial services firms.
//!
//! ### PRIN: Principles for Businesses (The 11 Principles)
//!
//! High-level standards that apply to all authorized firms:
//!
//! 1. **Integrity**: A firm must conduct its business with integrity
//! 2. **Skill, care and diligence**: Due skill, care and diligence
//! 3. **Management and control**: Take reasonable care to organize and control affairs
//! 4. **Financial prudence**: Maintain adequate financial resources
//! 5. **Market conduct**: Observe proper standards of market conduct
//! 6. **Customers' interests**: Pay due regard to interests of customers and treat fairly
//! 7. **Communications**: Clear, fair and not misleading communications
//! 8. **Conflicts of interest**: Manage conflicts fairly
//! 9. **Customers: relationships of trust**: Take reasonable care for suitability of advice
//! 10. **Clients' assets**: Arrange adequate protection for client assets
//! 11. **Relations with regulators**: Deal with regulators in open and cooperative way
//!
//! ### COBS: Conduct of Business Sourcebook
//!
//! #### COBS 3: Client Categorization
//!
//! Firms must categorize clients into three categories:
//!
//! **1. Retail Client** (highest protection):
//! - Default category for individuals and small businesses
//! - Full COBS protections apply
//! - Suitability assessment required for personal recommendations
//! - Appropriateness assessment for non-advised sales
//!
//! **2. Professional Client** (intermediate protection):
//! - Large undertakings meeting size thresholds
//! - Per se professional (authorized firms, governments)
//! - Elective professional (client requests, meets criteria)
//! - Reduced protections compared to retail
//!
//! **3. Eligible Counterparty** (minimal protection):
//! - Authorized firms, central banks, governments
//! - Wholesale market participants
//! - Minimal COBS protections apply
//!
//! #### COBS 9: Suitability
//!
//! When providing **personal recommendations** or managing investments for retail clients,
//! firm must:
//!
//! 1. Obtain necessary information about client:
//!    - **Knowledge and experience** in relevant investment types
//!    - **Financial situation** (income, assets, commitments)
//!    - **Investment objectives** (time horizon, risk tolerance, purpose)
//!
//! 2. Assess whether investment is **suitable** for client
//!
//! 3. Provide **suitability report** explaining why recommendation is suitable
//!
//! #### COBS 10: Appropriateness
//!
//! For **non-advised sales** of complex products, firm must assess whether client has
//! necessary knowledge and experience to understand risks.
//!
//! If inappropriate, must warn client. If client insists, can proceed.
//!
//! #### COBS 11: Best Execution
//!
//! Firm must take **all sufficient steps** to obtain best possible result for clients
//! when executing orders, considering:
//! - Price
//! - Costs
//! - Speed
//! - Likelihood of execution and settlement
//! - Size and nature of order
//!
//! Must establish and implement best execution policy.
//!
//! ### CASS: Client Assets Sourcebook
//!
//! Rules for protecting client assets:
//!
//! #### CASS 6: Custody Rules
//! - Client assets must be properly segregated
//! - Held with approved custodians or depositaries
//! - Regular reconciliations
//!
//! #### CASS 7: Client Money Rules
//! - Client money must be segregated from firm's own money
//! - Held in designated **client bank accounts**
//! - Trust arrangements required
//! - **Daily reconciliation** of client money
//! - Cannot be used by firm (ring-fenced)
//!
//! ## Market Abuse Regulation (UK MAR)
//!
//! Retained EU law prohibiting:
//! - **Insider dealing**: Trading on inside information
//! - **Unlawful disclosure**: Disclosing inside information
//! - **Market manipulation**: Artificial price movements
//! - **Benchmark manipulation**: LIBOR rigging, etc.
//!
//! Firms must:
//! - Report suspicious transactions to FCA
//! - Maintain insider lists
//! - Implement market abuse detection systems
//!
//! ## Senior Managers and Certification Regime (SM&CR)
//!
//! Introduced 2016 to increase individual accountability:
//!
//! ### Senior Managers Regime
//! - Senior management functions (SMFs) require FCA approval
//! - Each senior manager has **statement of responsibilities**
//! - Personal accountability for their areas
//! - Regulatory references required
//!
//! ### Key SMFs:
//! - **SMF1**: Chief Executive
//! - **SMF2**: Chief Finance Officer
//! - **SMF4**: Chief Risk Officer
//! - **SMF16**: Compliance Oversight
//! - **SMF17**: Money Laundering Reporting Officer
//!
//! ## Financial Promotions (FSMA Section 21)
//!
//! **"A person must not, in the course of business, communicate an invitation or
//! inducement to engage in investment activity."**
//!
//! Unless:
//! - The person is authorized, or
//! - The content is approved by authorized person, or
//! - Exemption applies
//!
//! All promotions must be **fair, clear and not misleading** (COBS 4).
//!
//! # International Context: London as Global Financial Centre
//!
//! London is one of the world's leading financial centres, alongside New York and Tokyo.
//!
//! ## Key Statistics:
//! - Over 250 foreign banks with offices in London
//! - 40% of global foreign exchange trading
//! - Largest international insurance market
//! - Leading centre for derivatives trading
//!
//! ## Post-Brexit Changes:
//!
//! ### Loss of Passporting
//! - UK firms lost automatic right to provide services in EU
//! - EU firms lost automatic right to provide services in UK
//! - **Temporary Permissions Regime** (TPR) allowed transition
//!
//! ### Equivalence
//! - UK and EU can grant "equivalence" decisions
//! - Allows cross-border access in specific areas
//! - Not comprehensive replacement for passporting
//!
//! ### Regulatory Divergence
//! - UK can now diverge from EU financial regulation
//! - "Regulatory alignment" vs "regulatory autonomy" debate
//! - Some divergence already implemented (e.g., ring-fencing reforms)
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::financial_services::*;
//! use chrono::NaiveDate;
//!
//! // Check FCA authorization
//! let authorization = FcaAuthorization {
//!     firm_reference_number: "123456".to_string(),
//!     firm_name: "Example Investment Advisers Ltd".to_string(),
//!     status: AuthorizationStatus::Authorized,
//!     authorization_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!     regulated_activities: vec![
//!         RegulatedActivity::AdvisingOnInvestments {
//!             investment_type: InvestmentType::Shares,
//!         },
//!     ],
//!     passporting_rights: vec![],
//! };
//!
//! let activity = RegulatedActivity::AdvisingOnInvestments {
//!     investment_type: InvestmentType::Shares,
//! };
//!
//! validate_fca_authorization(&authorization, &activity)?;
//!
//! // Assess suitability for retail client (see types.rs for full struct)
//! let assessment: SuitabilityAssessment = /* ... */;
//! validate_suitability_assessment(&assessment)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Sub-modules for expanded financial services
pub mod aml_ctf;
pub mod cryptoassets;
pub mod mifid2;
pub mod payment_services;

// Re-export key types
pub use error::{FinancialServicesError, Result};

// AML/CTF re-exports
pub use aml_ctf::{
    AmlCtfError, BeneficialOwner, CddLevel, CustomerDueDiligence, CustomerType, EntityType,
    IdentityDocument, MonitoringFrequency, PepStatus, SanctionsScreening, SuspicionType,
    SuspiciousActivityReport, TravelRuleTransfer, validate_cdd, validate_enhanced_dd,
    validate_sanctions_screening, validate_sar, validate_travel_rule,
};

// Payment Services re-exports
pub use payment_services::{
    AuthorizationType, ClientFundsSafeguarding, ConsentStatus, OpenBankingConsent,
    OpenBankingProviderType, PaymentServicesError, PaymentTransaction, Permission,
    SafeguardingMethod, StrongCustomerAuthentication, validate_open_banking_consent,
    validate_safeguarding, validate_sca,
};

// MiFID II re-exports
pub use mifid2::{
    AbilityToBearLosses, BestExecutionReport, ExecutionVenue, KnowledgeLevel, Mifid2Error,
    MifidFirmType, ProductGovernance, ResearchPayment, TargetMarket, TimeHorizon,
    TransactionReport, validate_best_execution_report, validate_product_governance,
    validate_research_payment, validate_target_market_match, validate_transaction_report,
};

// Cryptoassets re-exports
pub use cryptoassets::{
    CryptoassetClassification, CryptoassetExchangeProvider, CryptoassetPromotion,
    CryptoassetsError, RedemptionRights, ReserveBacking, SecurityTokenAssessment,
    SecurityTokenType, Stablecoin, StablecoinPeg, validate_cryptoasset_classification,
    validate_cryptoasset_promotion, validate_exchange_provider, validate_security_token_assessment,
    validate_stablecoin,
};
pub use types::{
    AuthorizationStatus, BestExecutionPolicy, ClientAssetsProtection, ClientCategory,
    EducationLevel, ExecutionFactor, FcaAuthorization, FinancialPromotion, FinancialSituation,
    InvestmentObjective, InvestmentObjectives, InvestmentRecommendation, InvestmentType,
    KnowledgeExperience, LiquidityNeeds, MarketAbuseReport, MarketAbuseType, PassportingRight,
    PassportingStatus, PrincipleCompliance, PrinciplesCompliance, PromotionAudience,
    PromotionMedium, RegulatedActivity, RiskRating, RiskTolerance, SeniorManagementFunction,
    SeniorManager, SuitabilityAssessment,
};
pub use validator::{
    validate_best_execution_policy, validate_client_assets_protection, validate_fca_authorization,
    validate_financial_promotion, validate_principles_compliance, validate_suitability_assessment,
};
