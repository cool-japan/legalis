//! Administrative Law Errors for Lao PDR (ຂໍ້ຜິດພາດກົດໝາຍບໍລິຫານ)
//!
//! This module defines comprehensive error types for administrative law validation
//! with bilingual support (Lao and English).
//!
//! ## Error Categories
//!
//! - **AdministrativeDecisionError**: Missing legal basis, invalid authority
//! - **LicenseError**: Invalid license type, expired license
//! - **PermitError**: Invalid permit type, conditions not met
//! - **SanctionError**: Disproportionate sanction, missing grounds
//! - **AppealError**: Deadline missed, wrong level, insufficient grounds
//! - **StateLiabilityError**: Claim deadline, insufficient evidence
//! - **NotificationError**: Party not notified, invalid notification
//! - **AuthorityError**: Invalid authority level, jurisdiction exceeded

use crate::administrative_law::types::STATE_LIABILITY_CLAIM_DEADLINE_YEARS;
use thiserror::Error;

/// Result type for administrative law operations
pub type AdministrativeLawResult<T> = Result<T, AdministrativeLawError>;

// ============================================================================
// Main Error Enum
// ============================================================================

/// Comprehensive error types for administrative law
/// ປະເພດຂໍ້ຜິດພາດສຳລັບກົດໝາຍບໍລິຫານ
#[derive(Debug, Error)]
pub enum AdministrativeLawError {
    /// Administrative decision error (ຂໍ້ຜິດພາດການຕັດສິນໃຈບໍລິຫານ)
    #[error("Administrative Decision Error: {0}")]
    DecisionError(#[from] AdministrativeDecisionError),

    /// License error (ຂໍ້ຜິດພາດໃບອະນຸຍາດ)
    #[error("License Error: {0}")]
    LicenseError(#[from] LicenseError),

    /// Permit error (ຂໍ້ຜິດພາດໃບຢັ້ງຢືນ)
    #[error("Permit Error: {0}")]
    PermitError(#[from] PermitError),

    /// Sanction error (ຂໍ້ຜິດພາດການລົງໂທດ)
    #[error("Sanction Error: {0}")]
    SanctionError(#[from] SanctionError),

    /// Appeal error (ຂໍ້ຜິດພາດການອຸທອນ)
    #[error("Appeal Error: {0}")]
    AppealError(#[from] AppealError),

    /// State liability error (ຂໍ້ຜິດພາດຄວາມຮັບຜິດຊອບຂອງລັດ)
    #[error("State Liability Error: {0}")]
    StateLiabilityError(#[from] StateLiabilityError),

    /// Notification error (ຂໍ້ຜິດພາດການແຈ້ງ)
    #[error("Notification Error: {0}")]
    NotificationError(#[from] NotificationError),

    /// Authority error (ຂໍ້ຜິດພາດອຳນາດ)
    #[error("Authority Error: {0}")]
    AuthorityError(#[from] AuthorityError),

    /// Validation error (ຂໍ້ຜິດພາດການກວດສອບ)
    #[error("Validation Error: {0}")]
    ValidationError(String),

    /// Internal error (ຂໍ້ຜິດພາດພາຍໃນ)
    #[error("Internal Error: {0}")]
    InternalError(String),
}

impl AdministrativeLawError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            AdministrativeLawError::DecisionError(e) => e.message_lao(),
            AdministrativeLawError::LicenseError(e) => e.message_lao(),
            AdministrativeLawError::PermitError(e) => e.message_lao(),
            AdministrativeLawError::SanctionError(e) => e.message_lao(),
            AdministrativeLawError::AppealError(e) => e.message_lao(),
            AdministrativeLawError::StateLiabilityError(e) => e.message_lao(),
            AdministrativeLawError::NotificationError(e) => e.message_lao(),
            AdministrativeLawError::AuthorityError(e) => e.message_lao(),
            AdministrativeLawError::ValidationError(msg) => {
                format!("ຂໍ້ຜິດພາດການກວດສອບ: {}", msg)
            }
            AdministrativeLawError::InternalError(msg) => {
                format!("ຂໍ້ຜິດພາດພາຍໃນ: {}", msg)
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        match self {
            AdministrativeLawError::DecisionError(e) => e.message_en(),
            AdministrativeLawError::LicenseError(e) => e.message_en(),
            AdministrativeLawError::PermitError(e) => e.message_en(),
            AdministrativeLawError::SanctionError(e) => e.message_en(),
            AdministrativeLawError::AppealError(e) => e.message_en(),
            AdministrativeLawError::StateLiabilityError(e) => e.message_en(),
            AdministrativeLawError::NotificationError(e) => e.message_en(),
            AdministrativeLawError::AuthorityError(e) => e.message_en(),
            AdministrativeLawError::ValidationError(msg) => {
                format!("Validation Error: {}", msg)
            }
            AdministrativeLawError::InternalError(msg) => {
                format!("Internal Error: {}", msg)
            }
        }
    }

    /// Get both Lao and English error messages
    pub fn bilingual_message(&self) -> (String, String) {
        (self.message_lao(), self.message_en())
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        AdministrativeLawError::ValidationError(message.into())
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        AdministrativeLawError::InternalError(message.into())
    }
}

// ============================================================================
// Administrative Decision Errors
// ============================================================================

/// Errors related to administrative decisions
/// ຂໍ້ຜິດພາດກ່ຽວກັບການຕັດສິນໃຈບໍລິຫານ
#[derive(Debug, Error)]
pub enum AdministrativeDecisionError {
    /// Missing legal basis (ຂາດພື້ນຖານທາງກົດໝາຍ)
    #[error("Missing legal basis for administrative decision")]
    MissingLegalBasis,

    /// Invalid legal basis (ພື້ນຖານທາງກົດໝາຍບໍ່ຖືກຕ້ອງ)
    #[error("Invalid legal basis: {law_name} Article {article}")]
    InvalidLegalBasis {
        law_name: String,
        article: u16,
        reason: String,
    },

    /// Invalid authority level (ລະດັບອຳນາດບໍ່ຖືກຕ້ອງ)
    #[error("Authority level {authority_level} is not authorized for this decision type")]
    InvalidAuthorityLevel {
        authority_level: String,
        decision_type: String,
        required_level: String,
    },

    /// Missing decision number (ຂາດເລກທີການຕັດສິນໃຈ)
    #[error("Missing decision number")]
    MissingDecisionNumber,

    /// Missing decision date (ຂາດວັນທີຕັດສິນໃຈ)
    #[error("Missing decision date")]
    MissingDecisionDate,

    /// Missing subject (ຂາດຫົວຂໍ້)
    #[error("Missing subject matter for decision")]
    MissingSubject,

    /// Invalid decision type (ປະເພດການຕັດສິນໃຈບໍ່ຖືກຕ້ອງ)
    #[error("Invalid decision type: {0}")]
    InvalidDecisionType(String),

    /// Missing affected parties (ຂາດຝ່າຍທີ່ໄດ້ຮັບຜົນກະທົບ)
    #[error("No affected parties specified for decision")]
    MissingAffectedParties,

    /// Conflicting decisions (ການຕັດສິນໃຈຂັດແຍ່ງກັນ)
    #[error("Decision conflicts with existing decision {existing_decision}")]
    ConflictingDecision { existing_decision: String },

    /// Decision already final (ການຕັດສິນໃຈເປັນສຸດທ້າຍແລ້ວ)
    #[error("Decision {decision_number} is already final and cannot be modified")]
    DecisionAlreadyFinal { decision_number: String },
}

impl AdministrativeDecisionError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            AdministrativeDecisionError::MissingLegalBasis => {
                "ຂາດພື້ນຖານທາງກົດໝາຍສຳລັບການຕັດສິນໃຈບໍລິຫານ".to_string()
            }
            AdministrativeDecisionError::InvalidLegalBasis {
                law_name,
                article,
                reason,
            } => {
                format!(
                    "ພື້ນຖານທາງກົດໝາຍບໍ່ຖືກຕ້ອງ: {} ມາດຕາ {} - {}",
                    law_name, article, reason
                )
            }
            AdministrativeDecisionError::InvalidAuthorityLevel {
                authority_level,
                decision_type,
                required_level,
            } => {
                format!(
                    "ລະດັບອຳນາດ {} ບໍ່ມີສິດອອກການຕັດສິນໃຈປະເພດ {} (ຕ້ອງການ: {})",
                    authority_level, decision_type, required_level
                )
            }
            AdministrativeDecisionError::MissingDecisionNumber => "ຂາດເລກທີການຕັດສິນໃຈ".to_string(),
            AdministrativeDecisionError::MissingDecisionDate => "ຂາດວັນທີຕັດສິນໃຈ".to_string(),
            AdministrativeDecisionError::MissingSubject => "ຂາດຫົວຂໍ້ສຳລັບການຕັດສິນໃຈ".to_string(),
            AdministrativeDecisionError::InvalidDecisionType(dtype) => {
                format!("ປະເພດການຕັດສິນໃຈບໍ່ຖືກຕ້ອງ: {}", dtype)
            }
            AdministrativeDecisionError::MissingAffectedParties => {
                "ບໍ່ໄດ້ລະບຸຝ່າຍທີ່ໄດ້ຮັບຜົນກະທົບ".to_string()
            }
            AdministrativeDecisionError::ConflictingDecision { existing_decision } => {
                format!("ການຕັດສິນໃຈຂັດແຍ່ງກັບການຕັດສິນໃຈທີ່ມີຢູ່ແລ້ວ {}", existing_decision)
            }
            AdministrativeDecisionError::DecisionAlreadyFinal { decision_number } => {
                format!("ການຕັດສິນໃຈ {} ເປັນສຸດທ້າຍແລ້ວ ບໍ່ສາມາດແກ້ໄຂໄດ້", decision_number)
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a missing legal basis error
    pub fn missing_legal_basis() -> AdministrativeLawError {
        AdministrativeDecisionError::MissingLegalBasis.into()
    }

    /// Create an invalid legal basis error
    pub fn invalid_legal_basis(
        law_name: impl Into<String>,
        article: u16,
        reason: impl Into<String>,
    ) -> AdministrativeLawError {
        AdministrativeDecisionError::InvalidLegalBasis {
            law_name: law_name.into(),
            article,
            reason: reason.into(),
        }
        .into()
    }

    /// Create an invalid authority level error
    pub fn invalid_authority_level(
        authority_level: impl Into<String>,
        decision_type: impl Into<String>,
        required_level: impl Into<String>,
    ) -> AdministrativeLawError {
        AdministrativeDecisionError::InvalidAuthorityLevel {
            authority_level: authority_level.into(),
            decision_type: decision_type.into(),
            required_level: required_level.into(),
        }
        .into()
    }
}

// ============================================================================
// License Errors
// ============================================================================

/// Errors related to licenses
/// ຂໍ້ຜິດພາດກ່ຽວກັບໃບອະນຸຍາດ
#[derive(Debug, Error)]
pub enum LicenseError {
    /// Invalid license type (ປະເພດໃບອະນຸຍາດບໍ່ຖືກຕ້ອງ)
    #[error("Invalid license type: {license_type}")]
    InvalidType { license_type: String },

    /// License expired (ໃບອະນຸຍາດໝົດອາຍຸ)
    #[error("License {license_number} expired on {expiry_date}")]
    Expired {
        license_number: String,
        expiry_date: String,
    },

    /// License suspended (ໃບອະນຸຍາດຖືກລະງັບ)
    #[error("License {license_number} is suspended until {until_date}")]
    Suspended {
        license_number: String,
        until_date: String,
        reason: String,
    },

    /// License revoked (ໃບອະນຸຍາດຖືກຖອນຄືນ)
    #[error("License {license_number} has been revoked")]
    Revoked {
        license_number: String,
        reason: String,
    },

    /// Missing requirements (ຂາດເງື່ອນໄຂ)
    #[error("Missing requirements for {license_type}: {missing_requirements}")]
    MissingRequirements {
        license_type: String,
        missing_requirements: String,
    },

    /// Unauthorized issuing authority (ອົງການອອກບໍ່ມີສິດ)
    #[error("Authority {authority} is not authorized to issue {license_type}")]
    UnauthorizedAuthority {
        authority: String,
        license_type: String,
    },

    /// Duplicate license (ໃບອະນຸຍາດຊ້ຳ)
    #[error("Duplicate license application: existing license {existing_number}")]
    DuplicateLicense { existing_number: String },

    /// License not found (ບໍ່ພົບໃບອະນຸຍາດ)
    #[error("License {license_number} not found")]
    NotFound { license_number: String },
}

impl LicenseError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            LicenseError::InvalidType { license_type } => {
                format!("ປະເພດໃບອະນຸຍາດບໍ່ຖືກຕ້ອງ: {}", license_type)
            }
            LicenseError::Expired {
                license_number,
                expiry_date,
            } => {
                format!("ໃບອະນຸຍາດ {} ໝົດອາຍຸແລ້ວວັນທີ {}", license_number, expiry_date)
            }
            LicenseError::Suspended {
                license_number,
                until_date,
                reason,
            } => {
                format!(
                    "ໃບອະນຸຍາດ {} ຖືກລະງັບຈົນຮອດວັນທີ {} (ເຫດຜົນ: {})",
                    license_number, until_date, reason
                )
            }
            LicenseError::Revoked {
                license_number,
                reason,
            } => {
                format!("ໃບອະນຸຍາດ {} ຖືກຖອນຄືນແລ້ວ (ເຫດຜົນ: {})", license_number, reason)
            }
            LicenseError::MissingRequirements {
                license_type,
                missing_requirements,
            } => {
                format!("ຂາດເງື່ອນໄຂສຳລັບ {}: {}", license_type, missing_requirements)
            }
            LicenseError::UnauthorizedAuthority {
                authority,
                license_type,
            } => {
                format!("ອົງການ {} ບໍ່ມີສິດອອກໃບອະນຸຍາດ {}", authority, license_type)
            }
            LicenseError::DuplicateLicense { existing_number } => {
                format!("ຄຳຮ້ອງໃບອະນຸຍາດຊ້ຳກັບ: ມີໃບອະນຸຍາດ {} ແລ້ວ", existing_number)
            }
            LicenseError::NotFound { license_number } => {
                format!("ບໍ່ພົບໃບອະນຸຍາດ {}", license_number)
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create an expired license error
    pub fn expired(
        license_number: impl Into<String>,
        expiry_date: impl Into<String>,
    ) -> AdministrativeLawError {
        LicenseError::Expired {
            license_number: license_number.into(),
            expiry_date: expiry_date.into(),
        }
        .into()
    }

    /// Create an invalid type error
    pub fn invalid_type(license_type: impl Into<String>) -> AdministrativeLawError {
        LicenseError::InvalidType {
            license_type: license_type.into(),
        }
        .into()
    }
}

// ============================================================================
// Permit Errors
// ============================================================================

/// Errors related to permits
/// ຂໍ້ຜິດພາດກ່ຽວກັບໃບຢັ້ງຢືນ
#[derive(Debug, Error)]
pub enum PermitError {
    /// Invalid permit type (ປະເພດໃບຢັ້ງຢືນບໍ່ຖືກຕ້ອງ)
    #[error("Invalid permit type: {permit_type}")]
    InvalidType { permit_type: String },

    /// Permit expired (ໃບຢັ້ງຢືນໝົດອາຍຸ)
    #[error("Permit {permit_number} expired on {expiry_date}")]
    Expired {
        permit_number: String,
        expiry_date: String,
    },

    /// Conditions not met (ເງື່ອນໄຂບໍ່ຄົບຖ້ວນ)
    #[error("Conditions not met for permit: {unmet_conditions}")]
    ConditionsNotMet { unmet_conditions: String },

    /// Missing documentation (ຂາດເອກະສານ)
    #[error("Missing required documentation: {missing_docs}")]
    MissingDocumentation { missing_docs: String },

    /// Permit revoked (ໃບຢັ້ງຢືນຖືກຖອນຄືນ)
    #[error("Permit {permit_number} has been revoked")]
    Revoked {
        permit_number: String,
        reason: String,
    },

    /// Unauthorized issuing authority (ອົງການອອກບໍ່ມີສິດ)
    #[error("Authority {authority} is not authorized to issue {permit_type}")]
    UnauthorizedAuthority {
        authority: String,
        permit_type: String,
    },

    /// Nationality requirement not met (ເງື່ອນໄຂສັນຊາດບໍ່ຖືກຕ້ອງ)
    #[error("Nationality requirement not met: {nationality}")]
    NationalityRequirementNotMet { nationality: String },

    /// Permit not found (ບໍ່ພົບໃບຢັ້ງຢືນ)
    #[error("Permit {permit_number} not found")]
    NotFound { permit_number: String },
}

impl PermitError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            PermitError::InvalidType { permit_type } => {
                format!("ປະເພດໃບຢັ້ງຢືນບໍ່ຖືກຕ້ອງ: {}", permit_type)
            }
            PermitError::Expired {
                permit_number,
                expiry_date,
            } => {
                format!("ໃບຢັ້ງຢືນ {} ໝົດອາຍຸແລ້ວວັນທີ {}", permit_number, expiry_date)
            }
            PermitError::ConditionsNotMet { unmet_conditions } => {
                format!("ເງື່ອນໄຂບໍ່ຄົບຖ້ວນສຳລັບໃບຢັ້ງຢືນ: {}", unmet_conditions)
            }
            PermitError::MissingDocumentation { missing_docs } => {
                format!("ຂາດເອກະສານທີ່ຕ້ອງການ: {}", missing_docs)
            }
            PermitError::Revoked {
                permit_number,
                reason,
            } => {
                format!("ໃບຢັ້ງຢືນ {} ຖືກຖອນຄືນແລ້ວ (ເຫດຜົນ: {})", permit_number, reason)
            }
            PermitError::UnauthorizedAuthority {
                authority,
                permit_type,
            } => {
                format!("ອົງການ {} ບໍ່ມີສິດອອກໃບຢັ້ງຢືນ {}", authority, permit_type)
            }
            PermitError::NationalityRequirementNotMet { nationality } => {
                format!("ເງື່ອນໄຂສັນຊາດບໍ່ຖືກຕ້ອງ: {}", nationality)
            }
            PermitError::NotFound { permit_number } => {
                format!("ບໍ່ພົບໃບຢັ້ງຢືນ {}", permit_number)
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a conditions not met error
    pub fn conditions_not_met(unmet_conditions: impl Into<String>) -> AdministrativeLawError {
        PermitError::ConditionsNotMet {
            unmet_conditions: unmet_conditions.into(),
        }
        .into()
    }
}

// ============================================================================
// Sanction Errors
// ============================================================================

/// Errors related to administrative sanctions
/// ຂໍ້ຜິດພາດກ່ຽວກັບການລົງໂທດບໍລິຫານ
#[derive(Debug, Error)]
pub enum SanctionError {
    /// Disproportionate sanction (ການລົງໂທດບໍ່ສົມເຫດສົມຜົນ)
    #[error("Sanction is disproportionate to violation: severity {severity} for {violation}")]
    Disproportionate { severity: u8, violation: String },

    /// Missing grounds (ຂາດເຫດຜົນ)
    #[error("Missing grounds for sanction")]
    MissingGrounds,

    /// Missing legal basis (ຂາດພື້ນຖານທາງກົດໝາຍ)
    #[error("Missing legal basis for sanction")]
    MissingLegalBasis,

    /// Unauthorized authority (ອົງການບໍ່ມີສິດ)
    #[error("Authority {authority} is not authorized to impose this sanction")]
    UnauthorizedAuthority { authority: String },

    /// Fine amount exceeds limit (ຈຳນວນເງິນປັບເກີນຂີດຈຳກັດ)
    #[error("Fine amount {amount} LAK exceeds limit {limit} LAK for authority level")]
    FineExceedsLimit { amount: u64, limit: u64 },

    /// Fine amount below minimum (ຈຳນວນເງິນປັບຕ່ຳກວ່າຂັ້ນຕ່ຳ)
    #[error("Fine amount {amount} LAK is below minimum {minimum} LAK")]
    FineBelowMinimum { amount: u64, minimum: u64 },

    /// Suspension exceeds maximum (ໄລຍະເວລາລະງັບເກີນສູງສຸດ)
    #[error("Suspension duration {days} days exceeds maximum {max_days} days")]
    SuspensionExceedsMaximum { days: u32, max_days: u32 },

    /// Invalid sanction type (ປະເພດການລົງໂທດບໍ່ຖືກຕ້ອງ)
    #[error("Invalid sanction type: {sanction_type}")]
    InvalidSanctionType { sanction_type: String },

    /// Subject not identified (ບໍ່ໄດ້ລະບຸຜູ້ຖືກລົງໂທດ)
    #[error("Subject of sanction not identified")]
    SubjectNotIdentified,

    /// Double jeopardy (ລົງໂທດສອງຄັ້ງ)
    #[error("Subject already sanctioned for same violation: previous sanction {previous_id}")]
    DoubleJeopardy { previous_id: String },
}

impl SanctionError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            SanctionError::Disproportionate {
                severity,
                violation,
            } => {
                format!(
                    "ການລົງໂທດບໍ່ສົມເຫດສົມຜົນ: ລະດັບ {} ສຳລັບການລະເມີດ {}",
                    severity, violation
                )
            }
            SanctionError::MissingGrounds => "ຂາດເຫດຜົນສຳລັບການລົງໂທດ".to_string(),
            SanctionError::MissingLegalBasis => "ຂາດພື້ນຖານທາງກົດໝາຍສຳລັບການລົງໂທດ".to_string(),
            SanctionError::UnauthorizedAuthority { authority } => {
                format!("ອົງການ {} ບໍ່ມີສິດອອກການລົງໂທດນີ້", authority)
            }
            SanctionError::FineExceedsLimit { amount, limit } => {
                format!(
                    "ຈຳນວນເງິນປັບ {} ກີບ ເກີນຂີດຈຳກັດ {} ກີບ ສຳລັບລະດັບອຳນາດ",
                    amount, limit
                )
            }
            SanctionError::FineBelowMinimum { amount, minimum } => {
                format!("ຈຳນວນເງິນປັບ {} ກີບ ຕ່ຳກວ່າຂັ້ນຕ່ຳ {} ກີບ", amount, minimum)
            }
            SanctionError::SuspensionExceedsMaximum { days, max_days } => {
                format!("ໄລຍະເວລາລະງັບ {} ວັນ ເກີນສູງສຸດ {} ວັນ", days, max_days)
            }
            SanctionError::InvalidSanctionType { sanction_type } => {
                format!("ປະເພດການລົງໂທດບໍ່ຖືກຕ້ອງ: {}", sanction_type)
            }
            SanctionError::SubjectNotIdentified => "ບໍ່ໄດ້ລະບຸຜູ້ຖືກລົງໂທດ".to_string(),
            SanctionError::DoubleJeopardy { previous_id } => {
                format!(
                    "ຜູ້ຖືກລົງໂທດໄດ້ຮັບການລົງໂທດແລ້ວສຳລັບການລະເມີດດຽວກັນ: ການລົງໂທດກ່ອນໜ້າ {}",
                    previous_id
                )
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a disproportionate error
    pub fn disproportionate(severity: u8, violation: impl Into<String>) -> AdministrativeLawError {
        SanctionError::Disproportionate {
            severity,
            violation: violation.into(),
        }
        .into()
    }

    /// Create a fine exceeds limit error
    pub fn fine_exceeds_limit(amount: u64, limit: u64) -> AdministrativeLawError {
        SanctionError::FineExceedsLimit { amount, limit }.into()
    }
}

// ============================================================================
// Appeal Errors
// ============================================================================

/// Errors related to administrative appeals
/// ຂໍ້ຜິດພາດກ່ຽວກັບການອຸທອນບໍລິຫານ
#[derive(Debug, Error)]
pub enum AppealError {
    /// Deadline missed (ໝົດກຳນົດເວລາ)
    #[error(
        "Appeal deadline missed: filed on {filed_date}, deadline was {deadline_date} ({deadline_days} days)"
    )]
    DeadlineMissed {
        filed_date: String,
        deadline_date: String,
        deadline_days: u8,
    },

    /// Wrong appeal level (ລະດັບອຸທອນບໍ່ຖືກຕ້ອງ)
    #[error("Wrong appeal level: {attempted_level}, should be {correct_level}")]
    WrongLevel {
        attempted_level: String,
        correct_level: String,
    },

    /// Insufficient grounds (ເຫດຜົນບໍ່ພຽງພໍ)
    #[error("Insufficient grounds for appeal")]
    InsufficientGrounds,

    /// Missing grounds (ຂາດເຫດຜົນ)
    #[error("No grounds specified for appeal")]
    MissingGrounds,

    /// Original decision not found (ບໍ່ພົບການຕັດສິນໃຈເດີມ)
    #[error("Original decision {decision_number} not found")]
    OriginalDecisionNotFound { decision_number: String },

    /// No standing (ບໍ່ມີສິດອຸທອນ)
    #[error("Appellant lacks standing to appeal this decision")]
    NoStanding,

    /// Appeal already filed (ໄດ້ຍື່ນອຸທອນແລ້ວ)
    #[error("Appeal already filed for decision {decision_number}")]
    AlreadyFiled { decision_number: String },

    /// Appeal already decided (ໄດ້ຕັດສິນແລ້ວ)
    #[error("Appeal {appeal_number} has already been decided")]
    AlreadyDecided { appeal_number: String },

    /// Administrative remedies not exhausted (ຍັງບໍ່ໄດ້ໃຊ້ວິທີການແກ້ໄຂທາງບໍລິຫານຄົບຖ້ວນ)
    #[error("Administrative remedies must be exhausted before court appeal")]
    RemediesNotExhausted,

    /// Missing supporting documents (ຂາດເອກະສານສະໜັບສະໜູນ)
    #[error("Missing required supporting documents")]
    MissingSupportingDocuments,
}

impl AppealError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            AppealError::DeadlineMissed {
                filed_date,
                deadline_date,
                deadline_days,
            } => {
                format!(
                    "ໝົດກຳນົດເວລາອຸທອນ: ຍື່ນວັນທີ {}, ກຳນົດເວລາແມ່ນ {} ({} ວັນ)",
                    filed_date, deadline_date, deadline_days
                )
            }
            AppealError::WrongLevel {
                attempted_level,
                correct_level,
            } => {
                format!(
                    "ລະດັບອຸທອນບໍ່ຖືກຕ້ອງ: ພະຍາຍາມ {}, ຄວນເປັນ {}",
                    attempted_level, correct_level
                )
            }
            AppealError::InsufficientGrounds => "ເຫດຜົນບໍ່ພຽງພໍສຳລັບການອຸທອນ".to_string(),
            AppealError::MissingGrounds => "ບໍ່ໄດ້ລະບຸເຫດຜົນສຳລັບການອຸທອນ".to_string(),
            AppealError::OriginalDecisionNotFound { decision_number } => {
                format!("ບໍ່ພົບການຕັດສິນໃຈເດີມ {}", decision_number)
            }
            AppealError::NoStanding => "ຜູ້ອຸທອນບໍ່ມີສິດອຸທອນການຕັດສິນໃຈນີ້".to_string(),
            AppealError::AlreadyFiled { decision_number } => {
                format!("ໄດ້ຍື່ນອຸທອນແລ້ວສຳລັບການຕັດສິນໃຈ {}", decision_number)
            }
            AppealError::AlreadyDecided { appeal_number } => {
                format!("ການອຸທອນ {} ໄດ້ຖືກຕັດສິນແລ້ວ", appeal_number)
            }
            AppealError::RemediesNotExhausted => {
                "ຕ້ອງໃຊ້ວິທີການແກ້ໄຂທາງບໍລິຫານຄົບຖ້ວນກ່ອນຟ້ອງຕໍ່ສານ".to_string()
            }
            AppealError::MissingSupportingDocuments => "ຂາດເອກະສານສະໜັບສະໜູນທີ່ຕ້ອງການ".to_string(),
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a deadline missed error
    pub fn deadline_missed(
        filed_date: impl Into<String>,
        deadline_date: impl Into<String>,
        deadline_days: u8,
    ) -> AdministrativeLawError {
        AppealError::DeadlineMissed {
            filed_date: filed_date.into(),
            deadline_date: deadline_date.into(),
            deadline_days,
        }
        .into()
    }

    /// Create a wrong level error
    pub fn wrong_level(
        attempted_level: impl Into<String>,
        correct_level: impl Into<String>,
    ) -> AdministrativeLawError {
        AppealError::WrongLevel {
            attempted_level: attempted_level.into(),
            correct_level: correct_level.into(),
        }
        .into()
    }
}

// ============================================================================
// State Liability Errors
// ============================================================================

/// Errors related to state liability claims
/// ຂໍ້ຜິດພາດກ່ຽວກັບການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ
#[derive(Debug, Error)]
pub enum StateLiabilityError {
    /// Claim deadline exceeded (ໝົດກຳນົດເວລາການຮ້ອງຂໍ)
    #[error(
        "State liability claim deadline exceeded: {years_elapsed} years since wrongful act (limit: {limit_years} years)"
    )]
    ClaimDeadlineExceeded { years_elapsed: u8, limit_years: u8 },

    /// Insufficient evidence (ພະຍານບໍ່ພຽງພໍ)
    #[error("Insufficient evidence to support claim: {details}")]
    InsufficientEvidence { details: String },

    /// Causation not established (ບໍ່ໄດ້ພິສູດສາເຫດ)
    #[error("Causal link between wrongful act and damage not established")]
    CausationNotEstablished,

    /// Amount exceeds reasonable limits (ຈຳນວນເງິນເກີນຂອບເຂດ)
    #[error("Claimed amount {claimed} LAK exceeds reasonable limits")]
    AmountExceedsLimits { claimed: u64 },

    /// Missing wrongful act description (ຂາດລາຍລະອຽດການກະທຳຜິດ)
    #[error("Missing description of wrongful act")]
    MissingWrongfulActDescription,

    /// Missing damage description (ຂາດລາຍລະອຽດຄວາມເສຍຫາຍ)
    #[error("Missing description of damage")]
    MissingDamageDescription,

    /// Invalid liability type (ປະເພດຄວາມຮັບຜິດຊອບບໍ່ຖືກຕ້ອງ)
    #[error("Invalid liability type: {liability_type}")]
    InvalidLiabilityType { liability_type: String },

    /// No responsible authority identified (ບໍ່ໄດ້ລະບຸອົງການຮັບຜິດຊອບ)
    #[error("No responsible authority identified for claim")]
    NoResponsibleAuthority,

    /// Claim already filed (ໄດ້ຍື່ນຄຳຮ້ອງແລ້ວ)
    #[error("Claim already filed for this wrongful act: {existing_claim}")]
    AlreadyFiled { existing_claim: String },
}

impl StateLiabilityError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            StateLiabilityError::ClaimDeadlineExceeded {
                years_elapsed,
                limit_years,
            } => {
                format!(
                    "ໝົດກຳນົດເວລາການຮ້ອງຂໍຄ່າເສຍຫາຍ: {} ປີ ນັບແຕ່ການກະທຳຜິດ (ກຳນົດ: {} ປີ)",
                    years_elapsed, limit_years
                )
            }
            StateLiabilityError::InsufficientEvidence { details } => {
                format!("ພະຍານບໍ່ພຽງພໍສຳລັບຄຳຮ້ອງ: {}", details)
            }
            StateLiabilityError::CausationNotEstablished => {
                "ບໍ່ໄດ້ພິສູດຄວາມສຳພັນລະຫວ່າງການກະທຳຜິດ ແລະ ຄວາມເສຍຫາຍ".to_string()
            }
            StateLiabilityError::AmountExceedsLimits { claimed } => {
                format!("ຈຳນວນເງິນທີ່ຮ້ອງຂໍ {} ກີບ ເກີນຂອບເຂດທີ່ສົມເຫດສົມຜົນ", claimed)
            }
            StateLiabilityError::MissingWrongfulActDescription => {
                "ຂາດລາຍລະອຽດການກະທຳຜິດ".to_string()
            }
            StateLiabilityError::MissingDamageDescription => "ຂາດລາຍລະອຽດຄວາມເສຍຫາຍ".to_string(),
            StateLiabilityError::InvalidLiabilityType { liability_type } => {
                format!("ປະເພດຄວາມຮັບຜິດຊອບບໍ່ຖືກຕ້ອງ: {}", liability_type)
            }
            StateLiabilityError::NoResponsibleAuthority => "ບໍ່ໄດ້ລະບຸອົງການທີ່ຮັບຜິດຊອບ".to_string(),
            StateLiabilityError::AlreadyFiled { existing_claim } => {
                format!("ໄດ້ຍື່ນຄຳຮ້ອງແລ້ວສຳລັບການກະທຳຜິດນີ້: {}", existing_claim)
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a claim deadline exceeded error
    pub fn claim_deadline_exceeded(years_elapsed: u8) -> AdministrativeLawError {
        StateLiabilityError::ClaimDeadlineExceeded {
            years_elapsed,
            limit_years: STATE_LIABILITY_CLAIM_DEADLINE_YEARS,
        }
        .into()
    }

    /// Create an insufficient evidence error
    pub fn insufficient_evidence(details: impl Into<String>) -> AdministrativeLawError {
        StateLiabilityError::InsufficientEvidence {
            details: details.into(),
        }
        .into()
    }
}

// ============================================================================
// Notification Errors
// ============================================================================

/// Errors related to notification requirements
/// ຂໍ້ຜິດພາດກ່ຽວກັບການແຈ້ງ
#[derive(Debug, Error)]
pub enum NotificationError {
    /// Party not notified (ບໍ່ໄດ້ແຈ້ງຝ່າຍ)
    #[error("Party {party_name} was not properly notified")]
    PartyNotNotified { party_name: String },

    /// Invalid notification (ການແຈ້ງບໍ່ຖືກຕ້ອງ)
    #[error("Invalid notification: {reason}")]
    InvalidNotification { reason: String },

    /// Notification too late (ການແຈ້ງຊ້າເກີນໄປ)
    #[error("Notification was {days_late} days late")]
    NotificationLate { days_late: u32 },

    /// Missing notification date (ຂາດວັນທີແຈ້ງ)
    #[error("Missing notification date for party {party_name}")]
    MissingNotificationDate { party_name: String },

    /// Wrong notification method (ວິທີການແຈ້ງບໍ່ຖືກຕ້ອງ)
    #[error("Wrong notification method used: {method}")]
    WrongMethod { method: String },

    /// Incomplete notification (ການແຈ້ງບໍ່ຄົບຖ້ວນ)
    #[error("Incomplete notification: {missing_elements}")]
    IncompleteNotification { missing_elements: String },

    /// Notification not acknowledged (ການແຈ້ງບໍ່ໄດ້ຮັບການຢືນຢັນ)
    #[error("Notification to {party_name} was not acknowledged")]
    NotAcknowledged { party_name: String },
}

impl NotificationError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            NotificationError::PartyNotNotified { party_name } => {
                format!("ບໍ່ໄດ້ແຈ້ງ {} ຢ່າງຖືກຕ້ອງ", party_name)
            }
            NotificationError::InvalidNotification { reason } => {
                format!("ການແຈ້ງບໍ່ຖືກຕ້ອງ: {}", reason)
            }
            NotificationError::NotificationLate { days_late } => {
                format!("ການແຈ້ງຊ້າໄປ {} ວັນ", days_late)
            }
            NotificationError::MissingNotificationDate { party_name } => {
                format!("ຂາດວັນທີແຈ້ງສຳລັບ {}", party_name)
            }
            NotificationError::WrongMethod { method } => {
                format!("ວິທີການແຈ້ງບໍ່ຖືກຕ້ອງ: {}", method)
            }
            NotificationError::IncompleteNotification { missing_elements } => {
                format!("ການແຈ້ງບໍ່ຄົບຖ້ວນ: {}", missing_elements)
            }
            NotificationError::NotAcknowledged { party_name } => {
                format!("ການແຈ້ງໃຫ້ {} ບໍ່ໄດ້ຮັບການຢືນຢັນ", party_name)
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a party not notified error
    pub fn party_not_notified(party_name: impl Into<String>) -> AdministrativeLawError {
        NotificationError::PartyNotNotified {
            party_name: party_name.into(),
        }
        .into()
    }

    /// Create an invalid notification error
    pub fn invalid_notification(reason: impl Into<String>) -> AdministrativeLawError {
        NotificationError::InvalidNotification {
            reason: reason.into(),
        }
        .into()
    }
}

// ============================================================================
// Authority Errors
// ============================================================================

/// Errors related to administrative authority
/// ຂໍ້ຜິດພາດກ່ຽວກັບອຳນາດບໍລິຫານ
#[derive(Debug, Error)]
pub enum AuthorityError {
    /// Invalid authority level (ລະດັບອຳນາດບໍ່ຖືກຕ້ອງ)
    #[error("Invalid authority level: {level}")]
    InvalidLevel { level: String },

    /// Jurisdiction exceeded (ເກີນຂອບເຂດອຳນາດ)
    #[error("Authority {authority} exceeded jurisdiction: {reason}")]
    JurisdictionExceeded { authority: String, reason: String },

    /// No jurisdiction (ບໍ່ມີອຳນາດ)
    #[error("Authority {authority} has no jurisdiction over this matter")]
    NoJurisdiction { authority: String },

    /// Conflict of interest (ຜົນປະໂຫຍດທັບຊ້ອນ)
    #[error("Conflict of interest: {details}")]
    ConflictOfInterest { details: String },

    /// Authority not found (ບໍ່ພົບອົງການ)
    #[error("Authority {authority} not found")]
    NotFound { authority: String },

    /// Authority level too low (ລະດັບອຳນາດຕ່ຳເກີນໄປ)
    #[error("Authority level {current} is too low, requires {required}")]
    LevelTooLow { current: String, required: String },

    /// Action requires superior authority (ຕ້ອງການອົງການຂັ້ນເທິງ)
    #[error("This action requires approval from superior authority")]
    RequiresSuperiorApproval,
}

impl AuthorityError {
    /// Get the error message in Lao
    pub fn message_lao(&self) -> String {
        match self {
            AuthorityError::InvalidLevel { level } => {
                format!("ລະດັບອຳນາດບໍ່ຖືກຕ້ອງ: {}", level)
            }
            AuthorityError::JurisdictionExceeded { authority, reason } => {
                format!("ອົງການ {} ເກີນຂອບເຂດອຳນາດ: {}", authority, reason)
            }
            AuthorityError::NoJurisdiction { authority } => {
                format!("ອົງການ {} ບໍ່ມີອຳນາດໃນເລື່ອງນີ້", authority)
            }
            AuthorityError::ConflictOfInterest { details } => {
                format!("ຜົນປະໂຫຍດທັບຊ້ອນ: {}", details)
            }
            AuthorityError::NotFound { authority } => {
                format!("ບໍ່ພົບອົງການ {}", authority)
            }
            AuthorityError::LevelTooLow { current, required } => {
                format!("ລະດັບອຳນາດ {} ຕ່ຳເກີນໄປ, ຕ້ອງການລະດັບ {}", current, required)
            }
            AuthorityError::RequiresSuperiorApproval => {
                "ການດຳເນີນການນີ້ຕ້ອງການການອະນຸມັດຈາກອົງການຂັ້ນເທິງ".to_string()
            }
        }
    }

    /// Get the error message in English
    pub fn message_en(&self) -> String {
        format!("{}", self)
    }

    /// Create a jurisdiction exceeded error
    pub fn jurisdiction_exceeded(
        authority: impl Into<String>,
        reason: impl Into<String>,
    ) -> AdministrativeLawError {
        AuthorityError::JurisdictionExceeded {
            authority: authority.into(),
            reason: reason.into(),
        }
        .into()
    }

    /// Create a level too low error
    pub fn level_too_low(
        current: impl Into<String>,
        required: impl Into<String>,
    ) -> AdministrativeLawError {
        AuthorityError::LevelTooLow {
            current: current.into(),
            required: required.into(),
        }
        .into()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::administrative_law::types::ADMINISTRATIVE_APPEAL_DEADLINE_DAYS;

    #[test]
    fn test_bilingual_error_messages() {
        let error = AdministrativeDecisionError::MissingLegalBasis;
        let admin_error: AdministrativeLawError = error.into();

        let lao = admin_error.message_lao();
        let en = admin_error.message_en();

        assert!(lao.contains("ຂາດພື້ນຖານທາງກົດໝາຍ"));
        assert!(en.contains("Missing legal basis"));
    }

    #[test]
    fn test_license_error_messages() {
        let error = LicenseError::Expired {
            license_number: "BL-2024-001".to_string(),
            expiry_date: "2024-01-01".to_string(),
        };

        let lao = error.message_lao();
        let en = error.message_en();

        assert!(lao.contains("BL-2024-001"));
        assert!(lao.contains("ໝົດອາຍຸ"));
        assert!(en.contains("expired"));
    }

    #[test]
    fn test_appeal_error_deadline_missed() {
        let error = AppealError::deadline_missed(
            "2024-03-01",
            "2024-02-15",
            ADMINISTRATIVE_APPEAL_DEADLINE_DAYS,
        );

        let (lao, en) = error.bilingual_message();

        assert!(lao.contains("ໝົດກຳນົດເວລາ"));
        assert!(en.contains("deadline missed"));
    }

    #[test]
    fn test_sanction_error_disproportionate() {
        let error = SanctionError::disproportionate(5, "minor violation");

        let (lao, en) = error.bilingual_message();

        assert!(lao.contains("ບໍ່ສົມເຫດສົມຜົນ"));
        assert!(en.contains("disproportionate"));
    }

    #[test]
    fn test_state_liability_error() {
        let error = StateLiabilityError::claim_deadline_exceeded(3);

        let (lao, en) = error.bilingual_message();

        assert!(lao.contains("ໝົດກຳນົດເວລາ"));
        assert!(en.contains("deadline exceeded"));
    }

    #[test]
    fn test_notification_error() {
        let error = NotificationError::party_not_notified("John Doe");

        let (lao, en) = error.bilingual_message();

        assert!(lao.contains("ບໍ່ໄດ້ແຈ້ງ"));
        assert!(en.contains("not properly notified"));
    }

    #[test]
    fn test_authority_error() {
        let error = AuthorityError::level_too_low("District", "Provincial");

        let (lao, en) = error.bilingual_message();

        assert!(lao.contains("ຕ່ຳເກີນໄປ"));
        assert!(en.contains("too low"));
    }

    #[test]
    fn test_error_conversion() {
        let decision_error = AdministrativeDecisionError::MissingLegalBasis;
        let admin_error: AdministrativeLawError = decision_error.into();

        match admin_error {
            AdministrativeLawError::DecisionError(_) => {}
            _ => panic!("Expected DecisionError"),
        }
    }
}
