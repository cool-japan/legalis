//! Telecommunications Law Validators (ການກວດສອບກົດໝາຍໂທລະຄົມມະນາຄົມ)
//!
//! Validation functions for Lao telecommunications law based on the
//! **Law on Telecommunications (Lao PDR), No. 09/NA, 2011**.
//!
//! Each validator returns `Ok(())` on compliance, or a
//! [`TelecommunicationsLawError`] carrying bilingual messages and the governing
//! statute citation.

use crate::telecommunications_law::error::{TelecommunicationsLawError, TelecommunicationsResult};
use crate::telecommunications_law::types::*;

// ============================================================================
// Licensing Validators - ການກວດສອບໃບອະນຸຍາດ
// ============================================================================

/// Validate a telecommunications licence.
/// ກວດສອບໃບອະນຸຍາດໂທລະຄົມມະນາຄົມ
///
/// An operator must hold a granted licence to provide services, and the licence
/// term must be positive and not exceed the representative maximum term
/// ([`LICENSE_VALIDITY_YEARS`]).
pub fn validate_license(license: &TelecomLicense) -> TelecommunicationsResult<()> {
    if license.operator.trim().is_empty() {
        return Err(TelecommunicationsLawError::InvalidLicense {
            provision: "licensing of telecommunications operators",
            message_lao: "ຕ້ອງລະບຸຊື່ຜູ້ປະກອບການ".to_string(),
            message_en: "The licensed operator must be identified".to_string(),
        });
    }

    if !license.granted {
        return Err(TelecommunicationsLawError::UnlicensedOperation {
            provision: "licensing of telecommunications operators",
            message_lao: "ການໃຫ້ບໍລິການໂທລະຄົມມະນາຄົມຕ້ອງມີໃບອະນຸຍາດ".to_string(),
            message_en: "Providing telecommunications services requires a licence".to_string(),
        });
    }

    if license.validity_years == 0 {
        return Err(TelecommunicationsLawError::InvalidLicense {
            provision: "licensing of telecommunications operators",
            message_lao: "ໄລຍະເວລາໃບອະນຸຍາດຕ້ອງຫຼາຍກວ່າ 0 ປີ".to_string(),
            message_en: "Licence validity term must be greater than zero years".to_string(),
        });
    }

    if license.validity_years > LICENSE_VALIDITY_YEARS {
        return Err(TelecommunicationsLawError::InvalidLicense {
            provision: "licensing of telecommunications operators",
            message_lao: format!("ໄລຍະເວລາໃບອະນຸຍາດຕ້ອງບໍ່ເກີນ {} ປີ", LICENSE_VALIDITY_YEARS),
            message_en: format!(
                "Licence validity term must not exceed {} years",
                LICENSE_VALIDITY_YEARS
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Spectrum Validators - ການກວດສອບຄື້ນຄວາມຖີ່
// ============================================================================

/// Validate a single radio-frequency spectrum assignment.
/// ກວດສອບການມອບຄື້ນຄວາມຖີ່
///
/// The band start must be below the band end, and the band must lie within the
/// representable radio spectrum (from the [`SPECTRUM_MIN_KHZ`] floor up to the
/// [`SPECTRUM_MAX_GHZ`] / [`SPECTRUM_MAX_MHZ`] upper bound). Because bands are
/// modelled in integer MHz, the representable floor corresponds to a band start
/// of at least 1 MHz.
pub fn validate_spectrum_assignment(
    assignment: &SpectrumAssignment,
) -> TelecommunicationsResult<()> {
    if assignment.operator.trim().is_empty() {
        return Err(TelecommunicationsLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຜູ້ປະກອບການທີ່ໄດ້ຮັບການມອບຄື້ນຄວາມຖີ່".to_string(),
            message_en: "The operator receiving the spectrum assignment must be identified"
                .to_string(),
        });
    }

    if assignment.band_start_mhz >= assignment.band_end_mhz {
        return Err(TelecommunicationsLawError::InvalidSpectrumAssignment {
            provision: "radio frequency spectrum management",
            message_lao: "ຂອບເຂດເລີ່ມຕົ້ນຕ້ອງຕ່ຳກວ່າຂອບເຂດສິ້ນສຸດ".to_string(),
            message_en: "Band start frequency must be below the band end frequency".to_string(),
        });
    }

    if assignment.band_end_mhz > SPECTRUM_MAX_MHZ {
        return Err(TelecommunicationsLawError::InvalidSpectrumAssignment {
            provision: "radio frequency spectrum management",
            message_lao: format!("ແຖບຄື້ນຄວາມຖີ່ເກີນຂອບເຂດສູງສຸດ {} GHz", SPECTRUM_MAX_GHZ),
            message_en: format!(
                "Frequency band exceeds the {} GHz upper bound of the usable spectrum",
                SPECTRUM_MAX_GHZ
            ),
        });
    }

    let band_start_khz = u64::from(assignment.band_start_mhz) * 1_000;
    if band_start_khz < SPECTRUM_MIN_KHZ {
        return Err(TelecommunicationsLawError::InvalidSpectrumAssignment {
            provision: "radio frequency spectrum management",
            message_lao: format!(
                "ແຖບຄື້ນຄວາມຖີ່ຕ່ຳກວ່າຂອບເຂດຕ່ຳສຸດ {} kHz ທີ່ສະແດງໄດ້",
                SPECTRUM_MIN_KHZ
            ),
            message_en: format!(
                "Frequency band starts below the representable {} kHz spectrum floor",
                SPECTRUM_MIN_KHZ
            ),
        });
    }

    Ok(())
}

/// Validate that no two exclusive spectrum assignments overlap in frequency.
/// ກວດສອບວ່າການມອບຄື້ນຄວາມຖີ່ແບບຜູກຂາດບໍ່ຊ້ອນກັນ
///
/// Spectrum is assigned in non-overlapping bands; two assignments that are both
/// marked exclusive may not share any frequency. Non-exclusive (shared)
/// assignments are permitted to overlap.
pub fn validate_spectrum_no_overlap(
    assignments: &[SpectrumAssignment],
) -> TelecommunicationsResult<()> {
    for (index, first) in assignments.iter().enumerate() {
        if !first.exclusive {
            continue;
        }
        for second in assignments.iter().skip(index + 1) {
            if !second.exclusive {
                continue;
            }
            if first.overlaps(second) {
                return Err(TelecommunicationsLawError::SpectrumOverlap {
                    message_lao: format!(
                        "ການມອບຄື້ນຄວາມຖີ່ແບບຜູກຂາດຂອງ '{}' ແລະ '{}' ຊ້ອນກັນ",
                        first.operator, second.operator
                    ),
                    message_en: format!(
                        "Exclusive spectrum assignments for '{}' and '{}' overlap",
                        first.operator, second.operator
                    ),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// Interconnection Validators - ການກວດສອບການເຊື່ອມຕໍ່ໂຄງຂ່າຍ
// ============================================================================

/// Validate an interconnection request.
/// ກວດສອບຄຳຮ້ອງຂໍເຊື່ອມຕໍ່ໂຄງຂ່າຍ
///
/// Interconnection between operators must be granted and provided on
/// non-discriminatory and fair, reasonable terms.
pub fn validate_interconnection(request: &InterconnectionRequest) -> TelecommunicationsResult<()> {
    if request.requesting_operator.trim().is_empty() || request.host_operator.trim().is_empty() {
        return Err(TelecommunicationsLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຜູ້ປະກອບການທີ່ຮ້ອງຂໍ ແລະ ຜູ້ປະກອບການເຈົ້າຂອງໂຄງຂ່າຍ".to_string(),
            message_en: "Both the requesting and host operators must be identified".to_string(),
        });
    }

    if !request.granted {
        return Err(TelecommunicationsLawError::InterconnectionRefused {
            provision: "interconnection between operators",
            message_lao: "ການເຊື່ອມຕໍ່ໂຄງຂ່າຍລະຫວ່າງຜູ້ປະກອບການຕ້ອງໄດ້ຮັບການອະນຸຍາດ".to_string(),
            message_en: "Interconnection between operators must be provided".to_string(),
        });
    }

    if !request.non_discriminatory {
        return Err(TelecommunicationsLawError::InterconnectionRefused {
            provision: "non-discriminatory interconnection",
            message_lao: "ການເຊື່ອມຕໍ່ໂຄງຂ່າຍຕ້ອງເປັນໄປໂດຍບໍ່ເລືອກປະຕິບັດ".to_string(),
            message_en: "Interconnection must be provided on non-discriminatory terms".to_string(),
        });
    }

    if !request.fair_terms {
        return Err(TelecommunicationsLawError::InterconnectionRefused {
            provision: "fair and reasonable interconnection",
            message_lao: "ການເຊື່ອມຕໍ່ໂຄງຂ່າຍຕ້ອງເປັນໄປຕາມເງື່ອນໄຂທີ່ເປັນທຳ ແລະ ສົມເຫດສົມຜົນ".to_string(),
            message_en: "Interconnection must be provided on fair and reasonable terms".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Quality of Service & Tariff Validators - ການກວດສອບຄຸນນະພາບ ແລະ ອັດຕາຄ່າບໍລິການ
// ============================================================================

/// Validate a quality-of-service measurement against the representative targets.
/// ກວດສອບຄຸນນະພາບການບໍລິການ
///
/// Service availability must meet [`MIN_SERVICE_AVAILABILITY_PERCENT`] and the
/// call-drop rate must not exceed [`MAX_CALL_DROP_RATE_PERMILLE`].
pub fn validate_service_quality(quality: &ServiceQuality) -> TelecommunicationsResult<()> {
    if quality.availability_percent > 100 {
        return Err(TelecommunicationsLawError::ValidationError {
            message_lao: "ຄ່າຄວາມພ້ອມໃຫ້ບໍລິການຕ້ອງບໍ່ເກີນ 100%".to_string(),
            message_en: "Service availability percentage cannot exceed 100".to_string(),
        });
    }

    if quality.availability_percent < MIN_SERVICE_AVAILABILITY_PERCENT {
        return Err(TelecommunicationsLawError::QualityOfServiceBreach {
            message_lao: format!(
                "ຄວາມພ້ອມໃຫ້ບໍລິການ {}% ຕ່ຳກວ່າເປົ້າໝາຍ {}%",
                quality.availability_percent, MIN_SERVICE_AVAILABILITY_PERCENT
            ),
            message_en: format!(
                "Service availability {}% is below the {}% target",
                quality.availability_percent, MIN_SERVICE_AVAILABILITY_PERCENT
            ),
        });
    }

    if quality.call_drop_rate_permille > MAX_CALL_DROP_RATE_PERMILLE {
        return Err(TelecommunicationsLawError::QualityOfServiceBreach {
            message_lao: format!(
                "ອັດຕາການຫຼຸດສາຍ {} ຕໍ່ພັນ ສູງກວ່າເປົ້າໝາຍ {} ຕໍ່ພັນ",
                quality.call_drop_rate_permille, MAX_CALL_DROP_RATE_PERMILLE
            ),
            message_en: format!(
                "Call-drop rate {} per-mille exceeds the {} per-mille target",
                quality.call_drop_rate_permille, MAX_CALL_DROP_RATE_PERMILLE
            ),
        });
    }

    Ok(())
}

/// Validate a tariff.
/// ກວດສອບອັດຕາຄ່າບໍລິການ
///
/// A tariff must specify a positive price and carry the required regulatory
/// approval before it may apply.
pub fn validate_tariff(tariff: &Tariff) -> TelecommunicationsResult<()> {
    if tariff.price_lak == 0 {
        return Err(TelecommunicationsLawError::ValidationError {
            message_lao: "ລາຄາຄ່າບໍລິການຕ້ອງຫຼາຍກວ່າ 0 ກີບ".to_string(),
            message_en: "Tariff price must be greater than zero LAK".to_string(),
        });
    }

    if !tariff.regulator_approved {
        return Err(TelecommunicationsLawError::UnapprovedTariff {
            message_lao: "ອັດຕາຄ່າບໍລິການຕ້ອງໄດ້ຮັບການອະນຸມັດຈາກອົງການຄຸ້ມຄອງ".to_string(),
            message_en: "The tariff must be approved by the regulatory authority".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Equipment & Confidentiality Validators - ການກວດສອບອຸປະກອນ ແລະ ຄວາມລັບ
// ============================================================================

/// Validate that telecommunications equipment has the required type-approval.
/// ກວດສອບການຮັບຮອງປະເພດອຸປະກອນ
pub fn validate_equipment_type_approval(
    equipment: &EquipmentTypeApproval,
) -> TelecommunicationsResult<()> {
    if equipment.equipment_name.trim().is_empty() {
        return Err(TelecommunicationsLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ອຸປະກອນ".to_string(),
            message_en: "Equipment name is required".to_string(),
        });
    }

    if !equipment.approved {
        return Err(TelecommunicationsLawError::EquipmentNotApproved {
            message_lao: format!("ອຸປະກອນ '{}' ຍັງບໍ່ໄດ້ຮັບການຮັບຮອງປະເພດ", equipment.equipment_name),
            message_en: format!(
                "Equipment '{}' has not received type-approval",
                equipment.equipment_name
            ),
        });
    }

    Ok(())
}

/// Validate the confidentiality (secrecy) of communications.
/// ກວດສອບຄວາມລັບຂອງການສື່ສານ
///
/// The secrecy of communications is protected; intercepting communications
/// without lawful authorisation is prohibited.
pub fn validate_communication_confidentiality(
    lawful_authorization: bool,
    intercepted: bool,
) -> TelecommunicationsResult<()> {
    if intercepted && !lawful_authorization {
        return Err(TelecommunicationsLawError::UnlawfulInterception {
            provision: "confidentiality of communications",
            message_lao: "ການດັກຟັງການສື່ສານໂດຍບໍ່ມີການອະນຸຍາດທີ່ຖືກກົດໝາຍແມ່ນຖືກຫ້າມ".to_string(),
            message_en: "Intercepting communications without lawful authorisation is prohibited"
                .to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_license() -> TelecomLicense {
        TelecomLicense {
            operator: "Lao Telecom".to_string(),
            category: LicenseCategory::NetworkServices,
            granted: true,
            validity_years: 15,
            start_year: 2020,
            status: LicenseStatus::Active,
        }
    }

    fn sample_assignment(
        operator: &str,
        band_start_mhz: u32,
        band_end_mhz: u32,
        exclusive: bool,
    ) -> SpectrumAssignment {
        SpectrumAssignment {
            operator: operator.to_string(),
            band_start_mhz,
            band_end_mhz,
            exclusive,
        }
    }

    fn sample_interconnection() -> InterconnectionRequest {
        InterconnectionRequest {
            requesting_operator: "ETL".to_string(),
            host_operator: "Lao Telecom".to_string(),
            granted: true,
            non_discriminatory: true,
            fair_terms: true,
        }
    }

    // --- Licensing ---------------------------------------------------------

    #[test]
    fn test_valid_license_ok() {
        assert!(validate_license(&sample_license()).is_ok());
    }

    #[test]
    fn test_ungranted_license_fails() {
        let mut license = sample_license();
        license.granted = false;
        let err = validate_license(&license).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::UnlicensedOperation { .. }
        ));
    }

    #[test]
    fn test_license_zero_validity_fails() {
        let mut license = sample_license();
        license.validity_years = 0;
        assert!(validate_license(&license).is_err());
    }

    #[test]
    fn test_license_excessive_validity_fails() {
        let mut license = sample_license();
        license.validity_years = LICENSE_VALIDITY_YEARS + 1;
        let err = validate_license(&license).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::InvalidLicense { .. }
        ));
    }

    #[test]
    fn test_license_empty_operator_fails() {
        let mut license = sample_license();
        license.operator = String::new();
        assert!(validate_license(&license).is_err());
    }

    // --- Spectrum assignment ----------------------------------------------

    #[test]
    fn test_valid_spectrum_assignment_ok() {
        let assignment = sample_assignment("Lao Telecom", 900, 960, true);
        assert!(validate_spectrum_assignment(&assignment).is_ok());
    }

    #[test]
    fn test_spectrum_start_not_before_end_fails() {
        let assignment = sample_assignment("Lao Telecom", 960, 900, true);
        let err = validate_spectrum_assignment(&assignment).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::InvalidSpectrumAssignment { .. }
        ));
    }

    #[test]
    fn test_spectrum_exceeds_max_fails() {
        let assignment = sample_assignment(
            "Lao Telecom",
            SPECTRUM_MAX_MHZ - 10,
            SPECTRUM_MAX_MHZ + 10,
            true,
        );
        assert!(validate_spectrum_assignment(&assignment).is_err());
    }

    #[test]
    fn test_spectrum_below_floor_fails() {
        // A band starting at 0 MHz falls below the representable spectrum floor.
        let assignment = sample_assignment("Lao Telecom", 0, 5, true);
        assert!(validate_spectrum_assignment(&assignment).is_err());
    }

    #[test]
    fn test_spectrum_empty_operator_fails() {
        let assignment = sample_assignment("", 900, 960, true);
        assert!(validate_spectrum_assignment(&assignment).is_err());
    }

    // --- Spectrum non-overlap ---------------------------------------------

    #[test]
    fn test_spectrum_no_overlap_ok() {
        let assignments = vec![
            sample_assignment("A", 800, 820, true),
            sample_assignment("B", 820, 840, true),
            sample_assignment("C", 900, 960, true),
        ];
        assert!(validate_spectrum_no_overlap(&assignments).is_ok());
    }

    #[test]
    fn test_spectrum_overlap_detected() {
        let assignments = vec![
            sample_assignment("A", 800, 820, true),
            sample_assignment("B", 810, 830, true),
        ];
        let err = validate_spectrum_no_overlap(&assignments).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::SpectrumOverlap { .. }
        ));
    }

    #[test]
    fn test_spectrum_overlap_ignored_for_non_exclusive() {
        // Overlapping bands are permitted when neither assignment is exclusive.
        let assignments = vec![
            sample_assignment("A", 800, 820, false),
            sample_assignment("B", 810, 830, false),
        ];
        assert!(validate_spectrum_no_overlap(&assignments).is_ok());
    }

    // --- Interconnection ---------------------------------------------------

    #[test]
    fn test_interconnection_ok() {
        assert!(validate_interconnection(&sample_interconnection()).is_ok());
    }

    #[test]
    fn test_interconnection_refused_fails() {
        let mut request = sample_interconnection();
        request.granted = false;
        let err = validate_interconnection(&request).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::InterconnectionRefused { .. }
        ));
    }

    #[test]
    fn test_interconnection_discriminatory_fails() {
        let mut request = sample_interconnection();
        request.non_discriminatory = false;
        assert!(validate_interconnection(&request).is_err());
    }

    #[test]
    fn test_interconnection_unfair_terms_fails() {
        let mut request = sample_interconnection();
        request.fair_terms = false;
        assert!(validate_interconnection(&request).is_err());
    }

    #[test]
    fn test_interconnection_missing_party_fails() {
        let mut request = sample_interconnection();
        request.requesting_operator = String::new();
        assert!(validate_interconnection(&request).is_err());
    }

    // --- Quality of service ------------------------------------------------

    #[test]
    fn test_service_quality_ok() {
        let quality = ServiceQuality {
            service_type: ServiceType::Mobile,
            availability_percent: 99,
            call_drop_rate_permille: 10,
        };
        assert!(validate_service_quality(&quality).is_ok());
    }

    #[test]
    fn test_service_quality_low_availability_fails() {
        let quality = ServiceQuality {
            service_type: ServiceType::Mobile,
            availability_percent: 90,
            call_drop_rate_permille: 10,
        };
        let err = validate_service_quality(&quality).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::QualityOfServiceBreach { .. }
        ));
    }

    #[test]
    fn test_service_quality_high_drop_rate_fails() {
        let quality = ServiceQuality {
            service_type: ServiceType::Mobile,
            availability_percent: 99,
            call_drop_rate_permille: MAX_CALL_DROP_RATE_PERMILLE + 1,
        };
        assert!(validate_service_quality(&quality).is_err());
    }

    // --- Tariff ------------------------------------------------------------

    #[test]
    fn test_tariff_ok() {
        let tariff = Tariff {
            service_type: ServiceType::Internet,
            price_lak: 150_000,
            regulator_approved: true,
        };
        assert!(validate_tariff(&tariff).is_ok());
    }

    #[test]
    fn test_tariff_unapproved_fails() {
        let tariff = Tariff {
            service_type: ServiceType::Internet,
            price_lak: 150_000,
            regulator_approved: false,
        };
        let err = validate_tariff(&tariff).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::UnapprovedTariff { .. }
        ));
    }

    #[test]
    fn test_tariff_zero_price_fails() {
        let tariff = Tariff {
            service_type: ServiceType::Internet,
            price_lak: 0,
            regulator_approved: true,
        };
        assert!(validate_tariff(&tariff).is_err());
    }

    // --- Equipment type-approval ------------------------------------------

    #[test]
    fn test_equipment_approved_ok() {
        let equipment = EquipmentTypeApproval {
            equipment_name: "Model X router".to_string(),
            approved: true,
        };
        assert!(validate_equipment_type_approval(&equipment).is_ok());
    }

    #[test]
    fn test_equipment_not_approved_fails() {
        let equipment = EquipmentTypeApproval {
            equipment_name: "Model X router".to_string(),
            approved: false,
        };
        let err = validate_equipment_type_approval(&equipment).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::EquipmentNotApproved { .. }
        ));
    }

    // --- Confidentiality of communications --------------------------------

    #[test]
    fn test_confidentiality_lawful_ok() {
        // Lawful interception with authorisation is permitted.
        assert!(validate_communication_confidentiality(true, true).is_ok());
    }

    #[test]
    fn test_no_interception_ok() {
        assert!(validate_communication_confidentiality(false, false).is_ok());
    }

    #[test]
    fn test_unlawful_interception_fails() {
        let err = validate_communication_confidentiality(false, true).unwrap_err();
        assert!(matches!(
            err,
            TelecommunicationsLawError::UnlawfulInterception { .. }
        ));
    }
}
