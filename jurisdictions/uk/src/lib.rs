#![allow(clippy::large_enum_variant)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::new_ret_no_self)]

//! United Kingdom Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of UK law (England & Wales) across five major areas.
//!
//! **Status**: v0.2.0 - Initial Implementation 🚧
//! - Foundation complete
//! - Modules in development
//!
//! ## Legal Areas Covered
//!
//! 1. **Employment Law** (Employment Rights Act 1996) - 🚧 IN PROGRESS
//!    - Employment contracts and written particulars (s.1)
//!    - Statutory notice periods (s.86)
//!    - Unfair dismissal (s.98 - 2-year qualifying period)
//!    - Redundancy payments (s.162)
//!    - Working Time Regulations 1998 (48-hour week)
//!    - National Minimum Wage Act 1998 (age-based rates)
//!
//! 2. **Data Protection** (UK GDPR / Data Protection Act 2018) - ⏳ PLANNED
//!    - Reuses 80% from EU GDPR implementation
//!    - UK-specific: ICO enforcement, adequacy decisions
//!    - DPA 2018 exemptions (journalism, research, national security)
//!    - UK international data transfers (IDTA, SCCs with addendum)
//!
//! 3. **Consumer Rights** (Consumer Rights Act 2015) - ⏳ PLANNED
//!    - Goods contracts (s.9-11: quality, purpose, description)
//!    - Services contracts (s.49-52: care, skill, time, price)
//!    - Digital content (s.34-47)
//!    - Tiered remedies (short-term reject → repair/replace → price reduction/final reject)
//!    - Unfair terms (Part 2)
//!
//! 4. **Contract Law** (Common Law Principles) - ⏳ PLANNED
//!    - Contract formation (offer, acceptance, consideration, intention)
//!    - Common law rules: mirror image rule, postal rule
//!    - Case law integration: Hadley v Baxendale, Adams v Lindsell, etc.
//!    - Terms classification (condition, warranty, innominate)
//!    - Remedies (damages, specific performance, injunction)
//!
//! 5. **Company Law** (Companies Act 2006) - ⏳ PLANNED
//!    - Company formation (Part 2)
//!    - Seven statutory director duties (ss.171-177)
//!    - Share capital requirements (£50k minimum for plc)
//!    - Company name restrictions (ss.53-81)
//!    - Corporate governance
//!
//! ## UK Legal System Characteristics
//!
//! ### Common Law vs Civil Law
//!
//! The UK (England & Wales) follows the **Common Law** tradition, fundamentally different
//! from civil law systems (Germany, France, Japan):
//!
//! ```text
//! Common Law (UK)              Civil Law (DE, FR, JP)
//! ├── Primary source           ├── Primary source
//! │   └── Case law (precedent) │   └── Codified statutes
//! ├── Court role               ├── Court role
//! │   └── Law-making           │   └── Law-applying
//! ├── Reasoning                ├── Reasoning
//! │   └── Inductive            │   └── Deductive
//! └── Binding force            └── Binding force
//!     └── Stare decisis            └── Statutory text
//! ```
//!
//! ### Stare Decisis (Binding Precedent)
//!
//! UK courts follow precedent from higher courts:
//! - Supreme Court (binds all lower courts)
//! - Court of Appeal (binds High Court and below)
//! - High Court (binds County Court and tribunals)
//!
//! ### Statute Referencing
//!
//! UK statutes use section (s.) notation, not articles:
//! - `ERA 1996 s.86` (not "Article 86")
//! - `CA 2006 ss.171-177` (sections 171 to 177)
//! - `CRA 2015 s.9` (section 9)
//!
//! ### Contract Formation Requirements
//!
//! Unlike civil law, UK common law requires **consideration**:
//! - Must move from promisee (Tweddle v Atkinson 1861)
//! - Must not be past consideration (Re McArdle 1951)
//! - Must be sufficient but need not be adequate (Chappell v Nestlé 1960)
//!
//! ## Regional Coverage
//!
//! This crate currently covers **England & Wales** only:
//! - **Scotland**: Different legal system (hybrid civil/common law) - not yet implemented
//! - **Northern Ireland**: Separate but similar to E&W - not yet implemented
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::employment::{EmploymentContract, ContractType};
//!
//! let contract = EmploymentContract::builder()
//!     .with_employee_name("John Smith")
//!     .with_employer_name("Acme Ltd")
//!     .with_contract_type(ContractType::Permanent);
//!
//! // Validate against ERA 1996
//! contract.validate()?;
//! ```
//!
//! ## Architecture
//!
//! Each module follows the standard Legalis-RS pattern:
//! - `types.rs` - Core data structures
//! - `error.rs` - Error types with statute references
//! - `validator.rs` - Validation logic
//! - `mod.rs` - Module documentation and re-exports
//!
//! ## Dependencies
//!
//! - `legalis-core` - Core legal framework
//! - `legalis-eu` - EU GDPR implementation (reused for UK GDPR)
//! - `chrono` - Date/time handling
//! - `thiserror` - Error handling
//! - `uuid` - Unique identifiers
//! - `serde` - Serialization (optional feature)

