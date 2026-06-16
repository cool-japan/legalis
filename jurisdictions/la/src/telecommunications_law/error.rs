//! Telecommunications Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍໂທລະຄົມມະນາຄົມ
//!
//! Comprehensive error types for Lao telecommunications law validation.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - A `legal_basis()` reference to the governing statute by name and year
//! - A textual `provision` descriptor instead of fabricated article numbers
//!
//! # Legal Basis
//!
//! These errors are grounded in the **Law on Telecommunications (Lao PDR),
//! No. 09/NA, 2011** (ກົດໝາຍວ່າດ້ວຍໂທລະຄົມມະນາຄົມ). The law number (No. 09/NA,
//! 2011) is recorded as it appears in the available sources. Because the crate
//! cannot independently verify every internal article number of that law, each
//! error references the chapter/provision *topic* (a documented descriptor)
//! rather than asserting a specific article integer.

use thiserror::Error;

/// Canonical citation for the Lao Law on Telecommunications.
/// ການອ້າງອີງກົດໝາຍວ່າດ້ວຍໂທລະຄົມມະນາຄົມ
///
/// The law number is per available sources; where exact internal article
/// numbers cannot be verified, provisions are cited by this law name/year
/// together with a topic descriptor.
pub const TELECOMMUNICATIONS_LAW_CITATION: &str =
    "Law on Telecommunications (Lao PDR), No. 09/NA, 2011";

/// Result type for telecommunications law operations.
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານກົດໝາຍໂທລະຄົມມະນາຄົມ
pub type TelecommunicationsResult<T> = std::result::Result<T, TelecommunicationsLawError>;

