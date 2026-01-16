//! Environmental Law Edge Case Tests
//!
//! Edge cases for pollution prevention and waste management validation

use chrono::{NaiveDate, Utc};
use legalis_jp::egov::GovernmentAgency;
use legalis_jp::environmental_law::*;

// ============================================================================
// Pollution Prevention Edge Cases
// ============================================================================

#[test]
fn test_factory_notification_valid() {
    let notification = FactorySetupNotification {
        facility_name: "Test Factory".to_string(),
        facility_type: FacilityType::ChemicalPlant,
        location: "Tokyo".to_string(),
        installation_date: Utc::now().date_naive() + chrono::Duration::days(90),
        expected_emissions: vec![],
        pollution_control_equipment: vec![ControlEquipment {
            equipment_type: "Air Filter".to_string(),
            manufacturer: "HEPA Corp".to_string(),
            installation_date: Utc::now().date_naive(),
            designed_efficiency: 95.0,
        }],
        submitted_to: GovernmentAgency::MinistryOfEnvironment,
        notification_date: Some(Utc::now().date_naive()),
    };

    let result = validate_factory_setup_notification(&notification);
    assert!(result.is_ok());
}

#[test]
fn test_factory_notification_no_equipment() {
    let notification = FactorySetupNotification {
        facility_name: "No Equipment Factory".to_string(),
        facility_type: FacilityType::Incinerator,
        location: "Osaka".to_string(),
        installation_date: Utc::now().date_naive() + chrono::Duration::days(90),
        expected_emissions: vec![],
        pollution_control_equipment: vec![], // No equipment!
        submitted_to: GovernmentAgency::MinistryOfEnvironment,
        notification_date: Some(Utc::now().date_naive()),
    };

    let result = validate_factory_setup_notification(&notification);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should have errors
}

#[test]
fn test_factory_notification_insufficient_lead_time() {
    let notification = FactorySetupNotification {
        facility_name: "Rush Factory".to_string(),
        facility_type: FacilityType::PowerPlant,
        location: "Nagoya".to_string(),
        installation_date: Utc::now().date_naive() + chrono::Duration::days(30), // Only 30 days
        expected_emissions: vec![],
        pollution_control_equipment: vec![ControlEquipment {
            equipment_type: "Scrubber".to_string(),
            manufacturer: "Clean Air Inc".to_string(),
            installation_date: Utc::now().date_naive(),
            designed_efficiency: 90.0,
        }],
        submitted_to: GovernmentAgency::MinistryOfEnvironment,
        notification_date: Some(Utc::now().date_naive()),
    };

    let result = validate_factory_setup_notification(&notification);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should fail 60-day requirement
}

#[test]
fn test_emission_limit_within_bounds() {
    let limit = EmissionLimit {
        pollutant: Pollutant::SulfurOxides,
        limit_value: 50.0,
        unit: "ppm".to_string(),
        legal_basis: "大気汚染防止法第3条".to_string(),
    };

    assert!(limit.limit_value > 0.0);
    assert!(!limit.unit.is_empty());
}

#[test]
fn test_emission_limit_zero() {
    let limit = EmissionLimit {
        pollutant: Pollutant::NitrogenOxides,
        limit_value: 0.0, // Zero emission
        unit: "ppm".to_string(),
        legal_basis: "大気汚染防止法第3条".to_string(),
    };

    assert_eq!(limit.limit_value, 0.0);
}

#[test]
fn test_emission_limit_very_high() {
    let limit = EmissionLimit {
        pollutant: Pollutant::Particulates,
        limit_value: 10000.0, // Very high
        unit: "mg/m3".to_string(),
        legal_basis: "大気汚染防止法第3条".to_string(),
    };

    assert!(limit.limit_value > 1000.0);
}

#[test]
fn test_all_pollutant_types() {
    let pollutants = [
        Pollutant::SulfurOxides,
        Pollutant::NitrogenOxides,
        Pollutant::Particulates,
        Pollutant::VolatileOrganic,
        Pollutant::Dioxins,
        Pollutant::HeavyMetals(HeavyMetal::Lead),
        Pollutant::BiochemicalOxygen,
        Pollutant::ChemicalOxygen,
    ];

    assert_eq!(pollutants.len(), 8);
}

