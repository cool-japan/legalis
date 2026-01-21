//! Environmental Law Validators (ຕົວກວດສອບກົດໝາຍສິ່ງແວດລ້ອມ)
//!
//! Validation functions for Lao environmental law compliance based on:
//! - **Environmental Protection Law 2012** (Law No. 29/NA)
//!
//! # Legal References
//! - Articles 18-24: Environmental Impact Assessment
//! - Articles 25-27: Environmental Permits
//! - Articles 28-35: Pollution Control
//! - Articles 38-45: Protected Area Management

use super::error::{EnvironmentalLawError, Result};
use super::types::*;

// ============================================================================
// EIA Validation (ການກວດສອບ EIA)
// ============================================================================

/// Validate EIA requirement for a project (ກວດສອບຄວາມຕ້ອງການ EIA ສຳລັບໂຄງການ)
///
/// Article 18: Projects of certain types and scales require environmental
/// impact assessment before commencement.
///
/// # Arguments
/// * `project_type` - The type of project to validate
///
/// # Returns
/// * `Ok(Option<EIACategory>)` - EIA category required, or None if exempt
/// * `Err(EnvironmentalLawError)` if validation fails
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// let project = ProjectType::Mining { area_hectares: 150.0, mineral_type: None };
/// let result = validate_eia_requirement(&project);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), Some(EIACategory::CategoryA));
/// ```
pub fn validate_eia_requirement(project_type: &ProjectType) -> Result<Option<EIACategory>> {
    // Determine EIA category based on project type and scale
    let category = project_type.eia_category();

    // Validate specific project types have required parameters
    match project_type {
        ProjectType::Mining { area_hectares, .. } => {
            if *area_hectares < 0.0 {
                return Err(EnvironmentalLawError::ValidationError {
                    message: "Mining area cannot be negative".to_string(),
                });
            }
        }
        ProjectType::Hydropower { capacity_mw, .. } => {
            if *capacity_mw < 0.0 {
                return Err(EnvironmentalLawError::ValidationError {
                    message: "Hydropower capacity cannot be negative".to_string(),
                });
            }
        }
        ProjectType::Agricultural { area_hectares, .. } => {
            if *area_hectares < 0.0 {
                return Err(EnvironmentalLawError::ValidationError {
                    message: "Agricultural area cannot be negative".to_string(),
                });
            }
        }
        ProjectType::WasteManagement {
            capacity_tons_per_day,
            ..
        } => {
            if *capacity_tons_per_day < 0.0 {
                return Err(EnvironmentalLawError::ValidationError {
                    message: "Waste management capacity cannot be negative".to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(category)
}

/// Validate EIA completeness (ກວດສອບຄວາມຄົບຖ້ວນຂອງ EIA)
///
/// Article 20: EIA report must include:
/// - Project description
/// - Baseline environmental conditions
/// - Impact identification and assessment
/// - Mitigation measures
/// - Environmental Management Plan
/// - Public consultation results (for Category A)
///
/// # Arguments
/// * `eia` - The EIA to validate
///
/// # Returns
/// * `Ok(())` if EIA is complete
/// * `Err(EnvironmentalLawError)` if EIA is incomplete
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// let eia = EnvironmentalImpactAssessmentBuilder::new()
///     .project_name_lao("ໂຄງການທົດສອບ")
///     .project_name_en("Test Project")
///     .project_type(ProjectType::Industrial { factory_type: "Manufacturing".to_string(), capacity_description: None })
///     .project_developer("Test Company")
///     .location_province("Vientiane Capital")
///     .assessment_date("2026-01-15")
///     .eia_category(EIACategory::CategoryB)
///     .public_consultation(true, 5)
///     .build();
///
/// let result = validate_eia_completeness(&eia);
/// // Note: Will return error because no impacts or mitigations are added
/// ```
pub fn validate_eia_completeness(eia: &EnvironmentalImpactAssessment) -> Result<()> {
    // Check required fields
    if eia.project_name_lao.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "project_name_lao".to_string(),
        });
    }

    if eia.project_name_en.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "project_name_en".to_string(),
        });
    }

    if eia.project_developer.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "project_developer".to_string(),
        });
    }

    if eia.location_province.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "location_province".to_string(),
        });
    }

    if eia.assessment_date.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "assessment_date".to_string(),
        });
    }

    // Check that impacts have been assessed
    if eia.assessed_impacts.is_empty() {
        return Err(EnvironmentalLawError::IncompleteEIA {
            missing_component: "assessed environmental impacts".to_string(),
        });
    }

    // Check that significant impacts have mitigation measures
    let significant_impacts: Vec<_> = eia
        .assessed_impacts
        .iter()
        .filter(|i| i.severity().requires_mitigation())
        .collect();

    if !significant_impacts.is_empty() && eia.mitigation_measures.is_empty() {
        return Err(EnvironmentalLawError::MissingMitigationMeasures {
            impact_type: significant_impacts[0].lao_name().to_string(),
            severity: format!("{:?}", significant_impacts[0].severity()),
        });
    }

    // Category A projects require additional components
    if eia.eia_category == EIACategory::CategoryA {
        // Public consultation required
        if !eia.public_consultation_conducted {
            return Err(EnvironmentalLawError::MissingPublicConsultation);
        }

        // Environmental Management Plan required
        if !eia.has_management_plan {
            return Err(EnvironmentalLawError::MissingManagementPlan);
        }
    }

    Ok(())
}

