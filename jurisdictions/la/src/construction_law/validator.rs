//! Construction Law Validators (ການກວດສອບກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ)
//!
//! Validation functions for Lao construction law based on the
//! **Law on Construction (Lao PDR), No. 05/NA, 2009**.
//!
//! Each validator returns `Ok(())` on compliance, or a [`ConstructionLawError`]
//! carrying bilingual messages and the governing statute citation.

use crate::construction_law::error::{ConstructionLawError, ConstructionResult};
use crate::construction_law::types::*;

// ============================================================================
// Permit Validators - ການກວດສອບໃບອະນຸຍາດ
// ============================================================================

/// Validate that a building permit has been issued and authorises construction.
/// ກວດສອບໃບອະນຸຍາດກໍ່ສ້າງ
///
/// A construction/building permit must be obtained before construction begins.
pub fn validate_building_permit(permit: &BuildingPermit) -> ConstructionResult<()> {
    if permit.project_name.trim().is_empty() {
        return Err(ConstructionLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ໂຄງການ".to_string(),
            message_en: "Project name is required for a building permit".to_string(),
        });
    }

    if !permit.issued {
        return Err(ConstructionLawError::PermitNotIssued {
            provision: "construction permit requirement",
            message_lao: format!("ໂຄງການ '{}' ຍັງບໍ່ໄດ້ຮັບໃບອະນຸຍາດກໍ່ສ້າງ", permit.project_name),
            message_en: format!(
                "Building permit for project '{}' has not been issued",
                permit.project_name
            ),
        });
    }

    if !permit.status.permits_construction() {
        return Err(ConstructionLawError::PermitNotIssued {
            provision: "construction permit requirement",
            message_lao: format!(
                "ສະຖານະໃບອະນຸຍາດ ({}) ບໍ່ອະນຸຍາດໃຫ້ດຳເນີນການກໍ່ສ້າງ",
                permit.status.lao_name()
            ),
            message_en: format!(
                "Permit status ({}) does not authorise construction",
                permit.status.english_name()
            ),
        });
    }

    Ok(())
}

