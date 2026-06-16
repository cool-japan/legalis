//! Securities Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍຫຼັກຊັບ
//!
//! Comprehensive error types for Lao securities and capital-markets law validation.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - A `legal_basis()` reference to the governing statute by name and year
//! - A textual `provision` descriptor instead of fabricated article numbers
//!
//! # Legal Basis
//!
//! These errors are grounded in the **Law on Securities (Lao PDR), 2012**
//! (ກົດໝາຍວ່າດ້ວຍຫຼັກຊັບ). The statute governs the Lao securities market — the
//! Lao Securities Exchange (LSX), which opened in 2011 — and is administered by
//! the Lao Securities and Exchange Commission (Lao SEC). Because the crate cannot
//! independently verify every internal article number of that law, each error
//! references the provision *topic* (a documented descriptor) rather than
//! asserting a specific article integer.

use thiserror::Error;

/// Canonical citation for the Lao Law on Securities.
/// ການອ້າງອີງກົດໝາຍວ່າດ້ວຍຫຼັກຊັບ
///
/// The Law on Securities was enacted to govern the Lao securities (capital)
/// market. The Lao Securities Exchange (LSX, ຕະຫຼາດຫຼັກຊັບລາວ) opened in 2011 as
/// the market operator, and the Lao Securities and Exchange Commission (Lao SEC,
/// ຄະນະກຳມະການຄຸ້ມຄອງຫຼັກຊັບ) is the regulator. Where exact internal article
/// numbers of the law cannot be independently verified by this crate, provisions
/// are cited by this name/year citation together with a documented topic
/// descriptor rather than by fabricated article references.
pub const SECURITIES_LAW_CITATION: &str = "Law on Securities (Lao PDR), 2012";

/// Result type for securities law operations.
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານກົດໝາຍຫຼັກຊັບ
pub type SecuritiesResult<T> = std::result::Result<T, SecuritiesLawError>;

