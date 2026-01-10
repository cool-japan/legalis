//! Environmental Law Compliance Checker
//!
//! Demonstrates compliance checking for environmental regulations including:
//! - Air Pollution Control Act (大気汚染防止法)
//! - Water Pollution Prevention Act (水質汚濁防止法)
//! - Waste Management Act (廃棄物処理法)
//!
//! Run with:
//! ```bash
//! cargo run --example environmental-compliance-checker
//! ```

use chrono::{NaiveDate, Utc};
use legalis_jp::egov::GovernmentAgency;
use legalis_jp::environmental_law::{
    ControlEquipment, EmissionEstimate, EmissionLimit, FacilityType, FactorySetupNotification,
    MonitoringRequirement, Party, Pollutant, PollutionPreventionAgreement, PollutionType,
    WasteManagementPermit, WasteManifest, WastePermitType, WasteType,
    validate_factory_setup_notification, validate_pollution_prevention_agreement,
    validate_waste_management_permit, validate_waste_manifest,
};

fn main() {
    println!("=== Environmental Law Compliance Checker ===\n");

    // Example 1: Air pollution prevention agreement
    println!("🏭 Example 1: Air Pollution Control (大気汚染防止法)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_air_pollution_control();
    println!();

    // Example 2: Factory setup notification
    println!("🏗️  Example 2: Factory Setup Notification (Article 6)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_factory_setup_notification();
    println!();

    // Example 3: Waste management permit
    println!("♻️  Example 3: Waste Management Permit (廃棄物処理法)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_waste_management_permit();
    println!();

    // Example 4: Waste manifest system
    println!("📋 Example 4: Waste Manifest System (マニフェスト制度)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_waste_manifest();
}

fn example_air_pollution_control() {
    let agreement = PollutionPreventionAgreement {
        facility_name: "Tokyo Manufacturing Plant".to_string(),
        facility_type: FacilityType::ChemicalPlant,
        operator: "Tokyo Chemicals Inc.".to_string(),
        location: "1-2-3 Industrial Zone, Kawasaki City".to_string(),
        pollution_types: vec![PollutionType::Air],
        emission_limits: vec![
            EmissionLimit {
                pollutant: Pollutant::SulfurOxides,
                limit_value: 80.0, // Within legal limit
                unit: "ppm".to_string(),
                legal_basis: "大気汚染防止法第3条".to_string(),
            },
            EmissionLimit {
                pollutant: Pollutant::NitrogenOxides,
                limit_value: 120.0, // Within legal limit
                unit: "ppm".to_string(),
                legal_basis: "大気汚染防止法第3条".to_string(),
            },
            EmissionLimit {
                pollutant: Pollutant::Particulates,
                limit_value: 30.0, // Within legal limit
                unit: "mg/Nm³".to_string(),
                legal_basis: "大気汚染防止法第3条".to_string(),
            },
        ],
        monitoring_requirements: vec![
            MonitoringRequirement {
                parameter: "Sulfur Oxides (SOx)".to_string(),
                frequency: "Continuous monitoring".to_string(),
                reporting_required: true,
            },
            MonitoringRequirement {
                parameter: "Nitrogen Oxides (NOx)".to_string(),
                frequency: "Continuous monitoring".to_string(),
                reporting_required: true,
            },
        ],
        effective_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    };

    println!("Facility: {}", agreement.facility_name);
    println!("Type: {:?}", agreement.facility_type);
    println!("Operator: {}", agreement.operator);
    println!("Location: {}", agreement.location);
    println!("\nEmission Limits (Article 3):");
    for limit in &agreement.emission_limits {
        println!(
            "  • {:?}: {} {} ({})",
            limit.pollutant, limit.limit_value, limit.unit, limit.legal_basis
        );
    }
    println!(
        "\nMonitoring: {} requirements (Article 16)",
        agreement.monitoring_requirements.len()
    );

    match validate_pollution_prevention_agreement(&agreement) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Pollution prevention agreement is COMPLIANT");
                println!("  ✓ All emission limits within legal standards");
                println!("  ✓ Monitoring requirements specified");

                if !report.warnings.is_empty() {
                    println!("\n⚠️  Recommendations:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("\n❌ Compliance violations:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Validation error: {}", e),
    }
}

fn example_factory_setup_notification() {
    let notification = FactorySetupNotification {
        facility_name: "New Chemical Processing Plant".to_string(),
        facility_type: FacilityType::ChemicalPlant,
        location: "5-6-7 Eco Industrial Park, Yokohama".to_string(),
        installation_date: Utc::now().date_naive() + chrono::Duration::days(75), // 75 days from now
        expected_emissions: vec![
            EmissionEstimate {
                pollutant: Pollutant::SulfurOxides,
                estimated_value: 50.0,
                unit: "ppm".to_string(),
            },
            EmissionEstimate {
                pollutant: Pollutant::NitrogenOxides,
                estimated_value: 100.0,
                unit: "ppm".to_string(),
            },
        ],
        pollution_control_equipment: vec![
            ControlEquipment {
                equipment_type: "Electrostatic precipitator".to_string(),
                manufacturer: "Green Tech Industries".to_string(),
                installation_date: Utc::now().date_naive() + chrono::Duration::days(70),
                designed_efficiency: 99.5,
            },
            ControlEquipment {
                equipment_type: "Wet scrubber for SOx".to_string(),
                manufacturer: "Clean Air Solutions".to_string(),
                installation_date: Utc::now().date_naive() + chrono::Duration::days(70),
                designed_efficiency: 98.0,
            },
        ],
        submitted_to: GovernmentAgency::MinistryOfEnvironment,
        notification_date: Some(Utc::now().date_naive()),
    };

    println!("Facility: {}", notification.facility_name);
    println!("Location: {}", notification.location);
    println!("Installation Date: {}", notification.installation_date);
    if let Some(ndate) = notification.notification_date {
        println!("Notification Date: {}", ndate);
    }

    let days_notice = notification
        .notification_date
        .map(|ndate| (notification.installation_date - ndate).num_days())
        .unwrap_or(0);
    println!("Notice Period: {} days", days_notice);

    println!("\nPollution Control Equipment:");
    for (i, equipment) in notification.pollution_control_equipment.iter().enumerate() {
        println!(
            "  {}. {} (Efficiency: {}%)",
            i + 1,
            equipment.equipment_type,
            equipment.designed_efficiency
        );
    }

    match validate_factory_setup_notification(&notification) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Factory setup notification is VALID");
                println!("  ✓ Submitted at least 60 days before installation (Article 6)");
                println!("  ✓ Pollution control equipment specified");

                if !report.warnings.is_empty() {
                    println!("\n⚠️  Notes:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("\n❌ Notification issues:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_waste_management_permit() {
    // Example 1: Compliant permit
    let permit = WasteManagementPermit {
        permit_type: WastePermitType::Collection,
        permit_number: "IWC-2024-001".to_string(),
        operator_name: "Eco Waste Management Co., Ltd.".to_string(),
        waste_types: vec![WasteType::Industrial, WasteType::Municipal],
        processing_capacity_tons_per_day: 50.0,
        issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2029, 1, 1).unwrap(), // 5 years for collection
        facility_standards_met: true,
    };

    println!("Permit Type: {:?}", permit.permit_type);
    println!("Operator: {}", permit.operator_name);
    println!("Permit Number: {}", permit.permit_number);
    println!("Waste Types: {} types authorized", permit.waste_types.len());
    for waste_type in &permit.waste_types {
        println!("  • {:?}", waste_type);
    }
    println!(
        "Processing Capacity: {} tons/day",
        permit.processing_capacity_tons_per_day
    );

    let validity_years = (permit.expiration_date - permit.issue_date).num_days() / 365;
    println!("Permit Validity: {} years", validity_years);

    match validate_waste_management_permit(&permit) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Waste management permit is VALID");
                println!("  ✓ Facility standards met (Article 8)");
                println!("  ✓ Technical manager qualified");
                println!("  ✓ 5-year validity period (Article 7)");

                if !report.warnings.is_empty() {
                    println!("\n⚠️  Notes:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("\n❌ Permit issues:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Example 2: Non-compliant permit (missing facility standards)
    println!("\n--- Non-Compliant Example ---");
    let non_compliant = WasteManagementPermit {
        permit_type: WastePermitType::Disposal,
        permit_number: "IWD-2024-BAD".to_string(),
        operator_name: "Questionable Disposal Inc.".to_string(),
        waste_types: vec![WasteType::Toxic],
        processing_capacity_tons_per_day: 10.0,
        issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2031, 1, 1).unwrap(), // 7 years for disposal
        facility_standards_met: false,                                 // ❌ Not compliant
    };

    println!("Operator: {}", non_compliant.operator_name);

    match validate_waste_management_permit(&non_compliant) {
        Ok(report) => {
            if !report.is_valid() {
                println!("❌ COMPLIANCE VIOLATIONS:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_waste_manifest() {
    println!("Industrial Waste Manifest System (Article 12-3)\n");

    // Create manifest
    let manifest = WasteManifest {
        manifest_number: "MF-2026-0109-001".to_string(),
        waste_type: WasteType::Industrial,
        quantity_kg: 5000.0,
        generator: Party {
            name: "ABC Manufacturing Co., Ltd.".to_string(),
            address: "1-1-1 Factory Town, Aichi".to_string(),
            contact: Some("gen@abc-mfg.jp".to_string()),
        },
        transporter: Party {
            name: "Safe Transport Logistics".to_string(),
            address: "2-2-2 Transport Hub, Shizuoka".to_string(),
            contact: Some("transport@safelog.jp".to_string()),
        },
        processor: Party {
            name: "Eco Disposal Center".to_string(),
            address: "3-3-3 Eco Park, Nagano".to_string(),
            contact: Some("disposal@eco-center.jp".to_string()),
        },
        issue_date: Utc::now().date_naive(),
        completion_date: None,
    };

    println!("Manifest Number: {}", manifest.manifest_number);
    println!("Waste Type: {:?}", manifest.waste_type);
    println!("Quantity: {} kg", manifest.quantity_kg);
    println!("\nParties:");
    println!("  Generator: {}", manifest.generator.name);
    println!("    Address: {}", manifest.generator.address);
    println!("  Transporter: {}", manifest.transporter.name);
    println!("    Address: {}", manifest.transporter.address);
    println!("  Processor: {}", manifest.processor.name);
    println!("    Address: {}", manifest.processor.address);

    match validate_waste_manifest(&manifest) {
        Ok(_) => {
            println!("\n✅ Waste manifest is VALID");
            println!("  ✓ All parties identified (Article 12-3)");
            println!("  ✓ Manifest number assigned");
            println!("  ✓ Waste type and quantity specified");
            println!("\n📌 Next steps:");
            println!("  1. Waste collection by transporter");
            println!("  2. Transport to disposal facility");
            println!("  3. Disposal completion confirmation");
            println!("  4. Generator confirms completion (within 90 days)");
        }
        Err(e) => println!("❌ Validation error: {}", e),
    }

    println!("\n📌 Legal Requirements:");
    println!("  • Manifest must be retained for 5 years (Article 12-3)");
    println!("  • Generator must confirm disposal within 90 days");
    println!("  • Electronic manifest system (JWNET) available");
    println!("  • Article 7: Collection/transport permit (5-year validity)");
    println!("  • Article 14: Disposal permit (7-year validity)");
}
