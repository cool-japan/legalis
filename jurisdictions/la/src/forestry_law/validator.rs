//! Forestry Law Validators (ຕົວກວດສອບກົດໝາຍປ່າໄມ້)
//!
//! Validation functions for Lao forestry law compliance based on:
//! - **Forestry Law 2019** (Law No. 64/NA)
//!
//! # Legal References
//! - Articles 10-15: Forest Classification
//! - Articles 30-45: Forest Use Rights
//! - Articles 46-60: Timber Harvesting Regulations
//! - Articles 61-75: Forest Concessions
//! - Articles 76-90: Protected Species
//! - Articles 91-105: Community Forestry
//! - Articles 106-120: Penalties and Enforcement
//! - Articles 121-135: Permits and Fees

use super::error::{ForestryLawError, Result};
use super::types::*;

// ============================================================================
// Timber Harvesting Validation (ການກວດສອບການຕັດໄມ້)
// ============================================================================

/// Validate timber harvesting permit (ກວດສອບໃບອະນຸຍາດຕັດໄມ້)
///
/// Article 32: Timber harvesting permit requirements
///
/// # Arguments
/// * `permit` - The timber harvesting permit to validate
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_timber_harvesting_permit(permit: &TimberHarvestingPermit) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    if permit.holder_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    if permit.province.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "province".to_string(),
        });
    }

    // Check permit status
    match permit.status {
        PermitStatus::Active | PermitStatus::Completed => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        PermitStatus::Suspended | PermitStatus::Revoked => {
            return Err(ForestryLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: permit.status.lao_name().to_string(),
            });
        }
        PermitStatus::Pending => {
            return Err(ForestryLawError::ValidationError {
                message: "Permit is still pending approval".to_string(),
            });
        }
    }

    // Validate harvesting season
    validate_harvesting_season(permit.harvesting_month)?;

    // Validate species protection
    validate_species_protection(&permit.species)?;

    // Validate minimum diameter
    validate_minimum_diameter(&permit.species, permit.minimum_diameter_cm)?;

    // Validate forest type allows harvesting
    if !permit.forest_type.allows_limited_harvesting() {
        return Err(ForestryLawError::HarvestingInWrongForestType {
            forest_type: permit.forest_type.lao_name().to_string(),
            article: permit.forest_type.article_number(),
        });
    }

    // Check for quota requirement for Category II species
    if permit.species.protection_category().requires_quota() && permit.quota_reference.is_none() {
        return Err(ForestryLawError::MissingQuotaAllocation {
            species: permit.species.lao_name().to_string(),
        });
    }

    // Validate volume is positive
    if permit.volume_cubic_meters < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "volume_cubic_meters".to_string(),
            value: permit.volume_cubic_meters,
        });
    }

    Ok(())
}

/// Validate harvesting season (ກວດສອບລະດູຕັດໄມ້)
///
/// Article 48: Harvesting is only permitted during dry season (November-April)
///
/// # Arguments
/// * `month` - Month of harvesting (1-12)
///
/// # Returns
/// * `Ok(())` if month is within dry season
/// * `Err(ForestryLawError)` if month is in wet season
pub fn validate_harvesting_season(month: u8) -> Result<()> {
    // Dry season: November (11) to April (4)
    let is_dry_season =
        month >= HARVESTING_SEASON_START_MONTH || month <= HARVESTING_SEASON_END_MONTH;

    if !is_dry_season {
        return Err(ForestryLawError::HarvestingOutsideSeason { month });
    }

    Ok(())
}

/// Validate minimum cutting diameter (ກວດສອບເສັ້ນຜ່ານສູນກາງຂັ້ນຕ່ຳ)
///
/// Article 49: Species-specific minimum diameter requirements
///
/// # Arguments
/// * `species` - Tree species
/// * `diameter_cm` - Tree diameter in centimeters
///
/// # Returns
/// * `Ok(())` if diameter meets minimum
/// * `Err(ForestryLawError)` if diameter is below minimum
pub fn validate_minimum_diameter(species: &TreeSpecies, diameter_cm: u32) -> Result<()> {
    let min_diameter = species.minimum_diameter_cm();

    if diameter_cm < min_diameter {
        return Err(ForestryLawError::DiameterBelowMinimum {
            actual_cm: diameter_cm,
            required_cm: min_diameter,
            species: species.lao_name().to_string(),
        });
    }

    Ok(())
}

