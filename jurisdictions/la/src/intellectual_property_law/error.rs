//! Intellectual Property Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍຊັບສິນທາງປັນຍາ
//!
//! Comprehensive error types for Lao intellectual property law validation.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - A `legal_basis()` reference to the governing statute by name and year
//! - A textual `provision` descriptor instead of fabricated article numbers
//!
//! # Legal Basis
//!
//! These errors are grounded in the **Law on Intellectual Property (Lao PDR),
//! No. 38/NA, 2017** (ກົດໝາຍວ່າດ້ວຍຊັບສິນທາງປັນຍາ) — the consolidated/amended IP
//! Law (originally No. 01/NA 2011, amended 2017). Lao PDR is a WTO member (and so
//! bound by TRIPS) and a party to the Paris and Berne Conventions and the PCT, so
//! the protection terms it implements are well established. Because the crate
//! cannot independently verify every internal article number of that law, each
//! error references the provision *topic* (a documented descriptor) rather than
//! asserting a specific article integer.

use thiserror::Error;

/// Canonical citation for the Lao Law on Intellectual Property.
/// ການອ້າງອີງກົດໝາຍວ່າດ້ວຍຊັບສິນທາງປັນຍາ
pub const IP_LAW_CITATION: &str = "Law on Intellectual Property (Lao PDR), No. 38/NA, 2017";

/// Result type for intellectual property law operations.
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານກົດໝາຍຊັບສິນທາງປັນຍາ
pub type IpResult<T> = std::result::Result<T, IpLawError>;

