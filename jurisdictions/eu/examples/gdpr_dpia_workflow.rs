//! # Data Protection Impact Assessment (DPIA) Workflow
//!
//! This example demonstrates a complete DPIA workflow under GDPR Article 35.
//!
//! ## Scenario
//!
//! **Company**: SmartCity Analytics GmbH (Germany)
//! - Deploying facial recognition system in public transportation
//! - Large-scale biometric data processing
//! - Systematic monitoring of public spaces
//!
//! ## GDPR Articles
//!
//! - **Article 35(1)**: DPIA requirement when high risk
//! - **Article 35(3)**: Mandatory DPIA triggers
//! - **Article 35(7)**: DPIA minimum content
//! - **Article 36**: Prior consultation with supervisory authority

use chrono::Utc;
use legalis_core::LegalResult;
use legalis_eu::gdpr::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      DPIA Workflow: Facial Recognition in Public Transit    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════
    // STEP 1: Determine if DPIA is Required
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━ STEP 1: DPIA Requirement Assessment ━━━\n");

    println!("Article 35(3) Triggers:");
    println!("  ✅ (b) Large-scale special categories → YES (biometric data)");
    println!("  ✅ (c) Systematic monitoring → YES (public spaces)");
    println!("\n❗ DPIA is MANDATORY\n");

    // ═══════════════════════════════════════════════════════════════════
    // STEP 2: Conduct DPIA
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━ STEP 2: Conduct DPIA ━━━\n");

    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("SmartCity Analytics GmbH")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "Real-time facial recognition in 50 metro stations. \
             Biometric templates extracted and compared against watchlist.",
        )
        .with_purpose("Security threat identification")
        .add_data_category(PersonalDataCategory::Special(
            SpecialCategory::BiometricData,
        ))
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Storage)
        .add_operation(ProcessingOperation::Use)
        .add_trigger(DpiaTrigger::LargeScaleSpecialCategories {
            categories: vec![SpecialCategory::BiometricData],
            scale: 500_000,
        })
        .add_trigger(DpiaTrigger::SystematicMonitoring {
            monitoring_type: "Continuous biometric surveillance".to_string(),
            large_scale: true,
            scope: "Public transportation network".to_string(),
        })
        .with_necessity_assessment(
            "Necessary for rapid security threat identification. \
             Manual surveillance insufficient for volume/speed.",
        )
        .with_proportionality_assessment(
            "Data minimization: templates only (not raw images). \
             Purpose limitation: security use only. \
             Storage limitation: 30-day deletion.",
        )
        // Risk 1: Discrimination
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Facial recognition may exhibit bias against minorities, \
                          leading to false positives and discriminatory treatment."
                .to_string(),
        })
        // Risk 2: Unauthorized access
        .add_risk(RiskAssessment {
            risk_type: RiskType::UnauthorizedAccess,
            likelihood: Likelihood::Medium,
            severity: Severity::High,
            description: "Biometric database breach could lead to identity theft. \
                          Biometric data is immutable."
                .to_string(),
        })
        // Risk 3: Function creep
        .add_risk(RiskAssessment {
            risk_type: RiskType::Other("Function creep".to_string()),
            likelihood: Likelihood::High,
            severity: Severity::Medium,
            description: "System may expand beyond security to other purposes \
                          (demographics, law enforcement)."
                .to_string(),
        })
        // Mitigation 1: Algorithm fairness
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Independent audit of recognition accuracy across ethnic groups. \
                      98% parity required. Human review of all matches."
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        // Mitigation 2: Data breach protection
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::UnauthorizedAccess,
            measure: "End-to-end encryption. Air-gapped database. ISO 27001 certified. \
                      Quarterly penetration testing."
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        // Mitigation 3: Purpose limitation
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Other("Function creep".to_string()),
            measure: "Legislative constraint via city ordinance. Independent oversight board. \
                      Technical enforcement (separate systems)."
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .with_dpo_consulted(true);

    println!("✅ DPIA Components:");
    println!("   Processing: Facial recognition with biometric matching");
    println!("   Data: Biometric templates, location, timestamps");
    println!("   Scale: 50 stations, ~500,000 passengers/day");
    println!("   Risks Identified: 3 major risks");
    println!("   Mitigations: 3 comprehensive safeguards");
    println!("   DPO Consulted: Yes\n");

    // ═══════════════════════════════════════════════════════════════════
    // STEP 3: Validate DPIA
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━ STEP 3: DPIA Validation ━━━\n");

    match dpia.validate() {
        Ok(validation) => {
            let dpia_complete =
                matches!(validation.dpia_complete, LegalResult::Deterministic(true));

            if dpia_complete {
                println!("✅ DPIA Compliant with Article 35\n");

                println!("Article 35(7) Requirements:");
                println!("  ✅ Processing description: Complete");
                println!("  ✅ Necessity/proportionality: Assessed");
                println!("  ✅ Risks identified and mitigated: 3 risks");
                println!("  ✅ Safeguards: High-effectiveness mitigations\n");

                println!("Risk Assessment:");
                println!(
                    "  Residual Risk Level: {:?}",
                    validation.residual_risk_level
                );
                println!(
                    "  Mandatory DPIA Trigger: {}",
                    validation.has_mandatory_trigger
                );

                // ═══════════════════════════════════════════════════════════════════
                // STEP 4: Article 36 Prior Consultation
                // ═══════════════════════════════════════════════════════════════════
                println!("\n━━━ STEP 4: Article 36 Prior Consultation ━━━\n");

                if validation.prior_consultation_required {
                    println!("❗ PRIOR CONSULTATION REQUIRED (Article 36)");
                    println!(
                        "\n   Residual risk level: {:?}",
                        validation.residual_risk_level
                    );
                    println!("\n   Action Required:");
                    println!("     1. Submit DPIA to supervisory authority (BfDI)");
                    println!("     2. Await authorization (up to 8 weeks, Article 36(2))");
                    println!("     3. Authority may require additional safeguards");
                    println!("\n   ⚠️  Processing MUST NOT commence until consultation complete\n");
                } else {
                    println!("✅ No Prior Consultation Required");
                    println!(
                        "   Residual risk level: {:?}",
                        validation.residual_risk_level
                    );
                    println!("   Processing may commence (subject to GDPR compliance)\n");
                }

                if !validation.recommendations.is_empty() {
                    println!("Recommendations:");
                    for rec in &validation.recommendations {
                        println!("  • {}", rec);
                    }
                    println!();
                }
            } else {
                println!("❌ DPIA Non-Compliant\n");
                println!("The DPIA does not meet Article 35(7) requirements.\n");

                if !validation.recommendations.is_empty() {
                    println!("Required Actions:");
                    for rec in &validation.recommendations {
                        println!("  • {}", rec);
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            println!("❌ DPIA Error: {}\n", e);
            return;
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Summary
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              DPIA WORKFLOW COMPLETE                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("SmartCity Analytics GmbH - DPIA Summary\n");

    println!("Workflow Steps:");
    println!("  ✅ DPIA requirement determined (Art. 35(3))");
    println!("  ✅ Processing described (Art. 35(7)(a))");
    println!("  ✅ Necessity & proportionality assessed (Art. 35(7)(b))");
    println!("  ✅ Risks identified and assessed (Art. 35(7)(c))");
    println!("  ✅ Safeguards implemented (Art. 35(7)(d))");
    println!("  ✅ DPO consulted (Art. 35(2))");
    println!("  ⚠️  Prior consultation required (Art. 36)\n");

    println!("Key Findings:");
    println!("  • High-risk: Biometric data + systematic monitoring");
    println!("  • 3 significant risks identified");
    println!("  • Strong technical/organizational safeguards");
    println!("  • Supervisory authority consultation mandatory\n");

    println!("Next Steps:");
    println!("  1. Submit DPIA to BfDI");
    println!("  2. Implement additional safeguards as required");
    println!("  3. Obtain authorization before processing");
    println!("  4. Review DPIA regularly (Art. 35(11))\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