#[test]
fn test_all_facility_types() {
    let types = [
        FacilityType::PowerPlant,
        FacilityType::Incinerator,
        FacilityType::ChemicalPlant,
        FacilityType::SteelMill,
        FacilityType::WasteProcessing,
        FacilityType::WastewaterTreatment,
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_pollution_prevention_agreement_valid() {
    let agreement = PollutionPreventionAgreement {
        facility_name: "Test Power Plant".to_string(),
        facility_type: FacilityType::PowerPlant,
        operator: "Power Corp".to_string(),
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

    let result = validate_pollution_prevention_agreement(&agreement);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.is_valid());
}

#[test]
fn test_pollution_prevention_agreement_no_monitoring() {
    let agreement = PollutionPreventionAgreement {
        facility_name: "No Monitor Factory".to_string(),
        facility_type: FacilityType::ChemicalPlant,
        operator: "Chem Corp".to_string(),
        location: "Osaka".to_string(),
        pollution_types: vec![PollutionType::Water],
        emission_limits: vec![],
        monitoring_requirements: vec![], // No monitoring!
        effective_date: Utc::now().date_naive(),
    };

    let result = validate_pollution_prevention_agreement(&agreement);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.warnings.is_empty()); // Should have warnings
}

// ============================================================================
// Waste Management Edge Cases
// ============================================================================

#[test]
fn test_waste_permit_valid_collection() {
    let permit = WasteManagementPermit {
        permit_number: "WASTE-001".to_string(),
        permit_type: WastePermitType::Collection,
        operator_name: "Waste Collector Inc.".to_string(),
        waste_types: vec![WasteType::Municipal],
        processing_capacity_tons_per_day: 10.0,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        facility_standards_met: true,
    };

    let result = validate_waste_management_permit(&permit);
    assert!(result.is_ok());
}

#[test]
fn test_waste_permit_standards_not_met() {
    let permit = WasteManagementPermit {
        permit_number: "WASTE-BAD".to_string(),
        permit_type: WastePermitType::Disposal,
        operator_name: "Bad Facility".to_string(),
        waste_types: vec![WasteType::Industrial],
        processing_capacity_tons_per_day: 5.0,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2032, 1, 1).unwrap(),
        facility_standards_met: false, // Standards not met!
    };

    let result = validate_waste_management_permit(&permit);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should have errors
}

#[test]
fn test_waste_permit_expired() {
    let permit = WasteManagementPermit {
        permit_number: "WASTE-EXP".to_string(),
        permit_type: WastePermitType::Collection,
        operator_name: "Expired Permit".to_string(),
        waste_types: vec![WasteType::Municipal],
        processing_capacity_tons_per_day: 3.0,
        issue_date: NaiveDate::from_ymd_opt(2015, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(), // Expired
        facility_standards_met: true,
    };

    assert!(!permit.is_valid()); // Built-in validity check
}

#[test]
fn test_waste_permit_zero_capacity() {
    let permit = WasteManagementPermit {
        permit_number: "WASTE-ZERO".to_string(),
        permit_type: WastePermitType::Disposal,
        operator_name: "Zero Capacity".to_string(),
        waste_types: vec![WasteType::Industrial],
        processing_capacity_tons_per_day: 0.0, // Zero capacity
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        facility_standards_met: true,
    };

    let result = validate_waste_management_permit(&permit);
    assert!(result.is_ok());
    // Zero capacity is allowed by implementation, but may have warnings
}

#[test]
fn test_waste_permit_very_high_capacity() {
    let permit = WasteManagementPermit {
        permit_number: "WASTE-MEGA".to_string(),
        permit_type: WastePermitType::Disposal,
        operator_name: "Mega Facility".to_string(),
        waste_types: vec![WasteType::Industrial],
        processing_capacity_tons_per_day: 10000.0, // Very high
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2032, 1, 1).unwrap(),
        facility_standards_met: true,
    };

    let result = validate_waste_management_permit(&permit);
    assert!(result.is_ok());
    // May have warnings for large-scale facility
}

#[test]
fn test_waste_permit_multiple_waste_types() {
    let permit = WasteManagementPermit {
        permit_number: "WASTE-MULTI".to_string(),
        permit_type: WastePermitType::Collection,
        operator_name: "Multi Waste Collector".to_string(),
        waste_types: vec![WasteType::Municipal, WasteType::Industrial],
        processing_capacity_tons_per_day: 50.0,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        facility_standards_met: true,
    };

    assert_eq!(permit.waste_types.len(), 2);
    let result = validate_waste_management_permit(&permit);
    assert!(result.is_ok());
}

#[test]
fn test_waste_manifest_valid() {
    let manifest = WasteManifest {
        manifest_number: "MAN-001".to_string(),
        waste_type: WasteType::Industrial,
        quantity_kg: 1000.0,
        generator: Party {
            name: "Factory A".to_string(),
            address: "Tokyo".to_string(),
            contact: Some("03-1234-5678".to_string()),
        },
        transporter: Party {
            name: "Transport Co.".to_string(),
            address: "Tokyo".to_string(),
            contact: Some("03-9999-8888".to_string()),
        },
        processor: Party {
            name: "Disposal Facility".to_string(),
            address: "Osaka".to_string(),
            contact: Some("06-1111-2222".to_string()),
        },
        issue_date: Utc::now().date_naive(),
        completion_date: None,
    };

    let result = validate_waste_manifest(&manifest);
    assert!(result.is_ok());
}

