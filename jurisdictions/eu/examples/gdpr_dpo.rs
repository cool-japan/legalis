//! GDPR Data Protection Officer (DPO) Example
//!
//! This example demonstrates DPO designation requirements under Articles 37-39 GDPR.
//!
//! ## Scenarios Covered
//!
//! 1. Public authority (mandatory DPO)
//! 2. Judicial court (exempt from DPO)
//! 3. Large-scale monitoring (mandatory DPO)
//! 4. Large-scale special categories processing (mandatory DPO)
//! 5. Small business (no DPO required)
//! 6. Member state law requirement (German BDSG)
//! 7. Complete DPO designation validation

use chrono::Utc;
use legalis_eu::gdpr::GdprError;
use legalis_eu::gdpr::dpo::*;
use legalis_eu::gdpr::types::SpecialCategory;
use legalis_eu::shared::MemberState;

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Data Protection Officer (DPO) Examples ===\n");

    scenario_1_public_authority()?;
    scenario_2_judicial_court()?;
    scenario_3_large_scale_monitoring()?;
    scenario_4_special_categories_hospital()?;
    scenario_5_small_business()?;
    scenario_6_member_state_law()?;
    scenario_7_complete_dpo_designation()?;

    println!("\n✅ All DPO scenarios completed");
    Ok(())
}

