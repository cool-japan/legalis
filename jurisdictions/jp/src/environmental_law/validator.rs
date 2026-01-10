//! Environmental Law Validation
//!
//! Validation logic for pollution prevention agreements, factory notifications,
//! waste management permits, and manifests.

use crate::egov::ValidationReport;

use super::error::{EnvironmentalError, Result};
use super::types::*;

/// Validate pollution prevention agreement
pub fn validate_pollution_prevention_agreement(
    agreement: &PollutionPreventionAgreement,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check facility name
    if agreement.facility_name.is_empty() {
        report.add_error("Missing facility name".to_string());
    }

    // Check operator
    if agreement.operator.is_empty() {
        report.add_error("Missing operator name".to_string());
    }

    // Check location
    if agreement.location.is_empty() {
        report.add_error("Missing location".to_string());
    }

    // Check emission limits against legal standards
    for limit in &agreement.emission_limits {
        if let Some(legal_limit) =
            get_legal_emission_limit(&limit.pollutant, &agreement.facility_type)
        {
            if limit.limit_value > legal_limit {
                report.add_error(format!(
                    "{:?} limit {} {} exceeds legal limit {} {} ({})",
                    limit.pollutant,
                    limit.limit_value,
                    limit.unit,
                    legal_limit,
                    limit.unit,
                    limit.legal_basis
                ));
            }
        }
    }

    // Check monitoring requirements
    if agreement.monitoring_requirements.is_empty() {
        report
            .add_warning("No monitoring requirements specified (大気汚染防止法第16条)".to_string());
    }

    // Check pollution control measures
    if agreement.emission_limits.is_empty() {
        report.add_warning("No emission limits specified".to_string());
    }

    Ok(report)
}

/// Validate factory setup notification
pub fn validate_factory_setup_notification(
    notification: &FactorySetupNotification,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check facility name
    if notification.facility_name.is_empty() {
        report.add_error("Missing facility name".to_string());
    }

    // Check location
    if notification.location.is_empty() {
        report.add_error("Missing location".to_string());
    }

    // Article 6: Prior notification required (60 days before installation)
    if !notification.meets_prior_notification() {
        report.add_error(
            "Prior notification must be filed 60 days before installation (大気汚染防止法第6条)"
                .to_string(),
        );
    }

    // Check pollution control equipment
    if notification.pollution_control_equipment.is_empty() {
        report.add_error("Pollution control equipment required (大気汚染防止法第3条)".to_string());
    }

    // Validate expected emissions
    if notification.expected_emissions.is_empty() {
        report.add_warning("No emission estimates provided".to_string());
    }

    // Check equipment efficiency
    for equipment in &notification.pollution_control_equipment {
        if equipment.designed_efficiency < 90.0 {
            report.add_warning(format!(
                "Equipment '{}' has low efficiency: {}%",
                equipment.equipment_type, equipment.designed_efficiency
            ));
        }
    }

    Ok(report)
}

/// Validate waste management permit
pub fn validate_waste_management_permit(
    permit: &WasteManagementPermit,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check permit number
    if permit.permit_number.is_empty() {
        report.add_error("Missing permit number".to_string());
    }

    // Check operator name
    if permit.operator_name.is_empty() {
        report.add_error("Missing operator name".to_string());
    }

    // Article 8, 14: Facility standards must be met
    if !permit.facility_standards_met {
        report.add_error(
            "Facility must meet technical standards (廃棄物処理法第8条・第14条)".to_string(),
        );
    }

    // Check permit validity period
    if !permit.has_correct_validity_period() {
        report.add_warning(format!(
            "Permit validity period should be {} years",
            permit.permit_type.validity_years()
        ));
    }

    // Check processing capacity limits
    if permit.processing_capacity_tons_per_day > 1000.0 {
        report.add_warning(
            "Large-scale facility (>1000 tons/day) requires additional permits (廃棄物処理法施行令)"
                .to_string(),
        );
    }

    // Check for expired permit
    if !permit.is_valid() {
        report.add_error(format!("Permit expired: {}", permit.expiration_date));
    }

    // Check waste types
    if permit.waste_types.is_empty() {
        report.add_error("No waste types specified".to_string());
    }

    // Check special handling requirements
    for waste_type in &permit.waste_types {
        if waste_type.requires_special_handling() {
            report.add_warning(format!(
                "Special handling required for {:?} ({})",
                waste_type,
                waste_type.name_ja()
            ));
        }
    }

    Ok(report)
}

