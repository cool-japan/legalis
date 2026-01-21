//! Anti-Corruption Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ
//!
//! Comprehensive error types for Anti-Corruption Law validation and processing.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - Article references from Anti-Corruption Law 2012 (as amended 2019)
//! - Structured error categories
//!
//! # Error Categories
//!
//! - **OffenseError**: Corruption offense-related errors
//! - **AssetDeclarationError**: Asset declaration errors
//! - **InvestigationError**: Investigation procedure errors
//! - **PenaltyError**: Penalty determination errors
//! - **WhistleblowerError**: Whistleblower protection errors
//! - **PreventionError**: Prevention measure errors
//! - **InternationalCooperationError**: International cooperation errors

use thiserror::Error;

/// Result type for anti-corruption law operations
pub type AntiCorruptionLawResult<T> = std::result::Result<T, AntiCorruptionLawError>;

/// Main anti-corruption law error type - ປະເພດຄວາມຜິດພາດກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ
///
/// Comprehensive error type covering all aspects of anti-corruption law validation.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AntiCorruptionLawError {
    // ========================================================================
    // Corruption Offense Errors - ຄວາມຜິດພາດການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
    // ========================================================================
    /// Invalid corruption offense - ການກະທຳຜິດສໍ້ລາດບັງຫຼວງບໍ່ຖືກຕ້ອງ
    #[error("Invalid corruption offense (Art. {article}): {message_en} / {message_lao}")]
    InvalidCorruptionOffense {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Bribery offense error - ຄວາມຜິດພາດການໃຫ້/ຮັບສິນບົນ (Articles 25-27)
    #[error("Bribery offense error (Art. {article}): {message_en} / {message_lao}")]
    BriberyOffense {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Embezzlement offense error - ຄວາມຜິດພາດການສໍ້ໂກງ (Articles 28-31)
    #[error("Embezzlement offense error (Art. {article}): {message_en} / {message_lao}")]
    EmbezzlementOffense {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Abuse of position error - ຄວາມຜິດພາດການໃຊ້ຕຳແໜ່ງໃນທາງທີ່ຜິດ (Articles 32-34)
    #[error("Abuse of position error (Art. {article}): {message_en} / {message_lao}")]
    AbuseOfPosition {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Nepotism error - ຄວາມຜິດພາດການລຳອຽງເພາະຍາດພີ່ນ້ອງ (Articles 35-37)
    #[error("Nepotism error (Art. {article}): {message_en} / {message_lao}")]
    Nepotism {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Conflict of interest error - ຄວາມຜິດພາດຜົນປະໂຫຍດຂັດກັນ (Articles 38-41)
    #[error("Conflict of interest error (Art. {article}): {message_en} / {message_lao}")]
    ConflictOfInterest {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Illicit enrichment error - ຄວາມຜິດພາດການຮັ່ງມີທີ່ບໍ່ຊອບດ້ວຍກົດໝາຍ (Articles 42-45)
    #[error("Illicit enrichment error (Art. {article}): {message_en} / {message_lao}")]
    IllicitEnrichment {
        article: u32,
        message_lao: String,
        message_en: String,
        unexplained_wealth_lak: u64,
    },

    // ========================================================================
    // Asset Declaration Errors - ຄວາມຜິດພາດການປະກາດຊັບສິນ
    // ========================================================================
    /// Invalid asset declaration - ການປະກາດຊັບສິນບໍ່ຖືກຕ້ອງ (Articles 50-60)
    #[error("Invalid asset declaration (Art. {article}): {message_en} / {message_lao}")]
    InvalidAssetDeclaration {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Missing required declaration - ການປະກາດຊັບສິນບໍ່ຄົບຖ້ວນ (Article 50)
    #[error(
        "Missing required declaration (Art. 50): Official of grade {grade} must declare / ພະນັກງານລະດັບ {grade} ຕ້ອງປະກາດ"
    )]
    MissingDeclaration {
        grade: u8,
        message_lao: String,
        message_en: String,
    },

    /// Late declaration - ການປະກາດຊ້າ (Article 52)
    #[error(
        "Late declaration (Art. 52): Submitted {days_late} days after deadline / ຍື່ນຊ້າ {days_late} ວັນຫຼັງກຳນົດ"
    )]
    LateDeclaration {
        days_late: u32,
        message_lao: String,
        message_en: String,
    },

    /// Incomplete declaration - ການປະກາດບໍ່ຄົບຖ້ວນ (Article 55)
    #[error("Incomplete declaration (Art. 55): {message_en} / {message_lao}")]
    IncompleteDeclaration {
        missing_fields: Vec<String>,
        message_lao: String,
        message_en: String,
    },

    /// False declaration - ການປະກາດບໍ່ຖືກຕ້ອງ (Article 58)
    #[error("False declaration (Art. 58): {message_en} / {message_lao}")]
    FalseDeclaration {
        message_lao: String,
        message_en: String,
        discrepancy_amount_lak: Option<u64>,
    },

    // ========================================================================
    // Investigation Errors - ຄວາມຜິດພາດການສືບສວນ
    // ========================================================================
    /// Invalid investigation - ການສືບສວນບໍ່ຖືກຕ້ອງ (Articles 10-15)
    #[error("Invalid investigation (Art. {article}): {message_en} / {message_lao}")]
    InvalidInvestigation {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Investigation timeline exceeded - ເກີນກຳນົດເວລາສືບສວນ (Article 12)
    #[error(
        "Investigation timeline exceeded (Art. 12): {days} days exceeds {max_days} days / {days} ວັນເກີນກຳນົດ {max_days} ວັນ"
    )]
    InvestigationTimelineExceeded {
        days: u32,
        max_days: u32,
        message_lao: String,
        message_en: String,
    },

    /// Insufficient evidence - ຫຼັກຖານບໍ່ພຽງພໍ (Article 14)
    #[error("Insufficient evidence (Art. 14): {message_en} / {message_lao}")]
    InsufficientEvidence {
        message_lao: String,
        message_en: String,
    },

    /// Jurisdiction error - ຄວາມຜິດພາດຂອບເຂດອຳນາດ (Article 10)
    #[error("Jurisdiction error (Art. 10): {message_en} / {message_lao}")]
    JurisdictionError {
        message_lao: String,
        message_en: String,
    },

    // ========================================================================
    // Penalty Errors - ຄວາມຜິດພາດການລົງໂທດ
    // ========================================================================
    /// Invalid penalty - ການລົງໂທດບໍ່ຖືກຕ້ອງ (Articles 65-80)
    #[error("Invalid penalty (Art. {article}): {message_en} / {message_lao}")]
    InvalidPenalty {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Penalty exceeds maximum - ການລົງໂທດເກີນສູງສຸດ (Article 68)
    #[error(
        "Penalty exceeds maximum (Art. 68): {years} years exceeds max {max_years} years / {years} ປີເກີນ {max_years} ປີ"
    )]
    PenaltyExceedsMaximum {
        years: u32,
        max_years: u32,
        message_lao: String,
        message_en: String,
    },

    /// Penalty below minimum - ການລົງໂທດຕ່ຳກວ່າຕ່ຳສຸດ (Article 65)
    #[error(
        "Penalty below minimum (Art. 65): {months} months below min {min_months} months / {months} ເດືອນຕ່ຳກວ່າ {min_months} ເດືອນ"
    )]
    PenaltyBelowMinimum {
        months: u32,
        min_months: u32,
        message_lao: String,
        message_en: String,
    },

    /// Disproportionate penalty - ການລົງໂທດບໍ່ສົມດູນ (Article 70)
    #[error("Disproportionate penalty (Art. 70): {message_en} / {message_lao}")]
    DisproportionatePenalty {
        message_lao: String,
        message_en: String,
        severity: String,
        amount_lak: u64,
    },

    // ========================================================================
    // Whistleblower Errors - ຄວາມຜິດພາດຜູ້ແຈ້ງຂ່າວ
    // ========================================================================
    /// Invalid whistleblower report - ການແຈ້ງຂ່າວບໍ່ຖືກຕ້ອງ (Articles 85-95)
    #[error("Invalid whistleblower report (Art. {article}): {message_en} / {message_lao}")]
    InvalidWhistleblowerReport {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Insufficient report details - ລາຍລະອຽດການແຈ້ງຂ່າວບໍ່ພຽງພໍ (Article 87)
    #[error("Insufficient report details (Art. 87): {message_en} / {message_lao}")]
    InsufficientReportDetails {
        missing_elements: Vec<String>,
        message_lao: String,
        message_en: String,
    },

    /// Retaliation detected - ກວດພົບການແກ້ແຄ້ນ (Article 88)
    #[error("Retaliation detected (Art. 88): {message_en} / {message_lao}")]
    RetaliationDetected {
        retaliation_type: String,
        message_lao: String,
        message_en: String,
    },

    /// Invalid reward calculation - ການຄຳນວນລາງວັນບໍ່ຖືກຕ້ອງ (Article 90)
    #[error("Invalid reward calculation (Art. 90): {message_en} / {message_lao}")]
    InvalidRewardCalculation {
        calculated_amount_lak: u64,
        message_lao: String,
        message_en: String,
    },

    // ========================================================================
    // Prevention Errors - ຄວາມຜິດພາດການປ້ອງກັນ
    // ========================================================================
    /// Code of conduct violation - ການລະເມີດລະບຽບພຶດຕິກຳ (Article 100)
    #[error("Code of conduct violation (Art. 100): {message_en} / {message_lao}")]
    CodeOfConductViolation {
        violation_type: String,
        message_lao: String,
        message_en: String,
    },

    /// Gift limit exceeded - ຂອງຂວັນເກີນກຳນົດ (Article 108)
    #[error(
        "Gift limit exceeded (Art. 108): {amount_lak} LAK exceeds limit of {limit_lak} LAK / {amount_lak} ກີບເກີນກຳນົດ {limit_lak} ກີບ"
    )]
    GiftLimitExceeded {
        amount_lak: u64,
        limit_lak: u64,
        message_lao: String,
        message_en: String,
    },

    /// Cooling-off period violation - ການລະເມີດໄລຍະຫ່າງ (Article 112)
    #[error(
        "Cooling-off period violation (Art. 112): {years_since_leaving} years is less than required {required_years} years / {years_since_leaving} ປີໜ້ອຍກວ່າ {required_years} ປີທີ່ຕ້ອງການ"
    )]
    CoolingOffPeriodViolation {
        years_since_leaving: u32,
        required_years: u32,
        message_lao: String,
        message_en: String,
    },

    /// Procurement transparency violation - ການລະເມີດຄວາມໂປ່ງໃສໃນການຈັດຊື້ (Article 105)
    #[error("Procurement transparency violation (Art. 105): {message_en} / {message_lao}")]
    ProcurementTransparencyViolation {
        message_lao: String,
        message_en: String,
    },

    // ========================================================================
    // International Cooperation Errors - ຄວາມຜິດພາດການຮ່ວມມືສາກົນ
    // ========================================================================
    /// Invalid international cooperation request - ຄຳຮ້ອງຮ່ວມມືສາກົນບໍ່ຖືກຕ້ອງ (Articles 120-135)
    #[error(
        "Invalid international cooperation request (Art. {article}): {message_en} / {message_lao}"
    )]
    InvalidInternationalCooperation {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// UNCAC compliance error - ຄວາມຜິດພາດການປະຕິບັດຕາມ UNCAC (Article 120)
    #[error("UNCAC compliance error (Art. 120): {message_en} / {message_lao}")]
    UncacComplianceError {
        uncac_article: String,
        message_lao: String,
        message_en: String,
    },

    /// Asset recovery error - ຄວາມຜິດພາດການຂໍຊັບຄືນ (Article 128)
    #[error("Asset recovery error (Art. 128): {message_en} / {message_lao}")]
    AssetRecoveryError {
        foreign_jurisdiction: String,
        amount_lak: u64,
        message_lao: String,
        message_en: String,
    },

    /// Mutual legal assistance error - ຄວາມຜິດພາດການຊ່ວຍເຫຼືອທາງກົດໝາຍ (Article 125)
    #[error("Mutual legal assistance error (Art. 125): {message_en} / {message_lao}")]
    MutualLegalAssistanceError {
        requesting_country: String,
        message_lao: String,
        message_en: String,
    },

    // ========================================================================
    // SIA Errors - ຄວາມຜິດພາດ ອົງການກວດກາແຫ່ງລັດ
    // ========================================================================
    /// SIA jurisdiction error - ຄວາມຜິດພາດຂອບເຂດອຳນາດ ອກລ (Article 8)
    #[error("SIA jurisdiction error (Art. 8): {message_en} / {message_lao}")]
    SiaJurisdictionError {
        message_lao: String,
        message_en: String,
    },

    /// SIA power exceeded - ເກີນອຳນາດ ອກລ (Article 10)
    #[error("SIA power exceeded (Art. 10): {message_en} / {message_lao}")]
    SiaPowerExceeded {
        power_type: String,
        message_lao: String,
        message_en: String,
    },

    // ========================================================================
    // General Errors - ຄວາມຜິດພາດທົ່ວໄປ
    // ========================================================================
    /// Validation error - ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message_en} / {message_lao}")]
    ValidationError {
        message_lao: String,
        message_en: String,
    },

    /// Official not covered - ພະນັກງານບໍ່ຢູ່ໃນຂອບເຂດ (Article 5)
    #[error("Official not covered (Art. 5): {message_en} / {message_lao}")]
    OfficialNotCovered {
        official_type: String,
        message_lao: String,
        message_en: String,
    },
}

