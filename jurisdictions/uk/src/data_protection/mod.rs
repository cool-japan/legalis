//! UK Data Protection Module
//!
//! Implementation of UK GDPR and Data Protection Act 2018.
//!
//! # Strategy
//!
//! This module **reuses 80%** of the EU GDPR implementation from `legalis-eu`,
//! as UK GDPR is substantially identical to EU GDPR (UK retained EU law post-Brexit).
//!
//! ## What's Reused from EU (Identical)
//!
//! - **Article 6**: Lawfulness of processing (6 legal bases)
//! - **Article 9**: Special categories of personal data
//! - **Articles 15-22**: Data subject rights
//! - **Article 24**: Controller accountability
//! - **Article 25**: Data protection by design and by default
//! - **Article 26**: Joint controllers
//! - **Article 28**: Processor contracts
//! - **Article 30**: Records of Processing Activities (ROPA)
//! - **Article 32**: Security of processing
//! - **Article 35**: Data Protection Impact Assessment (DPIA)
//! - **Core types**: PersonalDataCategory, DataSubject, DataController, etc.
//!
//! ## UK-Specific Extensions (20%)
//!
//! - **ICO Enforcement**: Information Commissioner's Office powers (not EDPB)
//! - **UK Adequacy Decisions**: Separate from EU post-Brexit
//! - **DPA 2018 Exemptions**: Journalism, research, national security
//! - **Cross-Border Transfers**: UK IDTA, EU SCCs with ICO Addendum
//! - **DPO Registration**: ICO notification requirements
//!
//! # Key Legislation
//!
//! ## UK GDPR (Retained EU Law)
//!
//! UK GDPR is EU Regulation 2016/679 as incorporated into UK law by the
//! Data Protection Act 2018 and amended by the Data Protection, Privacy and
//! Electronic Communications (Amendments etc.) (EU Exit) Regulations 2019.
//!
//! ### Key Differences from EU GDPR
//!
//! - **Supervisory Authority**: ICO (not EDPB)
//! - **Adequacy Decisions**: UK maintains separate list
//! - **Fines**: Issued by ICO (not EU supervisory authorities)
//! - **International Transfers**: UK mechanisms (IDTA, SCCs with Addendum)
//! - **Some exemptions**: Additional UK-specific exemptions in DPA 2018
//!
//! ## Data Protection Act 2018
//!
//! DPA 2018 supplements UK GDPR with:
//! - Additional exemptions (Schedule 2)
//! - Processing for law enforcement purposes (Part 3)
//! - Intelligence services processing (Part 4)
//! - Criminal offences (Part 6)
//! - ICO enforcement powers
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::data_protection::*;
//!
//! // Use EU GDPR types directly (reused)
//! let controller = DataController {
//!     id: "ctrl-001".to_string(),
//!     name: "Acme Ltd".to_string(),
//!     established_in_eu: false, // UK-based
//!     dpo_appointed: true,
//! };
//!
//! // UK-specific: ICO enforcement
//! let ico_action = IcoEnforcement::InformationNotice {
//!     deadline_days: 30,
//!     information_required: vec![
//!         "List all processors".to_string(),
//!         "Provide ROPA".to_string(),
//!     ],
//! };
//!
//! // UK-specific: Adequacy check
//! let transfer_ok = is_adequate_country_uk("United States"); // false
//! ```
//!
//! # Legal References
//!
//! - [UK GDPR](https://www.legislation.gov.uk/eur/2016/679)
//! - [Data Protection Act 2018](https://www.legislation.gov.uk/ukpga/2018/12)
//! - [ICO Guidance](https://ico.org.uk/for-organisations/guide-to-data-protection/)

#![allow(missing_docs)]

// ============================================================================
// RE-EXPORT EU GDPR TYPES (80% REUSE)
// ============================================================================

// Article 6: Lawfulness of processing
pub use legalis_eu::gdpr::{DataProcessing, LawfulBasis, ProcessingValidation};

// Article 9: Special categories
pub use legalis_eu::gdpr::{
    Article9Exception, Article9Processing, Article9Validation, SpecialCategory,
};

// Core types
pub use legalis_eu::gdpr::{
    ConsentQuality, DataController, DataProcessor, DataSubject, PersonalDataCategory,
    ProcessingOperation,
};

// Article 32: Security
pub use legalis_eu::gdpr::{
    DataBreach, OrganizationalMeasure, SecurityAssessment, SecurityValidation, TechnicalMeasure,
};

// Articles 15-22: Data subject rights
pub use legalis_eu::gdpr::{DataSubjectRequest, DataSubjectRight, RequestValidation};

// Article 24: Accountability
pub use legalis_eu::gdpr::{AccountabilityMeasure, ControllerAccountability};

// Article 25: Data protection by design
pub use legalis_eu::gdpr::{DataProtectionByDesign, DesignPrinciple};

// Article 26: Joint controllers
pub use legalis_eu::gdpr::{JointControllerArrangement, JointControllershipBasis};

// Article 28: Processor contracts
pub use legalis_eu::gdpr::ProcessorContract;

// Article 30: Records of Processing Activities
pub use legalis_eu::gdpr::{ProcessingRecord, RecordsOfProcessingActivities};

// Article 35: Data Protection Impact Assessment
pub use legalis_eu::gdpr::{DataProtectionImpactAssessment, DpiaValidation, RiskAssessment};

// GDPR Errors
pub use legalis_eu::gdpr::GdprError;

// ============================================================================
// UK-SPECIFIC MODULES (20% NEW)
// ============================================================================

pub mod adequacy;
pub mod enforcement;
pub mod exemptions;

// Re-export UK-specific types
pub use adequacy::{UkAdequacyDecision, is_adequate_country_uk};
pub use enforcement::{IcoEnforcement, IcoEnforcementType};
pub use exemptions::{Dpa2018Exemption, ExemptionType};

/// UK GDPR Result type
pub type Result<T> = std::result::Result<T, UkDataProtectionError>;

/// UK Data Protection errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum UkDataProtectionError {
    /// EU GDPR error (reused)
    #[error("GDPR violation: {0}")]
    Gdpr(#[from] GdprError),

    /// ICO enforcement action required
    #[error("ICO enforcement action: {0:?}")]
    IcoEnforcement(IcoEnforcementType),

    /// Invalid adequacy for international transfer
    #[error(
        "Country '{country}' does not have UK adequacy decision. Transfer requires safeguards (IDTA, SCCs with Addendum, or Article 49 derogation)."
    )]
    NoAdequacyDecision { country: String },

    /// Exemption not applicable
    #[error("DPA 2018 exemption not applicable: {reason}")]
    ExemptionNotApplicable { reason: String },
}
