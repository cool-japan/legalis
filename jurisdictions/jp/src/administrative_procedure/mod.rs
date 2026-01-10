//! Administrative Procedure Act (行政手続法) Implementation
//!
//! Comprehensive support for Japanese Administrative Procedure Act (Act No. 88 of 1993)
//! and Electronic Signatures Act (電子署名法 Act No. 102 of 2000).
//!
//! ## Overview
//!
//! This module provides:
//! - Administrative procedure types (申請、届出、行政指導、処分)
//! - Electronic signature support with certificate validation
//! - Article 5 (reason statement) validation
//! - Article 7 (standard processing period) validation
//! - e-Gov electronic filing integration
//!
//! ## Key Concepts
//!
//! ### Administrative Procedure Act (行政手続法)
//!
//! - **Article 2**: Definitions of procedures
//!   - Application (申請) - Article 2-3
//!   - Notification (届出) - Article 2-7
//!   - Administrative Guidance (行政指導) - Article 2-6
//!   - Administrative Disposition (処分) - Article 2-2
//!
//! - **Article 5**: Reason Statement Requirement (理由の提示)
//!   When making a disposition that adversely affects rights, the agency must
//!   simultaneously present the reasons.
//!
//! - **Article 7**: Standard Processing Period (標準処理期間)
//!   Agencies should set standard processing periods for applications.
//!
//! ### Electronic Signatures Act (電子署名法)
//!
//! - Digital signatures for legal validity
//! - Certificate validation (issuer, validity period, algorithm)
//! - Supported algorithms: RSA-2048, RSA-4096, ECDSA-P256, ECDSA-P384
//!
//! ## Examples
//!
//! ### Creating an Application
//!
//! ```
//! use legalis_jp::administrative_procedure::{
//!     AdministrativeProcedure, Applicant, ProcedureType
//! };
//! use legalis_jp::GovernmentAgency;
//!
//! let applicant = Applicant::individual("田中太郎", "東京都渋谷区1-1-1")
//!     .with_phone("03-1234-5678")
//!     .with_email("tanaka@example.com");
//!
//! let procedure = AdministrativeProcedure::new(
//!     "APP-2026-001",
//!     ProcedureType::Application,
//!     GovernmentAgency::DigitalAgency,
//!     applicant,
//! );
//! ```
//!
//! ### Using Builder Pattern
//!
//! ```
//! use legalis_jp::administrative_procedure::{
//!     ProcedureBuilder, Applicant, ProcedureType
//! };
//! use legalis_jp::GovernmentAgency;
//!
//! let applicant = Applicant::corporation("株式会社テスト", "大阪府大阪市");
//!
//! let procedure = ProcedureBuilder::new()
//!     .procedure_id("APP-2026-002")
//!     .procedure_type(ProcedureType::Application)
//!     .agency(GovernmentAgency::MinistryOfJustice)
//!     .applicant(applicant)
//!     .processing_period_days(30)
//!     .notes("重要な申請")
//!     .build()?;
//! # Ok::<(), legalis_jp::administrative_procedure::AdministrativeError>(())
//! ```
//!
//! ### Electronic Signature
//!
//! ```
//! use legalis_jp::administrative_procedure::{
//!     CertificateBuilder, SignatureBuilder, SignatureAlgorithm
//! };
//!
//! // Create certificate
//! let cert = CertificateBuilder::new()
//!     .issuer("Government PKI")
//!     .subject("田中太郎")
//!     .valid_for_days(365)
//!     .public_key(vec![1, 2, 3]) // Actual public key data
//!     .build()?;
//!
//! // Create signature
//! let signature = SignatureBuilder::new()
//!     .algorithm(SignatureAlgorithm::Rsa2048)
//!     .certificate(cert)
//!     .signature_value(vec![4, 5, 6]) // Actual signature
//!     .build()?;
//! # Ok::<(), legalis_jp::administrative_procedure::AdministrativeError>(())
//! ```
//!
//! ### Validation
//!
//! ```
//! use legalis_jp::administrative_procedure::{
//!     AdministrativeProcedure, Applicant, ProcedureType, validate_procedure
//! };
//! use legalis_jp::GovernmentAgency;
//!
//! let applicant = Applicant::individual("Test User", "Tokyo");
//! let procedure = AdministrativeProcedure::new(
//!     "APP-001",
//!     ProcedureType::Application,
//!     GovernmentAgency::DigitalAgency,
//!     applicant,
//! );
//!
//! let report = validate_procedure(&procedure)?;
//! if report.is_valid() {
//!     println!("Procedure is valid!");
//! } else {
//!     println!("Errors: {:?}", report.errors);
//! }
//! # Ok::<(), legalis_jp::administrative_procedure::AdministrativeError>(())
//! ```
//!
//! ### e-Gov Filing
//!
//! ```
//! use legalis_jp::administrative_procedure::{
//!     AdministrativeProcedure, AdministrativeFilingService,
//!     Applicant, ProcedureType
//! };
//! use legalis_jp::GovernmentAgency;
//!
//! let applicant = Applicant::individual("田中太郎", "東京都");
//! let procedure = AdministrativeProcedure::new(
//!     "APP-001",
//!     ProcedureType::Application,
//!     GovernmentAgency::DigitalAgency,
//!     applicant,
//! );
//!
//! let service = AdministrativeFilingService::new();
//! let xml = service.export_xml(&procedure)?;
//! let json = service.export_json(&procedure)?;
//! # Ok::<(), legalis_jp::administrative_procedure::AdministrativeError>(())
//! ```

pub mod builder;
pub mod error;
pub mod filing;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use builder::{CertificateBuilder, ProcedureBuilder, SignatureBuilder};
pub use error::{AdministrativeError, Result};
pub use filing::AdministrativeFilingService;
pub use types::{
    AdministrativeProcedure, Applicant, ApplicantType, Certificate, ContactInfo, Document,
    DocumentType, ElectronicSignature, Identification, ProcedureType, SignatureAlgorithm,
};
pub use validator::{
    quick_validate, validate_electronic_signature, validate_procedure, validate_signature_algorithm,
};
