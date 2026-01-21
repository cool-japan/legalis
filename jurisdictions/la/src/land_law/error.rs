//! Error types for Land Law (ກົດໝາຍທີ່ດິນ)
//!
//! This module defines all error types related to Lao Land Law 2019 (Law No. 70/NA).
//! All errors include bilingual messages (Lao/English) for better internationalization.
//!
//! ## Legal Context
//!
//! The Land Law 2019 establishes the fundamental principle that all land in Lao PDR
//! is owned by the state (Article 3). Citizens and legal entities may only acquire
//! land use rights, not ownership. This module's error types reflect this principle.
//!
//! ## Key Principles
//!
//! - State land ownership (ທີ່ດິນເປັນຊັບສິນຂອງຊາດ)
//! - Land use rights only (ສິດນຳໃຊ້ທີ່ດິນ)
//! - Foreign ownership restrictions (ຂໍ້ຈຳກັດຕໍ່ຊາວຕ່າງປະເທດ)
//! - Registration requirements (ການລົງທະບຽນທີ່ດິນ)

use thiserror::Error;

/// Result type for land law operations
pub type Result<T> = std::result::Result<T, LandLawError>;

/// Error types for Land Law (ກົດໝາຍທີ່ດິນ)
///
/// All errors include bilingual descriptions in Lao and English.
/// The Lao text appears first, followed by the English translation.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LandLawError {
    /// Invalid land use right
    ///
    /// Lao: ສິດນຳໃຊ້ທີ່ດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid land use right
    #[error("Invalid land use right (ສິດນຳໃຊ້ທີ່ດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidLandUseRight {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid land concession
    ///
    /// Lao: ສິດສຳປະທານທີ່ດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid land concession
    #[error("Invalid land concession (ສິດສຳປະທານທີ່ດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidLandConcession {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Foreign ownership violation
    ///
    /// Lao: ການລະເມີດຂໍ້ຈຳກັດຊາວຕ່າງປະເທດ
    /// English: Foreign ownership restriction violated
    #[error("Foreign ownership violation (ການລະເມີດຂໍ້ຈຳກັດຊາວຕ່າງປະເທດ): {message_en} / {message_lo}")]
    ForeignOwnershipViolation {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid land registration
    ///
    /// Lao: ການລົງທະບຽນທີ່ດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid land registration
    #[error("Invalid land registration (ການລົງທະບຽນທີ່ດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidLandRegistration {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid land title
    ///
    /// Lao: ໃບຕາດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid land title
    #[error("Invalid land title (ໃບຕາດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidLandTitle {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid land transaction
    ///
    /// Lao: ການເຮັດທຸລະກຳທີ່ດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid land transaction
    #[error("Invalid land transaction (ການເຮັດທຸລະກຳທີ່ດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidLandTransaction {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid cadastral survey
    ///
    /// Lao: ການສຳຫຼວດທີ່ດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid cadastral survey
    #[error("Invalid cadastral survey (ການສຳຫຼວດທີ່ດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidCadastralSurvey {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Land dispute error
    ///
    /// Lao: ຂໍ້ຂັດແຍ່ງກ່ຽວກັບທີ່ດິນ
    /// English: Land dispute error
    #[error("Land dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບທີ່ດິນ): {message_en} / {message_lo}")]
    LandDispute {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// State ownership violation
    ///
    /// Lao: ການລະເມີດຫຼັກການທີ່ດິນເປັນຊັບສິນຂອງຊາດ
    /// English: State ownership principle violated
    #[error(
        "State ownership violation (ການລະເມີດຫຼັກການທີ່ດິນເປັນຊັບສິນຂອງຊາດ): {message_en} / {message_lo}"
    )]
    StateOwnershipViolation {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Land area exceeds maximum
    ///
    /// Lao: ເນື້ອທີ່ດິນເກີນກຳນົດສູງສຸດ
    /// English: Land area exceeds maximum limit
    #[error("Land area exceeds maximum (ເນື້ອທີ່ດິນເກີນກຳນົດສູງສຸດ): {message_en} / {message_lo}")]
    LandAreaExceedsMaximum {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Land use right expired
    ///
    /// Lao: ສິດນຳໃຊ້ທີ່ດິນໝົດອາຍຸແລ້ວ
    /// English: Land use right has expired
    #[error("Land use right expired (ສິດນຳໃຊ້ທີ່ດິນໝົດອາຍຸແລ້ວ): {message_en} / {message_lo}")]
    LandUseRightExpired {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Unauthorized land transfer
    ///
    /// Lao: ການໂອນທີ່ດິນໂດຍບໍ່ມີອຳນາດ
    /// English: Unauthorized land transfer
    #[error("Unauthorized land transfer (ການໂອນທີ່ດິນໂດຍບໍ່ມີອຳນາດ): {message_en} / {message_lo}")]
    UnauthorizedTransfer {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid land use purpose
    ///
    /// Lao: ຈຸດປະສົງການນຳໃຊ້ທີ່ດິນບໍ່ຖືກຕ້ອງ
    /// English: Invalid land use purpose
    #[error("Invalid land use purpose (ຈຸດປະສົງການນຳໃຊ້ທີ່ດິນບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidLandUsePurpose {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Missing required documentation
    ///
    /// Lao: ຂາດເອກະສານທີ່ຈຳເປັນ
    /// English: Missing required documentation
    #[error("Missing required documentation (ຂາດເອກະສານທີ່ຈຳເປັນ): {message_en} / {message_lo}")]
    MissingDocumentation {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Invalid concession duration
    ///
    /// Lao: ໄລຍະເວລາສຳປະທານບໍ່ຖືກຕ້ອງ
    /// English: Invalid concession duration
    #[error("Invalid concession duration (ໄລຍະເວລາສຳປະທານບໍ່ຖືກຕ້ອງ): {message_en} / {message_lo}")]
    InvalidConcessionDuration {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },

    /// Land not suitable for purpose
    ///
    /// Lao: ທີ່ດິນບໍ່ເໝາະສົມກັບຈຸດປະສົງ
    /// English: Land not suitable for intended purpose
    #[error("Land not suitable for purpose (ທີ່ດິນບໍ່ເໝາະສົມກັບຈຸດປະສົງ): {message_en} / {message_lo}")]
    LandNotSuitableForPurpose {
        /// English error message
        message_en: String,
        /// Lao error message
        message_lo: String,
    },
}

impl LandLawError {
    /// Creates an InvalidLandUseRight error with bilingual messages
    pub fn invalid_land_use_right(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidLandUseRight {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidLandConcession error with bilingual messages
    pub fn invalid_land_concession(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidLandConcession {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a ForeignOwnershipViolation error with bilingual messages
    pub fn foreign_ownership_violation(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::ForeignOwnershipViolation {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidLandRegistration error with bilingual messages
    pub fn invalid_land_registration(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidLandRegistration {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidLandTitle error with bilingual messages
    pub fn invalid_land_title(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidLandTitle {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidLandTransaction error with bilingual messages
    pub fn invalid_land_transaction(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidLandTransaction {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidCadastralSurvey error with bilingual messages
    pub fn invalid_cadastral_survey(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidCadastralSurvey {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a LandDispute error with bilingual messages
    pub fn land_dispute(message_en: impl Into<String>, message_lo: impl Into<String>) -> Self {
        Self::LandDispute {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a StateOwnershipViolation error with bilingual messages
    pub fn state_ownership_violation(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::StateOwnershipViolation {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a LandAreaExceedsMaximum error with bilingual messages
    pub fn land_area_exceeds_maximum(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::LandAreaExceedsMaximum {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a LandUseRightExpired error with bilingual messages
    pub fn land_use_right_expired(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::LandUseRightExpired {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an UnauthorizedTransfer error with bilingual messages
    pub fn unauthorized_transfer(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::UnauthorizedTransfer {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidLandUsePurpose error with bilingual messages
    pub fn invalid_land_use_purpose(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidLandUsePurpose {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a MissingDocumentation error with bilingual messages
    pub fn missing_documentation(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::MissingDocumentation {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates an InvalidConcessionDuration error with bilingual messages
    pub fn invalid_concession_duration(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::InvalidConcessionDuration {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Creates a LandNotSuitableForPurpose error with bilingual messages
    pub fn land_not_suitable_for_purpose(
        message_en: impl Into<String>,
        message_lo: impl Into<String>,
    ) -> Self {
        Self::LandNotSuitableForPurpose {
            message_en: message_en.into(),
            message_lo: message_lo.into(),
        }
    }

    /// Returns the English error message
    pub fn message_en(&self) -> &str {
        match self {
            Self::InvalidLandUseRight { message_en, .. } => message_en,
            Self::InvalidLandConcession { message_en, .. } => message_en,
            Self::ForeignOwnershipViolation { message_en, .. } => message_en,
            Self::InvalidLandRegistration { message_en, .. } => message_en,
            Self::InvalidLandTitle { message_en, .. } => message_en,
            Self::InvalidLandTransaction { message_en, .. } => message_en,
            Self::InvalidCadastralSurvey { message_en, .. } => message_en,
            Self::LandDispute { message_en, .. } => message_en,
            Self::StateOwnershipViolation { message_en, .. } => message_en,
            Self::LandAreaExceedsMaximum { message_en, .. } => message_en,
            Self::LandUseRightExpired { message_en, .. } => message_en,
            Self::UnauthorizedTransfer { message_en, .. } => message_en,
            Self::InvalidLandUsePurpose { message_en, .. } => message_en,
            Self::MissingDocumentation { message_en, .. } => message_en,
            Self::InvalidConcessionDuration { message_en, .. } => message_en,
            Self::LandNotSuitableForPurpose { message_en, .. } => message_en,
        }
    }

    /// Returns the Lao error message
    pub fn message_lo(&self) -> &str {
        match self {
            Self::InvalidLandUseRight { message_lo, .. } => message_lo,
            Self::InvalidLandConcession { message_lo, .. } => message_lo,
            Self::ForeignOwnershipViolation { message_lo, .. } => message_lo,
            Self::InvalidLandRegistration { message_lo, .. } => message_lo,
            Self::InvalidLandTitle { message_lo, .. } => message_lo,
            Self::InvalidLandTransaction { message_lo, .. } => message_lo,
            Self::InvalidCadastralSurvey { message_lo, .. } => message_lo,
            Self::LandDispute { message_lo, .. } => message_lo,
            Self::StateOwnershipViolation { message_lo, .. } => message_lo,
            Self::LandAreaExceedsMaximum { message_lo, .. } => message_lo,
            Self::LandUseRightExpired { message_lo, .. } => message_lo,
            Self::UnauthorizedTransfer { message_lo, .. } => message_lo,
            Self::InvalidLandUsePurpose { message_lo, .. } => message_lo,
            Self::MissingDocumentation { message_lo, .. } => message_lo,
            Self::InvalidConcessionDuration { message_lo, .. } => message_lo,
            Self::LandNotSuitableForPurpose { message_lo, .. } => message_lo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_land_use_right_error() {
        let error = LandLawError::invalid_land_use_right(
            "Land use right type is invalid",
            "ປະເພດສິດນຳໃຊ້ທີ່ດິນບໍ່ຖືກຕ້ອງ",
        );
        assert_eq!(error.message_en(), "Land use right type is invalid");
        assert_eq!(error.message_lo(), "ປະເພດສິດນຳໃຊ້ທີ່ດິນບໍ່ຖືກຕ້ອງ");
    }

    #[test]
    fn test_foreign_ownership_violation_error() {
        let error = LandLawError::foreign_ownership_violation(
            "Foreign nationals cannot own land",
            "ຊາວຕ່າງປະເທດບໍ່ສາມາດມີກຳມະສິດທີ່ດິນໄດ້",
        );
        assert_eq!(error.message_en(), "Foreign nationals cannot own land");
        assert_eq!(error.message_lo(), "ຊາວຕ່າງປະເທດບໍ່ສາມາດມີກຳມະສິດທີ່ດິນໄດ້");
    }

    #[test]
    fn test_state_ownership_violation_error() {
        let error = LandLawError::state_ownership_violation(
            "All land is owned by the state",
            "ທີ່ດິນທັງໝົດເປັນຊັບສິນຂອງລັດ",
        );
        assert_eq!(error.message_en(), "All land is owned by the state");
        assert_eq!(error.message_lo(), "ທີ່ດິນທັງໝົດເປັນຊັບສິນຂອງລັດ");
    }

    #[test]
    fn test_error_display() {
        let error = LandLawError::invalid_land_title(
            "Land title is missing required information",
            "ໃບຕາດິນຂາດຂໍ້ມູນທີ່ຈຳເປັນ",
        );
        let display = format!("{}", error);
        assert!(display.contains("Invalid land title"));
        assert!(display.contains("Land title is missing required information"));
    }

    #[test]
    fn test_error_clone() {
        let error = LandLawError::invalid_land_transaction(
            "Transaction requires government approval",
            "ການເຮັດທຸລະກຳຕ້ອງໄດ້ຮັບການອະນຸມັດຈາກລັດຖະບານ",
        );
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }
}
