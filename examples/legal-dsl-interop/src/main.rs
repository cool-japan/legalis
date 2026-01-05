//! Legal DSL Interoperability Example
//!
//! This example demonstrates how to use `legalis-interop` for converting
//! between legal domain-specific languages (DSLs).
//!
//! ## Supported Formats (27+)
//!
//! **Primary Legal DSLs:**
//! - Catala (Inria, France) - Literate programming for law
//! - L4 (Singapore) - Deontic logic (MUST/MAY/SHANT)
//! - Stipula (U. Bologna) - Smart contract agreements
//!
//! **Standards:**
//! - Akoma Ntoso (OASIS) - Legislative XML
//! - LegalRuleML - Rule-based legal reasoning
//!
//! **Business:**
//! - BPMN, DMN, CMMN - Process and decision modeling

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_interop::{LegalConverter, LegalFormat};

fn create_test_statute() -> Statute {
    Statute::new(
        "voting-eligibility",
        "Voting Eligibility Rule",
        Effect::new(EffectType::Grant, "Eligible to vote in elections"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::HasAttribute {
            key: "citizen".to_string(),
        }),
    ))
    .with_jurisdiction("INTL")
}

fn create_welfare_statute() -> Statute {
    Statute::new(
        "welfare-benefit",
        "Welfare Benefit Eligibility",
        Effect::new(EffectType::Grant, "Eligible for welfare benefits"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 25000,
        }),
    ))
    .with_jurisdiction("INTL")
}

fn print_output(output: &str, title: &str, max_lines: usize) {
    println!("   {}", "-".repeat(70));
    println!("   {}", title);
    println!("   {}", "-".repeat(70));
    let lines: Vec<&str> = output.lines().collect();
    let display_lines = lines.len().min(max_lines);
    for line in lines.iter().take(display_lines) {
        println!("   {}", line);
    }
    if lines.len() > max_lines {
        println!("   ... ({} more lines)", lines.len() - max_lines);
    }
    println!();
}

fn print_report(report: &legalis_interop::ConversionReport) {
    println!("      Confidence: {:.1}%", report.confidence * 100.0);
    println!("      Statutes converted: {}", report.statutes_converted);
    if !report.warnings.is_empty() {
        println!("      Warnings: {:?}", report.warnings);
    }
    if !report.unsupported_features.is_empty() {
        println!("      Unsupported: {:?}", report.unsupported_features);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   LEGAL DSL INTEROPERABILITY - Legalis-Interop Demo");
    println!("   法律DSL相互変換システム");
    println!("{}", "=".repeat(80));
    println!();

    let mut converter = LegalConverter::new();
    let statute = create_test_statute();
    let welfare_statute = create_welfare_statute();

    println!("Step 1: Test Statute\n");
    println!("   ID: {}", statute.id);
    println!("   Title: {}", statute.title);
    println!("   Effect: {}", statute.effect.description);
    println!("   Conditions: age >= 18 AND has 'citizen' attribute");
    println!();

    // =========================================================================
    // Export to Catala
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 2: Export to Catala Format\n");

    match converter.export(std::slice::from_ref(&statute), LegalFormat::Catala) {
        Ok((catala_output, report)) => {
            print_output(&catala_output, "Catala Output", 20);
            println!("   Conversion Report:");
            print_report(&report);
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();

    // =========================================================================
    // Export to L4
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 3: Export to L4 Format (Deontic Logic)\n");

    match converter.export(std::slice::from_ref(&statute), LegalFormat::L4) {
        Ok((l4_output, report)) => {
            print_output(&l4_output, "L4 Output (MUST/MAY/SHANT)", 15);
            println!("   Conversion Report:");
            print_report(&report);
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();

    // =========================================================================
    // Export to Stipula
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 4: Export to Stipula Format (Agreement-based)\n");

    match converter.export(std::slice::from_ref(&statute), LegalFormat::Stipula) {
        Ok((stipula_output, report)) => {
            print_output(&stipula_output, "Stipula Output", 20);
            println!("   Conversion Report:");
            print_report(&report);
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();

    // =========================================================================
    // Multi-format batch export
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 5: Batch Export to Multiple Formats\n");

    let formats = vec![
        LegalFormat::Catala,
        LegalFormat::L4,
        LegalFormat::Stipula,
        LegalFormat::AkomaNtoso,
        LegalFormat::LegalRuleML,
    ];

    match converter.batch_export(std::slice::from_ref(&welfare_statute), &formats) {
        Ok(results) => {
            println!("   Exported to {} formats:\n", results.len());
            for (format, output, report) in results {
                let lines = output.lines().count();
                println!(
                    "   {:15} - {:4} lines, {:.0}% confidence",
                    format!("{:?}", format),
                    lines,
                    report.confidence * 100.0
                );
            }
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();

    // =========================================================================
    // Auto-detection import
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 6: Auto-Detection Import\n");

    let l4_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";
    println!("   Input: \"{}\"", l4_source);

    match converter.auto_import(l4_source) {
        Ok((statutes, report)) => {
            println!("\n   Auto-detected format: {:?}", report.source_format);
            println!("   Imported {} statute(s)", statutes.len());
            for s in &statutes {
                println!("      - {} ({})", s.title, s.id);
            }
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();

    // =========================================================================
    // Roundtrip Validation
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 7: Roundtrip Validation (Semantic Preservation)\n");

    let test_l4 = "RULE TestRule WHEN age >= 21 THEN Adult MUST comply";
    println!("   Testing: Legalis → Catala → Legalis");
    println!("   Source: \"{}\"", test_l4);

    match converter.validate_roundtrip(test_l4, LegalFormat::L4, LegalFormat::Catala) {
        Ok(validation) => {
            println!("\n   Validation Results:");
            println!(
                "      Overall confidence: {:.1}%",
                validation.confidence * 100.0
            );
            println!("      Issues: {}", validation.issues.len());
            for issue in &validation.issues {
                println!("         - {}", issue);
            }
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();

    // =========================================================================
    // Summary
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("   DSL INTEROPERABILITY SUMMARY");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Supported Formats:");
    println!("   | Format       | Origin          | Key Feature                    |");
    println!("   |--------------|-----------------|--------------------------------|");
    println!("   | Catala       | Inria, France   | Literate programming for law   |");
    println!("   | L4           | Singapore       | Deontic logic (MUST/MAY/SHANT) |");
    println!("   | Stipula      | U. Bologna      | Agreement & state machines     |");
    println!("   | Akoma Ntoso  | OASIS           | Legislative XML standard       |");
    println!("   | LegalRuleML  | OASIS           | Rule interchange format        |");
    println!("   | BPMN/DMN     | OMG             | Process/decision modeling      |");
    println!();
    println!("   Conversion Quality Metrics:");
    println!("   - confidence: 0.0-1.0 (semantic preservation score)");
    println!("   - is_lossless(): true if confidence >= 1.0 and no warnings");
    println!("   - is_high_quality(): true if confidence >= 0.8");
    println!();
    println!("   Use Cases:");
    println!("   - Import existing Catala/L4 codebases into Legalis");
    println!("   - Export Legalis statutes for external tools");
    println!("   - Validate semantic equivalence across formats");
    println!("   - Bridge between legal DSL ecosystems");
    println!();

    Ok(())
}