/// Main intellectual property law error type.
/// ປະເພດຄວາມຜິດພາດກົດໝາຍຊັບສິນທາງປັນຍາ
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum IpLawError {
    /// Invention fails a patentability requirement - ການປະດິດບໍ່ເຂົ້າເງື່ອນໄຂສິດທິບັດ
    #[error("Not patentable [{provision}]: {message_en} / {message_lao}")]
    NotPatentable {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Mark fails a registrability requirement - ເຄື່ອງໝາຍບໍ່ສາມາດຈົດທະບຽນໄດ້
    #[error("Trademark not registrable [{provision}]: {message_en} / {message_lao}")]
    TrademarkNotRegistrable {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Trademark registration term has lapsed without renewal - ເຄື່ອງໝາຍໝົດອາຍຸ
    #[error("Trademark registration lapsed: {message_en} / {message_lao} (per {IP_LAW_CITATION})")]
    TrademarkLapsed {
        message_lao: String,
        message_en: String,
    },

    /// Work does not attract copyright protection - ຜົນງານບໍ່ໄດ້ຮັບການປົກປ້ອງລິຂະສິດ
    #[error("Copyright not protected [{provision}]: {message_en} / {message_lao}")]
    CopyrightNotProtected {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Copyright term has expired into the public domain - ລິຂະສິດໝົດອາຍຸ
    #[error("Copyright expired: {message_en} / {message_lao}")]
    CopyrightExpired {
        message_lao: String,
        message_en: String,
    },

    /// Information fails a trade-secret protection criterion - ຄວາມລັບທາງການຄ້າບໍ່ສົມບູນ
    #[error("Invalid trade secret [{provision}]: {message_en} / {message_lao}")]
    InvalidTradeSecret {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Industrial design lacks novelty/originality - ແບບອຸດສາຫະກຳບໍ່ໃໝ່
    #[error("Industrial design not new: {message_en} / {message_lao}")]
    IndustrialDesignNotNew {
        message_lao: String,
        message_en: String,
    },

    /// Geographical indication lacks a qualifying origin link - ສິ່ງບົ່ງຊີ້ບໍ່ຖືກຕ້ອງ
    #[error("Invalid geographical indication [{provision}]: {message_en} / {message_lao}")]
    InvalidGeographicalIndication {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Plant variety fails the DUS + novelty requirements - ພັນພືດບໍ່ເຂົ້າເງື່ອນໄຂ
    #[error("Plant variety requirements not met [{provision}]: {message_en} / {message_lao}")]
    PlantVarietyRequirementsNotMet {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Patent term has expired - ສິດທິບັດໝົດອາຍຸ
    #[error("Patent expired: {message_en} / {message_lao}")]
    PatentExpired {
        message_lao: String,
        message_en: String,
    },

    /// Unauthorised use of a protected IP right - ການລະເມີດສິດຊັບສິນທາງປັນຍາ
    #[error("IP infringement [{right}]: {message_en} / {message_lao}")]
    IpInfringement {
        right: &'static str,
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

impl IpLawError {
    /// Get the English error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> &str {
        match self {
            IpLawError::NotPatentable { message_en, .. }
            | IpLawError::TrademarkNotRegistrable { message_en, .. }
            | IpLawError::TrademarkLapsed { message_en, .. }
            | IpLawError::CopyrightNotProtected { message_en, .. }
            | IpLawError::CopyrightExpired { message_en, .. }
            | IpLawError::InvalidTradeSecret { message_en, .. }
            | IpLawError::IndustrialDesignNotNew { message_en, .. }
            | IpLawError::InvalidGeographicalIndication { message_en, .. }
            | IpLawError::PlantVarietyRequirementsNotMet { message_en, .. }
            | IpLawError::PatentExpired { message_en, .. }
            | IpLawError::IpInfringement { message_en, .. }
            | IpLawError::ValidationError { message_en, .. } => message_en,
        }
    }

    /// Get the Lao error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> &str {
        match self {
            IpLawError::NotPatentable { message_lao, .. }
            | IpLawError::TrademarkNotRegistrable { message_lao, .. }
            | IpLawError::TrademarkLapsed { message_lao, .. }
            | IpLawError::CopyrightNotProtected { message_lao, .. }
            | IpLawError::CopyrightExpired { message_lao, .. }
            | IpLawError::InvalidTradeSecret { message_lao, .. }
            | IpLawError::IndustrialDesignNotNew { message_lao, .. }
            | IpLawError::InvalidGeographicalIndication { message_lao, .. }
            | IpLawError::PlantVarietyRequirementsNotMet { message_lao, .. }
            | IpLawError::PatentExpired { message_lao, .. }
            | IpLawError::IpInfringement { message_lao, .. }
            | IpLawError::ValidationError { message_lao, .. } => message_lao,
        }
    }

    /// Get the governing statute citation - ໄດ້ການອ້າງອີງກົດໝາຍ
    pub fn legal_basis(&self) -> &'static str {
        IP_LAW_CITATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_contains_messages() {
        let error = IpLawError::PatentExpired {
            message_lao: "ສິດທິບັດໝົດອາຍຸແລ້ວ".to_string(),
            message_en: "The patent term of protection has expired".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("The patent term of protection has expired"));
        assert!(display.contains("ສິດທິບັດໝົດອາຍຸແລ້ວ"));
    }

    #[test]
    fn test_english_and_lao_message() {
        let error = IpLawError::CopyrightNotProtected {
            provision: "originality requirement",
            message_lao: "ຜົນງານບໍ່ມີຄວາມເປັນຕົ້ນສະບັບ".to_string(),
            message_en: "Work is not original".to_string(),
        };
        assert_eq!(error.english_message(), "Work is not original");
        assert_eq!(error.lao_message(), "ຜົນງານບໍ່ມີຄວາມເປັນຕົ້ນສະບັບ");
    }

    #[test]
    fn test_legal_basis_is_ip_law() {
        let error = IpLawError::ValidationError {
            message_lao: "ຄ່າທີ່ປ້ອນບໍ່ຖືກຕ້ອງ".to_string(),
            message_en: "Invalid input".to_string(),
        };
        assert_eq!(error.legal_basis(), IP_LAW_CITATION);
        assert!(error.legal_basis().contains("38/NA"));
        assert!(error.legal_basis().contains("2017"));
    }

    #[test]
    fn test_provision_descriptor_in_display() {
        let error = IpLawError::NotPatentable {
            provision: "novelty requirement",
            message_lao: "ການປະດິດບໍ່ມີຄວາມໃໝ່".to_string(),
            message_en: "The invention lacks novelty".to_string(),
        };
        assert!(format!("{}", error).contains("novelty requirement"));
    }

    #[test]
    fn test_infringement_right_descriptor_in_display() {
        let error = IpLawError::IpInfringement {
            right: "patent",
            message_lao: "ການນຳໃຊ້ໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ".to_string(),
            message_en: "Unauthorised use of a patented invention".to_string(),
        };
        assert!(format!("{}", error).contains("patent"));
        assert!(format!("{}", error).contains("Unauthorised use of a patented invention"));
    }
}