#![deny(missing_docs)]
#![warn(clippy::all)]

/// Employment Law (Employment Rights Act 1996, Working Time Regulations 1998, NMWA 1998)
///
/// Covers:
/// - Employment contracts and written particulars
/// - Statutory notice periods
/// - Unfair dismissal (2-year qualifying period)
/// - Redundancy payments (age-based multipliers)
/// - Working time regulations (48-hour week)
/// - National minimum wage (age-based rates)
pub mod employment;

/// Data Protection (UK GDPR, Data Protection Act 2018)
///
/// 80% reuse from EU GDPR with UK-specific adaptations:
/// - ICO enforcement (not EDPB)
/// - UK adequacy decisions (post-Brexit)
/// - DPA 2018 exemptions (journalism, research, national security)
/// - UK international data transfers (IDTA, SCCs with addendum)
pub mod data_protection;

/// Consumer Rights (Consumer Rights Act 2015)
///
/// Covers:
/// - Goods contracts (satisfactory quality, fit for purpose, as described)
/// - Services contracts (reasonable care and skill)
/// - Digital content
/// - Tiered remedies (short-term reject → repair/replace → price reduction/final reject)
/// - Unfair terms test (Part 2)
pub mod consumer_rights;

/// Contract Law (Common Law Principles)
///
/// Covers:
/// - Contract formation (offer, acceptance, consideration, intention)
/// - Common law rules (mirror image rule, postal rule)
/// - Case law integration (Hadley v Baxendale, Adams v Lindsell, etc.)
/// - Terms classification (condition, warranty, innominate)
/// - Breach and remedies
pub mod contract;

/// Company Law (Companies Act 2006)
///
/// Covers:
/// - Company formation (Part 2)
/// - Seven statutory director duties (ss.171-177)
/// - Share capital requirements
/// - Company name restrictions (ss.53-81)
/// - Corporate governance
pub mod company;

/// Financial Services (FSMA 2000, FCA Rules)
///
/// Covers:
/// - FCA authorization and regulated activities
/// - 11 Principles for Businesses (PRIN)
/// - Client categorization (COBS 3)
/// - Suitability and appropriateness (COBS 9, 10)
/// - Client assets protection (CASS 6, 7)
/// - Financial promotions (FSMA s.21)
/// - Market abuse (UK MAR)
/// - Best execution (COBS 11)
/// - Senior Managers Regime (SM&CR)
pub mod financial_services;

// Re-exports for convenience
pub use employment::{
    EmploymentContract, EmploymentError, MinimumWageAssessment, RedundancyPayment,
    validate_employment_contract,
};

// Data protection re-exports
pub use data_protection::{
    Article9Processing, DataController, DataProcessing, DataSubject, Dpa2018Exemption,
    IcoEnforcement, LawfulBasis, PersonalDataCategory, SpecialCategory, UkAdequacyDecision,
    UkDataProtectionError, is_adequate_country_uk,
};

// Consumer rights re-exports
pub use consumer_rights::{
    ConsumerRightsError, DigitalContentContract, GoodsContract, GoodsStatutoryRight,
    ServicesContract, ServicesStatutoryRight, UnfairTermAssessment, validate_as_described,
    validate_digital_content_contract, validate_fit_for_purpose, validate_goods_contract,
    validate_satisfactory_quality, validate_services_contract, validate_unfair_term,
};

// Contract law re-exports
pub use contract::{
    Acceptance, AcceptanceMethod, Consideration, ConsiderationType, ContractError,
    ContractFormation, IntentionToCreateLegalRelations, Offer, OfferType, validate_acceptance,
    validate_capacity, validate_consideration, validate_contract_formation, validate_intention,
    validate_offer,
};

// Company law re-exports
pub use company::{
    AnnualAccountsRequirement, CompanyFormation, CompanyLawError, CompanyType, Director,
    DirectorDutiesCompliance, RegisteredOffice, ShareCapital, Shareholder,
    validate_company_formation, validate_company_name, validate_director_duties,
};

// Financial services re-exports
pub use financial_services::{
    AuthorizationStatus, ClientCategory, FcaAuthorization, FinancialServicesError, InvestmentType,
    PrinciplesCompliance, RegulatedActivity, SuitabilityAssessment, validate_fca_authorization,
    validate_principles_compliance, validate_suitability_assessment,
};