/// Validate waste manifest
pub fn validate_waste_manifest(manifest: &WasteManifest) -> Result<()> {
    // Article 12-3: Manifest required for industrial waste
    if manifest.manifest_number.is_empty() {
        return Err(EnvironmentalError::InvalidManifest {
            reason: "Manifest number required (廃棄物処理法第12条の3)".to_string(),
        });
    }

    // Check all parties specified
    if manifest.generator.name.is_empty() {
        return Err(EnvironmentalError::InvalidManifest {
            reason: "Generator name required".to_string(),
        });
    }

    if manifest.transporter.name.is_empty() {
        return Err(EnvironmentalError::InvalidManifest {
            reason: "Transporter name required".to_string(),
        });
    }

    if manifest.processor.name.is_empty() {
        return Err(EnvironmentalError::InvalidManifest {
            reason: "Processor name required".to_string(),
        });
    }

    // Check quantity
    if manifest.quantity_kg <= 0.0 {
        return Err(EnvironmentalError::InvalidManifest {
            reason: "Quantity must be greater than zero".to_string(),
        });
    }

    Ok(())
}

/// Get legal emission limit for pollutant and facility type
fn get_legal_emission_limit(pollutant: &Pollutant, facility_type: &FacilityType) -> Option<f64> {
    // Simplified emission limits based on Japanese regulations
    match (pollutant, facility_type) {
        (Pollutant::SulfurOxides, FacilityType::PowerPlant) => Some(100.0),
        (Pollutant::NitrogenOxides, FacilityType::PowerPlant) => Some(150.0),
        (Pollutant::Particulates, FacilityType::Incinerator) => Some(50.0),
        (Pollutant::Dioxins, FacilityType::Incinerator) => Some(0.1),
        (Pollutant::SulfurOxides, FacilityType::ChemicalPlant) => Some(80.0),
        (Pollutant::NitrogenOxides, FacilityType::ChemicalPlant) => Some(120.0),
        (Pollutant::Particulates, FacilityType::SteelMill) => Some(100.0),
        (Pollutant::BiochemicalOxygen, FacilityType::WastewaterTreatment) => Some(15.0),
        (Pollutant::ChemicalOxygen, FacilityType::WastewaterTreatment) => Some(20.0),
        _ => None,
    }
}

/// Quick validation helper for pollution prevention agreement
pub fn quick_validate_pollution(agreement: &PollutionPreventionAgreement) -> Result<()> {
    let report = validate_pollution_prevention_agreement(agreement)?;
    if !report.is_valid() {
        Err(EnvironmentalError::Validation(format!(
            "{} validation errors",
            report.errors.len()
        )))
    } else {
        Ok(())
    }
}

