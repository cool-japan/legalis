//! Consumer Protection Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍປົກປ້ອງຜູ້ບໍລິໂພກ
//!
//! Comprehensive error types for Lao consumer protection law validation.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - A `legal_basis()` reference to the governing statute by name and year
//! - A textual `provision` descriptor instead of fabricated article numbers
//!
//! # Legal Basis
//!
//! These errors are grounded in the **Law on Consumer Protection (Lao PDR),
//! No. 02/NA, 30 June 2010** (ກົດໝາຍວ່າດ້ວຍການປົກປ້ອງຜູ້ບໍລິໂພກ). Because the
//! crate cannot independently verify every internal article number of that law,
//! each error references the chapter/provision *topic* (a documented descriptor)
//! rather than asserting a specific article integer.

use thiserror::Error;

/// Canonical citation for the Lao Law on Consumer Protection.
/// ການອ້າງອີງກົດໝາຍວ່າດ້ວຍການປົກປ້ອງຜູ້ບໍລິໂພກ
pub const CONSUMER_PROTECTION_LAW_CITATION: &str =
    "Law on Consumer Protection (Lao PDR), No. 02/NA, 2010";

/// Result type for consumer protection law operations.
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານກົດໝາຍປົກປ້ອງຜູ້ບໍລິໂພກ
pub type ConsumerProtectionResult<T> = std::result::Result<T, ConsumerProtectionError>;

