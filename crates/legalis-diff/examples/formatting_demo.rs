//! Example demonstrating the various formatting and analysis features.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_diff::{
    analysis::{analyze_changes, summarize_compatibility},
    diff,
    formats::{
        DiffFormatter, HtmlFormatter, JsonFormatter, MarkdownFormatter, SideBySideFormat,
        SideBySideFormatter,
    },
};

fn main() {
    // Create two versions of a statute
    let old_statute = Statute::new(
        "housing-subsidy-2023",
        "Housing Subsidy Program",
        Effect::new(EffectType::Grant, "Monthly housing subsidy of 50,000 yen"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 20,
    })
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessOrEqual,
        value: 3_000_000,
    });

    let new_statute = Statute::new(
        "housing-subsidy-2023",
        "Housing Subsidy Program (Revised)",
        Effect::new(EffectType::Grant, "Monthly housing subsidy of 60,000 yen"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18, // Relaxed from 20 to 18
    })
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessOrEqual,
        value: 4_000_000, // Relaxed from 3M to 4M
    })
    .with_discretion("Consider special circumstances for families with children");

    // Generate diff
    let statute_diff = diff(&old_statute, &new_statute).expect("Failed to generate diff");

    println!("=== BASIC DIFF SUMMARY ===\n");
    println!("{}", legalis_diff::summarize(&statute_diff));

    // JSON format
    println!("\n=== JSON FORMAT ===\n");
    let json_formatter = JsonFormatter::new().with_pretty(true);
    println!("{}", json_formatter.format(&statute_diff));

    // Markdown format
    println!("\n=== MARKDOWN FORMAT ===\n");
    let md_formatter = MarkdownFormatter::new();
    println!("{}", md_formatter.format(&statute_diff));

    // HTML format (shortened for display)
    println!("\n=== HTML FORMAT ===\n");
    let html_formatter = HtmlFormatter::new();
    let html_output = html_formatter.format(&statute_diff);
    println!("HTML output generated ({} bytes)", html_output.len());
    println!("First 500 characters:");
    println!("{}", &html_output[..html_output.len().min(500)]);

    // Side-by-side comparison
    println!("\n=== SIDE-BY-SIDE COMPARISON (Markdown) ===\n");
    let sbs_formatter = SideBySideFormatter::new(SideBySideFormat::Markdown);
    println!("{}", sbs_formatter.format(&statute_diff));

    // Advanced analysis
    println!("\n=== ADVANCED ANALYSIS ===\n");
    let analyses = analyze_changes(&statute_diff);
    for analysis in &analyses {
        println!(
            "Change: {:?} - Compatibility: {:?}",
            analysis.change.target, analysis.compatibility
        );
        println!("  Relaxes: {}", analysis.relaxes_conditions);
        println!("  Tightens: {}", analysis.tightens_conditions);
        println!("  Explanation: {}\n", analysis.explanation);
    }

    // Compatibility summary
    println!("=== COMPATIBILITY SUMMARY ===\n");
    let compat_summary = summarize_compatibility(&analyses);
    println!("Total changes: {}", compat_summary.total_changes);
    println!("Breaking changes: {}", compat_summary.breaking_changes);
    println!(
        "Backward compatible: {}",
        compat_summary.backward_compatible_changes
    );
    println!(
        "Forward compatible: {}",
        compat_summary.forward_compatible_changes
    );
    println!("Non-breaking: {}", compat_summary.non_breaking_changes);
    println!(
        "Overall compatibility: {:?}",
        compat_summary.overall_compatibility
    );
}
