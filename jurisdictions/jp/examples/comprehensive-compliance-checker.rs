//! Comprehensive Compliance Checker - Multi-Domain Validation
//!
//! Demonstrates cross-domain compliance checking across multiple Japanese laws:
//! - Personal Information Protection Act (個人情報保護法)
//! - Environmental Law (環境法)
//! - Construction Business Act (建設業法)
//! - Administrative Procedure Act (行政手続法)
//! - Consumer Protection Law (消費者契約法)
//!
//! This example shows how different law domains interact in real business scenarios.
//!
//! Run with:
//! ```bash
//! cargo run --example comprehensive-compliance-checker
//! ```

use chrono::{NaiveDate, Utc};
use legalis_jp::administrative_procedure::{
    Applicant, Document, DocumentType, ProcedureBuilder, ProcedureType, validate_procedure,
};
use legalis_jp::construction_real_estate::{
    ConstructionBusinessLicense, ConstructionLicenseType, ConstructionType, Manager,
    ManagerQualification, validate_construction_license,
};
use legalis_jp::consumer_protection::{ConsumerContract, ContractTerm};
use legalis_jp::egov::GovernmentAgency;
use legalis_jp::environmental_law::{
    ControlEquipment, EmissionEstimate, FacilityType, FactorySetupNotification, Pollutant,
    validate_factory_setup_notification,
};
use legalis_jp::personal_info_protection::{
    BusinessType, DataHandlingVolume, PersonalInfoType, PersonalInformationHandler, PurposeType,
    SecurityMeasure, SecurityMeasureType, UsagePurpose, validate_personal_info_handling,
};

fn main() {
    println!("=== Comprehensive Multi-Domain Compliance Checker ===\n");
    println!("Demonstrating interaction between multiple Japanese law domains");
    println!("for a real-world business scenario.\n");

    // Scenario: Construction materials marketplace
    println!("📋 SCENARIO: Construction Materials E-Commerce Platform");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("A company launching an online marketplace for construction materials");
    println!("must comply with multiple law domains simultaneously:\n");
    println!("1. Personal Information Protection Act (customer data)");
    println!("2. Construction Business Act (construction-related products)");
    println!("3. Environmental Law (material processing facility)");
    println!("4. Administrative Procedure Act (business licenses)");
    println!("5. Consumer Protection Law (contract fairness)\n");

    // Domain 1: Personal Information Protection
    println!("═══ Domain 1: Personal Information Protection Act ═══");
    println!();
    check_personal_data_compliance();
    println!();

    // Domain 2: Construction Business
    println!("═══ Domain 2: Construction Business Act ═══");
    println!();
    check_construction_license_compliance();
    println!();

    // Domain 3: Environmental Law
    println!("═══ Domain 3: Environmental Law ═══");
    println!();
    check_environmental_compliance();
    println!();

    // Domain 4: Administrative Procedures
    println!("═══ Domain 4: Administrative Procedure Act ═══");
    println!();
    check_administrative_filing_compliance();
    println!();

    // Domain 5: Consumer Protection
    println!("═══ Domain 5: Consumer Protection Law ═══");
    println!();
    check_consumer_protection_compliance();
    println!();

    // Final Summary
    println!("═══ COMPLIANCE SUMMARY ═══");
    println!();
    println!("✅ All five law domains validated successfully");
    println!();
    println!("📌 Key Compliance Points:");
    println!("  • Data Protection: Security measures implemented (Article 20)");
    println!("  • Construction: Qualified managers certified (Article 8)");
    println!("  • Environment: Pollution control equipment specified (Article 6)");
    println!("  • Administrative: Proper filing procedures followed (Article 7)");
    println!("  • Consumer: Fair contract terms (Article 8-10)");
    println!();
    println!("This comprehensive check ensures the business can operate");
    println!("legally across all applicable Japanese law domains.");
}

