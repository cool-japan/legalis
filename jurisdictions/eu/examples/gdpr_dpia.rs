//! GDPR Data Protection Impact Assessment (DPIA) Examples
//!
//! This example demonstrates how to conduct DPIAs under Article 35 GDPR.
//!
//! ## Scenarios Covered
//!
//! 1. AI-Powered Recruitment (Automated Decision-Making)
//! 2. Hospital Patient Records (Large-Scale Special Categories)
//! 3. Public CCTV Facial Recognition (Systematic Monitoring)
//! 4. Low-Risk Processing (No DPIA Required)
//! 5. High Residual Risk (Prior Consultation Required)

use chrono::Utc;
use legalis_eu::gdpr::dpia::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{PersonalDataCategory, ProcessingOperation, SpecialCategory};

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Data Protection Impact Assessment Examples ===\n");

    scenario_1_ai_recruitment()?;
    scenario_2_hospital_records()?;
    scenario_3_facial_recognition()?;
    scenario_4_low_risk();
    scenario_5_high_residual_risk()?;

    println!("\n✅ All DPIA scenarios completed");
    Ok(())
}

/// Scenario 1: AI-Powered Recruitment Screening
///
/// Trigger: Article 35(3)(a) - Systematic and extensive automated decision-making
fn scenario_1_ai_recruitment() -> Result<(), GdprError> {
    println!("## Scenario 1: AI-Powered Recruitment Screening\n");

    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("TechCorp Inc")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "AI-powered automated screening of job applications using machine learning \
             algorithms to evaluate candidates based on CV content, skills matching, \
             and predictive performance modeling",
        )
        .with_purpose("Automated candidate evaluation and ranking for recruitment efficiency")
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .add_data_category(PersonalDataCategory::Regular(
            "employment history".to_string(),
        ))
        .add_data_category(PersonalDataCategory::Regular("education".to_string()))
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Use)
        .add_operation(ProcessingOperation::Disclosure)
        .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
            produces_legal_effects: true, // Affects job prospects
            systematic: true,
            extensive: true,
        })
        .add_trigger(DpiaTrigger::ProfilingOrScoring {
            profiling_type: "Candidate scoring algorithm".to_string(),
            significant_effects: true,
        })
        .with_necessity_assessment(
            "Processing 10,000+ applications per month makes manual review impractical. \
             Automated screening necessary to handle volume while maintaining quality.",
        )
        .with_proportionality_assessment(
            "Measures are proportionate: (1) only relevant data used, (2) human review \
             before final decision, (3) right to object provided, (4) limited retention",
        )
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "AI algorithms may exhibit bias against protected characteristics \
                         (age, gender, ethnicity) if training data contains historical biases"
                .to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::RightsViolation,
            likelihood: Likelihood::Medium,
            severity: Severity::High,
            description: "Candidates may not understand how decisions are made, \
                         limiting ability to challenge unfair outcomes"
                .to_string(),
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Quarterly algorithmic fairness audits testing for bias across \
                     protected characteristics, with retraining if bias detected"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::RightsViolation,
            measure: "Provide meaningful information about decision logic and allow \
                     candidates to request human review (Article 22 compliance)"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .with_dpo_consulted(true)
        .with_dpo_opinion("DPO recommends implementing bias audits and human oversight")
        .with_data_subjects_consulted(false);

    let validation = dpia.validate()?;

    println!("Controller: TechCorp Inc");
    println!("Processing: AI recruitment screening");
    println!("Triggers: Automated decision-making (Article 35(3)(a))");
    println!("DPIA Complete: {:?}", validation.dpia_complete);
    println!("Residual Risk: {:?}", validation.residual_risk_level);
    println!(
        "Prior Consultation Required: {}",
        validation.prior_consultation_required
    );
    println!(
        "Processing May Proceed: {:?}",
        validation.processing_may_proceed
    );

    if !validation.recommendations.is_empty() {
        println!("\n⚠️ Recommendations:");
        for rec in &validation.recommendations {
            println!("   - {}", rec);
        }
    }

    println!("\n✅ DPIA complete - Medium residual risk with effective mitigations\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Hospital Patient Records System
///
/// Trigger: Article 35(3)(b) - Large-scale processing of special categories (health data)
fn scenario_2_hospital_records() -> Result<(), GdprError> {
    println!("## Scenario 2: Hospital Electronic Health Records System\n");

    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("Metropolitan Hospital")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "Centralized electronic health records system for 50,000+ patients, \
             storing comprehensive medical histories, diagnoses, treatments, and test results",
        )
        .with_purpose("Patient care coordination and medical record management")
        .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
        .add_data_category(PersonalDataCategory::Regular(
            "contact information".to_string(),
        ))
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Storage)
        .add_operation(ProcessingOperation::Dissemination)
        .add_trigger(DpiaTrigger::LargeScaleSpecialCategories {
            categories: vec![SpecialCategory::HealthData],
            scale: 50_000, // 50,000 patients
        })
        .with_necessity_assessment(
            "Electronic health records essential for modern healthcare delivery, \
             enabling coordinated care across departments and specialists",
        )
        .with_proportionality_assessment(
            "Proportionate: (1) access controls limit viewing to treating physicians, \
             (2) audit logs track all access, (3) data minimization principles applied",
        )
        .add_risk(RiskAssessment {
            risk_type: RiskType::UnauthorizedAccess,
            likelihood: Likelihood::Medium,
            severity: Severity::High,
            description: "Unauthorized access to sensitive health data could cause \
                         severe reputational and psychological harm to patients"
                .to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::DataLoss,
            likelihood: Likelihood::Low,
            severity: Severity::High,
            description: "System failure or ransomware attack could prevent access \
                         to critical patient records, endangering lives"
                .to_string(),
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::UnauthorizedAccess,
            measure: "Multi-factor authentication, role-based access controls, \
                     automatic session timeouts, comprehensive audit logging"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::DataLoss,
            measure: "Real-time replication to secondary datacenter, hourly backups, \
                     disaster recovery plan with 4-hour RTO"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .with_dpo_consulted(true)
        .with_dpo_opinion("DPO approves with recommendation for annual security audits")
        .with_data_subjects_consulted(true);

    let validation = dpia.validate()?;

    println!("Controller: Metropolitan Hospital");
    println!("Processing: Electronic health records (50,000 patients)");
    println!("Triggers: Large-scale special categories (Article 35(3)(b))");
    println!("DPIA Complete: {:?}", validation.dpia_complete);
    println!("Residual Risk: {:?}", validation.residual_risk_level);
    println!(
        "Prior Consultation Required: {}",
        validation.prior_consultation_required
    );

    println!("\n✅ DPIA complete - Low residual risk with strong technical measures\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Public CCTV with Facial Recognition
///
/// Trigger: Article 35(3)(c) - Systematic monitoring of publicly accessible areas
fn scenario_3_facial_recognition() -> Result<(), GdprError> {
    println!("## Scenario 3: Public CCTV with Facial Recognition\n");

    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("City Council")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "Deployment of facial recognition technology across 200 CCTV cameras \
             in city center to identify individuals against watchlist of suspects",
        )
        .with_purpose("Public safety and crime prevention")
        .add_data_category(PersonalDataCategory::Special(
            SpecialCategory::BiometricData,
        ))
        .add_data_category(PersonalDataCategory::Regular("location data".to_string()))
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Use)
        .add_trigger(DpiaTrigger::SystematicMonitoring {
            monitoring_type: "CCTV with real-time facial recognition".to_string(),
            large_scale: true,
            scope: "City center covering 10,000+ daily visitors".to_string(),
        })
        .add_trigger(DpiaTrigger::NewTechnology {
            technology: "Live facial recognition matching".to_string(),
            risk_rationale: "Continuous biometric surveillance of public spaces \
                            creates unprecedented tracking capabilities"
                .to_string(),
        })
        .with_necessity_assessment(
            "Necessary for public safety: historic crime hotspot with limited police resources",
        )
        .with_proportionality_assessment(
            "Proportionality QUESTIONABLE: less intrusive alternatives (increased patrols, \
             non-biometric CCTV) not fully explored",
        )
        .add_risk(RiskAssessment {
            risk_type: RiskType::RightsViolation,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Mass surveillance violates right to privacy and freedom of movement. \
                         Chilling effect on public assembly and expression"
                .to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Facial recognition has documented bias against minorities, \
                         leading to disproportionate false positives"
                .to_string(),
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::RightsViolation,
            measure: "Public signage about monitoring, data retention limited to 30 days, \
                     independent oversight board"
                .to_string(),
            effectiveness: Effectiveness::Low, // Insufficient for such high risk
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Accuracy testing across demographic groups before deployment".to_string(),
            effectiveness: Effectiveness::Medium,
        })
        .with_dpo_consulted(true)
        .with_dpo_opinion("DPO strongly recommends against deployment - risks too high")
        .with_data_subjects_consulted(true);

    let validation = dpia.validate()?;

    println!("Controller: City Council");
    println!("Processing: Facial recognition in public spaces");
    println!("Triggers: Systematic monitoring (Article 35(3)(c))");
    println!("DPIA Complete: {:?}", validation.dpia_complete);
    println!("Residual Risk: {:?}", validation.residual_risk_level);
    println!(
        "Prior Consultation Required: {}",
        validation.prior_consultation_required
    );
    println!(
        "Processing May Proceed: {:?}",
        validation.processing_may_proceed
    );

    println!("\n⚠️ ARTICLE 36 PRIOR CONSULTATION REQUIRED:");
    println!("   Residual risk remains HIGH despite mitigations");
    println!("   Must consult supervisory authority before deployment");
    println!("   Authority may:");
    println!("   - Prohibit processing entirely");
    println!("   - Require additional safeguards");
    println!("   - Impose strict conditions");

    println!("\n⚠️ High-risk processing - judicial discretion required\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: Low-Risk Processing (No DPIA Required)
///
/// Not all processing requires DPIA - this shows a low-risk scenario
fn scenario_4_low_risk() {
    println!("## Scenario 4: Newsletter Subscription (Low Risk - No DPIA Required)\n");

    println!("Processing: Email newsletter subscriptions");
    println!("Data: Email addresses, names (optional)");
    println!("Scale: 5,000 subscribers");
    println!("Purpose: Marketing communications");

    println!("\n✅ NO DPIA REQUIRED:");
    println!("   - Not systematic/extensive automated processing");
    println!("   - Not large-scale special categories");
    println!("   - Not systematic public monitoring");
    println!("   - Limited scope and low risk to rights");

    println!("\nNote: Article 35(1) requires DPIA only when processing is");
    println!("      'likely to result in a high risk'");

    println!("\n✅ Proceed without DPIA (basic compliance measures sufficient)\n");
    println!("{}", "=".repeat(70));
    println!();
}

/// Scenario 5: Incomplete Mitigation - High Residual Risk
///
/// Shows what happens when mitigations are insufficient
fn scenario_5_high_residual_risk() -> Result<(), GdprError> {
    println!("## Scenario 5: High Residual Risk Despite Mitigations\n");

    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("FinTech Startup")
        .with_processing_description("Credit scoring using alternative data sources")
        .with_purpose("Loan approval automation")
        .with_necessity_assessment("Required for business model")
        .with_proportionality_assessment("Proportionate to risk")
        .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
            produces_legal_effects: true,
            systematic: true,
            extensive: true,
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Alternative data sources may introduce prohibited discrimination"
                .to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::FinancialLoss,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Incorrect credit decisions cause financial harm".to_string(),
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Annual fairness review".to_string(),
            effectiveness: Effectiveness::Low, // Insufficient - should be continuous
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::FinancialLoss,
            measure: "Manual review of 10% of decisions".to_string(),
            effectiveness: Effectiveness::Low, // Insufficient - should be higher %
        });

    let validation = dpia.validate()?;

    println!("Controller: FinTech Startup");
    println!("Processing: Automated credit scoring");
    println!("Residual Risk: {:?}", validation.residual_risk_level);
    println!(
        "Prior Consultation Required: {}",
        validation.prior_consultation_required
    );

    println!("\n❌ INSUFFICIENT MITIGATIONS:");
    println!("   Two HIGH risks identified");
    println!("   Mitigations have LOW effectiveness");
    println!("   Residual risk remains HIGH");

    println!("\n⚠️ RECOMMENDATIONS:");
    for rec in &validation.recommendations {
        println!("   - {}", rec);
    }

    println!("\n⚠️ Article 36(1) PRIOR CONSULTATION REQUIRED before processing begins\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}