/// Validate species protection status (ກວດສອບສະຖານະການປົກປ້ອງຊະນິດພັນ)
///
/// Articles 50, 77: Category I species are strictly protected
///
/// # Arguments
/// * `species` - Tree species to check
///
/// # Returns
/// * `Ok(())` if species can be harvested
/// * `Err(ForestryLawError)` if species is prohibited
pub fn validate_species_protection(species: &TreeSpecies) -> Result<()> {
    if !species.protection_category().allows_harvesting() {
        return Err(ForestryLawError::ProhibitedSpeciesHarvesting {
            species: species.lao_name().to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Concession Validation (ການກວດສອບສຳປະທານ)
// ============================================================================

/// Validate forest concession (ກວດສອບສຳປະທານປ່າໄມ້)
///
/// Articles 61-75: Forest concession requirements
///
/// # Arguments
/// * `concession` - The forest concession to validate
///
/// # Returns
/// * `Ok(())` if concession is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_forest_concession(concession: &ForestConcession) -> Result<()> {
    // Check required fields
    if concession.concession_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "concession_number".to_string(),
        });
    }

    if concession.holder_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    if concession.province.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "province".to_string(),
        });
    }

    // Validate area
    validate_concession_area(&concession.concession_type, concession.area_hectares)?;

    // Validate term
    validate_concession_term(&concession.concession_type, concession.term_years)?;

    // Validate performance bond
    if let Some(project_value) = concession.project_value_lak {
        validate_performance_bond(
            &concession.concession_type,
            concession.performance_bond_lak,
            project_value,
        )?;
    }

    // Check EIA requirement
    if !concession.has_eia {
        return Err(ForestryLawError::MissingEIA);
    }

    // Check management plan requirement
    if !concession.has_management_plan {
        return Err(ForestryLawError::MissingManagementPlan);
    }

    Ok(())
}

/// Validate concession area (ກວດສອບເນື້ອທີ່ສຳປະທານ)
///
/// Articles 62-63: Area limits by concession type
///
/// # Arguments
/// * `concession_type` - Type of concession
/// * `area_hectares` - Area in hectares
///
/// # Returns
/// * `Ok(())` if area is within limits
/// * `Err(ForestryLawError)` if area exceeds maximum
pub fn validate_concession_area(
    concession_type: &ConcessionType,
    area_hectares: f64,
) -> Result<()> {
    if area_hectares < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "area_hectares".to_string(),
            value: area_hectares,
        });
    }

    let max_area = concession_type.max_area_hectares();
    let article = match concession_type {
        ConcessionType::Management => 62,
        ConcessionType::Plantation => 63,
    };

    if area_hectares > max_area {
        return Err(ForestryLawError::ConcessionAreaExceedsMaximum {
            actual_ha: area_hectares,
            max_ha: max_area,
            concession_type: concession_type.lao_name().to_string(),
            article,
        });
    }

    Ok(())
}

/// Validate concession term (ກວດສອບໄລຍະສຳປະທານ)
///
/// Articles 62-63: Term limits by concession type
///
/// # Arguments
/// * `concession_type` - Type of concession
/// * `term_years` - Term in years
///
/// # Returns
/// * `Ok(())` if term is within limits
/// * `Err(ForestryLawError)` if term exceeds maximum
pub fn validate_concession_term(concession_type: &ConcessionType, term_years: u32) -> Result<()> {
    let max_term = concession_type.max_term_years();
    let article = match concession_type {
        ConcessionType::Management => 62,
        ConcessionType::Plantation => 63,
    };

    if term_years > max_term {
        return Err(ForestryLawError::ConcessionTermExceedsMaximum {
            actual_years: term_years,
            max_years: max_term,
            concession_type: concession_type.lao_name().to_string(),
            article,
        });
    }

    Ok(())
}

/// Validate performance bond (ກວດສອບເງິນຄ້ຳປະກັນ)
///
/// Articles 62-63: Performance bond requirements
///
/// # Arguments
/// * `concession_type` - Type of concession
/// * `bond_amount` - Bond amount in LAK
/// * `project_value` - Project value in LAK
///
/// # Returns
/// * `Ok(())` if bond meets minimum
/// * `Err(ForestryLawError)` if bond is insufficient
pub fn validate_performance_bond(
    concession_type: &ConcessionType,
    bond_amount: u64,
    project_value: u64,
) -> Result<()> {
    let required_percent = concession_type.bond_percentage();
    let required_amount = (project_value as f64 * required_percent / 100.0) as u64;

    let article = match concession_type {
        ConcessionType::Management => 62,
        ConcessionType::Plantation => 63,
    };

    if bond_amount < required_amount {
        return Err(ForestryLawError::InsufficientPerformanceBond {
            actual_lak: bond_amount,
            required_lak: required_amount,
            percent: required_percent,
            article,
        });
    }

    Ok(())
}

// ============================================================================
// NTFP Validation (ການກວດສອບ NTFP)
// ============================================================================