/// Main securities law error type.
/// ປະເພດຄວາມຜິດພາດກົດໝາຍຫຼັກຊັບ
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SecuritiesLawError {
    /// Invalid public offering - ການສະເໜີຂາຍຕໍ່ສາທາລະນະບໍ່ຖືກຕ້ອງ
    #[error("Invalid public offering [{provision}]: {message_en} / {message_lao}")]
    InvalidPublicOffering {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Incomplete or missing prospectus - ໜັງສືຊີ້ຊວນບໍ່ສົມບູນ
    #[error("Incomplete prospectus [{provision}]: {message_en} / {message_lao}")]
    IncompleteProspectus {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Public offering not approved by the Lao SEC - ການສະເໜີຂາຍບໍ່ໄດ້ຮັບອະນຸມັດ
    #[error("Offering not approved: {message_en} / {message_lao} (per {SECURITIES_LAW_CITATION})")]
    OfferingNotApproved {
        message_lao: String,
        message_en: String,
    },

    /// Listing requirement not met - ບໍ່ໄດ້ມາດຕະຖານການຈົດທະບຽນ
    #[error("Listing requirement not met [{provision}]: {message_en} / {message_lao}")]
    ListingRequirementNotMet {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Foreign ownership limit exceeded - ເກີນຂີດຈຳກັດການຖືຄອງຂອງຕ່າງປະເທດ
    #[error("Foreign ownership exceeded: {message_en} / {message_lao}")]
    ForeignOwnershipExceeded {
        message_lao: String,
        message_en: String,
    },

    /// Unlicensed securities company / intermediary - ບໍລິສັດຫຼັກຊັບບໍ່ມີໃບອະນຸຍາດ
    #[error("Unlicensed securities company [{provision}]: {message_en} / {message_lao}")]
    UnlicensedSecuritiesCompany {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Insider trading on material non-public information - ການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນ
    #[error("Insider trading [{provision}]: {message_en} / {message_lao}")]
    InsiderTrading {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Market manipulation - ການປັ່ນປ່ວນຕະຫຼາດ
    #[error("Market manipulation [{provision}]: {message_en} / {message_lao}")]
    MarketManipulation {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Disclosure violation - ການລະເມີດການເປີດເຜີຍຂໍ້ມູນ
    #[error("Disclosure violation [{provision}]: {message_en} / {message_lao}")]
    DisclosureViolation {
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

impl SecuritiesLawError {
    /// Get the English error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> &str {
        match self {
            SecuritiesLawError::InvalidPublicOffering { message_en, .. }
            | SecuritiesLawError::IncompleteProspectus { message_en, .. }
            | SecuritiesLawError::OfferingNotApproved { message_en, .. }
            | SecuritiesLawError::ListingRequirementNotMet { message_en, .. }
            | SecuritiesLawError::ForeignOwnershipExceeded { message_en, .. }
            | SecuritiesLawError::UnlicensedSecuritiesCompany { message_en, .. }
            | SecuritiesLawError::InsiderTrading { message_en, .. }
            | SecuritiesLawError::MarketManipulation { message_en, .. }
            | SecuritiesLawError::DisclosureViolation { message_en, .. }
            | SecuritiesLawError::ValidationError { message_en, .. } => message_en,
        }
    }

    /// Get the Lao error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> &str {
        match self {
            SecuritiesLawError::InvalidPublicOffering { message_lao, .. }
            | SecuritiesLawError::IncompleteProspectus { message_lao, .. }
            | SecuritiesLawError::OfferingNotApproved { message_lao, .. }
            | SecuritiesLawError::ListingRequirementNotMet { message_lao, .. }
            | SecuritiesLawError::ForeignOwnershipExceeded { message_lao, .. }
            | SecuritiesLawError::UnlicensedSecuritiesCompany { message_lao, .. }
            | SecuritiesLawError::InsiderTrading { message_lao, .. }
            | SecuritiesLawError::MarketManipulation { message_lao, .. }
            | SecuritiesLawError::DisclosureViolation { message_lao, .. }
            | SecuritiesLawError::ValidationError { message_lao, .. } => message_lao,
        }
    }

    /// Get the governing statute citation - ໄດ້ການອ້າງອີງກົດໝາຍ
    pub fn legal_basis(&self) -> &'static str {
        SECURITIES_LAW_CITATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_contains_messages() {
        let error = SecuritiesLawError::InsiderTrading {
            provision: "prohibited conduct — insider trading",
            message_lao: "ການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນ".to_string(),
            message_en: "Trading on material non-public information".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Trading on material non-public information"));
        assert!(display.contains("ການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນ"));
    }

    #[test]
    fn test_english_and_lao_message() {
        let error = SecuritiesLawError::ForeignOwnershipExceeded {
            message_lao: "ເກີນຂີດຈຳກັດການຖືຄອງຂອງຕ່າງປະເທດ".to_string(),
            message_en: "Foreign ownership exceeded".to_string(),
        };
        assert_eq!(error.english_message(), "Foreign ownership exceeded");
        assert_eq!(error.lao_message(), "ເກີນຂີດຈຳກັດການຖືຄອງຂອງຕ່າງປະເທດ");
    }

    #[test]
    fn test_legal_basis_is_securities_law() {
        let error = SecuritiesLawError::OfferingNotApproved {
            message_lao: "ບໍ່ໄດ້ຮັບອະນຸມັດ".to_string(),
            message_en: "Not approved".to_string(),
        };
        assert_eq!(error.legal_basis(), SECURITIES_LAW_CITATION);
        assert!(error.legal_basis().contains("Securities"));
        assert!(error.legal_basis().contains("2012"));
    }

    #[test]
    fn test_provision_descriptor_in_display() {
        let error = SecuritiesLawError::ListingRequirementNotMet {
            provision: "listing — minimum public float",
            message_lao: "ບໍ່ໄດ້ມາດຕະຖານ".to_string(),
            message_en: "Requirement not met".to_string(),
        };
        assert!(format!("{}", error).contains("listing — minimum public float"));
    }

    #[test]
    fn test_validation_error_messages() {
        let error = SecuritiesLawError::ValidationError {
            message_lao: "ຄວາມຜິດພາດການກວດສອບ".to_string(),
            message_en: "Validation failed".to_string(),
        };
        assert_eq!(error.english_message(), "Validation failed");
        assert_eq!(error.lao_message(), "ຄວາມຜິດພາດການກວດສອບ");
        assert_eq!(error.legal_basis(), SECURITIES_LAW_CITATION);
    }
}
