//! Water Law Validators (ຕົວກວດສອບກົດໝາຍຊັບພະຍາກອນນໍ້າ)
//!
//! Validation functions for Lao water and water resources law compliance based on:
//! - **Water and Water Resources Law 2017** (Law No. 23/NA)
//! - Mekong River Commission 1995 Agreement
//!
//! # Legal References
//! - Articles 35-44: Water Use Rights and Permits
//! - Articles 45-54: Hydropower Regulations
//! - Articles 55-58: Water Quality Standards
//! - Articles 60-63: MRC Compliance
//! - Articles 70-73: Irrigation Districts
//! - Articles 75-78: Groundwater Management
//! - Articles 80-82: Pollution Prevention

use super::error::{Result, WaterLawError};
use super::types::*;

// ============================================================================
// Water Use Rights Validation (ການກວດສອບສິດນຳໃຊ້ນໍ້າ)
// ============================================================================

/// Validate water use permit requirement (ກວດສອບຄວາມຕ້ອງການໃບອະນຸຍາດນຳໃຊ້ນໍ້າ)
///
/// Article 35: Water use permit requirements
///
/// # Arguments
/// * `use_type` - Type of water use
/// * `has_permit` - Whether permit is held
///
/// # Returns
/// * `Ok(())` if permit requirement is met
/// * `Err(WaterLawError)` if permit is required but missing
pub fn validate_water_use_permit(use_type: WaterUseType, has_permit: bool) -> Result<()> {
    if use_type.requires_permit() && !has_permit {
        return Err(WaterLawError::MissingWaterUsePermit {
            use_type: use_type.lao_name().to_string(),
        });
    }
    Ok(())
}

/// Validate water extraction against permit limit (ກວດສອບການສູບນໍ້າຕາມຂີດຈຳກັດໃບອະນຸຍາດ)
///
/// Article 36: Extraction limits
///
/// # Arguments
/// * `actual_extraction_m3_day` - Actual extraction in m3/day
/// * `permitted_extraction_m3_day` - Permitted extraction in m3/day
///
/// # Returns
/// * `Ok(())` if extraction is within limit
/// * `Err(WaterLawError)` if extraction exceeds limit
pub fn validate_extraction_limit(
    actual_extraction_m3_day: f64,
    permitted_extraction_m3_day: f64,
) -> Result<()> {
    if actual_extraction_m3_day < 0.0 {
        return Err(WaterLawError::ValidationError {
            message: "Extraction cannot be negative".to_string(),
        });
    }

    if actual_extraction_m3_day > permitted_extraction_m3_day {
        return Err(WaterLawError::ExtractionExceedsLimit {
            actual: actual_extraction_m3_day,
            limit: permitted_extraction_m3_day,
        });
    }
    Ok(())
}

/// Validate water use priority hierarchy (ກວດສອບລຳດັບບຸລິມະສິດການນຳໃຊ້ນໍ້າ)
///
/// Article 38-40: Water allocation priority
///
/// # Arguments
/// * `requesting_use` - Type of use requesting water
/// * `existing_use` - Type of existing use
/// * `water_shortage` - Whether there is a water shortage
///
/// # Returns
/// * `Ok(true)` if requesting use has priority
/// * `Ok(false)` if existing use has priority
/// * `Err(WaterLawError)` if priority is violated during shortage
pub fn validate_water_use_priority(
    requesting_use: WaterUseType,
    existing_use: WaterUseType,
    water_shortage: bool,
) -> Result<bool> {
    let requesting_has_priority = requesting_use.has_priority_over(&existing_use);

    // During water shortage, domestic use must have absolute priority
    if water_shortage
        && existing_use == WaterUseType::Domestic
        && requesting_use != WaterUseType::Domestic
    {
        return Err(WaterLawError::DomesticPriorityViolation {
            other_use: requesting_use.lao_name().to_string(),
        });
    }

    Ok(requesting_has_priority)
}