/// Validate NTFP permit (ກວດສອບໃບອະນຸຍາດ NTFP)
///
/// Article 33: NTFP collection permit requirements
///
/// # Arguments
/// * `permit` - The NTFP permit to validate
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_ntfp_permit(permit: &NtfpPermit) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    if permit.holder_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    // Check permit status
    match permit.status {
        PermitStatus::Active => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        PermitStatus::Suspended | PermitStatus::Revoked => {
            return Err(ForestryLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: permit.status.lao_name().to_string(),
            });
        }
        _ => {
            return Err(ForestryLawError::ValidationError {
                message: format!("Invalid permit status: {:?}", permit.status),
            });
        }
    }

    // Validate quantity
    if permit.quantity_allowed < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "quantity_allowed".to_string(),
            value: permit.quantity_allowed,
        });
    }

    Ok(())
}

/// Validate NTFP sustainable harvest (ກວດສອບການເກັບກ່ຽວ NTFP ແບບຍືນຍົງ)
///
/// # Arguments
/// * `ntfp_type` - Type of NTFP
/// * `quantity` - Harvested quantity
/// * `permitted_quantity` - Maximum permitted quantity
///
/// # Returns
/// * `Ok(())` if within sustainable limits
/// * `Err(ForestryLawError)` if exceeds limits
pub fn validate_ntfp_sustainable_harvest(
    _ntfp_type: &NtfpType,
    quantity: f64,
    permitted_quantity: f64,
) -> Result<()> {
    if quantity < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "quantity".to_string(),
            value: quantity,
        });
    }

    if quantity > permitted_quantity {
        return Err(ForestryLawError::ValidationError {
            message: format!(
                "Harvested quantity {} exceeds permitted quantity {}",
                quantity, permitted_quantity
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Village Forest Validation (ການກວດສອບປ່າບ້ານ)
// ============================================================================

/// Validate village forest (ກວດສອບປ່າບ້ານ)
///
/// Article 15: Village forest requirements
///
/// # Arguments
/// * `forest` - The village forest to validate
///
/// # Returns
/// * `Ok(())` if village forest is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_village_forest(forest: &VillageForest) -> Result<()> {
    // Check required fields
    if forest.village_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "village_name".to_string(),
        });
    }

    if forest.district.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "district".to_string(),
        });
    }

    if forest.province.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "province".to_string(),
        });
    }

    // Validate area
    if forest.area_hectares < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "area_hectares".to_string(),
            value: forest.area_hectares,
        });
    }

    // Check management agreement requirement
    if !forest.has_management_agreement {
        return Err(ForestryLawError::MissingVillageForestAgreement);
    }

    Ok(())
}

/// Validate village forest agreement (ກວດສອບຂໍ້ຕົກລົງປ່າບ້ານ)
///
/// Article 92: Village forest management agreement requirements
///
/// # Arguments
/// * `agreement` - The agreement to validate
///
/// # Returns
/// * `Ok(())` if agreement is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_village_forest_agreement(agreement: &VillageForestAgreement) -> Result<()> {
    // Check required fields
    if agreement.agreement_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "agreement_number".to_string(),
        });
    }

    if agreement.village_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "village_name".to_string(),
        });
    }

    // Check permit status
    match agreement.status {
        PermitStatus::Active => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: agreement.agreement_number.clone(),
                expiry_date: agreement.end_date.clone(),
            });
        }
        PermitStatus::Suspended | PermitStatus::Revoked => {
            return Err(ForestryLawError::PermitSuspendedOrRevoked {
                permit_number: agreement.agreement_number.clone(),
                status: agreement.status.lao_name().to_string(),
            });
        }
        _ => {}
    }

    // Validate benefit sharing if present
    if let Some(ref sharing) = agreement.benefit_sharing {
        validate_benefit_sharing(sharing)?;
    }

    Ok(())
}

/// Validate benefit sharing arrangement (ກວດສອບການຈັດສັນການແບ່ງປັນຜົນປະໂຫຍດ)
///
/// Article 94: Benefit sharing requirements
///
/// # Arguments
/// * `sharing` - The benefit sharing arrangement
///
/// # Returns
/// * `Ok(())` if percentages are valid
/// * `Err(ForestryLawError)` if percentages don't total 100%
pub fn validate_benefit_sharing(sharing: &BenefitSharingArrangement) -> Result<()> {
    let total = sharing.village_share_percent
        + sharing.district_share_percent
        + sharing.national_share_percent;

    // Allow small floating point tolerance
    if (total - 100.0).abs() > 0.01 {
        return Err(ForestryLawError::InvalidBenefitSharing {
            village: sharing.village_share_percent,
            district: sharing.district_share_percent,
            national: sharing.national_share_percent,
        });
    }

    Ok(())
}

// ============================================================================
// Log Tracking Validation (ການກວດສອບການຕິດຕາມໄມ້ທ່ອນ)
// ============================================================================