/// Validate that a construction project is authorised by a permit.
/// ກວດສອບວ່າໂຄງການກໍ່ສ້າງໄດ້ຮັບອະນຸຍາດ
///
/// Unpermitted construction is an offence subject to penalties (stop-work, fine,
/// demolition).
pub fn validate_construction_authorized(project: &ConstructionProject) -> ConstructionResult<()> {
    if !project.has_permit {
        return Err(ConstructionLawError::UnpermittedConstruction {
            provision: "offence of unpermitted construction",
            message_lao: format!("ການກໍ່ສ້າງໂຄງການ '{}' ໂດຍບໍ່ມີໃບອະນຸຍາດແມ່ນຜິດກົດໝາຍ", project.name),
            message_en: format!(
                "Construction of project '{}' without a permit is an offence",
                project.name
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Contractor Validators - ການກວດສອບຜູ້ຮັບເໝົາ
// ============================================================================

/// Validate that a contractor is licensed/registered.
/// ກວດສອບໃບອະນຸຍາດຂອງຜູ້ຮັບເໝົາ
pub fn validate_contractor_license(contractor: &Contractor) -> ConstructionResult<()> {
    if contractor.name.trim().is_empty() {
        return Err(ConstructionLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ຜູ້ຮັບເໝົາ".to_string(),
            message_en: "Contractor name is required".to_string(),
        });
    }

    if !contractor.licensed {
        return Err(ConstructionLawError::UnlicensedContractor {
            provision: "contractor licensing and registration",
            message_lao: format!("ຜູ້ຮັບເໝົາ '{}' ບໍ່ມີໃບອະນຸຍາດ ຫຼື ຍັງບໍ່ໄດ້ຂຶ້ນທະບຽນ", contractor.name),
            message_en: format!(
                "Contractor '{}' is not licensed or registered",
                contractor.name
            ),
        });
    }

    Ok(())
}

/// Validate that a contractor's grade is adequate for the project value.
/// ກວດສອບຊັ້ນຄວາມສາມາດຂອງຜູ້ຮັບເໝົາ
///
/// A contractor's grade limits the value of the projects they may undertake; the
/// project value must not exceed the maximum permitted by the contractor's grade.
pub fn validate_contractor_grade(
    contractor: &Contractor,
    project_value_lak: u64,
) -> ConstructionResult<()> {
    if !contractor.can_undertake_value(project_value_lak) {
        return Err(ConstructionLawError::ContractorGradeInadequate {
            provision: "contractor grading and capacity limit",
            message_lao: format!(
                "ຜູ້ຮັບເໝົາ '{}' ({}) ບໍ່ສາມາດຮັບໂຄງການມູນຄ່າ {} ກີບ (ສູງສຸດ {} ກີບ)",
                contractor.name,
                contractor.grade.lao_name(),
                project_value_lak,
                contractor.max_project_value_lak
            ),
            message_en: format!(
                "Contractor '{}' ({}) may not undertake a project valued at {} LAK (grade limit {} LAK)",
                contractor.name,
                contractor.grade.english_name(),
                project_value_lak,
                contractor.max_project_value_lak
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Safety Validators - ການກວດສອບຄວາມປອດໄພ
// ============================================================================

/// Validate that a project has an on-site safety plan.
/// ກວດສອບແຜນຄວາມປອດໄພຂອງໂຄງການ
///
/// Construction must comply with technical standards and on-site safety
/// requirements, including a safety plan.
pub fn validate_safety_plan(project: &ConstructionProject) -> ConstructionResult<()> {
    if !project.has_safety_plan {
        return Err(ConstructionLawError::MissingSafetyPlan {
            provision: "on-site construction safety requirement",
            message_lao: format!("ໂຄງການ '{}' ຕ້ອງມີແຜນຄວາມປອດໄພໃນສະຖານທີ່ກໍ່ສ້າງ", project.name),
            message_en: format!(
                "Project '{}' must have an on-site safety plan",
                project.name
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Inspection & Acceptance Validators - ການກວດສອບການກວດກາ ແລະ ການກວດຮັບ
// ============================================================================

/// Validate the order of staged construction inspections.
/// ກວດສອບລຳດັບການກວດກາການກໍ່ສ້າງ
///
/// Inspection proceeds in mandatory stages (foundation -> structure ->
/// completion). The `next` stage cannot be performed until all earlier stages
/// (by [`InspectionStage::order`]) appear in `completed`.
pub fn validate_inspection_sequence(
    completed: &[InspectionStage],
    next: InspectionStage,
) -> ConstructionResult<()> {
    for stage in InspectionStage::all() {
        if stage.order() < next.order() && !completed.contains(&stage) {
            return Err(ConstructionLawError::ImproperInspectionSequence {
                provision: "staged construction inspection",
                message_lao: format!("ບໍ່ສາມາດດຳເນີນ{}ກ່ອນຜ່ານ{}", next.lao_name(), stage.lao_name()),
                message_en: format!(
                    "Cannot perform the {} inspection before the {} inspection is completed",
                    next.english_name(),
                    stage.english_name()
                ),
            });
        }
    }

    Ok(())
}

/// Validate the acceptance/handover before occupancy.
/// ກວດສອບການກວດຮັບກ່ອນການເຂົ້າຢູ່ອາໄສ
///
/// Formal acceptance and handover must precede occupancy; occupancy may only be
/// permitted once all mandatory inspections have passed.
pub fn validate_work_acceptance(acceptance: &WorkAcceptance) -> ConstructionResult<()> {
    if acceptance.occupancy_permitted && !acceptance.all_inspections_passed {
        return Err(ConstructionLawError::PrematureOccupancy {
            provision: "acceptance and handover before occupancy",
            message_lao:
                "ບໍ່ສາມາດອະນຸຍາດໃຫ້ເຂົ້າຢູ່ອາໄສກ່ອນຜ່ານການກວດກາ ແລະ ກວດຮັບວຽກທັງໝົດ".to_string(),
            message_en:
                "Occupancy cannot be permitted before all inspections are passed and the works are accepted"
                    .to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Defects-Liability Validators - ການກວດສອບໄລຍະຮັບປະກັນ
// ============================================================================

/// Validate the defects-liability (warranty) period of a contract.
/// ກວດສອບໄລຍະຮັບປະກັນຄວາມເສຍຫາຍຂອງສັນຍາ
///
/// A defects-liability period applies to completed works, during which the
/// contractor remains responsible for defects. The period must be positive and
/// at least the expected minimum of [`DEFECTS_LIABILITY_PERIOD_MONTHS`] months.
pub fn validate_defects_liability(contract: &ConstructionContract) -> ConstructionResult<()> {
    if contract.defects_liability_months == 0 {
        return Err(ConstructionLawError::InvalidDefectsLiability {
            provision: "defects-liability (warranty) period",
            message_lao: "ສັນຍາຕ້ອງກຳນົດໄລຍະຮັບປະກັນຄວາມເສຍຫາຍ".to_string(),
            message_en: "The contract must stipulate a defects-liability period".to_string(),
        });
    }

    if contract.defects_liability_months < DEFECTS_LIABILITY_PERIOD_MONTHS {
        return Err(ConstructionLawError::InvalidDefectsLiability {
            provision: "defects-liability (warranty) period",
            message_lao: format!(
                "ໄລຍະຮັບປະກັນ ({} ເດືອນ) ໜ້ອຍກວ່າຂັ້ນຕ່ຳທີ່ຄາດໝາຍ {} ເດືອນ",
                contract.defects_liability_months, DEFECTS_LIABILITY_PERIOD_MONTHS
            ),
            message_en: format!(
                "Defects-liability period ({} months) is below the expected minimum of {} months",
                contract.defects_liability_months, DEFECTS_LIABILITY_PERIOD_MONTHS
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approved_permit() -> BuildingPermit {
        BuildingPermit {
            project_name: "Vientiane riverside housing".to_string(),
            category: ConstructionCategory::Medium,
            applicant: "Mekong Developments Co.".to_string(),
            issued: true,
            status: PermitStatus::Approved,
        }
    }

    fn licensed_contractor() -> Contractor {
        Contractor {
            name: "Lao Build Co.".to_string(),
            grade: ContractorGrade::GradeII,
            licensed: true,
            max_project_value_lak: 10_000_000_000,
        }
    }

    fn permitted_project() -> ConstructionProject {
        ConstructionProject {
            name: "Vientiane riverside housing".to_string(),
            project_type: ProjectType::Residential,
            value_lak: 5_000_000_000,
            has_permit: true,
            has_safety_plan: true,
            contractor_grade: ContractorGrade::GradeII,
        }
    }

    // ----- Permit -----

    #[test]
    fn test_building_permit_issued_ok() {
        assert!(validate_building_permit(&approved_permit()).is_ok());
    }

    #[test]
    fn test_building_permit_not_issued_fails() {
        let mut permit = approved_permit();
        permit.issued = false;
        let err = validate_building_permit(&permit).unwrap_err();
        assert!(matches!(err, ConstructionLawError::PermitNotIssued { .. }));
    }

    #[test]
    fn test_building_permit_status_not_approved_fails() {
        let mut permit = approved_permit();
        permit.status = PermitStatus::Pending;
        assert!(validate_building_permit(&permit).is_err());
    }

    #[test]
    fn test_building_permit_empty_name_fails() {
        let mut permit = approved_permit();
        permit.project_name = String::new();
        let err = validate_building_permit(&permit).unwrap_err();
        assert!(matches!(err, ConstructionLawError::ValidationError { .. }));
    }

    // ----- Construction authorization -----

    #[test]
    fn test_construction_authorized_ok() {
        assert!(validate_construction_authorized(&permitted_project()).is_ok());
    }

    #[test]
    fn test_unpermitted_construction_fails() {
        let mut project = permitted_project();
        project.has_permit = false;
        let err = validate_construction_authorized(&project).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::UnpermittedConstruction { .. }
        ));
    }

    // ----- Contractor licence -----

    #[test]
    fn test_contractor_license_ok() {
        assert!(validate_contractor_license(&licensed_contractor()).is_ok());
    }

    #[test]
    fn test_contractor_unlicensed_fails() {
        let mut contractor = licensed_contractor();
        contractor.licensed = false;
        let err = validate_contractor_license(&contractor).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::UnlicensedContractor { .. }
        ));
    }

    #[test]
    fn test_contractor_empty_name_fails() {
        let mut contractor = licensed_contractor();
        contractor.name = String::new();
        assert!(validate_contractor_license(&contractor).is_err());
    }

    // ----- Contractor grade -----

    #[test]
    fn test_contractor_grade_adequate_ok() {
        assert!(validate_contractor_grade(&licensed_contractor(), 5_000_000_000).is_ok());
    }

    #[test]
    fn test_contractor_grade_inadequate_fails() {
        let err = validate_contractor_grade(&licensed_contractor(), 50_000_000_000).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::ContractorGradeInadequate { .. }
        ));
    }

    #[test]
    fn test_contractor_grade_exact_limit_ok() {
        assert!(validate_contractor_grade(&licensed_contractor(), 10_000_000_000).is_ok());
    }

    // ----- Safety plan -----

    #[test]
    fn test_safety_plan_ok() {
        assert!(validate_safety_plan(&permitted_project()).is_ok());
    }

    #[test]
    fn test_safety_plan_missing_fails() {
        let mut project = permitted_project();
        project.has_safety_plan = false;
        let err = validate_safety_plan(&project).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::MissingSafetyPlan { .. }
        ));
    }

    // ----- Inspection sequence -----

    #[test]
    fn test_inspection_sequence_first_stage_ok() {
        assert!(validate_inspection_sequence(&[], InspectionStage::Foundation).is_ok());
    }

    #[test]
    fn test_inspection_sequence_in_order_ok() {
        assert!(
            validate_inspection_sequence(
                &[InspectionStage::Foundation],
                InspectionStage::Structure
            )
            .is_ok()
        );
        assert!(
            validate_inspection_sequence(
                &[InspectionStage::Foundation, InspectionStage::Structure],
                InspectionStage::Completion
            )
            .is_ok()
        );
    }

    #[test]
    fn test_inspection_sequence_skipping_stage_fails() {
        // Attempting completion without structure (and foundation) completed.
        let err = validate_inspection_sequence(
            &[InspectionStage::Foundation],
            InspectionStage::Completion,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::ImproperInspectionSequence { .. }
        ));
    }

    #[test]
    fn test_inspection_sequence_out_of_order_fails() {
        // Attempting structure before foundation completed.
        assert!(validate_inspection_sequence(&[], InspectionStage::Structure).is_err());
    }

    // ----- Work acceptance -----

    #[test]
    fn test_work_acceptance_ok() {
        let acceptance = WorkAcceptance {
            all_inspections_passed: true,
            occupancy_permitted: true,
        };
        assert!(validate_work_acceptance(&acceptance).is_ok());
    }

    #[test]
    fn test_work_acceptance_no_occupancy_ok() {
        // Occupancy not yet permitted while inspections incomplete is acceptable.
        let acceptance = WorkAcceptance {
            all_inspections_passed: false,
            occupancy_permitted: false,
        };
        assert!(validate_work_acceptance(&acceptance).is_ok());
    }

    #[test]
    fn test_premature_occupancy_fails() {
        let acceptance = WorkAcceptance {
            all_inspections_passed: false,
            occupancy_permitted: true,
        };
        let err = validate_work_acceptance(&acceptance).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::PrematureOccupancy { .. }
        ));
    }

    // ----- Defects liability -----

    fn contract_with_months(months: u32) -> ConstructionContract {
        ConstructionContract {
            owner: "Ministry of Public Works".to_string(),
            contractor: "Lao Build Co.".to_string(),
            value_lak: 5_000_000_000,
            defects_liability_months: months,
        }
    }

    #[test]
    fn test_defects_liability_ok() {
        assert!(
            validate_defects_liability(&contract_with_months(DEFECTS_LIABILITY_PERIOD_MONTHS))
                .is_ok()
        );
    }

    #[test]
    fn test_defects_liability_zero_fails() {
        let err = validate_defects_liability(&contract_with_months(0)).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::InvalidDefectsLiability { .. }
        ));
    }

    #[test]
    fn test_defects_liability_below_minimum_fails() {
        let err = validate_defects_liability(&contract_with_months(12)).unwrap_err();
        assert!(matches!(
            err,
            ConstructionLawError::InvalidDefectsLiability { .. }
        ));
    }
}
