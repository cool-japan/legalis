//! Commercial Law Error Types
//!
//! This module defines error types for Lao commercial law operations,
//! including enterprise formation, investment approval, and IP registration.
//!
//! All errors provide bilingual messages (Lao/English) for better accessibility.

use thiserror::Error;

/// Commercial law error types with bilingual support
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CommercialLawError {
    /// Enterprise formation errors
    #[error("Invalid enterprise formation: {en} / ການສ້າງວິສາຫະກິດບໍ່ຖືກຕ້ອງ: {lo}")]
    InvalidEnterpriseFormation { en: String, lo: String },

    /// Insufficient capital errors
    #[error("Insufficient capital: {en} / ທຶນບໍ່ພຽງພໍ: {lo}")]
    InsufficientCapital { en: String, lo: String },

    /// Invalid business name
    #[error("Invalid business name: {en} / ຊື່ທຸລະກິດບໍ່ຖືກຕ້ອງ: {lo}")]
    InvalidBusinessName { en: String, lo: String },

    /// Board composition errors
    #[error("Invalid board composition: {en} / ອົງປະກອບຄະນະກໍາມະການບໍ່ຖືກຕ້ອງ: {lo}")]
    InvalidBoardComposition { en: String, lo: String },

    /// Foreign investment errors
    #[error("Foreign investment violation: {en} / ການລົງທຶນຕ່າງປະເທດຜິດກົດໝາຍ: {lo}")]
    ForeignInvestmentViolation { en: String, lo: String },

    /// Restricted sector errors
    #[error("Restricted sector violation: {en} / ການລະເມີດຂະແໜງຈໍາກັດ: {lo}")]
    RestrictedSectorViolation { en: String, lo: String },

    /// Investment approval errors
    #[error("Investment approval required: {en} / ຕ້ອງການການອະນຸມັດການລົງທຶນ: {lo}")]
    InvestmentApprovalRequired { en: String, lo: String },

    /// Intellectual property errors
    #[error("Intellectual property error: {en} / ຂໍ້ຜິດພາດກ່ຽວກັບຊັບສິນທາງປັນຍາ: {lo}")]
    IntellectualPropertyError { en: String, lo: String },

    /// Registration errors
    #[error("Registration error: {en} / ຂໍ້ຜິດພາດການລົງທະບຽນ: {lo}")]
    RegistrationError { en: String, lo: String },

    /// License requirement errors
    #[error("License required: {en} / ຕ້ອງການໃບອະນຸຍາດ: {lo}")]
    LicenseRequired { en: String, lo: String },

    /// Shareholder errors
    #[error("Shareholder error: {en} / ຂໍ້ຜິດພາດຜູ້ຖືຮຸ້ນ: {lo}")]
    ShareholderError { en: String, lo: String },

    /// Corporate governance errors
    #[error("Corporate governance violation: {en} / ການລະເມີດການຄຸ້ມຄອງວິສາຫະກິດ: {lo}")]
    CorporateGovernanceViolation { en: String, lo: String },

    /// Investment incentive errors
    #[error("Investment incentive error: {en} / ຂໍ້ຜິດພາດການຊຸກຍູ້ການລົງທຶນ: {lo}")]
    InvestmentIncentiveError { en: String, lo: String },

    /// Concession errors
    #[error("Concession error: {en} / ຂໍ້ຜິດພາດການສໍາປະທານ: {lo}")]
    ConcessionError { en: String, lo: String },

    /// Partnership errors
    #[error("Partnership error: {en} / ຂໍ້ຜິດພາດຫ້າງຫຸ້ນສ່ວນ: {lo}")]
    PartnershipError { en: String, lo: String },
}

/// Result type for commercial law operations
pub type Result<T> = std::result::Result<T, CommercialLawError>;