/// Validate log entry (ກວດສອບບັນທຶກໄມ້ທ່ອນ)
///
/// Article 51: Log tracking requirements
///
/// # Arguments
/// * `entry` - The log entry to validate
///
/// # Returns
/// * `Ok(())` if log entry is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_log_entry(entry: &LogEntry) -> Result<()> {
    // Check log ID
    if entry.log_id.trim().is_empty() {
        return Err(ForestryLawError::MissingLogMarking);
    }

    // Check harvest permit reference
    if entry.harvest_permit_reference.trim().is_empty() {
        return Err(ForestryLawError::InvalidHarvestPermitReference {
            log_id: entry.log_id.clone(),
        });
    }

    // Validate dimensions
    if entry.length_meters < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "length_meters".to_string(),
            value: entry.length_meters,
        });
    }

    if entry.volume_cubic_meters < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "volume_cubic_meters".to_string(),
            value: entry.volume_cubic_meters,
        });
    }

    // Check species protection
    validate_species_protection(&entry.species)?;

    Ok(())
}

/// Validate transport permit (ກວດສອບໃບອະນຸຍາດຂົນສົ່ງ)
///
/// Article 122: Transport permit requirements
///
/// # Arguments
/// * `permit` - The transport permit to validate
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_transport_permit(permit: &TransportPermit) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    if permit.holder_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    if permit.harvest_permit_reference.trim().is_empty() {
        return Err(ForestryLawError::TransportPermitViolation {
            reason: "Missing harvest permit reference".to_string(),
        });
    }

    if permit.vehicle_registration.trim().is_empty() {
        return Err(ForestryLawError::TransportPermitViolation {
            reason: "Missing vehicle registration".to_string(),
        });
    }

    // Check permit status
    match permit.status {
        PermitStatus::Active => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        _ => {
            return Err(ForestryLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: permit.status.lao_name().to_string(),
            });
        }
    }

    // Validate volume
    if permit.volume_cubic_meters < 0.0 {
        return Err(ForestryLawError::NegativeValueNotAllowed {
            field: "volume_cubic_meters".to_string(),
            value: permit.volume_cubic_meters,
        });
    }

    Ok(())
}

/// Validate chain of custody (ກວດສອບຕ່ອງໂສ້ການຄຸ້ມຄອງ)
///
/// Article 51: Chain of custody requirements
///
/// # Arguments
/// * `entries` - Chain of custody entries
///
/// # Returns
/// * `Ok(())` if chain is complete
/// * `Err(ForestryLawError)` if chain is broken
pub fn validate_chain_of_custody(entries: &[ChainOfCustodyEntry]) -> Result<()> {
    if entries.is_empty() {
        return Err(ForestryLawError::BrokenChainOfCustody {
            gap_description: "No chain of custody entries".to_string(),
        });
    }

    // Check that each entry has required fields
    for (i, entry) in entries.iter().enumerate() {
        if entry.date.trim().is_empty() {
            return Err(ForestryLawError::BrokenChainOfCustody {
                gap_description: format!("Missing date in entry {}", i + 1),
            });
        }

        if entry.handler_name.trim().is_empty() {
            return Err(ForestryLawError::BrokenChainOfCustody {
                gap_description: format!("Missing handler name in entry {}", i + 1),
            });
        }

        // Check location continuity
        if i > 0 {
            let prev_entry = &entries[i - 1];
            if prev_entry.to_location != entry.from_location {
                return Err(ForestryLawError::BrokenChainOfCustody {
                    gap_description: format!(
                        "Location gap between entries {} and {}: '{}' != '{}'",
                        i,
                        i + 1,
                        prev_entry.to_location,
                        entry.from_location
                    ),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// License Validation (ການກວດສອບໃບອະນຸຍາດ)
// ============================================================================

/// Validate sawmill license (ກວດສອບໃບອະນຸຍາດໂຮງເລື່ອຍ)
///
/// Article 123: Sawmill license requirements
///
/// # Arguments
/// * `license` - The sawmill license to validate
///
/// # Returns
/// * `Ok(())` if license is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_sawmill_license(license: &SawmillLicense) -> Result<()> {
    // Check required fields
    if license.license_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "license_number".to_string(),
        });
    }

    if license.facility_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "facility_name".to_string(),
        });
    }

    // Check license status
    match license.status {
        PermitStatus::Active => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: license.license_number.clone(),
                expiry_date: license.expiry_date.clone(),
            });
        }
        PermitStatus::Suspended | PermitStatus::Revoked => {
            return Err(ForestryLawError::PermitSuspendedOrRevoked {
                permit_number: license.license_number.clone(),
                status: license.status.lao_name().to_string(),
            });
        }
        _ => {}
    }

    // Check environmental compliance
    if !license.environmental_compliance {
        return Err(ForestryLawError::EnvironmentalComplianceViolation {
            description: "Environmental compliance not met".to_string(),
        });
    }

    // Check log tracking requirement
    if !license.has_log_tracking {
        return Err(ForestryLawError::MissingLogTrackingSystem);
    }

    Ok(())
}