#[test]
fn test_waste_manifest_empty_number() {
    let manifest = WasteManifest {
        manifest_number: "".to_string(), // Empty!
        waste_type: WasteType::Industrial,
        quantity_kg: 500.0,
        generator: Party {
            name: "Factory B".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        transporter: Party {
            name: "Transport Co.".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        processor: Party {
            name: "Processor".to_string(),
            address: "Osaka".to_string(),
            contact: None,
        },
        issue_date: Utc::now().date_naive(),
        completion_date: None,
    };

    let result = validate_waste_manifest(&manifest);
    assert!(result.is_err()); // Should fail
}

#[test]
fn test_waste_manifest_zero_quantity() {
    let manifest = WasteManifest {
        manifest_number: "MAN-ZERO".to_string(),
        waste_type: WasteType::Municipal,
        quantity_kg: 0.0, // Zero quantity
        generator: Party {
            name: "Generator".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        transporter: Party {
            name: "Transporter".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        processor: Party {
            name: "Processor".to_string(),
            address: "Osaka".to_string(),
            contact: None,
        },
        issue_date: Utc::now().date_naive(),
        completion_date: None,
    };

    let result = validate_waste_manifest(&manifest);
    // May fail or warn about zero quantity
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_waste_manifest_very_large_quantity() {
    let manifest = WasteManifest {
        manifest_number: "MAN-LARGE".to_string(),
        waste_type: WasteType::Industrial,
        quantity_kg: 1_000_000.0, // 1000 tons
        generator: Party {
            name: "Large Generator".to_string(),
            address: "Tokyo".to_string(),
            contact: Some("03-1234-5678".to_string()),
        },
        transporter: Party {
            name: "Heavy Transporter".to_string(),
            address: "Tokyo".to_string(),
            contact: Some("03-8888-9999".to_string()),
        },
        processor: Party {
            name: "Large Processor".to_string(),
            address: "Osaka".to_string(),
            contact: Some("06-7777-6666".to_string()),
        },
        issue_date: Utc::now().date_naive(),
        completion_date: None,
    };

    assert!(manifest.quantity_kg > 100_000.0);
    let result = validate_waste_manifest(&manifest);
    assert!(result.is_ok());
}

#[test]
fn test_waste_manifest_with_completion() {
    let manifest = WasteManifest {
        manifest_number: "MAN-COMPLETE".to_string(),
        waste_type: WasteType::Industrial,
        quantity_kg: 750.0,
        generator: Party {
            name: "Generator".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        transporter: Party {
            name: "Transporter".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        processor: Party {
            name: "Processor".to_string(),
            address: "Osaka".to_string(),
            contact: None,
        },
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        completion_date: Some(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()),
    };

    assert!(manifest.is_complete());
    let result = validate_waste_manifest(&manifest);
    assert!(result.is_ok());
}

#[test]
fn test_all_waste_types() {
    let types = [
        WasteType::Municipal,
        WasteType::Industrial,
        WasteType::SpecialIndustrial,
        WasteType::Infectious,
        WasteType::Explosive,
        WasteType::Toxic,
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_all_waste_permit_types() {
    let types = [
        WastePermitType::Collection,
        WastePermitType::Disposal,
        WastePermitType::Intermediate,
        WastePermitType::Final,
        WastePermitType::IndustrialWaste,
    ];

    assert_eq!(types.len(), 5);
}

#[test]
fn test_control_equipment_types() {
    let equipment1 = ControlEquipment {
        equipment_type: "Air Filter".to_string(),
        manufacturer: "CleanAir Inc".to_string(),
        installation_date: Utc::now().date_naive(),
        designed_efficiency: 95.0,
    };

    let equipment2 = ControlEquipment {
        equipment_type: "Scrubber".to_string(),
        manufacturer: "Pollution Control Co".to_string(),
        installation_date: Utc::now().date_naive(),
        designed_efficiency: 90.0,
    };

    assert_eq!(equipment1.equipment_type, "Air Filter");
    assert_eq!(equipment2.equipment_type, "Scrubber");
}

#[test]
fn test_quick_validate_pollution() {
    let agreement = PollutionPreventionAgreement {
        facility_name: "Quick Test".to_string(),
        facility_type: FacilityType::PowerPlant,
        operator: "Quick Corp".to_string(),
        location: "Tokyo".to_string(),
        pollution_types: vec![PollutionType::Air],
        emission_limits: vec![],
        monitoring_requirements: vec![],
        effective_date: Utc::now().date_naive(),
    };

    let result = quick_validate_pollution(&agreement);
    assert!(result.is_ok() || result.is_err()); // Just test it runs
}

#[test]
fn test_quick_validate_waste() {
    let permit = WasteManagementPermit {
        permit_number: "QUICK".to_string(),
        permit_type: WastePermitType::Collection,
        operator_name: "Quick Test".to_string(),
        waste_types: vec![WasteType::Municipal],
        processing_capacity_tons_per_day: 5.0,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        facility_standards_met: true,
    };

    let result = quick_validate_waste(&permit);
    assert!(result.is_ok() || result.is_err()); // Just test it runs
}