impl AntiCorruptionLawError {
    /// Get the English error message
    /// ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> String {
        match self {
            AntiCorruptionLawError::InvalidCorruptionOffense { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::BriberyOffense { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::EmbezzlementOffense { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::AbuseOfPosition { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::Nepotism { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::ConflictOfInterest { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::IllicitEnrichment { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::InvalidAssetDeclaration { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::MissingDeclaration { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::LateDeclaration { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::IncompleteDeclaration { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::FalseDeclaration { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::InvalidInvestigation { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::InvestigationTimelineExceeded { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::InsufficientEvidence { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::JurisdictionError { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::InvalidPenalty { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::PenaltyExceedsMaximum { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::PenaltyBelowMinimum { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::DisproportionatePenalty { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::InvalidWhistleblowerReport { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::InsufficientReportDetails { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::RetaliationDetected { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::InvalidRewardCalculation { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::CodeOfConductViolation { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::GiftLimitExceeded { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::CoolingOffPeriodViolation { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::ProcurementTransparencyViolation { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::InvalidInternationalCooperation { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::UncacComplianceError { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::AssetRecoveryError { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::MutualLegalAssistanceError { message_en, .. } => {
                message_en.clone()
            }
            AntiCorruptionLawError::SiaJurisdictionError { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::SiaPowerExceeded { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::ValidationError { message_en, .. } => message_en.clone(),
            AntiCorruptionLawError::OfficialNotCovered { message_en, .. } => message_en.clone(),
        }
    }

    /// Get the Lao error message
    /// ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> String {
        match self {
            AntiCorruptionLawError::InvalidCorruptionOffense { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::BriberyOffense { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::EmbezzlementOffense { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::AbuseOfPosition { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::Nepotism { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::ConflictOfInterest { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::IllicitEnrichment { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::InvalidAssetDeclaration { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::MissingDeclaration { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::LateDeclaration { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::IncompleteDeclaration { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::FalseDeclaration { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::InvalidInvestigation { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::InvestigationTimelineExceeded { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::InsufficientEvidence { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::JurisdictionError { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::InvalidPenalty { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::PenaltyExceedsMaximum { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::PenaltyBelowMinimum { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::DisproportionatePenalty { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::InvalidWhistleblowerReport { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::InsufficientReportDetails { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::RetaliationDetected { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::InvalidRewardCalculation { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::CodeOfConductViolation { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::GiftLimitExceeded { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::CoolingOffPeriodViolation { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::ProcurementTransparencyViolation { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::InvalidInternationalCooperation { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::UncacComplianceError { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::AssetRecoveryError { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::MutualLegalAssistanceError { message_lao, .. } => {
                message_lao.clone()
            }
            AntiCorruptionLawError::SiaJurisdictionError { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::SiaPowerExceeded { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::ValidationError { message_lao, .. } => message_lao.clone(),
            AntiCorruptionLawError::OfficialNotCovered { message_lao, .. } => message_lao.clone(),
        }
    }

    /// Get the article reference if available
    /// ໄດ້ເລກມາດຕາອ້າງອີງຖ້າມີ
    pub fn article_reference(&self) -> Option<u32> {
        match self {
            AntiCorruptionLawError::InvalidCorruptionOffense { article, .. } => Some(*article),
            AntiCorruptionLawError::BriberyOffense { article, .. } => Some(*article),
            AntiCorruptionLawError::EmbezzlementOffense { article, .. } => Some(*article),
            AntiCorruptionLawError::AbuseOfPosition { article, .. } => Some(*article),
            AntiCorruptionLawError::Nepotism { article, .. } => Some(*article),
            AntiCorruptionLawError::ConflictOfInterest { article, .. } => Some(*article),
            AntiCorruptionLawError::IllicitEnrichment { article, .. } => Some(*article),
            AntiCorruptionLawError::InvalidAssetDeclaration { article, .. } => Some(*article),
            AntiCorruptionLawError::MissingDeclaration { .. } => Some(50),
            AntiCorruptionLawError::LateDeclaration { .. } => Some(52),
            AntiCorruptionLawError::IncompleteDeclaration { .. } => Some(55),
            AntiCorruptionLawError::FalseDeclaration { .. } => Some(58),
            AntiCorruptionLawError::InvalidInvestigation { article, .. } => Some(*article),
            AntiCorruptionLawError::InvestigationTimelineExceeded { .. } => Some(12),
            AntiCorruptionLawError::InsufficientEvidence { .. } => Some(14),
            AntiCorruptionLawError::JurisdictionError { .. } => Some(10),
            AntiCorruptionLawError::InvalidPenalty { article, .. } => Some(*article),
            AntiCorruptionLawError::PenaltyExceedsMaximum { .. } => Some(68),
            AntiCorruptionLawError::PenaltyBelowMinimum { .. } => Some(65),
            AntiCorruptionLawError::DisproportionatePenalty { .. } => Some(70),
            AntiCorruptionLawError::InvalidWhistleblowerReport { article, .. } => Some(*article),
            AntiCorruptionLawError::InsufficientReportDetails { .. } => Some(87),
            AntiCorruptionLawError::RetaliationDetected { .. } => Some(88),
            AntiCorruptionLawError::InvalidRewardCalculation { .. } => Some(90),
            AntiCorruptionLawError::CodeOfConductViolation { .. } => Some(100),
            AntiCorruptionLawError::GiftLimitExceeded { .. } => Some(108),
            AntiCorruptionLawError::CoolingOffPeriodViolation { .. } => Some(112),
            AntiCorruptionLawError::ProcurementTransparencyViolation { .. } => Some(105),
            AntiCorruptionLawError::InvalidInternationalCooperation { article, .. } => {
                Some(*article)
            }
            AntiCorruptionLawError::UncacComplianceError { .. } => Some(120),
            AntiCorruptionLawError::AssetRecoveryError { .. } => Some(128),
            AntiCorruptionLawError::MutualLegalAssistanceError { .. } => Some(125),
            AntiCorruptionLawError::SiaJurisdictionError { .. } => Some(8),
            AntiCorruptionLawError::SiaPowerExceeded { .. } => Some(10),
            AntiCorruptionLawError::ValidationError { .. } => None,
            AntiCorruptionLawError::OfficialNotCovered { .. } => Some(5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anti_corruption_error_display() {
        let error = AntiCorruptionLawError::BriberyOffense {
            article: 25,
            message_lao: "ການຮັບສິນບົນເກີນ 5 ລ້ານກີບ".to_string(),
            message_en: "Bribery amount exceeds 5 million LAK".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Art. 25"));
        assert!(display.contains("Bribery"));
    }

    #[test]
    fn test_english_message() {
        let error = AntiCorruptionLawError::GiftLimitExceeded {
            amount_lak: 600_000,
            limit_lak: 500_000,
            message_lao: "ຂອງຂວັນເກີນກຳນົດ".to_string(),
            message_en: "Gift exceeds maximum allowed limit".to_string(),
        };
        assert_eq!(
            error.english_message(),
            "Gift exceeds maximum allowed limit"
        );
    }

    #[test]
    fn test_lao_message() {
        let error = AntiCorruptionLawError::GiftLimitExceeded {
            amount_lak: 600_000,
            limit_lak: 500_000,
            message_lao: "ຂອງຂວັນເກີນກຳນົດ".to_string(),
            message_en: "Gift exceeds maximum allowed limit".to_string(),
        };
        assert_eq!(error.lao_message(), "ຂອງຂວັນເກີນກຳນົດ");
    }

    #[test]
    fn test_article_reference() {
        let error = AntiCorruptionLawError::CoolingOffPeriodViolation {
            years_since_leaving: 1,
            required_years: 2,
            message_lao: "ໄລຍະຫ່າງບໍ່ພຽງພໍ".to_string(),
            message_en: "Cooling-off period not satisfied".to_string(),
        };
        assert_eq!(error.article_reference(), Some(112));
    }

    #[test]
    fn test_illicit_enrichment_error() {
        let error = AntiCorruptionLawError::IllicitEnrichment {
            article: 42,
            message_lao: "ຊັບສິນເພີ່ມຂຶ້ນໂດຍບໍ່ສາມາດອະທິບາຍໄດ້".to_string(),
            message_en: "Unexplained increase in wealth".to_string(),
            unexplained_wealth_lak: 100_000_000,
        };
        assert!(format!("{}", error).contains("Art. 42"));
    }

    #[test]
    fn test_investigation_timeline_error() {
        let error = AntiCorruptionLawError::InvestigationTimelineExceeded {
            days: 100,
            max_days: 90,
            message_lao: "ເກີນກຳນົດເວລາສືບສວນ".to_string(),
            message_en: "Investigation timeline exceeded".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("100"));
        assert!(display.contains("90"));
    }
}