/// Validate EIA approval status and expiry (ກວດສອບສະຖານະ ແລະ ວັນໝົດອາຍຸຂອງ EIA)
///
/// Article 22: EIA certificates have validity periods
///
/// # Arguments
/// * `eia` - The EIA to validate
/// * `current_date` - Current date for expiry check (YYYY-MM-DD format)
///
/// # Returns
/// * `Ok(())` if EIA is valid and not expired
/// * `Err(EnvironmentalLawError)` if EIA is invalid or expired
pub fn validate_eia_approval(
    eia: &EnvironmentalImpactAssessment,
    current_date: &str,
) -> Result<()> {
    // Check approval status
    match eia.approval_status {
        EIAApprovalStatus::Approved | EIAApprovalStatus::ApprovedWithConditions => {
            // Check expiry
            if let Some(expiry) = &eia.certificate_expiry
                && current_date > expiry.as_str()
            {
                return Err(EnvironmentalLawError::EIAExpired {
                    expiry_date: expiry.clone(),
                    project_date: current_date.to_string(),
                });
            }
            Ok(())
        }
        EIAApprovalStatus::Expired => Err(EnvironmentalLawError::EIAExpired {
            expiry_date: eia.certificate_expiry.clone().unwrap_or_default(),
            project_date: current_date.to_string(),
        }),
        EIAApprovalStatus::Rejected | EIAApprovalStatus::Suspended => {
            Err(EnvironmentalLawError::ValidationError {
                message: format!("EIA status is {:?}", eia.approval_status),
            })
        }
        _ => Err(EnvironmentalLawError::MissingEIA),
    }
}

// ============================================================================
// Air Quality Validation (ການກວດສອບຄຸນນະພາບອາກາດ)
// ============================================================================

/// Validate air quality against national standards (ກວດສອບຄຸນນະພາບອາກາດຕາມມາດຕະຖານແຫ່ງຊາດ)
///
/// Article 30: National ambient air quality standards
///
/// # Arguments
/// * `pollutant` - Type of air pollutant
/// * `concentration` - Measured concentration
/// * `unit` - Measurement unit
///
/// # Returns
/// * `Ok(())` if air quality meets standards
/// * `Err(EnvironmentalLawError)` if air quality exceeds limits
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// // Check PM2.5 annual average
/// let result = validate_air_quality(AirPollutant::PM25, 20.0, "μg/m³");
/// assert!(result.is_ok());
///
/// // Check PM2.5 exceeding limit
/// let result = validate_air_quality(AirPollutant::PM25, 30.0, "μg/m³");
/// assert!(result.is_err());
/// ```
pub fn validate_air_quality(pollutant: AirPollutant, concentration: f64, unit: &str) -> Result<()> {
    let (limit, standard_unit) = match pollutant {
        AirPollutant::PM25 => (MAX_PM25_ANNUAL, "μg/m³"),
        AirPollutant::PM10 => (MAX_PM10_ANNUAL, "μg/m³"),
        AirPollutant::SO2 => (80.0, "μg/m³"),  // Annual average
        AirPollutant::NO2 => (100.0, "μg/m³"), // Annual average
        AirPollutant::CO => (10.0, "mg/m³"),   // 8-hour average
        AirPollutant::O3 => (160.0, "μg/m³"),  // 8-hour average
        AirPollutant::Lead => (0.5, "μg/m³"),  // Annual average
        AirPollutant::VOC => (200.0, "μg/m³"), // General limit
        AirPollutant::H2S => (7.0, "μg/m³"),   // 24-hour average
        AirPollutant::NH3 => (200.0, "μg/m³"), // 24-hour average
    };

    // Validate concentration
    if concentration < 0.0 {
        return Err(EnvironmentalLawError::ValidationError {
            message: "Concentration cannot be negative".to_string(),
        });
    }

    if concentration > limit {
        return Err(EnvironmentalLawError::AirQualityExceedsLimit {
            pollutant: pollutant.lao_name().to_string(),
            actual: concentration,
            limit,
            unit: unit.to_string(),
        });
    }

    // Warn about unit mismatch (informational)
    if unit != standard_unit {
        // In production, this would log a warning
    }

    Ok(())
}

