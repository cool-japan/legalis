//! GDPR Article 24 - Responsibility of the Controller
//!
//! This example demonstrates how to assess controller accountability under Article 24,
//! which requires controllers to implement appropriate technical and organizational
//! measures and demonstrate compliance with the GDPR.
//!
//! Article 24 is the **foundational accountability principle** that ties together
//! all other GDPR obligations.
//!
//! ## Scenarios
//!
//! 1. **Complete Accountability Framework** - Fully compliant controller
//! 2. **Healthcare Provider** - High-risk processing with DPIA
//! 3. **Small Business** - Proportionate measures for low-risk processing
//! 4. **Non-Compliant Missing Considerations** - Article 24(1) considerations not documented
//! 5. **Missing Essential Measures** - No Article 25/32/30 compliance
//! 6. **International Tech Company** - Complex accountability with certifications
//!
//! Run with:
//! ```bash
//! cargo run --example gdpr_article24_accountability
//! ```

use chrono::Utc;
use legalis_eu::gdpr::*;

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("GDPR Article 24 - Responsibility of the Controller");
    println!("═══════════════════════════════════════════════════════════════\n");

    scenario1_complete_accountability();
    scenario2_healthcare_provider();
    scenario3_small_business();
    scenario4_missing_considerations();
    scenario5_missing_essential_measures();
    scenario6_international_tech_company();
}

