//! AML/CTF Module (Money Laundering Regulations 2017, POCA 2002, Terrorism Act 2000)
//!
//! Comprehensive implementation of UK Anti-Money Laundering and Counter-Terrorist Financing
//! regulations for financial institutions.
//!
//! # Key Legislation
//!
//! ## Money Laundering Regulations 2017 (MLR 2017)
//!
//! Primary UK anti-money laundering legislation implementing the EU's Fourth and Fifth
//! Money Laundering Directives.
//!
//! ### Regulation 27: Customer Due Diligence (CDD)
//!
//! Firms must apply CDD measures when:
//! - **Establishing a business relationship**
//! - **Carrying out an occasional transaction** of €15,000 or more
//! - **Suspecting money laundering or terrorist financing**
//! - **Doubting the veracity** of previously obtained customer identification
//!
//! Three levels of CDD:
//!
//! 1. **Simplified Due Diligence (SDD)** (Reg 37)
//!    - Lower-risk customers (e.g., credit institutions, listed companies)
//!    - Reduced verification requirements
//!
//! 2. **Standard Due Diligence**
//!    - Default level for most customers
//!    - Identity verification, beneficial ownership, purpose of relationship
//!
//! 3. **Enhanced Due Diligence (EDD)** (Reg 33)
//!    - High-risk customers (PEPs, high-risk countries, complex structures)
//!    - Additional measures: source of wealth/funds, senior management approval
//!
//! ### Regulation 28: CDD Measures
//!
//! CDD must:
//! - **(1)** Identify the customer
//! - **(2)** Verify customer identity using documents, data or information from reliable source
//! - **(3)(a)** Identify beneficial owner (individuals owning >25% of entity)
//! - **(3)(b)** Verify beneficial owner identity
//! - **(3)(c)** Obtain information on purpose and intended nature of business relationship
//! - **(4)** Conduct ongoing monitoring of business relationship
//!
//! ### Regulation 35: Politically Exposed Persons (PEPs)
//!
//! PEPs are individuals entrusted with prominent public functions:
//! - **Domestic PEPs**: UK public figures (ministers, MPs, senior judges, military officers)
//! - **Foreign PEPs**: Foreign country public figures
//! - **International organization PEPs**: UN, EU, NATO officials
//! - **PEP family members**: Spouse, children, parents
//! - **PEP close associates**: Joint beneficial ownership or close business relationship
//!
//! Enhanced DD required for ALL PEP categories (Reg 35(4)):
//! - **(a)** Senior management approval for establishing relationship
//! - **(b)** Adequate measures to establish source of wealth and source of funds
//! - **(c)** Enhanced ongoing monitoring of business relationship
//!
//! ### Regulation 40: Suspicious Activity Reports (SARs)
//!
//! Firms must report to National Crime Agency (NCA) if they:
//! - Know or suspect money laundering or terrorist financing
//! - Have reasonable grounds for knowledge or suspicion
//!
//! ## Proceeds of Crime Act 2002 (POCA 2002)
//!
//! Creates criminal offences for money laundering and failure to report.
//!
//! ### Section 330: Failure to Disclose (Regulated Sector)
//!
//! Criminal offence for person in regulated sector to fail to disclose knowledge or suspicion
//! of money laundering to NCA as soon as practicable.
//!
//! **Penalty**: Up to 5 years imprisonment and/or unlimited fine
//!
//! ### Section 333A: Tipping Off
//!
//! Criminal offence to make disclosure likely to prejudice investigation if person knows or
//! suspects a SAR has been made.
//!
//! **Penalty**: Up to 5 years imprisonment and/or unlimited fine
//!
//! ### Section 335: Consent Regime
//!
//! For some transactions, firms must obtain NCA consent before proceeding. NCA has 7 working
//! days to respond, extendable to 31 days.
//!
//! ## Terrorism Act 2000
//!
//! ### Section 21A: Failure to Disclose (Terrorist Financing)
//!
//! Similar to POCA s.330 but for terrorist financing. Criminal offence to fail to disclose
//! knowledge or suspicion of terrorist financing to NCA.
//!
//! **Penalty**: Up to 5 years imprisonment and/or unlimited fine
//!
//! ### Section 21D: Tipping Off (Terrorist Financing)
//!
//! Criminal offence to make disclosure likely to prejudice terrorist financing investigation.
//!
//! ## Sanctions and Anti-Money Laundering Act 2018
//!
//! Provides framework for UK sanctions regime post-Brexit.
//!
//! ### Office of Financial Sanctions Implementation (OFSI)
//!
//! UK body responsible for:
//! - Implementing and enforcing financial sanctions
//! - Maintaining UK sanctions list
//! - Issuing licenses for sanctioned activities
//!
//! **Penalty for sanctions violations**: Up to 7 years imprisonment and/or unlimited fine
//!
//! ## MLR 2017 Regulation 14A: Cryptoasset Travel Rule
//!
//! Implements FATF Recommendation 16 (Travel Rule) for virtual assets.
//!
//! Cryptoasset exchange providers must obtain and transmit information on originator and
//! beneficiary for transfers **≥£1,000**:
//!
//! **Originator information** (reg 14A(3)):
//! - Name
//! - Account number (or unique transaction reference)
//! - Address, national identity number, customer ID, or date/place of birth
//!
//! **Beneficiary information** (reg 14A(4)):
//! - Name
//! - Account number (or unique transaction reference)
//!
//! # International Context: FATF Recommendations
//!
//! Financial Action Task Force (FATF) sets international standards for AML/CTF.
//!
//! Key FATF Recommendations:
//! - **Recommendation 10**: Customer Due Diligence
//! - **Recommendation 12**: Politically Exposed Persons
//! - **Recommendation 16**: Travel Rule for wire transfers (now includes virtual assets)
//! - **Recommendation 20**: Suspicious transaction reporting
//!
//! UK is FATF member and MLR 2017 implements FATF standards.
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::financial_services::aml_ctf::*;
//! use chrono::NaiveDate;
//!
//! // Standard CDD for individual customer
//! let cdd = CustomerDueDiligence {
//!     customer_name: "John Smith".to_string(),
//!     customer_type: CustomerType::Individual,
//!     cdd_level: CddLevel::Standard,
//!     assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     identity_verified: true,
//!     identity_documents: vec![
//!         IdentityDocument {
//!             document_type: "Passport".to_string(),
//!             document_number: "AB123456".to_string(),
//!             issuing_country: "GBR".to_string(),
//!             expiry_date: Some(NaiveDate::from_ymd_opt(2030, 1, 1).unwrap()),
//!             verified: true,
//!         }
//!     ],
//!     beneficial_owners: vec![],
//!     ownership_structure_verified: true,
//!     purpose_of_relationship: "Savings account".to_string(),
//!     nature_of_business: "Salaried employee".to_string(),
//!     source_of_funds: None,
//!     source_of_wealth: None,
//!     risk_rating: RiskRating::Low,
//!     pep_status: PepStatus::NonPep,
//!     sanctions_screening_passed: true,
//!     ongoing_monitoring_frequency: MonitoringFrequency::Annual,
//!     last_review_date: None,
//! };
//!
//! // Validate CDD
//! validate_cdd(&cdd)?;
//!
//! // Enhanced DD for PEP
//! let pep_cdd = CustomerDueDiligence {
//!     customer_name: "Foreign Minister".to_string(),
//!     cdd_level: CddLevel::Enhanced, // Enhanced DD required
//!     pep_status: PepStatus::ForeignPep {
//!         country: "France".to_string(),
//!         position: "Minister of Finance".to_string(),
//!     },
//!     source_of_wealth: Some("Government salary and family inheritance".to_string()),
//!     source_of_funds: Some("Monthly salary payments".to_string()),
//!     ongoing_monitoring_frequency: MonitoringFrequency::Quarterly, // More frequent
//!     // ... other fields
//! #   customer_type: CustomerType::Individual,
//! #   assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//! #   identity_verified: true,
//! #   identity_documents: vec![],
//! #   beneficial_owners: vec![],
//! #   ownership_structure_verified: true,
//! #   purpose_of_relationship: "Investment".to_string(),
//! #   nature_of_business: "Government official".to_string(),
//! #   risk_rating: RiskRating::High,
//! #   sanctions_screening_passed: true,
//! #   last_review_date: None,
//! };
//!
//! validate_cdd(&pep_cdd)?;
//! validate_enhanced_dd(&pep_cdd, true)?; // true = senior management approval obtained
//!
//! // Suspicious Activity Report
//! let sar = SuspiciousActivityReport {
//!     report_id: "SAR-2024-001".to_string(),
//!     report_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
//!     subject_name: "Suspicious Customer Ltd".to_string(),
//!     subject_id: Some("CUST-456".to_string()),
//!     suspicion_type: SuspicionType::MoneyLaundering,
//!     grounds_for_suspicion: "Customer made multiple large cash deposits over short period, inconsistent with stated business activity. Pattern suggests structuring to avoid reporting thresholds.".to_string(),
//!     transaction_amount_gbp: Some(100_000.0),
//!     transaction_date: Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()),
//!     transaction_description: Some("Cash deposits in amounts just below £10,000".to_string()),
//!     reported_to_nca: true,
//!     nca_reference: Some("NCA-2024-98765".to_string()),
//!     nca_consent_obtained: None,
//! };
//!
//! validate_sar(&sar)?;
//!
//! // Cryptoasset Travel Rule
//! let transfer = TravelRuleTransfer {
//!     transaction_id: "TX-2024-001".to_string(),
//!     transaction_date: NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
//!     amount_gbp: 5_000.0, // ≥£1000, Travel Rule applies
//!     originator_name: "Alice Smith".to_string(),
//!     originator_wallet_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
//!     originator_account_number: Some("ACC-001".to_string()),
//!     beneficiary_name: "Bob Jones".to_string(),
//!     beneficiary_wallet_address: "0x5aeda56215b167893e80b4fe645ba6d5bab767de".to_string(),
//!     beneficiary_account_number: Some("ACC-002".to_string()),
//!     information_transmitted: true,
//!     transmission_method: Some("TRP protocol".to_string()),
//! };
//!
//! validate_travel_rule(&transfer)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Compliance Checklist
//!
//! ## For All Customers
//! - [ ] Identity verified using reliable source (MLR 2017 Reg 28(2))
//! - [ ] Purpose of business relationship established (Reg 28(3)(c))
//! - [ ] Sanctions screening performed (OFSI, UN, EU lists)
//! - [ ] Risk rating assigned
//! - [ ] Ongoing monitoring frequency determined
//!
//! ## For Entity Customers
//! - [ ] Beneficial owners identified (individuals owning >25%)
//! - [ ] Beneficial owner identities verified
//! - [ ] Ownership structure understood and documented
//!
//! ## For PEPs (All Categories)
//! - [ ] Enhanced Due Diligence level applied
//! - [ ] Senior management approval obtained (Reg 35(4)(a))
//! - [ ] Source of wealth established (Reg 35(4)(b))
//! - [ ] Source of funds established (Reg 35(4)(b))
//! - [ ] Enhanced ongoing monitoring (quarterly or more frequent)
//!
//! ## For Suspicious Activity
//! - [ ] SAR prepared with detailed grounds for suspicion
//! - [ ] SAR submitted to NCA without delay
//! - [ ] NCA reference obtained
//! - [ ] Avoid tipping off customer (POCA s.333A, TA 2000 s.21D)
//! - [ ] Consider NCA consent regime if transaction proceeds
//!
//! ## For Cryptoasset Transfers ≥£1,000
//! - [ ] Originator information obtained (name, wallet, account/reference)
//! - [ ] Beneficiary information obtained (name, wallet, account/reference)
//! - [ ] Information transmitted to receiving institution
//! - [ ] Transmission method documented

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{AmlCtfError, Result};
pub use types::{
    BeneficialOwner, CddLevel, CustomerDueDiligence, CustomerType, EntityType, IdentityDocument,
    MonitoringFrequency, PepStatus, RiskRating, SanctionsScreening, SuspicionType,
    SuspiciousActivityReport, TravelRuleTransfer,
};
pub use validator::{
    validate_cdd, validate_enhanced_dd, validate_sanctions_screening, validate_sar,
    validate_travel_rule,
};