/// Quick validation helper for waste management permit
pub fn quick_validate_waste(permit: &WasteManagementPermit) -> Result<()> {
    let report = validate_waste_management_permit(permit)?;
    if !report.is_valid() {
        Err(EnvironmentalError::Validation(format!(
            "{} validation errors",
            report.errors.len()
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_valid_agreement() {
        let agreement = PollutionPreventionAgreement {
            facility_name: "Test Factory".to_string(),
            facility_type: FacilityType::PowerPlant,
            operator: "Test Operator".to_string(),
            location: "Tokyo".to_string(),
            pollution_types: vec![PollutionType::Air],
            emission_limits: vec![EmissionLimit {
                pollutant: Pollutant::SulfurOxides,
                limit_value: 80.0,
                unit: "ppm".to_string(),
                legal_basis: "大気汚染防止法第3条".to_string(),
            }],
            monitoring_requirements: vec![MonitoringRequirement {
                parameter: "SOx".to_string(),
                frequency: "daily".to_string(),
                reporting_required: true,
            }],
            effective_date: Utc::now().date_naive(),
        };

        let report = validate_pollution_prevention_agreement(&agreement).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_excessive_emission() {
        let agreement = PollutionPreventionAgreement {
            facility_name: "Test Factory".to_string(),
            facility_type: FacilityType::PowerPlant,
            operator: "Test Operator".to_string(),
            location: "Tokyo".to_string(),
            pollution_types: vec![PollutionType::Air],
            emission_limits: vec![EmissionLimit {
                pollutant: Pollutant::SulfurOxides,
                limit_value: 150.0, // Exceeds legal limit of 100.0
                unit: "ppm".to_string(),
                legal_basis: "大気汚染防止法第3条".to_string(),
            }],
            monitoring_requirements: vec![],
            effective_date: Utc::now().date_naive(),
        };

        let report = validate_pollution_prevention_agreement(&agreement).unwrap();
        assert!(!report.is_valid());
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn test_validate_missing_fields() {
        let agreement = PollutionPreventionAgreement {
            facility_name: String::new(),
            facility_type: FacilityType::Incinerator,
            operator: String::new(),
            location: String::new(),
            pollution_types: vec![],
            emission_limits: vec![],
            monitoring_requirements: vec![],
            effective_date: Utc::now().date_naive(),
        };

        let report = validate_pollution_prevention_agreement(&agreement).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.len() >= 3);
    }

    #[test]
    fn test_validate_factory_notification() {
        let notification = FactorySetupNotification {
            facility_name: "New Factory".to_string(),
            facility_type: FacilityType::ChemicalPlant,
            location: "Osaka".to_string(),
            installation_date: Utc::now().date_naive() + chrono::Duration::days(90),
            expected_emissions: vec![],
            pollution_control_equipment: vec![ControlEquipment {
                equipment_type: "Scrubber".to_string(),
                manufacturer: "ABC Corp".to_string(),
                installation_date: Utc::now().date_naive(),
                designed_efficiency: 95.0,
            }],
            submitted_to: crate::egov::GovernmentAgency::MinistryOfEnvironment,
            notification_date: Some(Utc::now().date_naive()),
        };

        let report = validate_factory_setup_notification(&notification).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_late_notification() {
        let notification = FactorySetupNotification {
            facility_name: "New Factory".to_string(),
            facility_type: FacilityType::Incinerator,
            location: "Kyoto".to_string(),
            installation_date: Utc::now().date_naive() + chrono::Duration::days(30),
            expected_emissions: vec![],
            pollution_control_equipment: vec![],
            submitted_to: crate::egov::GovernmentAgency::MinistryOfEnvironment,
            notification_date: Some(Utc::now().date_naive()),
        };

        let report = validate_factory_setup_notification(&notification).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_valid_permit() {
        let permit = WasteManagementPermit {
            permit_type: WastePermitType::Collection,
            permit_number: "WASTE-001".to_string(),
            operator_name: "Test Waste Co.".to_string(),
            waste_types: vec![WasteType::Industrial],
            processing_capacity_tons_per_day: 100.0,
            issue_date: Utc::now().date_naive(),
            expiration_date: Utc::now().date_naive() + chrono::Duration::days(365 * 5),
            facility_standards_met: true,
        };

        let report = validate_waste_management_permit(&permit).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_expired_permit() {
        let permit = WasteManagementPermit {
            permit_type: WastePermitType::Disposal,
            permit_number: "WASTE-002".to_string(),
            operator_name: "Test Waste Co.".to_string(),
            waste_types: vec![WasteType::Municipal],
            processing_capacity_tons_per_day: 50.0,
            issue_date: Utc::now().date_naive() - chrono::Duration::days(365 * 8),
            expiration_date: Utc::now().date_naive() - chrono::Duration::days(30),
            facility_standards_met: true,
        };

        let report = validate_waste_management_permit(&permit).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_facility_standards_not_met() {
        let permit = WasteManagementPermit {
            permit_type: WastePermitType::Collection,
            permit_number: "WASTE-003".to_string(),
            operator_name: "Test Waste Co.".to_string(),
            waste_types: vec![WasteType::Industrial],
            processing_capacity_tons_per_day: 75.0,
            issue_date: Utc::now().date_naive(),
            expiration_date: Utc::now().date_naive() + chrono::Duration::days(365 * 5),
            facility_standards_met: false,
        };

        let report = validate_waste_management_permit(&permit).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_valid_manifest() {
        let manifest = WasteManifest {
            manifest_number: "MF-001".to_string(),
            waste_type: WasteType::Industrial,
            quantity_kg: 1000.0,
            generator: Party {
                name: "Generator Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            transporter: Party {
                name: "Transporter Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            processor: Party {
                name: "Processor Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            issue_date: Utc::now().date_naive(),
            completion_date: None,
        };

        assert!(validate_waste_manifest(&manifest).is_ok());
    }

    #[test]
    fn test_validate_missing_manifest_fields() {
        let manifest = WasteManifest {
            manifest_number: String::new(),
            waste_type: WasteType::Industrial,
            quantity_kg: 1000.0,
            generator: Party {
                name: String::new(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            transporter: Party {
                name: "Transporter Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            processor: Party {
                name: "Processor Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            issue_date: Utc::now().date_naive(),
            completion_date: None,
        };

        assert!(validate_waste_manifest(&manifest).is_err());
    }

    #[test]
    fn test_quick_validate() {
        let agreement = PollutionPreventionAgreement {
            facility_name: "Test".to_string(),
            facility_type: FacilityType::PowerPlant,
            operator: "Operator".to_string(),
            location: "Tokyo".to_string(),
            pollution_types: vec![],
            emission_limits: vec![],
            monitoring_requirements: vec![],
            effective_date: Utc::now().date_naive(),
        };

        assert!(quick_validate_pollution(&agreement).is_ok());
    }
}
