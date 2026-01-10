//! GDPR Processor Contract (Article 28) Example
//!
//! This example demonstrates processor contracts under Article 28 GDPR.
//!
//! ## Scenarios Covered
//!
//! 1. Complete compliant processor contract
//! 2. Contract with specific sub-processor authorization
//! 3. Contract with general sub-processor authorization
//! 4. Contract missing mandatory clauses (non-compliant)
//! 5. Contract missing Article 28(1) elements

use chrono::Utc;
use legalis_eu::gdpr::GdprError;
use legalis_eu::gdpr::processor_contract::*;

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Processor Contract (Article 28) Examples ===\n");

    scenario_1_complete_contract()?;
    scenario_2_specific_sub_processor_auth()?;
    scenario_3_general_sub_processor_auth()?;
    scenario_4_missing_mandatory_clauses()?;
    scenario_5_missing_article_28_1_elements()?;

    println!("\n✅ All processor contract scenarios completed");
    Ok(())
}

/// Scenario 1: Complete GDPR-Compliant Processor Contract
fn scenario_1_complete_contract() -> Result<(), GdprError> {
    println!("## Scenario 1: Complete Processor Contract (Fully Compliant)\n");

    let contract = ProcessorContract::new()
        .with_controller("Acme Corporation", "privacy@acme.com")
        .with_processor("CloudService GmbH", "contact@cloudservice.de")
        .with_subject_matter("Customer data processing and cloud storage")
        .with_duration_months(24)
        .with_processing_nature("Cloud-based data storage, backup, and retrieval services")
        .with_processing_purpose("Providing secure data storage infrastructure for controller's customer relationship management system")
        .add_data_category("customer names")
        .add_data_category("customer email addresses")
        .add_data_category("customer phone numbers")
        .add_data_category("purchase history")
        .add_data_subject_category("customers")
        .add_data_subject_category("prospective customers")
        .with_all_mandatory_clauses()
        .in_writing(true)
        .with_signed_date(Utc::now())
        .with_notes("Contract includes SLA with 99.9% uptime guarantee and European data center requirements");

    let validation = contract.validate()?;

    println!("Controller: Acme Corporation (privacy@acme.com)");
    println!("Processor: CloudService GmbH (contact@cloudservice.de)");
    println!("Subject Matter: Customer data processing and cloud storage");
    println!("Duration: 24 months");
    println!("Contract in Writing: Yes");
    println!();

    println!("✅ ARTICLE 28(1) REQUIREMENTS:");
    println!(
        "  {} Controller details",
        if validation.has_controller {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Processor details",
        if validation.has_processor {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Subject matter specified",
        if validation.has_subject_matter {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Duration specified",
        if validation.has_duration {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Data categories specified",
        if validation.has_data_categories {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} In writing",
        if validation.in_writing { "✓" } else { "✗" }
    );
    println!();

    println!("✅ ARTICLE 28(3) MANDATORY CLAUSES:");
    for clause in Article28Clause::all_mandatory() {
        println!(
            "  ✓ {} - {}",
            clause.article_reference(),
            match clause {
                Article28Clause::ProcessOnlyOnInstructions => "Process only on instructions",
                Article28Clause::ConfidentialityObligation => "Confidentiality obligation",
                Article28Clause::SecurityMeasures => "Security measures (Art. 32)",
                Article28Clause::SubProcessorConditions => "Sub-processor conditions",
                Article28Clause::AssistDataSubjectRights => "Assist with data subject rights",
                Article28Clause::AssistSecurity => "Assist with security/breach/DPIA",
                Article28Clause::DeletionOrReturn => "Deletion or return of data",
                Article28Clause::AuditsAndInspections => "Audits and inspections",
            }
        );
    }
    println!();

    println!(
        "OVERALL COMPLIANCE: {}",
        if validation.compliant {
            "✅ FULLY COMPLIANT"
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

    println!("\n💡 KEY POINT:");
    println!("   Article 28(1) requires a written contract (or other legal act) that sets out");
    println!("   the subject matter, duration, nature, purpose, type of data, and categories");
    println!("   of data subjects. Article 28(3) requires 8 mandatory clauses that bind the");
    println!("   processor to specific obligations.\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Contract with Specific Sub-Processor Authorization
fn scenario_2_specific_sub_processor_auth() -> Result<(), GdprError> {
    println!("## Scenario 2: Specific Sub-Processor Authorization (Article 28(2))\n");

    let contract = ProcessorContract::new()
        .with_controller("TechStart GmbH", "data@techstart.de")
        .with_processor("DataCenter Solutions", "contracts@datacenter.com")
        .with_subject_matter("Email hosting and storage")
        .with_duration_indefinite(90)
        .with_processing_nature("Cloud email infrastructure")
        .with_processing_purpose("Business email services")
        .add_data_category("employee emails")
        .add_data_category("attachments")
        .add_data_subject_category("employees")
        .with_all_mandatory_clauses()
        .in_writing(true)
        .with_specific_sub_processor_auth(vec![
            "BackupService Inc (US)".to_string(),
            "SecurityMonitor Ltd (UK)".to_string(),
        ])
        .add_sub_processor(SubProcessor {
            name: "BackupService Inc".to_string(),
            contact: "legal@backupservice.com".to_string(),
            activities: vec!["Encrypted backup storage".to_string()],
            authorized_date: Some(Utc::now()),
        })
        .add_sub_processor(SubProcessor {
            name: "SecurityMonitor Ltd".to_string(),
            contact: "security@secmonitor.co.uk".to_string(),
            activities: vec!["Security monitoring and threat detection".to_string()],
            authorized_date: Some(Utc::now()),
        });

    let validation = contract.validate()?;

    println!("Controller: TechStart GmbH");
    println!("Processor: DataCenter Solutions");
    println!("Sub-Processor Authorization: Specific (controller must approve each)");
    println!(
        "Number of Sub-Processors: {}",
        validation.sub_processor_count
    );
    println!();

    println!("📋 AUTHORIZED SUB-PROCESSORS:");
    if let Some(SubProcessorAuthorization::Specific {
        authorized_processors,
    }) = &contract.sub_processor_authorization
    {
        for (idx, sp) in authorized_processors.iter().enumerate() {
            println!("   {}. {}", idx + 1, sp);
        }
    }
    println!();

    println!(
        "COMPLIANCE STATUS: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 28(2) SPECIFIC AUTHORIZATION:");
    println!(
        "   - Controller must give prior specific written authorization for EACH sub-processor"
    );
    println!("   - Processor cannot engage new sub-processor without explicit controller approval");
    println!("   - Provides maximum control to controller");
    println!("   - Article 28(4): Same obligations must be imposed on sub-processor\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Contract with General Sub-Processor Authorization
fn scenario_3_general_sub_processor_auth() -> Result<(), GdprError> {
    println!("## Scenario 3: General Sub-Processor Authorization (Article 28(2))\n");

    let contract = ProcessorContract::new()
        .with_controller("HealthCare AG", "compliance@healthcare.de")
        .with_processor("MedTech Services", "legal@medtech.com")
        .with_subject_matter("Patient data processing platform")
        .with_duration_fixed(Utc::now(), Utc::now() + chrono::Duration::days(365 * 3))
        .with_processing_nature("Healthcare IT infrastructure")
        .with_processing_purpose("Electronic health records management")
        .add_data_category("patient medical records")
        .add_data_category("diagnostic data")
        .add_data_subject_category("patients")
        .with_all_mandatory_clauses()
        .in_writing(true)
        .with_general_sub_processor_auth(
            30, // 30-day objection period
            vec![
                "CloudBackup Pro".to_string(),
                "DataAnalytics Inc".to_string(),
            ],
        )
        .add_sub_processor(SubProcessor {
            name: "CloudBackup Pro".to_string(),
            contact: "enterprise@cloudbackup.com".to_string(),
            activities: vec!["HIPAA-compliant backup services".to_string()],
            authorized_date: Some(Utc::now()),
        })
        .add_sub_processor(SubProcessor {
            name: "DataAnalytics Inc".to_string(),
            contact: "healthcare@datalytics.com".to_string(),
            activities: vec!["Anonymized data analytics".to_string()],
            authorized_date: Some(Utc::now()),
        });

    let validation = contract.validate()?;

    println!("Controller: HealthCare AG");
    println!("Processor: MedTech Services");
    println!("Sub-Processor Authorization: General (with notification and objection right)");

    if let Some(SubProcessorAuthorization::General {
        objection_period_days,
        current_processors,
    }) = &contract.sub_processor_authorization
    {
        println!("Objection Period: {} days", objection_period_days);
        println!("Current Sub-Processors: {}", current_processors.len());
    }
    println!();

    println!(
        "COMPLIANCE STATUS: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 28(2) GENERAL AUTHORIZATION:");
    println!("   - Controller gives prior general written authorization");
    println!("   - Processor must inform controller of intended changes (additions/replacements)");
    println!("   - Controller has right to OBJECT within reasonable period (e.g., 30 days)");
    println!("   - More flexible for processor, but controller retains oversight");
    println!("   - If controller objects, processor must not use that sub-processor\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: Contract Missing Mandatory Clauses (Non-Compliant)
fn scenario_4_missing_mandatory_clauses() -> Result<(), GdprError> {
    println!("## Scenario 4: Contract Missing Mandatory Clauses (Non-Compliant)\n");

    let contract = ProcessorContract::new()
        .with_controller("SmallBusiness Ltd", "owner@smallbiz.com")
        .with_processor("CheapHost.com", "support@cheaphost.com")
        .with_subject_matter("Website hosting")
        .with_duration_months(12)
        .with_processing_nature("Shared web hosting")
        .with_processing_purpose("Website operation")
        .add_data_category("website visitor data")
        .add_data_subject_category("visitors")
        // Only 2 of 8 mandatory clauses
        .with_clause(Article28Clause::ProcessOnlyOnInstructions)
        .with_clause(Article28Clause::SecurityMeasures)
        .in_writing(true);

    let validation = contract.validate()?;

    println!("Controller: SmallBusiness Ltd");
    println!("Processor: CheapHost.com");
    println!("Mandatory Clauses Included: 2 of 8");
    println!();

    println!("COMPLIANCE STATUS: ❌ NON-COMPLIANT");
    println!();

    println!("⚠️ MISSING MANDATORY CLAUSES:");
    for (idx, clause) in validation.missing_clauses.iter().enumerate() {
        println!(
            "   {}. {} - {}",
            idx + 1,
            clause.article_reference(),
            clause.description()
        );
    }
    println!();

    println!("💡 KEY POINT:");
    println!("   Article 28(3) requires ALL 8 clauses. Missing even one clause means");
    println!("   the contract is non-compliant with GDPR. Controllers are jointly liable");
    println!("   for processors that don't meet Article 28 requirements.\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 5: Contract Missing Article 28(1) Elements
fn scenario_5_missing_article_28_1_elements() -> Result<(), GdprError> {
    println!("## Scenario 5: Contract Missing Article 28(1) Elements\n");

    let contract = ProcessorContract::new()
        .with_all_mandatory_clauses()
        .in_writing(true)
        // Missing: controller, processor, subject matter, duration, nature, purpose, data categories
        .add_data_subject_category("users");

    let validation = contract.validate()?;

    println!("Contract has all 8 mandatory clauses BUT missing Article 28(1) elements");
    println!();

    println!("COMPLIANCE STATUS: ❌ NON-COMPLIANT");
    println!();

    println!("⚠️ WARNINGS ({} issues):", validation.warnings.len());
    for (idx, warning) in validation.warnings.iter().enumerate() {
        println!("   {}. {}", idx + 1, warning);
    }
    println!();

    println!("💡 KEY POINT:");
    println!("   Article 28(1) requires the contract to set out:");
    println!("   - Subject matter and duration of processing");
    println!("   - Nature and purpose of processing");
    println!("   - Type of personal data and categories of data subjects");
    println!("   - Obligations and rights of the controller");
    println!();
    println!("   Even with all mandatory clauses, missing these basic elements makes");
    println!("   the contract non-compliant. These details ensure both parties understand");
    println!("   the scope and boundaries of the processing arrangement.\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}
