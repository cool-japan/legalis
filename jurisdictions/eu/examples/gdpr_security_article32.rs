//! GDPR Article 32 - Security of Processing Example
//!
//! This example demonstrates security measures under Article 32 GDPR.
//!
//! ## Scenarios Covered
//!
//! 1. Comprehensive high-risk security assessment (fully compliant)
//! 2. Healthcare provider with medical data (high security requirements)
//! 3. Small business with basic security (low-risk compliant)
//! 4. Financial institution (critical risk with comprehensive measures)
//! 5. Non-compliant setup missing mandatory testing
//! 6. Inadequate high-risk setup missing encryption

use chrono::Utc;
use legalis_eu::gdpr::*;

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Article 32 - Security of Processing Examples ===\n");

    scenario_1_comprehensive_high_risk_assessment()?;
    scenario_2_healthcare_provider()?;
    scenario_3_small_business_low_risk()?;
    scenario_4_financial_institution_critical_risk()?;
    scenario_5_non_compliant_missing_testing()?;
    scenario_6_high_risk_missing_encryption()?;

    println!("\n✅ All Article 32 scenarios completed");
    Ok(())
}

/// Scenario 1: Comprehensive High-Risk Security Assessment (Fully Compliant)
fn scenario_1_comprehensive_high_risk_assessment() -> Result<(), GdprError> {
    println!("## Scenario 1: Comprehensive High-Risk Security Assessment\\n");

    let assessment = SecurityAssessment::new()
        .with_entity("TechCorp International")
        .with_risk_level(SecurityRiskLevel::High)
        // Article 32(1)(a) - Pseudonymisation and Encryption
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256-GCM".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "SHA-256 hashing with per-user salt".to_string(),
        })
        // Article 32(1)(b) - Confidentiality, Integrity, Availability, Resilience
        .add_technical_measure(TechnicalMeasure::Confidentiality {
            access_logging: true,
            intrusion_detection: true,
        })
        .add_technical_measure(TechnicalMeasure::Integrity {
            checksums: true,
            digital_signatures: true,
            version_control: true,
        })
        .add_technical_measure(TechnicalMeasure::Availability {
            redundancy: true,
            load_balancing: true,
            uptime_sla: Some(99.95),
        })
        .add_technical_measure(TechnicalMeasure::Resilience {
            fault_tolerance: true,
            geographic_redundancy: true,
        })
        // Article 32(1)(c) - Backup and Recovery
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Hourly incremental, daily full".to_string(),
            recovery_time_objective: "4 hours".to_string(),
            recovery_point_objective: "1 hour".to_string(),
            tested: true,
        })
        // Article 32(1)(d) - Testing and Assessment
        .add_technical_measure(TechnicalMeasure::TestingAssessment {
            penetration_testing: true,
            vulnerability_scanning: true,
            frequency: "Quarterly penetration tests, weekly vulnerability scans".to_string(),
        })
        // Organizational Measures
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: true,
            least_privilege: true,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Mandatory quarterly security awareness training".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::IncidentResponse {
            documented: true,
            tested: true,
        })
        .add_organizational_measure(OrganizationalMeasure::SecurityPolicies {
            documented: true,
            reviewed_regularly: true,
        })
        .add_organizational_measure(OrganizationalMeasure::VendorManagement {
            due_diligence: true,
            contracts_in_place: true, // Article 28 processor contracts
        })
        .add_organizational_measure(OrganizationalMeasure::PhysicalSecurity {
            access_control: true,
            surveillance: true,
        })
        // Article 32(1) Considerations
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true)
        .with_assessment_date(Utc::now())
        .with_notes("Annual security audit by external assessor. ISO 27001 certified.");

    let validation = assessment.validate()?;

    println!("Entity: TechCorp International");
    println!("Risk Level: High");
    println!(
        "Assessment Date: {}",
        assessment.assessment_date.unwrap().format("%Y-%m-%d")
    );
    println!();

    println!(
        "TECHNICAL MEASURES ({}):",
        validation.technical_measures_count
    );
    println!("  ✓ Encryption (AES-256-GCM): data at rest + in transit");
    println!("  ✓ Pseudonymisation: SHA-256 with per-user salt");
    println!("  ✓ Confidentiality: access logging + intrusion detection");
    println!("  ✓ Integrity: checksums + digital signatures + version control");
    println!("  ✓ Availability: redundancy + load balancing (99.95% SLA)");
    println!("  ✓ Resilience: fault tolerance + geographic redundancy");
    println!("  ✓ Backup/Recovery: hourly incremental, RTO 4h, RPO 1h");
    println!("  ✓ Testing: quarterly penetration tests + weekly vulnerability scans");
    println!();

    println!(
        "ORGANIZATIONAL MEASURES ({}):",
        validation.organizational_measures_count
    );
    println!("  ✓ Access control: role-based + least privilege");
    println!("  ✓ Staff training: quarterly mandatory");
    println!("  ✓ Incident response: documented + tested");
    println!("  ✓ Security policies: documented + regularly reviewed");
    println!("  ✓ Vendor management: due diligence + Article 28 contracts");
    println!("  ✓ Physical security: access control + surveillance");
    println!();

    println!("ARTICLE 32(1) COMPLIANCE:");
    println!(
        "  {} Encryption/Pseudonymisation",
        if validation.has_encryption || validation.has_pseudonymisation {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Backup and Recovery",
        if validation.has_backup_recovery {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Testing and Assessment",
        if validation.has_testing { "✓" } else { "✗" }
    );
    println!("  ✓ State of the art considered");
    println!("  ✓ Implementation costs considered");
    println!("  ✓ Processing context considered");
    println!();

    println!(
        "OVERALL COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    if !validation.warnings.is_empty() {
        println!("\n⚠️ WARNINGS:");
        for warning in &validation.warnings {
            println!("  - {}", warning);
        }
    }

    if !validation.recommendations.is_empty() {
        println!("\n💡 RECOMMENDATIONS:");
        for rec in &validation.recommendations {
            println!("  - {}", rec);
        }
    }

    println!("\n{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Healthcare Provider (High Security for Medical Data)
fn scenario_2_healthcare_provider() -> Result<(), GdprError> {
    println!("## Scenario 2: Healthcare Provider (Medical Data)\\n");

    let assessment = SecurityAssessment::new()
        .with_entity("City Hospital")
        .with_risk_level(SecurityRiskLevel::High)
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256 with FIPS 140-2 validated modules".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "Clinical data pseudonymisation for research".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Real-time replication + daily backups".to_string(),
            recovery_time_objective: "2 hours (patient care continuity)".to_string(),
            recovery_point_objective: "30 minutes".to_string(),
            tested: true,
        })
        .add_technical_measure(TechnicalMeasure::TestingAssessment {
            penetration_testing: true,
            vulnerability_scanning: true,
            frequency: "Monthly vulnerability scans, bi-annual penetration tests".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: true,
            least_privilege: true,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Annual HIPAA/GDPR training for all staff".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::PhysicalSecurity {
            access_control: true,
            surveillance: true,
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true)
        .with_notes("Healthcare context requires maximum availability and integrity. Article 9 special category data.");

    let validation = assessment.validate()?;

    println!("Entity: City Hospital");
    println!("Risk Level: High (Article 9 special category - health data)");
    println!("Context: Patient electronic health records");
    println!();

    println!("KEY SECURITY MEASURES:");
    println!("  ✓ FIPS 140-2 validated encryption modules");
    println!("  ✓ Clinical data pseudonymisation for research");
    println!("  ✓ Real-time replication (RTO: 2h, RPO: 30min)");
    println!("  ✓ Role-based access control for medical staff");
    println!("  ✓ Physical security: badge access + CCTV");
    println!();

    println!(
        "COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 32 requires measures \"appropriate to the risk\". For healthcare");
    println!("   (Article 9 special categories), high security is mandatory including:");
    println!("   - Strong encryption for patient data at rest and in transit");
    println!("   - Stringent access controls (medical ethics + GDPR)");
    println!("   - Low RTO/RPO for patient care continuity");
    println!("   - Regular testing to ensure effectiveness\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Small Business with Basic Security (Low-Risk Compliant)
fn scenario_3_small_business_low_risk() -> Result<(), GdprError> {
    println!("## Scenario 3: Small Business with Basic Security\\n");

    let assessment = SecurityAssessment::new()
        .with_entity("Local Bakery")
        .with_risk_level(SecurityRiskLevel::Low)
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: false,
            data_in_transit: true,
            algorithm: "TLS 1.3 for online orders".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Weekly cloud backups".to_string(),
            recovery_time_objective: "24 hours".to_string(),
            recovery_point_objective: "1 week".to_string(),
            tested: false,
        })
        .add_technical_measure(TechnicalMeasure::TestingAssessment {
            penetration_testing: false,
            vulnerability_scanning: true,
            frequency: "Annual vulnerability scans via cloud provider".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Annual data protection training for 2 employees".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: false,
            least_privilege: true,
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true)
        .with_notes("Small business with limited customer data (names, emails, phone numbers for orders). No special categories.");

    let validation = assessment.validate()?;

    println!("Entity: Local Bakery");
    println!("Risk Level: Low");
    println!("Data Processed: Customer contact info (names, emails, phone) for orders");
    println!();

    println!("SECURITY MEASURES:");
    println!("  ✓ TLS 1.3 encryption for online orders");
    println!("  ✓ Weekly cloud backups (RTO: 24h, RPO: 1 week)");
    println!("  ✓ Annual vulnerability scans");
    println!("  ✓ Staff training (2 employees)");
    println!("  ✓ Least-privilege access");
    println!();

    println!(
        "COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    if !validation.recommendations.is_empty() {
        println!("\n💡 RECOMMENDATIONS FOR IMPROVEMENT:");
        for rec in &validation.recommendations {
            println!("  - {}", rec);
        }
        println!("\nNote: These are optional improvements for low-risk processing.");
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 32 is risk-based and proportionate. Small businesses processing");
    println!("   low-risk data (no special categories, limited volume) can implement");
    println!("   basic security measures. The key is:");
    println!("   - Appropriate to the risk (not excessive or insufficient)");
    println!("   - Consider costs and state of the art");
    println!("   - Regular testing (annual scans acceptable for low-risk)\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: Financial Institution (Critical Risk)
fn scenario_4_financial_institution_critical_risk() -> Result<(), GdprError> {
    println!("## Scenario 4: Financial Institution (Critical Risk)\\n");

    let assessment = SecurityAssessment::new()
        .with_entity("European Investment Bank")
        .with_risk_level(SecurityRiskLevel::Critical)
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256-GCM (data at rest), TLS 1.3 with perfect forward secrecy (in transit)".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "Tokenization for payment card data (PCI-DSS compliance)".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Confidentiality {
            access_logging: true,
            intrusion_detection: true,
        })
        .add_technical_measure(TechnicalMeasure::Integrity {
            checksums: true,
            digital_signatures: true,
            version_control: true,
        })
        .add_technical_measure(TechnicalMeasure::Availability {
            redundancy: true,
            load_balancing: true,
            uptime_sla: Some(99.99),
        })
        .add_technical_measure(TechnicalMeasure::Resilience {
            fault_tolerance: true,
            geographic_redundancy: true,
        })
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Continuous replication + hourly snapshots".to_string(),
            recovery_time_objective: "1 hour".to_string(),
            recovery_point_objective: "15 minutes".to_string(),
            tested: true,
        })
        .add_technical_measure(TechnicalMeasure::TestingAssessment {
            penetration_testing: true,
            vulnerability_scanning: true,
            frequency: "Monthly vulnerability scans, quarterly penetration tests, annual red team exercises".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: true,
            least_privilege: true,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Mandatory monthly security training + annual certification".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::IncidentResponse {
            documented: true,
            tested: true,
        })
        .add_organizational_measure(OrganizationalMeasure::SecurityPolicies {
            documented: true,
            reviewed_regularly: true,
        })
        .add_organizational_measure(OrganizationalMeasure::VendorManagement {
            due_diligence: true,
            contracts_in_place: true,
        })
        .add_organizational_measure(OrganizationalMeasure::PhysicalSecurity {
            access_control: true,
            surveillance: true,
        })
        .add_organizational_measure(OrganizationalMeasure::BusinessContinuity {
            documented: true,
            tested: true,
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true)
        .with_notes("Financial sector requires highest security standards. ISO 27001, PCI-DSS, and SOC 2 Type II certified.");

    let validation = assessment.validate()?;

    println!("Entity: European Investment Bank");
    println!("Risk Level: Critical");
    println!("Compliance: ISO 27001, PCI-DSS, SOC 2 Type II");
    println!();

    println!(
        "TECHNICAL MEASURES: {}",
        validation.technical_measures_count
    );
    println!("  ✓ Military-grade encryption (AES-256-GCM, TLS 1.3 with PFS)");
    println!("  ✓ Payment card tokenization (PCI-DSS)");
    println!("  ✓ 24/7 intrusion detection and access logging");
    println!("  ✓ Digital signatures for all transactions");
    println!("  ✓ 99.99% uptime SLA with geographic redundancy");
    println!("  ✓ Continuous replication (RTO: 1h, RPO: 15min)");
    println!("  ✓ Monthly scans + quarterly pentests + annual red team");
    println!();

    println!(
        "ORGANIZATIONAL MEASURES: {}",
        validation.organizational_measures_count
    );
    println!("  ✓ Strict role-based access control");
    println!("  ✓ Monthly training + annual security certification");
    println!("  ✓ Incident response + business continuity (tested)");
    println!("  ✓ Physical security: biometric access + 24/7 CCTV");
    println!();

    println!(
        "COMPLIANCE: {}",
        if validation.compliant {
            "✅ FULLY COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Critical risk processing (financial data, large-scale operations) requires:");
    println!("   - Article 32: Maximum technical and organizational measures");
    println!("   - Article 35: DPIA is mandatory");
    println!("   - Article 28: Strict processor contracts with sub-processor approval");
    println!("   - Regular audits and certifications (ISO 27001, SOC 2)\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 5: Non-Compliant Setup (Missing Mandatory Testing)
fn scenario_5_non_compliant_missing_testing() -> Result<(), GdprError> {
    println!("## Scenario 5: Non-Compliant Setup (Missing Testing)\\n");

    let assessment = SecurityAssessment::new()
        .with_entity("StartupCo")
        .with_risk_level(SecurityRiskLevel::Medium)
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Daily".to_string(),
            recovery_time_objective: "8 hours".to_string(),
            recovery_point_objective: "24 hours".to_string(),
            tested: false,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Annual".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true);

    let validation = assessment.validate()?;

    println!("Entity: StartupCo");
    println!("Risk Level: Medium");
    println!();

    println!("SECURITY MEASURES:");
    println!("  ✓ Encryption (AES-256)");
    println!("  ✓ Daily backups");
    println!("  ✓ Staff training");
    println!("  ✗ NO TESTING OR ASSESSMENT");
    println!();

    println!("COMPLIANCE: ❌ NON-COMPLIANT");
    println!();

    println!("⚠️ CRITICAL VIOLATIONS:");
    for warning in &validation.warnings {
        println!("  - {}", warning);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 32(1)(d) requires \"regular testing, assessment and evaluation");
    println!("   of the effectiveness of technical and organizational measures.\"");
    println!();
    println!("   This is MANDATORY, not optional. Organizations must:");
    println!("   - Conduct vulnerability scanning");
    println!("   - Perform penetration testing (frequency depends on risk)");
    println!("   - Test backup recovery procedures");
    println!("   - Evaluate effectiveness of security policies");
    println!();
    println!("   Failure to test = Article 32 violation = potential administrative fine\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 6: High-Risk Setup Missing Encryption (Non-Compliant)
fn scenario_6_high_risk_missing_encryption() -> Result<(), GdprError> {
    println!("## Scenario 6: High-Risk Setup Missing Encryption\\n");

    let assessment = SecurityAssessment::new()
        .with_entity("DataBroker Inc")
        .with_risk_level(SecurityRiskLevel::High)
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Daily".to_string(),
            recovery_time_objective: "4 hours".to_string(),
            recovery_point_objective: "1 hour".to_string(),
            tested: true,
        })
        .add_technical_measure(TechnicalMeasure::TestingAssessment {
            penetration_testing: true,
            vulnerability_scanning: true,
            frequency: "Quarterly".to_string(),
        })
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: true,
            least_privilege: true,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Quarterly".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true)
        .with_notes("Large-scale profiling and behavioral analysis - high risk");

    let validation = assessment.validate()?;

    println!("Entity: DataBroker Inc");
    println!("Risk Level: High (large-scale profiling and behavioral analysis)");
    println!();

    println!("SECURITY MEASURES:");
    println!("  ✓ Backup and recovery (tested)");
    println!("  ✓ Quarterly penetration testing");
    println!("  ✓ Role-based access control");
    println!("  ✓ Quarterly staff training");
    println!("  ✗ NO ENCRYPTION OR PSEUDONYMISATION");
    println!();

    println!("COMPLIANCE: ❌ NON-COMPLIANT");
    println!();

    println!("⚠️ CRITICAL VIOLATIONS:");
    for warning in &validation.warnings {
        println!("  - {}", warning);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 32(1)(a) explicitly lists pseudonymisation and encryption as");
    println!("   primary security measures. For high/critical risk processing:");
    println!();
    println!("   - At least ONE of these is effectively mandatory");
    println!("   - Encryption should cover data at rest AND in transit");
    println!("   - Pseudonymisation reduces risk of re-identification");
    println!();
    println!("   Recital 83: \"In order to maintain security and prevent processing in");
    println!("   infringement of this Regulation, the controller or processor should");
    println!("   evaluate the risks inherent in the processing and implement measures");
    println!("   to mitigate those risks, such as encryption.\"");
    println!();
    println!("   For high-risk data profiling WITHOUT encryption = serious violation\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}
