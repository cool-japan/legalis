//! GDPR (General Data Protection Regulation 2016/679) Implementation
//!
//! This module provides comprehensive modeling of the GDPR, the EU's primary data protection law.
//!
//! ## Covered Articles
//!
//! - **Article 6**: Lawfulness of processing (6 legal bases)
//! - **Article 9**: Processing of special categories of personal data
//! - **Articles 15-22**: Data subject rights (access, erasure, portability, etc.)
//! - **Article 24**: Responsibility of the controller
//! - **Article 25**: Data protection by design and by default
//! - **Article 26**: Joint controllers
//! - **Article 28**: Processor contracts
//! - **Article 30**: Records of Processing Activities (ROPA)
//! - **Articles 32-34**: Security of processing and breach notification
//! - **Article 35**: Data Protection Impact Assessment (DPIA)
//! - **Articles 37-39**: Data Protection Officer (DPO) designation and tasks
//! - **Articles 44-49**: Cross-border transfers
//! - **Article 83**: Administrative fines
//!
//! ## Key Concepts
//!
//! ### Lawful Basis (Article 6)
//!
//! Processing requires at least one lawful basis:
//! 1. Consent (Article 6(1)(a))
//! 2. Contract (Article 6(1)(b))
//! 3. Legal obligation (Article 6(1)(c))
//! 4. Vital interests (Article 6(1)(d))
//! 5. Public task (Article 6(1)(e))
//! 6. Legitimate interests (Article 6(1)(f)) - requires balancing test
//!
//! ### Special Categories (Article 9)
//!
//! Processing prohibited except under specific exceptions:
//! - Racial or ethnic origin
//! - Political opinions
//! - Religious beliefs
//! - Health data
//! - Genetic/biometric data
//! - Sex life/orientation
//!
//! ### Data Subject Rights (Chapter III)
//!
//! Individuals have rights to:
//! - Access their data (Article 15)
//! - Rectify inaccurate data (Article 16)
//! - Erasure/"right to be forgotten" (Article 17)
//! - Data portability (Article 20)
//! - Object to processing (Article 21)
//!
//! ## Example Usage
//!
//! ```rust
//! use legalis_eu::gdpr::*;
//!
//! // Validate consent-based processing
//! let processing = DataProcessing::new()
//!     .with_controller("Acme Corp")
//!     .with_purpose("Marketing emails")
//!     .add_data_category(PersonalDataCategory::Regular("email".to_string()))
//!     .with_lawful_basis(LawfulBasis::Consent {
//!         freely_given: true,
//!         specific: true,
//!         informed: true,
//!         unambiguous: true,
//!     });
//!
//! match processing.validate() {
//!     Ok(validation) => {
//!         if validation.is_compliant() {
//!             println!("✅ Processing is GDPR compliant");
//!         }
//!     }
//!     Err(e) => println!("❌ Error: {}", e),
//! }
//! ```

pub mod article24;
pub mod article25;
pub mod article26;
pub mod article6;
pub mod article83;
pub mod article9;
pub mod cross_border;
pub mod dpia;
pub mod dpo;
pub mod error;
pub mod processor_contract;
pub mod rights;
pub mod ropa;
pub mod security;
pub mod types;

// Re-exports
pub use article6::{DataProcessing, ProcessingValidation};
pub use article9::{
    Article9Exception, Article9Processing, Article9Validation, HealthcarePurpose, ResearchPurpose,
};
pub use article24::{
    AccountabilityMeasure, Article24Validation, ComplianceCertification, ControllerAccountability,
    DataSensitivity, DataVolume,
};
pub use article25::{
    Article25Validation, DataProtectionByDesign, DefaultSetting, DesignPrinciple,
    PrivacyEnhancingTechnology,
};
pub use article26::{
    Article26Validation, JointController, JointControllerArrangement, JointControllershipBasis,
    Responsibility,
};
pub use article83::{
    AdministrativeFine, Article83Factors, FineCalculation, FineTier, ViolatedArticle,
};
pub use cross_border::{
    AdequateCountry, CrossBorderTransfer, CrossBorderTransferValidation, TransferDerogation,
    TransferLegalBasis, TransferSafeguard,
};
pub use dpia::{
    DataProtectionImpactAssessment, DpiaTrigger, DpiaValidation, Effectiveness, Likelihood,
    Mitigation, RiskAssessment, RiskLevel, RiskType, Severity,
};
pub use dpo::{
    CoreActivity, DpoContactDetails, DpoDesignation, DpoDesignationAssessment, DpoEntityType,
    DpoQualification, DpoRequirementResult, DpoTask, DpoValidation, MonitoringType,
    ProcessingScale,
};
pub use error::GdprError;
pub use processor_contract::{
    Article28Clause, ContractDuration, ContractParty, ProcessorContract,
    ProcessorContractValidation, SubProcessor, SubProcessorAuthorization,
};
pub use rights::{DataSubjectRequest, RequestValidation};
pub use ropa::{
    ContactDetails, EntityType as RopaEntityType, ProcessingRecord, RecordValidation,
    RecordsOfProcessingActivities, RopaExemption, RopaValidation, ThirdCountryTransfer,
};
pub use security::{
    BreachComplianceStatus, BreachNotificationRequirements, DataBreach, OrganizationalMeasure,
    RiskLevel as SecurityRiskLevel, SecurityAssessment, SecurityValidation, TechnicalMeasure,
};
pub use types::{
    Article49Derogation, BreachCategory, BreachSeverity, ComplianceStatus, ConsentQuality,
    ConsentRecord, CrossBorderMechanism, DataController, DataProcessor, DataSubject,
    DataSubjectRight, LawfulBasis, PersonalDataCategory, ProcessingOperation, SpecialCategory,
};