/// Validate processing facility license (ກວດສອບໃບອະນຸຍາດໂຮງງານແປຮູບ)
///
/// Article 125: Processing facility license requirements
///
/// # Arguments
/// * `license` - The processing facility license to validate
///
/// # Returns
/// * `Ok(())` if license is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_processing_facility_license(license: &ProcessingFacilityLicense) -> Result<()> {
    // Check required fields
    if license.license_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "license_number".to_string(),
        });
    }

    if license.facility_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "facility_name".to_string(),
        });
    }

    // Check license status
    match license.status {
        PermitStatus::Active => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: license.license_number.clone(),
                expiry_date: license.expiry_date.clone(),
            });
        }
        _ => {}
    }

    // Check raw material tracking
    if !license.has_raw_material_tracking {
        return Err(ForestryLawError::MissingLogTrackingSystem);
    }

    Ok(())
}

/// Validate export permit (ກວດສອບໃບອະນຸຍາດສົ່ງອອກ)
///
/// Article 124: Export permit requirements
///
/// # Arguments
/// * `permit` - The export permit to validate
///
/// # Returns
/// * `Ok(())` if permit is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_export_permit(permit: &ForestProductExportPermit) -> Result<()> {
    // Check required fields
    if permit.permit_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "permit_number".to_string(),
        });
    }

    if permit.exporter_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "exporter_name".to_string(),
        });
    }

    if permit.destination_country.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "destination_country".to_string(),
        });
    }

    // Check permit status
    match permit.status {
        PermitStatus::Active => {}
        PermitStatus::Expired => {
            return Err(ForestryLawError::PermitExpired {
                permit_number: permit.permit_number.clone(),
                expiry_date: permit.expiry_date.clone(),
            });
        }
        _ => {
            return Err(ForestryLawError::PermitSuspendedOrRevoked {
                permit_number: permit.permit_number.clone(),
                status: permit.status.lao_name().to_string(),
            });
        }
    }

    // Check log export restriction
    if permit.product_type.is_restricted() {
        return Err(ForestryLawError::LogExportRestricted);
    }

    // Check source permits
    if permit.source_permits.is_empty() {
        return Err(ForestryLawError::MissingSourcePermits);
    }

    // Check CITES requirements
    if let Some(species) = &permit.species
        && species.is_cites_listed()
        && permit.cites_permit_number.is_none()
    {
        return Err(ForestryLawError::CitesPermitRequired {
            species: species.lao_name().to_string(),
            appendix: species.cites_appendix().unwrap_or("II").to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Penalty Validation (ການກວດສອບໂທດ)
// ============================================================================

/// Validate forestry violation (ກວດສອບການລະເມີດກົດໝາຍປ່າໄມ້)
///
/// Articles 106-120: Forestry violations and penalties
///
/// # Arguments
/// * `violation` - The violation to validate
///
/// # Returns
/// * `Ok(())` if violation record is valid
/// * `Err(ForestryLawError)` if validation fails
pub fn validate_forestry_violation(violation: &ForestryViolation) -> Result<()> {
    // Check required fields
    if violation.case_number.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "case_number".to_string(),
        });
    }

    if violation.violator_name.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "violator_name".to_string(),
        });
    }

    if violation.detection_date.trim().is_empty() {
        return Err(ForestryLawError::MissingRequiredField {
            field_name: "detection_date".to_string(),
        });
    }

    // Check penalty assessment for serious violations
    if violation.penalty.is_none() {
        let requires_penalty = matches!(
            violation.violation_type,
            ViolationType::IllegalLogging
                | ViolationType::WildlifeTrafficking
                | ViolationType::ProhibitedSpeciesHarvesting
        );

        if requires_penalty {
            return Err(ForestryLawError::PenaltyAssessmentRequired {
                violation_type: violation.violation_type.lao_name().to_string(),
            });
        }
    }

    Ok(())
}

