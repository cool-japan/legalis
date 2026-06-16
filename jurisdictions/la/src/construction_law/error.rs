//! Construction Law Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ
//!
//! Comprehensive error types for Lao construction law validation.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - A `legal_basis()` reference to the governing statute by name and year
//! - A textual `provision` descriptor instead of fabricated article numbers
//!
//! # Legal Basis
//!
//! These errors are grounded in the **Law on Construction (Lao PDR),
//! No. 05/NA, 2009** (ກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ). Because the crate cannot
//! independently verify every internal article number of that law, each error
//! references the chapter/provision *topic* (a documented descriptor) rather than
//! asserting a specific article integer.

use thiserror::Error;

/// Canonical citation for the Lao Law on Construction.
/// ການອ້າງອີງກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ
pub const CONSTRUCTION_LAW_CITATION: &str = "Law on Construction (Lao PDR), No. 05/NA, 2009";

/// Result type for construction law operations.
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ
pub type ConstructionResult<T> = std::result::Result<T, ConstructionLawError>;

/// Main construction law error type.
/// ປະເພດຄວາມຜິດພາດກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConstructionLawError {
    /// A required construction/building permit has not been issued.
    /// ໃບອະນຸຍາດກໍ່ສ້າງຍັງບໍ່ໄດ້ອອກ
    #[error(
        "Construction permit not issued [{provision}]: {message_en} / {message_lao} (per {CONSTRUCTION_LAW_CITATION})"
    )]
    PermitNotIssued {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Construction carried out without the mandatory permit.
    /// ການກໍ່ສ້າງໂດຍບໍ່ມີໃບອະນຸຍາດ
    #[error("Unpermitted construction [{provision}]: {message_en} / {message_lao}")]
    UnpermittedConstruction {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// The contractor is not licensed/registered.
    /// ຜູ້ຮັບເໝົາບໍ່ມີໃບອະນຸຍາດ
    #[error("Unlicensed contractor [{provision}]: {message_en} / {message_lao}")]
    UnlicensedContractor {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// The contractor's grade is inadequate for the project's value/scale.
    /// ຊັ້ນຄວາມສາມາດຂອງຜູ້ຮັບເໝົາບໍ່ພຽງພໍ
    #[error("Contractor grade inadequate [{provision}]: {message_en} / {message_lao}")]
    ContractorGradeInadequate {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// A required on-site safety plan is missing.
    /// ຂາດແຜນຄວາມປອດໄພໃນສະຖານທີ່ກໍ່ສ້າງ
    #[error("Missing safety plan [{provision}]: {message_en} / {message_lao}")]
    MissingSafetyPlan {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Inspections were performed out of the mandatory sequence.
    /// ການກວດກາບໍ່ຖືກຕາມລຳດັບ
    #[error("Improper inspection sequence [{provision}]: {message_en} / {message_lao}")]
    ImproperInspectionSequence {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// Occupancy was permitted before acceptance/handover requirements were met.
    /// ການເຂົ້າຢູ່ອາໄສກ່ອນການກວດຮັບ
    #[error("Premature occupancy [{provision}]: {message_en} / {message_lao}")]
    PrematureOccupancy {
        provision: &'static str,
        message_lao: String,
        message_en: String,
    },

    /// The defects-liability (warranty) period is invalid or inadequate.
    /// ໄລຍະຮັບປະກັນຄວາມເສຍຫາຍບໍ່ຖືກຕ້ອງ
    #[error("Invalid defects-liability period [{provision}]: {message_en} / {message_lao}")]
    InvalidDefectsLiability {
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

impl ConstructionLawError {
    /// Get the English error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> &str {
        match self {
            ConstructionLawError::PermitNotIssued { message_en, .. }
            | ConstructionLawError::UnpermittedConstruction { message_en, .. }
            | ConstructionLawError::UnlicensedContractor { message_en, .. }
            | ConstructionLawError::ContractorGradeInadequate { message_en, .. }
            | ConstructionLawError::MissingSafetyPlan { message_en, .. }
            | ConstructionLawError::ImproperInspectionSequence { message_en, .. }
            | ConstructionLawError::PrematureOccupancy { message_en, .. }
            | ConstructionLawError::InvalidDefectsLiability { message_en, .. }
            | ConstructionLawError::ValidationError { message_en, .. } => message_en,
        }
    }

    /// Get the Lao error message - ໄດ້ຂໍ້ຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> &str {
        match self {
            ConstructionLawError::PermitNotIssued { message_lao, .. }
            | ConstructionLawError::UnpermittedConstruction { message_lao, .. }
            | ConstructionLawError::UnlicensedContractor { message_lao, .. }
            | ConstructionLawError::ContractorGradeInadequate { message_lao, .. }
            | ConstructionLawError::MissingSafetyPlan { message_lao, .. }
            | ConstructionLawError::ImproperInspectionSequence { message_lao, .. }
            | ConstructionLawError::PrematureOccupancy { message_lao, .. }
            | ConstructionLawError::InvalidDefectsLiability { message_lao, .. }
            | ConstructionLawError::ValidationError { message_lao, .. } => message_lao,
        }
    }

    /// Get the governing statute citation - ໄດ້ການອ້າງອີງກົດໝາຍ
    pub fn legal_basis(&self) -> &'static str {
        CONSTRUCTION_LAW_CITATION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_contains_messages() {
        let error = ConstructionLawError::MissingSafetyPlan {
            provision: "on-site construction safety requirement",
            message_lao: "ໂຄງການຕ້ອງມີແຜນຄວາມປອດໄພ".to_string(),
            message_en: "The project must have a safety plan".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("The project must have a safety plan"));
        assert!(display.contains("ໂຄງການຕ້ອງມີແຜນຄວາມປອດໄພ"));
    }

    #[test]
    fn test_english_and_lao_message() {
        let error = ConstructionLawError::UnlicensedContractor {
            provision: "contractor licensing and registration",
            message_lao: "ຜູ້ຮັບເໝົາບໍ່ມີໃບອະນຸຍາດ".to_string(),
            message_en: "Contractor is not licensed".to_string(),
        };
        assert_eq!(error.english_message(), "Contractor is not licensed");
        assert_eq!(error.lao_message(), "ຜູ້ຮັບເໝົາບໍ່ມີໃບອະນຸຍາດ");
    }

    #[test]
    fn test_legal_basis_is_construction_law() {
        let error = ConstructionLawError::ValidationError {
            message_lao: "ຄວາມຜິດພາດ".to_string(),
            message_en: "Error".to_string(),
        };
        assert_eq!(error.legal_basis(), CONSTRUCTION_LAW_CITATION);
        assert!(error.legal_basis().contains("05/NA"));
        assert!(error.legal_basis().contains("2009"));
    }

    #[test]
    fn test_provision_descriptor_in_display() {
        let error = ConstructionLawError::ImproperInspectionSequence {
            provision: "staged construction inspection",
            message_lao: "ບໍ່ຖືກຕາມລຳດັບ".to_string(),
            message_en: "Out of sequence".to_string(),
        };
        assert!(format!("{}", error).contains("staged construction inspection"));
    }

    #[test]
    fn test_citation_referenced_in_permit_display() {
        let error = ConstructionLawError::PermitNotIssued {
            provision: "construction permit requirement",
            message_lao: "ຍັງບໍ່ໄດ້ອອກໃບອະນຸຍາດ".to_string(),
            message_en: "Permit not issued".to_string(),
        };
        assert!(format!("{}", error).contains(CONSTRUCTION_LAW_CITATION));
    }
}