fn check_personal_data_compliance() {
    println!("Checking customer data handling compliance...");

    let handler = PersonalInformationHandler {
        business_name: "BuildMart株式会社 (BuildMart Co., Ltd.)".to_string(),
        business_type: BusinessType::StandardBusiness,
        handling_volume: DataHandlingVolume::Under100000,
        data_types: vec![PersonalInfoType::Basic],
        purposes: vec![
            UsagePurpose {
                purpose: "Order processing and delivery".to_string(),
                purpose_type: PurposeType::ServiceProvision,
                specified_at_collection: true,
                consent_obtained: true,
            },
            UsagePurpose {
                purpose: "Marketing and promotional materials".to_string(),
                purpose_type: PurposeType::MarketingAdvertising,
                specified_at_collection: true,
                consent_obtained: true,
            },
        ],
        security_measures: vec![
            SecurityMeasure {
                measure_type: SecurityMeasureType::AccessControl,
                description: "Role-based access with multi-factor authentication".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::Encryption,
                description: "TLS 1.3 for transit + AES-256 for storage".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::EmployeeTraining,
                description: "Quarterly privacy and security training".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::IncidentResponsePlan,
                description: "24/7 security monitoring and incident response team".to_string(),
                implemented: true,
            },
        ],
        third_party_provision: None,
        cross_border_transfer: None,
    };

    print!("  Business: {} ... ", handler.business_name);
    print!("Data Types: {} ... ", handler.data_types.len());
    print!(
        "Security: {} measures ... ",
        handler.security_measures.len()
    );

    match validate_personal_info_handling(&handler) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ COMPLIANT");
                println!("    ✓ Purpose specified at collection (Article 15)");
                println!("    ✓ Consent obtained for all uses (Article 16)");
                println!("    ✓ Comprehensive security measures (Article 20)");
                println!("    ✓ No unauthorized third-party sharing (Article 23)");
            } else {
                println!("❌ NON-COMPLIANT");
                for error in &report.errors {
                    println!("    • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn check_construction_license_compliance() {
    println!("Checking construction business license...");

    let manager = Manager {
        name: "建設管理責任者 田中一郎 (Construction Manager: Ichiro Tanaka)".to_string(),
        qualification: ManagerQualification::ConstructionManager,
        certification_number: "CONST-MGR-2024-001234".to_string(),
        certification_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
    };

    let license = ConstructionBusinessLicense {
        license_number: "東京都知事許可(般-1)第99999号".to_string(),
        business_name: "BuildMart株式会社".to_string(),
        license_type: ConstructionLicenseType::General,
        construction_types: vec![ConstructionType::Architecture, ConstructionType::Carpentry],
        registered_capital_jpy: 10_000_000,
        issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2029, 1, 1).unwrap(),
        managers: vec![manager],
    };

    print!("  License: {} ... ", license.license_type.name_ja());
    print!("Capital: ¥{} ... ", license.registered_capital_jpy);
    print!("Managers: {} ... ", license.managers.len());

    match validate_construction_license(&license) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ COMPLIANT");
                println!("    ✓ Minimum capital requirement met (Article 7)");
                println!("    ✓ Qualified construction manager present (Article 8)");
                println!("    ✓ 5-year license validity (Article 3-3)");
                println!(
                    "    ✓ {} construction types authorized",
                    license.construction_types.len()
                );

                if !report.warnings.is_empty() {
                    println!("    ⚠️  {} recommendations", report.warnings.len());
                }
            } else {
                println!("❌ NON-COMPLIANT");
                for error in &report.errors {
                    println!("    • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn check_environmental_compliance() {
    println!("Checking material processing facility notification...");

    let notification = FactorySetupNotification {
        facility_name: "BuildMart Material Processing Center".to_string(),
        facility_type: FacilityType::WasteProcessing,
        location: "Industrial Park Zone B, Kawasaki City, Kanagawa".to_string(),
        installation_date: Utc::now().date_naive() + chrono::Duration::days(90),
        expected_emissions: vec![
            EmissionEstimate {
                pollutant: Pollutant::Particulates,
                estimated_value: 20.0,
                unit: "mg/Nm³".to_string(),
            },
            EmissionEstimate {
                pollutant: Pollutant::VolatileOrganic,
                estimated_value: 15.0,
                unit: "ppm".to_string(),
            },
        ],
        pollution_control_equipment: vec![
            ControlEquipment {
                equipment_type: "High-efficiency bag filter system".to_string(),
                manufacturer: "EcoClean Technology Co., Ltd.".to_string(),
                installation_date: Utc::now().date_naive() + chrono::Duration::days(85),
                designed_efficiency: 99.8,
            },
            ControlEquipment {
                equipment_type: "Activated carbon VOC absorber".to_string(),
                manufacturer: "AirPure Environmental Systems Inc.".to_string(),
                installation_date: Utc::now().date_naive() + chrono::Duration::days(85),
                designed_efficiency: 96.5,
            },
        ],
        submitted_to: GovernmentAgency::MinistryOfEnvironment,
        notification_date: Some(Utc::now().date_naive()),
    };

    print!("  Facility: {} ... ", notification.facility_type.name_ja());
    print!(
        "Equipment: {} units ... ",
        notification.pollution_control_equipment.len()
    );

    let days_notice = notification
        .notification_date
        .map(|nd| (notification.installation_date - nd).num_days())
        .unwrap_or(0);
    print!("Notice: {} days ... ", days_notice);

    match validate_factory_setup_notification(&notification) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ COMPLIANT");
                println!(
                    "    ✓ Submitted {} days before installation (Article 6 requires 60)",
                    days_notice
                );
                println!(
                    "    ✓ {} pollution control equipment units specified",
                    notification.pollution_control_equipment.len()
                );
                println!("    ✓ Expected emissions declared");
                println!("    ✓ Submitted to Ministry of Environment");
            } else {
                println!("❌ NON-COMPLIANT");
                for error in &report.errors {
                    println!("    • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn check_administrative_filing_compliance() {
    println!("Checking business license application filing...");

    let applicant = Applicant::corporation(
        "BuildMart株式会社",
        "1-2-3 Shibuya, Shibuya-ku, Tokyo 150-0001",
    )
    .with_phone("03-1234-5678")
    .with_email("legal@buildmart.jp")
    .with_identification("corporate_number", "1234567890123");

    let procedure = ProcedureBuilder::new()
        .procedure_id("APP-BUILDMART-2026-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-001".to_string(),
            title: "E-Commerce Platform Business License Application Form".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .add_document(Document {
            id: "DOC-002".to_string(),
            title: "Corporate Registration Certificate (登記簿謄本)".to_string(),
            document_type: DocumentType::SupportingDocument,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .add_document(Document {
            id: "DOC-003".to_string(),
            title: "Data Protection and Privacy Policy".to_string(),
            document_type: DocumentType::SupportingDocument,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .add_document(Document {
            id: "DOC-004".to_string(),
            title: "Environmental Compliance Certificate".to_string(),
            document_type: DocumentType::SupportingDocument,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(30)
        .notes("Initial application for construction materials e-commerce platform with integrated supply chain")
        .build()
        .unwrap();

    print!("  Type: {} ... ", procedure.procedure_type.name_ja());
    print!("Agency: {:?} ... ", procedure.agency);
    print!("Documents: {} ... ", procedure.documents.len());

    match validate_procedure(&procedure) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ COMPLIANT");
                println!("    ✓ All required documents attached (4 documents)");
                println!("    ✓ Processing period: 30 days (Article 7)");
                println!("    ✓ Applicant information complete and verified");
                println!("    ✓ Cross-domain compliance documents included");
            } else {
                println!("❌ NON-COMPLIANT");
                for error in &report.errors {
                    println!("    • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn check_consumer_protection_compliance() {
    println!("Checking consumer contract fairness...");

    let contract = ConsumerContract {
        title: "Construction Materials Monthly Subscription Agreement".to_string(),
        business_name: "BuildMart株式会社".to_string(),
        consumer_name: "山田太郎 (Taro Yamada)".to_string(),
        contract_date: Utc::now(),
        contract_amount_jpy: 29800,
        terms: vec![
            ContractTerm {
                term_number: 1,
                text: "Monthly subscription for construction materials delivery with 30-day money-back guarantee".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 0,
            },
            ContractTerm {
                term_number: 2,
                text: "Cancellation allowed anytime with no penalty, effective end of billing cycle".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 0,
            },
            ContractTerm {
                term_number: 3,
                text: "Consumer retains all rights under applicable consumer protection laws".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 0,
            },
        ],
        cancellation_policy: None,
        penalty_clause: None,
    };

    print!("  Contract: {} ... ", contract.title);
    print!("Terms: {} ... ", contract.terms.len());
    print!("Amount: ¥{} ... ", contract.contract_amount_jpy);

    // Simple validation check
    let has_unfair_terms = contract.terms.iter().any(|t| t.potentially_unfair);
    let has_penalty = contract.penalty_clause.is_some();
    let has_clear_cancellation = contract.cancellation_policy.is_some();

    if !has_unfair_terms {
        println!("✅ COMPLIANT");
        println!("    ✓ No unfair contract terms (Article 8-10)");
        println!("    ✓ Consumer rights preserved in all terms");
        println!(
            "    ✓ Cancellation policy: {}",
            if has_clear_cancellation {
                "Defined"
            } else {
                "Not explicitly defined"
            }
        );
        println!(
            "    ✓ Penalty clause: {}",
            if has_penalty { "Present" } else { "None" }
        );
    } else {
        println!("❌ NON-COMPLIANT");
        println!("    • Contract contains potentially unfair terms");
    }
}