/// Calculate penalty for violation (ຄຳນວນໂທດສຳລັບການລະເມີດ)
///
/// Articles 107-108: Penalty calculation
///
/// # Arguments
/// * `violation_type` - Type of violation
/// * `base_value` - Base value for calculation (timber/species value)
/// * `is_repeat_offender` - Whether this is a repeat offense
///
/// # Returns
/// * `(min_fine, max_fine)` - Fine range in LAK
pub fn calculate_penalty(
    violation_type: &ViolationType,
    base_value: u64,
    is_repeat_offender: bool,
) -> (u64, u64) {
    let (min_mult, max_mult) = violation_type.fine_multiplier_range();

    let mut min_fine = (base_value as f64 * min_mult) as u64;
    let mut max_fine = (base_value as f64 * max_mult) as u64;

    // Increase for repeat offenders
    if is_repeat_offender {
        min_fine = (min_fine as f64 * 1.5) as u64;
        max_fine = (max_fine as f64 * 2.0) as u64;
    }

    (min_fine, max_fine)
}

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive forestry compliance validation
/// ກວດສອບການປະຕິບັດຕາມກົດໝາຍປ່າໄມ້ແບບຄົບຖ້ວນ
///
/// # Arguments
/// * `permit` - Timber harvesting permit
/// * `log_entries` - Log entries for chain of custody
/// * `transport_permit` - Transport permit (optional)
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(ForestryLawError)` - Critical violation found
pub fn validate_forestry_compliance(
    permit: &TimberHarvestingPermit,
    log_entries: &[LogEntry],
    transport_permit: Option<&TransportPermit>,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Validate harvesting permit
    validate_timber_harvesting_permit(permit)?;

    // Validate all log entries
    for entry in log_entries {
        validate_log_entry(entry)?;

        // Check log matches permit
        if entry.harvest_permit_reference != permit.permit_number {
            return Err(ForestryLawError::InvalidHarvestPermitReference {
                log_id: entry.log_id.clone(),
            });
        }

        // Check species matches permit
        if entry.species != permit.species {
            warnings.push(format!(
                "Log {} species ({}) differs from permit species ({})",
                entry.log_id,
                entry.species.lao_name(),
                permit.species.lao_name()
            ));
        }
    }

    // Validate transport permit if provided
    if let Some(tp) = transport_permit {
        validate_transport_permit(tp)?;

        // Check transport permit matches harvesting permit
        if tp.harvest_permit_reference != permit.permit_number {
            return Err(ForestryLawError::TransportPermitViolation {
                reason: "Transport permit does not reference harvesting permit".to_string(),
            });
        }
    }

    // Check total volume doesn't exceed permit
    let total_volume: f64 = log_entries.iter().map(|e| e.volume_cubic_meters).sum();
    if total_volume > permit.volume_cubic_meters {
        warnings.push(format!(
            "Total logged volume ({:.2} m3) exceeds permit volume ({:.2} m3)",
            total_volume, permit.volume_cubic_meters
        ));
    }

    // Check reforestation requirement
    if permit.reforestation_required && permit.reforestation_area_hectares.is_none() {
        warnings.push("Reforestation area not specified".to_string());
    }

    Ok(warnings)
}

/// Validate CITES compliance for export (ກວດສອບການປະຕິບັດຕາມ CITES ສຳລັບການສົ່ງອອກ)
///
/// Article 80: CITES compliance requirements
///
/// # Arguments
/// * `species` - Tree species
/// * `cites_permit` - CITES permit number (if any)
///
/// # Returns
/// * `Ok(())` if CITES compliant
/// * `Err(ForestryLawError)` if CITES permit required but missing
pub fn validate_cites_compliance(species: &TreeSpecies, cites_permit: Option<&str>) -> Result<()> {
    if species.is_cites_listed() {
        match cites_permit {
            Some(permit) if !permit.trim().is_empty() => Ok(()),
            _ => Err(ForestryLawError::CitesPermitRequired {
                species: species.lao_name().to_string(),
                appendix: species.cites_appendix().unwrap_or("II").to_string(),
            }),
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harvesting_season_dry() {
        // November to April should pass
        assert!(validate_harvesting_season(11).is_ok());
        assert!(validate_harvesting_season(12).is_ok());
        assert!(validate_harvesting_season(1).is_ok());
        assert!(validate_harvesting_season(2).is_ok());
        assert!(validate_harvesting_season(3).is_ok());
        assert!(validate_harvesting_season(4).is_ok());
    }

    #[test]
    fn test_harvesting_season_wet() {
        // May to October should fail
        assert!(validate_harvesting_season(5).is_err());
        assert!(validate_harvesting_season(6).is_err());
        assert!(validate_harvesting_season(7).is_err());
        assert!(validate_harvesting_season(8).is_err());
        assert!(validate_harvesting_season(9).is_err());
        assert!(validate_harvesting_season(10).is_err());
    }

    #[test]
    fn test_minimum_diameter_teak() {
        // Teak minimum is 40 cm
        assert!(validate_minimum_diameter(&TreeSpecies::Teak, 45).is_ok());
        assert!(validate_minimum_diameter(&TreeSpecies::Teak, 40).is_ok());
        assert!(validate_minimum_diameter(&TreeSpecies::Teak, 35).is_err());
    }

    #[test]
    fn test_minimum_diameter_rosewood() {
        // Rosewood is Category I - should fail regardless of diameter
        assert!(validate_species_protection(&TreeSpecies::Rosewood).is_err());
    }

    #[test]
    fn test_species_protection_category_i() {
        assert!(validate_species_protection(&TreeSpecies::Rosewood).is_err());
        assert!(validate_species_protection(&TreeSpecies::Agarwood).is_err());
    }

    #[test]
    fn test_species_protection_category_ii_iii() {
        assert!(validate_species_protection(&TreeSpecies::Teak).is_ok());
        assert!(validate_species_protection(&TreeSpecies::MaiDou).is_ok());
        assert!(validate_species_protection(&TreeSpecies::Pine).is_ok());
    }

    #[test]
    fn test_concession_area_management() {
        // Management concession max is 10,000 ha
        assert!(validate_concession_area(&ConcessionType::Management, 5000.0).is_ok());
        assert!(validate_concession_area(&ConcessionType::Management, 10000.0).is_ok());
        assert!(validate_concession_area(&ConcessionType::Management, 15000.0).is_err());
    }

    #[test]
    fn test_concession_area_plantation() {
        // Plantation concession max is 15,000 ha
        assert!(validate_concession_area(&ConcessionType::Plantation, 10000.0).is_ok());
        assert!(validate_concession_area(&ConcessionType::Plantation, 15000.0).is_ok());
        assert!(validate_concession_area(&ConcessionType::Plantation, 20000.0).is_err());
    }

    #[test]
    fn test_concession_term_management() {
        // Management concession max is 40 years
        assert!(validate_concession_term(&ConcessionType::Management, 30).is_ok());
        assert!(validate_concession_term(&ConcessionType::Management, 40).is_ok());
        assert!(validate_concession_term(&ConcessionType::Management, 50).is_err());
    }

    #[test]
    fn test_concession_term_plantation() {
        // Plantation concession max is 50 years
        assert!(validate_concession_term(&ConcessionType::Plantation, 40).is_ok());
        assert!(validate_concession_term(&ConcessionType::Plantation, 50).is_ok());
        assert!(validate_concession_term(&ConcessionType::Plantation, 60).is_err());
    }

    #[test]
    fn test_performance_bond() {
        // Management concession requires 5% bond
        assert!(
            validate_performance_bond(&ConcessionType::Management, 500_000, 10_000_000).is_ok()
        );
        assert!(
            validate_performance_bond(&ConcessionType::Management, 400_000, 10_000_000).is_err()
        );

        // Plantation concession requires 3% bond
        assert!(
            validate_performance_bond(&ConcessionType::Plantation, 300_000, 10_000_000).is_ok()
        );
        assert!(
            validate_performance_bond(&ConcessionType::Plantation, 200_000, 10_000_000).is_err()
        );
    }

    #[test]
    fn test_benefit_sharing_valid() {
        let sharing = BenefitSharingArrangement {
            village_share_percent: 50.0,
            district_share_percent: 30.0,
            national_share_percent: 20.0,
            agreement_date: "2026-01-01".to_string(),
            validity_years: 5,
        };
        assert!(validate_benefit_sharing(&sharing).is_ok());
    }

    #[test]
    fn test_benefit_sharing_invalid() {
        let sharing = BenefitSharingArrangement {
            village_share_percent: 60.0,
            district_share_percent: 30.0,
            national_share_percent: 20.0, // Total = 110%
            agreement_date: "2026-01-01".to_string(),
            validity_years: 5,
        };
        assert!(validate_benefit_sharing(&sharing).is_err());
    }

    #[test]
    fn test_cites_compliance_listed_species() {
        // Rosewood is CITES listed
        assert!(validate_cites_compliance(&TreeSpecies::Rosewood, None).is_err());
        assert!(validate_cites_compliance(&TreeSpecies::Rosewood, Some("CITES-001")).is_ok());
    }

    #[test]
    fn test_cites_compliance_unlisted_species() {
        // Pine is not CITES listed
        assert!(validate_cites_compliance(&TreeSpecies::Pine, None).is_ok());
    }

    #[test]
    fn test_penalty_calculation_illegal_logging() {
        let (min, max) = calculate_penalty(&ViolationType::IllegalLogging, 1_000_000, false);
        assert_eq!(min, 2_000_000); // 2x
        assert_eq!(max, 10_000_000); // 10x
    }

    #[test]
    fn test_penalty_calculation_repeat_offender() {
        let (min, max) = calculate_penalty(&ViolationType::IllegalLogging, 1_000_000, true);
        assert_eq!(min, 3_000_000); // 2x * 1.5
        assert_eq!(max, 20_000_000); // 10x * 2.0
    }

    #[test]
    fn test_timber_permit_builder() {
        let permit = TimberHarvestingPermitBuilder::new()
            .permit_number("THP-2026-001")
            .holder_name("Test Company")
            .holder_name_lao("ບໍລິສັດທົດສອບ")
            .forest_type(ForestClassification::Production)
            .province("Savannakhet")
            .district("Sepon")
            .species(TreeSpecies::Teak)
            .volume_cubic_meters(500.0)
            .harvesting_month(12)
            .issue_date("2026-01-01")
            .expiry_date("2026-04-30")
            .status(PermitStatus::Active)
            .build();

        assert_eq!(permit.permit_number, "THP-2026-001");
        assert_eq!(permit.species, TreeSpecies::Teak);
        assert_eq!(permit.minimum_diameter_cm, MIN_DIAMETER_TEAK_CM);
    }

    #[test]
    fn test_timber_permit_validation_valid() {
        let permit = TimberHarvestingPermitBuilder::new()
            .permit_number("THP-2026-001")
            .holder_name("Test Company")
            .forest_type(ForestClassification::Production)
            .province("Savannakhet")
            .district("Sepon")
            .species(TreeSpecies::Teak)
            .volume_cubic_meters(500.0)
            .harvesting_month(12)
            .minimum_diameter_cm(45)
            .issue_date("2026-01-01")
            .expiry_date("2026-04-30")
            .status(PermitStatus::Active)
            .quota_reference("AAC-2026-SVK-001") // Required for Category II species (Teak)
            .build();

        assert!(validate_timber_harvesting_permit(&permit).is_ok());
    }

    #[test]
    fn test_timber_permit_validation_prohibited_species() {
        let permit = TimberHarvestingPermitBuilder::new()
            .permit_number("THP-2026-001")
            .holder_name("Test Company")
            .forest_type(ForestClassification::Production)
            .province("Savannakhet")
            .district("Sepon")
            .species(TreeSpecies::Rosewood) // Category I - prohibited
            .volume_cubic_meters(500.0)
            .harvesting_month(12)
            .status(PermitStatus::Active)
            .build();

        let result = validate_timber_harvesting_permit(&permit);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ForestryLawError::ProhibitedSpeciesHarvesting { .. }
        ));
    }

    #[test]
    fn test_forest_concession_validation() {
        let concession = ForestConcessionBuilder::new()
            .concession_number("FC-2026-001")
            .holder_name("Plantation Company")
            .concession_type(ConcessionType::Plantation)
            .area_hectares(5000.0)
            .term_years(40)
            .province("Attapeu")
            .performance_bond_lak(500_000_000)
            .project_value_lak(10_000_000_000)
            .has_eia(true)
            .has_management_plan(true)
            .build();

        assert!(validate_forest_concession(&concession).is_ok());
    }

    #[test]
    fn test_forest_concession_missing_eia() {
        let concession = ForestConcessionBuilder::new()
            .concession_number("FC-2026-001")
            .holder_name("Plantation Company")
            .concession_type(ConcessionType::Plantation)
            .area_hectares(5000.0)
            .term_years(40)
            .province("Attapeu")
            .has_eia(false) // Missing EIA
            .has_management_plan(true)
            .build();

        let result = validate_forest_concession(&concession);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ForestryLawError::MissingEIA));
    }

    #[test]
    fn test_chain_of_custody_valid() {
        let entries = vec![
            ChainOfCustodyEntry {
                date: "2026-01-15".to_string(),
                from_location: "Forest A".to_string(),
                to_location: "Sawmill B".to_string(),
                transport_permit: Some("TP-001".to_string()),
                handler_name: "Handler 1".to_string(),
                remarks: None,
            },
            ChainOfCustodyEntry {
                date: "2026-01-16".to_string(),
                from_location: "Sawmill B".to_string(),
                to_location: "Factory C".to_string(),
                transport_permit: Some("TP-002".to_string()),
                handler_name: "Handler 2".to_string(),
                remarks: None,
            },
        ];

        assert!(validate_chain_of_custody(&entries).is_ok());
    }

    #[test]
    fn test_chain_of_custody_broken() {
        let entries = vec![
            ChainOfCustodyEntry {
                date: "2026-01-15".to_string(),
                from_location: "Forest A".to_string(),
                to_location: "Sawmill B".to_string(),
                transport_permit: Some("TP-001".to_string()),
                handler_name: "Handler 1".to_string(),
                remarks: None,
            },
            ChainOfCustodyEntry {
                date: "2026-01-16".to_string(),
                from_location: "Different Location".to_string(), // Gap!
                to_location: "Factory C".to_string(),
                transport_permit: Some("TP-002".to_string()),
                handler_name: "Handler 2".to_string(),
                remarks: None,
            },
        ];

        let result = validate_chain_of_custody(&entries);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ForestryLawError::BrokenChainOfCustody { .. }
        ));
    }
}
