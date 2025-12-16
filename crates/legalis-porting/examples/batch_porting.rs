//! Batch porting example.
//!
//! This example demonstrates how to port multiple statutes at once
//! and generate comprehensive reports.

use legalis_core::{Effect, EffectType, Statute};
use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
use legalis_porting::{PortingEngine, PortingOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Batch Statute Porting Example ===\n");

    // Create jurisdictions
    let japan = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::japan());

    let usa = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
        .with_legal_system(LegalSystem::CommonLaw)
        .with_cultural_params(CulturalParams::for_country("US"));

    // Create multiple statutes
    let statutes = vec![
        Statute::new(
            "voting-rights-001",
            "投票権法 (Voting Rights Act)",
            Effect::new(EffectType::Grant, "Right to vote in elections"),
        ),
        Statute::new(
            "contract-capacity-001",
            "契約能力法 (Contract Capacity Act)",
            Effect::new(EffectType::Grant, "Ability to enter into legal contracts"),
        ),
        Statute::new(
            "property-rights-001",
            "財産権法 (Property Rights Act)",
            Effect::new(EffectType::Grant, "Right to own and dispose of property"),
        ),
        Statute::new(
            "marriage-consent-001",
            "婚姻同意法 (Marriage Consent Act)",
            Effect::new(
                EffectType::Grant,
                "Ability to marry without parental consent",
            ),
        ),
        Statute::new(
            "employment-rights-001",
            "雇用権法 (Employment Rights Act)",
            Effect::new(EffectType::Grant, "Right to full-time employment"),
        ),
    ];

    println!(
        "Porting {} statutes from Japan to United States...\n",
        statutes.len()
    );

    // Create porting engine
    let engine = PortingEngine::new(japan, usa);

    // Configure options for comprehensive analysis
    let options = PortingOptions {
        apply_cultural_params: true,
        translate_terms: true,
        generate_report: true,
        detect_conflicts: true,
        validate_semantics: true,
        ..Default::default()
    };

    // Perform batch porting
    let result = engine.batch_port(&statutes, &options).await?;

    // Display results
    println!("Batch Porting Results:");
    println!("  Total Statutes Ported: {}", result.statutes.len());
    println!("  Warnings: {}", result.warnings.len());
    println!("  Conflicts Detected: {}", result.conflicts.len());
    println!("  AI Suggestions: {}", result.ai_suggestions.len());
    println!();

    // Show each ported statute
    for (i, ported) in result.statutes.iter().enumerate() {
        println!("Statute {} of {}:", i + 1, result.statutes.len());
        println!("  Original ID: {}", ported.original_id);
        println!("  New ID: {}", ported.statute.id);
        println!("  Title: {}", ported.statute.title);
        println!("  Changes: {}", ported.changes.len());

        // Show first 3 changes
        for (j, change) in ported.changes.iter().take(3).enumerate() {
            println!(
                "    {}) {:?}: {}",
                j + 1,
                change.change_type,
                change.description
            );
        }
        if ported.changes.len() > 3 {
            println!("    ... and {} more", ported.changes.len() - 3);
        }
        println!();
    }

    // Display compatibility report
    if let Some(report) = &result.report {
        println!("Overall Compatibility Report:");
        println!(
            "  Compatibility Score: {:.1}%",
            report.compatibility_score * 100.0
        );
        println!("  Adaptations Required: {}", report.adaptations_required);
        println!("  Incompatibilities: {}", report.incompatibilities);
        println!("  Total Findings: {}", report.findings.len());
        println!();

        // Group findings by severity
        let mut by_severity = std::collections::HashMap::new();
        for finding in &report.findings {
            *by_severity
                .entry(format!("{:?}", finding.severity))
                .or_insert(0) += 1;
        }
        println!("  Findings by Severity:");
        for (severity, count) in by_severity {
            println!("    {}: {}", severity, count);
        }
        println!();

        println!("  Recommendations:");
        for rec in &report.recommendations {
            println!("    • {}", rec);
        }
        println!();
    }

    // Display conflicts
    if !result.conflicts.is_empty() {
        println!("Conflicts Summary:");
        let mut by_type = std::collections::HashMap::new();
        for conflict in &result.conflicts {
            *by_type
                .entry(format!("{:?}", conflict.conflict_type))
                .or_insert(0) += 1;
        }
        for (conflict_type, count) in by_type {
            println!("  {}: {}", conflict_type, count);
        }
        println!();
    }

    // Display semantic validation
    if let Some(validation) = &result.semantic_validation {
        println!("Semantic Validation:");
        println!(
            "  Preservation Score: {:.1}%",
            validation.preservation_score * 100.0
        );
        println!(
            "  Valid: {}",
            if validation.is_valid { "Yes" } else { "No" }
        );
        println!("  Findings: {}", validation.findings.len());
        println!();
    }

    // Display risk assessment
    if let Some(risk) = &result.risk_assessment {
        println!("Risk Assessment:");
        println!("  Risk Level: {:?}", risk.risk_level);
        println!("  Risk Score: {:.1}%", risk.risk_score * 100.0);
        println!("  Risks Identified: {}", risk.risks.len());
        for r in &risk.risks {
            println!("    • [{:?}] {}: {}", r.severity, r.category, r.description);
        }
        println!();
    }

    // Display warnings
    if !result.warnings.is_empty() {
        println!("Warnings:");
        for warning in &result.warnings {
            println!("  ⚠ {}", warning);
        }
    }

    println!("\n✓ Batch porting completed successfully!");

    Ok(())
}
