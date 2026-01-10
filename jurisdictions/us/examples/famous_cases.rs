//! Famous Common Law Cases
//!
//! This example demonstrates landmark tort cases that shaped American law.
//!
//! Run with: `cargo run --example famous_cases`

use legalis_us::cases::{
    donoghue_v_stevenson, garratt_v_dailey, palsgraf_v_long_island, vosburg_v_putney,
};

fn main() {
    println!("=== Famous Common Law Cases ===\n");

    // 1. Palsgraf v. Long Island Railroad Co. (1928)
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  PALSGRAF v. LONG ISLAND RAILROAD CO. (1928)              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let palsgraf = palsgraf_v_long_island();
    println!("Case: {}", palsgraf.short_name);
    println!("Citation: {}", palsgraf.citation);
    println!("Court: {:?}", palsgraf.court);
    println!("Year: {}", palsgraf.year);
    println!("Judge: Benjamin Cardozo (Chief Judge)\n");
    println!("Facts:");
    println!("{}", palsgraf.facts);
    println!("\nLegal Issues:");
    for issue in &palsgraf.issues {
        println!("  • {}", issue);
    }
    println!("\nHolding:");
    println!("{}", palsgraf.holding);
    println!("\nImpact:");
    println!("  → Established 'foreseeable plaintiff' rule in US tort law");
    println!("  → Majority rule: No duty to unforeseeable plaintiffs\n");

    println!("\n{}", "=".repeat(60));
    println!("\n");

    // 2. Garratt v. Dailey (1955)
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  GARRATT v. DAILEY (1955)                                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let garratt = garratt_v_dailey();
    println!("Case: {}", garratt.short_name);
    println!("Citation: {}", garratt.citation);
    println!("Court: {:?}", garratt.court);
    println!("Facts:");
    println!("  • Brian Dailey, 5 years old");
    println!("  • Ruth Garratt (adult) about to sit in lawn chair");
    println!("  • Brian pulled chair away");
    println!("  • Ruth fell to ground, fractured hip\n");
    println!("Legal Issue:");
    println!("  Can 5-year-old have intent for battery?\n");
    println!("Holding:");
    println!("  ✓ YES - Intent satisfied if child knew with 'substantial certainty'");
    println!("     that harmful contact would result\n");
    println!("Two Types of Intent:");
    println!("  1. Purpose (desire to cause result)");
    println!("  2. Substantial Certainty (knowledge result will follow)\n");
    println!("\nFacts:");
    println!("{}", garratt.facts);
    println!("\nLegal Issues:");
    for issue in &garratt.issues {
        println!("  • {}", issue);
    }
    println!("\nHolding:");
    println!("{}", garratt.holding);
    println!("\nImpact:");
    println!("  → Expanded definition of 'intent' beyond mere purpose");
    println!("  → Children can be liable for intentional torts");
    println!("  → 'Substantial certainty' doctrine widely adopted\n");

    println!("\n{}", "=".repeat(60));
    println!("\n");

    // 3. Vosburg v. Putney (1891)
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  VOSBURG v. PUTNEY (1891)                                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let vosburg = vosburg_v_putney();
    println!("Case: {}", vosburg.short_name);
    println!("Citation: {}", vosburg.citation);
    println!("Court: {:?}", vosburg.court);
    println!("Facts:");
    println!("  • Classroom setting after recess");
    println!("  • George Putney (12) kicked Andrew Vosburg (14) in shin");
    println!("  • Kick was light, playful");
    println!("  • Vosburg had pre-existing injury");
    println!("  • Kick caused serious, permanent leg damage\n");
    println!("Legal Issue:");
    println!("  Liable for unintended severe consequences of intended touching?\n");
    println!("Holding:");
    println!("  ✓ YES - Intent to touch + unlawful contact = battery");
    println!("  ✓ Liable for ALL resulting damages (eggshell plaintiff rule)\n");
    println!("Eggshell Plaintiff Rule:");
    println!("  'Defendant takes plaintiff as he finds him'");
    println!("  Liable for full extent of injury, even if unforeseeable\n");
    println!("\nFacts:");
    println!("{}", vosburg.facts);
    println!("\nLegal Issues:");
    for issue in &vosburg.issues {
        println!("  • {}", issue);
    }
    println!("\nHolding:");
    println!("{}", vosburg.holding);
    println!("\nImpact:");
    println!("  → Intent to touch = intent to batter (if unlawful)");
    println!("  → Defendant liable for all consequences");
    println!("  → Classic 'thin skull' / 'eggshell plaintiff' case\n");

    println!("\n{}", "=".repeat(60));
    println!("\n");

    // 4. Donoghue v. Stevenson (1932) - English case, influential in US
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  DONOGHUE v. STEVENSON (1932) - English Law                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let donoghue = donoghue_v_stevenson();
    println!("Case: {}", donoghue.short_name);
    println!("Citation: {}", donoghue.citation);
    println!("Court: {:?}", donoghue.court);
    println!("Facts:");
    println!("  • Friend bought ginger beer for Mrs. Donoghue");
    println!("  • Opaque bottle contained decomposed snail");
    println!("  • Mrs. Donoghue became ill\n");
    println!("Legal Issue:");
    println!("  Manufacturer liable to ultimate consumer (no contract)?\n");
    println!("Holding (Lord Atkin):");
    println!("  ✓ YES - Manufacturer owes duty to ultimate consumer");
    println!("  'You must take reasonable care to avoid acts or omissions");
    println!("   which you can reasonably foresee would be likely to injure");
    println!("   your neighbour.'\n");
    println!("Neighbor Principle:");
    println!("  Who is my neighbor? Persons so closely and directly affected");
    println!("  by my act that I ought reasonably to have them in contemplation.\n");
    println!("\nFacts:");
    println!("{}", donoghue.facts);
    println!("\nIssue:");
    for issue in &donoghue.issues {
        println!("  • {}", issue);
    }
    println!("\nHolding:");
    println!("{}", donoghue.holding);
    println!("\nImpact on US Law:");
    println!("  → Highly influential, though not binding");
    println!("  → Led to development of products liability");
    println!("  → Influenced Restatement (Second) § 402A");
    println!("  → Foundation of modern negligence law\n");

    println!("\n{}", "=".repeat(60));
    println!("\n");

    println!("=== Why These Cases Matter ===\n");
    println!("Common Law Development:");
    println!("  ✓ Courts create law through decisions (stare decisis)");
    println!("  ✓ Each case refines and extends legal principles");
    println!("  ✓ No comprehensive code like Civil Law systems\n");
    println!("Comparison with Civil Law:");
    println!("  Civil Law: Legislature writes code → Courts apply");
    println!("  Common Law: Courts decide cases → Rules emerge → Restatement synthesizes\n");
    println!("These cases are still cited today:");
    println!("  • Palsgraf: Duty and foreseeability");
    println!("  • Garratt: Intent in torts");
    println!("  • Vosburg: Eggshell plaintiff");
    println!("  • Donoghue: Negligence duty\n");

    println!("✓ Example completed successfully");
}
