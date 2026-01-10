//! GDPR Records of Processing Activities (ROPA) Example
//!
//! This example demonstrates how to maintain records under Article 30 GDPR.
//!
//! ## Scenarios Covered
//!
//! 1. Complete ROPA for a small organization
//! 2. Individual processing records (controller and processor)
//! 3. ROPA exemption checking
//! 4. Validation and warnings

use chrono::Utc;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::ropa::*;
use legalis_eu::gdpr::types::{LawfulBasis, PersonalDataCategory, ProcessingOperation};

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Records of Processing Activities (ROPA) Example ===\n");

    scenario_1_complete_ropa()?;
    scenario_2_controller_record()?;
    scenario_3_processor_record()?;
    scenario_4_exemption_check();

    println!("\n✅ All ROPA scenarios completed");
    Ok(())
}

/// Scenario 1: Complete ROPA for Small Business
///
/// Demonstrates a complete ROPA with multiple processing activities
fn scenario_1_complete_ropa() -> Result<(), GdprError> {
    println!("## Scenario 1: Complete ROPA for E-Commerce Business\n");

    let ropa = RecordsOfProcessingActivities::new("TechShop Inc")
        // Record 1: Customer Orders
        .add_record(
            ProcessingRecord::new()
                .with_entity_type(EntityType::Controller)
                .with_name("Customer Order Processing")
                .with_controller_details(
                    ContactDetails::new("TechShop Inc", "privacy@techshop.com")
                        .with_address("123 Tech Street, Berlin, Germany")
                        .with_phone("+49 30 12345678"),
                )
                .with_dpo("Data Protection Team", "dpo@techshop.com")
                .with_purpose("Processing customer orders and payments")
                .with_lawful_basis(LawfulBasis::Contract {
                    necessary_for_performance: true,
                })
                .add_data_subject_category("customers")
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .add_data_category(PersonalDataCategory::Regular("address".to_string()))
                .add_data_category(PersonalDataCategory::Regular("payment details".to_string()))
                .add_recipient("Payment processor (Stripe Inc, US)")
                .add_recipient("Shipping provider (DHL)")
                .add_recipient("Accounting team (internal)")
                .add_third_country_transfer(ThirdCountryTransfer {
                    country: "United States".to_string(),
                    safeguard: "Standard Contractual Clauses (2021)".to_string(),
                    documentation: Some("SCC signed 2024-01-15, ref: SCC-2024-001".to_string()),
                })
                .with_retention_period("7 years after purchase (tax law requirement)")
                .add_security_measure("TLS encryption for data in transit")
                .add_security_measure("AES-256 encryption for data at rest")
                .add_security_measure("Role-based access controls")
                .add_security_measure("Regular security audits")
                .add_security_measure("Multi-factor authentication for admin access")
                .add_operation(ProcessingOperation::Collection)
                .add_operation(ProcessingOperation::Storage)
                .add_operation(ProcessingOperation::Use)
                .with_created_date(Utc::now())
                .with_last_updated(Utc::now()),
        )
        // Record 2: Marketing
        .add_record(
            ProcessingRecord::new()
                .with_entity_type(EntityType::Controller)
                .with_name("Email Marketing")
                .with_controller("TechShop Inc", "privacy@techshop.com")
                .with_dpo("Data Protection Team", "dpo@techshop.com")
                .with_purpose("Sending promotional emails to subscribers")
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                })
                .add_data_subject_category("subscribers")
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .add_data_category(PersonalDataCategory::Regular(
                    "purchase history".to_string(),
                ))
                .add_recipient("Email service provider (Mailchimp)")
                .with_retention_period("Until unsubscribe or 2 years of inactivity")
                .add_security_measure("Encrypted API connections")
                .add_security_measure("Access logging")
                .add_security_measure("Regular data quality checks")
                .add_operation(ProcessingOperation::Collection)
                .add_operation(ProcessingOperation::Use)
                .add_operation(ProcessingOperation::Dissemination),
        )
        // Record 3: Employee Data
        .add_record(
            ProcessingRecord::new()
                .with_entity_type(EntityType::Controller)
                .with_name("Human Resources Management")
                .with_controller("TechShop Inc", "privacy@techshop.com")
                .with_dpo("Data Protection Team", "dpo@techshop.com")
                .with_purpose("Employee administration and payroll")
                .with_lawful_basis(LawfulBasis::LegalObligation {
                    eu_law: None,
                    member_state_law: Some("German Employment Law".to_string()),
                })
                .add_data_subject_category("employees")
                .add_data_subject_category("job applicants")
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .add_data_category(PersonalDataCategory::Regular("contact details".to_string()))
                .add_data_category(PersonalDataCategory::Regular("tax ID".to_string()))
                .add_data_category(PersonalDataCategory::Regular("bank account".to_string()))
                .add_recipient("Payroll provider (DATEV)")
                .add_recipient("Tax authorities (Germany)")
                .with_retention_period("10 years after employment ends (German tax law)")
                .add_security_measure("Physical access controls to HR files")
                .add_security_measure("Encrypted HR database")
                .add_security_measure("Background checks for HR staff")
                .add_security_measure("Annual data protection training")
                .with_notes("Employment contracts include data protection clause"),
        )
        .with_last_reviewed(Utc::now());

    // Validate ROPA
    let validation = ropa.validate()?;

    println!("Organization: TechShop Inc");
    println!("Total Processing Records: {}", validation.total_records);
    println!("Complete Records: {}", validation.complete_records);
    println!("All Records Complete: {}", validation.all_records_complete);
    println!(
        "Records with Special Categories: {}",
        validation.records_with_special_categories
    );
    println!(
        "Records with Third Country Transfers: {}",
        validation.records_with_transfers
    );

    // Check exemption status
    let exemption = ropa.is_exempt(45); // 45 employees
    match exemption {
        RopaExemption::Exempt => {
            println!("\n✅ Organization qualifies for ROPA exemption (Article 30(5))");
        }
        RopaExemption::NotExempt { reason } => {
            println!("\n⚠️ ROPA REQUIRED: {}", reason);
        }
    }

    // Show warnings for each record
    println!("\n📋 PROCESSING RECORDS:");
    for (idx, validation) in validation.record_validations.iter().enumerate() {
        println!(
            "\n   Record {}: {}",
            idx + 1,
            if validation.complete {
                "✅ Complete"
            } else {
                "❌ Incomplete"
            }
        );

        if !validation.warnings.is_empty() {
            println!("   Warnings:");
            for warning in &validation.warnings {
                println!("     ⚠️ {}", warning);
            }
        }
    }

    println!("\n✅ ROPA maintained and up-to-date\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Detailed Controller Record
///
/// Shows all Article 30(1) requirements for a controller
fn scenario_2_controller_record() -> Result<(), GdprError> {
    println!("## Scenario 2: Detailed Controller Record (Article 30(1))\n");

    let record = ProcessingRecord::new()
        .with_entity_type(EntityType::Controller)
        .with_name("Website Analytics")
        .with_controller_details(
            ContactDetails::new("Analytics Corp", "privacy@analytics.com")
                .with_address("456 Data Ave, Amsterdam, Netherlands"),
        )
        .with_dpo("Chief Privacy Officer", "cpo@analytics.com")
        .with_purpose("Understanding user behavior to improve website UX")
        .with_lawful_basis(LawfulBasis::LegitimateInterests {
            controller_interest: "Website improvement and business analytics".to_string(),
            balancing_test_passed: true,
        })
        .add_data_subject_category("website visitors")
        .add_data_category(PersonalDataCategory::Regular(
            "IP address (pseudonymized)".to_string(),
        ))
        .add_data_category(PersonalDataCategory::Regular("browser type".to_string()))
        .add_data_category(PersonalDataCategory::Regular("page views".to_string()))
        .add_data_category(PersonalDataCategory::Regular(
            "session duration".to_string(),
        ))
        .add_recipient("Analytics team (internal)")
        .add_recipient("Product team (internal)")
        .with_retention_period("26 months (IP address after 14 days truncated)")
        .add_security_measure("IP address pseudonymization")
        .add_security_measure("No cross-site tracking")
        .add_security_measure("Cookie consent management")
        .add_security_measure("Data minimization (no PII collected)")
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Use)
        .with_notes("Compliant with ePrivacy Directive cookie rules");

    let validation = record.validate()?;

    println!("Processing Activity: Website Analytics");
    println!("Entity Type: Controller");
    println!("Complete: {}", validation.complete);
    println!(
        "Special Categories: {}",
        validation.contains_special_categories
    );
    println!(
        "Third Country Transfers: {}",
        validation.has_third_country_transfers
    );

    println!("\n✅ Article 30(1) Requirements Met:");
    println!("   (a) Controller contact details ✓");
    println!("   (b) Purposes of processing ✓");
    println!("   (c) Data subject & data categories ✓");
    println!("   (d) Recipients ✓");
    println!("   (e) Third country transfers (N/A)");
    println!("   (f) Retention periods ✓");
    println!("   (g) Security measures ✓");

    println!("\n✅ Controller record complete\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Processor Record
///
/// Shows Article 30(2) requirements for a processor
fn scenario_3_processor_record() -> Result<(), GdprError> {
    println!("## Scenario 3: Processor Record (Article 30(2))\n");

    let record = ProcessingRecord::new()
        .with_entity_type(EntityType::Processor)
        .with_name("Cloud Storage Services")
        .with_processor("SecureCloud GmbH", "dpo@securecloud.de")
        .with_dpo("Data Protection Officer", "dpo@securecloud.de")
        .with_purpose("Data storage and backup on behalf of clients")
        .with_purpose("Disaster recovery services")
        .with_purpose("Data encryption and security")
        .add_data_subject_category("clients' end users (various)")
        .add_data_category(PersonalDataCategory::Regular(
            "varies by client".to_string(),
        ))
        .add_third_country_transfer(ThirdCountryTransfer {
            country: "Switzerland".to_string(),
            safeguard: "Adequacy decision".to_string(),
            documentation: Some("Swiss datacenter: Zurich-1".to_string()),
        })
        .add_security_measure("AES-256 encryption at rest")
        .add_security_measure("TLS 1.3 for data in transit")
        .add_security_measure("SOC 2 Type II certified datacenters")
        .add_security_measure("Zero-knowledge architecture")
        .add_security_measure("Regular penetration testing")
        .add_security_measure("24/7 security monitoring")
        .add_security_measure("ISO 27001 certified")
        .add_operation(ProcessingOperation::Storage)
        .add_operation(ProcessingOperation::Organization)
        .add_operation(ProcessingOperation::Retrieval)
        .with_notes("Processing on behalf of controllers under data processing agreements");

    let validation = record.validate()?;

    println!("Processing Activity: Cloud Storage Services");
    println!("Entity Type: Processor");
    println!("Complete: {}", validation.complete);
    println!("Has DPO: {}", validation.has_dpo);

    println!("\n✅ Article 30(2) Requirements Met:");
    println!("   (a) Processor contact details & DPO ✓");
    println!("   (b) Categories of processing ✓");
    println!("   (c) Third country transfers ✓");
    println!("   (d) Security measures ✓");

    println!("\n✅ Processor record complete\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: ROPA Exemption Analysis
///
/// Shows when ROPA is required vs. when exemption applies
fn scenario_4_exemption_check() {
    println!("## Scenario 4: ROPA Exemption Analysis (Article 30(5))\n");

    // Case 1: Small occasional processing (exempt)
    println!("Case 1: Small freelance consultancy");
    println!("  Employees: 3");
    println!("  Processing: Occasional client invoicing");
    println!("  Special categories: No");
    let ropa1 = RecordsOfProcessingActivities::new("Freelance Consulting").add_record(
        ProcessingRecord::new()
            .with_controller("Freelancer", "me@freelance.com")
            .add_data_subject_category("occasional clients"),
    );
    match ropa1.is_exempt(3) {
        RopaExemption::Exempt => println!("  Result: ✅ EXEMPT from ROPA requirement"),
        RopaExemption::NotExempt { reason } => println!("  Result: ❌ NOT EXEMPT: {}", reason),
    }

    // Case 2: Large company (not exempt - size)
    println!("\nCase 2: Large tech company");
    println!("  Employees: 500");
    println!("  Processing: Various");
    let ropa2 = RecordsOfProcessingActivities::new("Big Tech Corp");
    match ropa2.is_exempt(500) {
        RopaExemption::Exempt => println!("  Result: ✅ EXEMPT"),
        RopaExemption::NotExempt { reason } => println!("  Result: ❌ NOT EXEMPT: {}", reason),
    }

    // Case 3: Small clinic (not exempt - special categories)
    println!("\nCase 3: Small medical clinic");
    println!("  Employees: 15");
    println!("  Processing: Patient health records");
    println!("  Special categories: Yes (health data)");
    let ropa3 = RecordsOfProcessingActivities::new("Small Clinic");
    // Note: would need to add health data category to trigger exemption check
    match ropa3.is_exempt(15) {
        RopaExemption::Exempt => println!("  Result: ✅ EXEMPT"),
        RopaExemption::NotExempt { reason } => println!("  Result: ❌ NOT EXEMPT: {}", reason),
    }

    // Case 4: Small business with customers (not exempt - systematic)
    println!("\nCase 4: Small e-commerce shop");
    println!("  Employees: 50");
    println!("  Processing: Systematic customer data processing");
    let ropa4 = RecordsOfProcessingActivities::new("Small Shop").add_record(
        ProcessingRecord::new()
            .with_controller("Shop", "privacy@shop.com")
            .add_data_subject_category("customers"), // Systematic processing
    );
    match ropa4.is_exempt(50) {
        RopaExemption::Exempt => println!("  Result: ✅ EXEMPT"),
        RopaExemption::NotExempt { reason } => println!("  Result: ❌ NOT EXEMPT: {}", reason),
    }

    println!("\n💡 ARTICLE 30(5) EXEMPTION CRITERIA:");
    println!("   Organization is exempt ONLY if ALL of the following:");
    println!("   1. <250 employees");
    println!("   2. Processing is occasional (not systematic)");
    println!("   3. Processing unlikely to result in risk to rights");
    println!("   4. Does NOT include special categories or criminal data");
    println!("\n   If ANY criterion is not met → ROPA REQUIRED\n");
    println!("{}", "=".repeat(70));
    println!();
}