/// Validate multiple air quality parameters (ກວດສອບພາລາມິເຕີຄຸນນະພາບອາກາດຫຼາຍອັນ)
///
/// # Arguments
/// * `measurements` - Vector of (pollutant, concentration, unit) tuples
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (if any)
/// * `Err(EnvironmentalLawError)` - First violation found
pub fn validate_air_quality_batch(
    measurements: &[(AirPollutant, f64, &str)],
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    for (pollutant, concentration, unit) in measurements {
        validate_air_quality(*pollutant, *concentration, unit)?;

        // Add warnings for values approaching limits (>80% of limit)
        let limit = match pollutant {
            AirPollutant::PM25 => MAX_PM25_ANNUAL,
            AirPollutant::PM10 => MAX_PM10_ANNUAL,
            _ => continue,
        };

        if *concentration > limit * 0.8 {
            warnings.push(format!(
                "{} at {:.1}% of limit ({} {})",
                pollutant.lao_name(),
                (concentration / limit) * 100.0,
                concentration,
                unit
            ));
        }
    }

    Ok(warnings)
}

// ============================================================================
// Water Quality Validation (ການກວດສອບຄຸນນະພາບນ້ຳ)
// ============================================================================

/// Validate water quality against discharge limits (ກວດສອບຄຸນນະພາບນ້ຳຕາມມາດຕະຖານການປ່ອຍ)
///
/// Article 31: Effluent discharge standards
///
/// # Arguments
/// * `parameter` - Type of water quality parameter
/// * `value` - Measured value
/// * `unit` - Measurement unit
///
/// # Returns
/// * `Ok(())` if water quality meets standards
/// * `Err(EnvironmentalLawError)` if water quality exceeds limits
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// // Check BOD discharge
/// let result = validate_water_quality(WaterPollutant::BOD, 15.0, "mg/L");
/// assert!(result.is_ok());
///
/// // Check BOD exceeding limit
/// let result = validate_water_quality(WaterPollutant::BOD, 25.0, "mg/L");
/// assert!(result.is_err());
/// ```
pub fn validate_water_quality(parameter: WaterPollutant, value: f64, unit: &str) -> Result<()> {
    match parameter {
        WaterPollutant::BOD => {
            if value > MAX_BOD_DISCHARGE {
                return Err(EnvironmentalLawError::WaterQualityExceedsLimit {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: MAX_BOD_DISCHARGE,
                    unit: unit.to_string(),
                });
            }
        }
        WaterPollutant::COD => {
            if value > MAX_COD_DISCHARGE {
                return Err(EnvironmentalLawError::WaterQualityExceedsLimit {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: MAX_COD_DISCHARGE,
                    unit: unit.to_string(),
                });
            }
        }
        WaterPollutant::TSS => {
            if value > MAX_TSS_DISCHARGE {
                return Err(EnvironmentalLawError::WaterQualityExceedsLimit {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit: MAX_TSS_DISCHARGE,
                    unit: unit.to_string(),
                });
            }
        }
        WaterPollutant::PH => {
            if !(MIN_PH_DISCHARGE..=MAX_PH_DISCHARGE).contains(&value) {
                return Err(EnvironmentalLawError::PHOutOfRange {
                    actual: value,
                    min: MIN_PH_DISCHARGE,
                    max: MAX_PH_DISCHARGE,
                });
            }
        }
        WaterPollutant::Temperature => {
            if value > MAX_TEMPERATURE_DISCHARGE {
                return Err(EnvironmentalLawError::TemperatureExceedsLimit {
                    actual: value,
                    limit: MAX_TEMPERATURE_DISCHARGE,
                });
            }
        }
        WaterPollutant::OilGrease => {
            let limit = 5.0; // mg/L
            if value > limit {
                return Err(EnvironmentalLawError::WaterQualityExceedsLimit {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit,
                    unit: unit.to_string(),
                });
            }
        }
        WaterPollutant::TotalNitrogen => {
            let limit = 15.0; // mg/L
            if value > limit {
                return Err(EnvironmentalLawError::WaterQualityExceedsLimit {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit,
                    unit: unit.to_string(),
                });
            }
        }
        WaterPollutant::TotalPhosphorus => {
            let limit = 2.0; // mg/L
            if value > limit {
                return Err(EnvironmentalLawError::WaterQualityExceedsLimit {
                    parameter: parameter.lao_name().to_string(),
                    actual: value,
                    limit,
                    unit: unit.to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate comprehensive water discharge (ກວດສອບການປ່ອຍນ້ຳເສຍແບບຄົບຖ້ວນ)
///
/// # Arguments
/// * `bod` - BOD value in mg/L
/// * `cod` - COD value in mg/L
/// * `tss` - TSS value in mg/L
/// * `ph` - pH value
/// * `temperature` - Temperature in °C
///
/// # Returns
/// * `Ok(())` if all parameters meet standards
/// * `Err(EnvironmentalLawError)` if any parameter exceeds limits
pub fn validate_water_discharge_comprehensive(
    bod: f64,
    cod: f64,
    tss: f64,
    ph: f64,
    temperature: f64,
) -> Result<()> {
    validate_water_quality(WaterPollutant::BOD, bod, "mg/L")?;
    validate_water_quality(WaterPollutant::COD, cod, "mg/L")?;
    validate_water_quality(WaterPollutant::TSS, tss, "mg/L")?;
    validate_water_quality(WaterPollutant::PH, ph, "")?;
    validate_water_quality(WaterPollutant::Temperature, temperature, "°C")?;

    Ok(())
}

// ============================================================================
// Noise Level Validation (ການກວດສອບລະດັບສຽງ)
// ============================================================================

/// Validate noise level against zone limits (ກວດສອບລະດັບສຽງຕາມມາດຕະຖານເຂດ)
///
/// Article 32: Noise pollution standards by zone type
///
/// # Arguments
/// * `zone` - Type of zone (residential, commercial, industrial, etc.)
/// * `noise_level` - Measured noise level in decibels
/// * `is_daytime` - Whether measurement is during daytime hours
///
/// # Returns
/// * `Ok(())` if noise level meets standards
/// * `Err(EnvironmentalLawError)` if noise level exceeds limits
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// // Check daytime noise in residential area
/// let result = validate_noise_level(ZoneType::Residential, 50, true);
/// assert!(result.is_ok());
///
/// // Check nighttime noise exceeding residential limit
/// let result = validate_noise_level(ZoneType::Residential, 50, false);
/// assert!(result.is_err());
/// ```
pub fn validate_noise_level(zone: ZoneType, noise_level: u8, is_daytime: bool) -> Result<()> {
    let limit = if is_daytime {
        zone.max_noise_day()
    } else {
        zone.max_noise_night()
    };

    if noise_level > limit {
        return Err(EnvironmentalLawError::NoiseLevelExceedsLimit {
            zone_type: zone.lao_name().to_string(),
            actual: noise_level,
            limit,
            period: if is_daytime {
                "daytime (ກາງວັນ)".to_string()
            } else {
                "nighttime (ກາງຄືນ)".to_string()
            },
        });
    }

    Ok(())
}

/// Validate noise from environmental impact (ກວດສອບສຽງຈາກຜົນກະທົບສິ່ງແວດລ້ອມ)
///
/// # Arguments
/// * `impact` - The noise impact to validate
/// * `zone` - Zone type where noise will occur
/// * `is_daytime` - Whether during daytime hours
///
/// # Returns
/// * `Ok(())` if noise level is acceptable
/// * `Err(EnvironmentalLawError)` if noise exceeds limits
pub fn validate_noise_impact(
    impact: &EnvironmentalImpact,
    zone: ZoneType,
    is_daytime: bool,
) -> Result<()> {
    if let EnvironmentalImpact::NoiseLevel { decibels, .. } = impact {
        validate_noise_level(zone, *decibels, is_daytime)
    } else {
        Err(EnvironmentalLawError::ValidationError {
            message: "Expected NoiseLevel impact type".to_string(),
        })
    }
}

// ============================================================================
// Protected Area Validation (ການກວດສອບເຂດປ່າປ້ອງກັນ)
// ============================================================================

/// Validate activity in protected area (ກວດສອບກິດຈະກຳໃນເຂດປ່າປ້ອງກັນ)
///
/// Article 40: Prohibited and permitted activities in protected areas
///
/// # Arguments
/// * `area` - The protected area
/// * `activity` - The proposed activity
///
/// # Returns
/// * `Ok(())` if activity is permitted
/// * `Err(EnvironmentalLawError)` if activity is prohibited
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// let area = ProtectedArea {
///     name_lao: "ເຂດປ່າປ້ອງກັນນ້ຳງື່ມ".to_string(),
///     name_en: "Nam Ngum Protection Forest".to_string(),
///     area_type: ProtectedAreaType::NationalProtectedArea,
///     area_hectares: 50000.0,
///     province: "Vientiane".to_string(),
///     districts: vec!["Sangthong".to_string()],
///     establishment_date: "1993-01-01".to_string(),
///     iucn_category: Some(IUCNCategory::II),
///     management_authority: None,
///     key_species: vec![],
///     buffer_zone_hectares: Some(5000.0),
/// };
///
/// // Research is allowed
/// let result = validate_protected_area_activity(&area, ProtectedAreaActivity::Research);
/// assert!(result.is_ok());
///
/// // Mining is not allowed
/// let result = validate_protected_area_activity(&area, ProtectedAreaActivity::Mining);
/// assert!(result.is_err());
/// ```
pub fn validate_protected_area_activity(
    area: &ProtectedArea,
    activity: ProtectedAreaActivity,
) -> Result<()> {
    if !activity.is_allowed_in(&area.area_type) {
        return Err(EnvironmentalLawError::UnauthorizedProtectedAreaActivity {
            activity: activity.lao_name().to_string(),
            area_type: area.area_type.lao_name().to_string(),
        });
    }

    Ok(())
}

/// Validate distance from protected area boundary (ກວດສອບໄລຍະຫ່າງຈາກເຂດແດນປ່າປ້ອງກັນ)
///
/// Article 41: Buffer zone requirements
///
/// # Arguments
/// * `distance_meters` - Distance from protected area boundary in meters
/// * `activity` - Type of activity
///
/// # Returns
/// * `Ok(())` if distance requirement is met
/// * `Err(EnvironmentalLawError)` if too close to protected area
pub fn validate_protected_area_distance(
    distance_meters: u32,
    activity: ProtectedAreaActivity,
) -> Result<()> {
    let required_distance = match activity {
        ProtectedAreaActivity::Mining => MIN_MINING_DISTANCE_FROM_PROTECTED_AREA,
        ProtectedAreaActivity::InfrastructureDevelopment => MIN_BUFFER_ZONE_METERS,
        ProtectedAreaActivity::Logging => MIN_BUFFER_ZONE_METERS,
        ProtectedAreaActivity::Agriculture => MIN_BUFFER_ZONE_METERS / 2,
        _ => 0, // Other activities don't have distance requirements
    };

    if distance_meters < required_distance {
        return Err(EnvironmentalLawError::ProtectedAreaBoundaryViolation {
            distance: distance_meters,
            required: required_distance,
        });
    }

    Ok(())
}

/// Check if activity affects endangered species
/// ກວດສອບວ່າກິດຈະກຳມີຜົນກະທົບຕໍ່ຊະນິດພັນທີ່ໃກ້ສູນພັນ
///
/// # Arguments
/// * `area` - Protected area
/// * `activity` - Proposed activity
///
/// # Returns
/// * `Ok(())` if no endangered species threatened
/// * `Err(EnvironmentalLawError)` if endangered species at risk
pub fn validate_endangered_species_impact(
    area: &ProtectedArea,
    activity: ProtectedAreaActivity,
) -> Result<()> {
    // High-impact activities in areas with key species
    let high_impact = matches!(
        activity,
        ProtectedAreaActivity::Mining
            | ProtectedAreaActivity::Logging
            | ProtectedAreaActivity::InfrastructureDevelopment
    );

    if high_impact && !area.key_species.is_empty() {
        return Err(EnvironmentalLawError::EndangeredSpeciesThreat);
    }

    Ok(())
}

// ============================================================================
// Environmental Permit Validation (ການກວດສອບໃບອະນຸຍາດສິ່ງແວດລ້ອມ)
// ============================================================================

/// Validate environmental permit (ກວດສອບໃບອະນຸຍາດສິ່ງແວດລ້ອມ)
///
/// Article 25-27: Environmental permit requirements
///
/// # Arguments
/// * `permit` - The permit to validate
/// * `current_date` - Current date for expiry check (YYYY-MM-DD format)
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(EnvironmentalLawError)` if permit is invalid or expired
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// let permit = EnvironmentalPermit {
///     permit_number: "EP-2026-001".to_string(),
///     holder_name: "Test Company".to_string(),
///     holder_name_lao: Some("ບໍລິສັດທົດສອບ".to_string()),
///     permit_type: EnvironmentalPermitType::EmissionPermit,
///     issue_date: "2024-01-01".to_string(),
///     expiry_date: "2029-01-01".to_string(),
///     issuing_authority: "MONRE".to_string(),
///     conditions: vec![],
///     status: PermitStatus::Active,
///     project_name: None,
///     location_province: None,
/// };
///
/// let result = validate_environmental_permit(&permit, "2026-06-15");
/// assert!(result.is_ok());
/// ```
pub fn validate_environmental_permit(
    permit: &EnvironmentalPermit,
    current_date: &str,
) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    if permit.holder_name.trim().is_empty() {
        return Err(EnvironmentalLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    // Check status
    match permit.status {
        PermitStatus::Active | PermitStatus::Renewed => {
            // Check expiry
            if current_date > permit.expiry_date.as_str() {
                return Err(EnvironmentalLawError::PermitExpired {
                    permit_number: permit.permit_number.clone(),
                    expiry_date: permit.expiry_date.clone(),
                });
            }
        }
        PermitStatus::Expired => {
            return Err(EnvironmentalLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        PermitStatus::Suspended => {
            return Err(EnvironmentalLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: "ໂຈະ (suspended)".to_string(),
            });
        }
        PermitStatus::Revoked => {
            return Err(EnvironmentalLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: "ຖືກຖອນ (revoked)".to_string(),
            });
        }
        PermitStatus::Pending => {
            return Err(EnvironmentalLawError::ValidationError {
                message: "Permit is still pending approval".to_string(),
            });
        }
    }

    // Check conditions compliance
    for condition in &permit.conditions {
        if !condition.compliant {
            return Err(EnvironmentalLawError::PermitConditionViolated {
                condition: condition.description.clone(),
            });
        }
    }

    Ok(())
}

/// Validate permit type for activity (ກວດສອບປະເພດໃບອະນຸຍາດສຳລັບກິດຈະກຳ)
///
/// # Arguments
/// * `permit_type` - Type of permit held
/// * `activity` - Proposed activity
///
/// # Returns
/// * `Ok(())` if permit type is appropriate
/// * `Err(EnvironmentalLawError)` if wrong permit type
pub fn validate_permit_for_activity(
    permit_type: EnvironmentalPermitType,
    activity: &str,
) -> Result<()> {
    let valid = match permit_type {
        EnvironmentalPermitType::EIACertificate => true, // General approval
        EnvironmentalPermitType::EmissionPermit => {
            activity.contains("emission") || activity.contains("air")
        }
        EnvironmentalPermitType::WasteDisposalPermit => {
            activity.contains("waste") || activity.contains("disposal")
        }
        EnvironmentalPermitType::WaterExtractionPermit => {
            activity.contains("water") || activity.contains("extraction")
        }
        EnvironmentalPermitType::ForestryPermit => {
            activity.contains("forest") || activity.contains("logging")
        }
        EnvironmentalPermitType::MiningEnvironmentalPermit => activity.contains("mining"),
        EnvironmentalPermitType::EnvironmentalComplianceCertificate => true,
        EnvironmentalPermitType::HazardousWasteTransportPermit => {
            activity.contains("hazardous") || activity.contains("transport")
        }
    };

    if !valid {
        return Err(EnvironmentalLawError::InvalidPermitForActivity {
            permit_type: permit_type.lao_name().to_string(),
            activity: activity.to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Waste Disposal Validation (ການກວດສອບການກຳຈັດຂີ້ເຫຍື້ອ)
// ============================================================================

/// Validate waste disposal method (ກວດສອບວິທີກຳຈັດຂີ້ເຫຍື້ອ)
///
/// Article 34-35: Waste management requirements
///
/// # Arguments
/// * `waste_type` - Type of waste
/// * `disposal_method` - Proposed disposal method
///
/// # Returns
/// * `Ok(())` if disposal method is appropriate
/// * `Err(EnvironmentalLawError)` if disposal method is improper
///
/// # Example
/// ```
/// use legalis_la::environmental_law::*;
///
/// // Composting is appropriate for organic waste
/// let result = validate_waste_disposal(WasteType::Organic, WasteDisposalMethod::Composting);
/// assert!(result.is_ok());
///
/// // Landfill is not appropriate for hazardous waste
/// let result = validate_waste_disposal(WasteType::Hazardous, WasteDisposalMethod::Landfill);
/// assert!(result.is_err());
/// ```
pub fn validate_waste_disposal(
    waste_type: WasteType,
    disposal_method: WasteDisposalMethod,
) -> Result<()> {
    if !disposal_method.is_appropriate_for(&waste_type) {
        return Err(EnvironmentalLawError::ImproperWasteDisposal {
            waste_type: waste_type.lao_name().to_string(),
            method: disposal_method.lao_name().to_string(),
        });
    }

    // Additional checks for hazardous waste
    if waste_type.is_hazardous() {
        match disposal_method {
            WasteDisposalMethod::Treatment | WasteDisposalMethod::SpecializedFacility => {}
            _ => {
                return Err(EnvironmentalLawError::HazardousWasteViolation {
                    description: format!(
                        "Hazardous waste ({}) requires treatment or specialized facility",
                        waste_type.lao_name()
                    ),
                });
            }
        }
    }

    Ok(())
}

/// Validate hazardous waste transport (ກວດສອບການຂົນສົ່ງຂີ້ເຫຍື້ອອັນຕະລາຍ)
///
/// # Arguments
/// * `waste_type` - Type of waste being transported
/// * `has_permit` - Whether transport permit is held
///
/// # Returns
/// * `Ok(())` if transport is valid
/// * `Err(EnvironmentalLawError)` if permit required but missing
pub fn validate_hazardous_waste_transport(waste_type: WasteType, has_permit: bool) -> Result<()> {
    if waste_type.is_hazardous() && !has_permit {
        return Err(EnvironmentalLawError::MissingWasteTransportPermit {
            waste_type: waste_type.lao_name().to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Pollution Source Validation (ການກວດສອບແຫຼ່ງມົນລະພິດ)
// ============================================================================

/// Validate pollution source compliance (ກວດສອບການປະຕິບັດຕາມຂອງແຫຼ່ງມົນລະພິດ)
///
/// # Arguments
/// * `source` - Pollution source to validate
///
/// # Returns
/// * `Ok(())` if source is compliant
/// * `Err(EnvironmentalLawError)` if source is non-compliant
pub fn validate_pollution_source(source: &PollutionSource) -> Result<()> {
    if !source.compliant {
        return Err(EnvironmentalLawError::NonCompliantPollutionSource {
            source_name: source.source_name.clone(),
        });
    }

    // Validate emission quantity
    if source.emission_quantity < 0.0 {
        return Err(EnvironmentalLawError::ValidationError {
            message: "Emission quantity cannot be negative".to_string(),
        });
    }

    Ok(())
}

/// Validate pollution monitoring schedule (ກວດສອບຕາຕະລາງການຕິດຕາມມົນລະພິດ)
///
/// # Arguments
/// * `last_inspection_date` - Date of last inspection (YYYY-MM-DD)
/// * `current_date` - Current date (YYYY-MM-DD)
/// * `required_interval_days` - Maximum days between inspections
///
/// # Returns
/// * `Ok(())` if monitoring is up to date
/// * `Err(EnvironmentalLawError)` if monitoring is overdue
pub fn validate_pollution_monitoring(
    last_inspection_date: &str,
    current_date: &str,
    required_interval_days: u32,
) -> Result<()> {
    // Simple date comparison (assumes YYYY-MM-DD format)
    if last_inspection_date > current_date {
        return Err(EnvironmentalLawError::ValidationError {
            message: "Last inspection date cannot be in the future".to_string(),
        });
    }

    // Calculate approximate days between dates
    // Note: This is a simplified calculation; in production, use proper date parsing
    let days_since = estimate_days_between(last_inspection_date, current_date);

    if days_since > required_interval_days {
        return Err(EnvironmentalLawError::MissingPollutionMonitoring { days_since });
    }

    Ok(())
}

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

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive environmental compliance validation
/// ກວດສອບການປະຕິບັດຕາມກົດໝາຍສິ່ງແວດລ້ອມແບບຄົບຖ້ວນ
///
/// # Arguments
/// * `eia` - Environmental Impact Assessment (optional)
/// * `permit` - Environmental Permit
/// * `current_date` - Current date for expiry checks
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(EnvironmentalLawError)` - Critical violation found
pub fn validate_environmental_compliance(
    eia: Option<&EnvironmentalImpactAssessment>,
    permit: &EnvironmentalPermit,
    current_date: &str,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Validate permit
    validate_environmental_permit(permit, current_date)?;

    // Validate EIA if provided
    if let Some(eia) = eia {
        validate_eia_completeness(eia)?;
        validate_eia_approval(eia, current_date)?;

        // Check for warnings
        if eia.communities_consulted < 3 {
            warnings.push("Limited public consultation - consider broader engagement".to_string());
        }

        if !eia.has_monitoring_plan {
            warnings.push("Monitoring plan not included - recommended for compliance".to_string());
        }
    }

    // Check permit conditions
    let pending_conditions: Vec<_> = permit.conditions.iter().filter(|c| !c.compliant).collect();

    for cond in pending_conditions {
        warnings.push(format!("Pending condition: {}", cond.description));
    }

    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eia_requirement_mining() {
        let mining = ProjectType::Mining {
            area_hectares: 150.0,
            mineral_type: None,
        };
        let result = validate_eia_requirement(&mining);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(EIACategory::CategoryA));
    }

    #[test]
    fn test_eia_requirement_small_project() {
        let small = ProjectType::Mining {
            area_hectares: 5.0,
            mineral_type: None,
        };
        let result = validate_eia_requirement(&small);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_air_quality_pass() {
        let result = validate_air_quality(AirPollutant::PM25, 20.0, "μg/m³");
        assert!(result.is_ok());
    }

    #[test]
    fn test_air_quality_fail() {
        let result = validate_air_quality(AirPollutant::PM25, 30.0, "μg/m³");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvironmentalLawError::AirQualityExceedsLimit { .. }
        ));
    }

    #[test]
    fn test_water_quality_bod() {
        let pass = validate_water_quality(WaterPollutant::BOD, 15.0, "mg/L");
        assert!(pass.is_ok());

        let fail = validate_water_quality(WaterPollutant::BOD, 25.0, "mg/L");
        assert!(fail.is_err());
    }

    #[test]
    fn test_water_quality_ph() {
        let pass = validate_water_quality(WaterPollutant::PH, 7.0, "");
        assert!(pass.is_ok());

        let fail_low = validate_water_quality(WaterPollutant::PH, 4.0, "");
        assert!(fail_low.is_err());

        let fail_high = validate_water_quality(WaterPollutant::PH, 10.0, "");
        assert!(fail_high.is_err());
    }

    #[test]
    fn test_noise_level() {
        let day_pass = validate_noise_level(ZoneType::Residential, 50, true);
        assert!(day_pass.is_ok());

        let night_fail = validate_noise_level(ZoneType::Residential, 50, false);
        assert!(night_fail.is_err());
    }

    #[test]
    fn test_protected_area_activity() {
        let area = ProtectedArea {
            name_lao: "ເຂດປ່າປ້ອງກັນທົດສອບ".to_string(),
            name_en: "Test Protected Area".to_string(),
            area_type: ProtectedAreaType::NationalProtectedArea,
            area_hectares: 10000.0,
            province: "Test".to_string(),
            districts: vec![],
            establishment_date: "2000-01-01".to_string(),
            iucn_category: None,
            management_authority: None,
            key_species: vec![],
            buffer_zone_hectares: None,
        };

        let research = validate_protected_area_activity(&area, ProtectedAreaActivity::Research);
        assert!(research.is_ok());

        let mining = validate_protected_area_activity(&area, ProtectedAreaActivity::Mining);
        assert!(mining.is_err());
    }

    #[test]
    fn test_waste_disposal() {
        let organic = validate_waste_disposal(WasteType::Organic, WasteDisposalMethod::Composting);
        assert!(organic.is_ok());

        let hazardous =
            validate_waste_disposal(WasteType::Hazardous, WasteDisposalMethod::Landfill);
        assert!(hazardous.is_err());
    }

    #[test]
    fn test_permit_validation() {
        let permit = EnvironmentalPermit {
            permit_number: "EP-2026-001".to_string(),
            holder_name: "Test Company".to_string(),
            holder_name_lao: None,
            permit_type: EnvironmentalPermitType::EmissionPermit,
            issue_date: "2024-01-01".to_string(),
            expiry_date: "2029-01-01".to_string(),
            issuing_authority: "MONRE".to_string(),
            conditions: vec![],
            status: PermitStatus::Active,
            project_name: None,
            location_province: None,
        };

        let valid = validate_environmental_permit(&permit, "2026-06-15");
        assert!(valid.is_ok());

        let expired = validate_environmental_permit(&permit, "2030-01-01");
        assert!(expired.is_err());
    }
}
