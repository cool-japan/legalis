//! Choice of Law - Multi-State Tort Example
//!
//! This example demonstrates how US choice of law rules determine which
//! state's law applies when a dispute involves multiple states.
//!
//! Run with: `cargo run --example choice_of_law_multistate`

use legalis_us::choice_of_law::{
    ChoiceOfLawApproach, ContactingFactor, USChoiceOfLawAnalyzer, USChoiceOfLawFactors,
};

fn main() {
    println!("=== US Choice of Law: Multi-State Tort Scenario ===\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  HYPOTHETICAL CASE: Car Accident Across State Lines       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("Facts:");
    println!("  • Alice (California resident) driving through Nevada");
    println!("  • Bob (Texas resident) on vacation in Nevada");
    println!("  • Car manufactured in Michigan, sold in Texas");
    println!("  • Accident occurs in Nevada");
    println!("  • Lawsuit filed in California federal court");
    println!("  • Issue: Damages cap - Nevada has $350k cap, California has none\n");

    println!("Connecting Factors:");
    println!("  ✓ Place of injury: Nevada");
    println!("  ✓ Place of conduct: Nevada");
    println!("  ✓ Plaintiff domicile: California");
    println!("  ✓ Defendant domicile: Texas");
    println!("  ✓ Forum: California\n");

    // Set up choice of law factors
    let factors = USChoiceOfLawFactors::new()
        .with_factor(ContactingFactor::PlaceOfInjury("Nevada".to_string()))
        .with_factor(ContactingFactor::PlaceOfConduct("Nevada".to_string()))
        .with_factor(ContactingFactor::PlaintiffDomicile(
            "California".to_string(),
        ))
        .with_factor(ContactingFactor::DefendantDomicile("Texas".to_string()))
        .with_factor(ContactingFactor::ForumState("California".to_string()));

    println!("{}", "=".repeat(60));
    println!("\n=== APPROACH 1: Restatement (First) - Traditional ===\n");

    let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::RestatementFirst);
    let result = analyzer.analyze_tort(&factors);

    println!("Chosen Law: {}", result.applicable_law);
    println!("Approach: {:?}", result.approach);
    println!("Confidence: {:.0}%", result.confidence * 100.0);
    println!("\nExplanation: {}", result.explanation);
    println!("\nRule: Lex loci delicti (place of wrong)");
    println!("Result: Apply NEVADA law (damages capped at $350k)");
    println!("\nCriticism:");
    println!("  ✗ Mechanical rule ignores policy considerations");
    println!("  ✗ Nevada has minimal connection to parties");
    println!("  ✗ Only 6 states still use this approach\n");

    println!("{}", "=".repeat(60));
    println!("\n=== APPROACH 2: Restatement (Second) - Modern Majority ===\n");

    let analyzer2 = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::RestatementSecond);
    let result2 = analyzer2.analyze_tort(&factors);

    println!("Chosen Law: {}", result2.applicable_law);
    println!("Approach: {:?}", result2.approach);
    println!("Confidence: {:.0}%", result2.confidence * 100.0);
    println!("\nExplanation: {}", result2.explanation);

    println!("\nRestatement (Second) § 145 Factors:");
    println!("  (a) Place of injury");
    println!("  (b) Place of conduct causing injury");
    println!("  (c) Domicile, residence, nationality of parties");
    println!("  (d) Place where relationship is centered");

    println!("\nStatus: Majority rule (44 states follow)\n");

    println!("{}", "=".repeat(60));
    println!("\n=== SUMMARY: Same Facts, Different Outcomes ===\n");

    println!("┌──────────────────────┬────────────────┬──────────────────┐");
    println!("│ Approach             │ Law Applied    │ Damages Cap?     │");
    println!("├──────────────────────┼────────────────┼──────────────────┤");
    println!("│ Restatement (First)  │ Nevada         │ YES ($350k cap)  │");
    println!("│ Restatement (Second) │ Varies         │ Depends on facts │");
    println!("│ Interest Analysis    │ California     │ NO (unlimited)   │");
    println!("└──────────────────────┴────────────────┴──────────────────┘");

    println!("\nPractical Impact:");
    println!("  • Same accident, different outcomes depending on forum");
    println!("  • Forum shopping incentive");
    println!("  • Most states (44) use Restatement (Second)");
    println!("  • Only 6 states still use traditional approach\n");

    println!("✓ Example completed successfully");
}