/// Main consumer protection law error type.
/// ປະເພດຄວາມຜິດພາດກົດໝາຍປົກປ້ອງຜູ້ບໍລິໂພກ
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConsumerProtectionError {
    /// Invalid product label - ສະຫຼາກສິນຄ້າບໍ່ຖືກຕ້ອງ
    #[error("Invalid product label [{provision}]: {message_en} / {message_lao}")]
    InvalidProductLabel {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Required Lao-language labelling is missing - ຂາດສະຫຼາກພາສາລາວ
    #[error(
        "Missing Lao-language label: {message_en} / {message_lao} (per {CONSUMER_PROTECTION_LAW_CITATION})"
    )]
    MissingLaoLanguageLabel {
        message_lao: String,
        message_en: String,
    },

    /// Prohibited (false or misleading) advertising - ການໂຄສະນາທີ່ຖືກຫ້າມ
    #[error("Prohibited advertising [{practice}]: {message_en} / {message_lao}")]
    ProhibitedAdvertising {
        practice: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Unfair contract term - ຂໍ້ກຳນົດສັນຍາທີ່ບໍ່ເປັນທຳ
    #[error("Unfair contract term: {message_en} / {message_lao}")]
    UnfairContractTerm {
        message_lao: String,
        message_en: String,
    },

    /// Unsafe product placed on the market - ສິນຄ້າທີ່ບໍ່ປອດໄພ
    #[error("Unsafe product: {message_en} / {message_lao}")]
    UnsafeProduct {
        message_lao: String,
        message_en: String,
    },

    /// Defective product - ສິນຄ້າທີ່ມີຂໍ້ບົກພ່ອງ
    #[error("Defective product: {message_en} / {message_lao}")]
    DefectiveProduct {
        message_lao: String,
        message_en: String,
    },

    /// Invalid consumer complaint - ຄຳຮ້ອງທຸກຂອງຜູ້ບໍລິໂພກບໍ່ຖືກຕ້ອງ
    #[error("Invalid consumer complaint [{provision}]: {message_en} / {message_lao}")]
    InvalidComplaint {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Invalid or inadequate redress - ການແກ້ໄຂ/ຊົດເຊີຍບໍ່ຖືກຕ້ອງ
    #[error("Invalid redress: {message_en} / {message_lao}")]
    InvalidRedress {
        message_lao: String,
        message_en: String,
    },

    /// Improper dispute escalation order - ການຍົກລະດັບຂໍ້ຂັດແຍ່ງບໍ່ຖືກຕາມຂັ້ນຕອນ
    #[error("Improper dispute escalation: {message_en} / {message_lao}")]
    ImproperDisputeEscalation {
        message_lao: String,
        message_en: String,
    },

    /// Consumer fundamental right violated - ການລະເມີດສິດຂັ້ນພື້ນຖານຂອງຜູ້ບໍລິໂພກ
    #[error("Consumer right violation [{right}]: {message_en} / {message_lao}")]
    ConsumerRightViolation {
        right: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Supplier obligation breached - ການລະເມີດພັນທະຂອງຜູ້ສະໜອງ
    #[error("Supplier obligation breach [{obligation}]: {message_en} / {message_lao}")]
    SupplierObligationBreach {
        obligation: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Invalid product recall - ການເກັບຄືນສິນຄ້າບໍ່ຖືກຕ້ອງ
    #[error("Invalid product recall: {message_en} / {message_lao}")]
    InvalidRecall {
        message_lao: String,
        message_en: String,
    },

    /// Invalid administrative sanction - ການລົງໂທດທາງບໍລິຫານບໍ່ຖືກຕ້ອງ
    #[error("Invalid sanction: {message_en} / {message_lao}")]
    InvalidSanction {
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

impl ConsumerProtectionError {
    /// Get the English error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> &str {
        match self {
            ConsumerProtectionError::InvalidProductLabel { message_en, .. }
            | ConsumerProtectionError::MissingLaoLanguageLabel { message_en, .. }
            | ConsumerProtectionError::ProhibitedAdvertising { message_en, .. }
            | ConsumerProtectionError::UnfairContractTerm { message_en, .. }
            | ConsumerProtectionError::UnsafeProduct { message_en, .. }
            | ConsumerProtectionError::DefectiveProduct { message_en, .. }
            | ConsumerProtectionError::InvalidComplaint { message_en, .. }
            | ConsumerProtectionError::InvalidRedress { message_en, .. }
            | ConsumerProtectionError::ImproperDisputeEscalation { message_en, .. }
            | ConsumerProtectionError::ConsumerRightViolation { message_en, .. }
            | ConsumerProtectionError::SupplierObligationBreach { message_en, .. }
            | ConsumerProtectionError::InvalidRecall { message_en, .. }
            | ConsumerProtectionError::InvalidSanction { message_en, .. }
            | ConsumerProtectionError::ValidationError { message_en, .. } => message_en,
        }
    }

    /// Get the Lao error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> &str {
        match self {
            ConsumerProtectionError::InvalidProductLabel { message_lao, .. }
            | ConsumerProtectionError::MissingLaoLanguageLabel { message_lao, .. }
            | ConsumerProtectionError::ProhibitedAdvertising { message_lao, .. }
            | ConsumerProtectionError::UnfairContractTerm { message_lao, .. }
            | ConsumerProtectionError::UnsafeProduct { message_lao, .. }
            | ConsumerProtectionError::DefectiveProduct { message_lao, .. }
            | ConsumerProtectionError::InvalidComplaint { message_lao, .. }
            | ConsumerProtectionError::InvalidRedress { message_lao, .. }
            | ConsumerProtectionError::ImproperDisputeEscalation { message_lao, .. }
            | ConsumerProtectionError::ConsumerRightViolation { message_lao, .. }
            | ConsumerProtectionError::SupplierObligationBreach { message_lao, .. }
            | ConsumerProtectionError::InvalidRecall { message_lao, .. }
            | ConsumerProtectionError::InvalidSanction { message_lao, .. }
            | ConsumerProtectionError::ValidationError { message_lao, .. } => message_lao,
        }
    }

    /// Get the governing statute citation - ໄດ້ການອ້າງອີງກົດໝາຍ
    pub fn legal_basis(&self) -> &'static str {
        CONSUMER_PROTECTION_LAW_CITATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_contains_messages() {
        let error = ConsumerProtectionError::UnsafeProduct {
            message_lao: "ສິນຄ້າບໍ່ປອດໄພ".to_string(),
            message_en: "Product fails the safety standard".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Product fails the safety standard"));
        assert!(display.contains("ສິນຄ້າບໍ່ປອດໄພ"));
    }

    #[test]
    fn test_english_and_lao_message() {
        let error = ConsumerProtectionError::ConsumerRightViolation {
            right: "right to safety",
            message_lao: "ລະເມີດສິດຄວາມປອດໄພ".to_string(),
            message_en: "Right to safety violated".to_string(),
        };
        assert_eq!(error.english_message(), "Right to safety violated");
        assert_eq!(error.lao_message(), "ລະເມີດສິດຄວາມປອດໄພ");
    }

    #[test]
    fn test_legal_basis_is_consumer_protection_law() {
        let error = ConsumerProtectionError::UnfairContractTerm {
            message_lao: "ຂໍ້ກຳນົດບໍ່ເປັນທຳ".to_string(),
            message_en: "Unfair term".to_string(),
        };
        assert_eq!(error.legal_basis(), CONSUMER_PROTECTION_LAW_CITATION);
        assert!(error.legal_basis().contains("02/NA"));
        assert!(error.legal_basis().contains("2010"));
    }

    #[test]
    fn test_provision_descriptor_in_display() {
        let error = ConsumerProtectionError::ProhibitedAdvertising {
            practice: "false advertising",
            message_lao: "ການໂຄສະນາຕົວະ".to_string(),
            message_en: "False advertising".to_string(),
        };
        assert!(format!("{}", error).contains("false advertising"));
    }
}
