//! Compliance checking and expert review workflow example.
//!
//! This example demonstrates the full workflow from porting to compliance
//! checking and expert review.

use legalis_core::{Effect, EffectType, Statute};
use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
use legalis_porting::{
    ApprovalStatus, ExpertReview, PortingEngine, PortingOptions, ReviewRecommendation, Severity,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Compliance Workflow Example ===\n");

    // Setup jurisdictions
    let uk = Jurisdiction::new("UK", "United Kingdom", Locale::new("en").with_country("GB"))
        .with_legal_system(LegalSystem::CommonLaw)
        .with_cultural_params(CulturalParams::for_country("GB"));

    let germany = Jurisdiction::new("DE", "Germany", Locale::new("de").with_country("DE"))
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::for_country("DE"));

    // Create a statute
    let statute = Statute::new(
        "data-protection-001",
        "Data Protection and Privacy Act",
        Effect::new(EffectType::Obligation, "Protect personal data of citizens"),
    );

    println!("Original Statute: {}", statute.title);
    println!("Porting from UK to Germany\n");

    // Create porting engine
    let engine = PortingEngine::new(uk, germany);

    // Port with full validation
    let options = PortingOptions {
        apply_cultural_params: true,
        generate_report: true,
        detect_conflicts: true,
        validate_semantics: true,
        ..Default::default()
    };

    let ported = engine.port_statute(&statute, &options)?;

    println!("Step 1: Statute Ported");
    println!("  New ID: {}", ported.statute.id);
    println!("  Changes: {}", ported.changes.len());
    println!();

    // Step 2: Compliance Check
    println!("Step 2: Compliance Check");
    let compliance = engine.check_compliance(&ported);

    println!("  Status: {:?}", compliance.status);
    println!(
        "  Compliance Score: {:.1}%",
        compliance.compliance_score * 100.0
    );
    println!("  Checks Performed: {}", compliance.checks.len());
    println!("  Violations Found: {}", compliance.violations.len());
    println!();

    // Display individual checks
    println!("  Check Results:");
    for check in &compliance.checks {
        let status = if check.passed { "✓" } else { "✗" };
        println!("    {} {}: {}", status, check.name, check.description);
        if let Some(details) = &check.details {
            println!("       {}", details);
        }
    }
    println!();

    // Display violations
    if !compliance.violations.is_empty() {
        println!("  Violations:");
        for violation in &compliance.violations {
            println!(
                "    [{:?}] {}: {}",
                violation.severity, violation.violation_type, violation.description
            );
            println!("      Regulation: {}", violation.regulation);
            println!("      Remediation:");
            for rem in &violation.remediation {
                println!("        • {}", rem);
            }
        }
        println!();
    }

    // Display recommendations
    if !compliance.recommendations.is_empty() {
        println!("  Recommendations:");
        for rec in &compliance.recommendations {
            println!("    • {}", rec);
        }
        println!();
    }

    // Step 3: Submit for Expert Review
    println!("Step 3: Submit for Expert Review");
    let mut review_request = engine.submit_for_review(ported.clone());

    println!("  Request ID: {}", review_request.id);
    println!("  Status: {:?}", review_request.status);
    println!("  Submitted at: {}", review_request.submitted_at);
    println!();

    // Assign expert
    println!("Step 4: Assign Expert");
    engine.assign_expert(&mut review_request, "expert-de-001".to_string());
    println!(
        "  Assigned to: {}",
        review_request.assigned_expert.as_ref().unwrap()
    );
    println!("  Status: {:?}", review_request.status);
    println!();

    // Create expert review
    println!("Step 5: Expert Review");
    let comments = vec![
        engine.create_review_comment(
            Some("data-protection-clause".to_string()),
            "This clause needs to align with GDPR requirements".to_string(),
            Severity::Warning,
            "Regulatory Compliance".to_string(),
        ),
        engine.create_review_comment(
            Some("consent-mechanism".to_string()),
            "German law requires explicit consent, not implied".to_string(),
            Severity::Error,
            "Legal Requirement".to_string(),
        ),
        engine.create_review_comment(
            None,
            "Overall structure is sound but needs specific adjustments".to_string(),
            Severity::Info,
            "General".to_string(),
        ),
    ];

    let expert_review = ExpertReview {
        id: "review-001".to_string(),
        expert_id: "expert-de-001".to_string(),
        expert_name: "Dr. Schmidt".to_string(),
        qualifications: vec![
            "German Bar Association".to_string(),
            "Data Protection Specialist".to_string(),
            "GDPR Expert".to_string(),
        ],
        reviewed_at: chrono::Utc::now().to_rfc3339(),
        recommendation: ReviewRecommendation::ApproveWithChanges,
        comments,
        confidence: 0.92,
        concerns: vec![
            "GDPR compliance alignment".to_string(),
            "Consent mechanism clarification".to_string(),
        ],
        suggested_modifications: vec![
            "Update consent provisions to require explicit consent".to_string(),
            "Add GDPR-specific data protection officer requirements".to_string(),
            "Include right to be forgotten provisions".to_string(),
        ],
    };

    println!("  Expert: {}", expert_review.expert_name);
    println!("  Qualifications:");
    for qual in &expert_review.qualifications {
        println!("    • {}", qual);
    }
    println!("  Recommendation: {:?}", expert_review.recommendation);
    println!("  Confidence: {:.1}%", expert_review.confidence * 100.0);
    println!();

    println!("  Comments:");
    for comment in &expert_review.comments {
        let section = comment.section.as_deref().unwrap_or("General");
        println!(
            "    [{:?}] {} - {}: {}",
            comment.severity, section, comment.category, comment.text
        );
    }
    println!();

    println!("  Concerns:");
    for concern in &expert_review.concerns {
        println!("    • {}", concern);
    }
    println!();

    println!("  Suggested Modifications:");
    for modification in &expert_review.suggested_modifications {
        println!("    • {}", modification);
    }
    println!();

    // Add review to request
    engine.add_expert_review(&mut review_request, expert_review)?;

    // Step 6: Workflow Management
    println!("Step 6: Workflow Management");
    let mut workflow = engine.create_workflow(ported.statute.id.clone());

    println!("  Workflow ID: {}", workflow.id);
    println!("  State: {:?}", workflow.state);
    println!("  Pending Steps: {}", workflow.pending_steps.len());
    println!();

    // Advance through workflow steps
    println!("  Advancing through workflow:");
    while !workflow.pending_steps.is_empty() {
        let current_step = &workflow.pending_steps[0];
        println!("    - {}: {}", current_step.name, current_step.description);
        engine.advance_workflow(&mut workflow)?;
    }
    println!("  Final State: {:?}", workflow.state);
    println!();

    // Step 7: Version Control
    println!("Step 7: Version Control");
    let versioned = engine.create_versioned_statute(
        ported,
        1,
        "compliance-officer-001".to_string(),
        "Initial porting from UK to Germany with expert review".to_string(),
    );

    println!("  Version: {}", versioned.version);
    println!("  Hash: {}", versioned.hash);
    println!("  Created by: {}", versioned.created_by);
    println!("  Created at: {}", versioned.created_at);
    println!("  Change notes: {}", versioned.change_notes);
    println!();

    // Final summary
    println!("=== Workflow Summary ===");
    println!("✓ Statute ported successfully");
    println!("✓ Compliance check completed: {:?}", compliance.status);
    println!("✓ Expert review completed: {:?}", review_request.status);
    println!("✓ Workflow advanced to: {:?}", workflow.state);
    println!("✓ Version 1 created and tracked");

    // Check approval status
    let approvals_pending = workflow
        .approvals
        .iter()
        .filter(|a| a.status == ApprovalStatus::Pending)
        .count();

    if approvals_pending > 0 {
        println!("\n⚠ {} approvals still pending", approvals_pending);
    } else {
        println!("\n✓ All approvals obtained");
    }

    Ok(())
}
