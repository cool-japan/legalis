//! AML/CTF Module (Anti-Money Laundering and Counter-Terrorism Financing Act 2006)
//!
//! Australian AML/CTF compliance requirements administered by AUSTRAC.
//!
//! # Key Legislation
//!
//! ## Anti-Money Laundering and Counter-Terrorism Financing Act 2006 (Cth)
//!
//! ### Part 2: Customer Identification
//!
//! **Division 3: Customer Identification Procedures**
//!
//! Reporting entities must:
//! - Collect and verify customer identification information
//! - Identify beneficial owners
//! - Determine if customer is a PEP
//! - Apply enhanced due diligence for high-risk customers
//!
//! ### Part 3: Reporting Obligations
//!
//! **Division 2: Suspicious Matter Reporting**
//!
//! Must report suspicious matters to AUSTRAC if:
//! - Suspect customer or transaction involves ML/TF
//! - Has reasonable grounds to suspect ML/TF
//!
//! **Division 3: Threshold Transaction Reporting (TTR)**
//!
//! Cash transactions of $10,000 or more (or foreign currency equivalent).
//!
//! **Division 4: International Funds Transfer Instructions (IFTI)**
//!
//! All international transfers must be reported.
//!
//! ### Part 7: AML/CTF Programs
//!
//! Reporting entities must have:
//! - Part A: General requirements (MLRO, employee training, etc.)
//! - Part B: Customer identification procedures
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_au::financial_services::aml_ctf::*;
//! use chrono::NaiveDate;
//!
//! let cdd = AuCustomerDueDiligence {
//!     customer_name: "John Smith".to_string(),
//!     customer_type: CustomerType::Individual,
//!     cdd_level: CddLevel::Standard,
//!     assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     identity_verified: true,
//!     documents: vec![
//!         IdentityDocument {
//!             document_type: DocumentType::Passport,
//!             document_number: "PA1234567".to_string(),
//!             issuing_country: "AUS".to_string(),
//!             expiry_date: Some(NaiveDate::from_ymd_opt(2030, 1, 1).unwrap()),
//!             verified: true,
//!         },
//!     ],
//!     beneficial_owners: vec![],
//!     pep_status: PepStatus::NonPep,
//!     risk_rating: RiskRating::Low,
//!     ongoing_monitoring: true,
//! };
//!
//! validate_customer_identification(&cdd)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{AmlCtfError, Result};
pub use types::{
    AmlCtfProgram, AuCustomerDueDiligence, AustracCompliance, BeneficialOwner, CddLevel,
    CustomerType, DocumentType, EntityType, IdentityDocument, InternationalFundsTransfer,
    MlroDetails, MonitoringFrequency, PepStatus, RiskRating, SuspiciousMatterReport,
    ThresholdTransaction,
};
pub use validator::{
    validate_austrac_compliance, validate_customer_identification, validate_ifti, validate_smr,
    validate_ttr,
};