impl CommercialLawError {
    /// Create an invalid enterprise formation error
    pub fn invalid_enterprise_formation(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::InvalidEnterpriseFormation {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create an insufficient capital error
    pub fn insufficient_capital(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::InsufficientCapital {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create an invalid business name error
    pub fn invalid_business_name(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::InvalidBusinessName {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create an invalid board composition error
    pub fn invalid_board_composition(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::InvalidBoardComposition {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a foreign investment violation error
    pub fn foreign_investment_violation(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::ForeignInvestmentViolation {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a restricted sector violation error
    pub fn restricted_sector_violation(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::RestrictedSectorViolation {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create an investment approval required error
    pub fn investment_approval_required(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::InvestmentApprovalRequired {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create an intellectual property error
    pub fn intellectual_property_error(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::IntellectualPropertyError {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a registration error
    pub fn registration_error(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::RegistrationError {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a license required error
    pub fn license_required(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::LicenseRequired {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a shareholder error
    pub fn shareholder_error(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::ShareholderError {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a corporate governance violation error
    pub fn corporate_governance_violation(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::CorporateGovernanceViolation {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create an investment incentive error
    pub fn investment_incentive_error(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::InvestmentIncentiveError {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a concession error
    pub fn concession_error(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::ConcessionError {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Create a partnership error
    pub fn partnership_error(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::PartnershipError {
            en: en.into(),
            lo: lo.into(),
        }
    }

    /// Get the English error message
    pub fn message_en(&self) -> String {
        match self {
            Self::InvalidEnterpriseFormation { en, .. } => en.clone(),
            Self::InsufficientCapital { en, .. } => en.clone(),
            Self::InvalidBusinessName { en, .. } => en.clone(),
            Self::InvalidBoardComposition { en, .. } => en.clone(),
            Self::ForeignInvestmentViolation { en, .. } => en.clone(),
            Self::RestrictedSectorViolation { en, .. } => en.clone(),
            Self::InvestmentApprovalRequired { en, .. } => en.clone(),
            Self::IntellectualPropertyError { en, .. } => en.clone(),
            Self::RegistrationError { en, .. } => en.clone(),
            Self::LicenseRequired { en, .. } => en.clone(),
            Self::ShareholderError { en, .. } => en.clone(),
            Self::CorporateGovernanceViolation { en, .. } => en.clone(),
            Self::InvestmentIncentiveError { en, .. } => en.clone(),
            Self::ConcessionError { en, .. } => en.clone(),
            Self::PartnershipError { en, .. } => en.clone(),
        }
    }

    /// Get the Lao error message
    pub fn message_lo(&self) -> String {
        match self {
            Self::InvalidEnterpriseFormation { lo, .. } => lo.clone(),
            Self::InsufficientCapital { lo, .. } => lo.clone(),
            Self::InvalidBusinessName { lo, .. } => lo.clone(),
            Self::InvalidBoardComposition { lo, .. } => lo.clone(),
            Self::ForeignInvestmentViolation { lo, .. } => lo.clone(),
            Self::RestrictedSectorViolation { lo, .. } => lo.clone(),
            Self::InvestmentApprovalRequired { lo, .. } => lo.clone(),
            Self::IntellectualPropertyError { lo, .. } => lo.clone(),
            Self::RegistrationError { lo, .. } => lo.clone(),
            Self::LicenseRequired { lo, .. } => lo.clone(),
            Self::ShareholderError { lo, .. } => lo.clone(),
            Self::CorporateGovernanceViolation { lo, .. } => lo.clone(),
            Self::InvestmentIncentiveError { lo, .. } => lo.clone(),
            Self::ConcessionError { lo, .. } => lo.clone(),
            Self::PartnershipError { lo, .. } => lo.clone(),
        }
    }

    /// Get both English and Lao messages as a tuple
    pub fn messages(&self) -> (String, String) {
        (self.message_en(), self.message_lo())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_enterprise_formation() {
        let err = CommercialLawError::invalid_enterprise_formation(
            "Missing required documents",
            "ຂາດເອກະສານທີ່ຕ້ອງການ",
        );
        assert_eq!(err.message_en(), "Missing required documents");
        assert_eq!(err.message_lo(), "ຂາດເອກະສານທີ່ຕ້ອງການ");
    }

    #[test]
    fn test_insufficient_capital() {
        let err = CommercialLawError::insufficient_capital(
            "Capital below minimum requirement of 100,000,000 LAK",
            "ທຶນຕ່ໍາກວ່າຄວາມຕ້ອງການຂັ້ນຕ່ໍາ 100,000,000 ກີບ",
        );
        assert_eq!(
            err.message_en(),
            "Capital below minimum requirement of 100,000,000 LAK"
        );
        assert_eq!(err.message_lo(), "ທຶນຕ່ໍາກວ່າຄວາມຕ້ອງການຂັ້ນຕ່ໍາ 100,000,000 ກີບ");
    }

    #[test]
    fn test_foreign_investment_violation() {
        let err = CommercialLawError::foreign_investment_violation(
            "Foreign ownership exceeds 49% in restricted sector",
            "ການຖືຫຸ້ນຂອງຕ່າງປະເທດເກີນ 49% ໃນຂະແໜງຈໍາກັດ",
        );
        assert!(err.message_en().contains("Foreign ownership"));
        assert!(err.message_lo().contains("ຕ່າງປະເທດ"));
    }

    #[test]
    fn test_restricted_sector_violation() {
        let err = CommercialLawError::restricted_sector_violation(
            "This sector requires special approval",
            "ຂະແໜງນີ້ຕ້ອງການການອະນຸມັດພິເສດ",
        );
        let (en, lo) = err.messages();
        assert_eq!(en, "This sector requires special approval");
        assert_eq!(lo, "ຂະແໜງນີ້ຕ້ອງການການອະນຸມັດພິເສດ");
    }

    #[test]
    fn test_invalid_board_composition() {
        let err = CommercialLawError::invalid_board_composition(
            "Board must have at least 3 directors",
            "ຄະນະກໍາມະການຕ້ອງມີຢ່າງໜ້ອຍ 3 ກໍາມະການ",
        );
        assert!(err.message_en().contains("3 directors"));
        assert!(err.message_lo().contains("3 ກໍາມະການ"));
    }

    #[test]
    fn test_intellectual_property_error() {
        let err = CommercialLawError::intellectual_property_error(
            "Patent application incomplete",
            "ການສະຫມັກສິດທິບັດບໍ່ສົມບູນ",
        );
        assert_eq!(err.message_en(), "Patent application incomplete");
        assert_eq!(err.message_lo(), "ການສະຫມັກສິດທິບັດບໍ່ສົມບູນ");
    }

    #[test]
    fn test_error_display() {
        let err = CommercialLawError::invalid_business_name("Name already taken", "ຊື່ຖືກໃຊ້ແລ້ວ");
        let display = format!("{}", err);
        assert!(display.contains("Name already taken"));
        assert!(display.contains("ຊື່ຖືກໃຊ້ແລ້ວ"));
    }

    #[test]
    fn test_error_cloning() {
        let err1 = CommercialLawError::shareholder_error("Insufficient shares", "ຮຸ້ນບໍ່ພຽງພໍ");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_license_required() {
        let err = CommercialLawError::license_required(
            "Business license required for this activity",
            "ຕ້ອງການໃບອະນຸຍາດທຸລະກິດສໍາລັບກິດຈະກໍານີ້",
        );
        assert!(err.message_en().contains("Business license"));
        assert!(err.message_lo().contains("ໃບອະນຸຍາດທຸລະກິດ"));
    }

    #[test]
    fn test_corporate_governance_violation() {
        let err = CommercialLawError::corporate_governance_violation(
            "Annual general meeting not held",
            "ບໍ່ໄດ້ຈັດກອງປະຊຸມໃຫຍ່ປະຈໍາປີ",
        );
        assert!(err.message_en().contains("Annual general meeting"));
        assert!(err.message_lo().contains("ກອງປະຊຸມໃຫຍ່ປະຈໍາປີ"));
    }

    #[test]
    fn test_investment_incentive_error() {
        let err = CommercialLawError::investment_incentive_error(
            "Does not qualify for tax exemption",
            "ບໍ່ມີສິດໄດ້ຮັບການຍົກເວັ້ນພາສີ",
        );
        assert!(err.message_en().contains("tax exemption"));
        assert!(err.message_lo().contains("ຍົກເວັ້ນພາສີ"));
    }
}