/// Scenario 1: Public Authority - Mandatory DPO (Article 37(1)(a))
fn scenario_1_public_authority() -> Result<(), GdprError> {
    println!("## Scenario 1: Public Authority (City Council)\n");

    let assessment = DpoDesignationAssessment::new()
        .with_entity_type(DpoEntityType::PublicAuthority)
        .with_organization_name("Hamburg City Council");

    let result = assessment.validate()?;

    println!("Organization: Hamburg City Council");
    println!("Entity Type: Public Authority");
    println!(
        "DPO Required: {}",
        if result.dpo_required {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("Legal Basis: {}", result.article);
    println!("Reason: {}", result.reason);

    println!("\n📋 Recommendations:");
    for (idx, rec) in result.recommendations.iter().enumerate() {
        println!("   {}. {}", idx + 1, rec);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 37(1)(a): Processing by a public authority (except courts in");
    println!("   judicial capacity) ALWAYS requires DPO designation.\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Judicial Court - Exempt from DPO Requirement
fn scenario_2_judicial_court() -> Result<(), GdprError> {
    println!("## Scenario 2: Judicial Court (Exempt)\n");

    let assessment = DpoDesignationAssessment::new()
        .with_entity_type(DpoEntityType::JudicialCourt)
        .with_organization_name("Federal Supreme Court");

    let result = assessment.validate()?;

    println!("Organization: Federal Supreme Court");
    println!("Entity Type: Court acting in judicial capacity");
    println!(
        "DPO Required: {}",
        if result.dpo_required {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("Legal Basis: {}", result.article);
    println!("Reason: {}", result.reason);

    println!("\n💡 KEY POINT:");
    println!("   Courts acting in their judicial capacity are exempt from Article 37(1)(a)");
    println!("   DPO requirements to preserve judicial independence.\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Large-Scale Monitoring - Online Advertising Platform (Article 37(1)(b))
fn scenario_3_large_scale_monitoring() -> Result<(), GdprError> {
    println!("## Scenario 3: Large-Scale Monitoring (AdTech Platform)\n");

    let assessment = DpoDesignationAssessment::new()
        .with_entity_type(DpoEntityType::PrivateEntity)
        .with_organization_name("GlobalAds GmbH")
        .add_monitoring_activity(MonitoringType::BehavioralAdvertising)
        .add_monitoring_activity(MonitoringType::LocationTracking)
        .add_monitoring_activity(MonitoringType::CreditScoring)
        .with_monitoring_core_activity(CoreActivity::Core {
            description: "Behavioral advertising and user profiling platform".to_string(),
        })
        .with_monitoring_scale(ProcessingScale::LargeScale)
        .monitoring_is_regular(true)
        .monitoring_is_systematic(true);

    let result = assessment.validate()?;

    println!("Organization: GlobalAds GmbH");
    println!("Entity Type: Private Company");
    println!("Core Activity: Behavioral advertising platform");
    println!("Monitoring Types:");
    println!("  - Behavioral advertising");
    println!("  - Location tracking");
    println!("  - Credit scoring/profiling");
    println!("Scale: Large");
    println!("Regular: Yes");
    println!("Systematic: Yes");
    println!();
    println!(
        "DPO Required: {}",
        if result.dpo_required {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("Legal Basis: {}", result.article);
    println!("Reason: {}", result.reason);

    println!("\n📋 Recommendations:");
    for (idx, rec) in result.recommendations.iter().enumerate() {
        println!("   {}. {}", idx + 1, rec);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 37(1)(b): DPO required when core activities consist of regular");
    println!("   and systematic monitoring of data subjects on a LARGE SCALE.");
    println!("   WP29 Guidelines: Consider number of data subjects, volume, duration,");
    println!("   geographical extent.\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: Large-Scale Special Categories - Hospital (Article 37(1)(c))
fn scenario_4_special_categories_hospital() -> Result<(), GdprError> {
    println!("## Scenario 4: Large-Scale Special Categories (Hospital)\n");

    let assessment = DpoDesignationAssessment::new()
        .with_entity_type(DpoEntityType::PrivateEntity)
        .with_organization_name("Charité University Hospital")
        .processes_special_categories(true)
        .add_special_category(SpecialCategory::HealthData)
        .add_special_category(SpecialCategory::GeneticData)
        .add_special_category(SpecialCategory::BiometricData)
        .with_special_categories_core_activity(CoreActivity::Core {
            description: "Patient care and medical records management".to_string(),
        })
        .with_special_categories_scale(ProcessingScale::LargeScale);

    let result = assessment.validate()?;

    println!("Organization: Charité University Hospital");
    println!("Entity Type: Private Hospital");
    println!("Core Activity: Patient care and medical records");
    println!("Special Categories:");
    println!("  - Health data");
    println!("  - Genetic data");
    println!("  - Biometric data");
    println!("Scale: Large (thousands of patients)");
    println!();
    println!(
        "DPO Required: {}",
        if result.dpo_required {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("Legal Basis: {}", result.article);
    println!("Reason: {}", result.reason);

    println!("\n📋 Recommendations:");
    for (idx, rec) in result.recommendations.iter().enumerate() {
        println!("   {}. {}", idx + 1, rec);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 37(1)(c): DPO required when core activities consist of");
    println!("   large-scale processing of SPECIAL CATEGORIES (Article 9) or");
    println!("   criminal conviction data (Article 10).\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 5: Small Business - No DPO Required
fn scenario_5_small_business() -> Result<(), GdprError> {
    println!("## Scenario 5: Small Business (No DPO Required)\n");

    let assessment = DpoDesignationAssessment::new()
        .with_entity_type(DpoEntityType::PrivateEntity)
        .with_organization_name("Small Café Berlin")
        .add_monitoring_activity(MonitoringType::VideoSurveillance)
        .with_monitoring_core_activity(CoreActivity::Ancillary {
            description: "Security cameras for premises security".to_string(),
        })
        .with_monitoring_scale(ProcessingScale::Small)
        .monitoring_is_regular(true)
        .monitoring_is_systematic(false);

    let result = assessment.validate()?;

    println!("Organization: Small Café Berlin");
    println!("Entity Type: Private Business");
    println!("Employees: 8");
    println!("Processing Activity: Security cameras (ancillary activity)");
    println!("Scale: Small");
    println!();
    println!(
        "DPO Required: {}",
        if result.dpo_required {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("Reason: {}", result.reason);

    println!("\n📋 Recommendations:");
    for (idx, rec) in result.recommendations.iter().enumerate() {
        println!("   {}. {}", idx + 1, rec);
    }

    println!("\n💡 KEY POINT:");
    println!("   Small businesses may not need a DPO if:");
    println!("   1. Processing is NOT a core activity (ancillary/supporting)");
    println!("   2. Scale is small (not large-scale monitoring)");
    println!("   3. No special categories on a large scale");
    println!("   However, voluntary DPO designation is good practice!\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 6: Member State Law Requirement (German BDSG)
fn scenario_6_member_state_law() -> Result<(), GdprError> {
    println!("## Scenario 6: Member State Law Requirement (German BDSG)\n");

    let assessment = DpoDesignationAssessment::new()
        .with_entity_type(DpoEntityType::PrivateEntity)
        .with_organization_name("German Software Company")
        .with_member_state_law_requirement(
            "German BDSG §38(1): Companies with 20+ employees regularly engaged in \
             automated processing of personal data must designate a DPO",
        );

    let result = assessment.validate()?;

    println!("Organization: German Software Company");
    println!("Entity Type: Private Company");
    println!("Location: Germany");
    println!("Employees: 25");
    println!("Processing: Automated customer data processing");
    println!();
    println!(
        "DPO Required: {}",
        if result.dpo_required {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("Legal Basis: {}", result.article);
    println!("Reason: {}", result.reason);

    println!("\n📋 Recommendations:");
    for (idx, rec) in result.recommendations.iter().enumerate() {
        println!("   {}. {}", idx + 1, rec);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 37(4): Member states may introduce STRICTER DPO requirements.");
    println!("   Germany (BDSG §38): 20+ employees with automated processing = DPO required");
    println!("   (Even if Article 37(1) criteria not met!)\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 7: Complete DPO Designation - Full Compliance Check
fn scenario_7_complete_dpo_designation() -> Result<(), GdprError> {
    println!("## Scenario 7: Complete DPO Designation (GDPR-Compliant)\n");

    let dpo = DpoDesignation::new()
        .with_name("Dr. Maria Schmidt")
        .with_email("dpo@example.com")
        .with_phone("+49 30 98765432")
        .with_qualifications(
            "CIPP/E certified (IAPP), CIPM certified, Law degree (University of Berlin), \
             12 years experience in data protection law and compliance",
        )
        .add_qualification(DpoQualification::Certification {
            certification_name: "Certified Information Privacy Professional/Europe (CIPP/E)"
                .to_string(),
            issuing_body: "International Association of Privacy Professionals (IAPP)".to_string(),
            certification_date: Utc::now(),
        })
        .add_qualification(DpoQualification::LegalQualification {
            degree: "Doctor of Law (Dr. jur.)".to_string(),
            year_obtained: 2012,
        })
        .add_qualification(DpoQualification::WorkExperience {
            years: 12,
            description: "Senior data protection consultant and legal counsel".to_string(),
        })
        .reports_to_highest_management(true)
        .is_independent(true)
        .involved_in_all_matters(true)
        .has_necessary_resources(true)
        .add_task(DpoTask::InformAndAdvise)
        .add_task(DpoTask::MonitorCompliance)
        .add_task(DpoTask::ProvideDpiaAdvice)
        .add_task(DpoTask::CooperateWithSupervisoryAuthority)
        .add_task(DpoTask::ActAsContactPoint)
        .with_designation_date(Utc::now())
        .notified_to_authority(MemberState::Germany, Utc::now())
        .with_publication(
            true,
            Some("https://example.com/privacy/dpo-contact".to_string()),
        )
        .with_notes(
            "DPO has direct access to CEO, dedicated budget for training and tools, \
             and protected from dismissal for performing DPO duties.",
        );

    let validation = dpo.validate()?;

    println!("DPO: Dr. Maria Schmidt");
    println!("Email: dpo@example.com");
    println!("Phone: +49 30 98765432");
    println!();

    println!("📜 Qualifications:");
    println!("  ✓ CIPP/E certified (IAPP)");
    println!("  ✓ Law degree (Dr. jur., 2012)");
    println!("  ✓ 12 years data protection experience");
    println!();

    println!("✅ ARTICLE 38 COMPLIANCE:");
    println!(
        "  {} Reports to highest management (Article 38(3))",
        if validation.reports_to_management {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Independent - no instructions on tasks (Article 38(3))",
        if validation.is_independent {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Provided with necessary resources (Article 38(2))",
        if validation.has_resources {
            "✓"
        } else {
            "✗"
        }
    );
    println!();

    println!("✅ ARTICLE 39 TASKS:");
    println!("  ✓ Inform and advise controller/processor (Article 39(1)(a))");
    println!("  ✓ Monitor compliance with GDPR (Article 39(1)(b))");
    println!("  ✓ Provide advice concerning DPIA (Article 39(1)(c))");
    println!("  ✓ Cooperate with supervisory authority (Article 39(1)(d))");
    println!("  ✓ Act as contact point (Article 39(1)(e))");
    println!();

    println!("✅ ARTICLE 37(6) PUBLICATION:");
    println!(
        "  {} Contact details published: https://example.com/privacy/dpo-contact",
        if validation.contact_published {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  {} Supervisory authority notified (Germany)",
        if validation.authority_notified {
            "✓"
        } else {
            "✗"
        }
    );
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

    if !validation.recommendations.is_empty() {
        println!("\n💡 RECOMMENDATIONS:");
        for rec in &validation.recommendations {
            println!("  - {}", rec);
        }
    }

    println!("\n📋 SUMMARY OF ARTICLES 37-39 REQUIREMENTS:");
    println!("┌─────────────┬──────────────────────────────────────────────────────┐");
    println!("│ Article     │ Requirement                                          │");
    println!("├─────────────┼──────────────────────────────────────────────────────┤");
    println!("│ Art. 37(1)  │ Designate DPO when required (public authority, etc.) │");
    println!("│ Art. 37(5)  │ DPO professional qualities and expert knowledge      │");
    println!("│ Art. 37(6)  │ Publish contact details, notify supervisory authority│");
    println!("│ Art. 38(1)  │ Involve DPO in all data protection issues            │");
    println!("│ Art. 38(2)  │ Provide DPO with necessary resources                 │");
    println!("│ Art. 38(3)  │ DPO reports to highest management, independent       │");
    println!("│ Art. 39(1)  │ DPO tasks: inform, monitor, advise, cooperate        │");
    println!("└─────────────┴──────────────────────────────────────────────────────┘\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}