/// Scenario 1: Complete Accountability Framework
///
/// A fully compliant e-commerce controller with all accountability measures in place.
fn scenario1_complete_accountability() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 1: Complete Accountability Framework");
    println!("──────────────────────────────────────────────────────────────\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("EuroShop GmbH")
        .with_processing_description("E-commerce platform processing customer orders, payments, and deliveries")
        .with_data_volume(DataVolume::Large)
        .with_data_sensitivity(DataSensitivity::Medium)
        .with_risk_level_assessed(SecurityRiskLevel::Medium)
        // Article 24(1) considerations
        .with_nature_considered(true)
        .with_scope_considered(true)
        .with_context_considered(true)
        .with_purposes_considered(true)
        // Technical measures
        .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
            implemented: true,
            documented: true,
            notes: Some("Privacy by design: data minimisation, privacy-preserving defaults, optional fields opt-in".to_string()),
        })
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: Some("TLS 1.3, AES-256 encryption, RBAC, quarterly penetration testing".to_string()),
        })
        .add_technical_measure(AccountabilityMeasure::DataProtectionImpactAssessment {
            dpia_required: false,
            dpia_conducted: false,
            notes: Some("DPIA screening completed - not required for standard e-commerce".to_string()),
        })
        .add_technical_measure(AccountabilityMeasure::InternationalTransfers {
            transfers_outside_eea: true,
            chapter5_compliant: true,
            notes: Some("SCCs with US payment processor (Stripe), adequacy decision for UK".to_string()),
        })
        // Organizational measures
        .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
            ropa_maintained: true,
            up_to_date: true,
            notes: Some("ROPA reviewed quarterly, 8 processing activities documented".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::DataProtectionOfficer {
            dpo_required: false,
            dpo_designated: true,
            contact_published: true,
            notes: Some("Voluntary DPO designation - contact: dpo@euroshop.de".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: true,
            notes: Some("Article 28 contracts with 5 processors: hosting, email, analytics, payment, shipping".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::DataSubjectRightsProcedures {
            procedures_documented: true,
            response_process_established: true,
            notes: Some("DSAR procedure: 30-day response, online portal, identity verification".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::BreachNotificationProcedures {
            procedures_documented: true,
            tested: true,
            notes: Some("Breach response plan tested annually, 72-hour notification workflow".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::StaffTraining {
            training_program_established: true,
            frequency: Some("Quarterly".to_string()),
            notes: Some("GDPR training for all staff, specialized training for IT and customer service".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::PrivacyNotices {
            provided: true,
            compliant_with_article13_14: true,
            notes: Some("Privacy policy meets Article 13 requirements, reviewed by legal counsel".to_string()),
        })
        .with_compliance_documentation(true)
        .with_responsible_person("Maria Schmidt, Legal Counsel")
        .with_assessment_date(Utc::now())
        .with_notes("Annual accountability review - all measures operational");

    match accountability.validate() {
        Ok(validation) => {
            println!("📊 Accountability Assessment Results:");
            println!("   Controller: EuroShop GmbH");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Compliance Score: {}/100", validation.compliance_score);
            println!(
                "   Article 24(1) Considerations: {}",
                if validation.considerations_complete {
                    "✅ Complete"
                } else {
                    "❌ Incomplete"
                }
            );
            println!(
                "   Documentation Maintained: {}",
                if validation.documentation_maintained {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Technical Measures: {}",
                validation.technical_measures_count
            );
            println!(
                "   Organizational Measures: {}",
                validation.organizational_measures_count
            );
            println!("\n🔑 Key Accountability Elements:");
            println!(
                "   Article 25 (Data Protection by Design): {}",
                if validation.has_article25_compliance {
                    "✅"
                } else {
                    "❌"
                }
            );
            println!(
                "   Article 32 (Security Measures): {}",
                if validation.has_article32_compliance {
                    "✅"
                } else {
                    "❌"
                }
            );
            println!(
                "   Article 30 (ROPA): {}",
                if validation.has_article30_compliance {
                    "✅"
                } else {
                    "❌"
                }
            );

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!("\n✅ EuroShop demonstrates full Article 24 accountability.");
            println!("   All essential measures implemented and documented.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 2: Healthcare Provider
///
/// High-risk processing (Article 9 health data) requires enhanced accountability measures.
fn scenario2_healthcare_provider() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 2: Healthcare Provider (High-Risk Processing)");
    println!("──────────────────────────────────────────────────────────────\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("Klinik München AG")
        .with_processing_description("Electronic health records (EHR) system for patient care")
        .with_data_volume(DataVolume::Large)
        .with_data_sensitivity(DataSensitivity::Critical) // Article 9 health data + children
        .with_risk_level_assessed(SecurityRiskLevel::Critical)
        .with_all_considerations(true)
        .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
            implemented: true,
            documented: true,
            notes: Some(
                "Medical pseudonymisation, role-based access (doctors/nurses/admin), audit logging"
                    .to_string(),
            ),
        })
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: Some(
                "ISO 27001 certified, FIPS 140-2 encryption, biometric access, 24/7 SOC monitoring"
                    .to_string(),
            ),
        })
        .add_technical_measure(AccountabilityMeasure::DataProtectionImpactAssessment {
            dpia_required: true,
            dpia_conducted: true,
            notes: Some(
                "DPIA conducted 2024: High risk identified and mitigated with enhanced security"
                    .to_string(),
            ),
        })
        .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
            ropa_maintained: true,
            up_to_date: true,
            notes: Some(
                "ROPA maintained, 15 processing activities including EHR, billing, research"
                    .to_string(),
            ),
        })
        .add_organizational_measure(AccountabilityMeasure::DataProtectionOfficer {
            dpo_required: true,
            dpo_designated: true,
            contact_published: true,
            notes: Some(
                "Mandatory DPO (Article 37(1)(c) - large-scale special category processing)"
                    .to_string(),
            ),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: true,
            notes: Some(
                "Article 28 contracts with medical IT provider, lab system, imaging system"
                    .to_string(),
            ),
        })
        .add_organizational_measure(AccountabilityMeasure::StaffTraining {
            training_program_established: true,
            frequency: Some("Monthly".to_string()),
            notes: Some(
                "Healthcare-specific GDPR training, patient confidentiality, security awareness"
                    .to_string(),
            ),
        })
        .add_organizational_measure(AccountabilityMeasure::BreachNotificationProcedures {
            procedures_documented: true,
            tested: true,
            notes: Some(
                "Breach response plan aligned with medical incident response, tested semi-annually"
                    .to_string(),
            ),
        })
        .add_certification(ComplianceCertification::InformationSecurity {
            standard: "ISO/IEC 27001:2022".to_string(),
            certified: true,
            valid_until: Some(Utc::now() + chrono::Duration::days(365)),
        })
        .add_certification(ComplianceCertification::Other {
            name: "ISO 27799:2016 (Health Informatics Security)".to_string(),
            description: "Specialized health information security standard".to_string(),
            valid_until: Some(Utc::now() + chrono::Duration::days(365)),
        })
        .with_compliance_documentation(true)
        .with_responsible_person("Dr. Hans Weber, Chief Medical Information Officer")
        .with_assessment_date(Utc::now());

    match accountability.validate() {
        Ok(validation) => {
            println!("📊 Accountability Assessment Results:");
            println!("   Controller: Klinik München AG");
            println!("   Processing Type: Article 9 health data (critical risk)");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Compliance Score: {}/100", validation.compliance_score);
            println!(
                "   Certifications: {} (ISO 27001, ISO 27799)",
                validation.certifications_count
            );

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!(
                "\n✅ Healthcare provider demonstrates enhanced accountability for critical-risk processing."
            );
            println!("   DPIA conducted, DPO designated, ISO certifications in place.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 3: Small Business
///
/// Proportionate accountability measures for low-risk processing.
fn scenario3_small_business() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 3: Small Business (Proportionate Measures)");
    println!("──────────────────────────────────────────────────────────────\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("Bakery Schmidt & Sons")
        .with_processing_description("Customer newsletter and online ordering")
        .with_data_volume(DataVolume::Small) // <1000 customers
        .with_data_sensitivity(DataSensitivity::Low)
        .with_risk_level_assessed(SecurityRiskLevel::Low)
        .with_all_considerations(true)
        .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
            implemented: true,
            documented: true,
            notes: Some(
                "Newsletter opt-in only, no pre-ticked boxes, easy unsubscribe".to_string(),
            ),
        })
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: Some("TLS encryption, password-protected admin, regular updates".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
            ropa_maintained: false, // Exempted under Article 30(5)
            up_to_date: false,
            notes: Some(
                "Exempt under Article 30(5): <250 employees, occasional processing, low risk"
                    .to_string(),
            ),
        })
        .add_organizational_measure(AccountabilityMeasure::DataProtectionOfficer {
            dpo_required: false,
            dpo_designated: false,
            contact_published: false,
            notes: Some("No DPO required - not large-scale processing".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: true,
            notes: Some("Article 28 contract with email service provider (Mailchimp)".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::PrivacyNotices {
            provided: true,
            compliant_with_article13_14: true,
            notes: Some("Privacy notice on website and in-store".to_string()),
        })
        .with_compliance_documentation(true)
        .with_assessment_date(Utc::now());

    match accountability.validate() {
        Ok(validation) => {
            println!("📊 Accountability Assessment Results:");
            println!("   Controller: Bakery Schmidt & Sons");
            println!("   Processing Type: Low-risk, small-scale");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Compliance Score: {}/100", validation.compliance_score);

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!("\n✅ Small business demonstrates proportionate accountability.");
            println!("   Appropriate measures for low-risk processing, ROPA exemption applies.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 4: Missing Article 24(1) Considerations
///
/// Non-compliant: Controller has not documented Article 24(1) considerations.
fn scenario4_missing_considerations() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 4: Missing Article 24(1) Considerations (Non-Compliant)");
    println!("──────────────────────────────────────────────────────────────\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("TechStartup Ltd")
        .with_processing_description("Mobile app user analytics")
        .with_data_volume(DataVolume::Medium)
        .with_data_sensitivity(DataSensitivity::Medium)
        // Article 24(1) considerations NOT documented
        .with_nature_considered(false)
        .with_scope_considered(false)
        .with_context_considered(false)
        .with_purposes_considered(false)
        .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
            implemented: true,
            documented: false,
            notes: None,
        })
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: false,
            notes: None,
        })
        .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
            ropa_maintained: true,
            up_to_date: true,
            notes: None,
        })
        .with_compliance_documentation(false); // Not maintaining documentation

    match accountability.validate() {
        Ok(validation) => {
            println!("📊 Accountability Assessment Results:");
            println!("   Controller: TechStartup Ltd");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Compliance Score: {}/100", validation.compliance_score);
            println!(
                "   Article 24(1) Considerations: {}",
                if validation.considerations_complete {
                    "✅ Complete"
                } else {
                    "❌ Incomplete"
                }
            );
            println!(
                "   Documentation Maintained: {}",
                if validation.documentation_maintained {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );

            println!("\n⚠️  Warnings:");
            for warning in &validation.warnings {
                println!("   - {}", warning);
            }

            println!("\n❌ TechStartup fails to demonstrate Article 24 accountability.");
            println!(
                "   Article 24(1) requires considering nature, scope, context, and purposes of processing."
            );
            println!(
                "   Compliance documentation must be maintained to demonstrate accountability."
            );
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 5: Missing Essential Accountability Measures
///
/// Non-compliant: No Article 25, 32, or 30 compliance.
fn scenario5_missing_essential_measures() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 5: Missing Essential Measures (Non-Compliant)");
    println!("──────────────────────────────────────────────────────────────\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("Legacy Systems Inc")
        .with_processing_description("Customer database (pre-GDPR system)")
        .with_data_volume(DataVolume::Large)
        .with_data_sensitivity(DataSensitivity::Medium)
        .with_all_considerations(true)
        .with_compliance_documentation(true)
        // No Article 25, 32, or 30 measures implemented
        .add_organizational_measure(AccountabilityMeasure::DataProtectionOfficer {
            dpo_required: false,
            dpo_designated: false,
            contact_published: false,
            notes: Some("No DPO".to_string()),
        });

    match accountability.validate() {
        Ok(validation) => {
            println!("📊 Accountability Assessment Results:");
            println!("   Controller: Legacy Systems Inc");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Compliance Score: {}/100", validation.compliance_score);

            println!("\n⚠️  Warnings ({}):", validation.warnings.len());
            for warning in &validation.warnings {
                println!("   - {}", warning);
            }

            println!("\n❌ Legacy Systems Inc demonstrates severe accountability gaps.");
            println!("   Missing all three essential measures:");
            println!("   - Article 25: Data protection by design and by default");
            println!("   - Article 32: Security of processing");
            println!("   - Article 30: Records of processing activities");
            println!("\n   These are foundational accountability obligations.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 6: International Tech Company
///
/// Complex accountability framework with multiple certifications.
fn scenario6_international_tech_company() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 6: International Tech Company (Complex Accountability)");
    println!("──────────────────────────────────────────────────────────────\n");

    let accountability = ControllerAccountability::new()
        .with_controller_name("GlobalCloud Technologies AG")
        .with_processing_description("Cloud SaaS platform for enterprise customers (B2B)")
        .with_data_volume(DataVolume::VeryLarge) // >1M data subjects
        .with_data_sensitivity(DataSensitivity::Medium)
        .with_risk_level_assessed(SecurityRiskLevel::High)
        .with_all_considerations(true)
        .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
            implemented: true,
            documented: true,
            notes: Some("Privacy engineering team, privacy by design methodology, PETs (encryption, pseudonymisation, differential privacy)".to_string()),
        })
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: Some("SOC 2 Type II, ISO 27001, 99.99% uptime SLA, continuous monitoring".to_string()),
        })
        .add_technical_measure(AccountabilityMeasure::DataProtectionImpactAssessment {
            dpia_required: true,
            dpia_conducted: true,
            notes: Some("DPIAs conducted for all new features, automated DPIA screening tool".to_string()),
        })
        .add_technical_measure(AccountabilityMeasure::InternationalTransfers {
            transfers_outside_eea: true,
            chapter5_compliant: true,
            notes: Some("SCCs with US, UK, Singapore, Japan. Adequacy decision for UK, Japan. BCRs approved.".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
            ropa_maintained: true,
            up_to_date: true,
            notes: Some("Automated ROPA system, 150+ processing activities tracked".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::DataProtectionOfficer {
            dpo_required: true,
            dpo_designated: true,
            contact_published: true,
            notes: Some("Group DPO + local DPOs in each EU member state, dpo@globalcloud.com".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: true,
            notes: Some("Article 28 contracts with 50+ sub-processors, vendor management program".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::JointControllerArrangements {
            joint_controllers: true,
            article26_arrangement_documented: true,
            notes: Some("Article 26 arrangements with enterprise customers for B2B SaaS".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::DataSubjectRightsProcedures {
            procedures_documented: true,
            response_process_established: true,
            notes: Some("Self-service DSAR portal, automated data export, 30-day SLA".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::BreachNotificationProcedures {
            procedures_documented: true,
            tested: true,
            notes: Some("Incident response team, 72-hour notification workflow, tested quarterly".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::StaffTraining {
            training_program_established: true,
            frequency: Some("Quarterly".to_string()),
            notes: Some("Mandatory GDPR training for all staff, specialized training by role".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::PrivacyNotices {
            provided: true,
            compliant_with_article13_14: true,
            notes: Some("Multi-layered privacy notices, translated into 24 EU languages".to_string()),
        })
        .add_certification(ComplianceCertification::InformationSecurity {
            standard: "ISO/IEC 27001:2022".to_string(),
            certified: true,
            valid_until: Some(Utc::now() + chrono::Duration::days(365)),
        })
        .add_certification(ComplianceCertification::InformationSecurity {
            standard: "SOC 2 Type II".to_string(),
            certified: true,
            valid_until: Some(Utc::now() + chrono::Duration::days(365)),
        })
        .add_certification(ComplianceCertification::Other {
            name: "Privacy Shield Alternative (Binding Corporate Rules)".to_string(),
            description: "BCRs approved by lead supervisory authority".to_string(),
            valid_until: None,
        })
        .add_certification(ComplianceCertification::CodeOfConduct {
            code_name: "Cloud Infrastructure Services Providers in Europe (CISPE)".to_string(),
            approval_authority: "Belgian Data Protection Authority".to_string(),
            valid_until: None,
        })
        .with_compliance_documentation(true)
        .with_responsible_person("Chief Privacy Officer")
        .with_assessment_date(Utc::now())
        .with_notes("Annual external GDPR audit - full compliance confirmed");

    match accountability.validate() {
        Ok(validation) => {
            println!("📊 Accountability Assessment Results:");
            println!("   Controller: GlobalCloud Technologies AG");
            println!("   Scale: Very large (>1M data subjects)");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
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
                "   Certifications: {} (ISO 27001, SOC 2, BCRs, CISPE Code)",
                validation.certifications_count
            );

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!("\n✅ GlobalCloud demonstrates comprehensive Article 24 accountability.");
            println!("   Enterprise-grade accountability framework with multiple certifications.");
            println!("   Article 24(2): 4 certifications/codes of conduct demonstrate compliance.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}
