//! Battery Tort Example - Restatement (Second) of Torts § 158
//!
//! This example demonstrates the application of the battery tort principle
//! from the Restatement (Second) of Torts § 158.
//!
//! Run with: `cargo run --example tort_battery_example`

use legalis_us::restatement::section_158_battery;

fn main() {
    println!("=== US Common Law: Battery Tort (Restatement § 158) ===\n");

    // Get the Restatement section for battery
    let battery_rule = section_158_battery();

    println!("Case Rule: {}", battery_rule.name);
    println!("Citation: Restatement (Second) of Torts § 158 (1965)");
    println!("Effect: {}", battery_rule.effect.description);

    println!("\n=== Elements of Battery ===");
    println!("1. Voluntary act by defendant");
    println!("2. Intent to cause harmful or offensive contact");
    println!("3. Harmful contact results");
    println!("4. Causation between act and contact");

    println!("\n=== Famous Battery Cases ===");
    println!("• Vosburg v. Putney, 80 Wis. 523 (1891)");
    println!("  - School child kicked another child's leg");
    println!("  - Held: Liable even though kick seemed minor");
    println!("  - Rule: Intent to touch + harmful result = battery");

    println!("\n• Garratt v. Dailey, 279 P.2d 1091 (1955)");
    println!("  - 5-year-old pulled chair away from elderly woman");
    println!("  - Held: Battery if child knew with substantial certainty contact would result");
    println!("  - Rule: Intent includes substantial certainty of result");

    println!("\n=== Comparison with Civil Law Systems ===");
    println!("┌────────────┬──────────────────────────────────────────┐");
    println!("│ System     │ Tort Principle                           │");
    println!("├────────────┼──────────────────────────────────────────┤");
    println!("│ US         │ Restatement § 158 (intent required)      │");
    println!("│ Japan      │ 民法709条 (intent OR negligence)          │");
    println!("│ Germany    │ BGB §823 (body as protected interest)    │");
    println!("│ France     │ Code civil 1240 (general fault)          │");
    println!("└────────────┴──────────────────────────────────────────┘");

    println!("\n=== Key Difference: Common Law vs Civil Law ===");
    println!("Common Law (US):");
    println!("  ✓ Battery = Specific tort requiring INTENT");
    println!("  ✓ Negligent touching → separate tort (negligence)");
    println!("  ✓ Rules from case precedents, synthesized in Restatement");

    println!("\nCivil Law (Japan, Germany, France):");
    println!("  ✓ Single general tort provision in code");
    println!("  ✓ Covers both intentional AND negligent harms");
    println!("  ✓ Statutory text is primary source");

    println!("\n=== Hypothetical Scenario ===");
    println!("Scenario: Alex taps Beth on shoulder to get attention.");
    println!("Analysis:");
    println!("  1. Voluntary act? YES (Alex intentionally moved arm)");
    println!("  2. Intent to contact? YES (purpose to tap shoulder)");
    println!("  3. Harmful/offensive? DEPENDS");
    println!("     - Light tap in social context: NOT offensive");
    println!("     - Aggressive grab: OFFENSIVE");
    println!("  4. Result: Battery only if contact was offensive to reasonable person");

    println!("\n✓ Example completed successfully");
}
