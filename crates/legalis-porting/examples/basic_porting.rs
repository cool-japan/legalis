//! Basic statute porting example.
//!
//! This example demonstrates how to port a statute from one jurisdiction to another
//! with cultural parameter adaptation.

use legalis_core::{Effect, EffectType, Statute};
use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
use legalis_porting::{PortingEngine, PortingOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Statute Porting Example ===\n");

    // Create source jurisdiction (Japan)
    let japan = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::japan());

    // Create target jurisdiction (United States)
    let usa = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
        .with_legal_system(LegalSystem::CommonLaw)
        .with_cultural_params(CulturalParams::for_country("US"));

    // Create a statute about adult rights
    let statute = Statute::new(
        "adult-rights-001",
        "成人権利法 (Adult Rights Act)",
        Effect::new(
            EffectType::Grant,
            "Full legal capacity upon reaching adulthood",
        ),
    );

    println!("Original Statute:");
    println!("  ID: {}", statute.id);
    println!("  Title: {}", statute.title);
    println!();

    // Create porting engine
    let engine = PortingEngine::new(japan, usa);

    // Configure porting options
    let options = PortingOptions {
        apply_cultural_params: true,
        translate_terms: true,
        generate_report: true,
        detect_conflicts: true,
        validate_semantics: true,
        ..Default::default()
    };

    // Port the statute
    println!("Porting from Japan to United States...\n");
    let ported = engine.port_statute(&statute, &options)?;

    // Display results
    println!("Ported Statute:");
    println!("  ID: {}", ported.statute.id);
    println!("  Title: {}", ported.statute.title);
    println!(
        "  Locale: {} ({})",
        ported.locale.language,
        ported.locale.country.as_deref().unwrap_or("")
    );
    println!();

    // Show changes made
    println!("Changes made during porting:");
    for (i, change) in ported.changes.iter().enumerate() {
        println!(
            "  {}. {:?}: {}",
            i + 1,
            change.change_type,
            change.description
        );
        if let Some(original) = &change.original {
            println!("     Original: {}", original);
        }
        if let Some(adapted) = &change.adapted {
            println!("     Adapted: {}", adapted);
        }
        println!("     Reason: {}", change.reason);
        println!();
    }

    // Generate and display compatibility report
    let report = engine.generate_report(std::slice::from_ref(&statute));
    println!("Compatibility Report:");
    println!("  Score: {:.1}%", report.compatibility_score * 100.0);
    println!("  Adaptations Required: {}", report.adaptations_required);
    println!("  Incompatibilities: {}", report.incompatibilities);
    println!();

    println!("Findings:");
    for finding in &report.findings {
        println!(
            "  [{:?}] {} - {}",
            finding.severity, finding.category, finding.description
        );
    }
    println!();

    println!("Recommendations:");
    for rec in &report.recommendations {
        println!("  • {}", rec);
    }

    // Detect conflicts
    let conflicts = engine.detect_conflicts(&statute);
    if !conflicts.is_empty() {
        println!("\nConflicts Detected:");
        for conflict in &conflicts {
            println!(
                "  [{:?}] {:?}: {}",
                conflict.severity, conflict.conflict_type, conflict.description
            );
            println!("    Resolutions:");
            for resolution in &conflict.resolutions {
                println!("      - {}", resolution);
            }
        }
    }

    // Validate semantics
    let validation = engine.validate_semantics(&statute, &ported);
    println!("\nSemantic Validation:");
    println!(
        "  Preservation Score: {:.1}%",
        validation.preservation_score * 100.0
    );
    println!(
        "  Valid: {}",
        if validation.is_valid { "Yes" } else { "No" }
    );
    if !validation.findings.is_empty() {
        println!("  Findings:");
        for finding in &validation.findings {
            println!(
                "    [{:?}] {}: {}",
                finding.severity, finding.description, finding.impact
            );
        }
    }

    // Risk assessment
    let risk = engine.assess_risks(&ported);
    println!("\nRisk Assessment:");
    println!("  Risk Level: {:?}", risk.risk_level);
    println!("  Risk Score: {:.1}%", risk.risk_score * 100.0);
    println!("  Risks Identified:");
    for r in &risk.risks {
        println!("    [{:?}] {} - {}", r.severity, r.category, r.description);
        println!(
            "      Likelihood: {:.1}%, Impact: {:.1}%",
            r.likelihood * 100.0,
            r.impact * 100.0
        );
    }
    println!("  Recommended Mitigations:");
    for mitigation in &risk.mitigations {
        println!("    • {}", mitigation);
    }

    Ok(())
}