/// Validate water permit validity (ກວດສອບຄວາມຖືກຕ້ອງຂອງໃບອະນຸຍາດນໍ້າ)
///
/// Article 35: Water permit validity
///
/// # Arguments
/// * `permit` - Water use right permit
/// * `current_date` - Current date (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(WaterLawError)` if permit is invalid
pub fn validate_water_permit(permit: &WaterUseRight, current_date: &str) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(WaterLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    if permit.holder_name.trim().is_empty() {
        return Err(WaterLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    // Check status
    match permit.status {
        WaterPermitStatus::Active | WaterPermitStatus::Renewed => {
            // Check expiry
            if current_date > permit.expiry_date.as_str() {
                return Err(WaterLawError::PermitExpired {
                    permit_number: permit.permit_number.clone(),
                    expiry_date: permit.expiry_date.clone(),
                });
            }
        }
        WaterPermitStatus::Expired => {
            return Err(WaterLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        WaterPermitStatus::Suspended => {
            return Err(WaterLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: "ໂຈະ (suspended)".to_string(),
            });
        }
        WaterPermitStatus::Revoked => {
            return Err(WaterLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: "ຖືກຖອນ (revoked)".to_string(),
            });
        }
        WaterPermitStatus::Pending => {
            return Err(WaterLawError::ValidationError {
                message: "Permit is still pending approval".to_string(),
            });
        }
    }

    // Check conditions compliance
    for condition in &permit.conditions {
        if !condition.compliant {
            return Err(WaterLawError::PermitConditionViolation {
                condition: condition.description.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Hydropower Validation (ການກວດສອບໄຟຟ້ານໍ້າຕົກ)
// ============================================================================

/// Validate hydropower concession (ກວດສອບສຳປະທານໄຟຟ້ານໍ້າຕົກ)
///
/// Articles 45-54: Hydropower regulations
///
/// # Arguments
/// * `concession` - Hydropower concession
/// * `current_date` - Current date (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` if concession is valid
/// * `Err(WaterLawError)` if concession is invalid
pub fn validate_hydropower_concession(
    concession: &HydropowerConcession,
    current_date: &str,
) -> Result<()> {
    // Check required fields
    if concession.concession_number.trim().is_empty() {
        return Err(WaterLawError::MissingRequiredField {
            field_name: "concession_number".to_string(),
        });
    }

    if concession.project_name.trim().is_empty() {
        return Err(WaterLawError::MissingRequiredField {
            field_name: "project_name".to_string(),
        });
    }

    // Validate capacity and category match
    let expected_category = HydropowerCategory::from_capacity(concession.installed_capacity_mw);
    if expected_category != concession.category {
        return Err(WaterLawError::InvalidHydropowerCategory {
            capacity_mw: concession.installed_capacity_mw,
            reason: format!(
                "Expected {:?}, got {:?}",
                expected_category, concession.category
            ),
        });
    }

    // Check concession period
    if concession.concession_years < HYDROPOWER_CONCESSION_MIN_YEARS {
        return Err(WaterLawError::ValidationError {
            message: format!(
                "Concession period {} years is less than minimum {} years",
                concession.concession_years, HYDROPOWER_CONCESSION_MIN_YEARS
            ),
        });
    }

    if concession.concession_years > HYDROPOWER_CONCESSION_MAX_YEARS {
        return Err(WaterLawError::ValidationError {
            message: format!(
                "Concession period {} years exceeds maximum {} years",
                concession.concession_years, HYDROPOWER_CONCESSION_MAX_YEARS
            ),
        });
    }

    // Check expiry
    if current_date > concession.end_date.as_str()
        && matches!(
            concession.status,
            ConcessionStatus::Active | ConcessionStatus::Construction
        )
    {
        return Err(WaterLawError::ConcessionExpired {
            expiry_date: concession.end_date.clone(),
        });
    }

    // Check MRC compliance for Mekong mainstream or large projects
    if concession
        .category
        .requires_mrc_consultation(concession.on_mekong_mainstream)
        && !concession.mrc_consultation_completed
    {
        return Err(WaterLawError::MissingPriorConsultation {
            project_type: "hydropower".to_string(),
        });
    }

    // Check resettlement plan if there are affected households
    if let Some(ref plan) = concession.resettlement_plan
        && plan.affected_households > 0
        && !matches!(
            plan.approval_status,
            ResettlementApprovalStatus::Approved
                | ResettlementApprovalStatus::Implementation
                | ResettlementApprovalStatus::Completed
        )
    {
        return Err(WaterLawError::MissingResettlementPlan {
            affected_households: plan.affected_households,
        });
    }

    Ok(())
}

/// Validate minimum environmental flow (ກວດສອບການໄຫຼຂັ້ນຕໍ່າເພື່ອສິ່ງແວດລ້ອມ)
///
/// Article 48: Minimum environmental flow requirements
///
/// # Arguments
/// * `actual_flow_m3s` - Actual flow in m3/s
/// * `minimum_flow_m3s` - Minimum required flow in m3/s
///
/// # Returns
/// * `Ok(())` if flow meets minimum
/// * `Err(WaterLawError)` if flow is below minimum
pub fn validate_minimum_environmental_flow(
    actual_flow_m3s: f64,
    minimum_flow_m3s: f64,
) -> Result<()> {
    if actual_flow_m3s < 0.0 {
        return Err(WaterLawError::ValidationError {
            message: "Flow cannot be negative".to_string(),
        });
    }

    if actual_flow_m3s < minimum_flow_m3s {
        return Err(WaterLawError::MinimumFlowViolation {
            actual: actual_flow_m3s,
            minimum: minimum_flow_m3s,
        });
    }
    Ok(())
}

/// Validate hydropower category (ກວດສອບປະເພດໄຟຟ້ານໍ້າຕົກ)
///
/// Article 46: Hydropower classification
///
/// # Arguments
/// * `capacity_mw` - Installed capacity in MW
///
/// # Returns
/// * `Ok(HydropowerCategory)` - The appropriate category
pub fn validate_hydropower_category(capacity_mw: f64) -> Result<HydropowerCategory> {
    if capacity_mw < 0.0 {
        return Err(WaterLawError::ValidationError {
            message: "Capacity cannot be negative".to_string(),
        });
    }

    Ok(HydropowerCategory::from_capacity(capacity_mw))
}

// ============================================================================
// Water Quality Validation (ການກວດສອບຄຸນນະພາບນໍ້າ)
// ============================================================================

/// Validate drinking water quality (ກວດສອບຄຸນນະພາບນໍ້າດື່ມ)
///
/// Article 55: Drinking water quality standards
///
/// # Arguments
/// * `parameter` - Water quality parameter
/// * `value` - Measured value
///
/// # Returns
/// * `Ok(())` if water quality meets standards
/// * `Err(WaterLawError)` if water quality fails standards
pub fn validate_drinking_water_quality(parameter: WaterQualityParameter, value: f64) -> Result<()> {
    match parameter {
        WaterQualityParameter::Ph => {
            if !(DRINKING_WATER_MIN_PH..=DRINKING_WATER_MAX_PH).contains(&value) {
                return Err(WaterLawError::DrinkingWaterViolation {
                    parameter: "pH".to_string(),
                    actual: value,
                    limit: DRINKING_WATER_MAX_PH,
                    unit: "".to_string(),
                });
            }
        }
        WaterQualityParameter::Turbidity => {
            if value > DRINKING_WATER_MAX_TURBIDITY_NTU {
                return Err(WaterLawError::DrinkingWaterViolation {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: DRINKING_WATER_MAX_TURBIDITY_NTU,
                    unit: "NTU".to_string(),
                });
            }
        }
        WaterQualityParameter::Arsenic => {
            if value > DRINKING_WATER_MAX_ARSENIC_MG_L {
                return Err(WaterLawError::DrinkingWaterViolation {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: DRINKING_WATER_MAX_ARSENIC_MG_L,
                    unit: "mg/L".to_string(),
                });
            }
        }
        WaterQualityParameter::Lead => {
            if value > DRINKING_WATER_MAX_LEAD_MG_L {
                return Err(WaterLawError::DrinkingWaterViolation {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: DRINKING_WATER_MAX_LEAD_MG_L,
                    unit: "mg/L".to_string(),
                });
            }
        }
        WaterQualityParameter::EColi => {
            if value > DRINKING_WATER_MAX_ECOLI {
                return Err(WaterLawError::DrinkingWaterViolation {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: DRINKING_WATER_MAX_ECOLI,
                    unit: "CFU/100mL".to_string(),
                });
            }
        }
        _ => {
            // Check generic limit if available
            if let Some(limit) = parameter.drinking_water_limit()
                && value > limit
            {
                return Err(WaterLawError::DrinkingWaterViolation {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit,
                    unit: parameter.unit().to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate industrial discharge quality (ກວດສອບຄຸນນະພາບການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ)
///
/// Article 57: Industrial discharge standards
///
/// # Arguments
/// * `parameter` - Water quality parameter
/// * `value` - Measured value
///
/// # Returns
/// * `Ok(())` if discharge meets standards
/// * `Err(WaterLawError)` if discharge fails standards
pub fn validate_industrial_discharge(parameter: WaterQualityParameter, value: f64) -> Result<()> {
    let (limit, unit) = match parameter {
        WaterQualityParameter::Bod => (INDUSTRIAL_DISCHARGE_MAX_BOD_MG_L, "mg/L"),
        WaterQualityParameter::Cod => (INDUSTRIAL_DISCHARGE_MAX_COD_MG_L, "mg/L"),
        WaterQualityParameter::Tss => (INDUSTRIAL_DISCHARGE_MAX_TSS_MG_L, "mg/L"),
        _ => return Ok(()), // No limit for other parameters
    };

    if value > limit {
        return Err(WaterLawError::IndustrialDischargeViolation {
            pollutant: parameter.lao_name().to_string(),
            actual: value,
            limit,
            unit: unit.to_string(),
        });
    }

    Ok(())
}

/// Validate wastewater treatment requirement (ກວດສອບຄວາມຕ້ອງການບຳບັດນໍ້າເສຍ)
///
/// Article 58: Wastewater treatment requirements
///
/// # Arguments
/// * `has_treatment` - Whether wastewater treatment is in place
/// * `discharge_volume_m3_day` - Volume of discharge in m3/day
/// * `discharge_threshold_m3_day` - Threshold requiring treatment
///
/// # Returns
/// * `Ok(())` if treatment requirement is met
/// * `Err(WaterLawError)` if treatment is required but missing
pub fn validate_wastewater_treatment(
    has_treatment: bool,
    discharge_volume_m3_day: f64,
    discharge_threshold_m3_day: f64,
) -> Result<()> {
    if discharge_volume_m3_day > discharge_threshold_m3_day && !has_treatment {
        return Err(WaterLawError::MissingWastewaterTreatment);
    }
    Ok(())
}

// ============================================================================
// MRC Compliance Validation (ການກວດສອບການປະຕິບັດຕາມ MRC)
// ============================================================================

/// Validate MRC prior consultation requirement (ກວດສອບຄວາມຕ້ອງການປຶກສາຫາລືລ່ວງໜ້າ MRC)
///
/// Article 60: Prior consultation requirements
///
/// # Arguments
/// * `water_source` - Water source type
/// * `project_type` - Type of project
/// * `consultation_completed` - Whether consultation is completed
///
/// # Returns
/// * `Ok(())` if MRC requirements are met
/// * `Err(WaterLawError)` if prior consultation is required but missing
pub fn validate_mrc_prior_consultation(
    water_source: &WaterSourceType,
    project_type: &str,
    consultation_completed: bool,
) -> Result<()> {
    if water_source.requires_mrc_procedures() && !consultation_completed {
        return Err(WaterLawError::MissingPriorConsultation {
            project_type: project_type.to_string(),
        });
    }
    Ok(())
}

/// Validate MRC notification requirement (ກວດສອບຄວາມຕ້ອງການແຈ້ງເຕືອນ MRC)
///
/// Article 61: Notification requirements for tributary projects
///
/// # Arguments
/// * `has_transboundary_impact` - Whether project has transboundary impact
/// * `notification_submitted` - Whether notification was submitted
///
/// # Returns
/// * `Ok(())` if notification requirement is met
/// * `Err(WaterLawError)` if notification is required but missing
pub fn validate_mrc_notification(
    has_transboundary_impact: bool,
    notification_submitted: bool,
) -> Result<()> {
    if has_transboundary_impact && !notification_submitted {
        return Err(WaterLawError::MissingMRCNotification);
    }
    Ok(())
}

/// Validate transboundary impact assessment (ກວດສອບການປະເມີນຜົນກະທົບຂ້າມຊາຍແດນ)
///
/// Article 62: Transboundary impact assessment requirements
///
/// # Arguments
/// * `project_name` - Project name
/// * `has_transboundary_impact` - Whether project has transboundary impact
/// * `assessment_completed` - Whether assessment is completed
///
/// # Returns
/// * `Ok(())` if assessment requirement is met
/// * `Err(WaterLawError)` if assessment is required but missing
pub fn validate_transboundary_assessment(
    project_name: &str,
    has_transboundary_impact: bool,
    assessment_completed: bool,
) -> Result<()> {
    if has_transboundary_impact && !assessment_completed {
        return Err(WaterLawError::MissingTransboundaryAssessment {
            project_name: project_name.to_string(),
        });
    }
    Ok(())
}

/// Validate MRC data sharing obligations (ກວດສອບຂໍ້ຜູກມັດການແບ່ງປັນຂໍ້ມູນ MRC)
///
/// Article 63: Data sharing requirements
///
/// # Arguments
/// * `data_type` - Type of data that should be shared
/// * `data_shared` - Whether data has been shared
///
/// # Returns
/// * `Ok(())` if data sharing requirement is met
/// * `Err(WaterLawError)` if data sharing is required but not done
pub fn validate_mrc_data_sharing(data_type: &str, data_shared: bool) -> Result<()> {
    if !data_shared {
        return Err(WaterLawError::DataSharingViolation {
            data_type: data_type.to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Irrigation District Validation (ການກວດສອບເຂດຊົນລະປະທານ)
// ============================================================================

/// Validate Water User Association registration (ກວດສອບການລົງທະບຽນ WUA)
///
/// Article 70: WUA registration requirements
///
/// # Arguments
/// * `wua` - Water User Association
///
/// # Returns
/// * `Ok(())` if WUA is properly registered
/// * `Err(WaterLawError)` if WUA is not registered
pub fn validate_wua_registration(wua: &WaterUserAssociation) -> Result<()> {
    if wua.registration_number.trim().is_empty() {
        return Err(WaterLawError::WUANotRegistered);
    }

    if !matches!(wua.status, WUAStatus::Active) {
        return Err(WaterLawError::ValidationError {
            message: format!("WUA status is {:?}, must be Active", wua.status),
        });
    }

    Ok(())
}

/// Validate irrigation service fee payment (ກວດສອບການຈ່າຍຄ່າບໍລິການຊົນລະປະທານ)
///
/// Article 72: Irrigation service fee requirements
///
/// # Arguments
/// * `fee` - Irrigation service fee record
/// * `current_date` - Current date (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` if fee is paid or not overdue
/// * `Err(WaterLawError)` if fee is overdue
pub fn validate_irrigation_fee(fee: &IrrigationServiceFee, current_date: &str) -> Result<()> {
    match fee.status {
        FeePaymentStatus::Paid | FeePaymentStatus::Waived => Ok(()),
        FeePaymentStatus::Overdue => {
            let days_overdue = estimate_days_between(&fee.due_date, current_date);
            Err(WaterLawError::IrrigationFeeOverdue {
                amount: fee.amount_due_lak - fee.amount_paid_lak,
                days: days_overdue,
            })
        }
        FeePaymentStatus::Pending | FeePaymentStatus::Partial => {
            if current_date > fee.due_date.as_str() {
                let days_overdue = estimate_days_between(&fee.due_date, current_date);
                Err(WaterLawError::IrrigationFeeOverdue {
                    amount: fee.amount_due_lak - fee.amount_paid_lak,
                    days: days_overdue,
                })
            } else {
                Ok(())
            }
        }
    }
}

/// Calculate irrigation service fee (ຄິດໄລ່ຄ່າບໍລິການຊົນລະປະທານ)
///
/// Article 72: Fee calculation
///
/// # Arguments
/// * `area_hectares` - Area serviced in hectares
///
/// # Returns
/// Fee amount in LAK
pub fn calculate_irrigation_fee(area_hectares: f64) -> u64 {
    (area_hectares * IRRIGATION_FEE_PER_HECTARE_LAK as f64) as u64
}

// ============================================================================
// Groundwater Management Validation (ການກວດສອບການຄຸ້ມຄອງນໍ້າໃຕ້ດິນ)
// ============================================================================

/// Validate well drilling permit requirement (ກວດສອບຄວາມຕ້ອງການໃບອະນຸຍາດຂຸດເຈາະບໍ່)
///
/// Article 75: Well drilling permit requirements
///
/// # Arguments
/// * `depth_meters` - Well depth in meters
/// * `has_permit` - Whether permit is held
///
/// # Returns
/// * `Ok(())` if permit requirement is met
/// * `Err(WaterLawError)` if permit is required but missing
pub fn validate_well_drilling_permit(depth_meters: f64, has_permit: bool) -> Result<()> {
    let threshold = WELL_PERMIT_DEPTH_THRESHOLD_M as f64;
    if depth_meters > threshold && !has_permit {
        return Err(WaterLawError::MissingWellDrillingPermit {
            threshold: WELL_PERMIT_DEPTH_THRESHOLD_M,
        });
    }
    Ok(())
}

/// Validate groundwater extraction (ກວດສອບການສູບນໍ້າໃຕ້ດິນ)
///
/// Article 76: Groundwater extraction limits
///
/// # Arguments
/// * `actual_extraction_m3_day` - Actual extraction in m3/day
/// * `sustainable_yield_m3_day` - Sustainable yield in m3/day
///
/// # Returns
/// * `Ok(())` if extraction is within sustainable yield
/// * `Err(WaterLawError)` if extraction exceeds sustainable yield
pub fn validate_groundwater_extraction(
    actual_extraction_m3_day: f64,
    sustainable_yield_m3_day: f64,
) -> Result<()> {
    if actual_extraction_m3_day < 0.0 {
        return Err(WaterLawError::ValidationError {
            message: "Extraction cannot be negative".to_string(),
        });
    }

    if actual_extraction_m3_day > sustainable_yield_m3_day {
        return Err(WaterLawError::GroundwaterExtractionExceeds {
            actual: actual_extraction_m3_day,
            limit: sustainable_yield_m3_day,
        });
    }
    Ok(())
}

/// Validate aquifer protection zone activity (ກວດສອບກິດຈະກຳໃນເຂດປ້ອງກັນຊັ້ນນໍ້າໃຕ້ດິນ)
///
/// Article 77: Aquifer protection zone restrictions
///
/// # Arguments
/// * `zone` - Aquifer protection zone
/// * `activity` - Proposed activity
///
/// # Returns
/// * `Ok(())` if activity is permitted
/// * `Err(WaterLawError)` if activity is prohibited
pub fn validate_aquifer_zone_activity(zone: &AquiferProtectionZone, activity: &str) -> Result<()> {
    for prohibited in &zone.prohibited_activities {
        if activity.to_lowercase().contains(&prohibited.to_lowercase()) {
            return Err(WaterLawError::AquiferProtectionZoneViolation {
                activity: activity.to_string(),
            });
        }
    }
    Ok(())
}

/// Validate groundwater monitoring compliance (ກວດສອບການປະຕິບັດຕາມການຕິດຕາມນໍ້າໃຕ້ດິນ)
///
/// Article 78: Groundwater monitoring requirements
///
/// # Arguments
/// * `last_monitoring_date` - Date of last monitoring (YYYY-MM-DD)
/// * `current_date` - Current date (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` if monitoring is up to date
/// * `Err(WaterLawError)` if monitoring is overdue
pub fn validate_groundwater_monitoring(
    last_monitoring_date: &str,
    current_date: &str,
) -> Result<()> {
    if last_monitoring_date > current_date {
        return Err(WaterLawError::ValidationError {
            message: "Last monitoring date cannot be in the future".to_string(),
        });
    }

    let days_since = estimate_days_between(last_monitoring_date, current_date);

    if days_since > GROUNDWATER_MONITORING_INTERVAL_DAYS {
        return Err(WaterLawError::MissingGroundwaterMonitoring { days_since });
    }

    Ok(())
}

/// Validate well permit (ກວດສອບໃບອະນຸຍາດບໍ່ນໍ້າ)
///
/// Article 75: Well permit validation
///
/// # Arguments
/// * `permit` - Well permit
/// * `current_date` - Current date (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(WaterLawError)` if permit is invalid
pub fn validate_well_permit(permit: &WellPermit, current_date: &str) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(WaterLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    // Check status
    match permit.status {
        WaterPermitStatus::Active | WaterPermitStatus::Renewed => {
            if current_date > permit.expiry_date.as_str() {
                return Err(WaterLawError::PermitExpired {
                    permit_number: permit.permit_number.clone(),
                    expiry_date: permit.expiry_date.clone(),
                });
            }
        }
        WaterPermitStatus::Expired => {
            return Err(WaterLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        WaterPermitStatus::Suspended => {
            return Err(WaterLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: "ໂຈະ (suspended)".to_string(),
            });
        }
        WaterPermitStatus::Revoked => {
            return Err(WaterLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: "ຖືກຖອນ (revoked)".to_string(),
            });
        }
        WaterPermitStatus::Pending => {
            return Err(WaterLawError::ValidationError {
                message: "Permit is still pending approval".to_string(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Pollution Prevention Validation (ການກວດສອບການປ້ອງກັນມົນລະພິດ)
// ============================================================================

/// Validate polluter pays compliance (ກວດສອບການປະຕິບັດຕາມຫຼັກການຜູ້ກໍ່ມົນລະພິດຕ້ອງຈ່າຍ)
///
/// Article 80: Polluter pays principle
///
/// # Arguments
/// * `record` - Polluter record
///
/// # Returns
/// * `Ok(())` if remediation is complete or in progress
/// * `Err(WaterLawError)` if remediation is not being addressed
pub fn validate_polluter_pays(record: &PolluterRecord) -> Result<()> {
    if record.remediation_required
        && matches!(
            record.remediation_status,
            RemediationStatus::Pending | RemediationStatus::NonCompliant
        )
    {
        return Err(WaterLawError::PolluterPaysViolation {
            polluter: record.polluter_name.clone(),
            cost: record.remediation_cost_lak,
        });
    }
    Ok(())
}

/// Validate agricultural runoff control (ກວດສອບການຄວບຄຸມນໍ້າໄຫຼລົ້ນຈາກກະສິກຳ)
///
/// Article 82: Agricultural runoff requirements
///
/// # Arguments
/// * `contaminant` - Contaminant detected
/// * `level` - Contaminant level
/// * `threshold` - Maximum allowed level
///
/// # Returns
/// * `Ok(())` if runoff is within limits
/// * `Err(WaterLawError)` if runoff exceeds limits
pub fn validate_agricultural_runoff(contaminant: &str, level: f64, threshold: f64) -> Result<()> {
    if level > threshold {
        return Err(WaterLawError::AgriculturalRunoffViolation {
            contaminant: contaminant.to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Water Allocation Validation (ການກວດສອບການຈັດສັນນໍ້າ)
// ============================================================================

/// Validate seasonal water allocation (ກວດສອບການຈັດສັນນໍ້າຕາມລະດູການ)
///
/// Article 41: Seasonal allocation rules
///
/// # Arguments
/// * `allocation` - Water allocation
/// * `actual_usage_m3` - Actual water usage in m3
///
/// # Returns
/// * `Ok(())` if usage is within allocation
/// * `Err(WaterLawError)` if usage exceeds allocation
pub fn validate_seasonal_allocation(
    allocation: &WaterAllocation,
    actual_usage_m3: f64,
    use_type: WaterUseType,
) -> Result<()> {
    let allocated = match use_type {
        WaterUseType::Domestic => allocation.domestic_allocation_m3,
        WaterUseType::Agricultural => allocation.agricultural_allocation_m3,
        WaterUseType::Industrial => allocation.industrial_allocation_m3,
        WaterUseType::Hydropower => allocation.hydropower_allocation_m3,
        _ => allocation.total_available_m3,
    };

    if actual_usage_m3 > allocated {
        let percentage = (actual_usage_m3 / allocated) * 100.0;
        return Err(WaterLawError::SeasonalAllocationViolation {
            season: allocation.season.lao_name().to_string(),
            actual: percentage,
        });
    }
    Ok(())
}

/// Validate drought protocol compliance (ກວດສອບການປະຕິບັດຕາມລະບຽບການແຫ້ງແລ້ງ)
///
/// Article 42: Drought management protocols
///
/// # Arguments
/// * `drought_level` - Current drought level
/// * `use_type` - Type of water use
/// * `requested_usage_m3` - Requested water usage
/// * `baseline_allocation_m3` - Normal baseline allocation
///
/// # Returns
/// * `Ok(f64)` - Adjusted allocation after drought restrictions
/// * `Err(WaterLawError)` if request violates drought protocol
pub fn validate_drought_protocol(
    drought_level: DroughtLevel,
    use_type: WaterUseType,
    requested_usage_m3: f64,
    baseline_allocation_m3: f64,
) -> Result<f64> {
    let restrictions = drought_level.use_restrictions();

    let reduction_pct = match use_type {
        WaterUseType::Domestic => restrictions.domestic_reduction_pct,
        WaterUseType::Agricultural => restrictions.agricultural_reduction_pct,
        WaterUseType::Industrial => restrictions.industrial_reduction_pct,
        WaterUseType::Hydropower => restrictions.hydropower_reduction_pct,
        _ => 0.0,
    };

    let adjusted_allocation = baseline_allocation_m3 * (1.0 - reduction_pct / 100.0);

    if requested_usage_m3 > adjusted_allocation {
        return Err(WaterLawError::DroughtProtocolViolation {
            violation: format!(
                "Requested {} m3 exceeds drought-adjusted allocation {} m3",
                requested_usage_m3, adjusted_allocation
            ),
        });
    }

    Ok(adjusted_allocation)
}

// ============================================================================
// Wetland Protection Validation (ການກວດສອບການປ້ອງກັນທີ່ດິນບຶງ)
// ============================================================================

/// Validate wetland protection (ກວດສອບການປ້ອງກັນທີ່ດິນບຶງ)
///
/// Article 20: Wetland protection requirements
///
/// # Arguments
/// * `wetland_name` - Name of the wetland
/// * `activity` - Proposed activity
/// * `significance` - Ecological significance
///
/// # Returns
/// * `Ok(())` if activity is permitted
/// * `Err(WaterLawError)` if activity is prohibited
pub fn validate_wetland_protection(
    wetland_name: &str,
    activity: &str,
    significance: EcologicalSignificance,
) -> Result<()> {
    let prohibited_activities = match significance {
        EcologicalSignificance::International | EcologicalSignificance::National => {
            vec![
                "drainage",
                "filling",
                "construction",
                "mining",
                "industrial",
            ]
        }
        EcologicalSignificance::Regional => {
            vec!["drainage", "filling", "mining"]
        }
        EcologicalSignificance::Local => {
            vec!["drainage"]
        }
    };

    for prohibited in prohibited_activities {
        if activity.to_lowercase().contains(prohibited) {
            return Err(WaterLawError::WetlandProtectionViolation {
                activity: activity.to_string(),
                wetland_name: wetland_name.to_string(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive water law compliance validation
/// ກວດສອບການປະຕິບັດຕາມກົດໝາຍນໍ້າແບບຄົບຖ້ວນ
///
/// # Arguments
/// * `permit` - Water use right permit
/// * `hydropower` - Optional hydropower concession
/// * `current_date` - Current date for expiry checks
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(WaterLawError)` - Critical violation found
pub fn validate_water_law_compliance(
    permit: &WaterUseRight,
    hydropower: Option<&HydropowerConcession>,
    current_date: &str,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Validate permit
    validate_water_permit(permit, current_date)?;

    // Validate hydropower if provided
    if let Some(hp) = hydropower {
        validate_hydropower_concession(hp, current_date)?;

        // Check for warnings
        if hp.minimum_environmental_flow_m3s < 1.0 {
            warnings.push("Low minimum environmental flow - review ecosystem impact".to_string());
        }

        if hp.resettlement_plan.is_none() && hp.reservoir_area_hectares.is_some() {
            warnings.push(
                "Reservoir project without resettlement plan - verify no affected communities"
                    .to_string(),
            );
        }
    }

    // Check MRC requirements for Mekong water sources
    if permit.water_source.requires_mrc_procedures() {
        warnings.push(
            "Mekong mainstream water source - ensure MRC procedures are followed".to_string(),
        );
    }

    Ok(warnings)
}

// ============================================================================
// Helper Functions (ຟັງຊັນຊ່ວຍເຫຼືອ)
// ============================================================================

/// Helper function to estimate days between two dates
/// This is a simplified implementation - in production, use chrono
fn estimate_days_between(date1: &str, date2: &str) -> u32 {
    // Parse years
    let year1: i32 = date1.get(0..4).and_then(|s| s.parse().ok()).unwrap_or(0);
    let year2: i32 = date2.get(0..4).and_then(|s| s.parse().ok()).unwrap_or(0);

    // Parse months
    let month1: i32 = date1.get(5..7).and_then(|s| s.parse().ok()).unwrap_or(1);
    let month2: i32 = date2.get(5..7).and_then(|s| s.parse().ok()).unwrap_or(1);

    // Parse days
    let day1: i32 = date1.get(8..10).and_then(|s| s.parse().ok()).unwrap_or(1);
    let day2: i32 = date2.get(8..10).and_then(|s| s.parse().ok()).unwrap_or(1);

    // Rough estimate: 365 days per year, 30 days per month
    let total_days1 = year1 * 365 + month1 * 30 + day1;
    let total_days2 = year2 * 365 + month2 * 30 + day2;

    (total_days2 - total_days1).unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_use_permit_requirement() {
        // Domestic use doesn't require permit
        assert!(validate_water_use_permit(WaterUseType::Domestic, false).is_ok());

        // Industrial use requires permit
        assert!(validate_water_use_permit(WaterUseType::Industrial, false).is_err());
        assert!(validate_water_use_permit(WaterUseType::Industrial, true).is_ok());
    }

    #[test]
    fn test_extraction_limit() {
        assert!(validate_extraction_limit(100.0, 200.0).is_ok());
        assert!(validate_extraction_limit(200.0, 200.0).is_ok());
        assert!(validate_extraction_limit(201.0, 200.0).is_err());
        assert!(validate_extraction_limit(-10.0, 200.0).is_err());
    }

    #[test]
    fn test_water_use_priority() {
        // Domestic has priority over industrial
        assert!(
            validate_water_use_priority(WaterUseType::Domestic, WaterUseType::Industrial, false)
                .is_ok()
        );

        // During shortage, domestic must be protected
        assert!(
            validate_water_use_priority(WaterUseType::Industrial, WaterUseType::Domestic, true)
                .is_err()
        );
    }

    #[test]
    fn test_hydropower_category() {
        assert_eq!(
            validate_hydropower_category(10.0).unwrap(),
            HydropowerCategory::Small
        );
        assert_eq!(
            validate_hydropower_category(50.0).unwrap(),
            HydropowerCategory::Medium
        );
        assert_eq!(
            validate_hydropower_category(150.0).unwrap(),
            HydropowerCategory::Large
        );
        assert!(validate_hydropower_category(-10.0).is_err());
    }

    #[test]
    fn test_minimum_environmental_flow() {
        assert!(validate_minimum_environmental_flow(10.0, 5.0).is_ok());
        assert!(validate_minimum_environmental_flow(5.0, 5.0).is_ok());
        assert!(validate_minimum_environmental_flow(3.0, 5.0).is_err());
    }

    #[test]
    fn test_drinking_water_quality() {
        // pH within range
        assert!(validate_drinking_water_quality(WaterQualityParameter::Ph, 7.0).is_ok());

        // pH out of range
        assert!(validate_drinking_water_quality(WaterQualityParameter::Ph, 5.0).is_err());

        // Turbidity within limit
        assert!(validate_drinking_water_quality(WaterQualityParameter::Turbidity, 3.0).is_ok());

        // Turbidity exceeds limit
        assert!(validate_drinking_water_quality(WaterQualityParameter::Turbidity, 10.0).is_err());

        // Arsenic within limit
        assert!(validate_drinking_water_quality(WaterQualityParameter::Arsenic, 0.005).is_ok());

        // Arsenic exceeds limit
        assert!(validate_drinking_water_quality(WaterQualityParameter::Arsenic, 0.02).is_err());
    }

    #[test]
    fn test_industrial_discharge() {
        assert!(validate_industrial_discharge(WaterQualityParameter::Bod, 15.0).is_ok());
        assert!(validate_industrial_discharge(WaterQualityParameter::Bod, 25.0).is_err());
        assert!(validate_industrial_discharge(WaterQualityParameter::Cod, 100.0).is_ok());
        assert!(validate_industrial_discharge(WaterQualityParameter::Cod, 150.0).is_err());
    }

    #[test]
    fn test_well_drilling_permit() {
        assert!(validate_well_drilling_permit(10.0, false).is_ok());
        assert!(validate_well_drilling_permit(25.0, false).is_err());
        assert!(validate_well_drilling_permit(25.0, true).is_ok());
    }

    #[test]
    fn test_groundwater_extraction() {
        assert!(validate_groundwater_extraction(500.0, 1000.0).is_ok());
        assert!(validate_groundwater_extraction(1000.0, 1000.0).is_ok());
        assert!(validate_groundwater_extraction(1001.0, 1000.0).is_err());
    }

    #[test]
    fn test_irrigation_fee_calculation() {
        assert_eq!(calculate_irrigation_fee(1.0), 150_000);
        assert_eq!(calculate_irrigation_fee(10.0), 1_500_000);
    }

    #[test]
    fn test_drought_protocol() {
        // Normal conditions - no reduction
        let result = validate_drought_protocol(
            DroughtLevel::Normal,
            WaterUseType::Agricultural,
            1000.0,
            1000.0,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1000.0);

        // Emergency conditions - 50% reduction for agriculture
        let result = validate_drought_protocol(
            DroughtLevel::Emergency,
            WaterUseType::Agricultural,
            600.0,
            1000.0,
        );
        assert!(result.is_err());

        let result = validate_drought_protocol(
            DroughtLevel::Emergency,
            WaterUseType::Agricultural,
            400.0,
            1000.0,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_wetland_protection() {
        // International significance - many activities prohibited
        assert!(
            validate_wetland_protection(
                "Ramsar Site",
                "ecotourism",
                EcologicalSignificance::International
            )
            .is_ok()
        );

        assert!(
            validate_wetland_protection(
                "Ramsar Site",
                "drainage project",
                EcologicalSignificance::International
            )
            .is_err()
        );

        // Local significance - fewer restrictions
        assert!(
            validate_wetland_protection(
                "Local Wetland",
                "construction",
                EcologicalSignificance::Local
            )
            .is_ok()
        );
    }

    #[test]
    fn test_mrc_prior_consultation() {
        let mekong_mainstream = WaterSourceType::MekongRiverSystem {
            location: MekongLocation::Mainstream,
            section_name: None,
            distance_from_border_km: None,
        };

        assert!(validate_mrc_prior_consultation(&mekong_mainstream, "hydropower", false).is_err());
        assert!(validate_mrc_prior_consultation(&mekong_mainstream, "hydropower", true).is_ok());

        let tributary = WaterSourceType::MekongRiverSystem {
            location: MekongLocation::MinorTributary,
            section_name: None,
            distance_from_border_km: None,
        };

        assert!(validate_mrc_prior_consultation(&tributary, "irrigation", false).is_ok());
    }

    #[test]
    fn test_groundwater_monitoring() {
        assert!(validate_groundwater_monitoring("2026-01-01", "2026-01-15").is_ok());
        assert!(validate_groundwater_monitoring("2026-01-01", "2026-05-01").is_err());
    }

    #[test]
    fn test_estimate_days_between() {
        assert_eq!(estimate_days_between("2026-01-01", "2026-01-31"), 30);
        assert_eq!(estimate_days_between("2026-01-01", "2026-02-01"), 30);
    }
}