/// Main telecommunications law error type.
/// ປະເພດຄວາມຜິດພາດກົດໝາຍໂທລະຄົມມະນາຄົມ
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TelecommunicationsLawError {
    /// Provision of services without the required licence - ດຳເນີນງານໂດຍບໍ່ມີໃບອະນຸຍາດ
    #[error("Unlicensed telecommunications operation [{provision}]: {message_en} / {message_lao}")]
    UnlicensedOperation {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Invalid telecommunications licence - ໃບອະນຸຍາດໂທລະຄົມບໍ່ຖືກຕ້ອງ
    #[error("Invalid telecommunications licence [{provision}]: {message_en} / {message_lao}")]
    InvalidLicense {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Invalid radio-frequency spectrum assignment - ການມອບຄື້ນຄວາມຖີ່ບໍ່ຖືກຕ້ອງ
    #[error("Invalid spectrum assignment [{provision}]: {message_en} / {message_lao}")]
    InvalidSpectrumAssignment {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Two exclusive spectrum assignments overlap in frequency - ຄື້ນຄວາມຖີ່ຊ້ອນກັນ
    #[error(
        "Spectrum overlap: {message_en} / {message_lao} (per {TELECOMMUNICATIONS_LAW_CITATION})"
    )]
    SpectrumOverlap {
        message_lao: String,
        message_en: String,
    },

    /// Interconnection wrongfully refused or offered on unfair terms - ການເຊື່ອມຕໍ່ຖືກປະຕິເສດ
    #[error("Interconnection refused [{provision}]: {message_en} / {message_lao}")]
    InterconnectionRefused {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Quality of service falls below the required target - ຄຸນນະພາບການບໍລິການຕ່ຳກວ່າມາດຕະຖານ
    #[error("Quality of service breach: {message_en} / {message_lao}")]
    QualityOfServiceBreach {
        message_lao: String,
        message_en: String,
    },

    /// Tariff applied without the required regulatory approval - ອັດຕາຄ່າບໍລິການບໍ່ໄດ້ຮັບການອະນຸມັດ
    #[error("Unapproved tariff: {message_en} / {message_lao}")]
    UnapprovedTariff {
        message_lao: String,
        message_en: String,
    },

    /// Equipment lacking the required type-approval - ອຸປະກອນບໍ່ໄດ້ຮັບການຮັບຮອງປະເພດ
    #[error("Equipment not type-approved: {message_en} / {message_lao}")]
    EquipmentNotApproved {
        message_lao: String,
        message_en: String,
    },

    /// Unlawful interception of communications - ການດັກຟັງການສື່ສານທີ່ຜິດກົດໝາຍ
    #[error("Unlawful interception [{provision}]: {message_en} / {message_lao}")]
    UnlawfulInterception {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Generic validation error - ຄວາມຜິດພາດການກວດສອບທົ່ວໄປ
    #[error("Validation error: {message_en} / {message_lao}")]
    ValidationError {
        message_lao: String,
        message_en: String,
    },
}

impl TelecommunicationsLawError {
    /// Get the English error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> &str {
        match self {
            TelecommunicationsLawError::UnlicensedOperation { message_en, .. }
            | TelecommunicationsLawError::InvalidLicense { message_en, .. }
            | TelecommunicationsLawError::InvalidSpectrumAssignment { message_en, .. }
            | TelecommunicationsLawError::SpectrumOverlap { message_en, .. }
            | TelecommunicationsLawError::InterconnectionRefused { message_en, .. }
            | TelecommunicationsLawError::QualityOfServiceBreach { message_en, .. }
            | TelecommunicationsLawError::UnapprovedTariff { message_en, .. }
            | TelecommunicationsLawError::EquipmentNotApproved { message_en, .. }
            | TelecommunicationsLawError::UnlawfulInterception { message_en, .. }
            | TelecommunicationsLawError::ValidationError { message_en, .. } => message_en,
        }
    }

    /// Get the Lao error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> &str {
        match self {
            TelecommunicationsLawError::UnlicensedOperation { message_lao, .. }
            | TelecommunicationsLawError::InvalidLicense { message_lao, .. }
            | TelecommunicationsLawError::InvalidSpectrumAssignment { message_lao, .. }
            | TelecommunicationsLawError::SpectrumOverlap { message_lao, .. }
            | TelecommunicationsLawError::InterconnectionRefused { message_lao, .. }
            | TelecommunicationsLawError::QualityOfServiceBreach { message_lao, .. }
            | TelecommunicationsLawError::UnapprovedTariff { message_lao, .. }
            | TelecommunicationsLawError::EquipmentNotApproved { message_lao, .. }
            | TelecommunicationsLawError::UnlawfulInterception { message_lao, .. }
            | TelecommunicationsLawError::ValidationError { message_lao, .. } => message_lao,
        }
    }

    /// Get the governing statute citation - ໄດ້ການອ້າງອີງກົດໝາຍ
    pub fn legal_basis(&self) -> &'static str {
        TELECOMMUNICATIONS_LAW_CITATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_contains_messages() {
        let error = TelecommunicationsLawError::QualityOfServiceBreach {
            message_lao: "ຄຸນນະພາບການບໍລິການຕ່ຳ".to_string(),
            message_en: "Service quality is below target".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Service quality is below target"));
        assert!(display.contains("ຄຸນນະພາບການບໍລິການຕ່ຳ"));
    }

    #[test]
    fn test_english_and_lao_message() {
        let error = TelecommunicationsLawError::InterconnectionRefused {
            provision: "interconnection between operators",
            message_lao: "ການເຊື່ອມຕໍ່ຖືກປະຕິເສດ".to_string(),
            message_en: "Interconnection was refused".to_string(),
        };
        assert_eq!(error.english_message(), "Interconnection was refused");
        assert_eq!(error.lao_message(), "ການເຊື່ອມຕໍ່ຖືກປະຕິເສດ");
    }

    #[test]
    fn test_legal_basis_is_telecommunications_law() {
        let error = TelecommunicationsLawError::UnapprovedTariff {
            message_lao: "ອັດຕາຄ່າບໍລິການບໍ່ໄດ້ຮັບການອະນຸມັດ".to_string(),
            message_en: "Tariff not approved".to_string(),
        };
        assert_eq!(error.legal_basis(), TELECOMMUNICATIONS_LAW_CITATION);
        assert!(error.legal_basis().contains("09/NA"));
        assert!(error.legal_basis().contains("2011"));
    }

    #[test]
    fn test_provision_descriptor_in_display() {
        let error = TelecommunicationsLawError::UnlicensedOperation {
            provision: "licensing of telecommunications operators",
            message_lao: "ດຳເນີນງານໂດຍບໍ່ມີໃບອະນຸຍາດ".to_string(),
            message_en: "Operating without a licence".to_string(),
        };
        assert!(format!("{}", error).contains("licensing of telecommunications operators"));
    }
}
