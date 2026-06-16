//! Insurance Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍປະກັນໄພ
//!
//! Comprehensive error types for Lao insurance law validation.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - A `legal_basis()` reference to the governing statute by name and year
//! - A textual `provision` descriptor instead of fabricated article numbers
//!
//! # Legal Basis
//!
//! These errors are grounded in the **Law on Insurance (Lao PDR), No. 06/NA,
//! 2011** (ກົດໝາຍວ່າດ້ວຍການປະກັນໄພ), administered by the Ministry of Finance.
//! Because the crate cannot independently verify every internal article number of
//! that law, each error references the chapter/provision *topic* (a documented
//! descriptor) rather than asserting a specific article integer.

use thiserror::Error;

/// Canonical citation for the Lao Law on Insurance.
/// ການອ້າງອີງກົດໝາຍວ່າດ້ວຍການປະກັນໄພ
pub const INSURANCE_LAW_CITATION: &str = "Law on Insurance (Lao PDR), No. 06/NA, 2011";

/// Result type for insurance law operations.
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານກົດໝາຍປະກັນໄພ
pub type InsuranceResult<T> = std::result::Result<T, InsuranceLawError>;

/// Main insurance law error type.
/// ປະເພດຄວາມຜິດພາດກົດໝາຍປະກັນໄພ
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InsuranceLawError {
    /// Invalid or missing insurer licence - ໃບອະນຸຍາດບໍລິສັດປະກັນໄພບໍ່ຖືກຕ້ອງ
    #[error("Invalid insurer license [{provision}]: {message_en} / {message_lao}")]
    InvalidInsurerLicense {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Insurer fails the solvency requirement - ບໍລິສັດປະກັນໄພບໍ່ມີຄວາມສາມາດຊຳລະໜີ້
    #[error("Insolvent insurer: {message_en} / {message_lao}")]
    InsolventInsurer {
        message_lao: String,
        message_en: String,
    },

    /// Invalid insurance policy - ສັນຍາປະກັນໄພບໍ່ຖືກຕ້ອງ
    #[error("Invalid insurance policy [{provision}]: {message_en} / {message_lao}")]
    InvalidPolicy {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Insurable interest is absent - ບໍ່ມີຜົນປະໂຫຍດທີ່ສາມາດເອົາປະກັນໄພໄດ້
    #[error("No insurable interest: {message_en} / {message_lao} (per {INSURANCE_LAW_CITATION})")]
    NoInsurableInterest {
        message_lao: String,
        message_en: String,
    },

    /// Invalid insurance claim - ການຮຽກຮ້ອງຄ່າສິນໄໝບໍ່ຖືກຕ້ອງ
    #[error("Invalid claim [{provision}]: {message_en} / {message_lao}")]
    InvalidClaim {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Fraudulent claim - ການຮຽກຮ້ອງຄ່າສິນໄໝທີ່ສໍ້ໂກງ
    #[error("Fraudulent claim: {message_en} / {message_lao}")]
    FraudulentClaim {
        message_lao: String,
        message_en: String,
    },

    /// Indemnity exceeds the actual loss or the sum insured - ຄ່າສິນໄໝເກີນຂອບເຂດ
    #[error("Indemnity exceeded: {message_en} / {message_lao}")]
    IndemnityExceeded {
        message_lao: String,
        message_en: String,
    },

    /// Compulsory insurance is not in place - ຂາດການປະກັນໄພທີ່ບັງຄັບ
    #[error("Compulsory insurance missing [{provision}]: {message_en} / {message_lao}")]
    CompulsoryInsuranceMissing {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Unlicensed insurance intermediary - ຕົວກາງປະກັນໄພບໍ່ມີໃບອະນຸຍາດ
    #[error("Unlicensed intermediary: {message_en} / {message_lao}")]
    UnlicensedIntermediary {
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

impl InsuranceLawError {
    /// Get the English error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> &str {
        match self {
            InsuranceLawError::InvalidInsurerLicense { message_en, .. }
            | InsuranceLawError::InsolventInsurer { message_en, .. }
            | InsuranceLawError::InvalidPolicy { message_en, .. }
            | InsuranceLawError::NoInsurableInterest { message_en, .. }
            | InsuranceLawError::InvalidClaim { message_en, .. }
            | InsuranceLawError::FraudulentClaim { message_en, .. }
            | InsuranceLawError::IndemnityExceeded { message_en, .. }
            | InsuranceLawError::CompulsoryInsuranceMissing { message_en, .. }
            | InsuranceLawError::UnlicensedIntermediary { message_en, .. }
            | InsuranceLawError::ValidationError { message_en, .. } => message_en,
        }
    }

    /// Get the Lao error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> &str {
        match self {
            InsuranceLawError::InvalidInsurerLicense { message_lao, .. }
            | InsuranceLawError::InsolventInsurer { message_lao, .. }
            | InsuranceLawError::InvalidPolicy { message_lao, .. }
            | InsuranceLawError::NoInsurableInterest { message_lao, .. }
            | InsuranceLawError::InvalidClaim { message_lao, .. }
            | InsuranceLawError::FraudulentClaim { message_lao, .. }
            | InsuranceLawError::IndemnityExceeded { message_lao, .. }
            | InsuranceLawError::CompulsoryInsuranceMissing { message_lao, .. }
            | InsuranceLawError::UnlicensedIntermediary { message_lao, .. }
            | InsuranceLawError::ValidationError { message_lao, .. } => message_lao,
        }
    }

    /// Get the governing statute citation - ໄດ້ການອ້າງອີງກົດໝາຍ
    pub fn legal_basis(&self) -> &'static str {
        INSURANCE_LAW_CITATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_contains_messages() {
        let error = InsuranceLawError::InsolventInsurer {
            message_lao: "ບໍ່ມີຄວາມສາມາດຊຳລະໜີ້".to_string(),
            message_en: "Admitted assets are below liabilities".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Admitted assets are below liabilities"));
        assert!(display.contains("ບໍ່ມີຄວາມສາມາດຊຳລະໜີ້"));
    }

    #[test]
    fn test_english_and_lao_message() {
        let error = InsuranceLawError::CompulsoryInsuranceMissing {
            provision: "compulsory motor third-party liability insurance",
            message_lao: "ຂາດການປະກັນໄພລົດທີ່ບັງຄັບ".to_string(),
            message_en: "Compulsory motor insurance is missing".to_string(),
        };
        assert_eq!(
            error.english_message(),
            "Compulsory motor insurance is missing"
        );
        assert_eq!(error.lao_message(), "ຂາດການປະກັນໄພລົດທີ່ບັງຄັບ");
    }

    #[test]
    fn test_legal_basis_is_insurance_law() {
        let error = InsuranceLawError::FraudulentClaim {
            message_lao: "ການຮຽກຮ້ອງສໍ້ໂກງ".to_string(),
            message_en: "Fraudulent claim".to_string(),
        };
        assert_eq!(error.legal_basis(), INSURANCE_LAW_CITATION);
        assert!(error.legal_basis().contains("06/NA"));
        assert!(error.legal_basis().contains("2011"));
    }

    #[test]
    fn test_provision_descriptor_in_display() {
        let error = InsuranceLawError::InvalidInsurerLicense {
            provision: "insurer licensing",
            message_lao: "ບໍ່ມີໃບອະນຸຍາດ".to_string(),
            message_en: "Insurer is not licensed".to_string(),
        };
        assert!(format!("{}", error).contains("insurer licensing"));
    }
}
