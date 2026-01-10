//! # GDPR Compliance Integration Example
//!
//! This example demonstrates how multiple GDPR articles work together
//! in a realistic business scenario: an e-commerce company.
//!
//! ## Business Scenario
//!
//! **Company**: TechShop Europe GmbH (Germany)
//! - E-commerce platform selling electronics
//! - Processes customer and employee data
//! - Uses AWS for cloud hosting (processor)
//! - Shares data with payment processors
//!
//! ## Articles Integrated
//!
//! 1. **Article 6**: Lawful basis for processing customer orders
//! 2. **Article 32**: Security measures for protecting data
//! 3. **Article 28**: Processor contract with AWS
//! 4. **Article 24**: Overall accountability framework
//!
//! This demonstrates end-to-end compliance for a common business operation.

use chrono::Utc;
use legalis_eu::gdpr::security::RiskLevel;
use legalis_eu::gdpr::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     GDPR Compliance Integration: TechShop Europe GmbH      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("Scenario: Customer places an order on e-commerce platform\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // ═══════════════════════════════════════════════════════════════════
    // Article 6: Lawful Basis for Processing
    // ═══════════════════════════════════════════════════════════════════
    println!("📋 Step 1: Establish Lawful Basis (Article 6)\n");

    let processing = DataProcessing::new()
        .with_controller("TechShop Europe GmbH")
        .with_purpose("Process customer orders and fulfill contracts")
        .add_data_category(PersonalDataCategory::Regular(
            "Name, email, shipping address, payment details".to_string(),
        ))
        .with_operations(vec![
            ProcessingOperation::Collection,
            ProcessingOperation::Storage,
            ProcessingOperation::Use,
            ProcessingOperation::Disclosure,
        ])
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    match processing.validate() {
        Ok(_validation) => {
            println!("✅ Article 6 Compliance:");
            println!("   Lawful Basis: Contract performance (Art. 6(1)(b))");
            println!("   Purpose: Order fulfillment");
            println!("   Operations: Collection, Storage, Use, Disclosure");
            println!("   Data: Customer details for order processing\n");
        }
        Err(e) => {
            println!("❌ Article 6 Error: {}\n", e);
            return;
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Article 32: Security Measures
    // ═══════════════════════════════════════════════════════════════════
    println!("🔒 Step 2: Implement Security Measures (Article 32)\n");

    let security = SecurityAssessment::new()
        .with_entity("TechShop Europe GmbH")
        .with_risk_level(RiskLevel::High) // Payment data = high risk
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256, TLS 1.3".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "Customer IDs in analytics".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Daily automated backups".to_string(),
            recovery_time_objective: "4 hours".to_string(),
            recovery_point_objective: "1 hour".to_string(),
            tested: true,
        })
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: true,
            least_privilege: true,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Quarterly GDPR training".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true);

    match security.validate() {
        Ok(validation) => {
            println!("✅ Article 32 Compliance:");
            println!("   Risk Level: High (payment data)");
            println!(
                "   Technical Measures: {}",
                validation.technical_measures_count
            );
            println!("     • Encryption: AES-256 at rest, TLS 1.3 in transit");
            println!("     • Pseudonymisation: Customer IDs anonymized");
            println!("     • Backup: Daily (RTO: 4h, RPO: 1h)");
            println!(
                "   Organizational Measures: {}",
                validation.organizational_measures_count
            );
            println!("     • Access Control: RBAC + least privilege");
            println!("     • Training: Quarterly for all staff");

            if validation.compliant {
                println!("   Status: ✅ Fully compliant\n");
            } else {
                println!("   Warnings: {}\n", validation.warnings.len());
            }
        }
        Err(e) => {
            println!("❌ Article 32 Error: {}\n", e);
            return;
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Article 28: Processor Contract with AWS
    // ═══════════════════════════════════════════════════════════════════
    println!("📝 Step 3: Establish Processor Contract (Article 28)\n");

    let processor_contract = ProcessorContract::new()
        .with_controller("TechShop Europe GmbH", "dpo@techshop.eu")
        .with_processor("Amazon Web Services EMEA", "aws-privacy@amazon.com")
        .with_subject_matter("Cloud hosting of e-commerce platform")
        .with_processing_purpose("Website hosting and database storage")
        .add_data_category("Customer orders, names, addresses")
        .add_data_subject_category("TechShop customers")
        .with_all_mandatory_clauses()
        .with_notes("ISO 27001, SOC 2 Type II certified. International transfers to US with SCCs.");

    match processor_contract.validate() {
        Ok(validation) => {
            println!("✅ Article 28 Compliance:");
            println!("   Processor: Amazon Web Services EMEA");
            println!("   Article 28(3) Clauses: All mandatory clauses present");
            println!("   Security: ISO 27001, SOC 2 Type II");
            println!("   International Transfers: US (SCCs)");

            if validation.compliant {
                println!("   Status: ✅ Fully compliant\n");
            } else {
                println!("   Warnings: {}\n", validation.warnings.len());
            }
        }
        Err(e) => {
            println!("❌ Article 28 Error: {}\n", e);
            return;
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Article 24: Accountability Framework
    // ═══════════════════════════════════════════════════════════════════
    println!("📊 Step 4: Demonstrate Accountability (Article 24)\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("TechShop Europe GmbH")
        .with_data_volume(DataVolume::Medium)
        .with_data_sensitivity(DataSensitivity::High)
        .with_risk_level_assessed(RiskLevel::High)
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: Some("Article 32 measures demonstrated above".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: true,
            notes: Some("AWS contract demonstrated above".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::StaffTraining {
            training_program_established: true,
            frequency: Some("Quarterly".to_string()),
            notes: None,
        })
        .add_organizational_measure(AccountabilityMeasure::DataSubjectRightsProcedures {
            procedures_documented: true,
            response_process_established: true,
            notes: Some("30-day response SLA established".to_string()),
        })
        .add_certification(ComplianceCertification::InformationSecurity {
            standard: "ISO/IEC 27001:2022".to_string(),
            certified: true,
            valid_until: Some(Utc::now() + chrono::Duration::days(365)),
        })
        .with_compliance_documentation(true)
        .with_nature_considered(true)
        .with_scope_considered(true)
        .with_context_considered(true)
        .with_purposes_considered(true);

    match accountability.validate() {
        Ok(validation) => {
            println!("✅ Article 24 Compliance:");
            println!("   Compliance Score: {}/100", validation.compliance_score);
            println!(
                "   Technical Measures: {}",
                validation.technical_measures_count
            );
            println!(
                "   Organizational Measures: {}",
                validation.organizational_measures_count
            );
            println!(
                "   Certifications: {} (ISO 27001)",
                validation.certifications_count
            );
            println!("   Documentation: Maintained and up-to-date");
            println!("   Article 24(1) Factors: All considered");
            println!("     • Nature of processing: E-commerce orders");
            println!("     • Scope: Medium volume, high sensitivity");
            println!("     • Context: Online retail platform");
            println!("     • Purpose: Contract fulfillment");
            println!("     • Risks: Payment data, customer information");

            if !validation.warnings.is_empty() {
                println!("\n   Recommendations:");
                for warning in &validation.warnings {
                    println!("     • {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n   Further Improvements:");
                for rec in &validation.recommendations {
                    println!("     • {}", rec);
                }
            }
            println!();
        }
        Err(e) => {
            println!("❌ Article 24 Error: {}\n", e);
            return;
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Final Summary
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         GDPR COMPLIANCE ACHIEVED ✅                        ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("TechShop Europe GmbH - Integrated Compliance Summary\n");

    println!("Integration Flow:");
    println!("  1️⃣  Article 6 → Lawful basis established (Contract)");
    println!("  2️⃣  Article 32 → Security measures implemented");
    println!("  3️⃣  Article 28 → Processor contracts in place (AWS)");
    println!("  4️⃣  Article 24 → Accountability framework documented\n");

    println!("Key Integration Points:");
    println!("  • Article 6 defines WHY we process (contract fulfillment)");
    println!("  • Article 32 defines HOW we protect (encryption, access control)");
    println!("  • Article 28 defines WHO helps us (AWS processor)");
    println!("  • Article 24 proves WE'RE accountable (documentation, measures)\n");

    println!("Business Impact:");
    println!("  ✅ Can legally process customer orders (Art. 6)");
    println!("  ✅ Data protected with appropriate security (Art. 32)");
    println!("  ✅ Processor relationships compliant (Art. 28)");
    println!("  ✅ Can demonstrate compliance to authorities (Art. 24)\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("This example demonstrates how GDPR articles work together");
    println!("to create a comprehensive compliance framework.");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
