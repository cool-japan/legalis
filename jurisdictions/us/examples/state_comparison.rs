//! State Comparison - Negligence Rules Across US States
//!
//! This example compares how different states handle contributory vs
//! comparative negligence in tort law.
//!
//! Run with: `cargo run --example state_comparison`

fn main() {
    println!("=== US State Comparison: Negligence Systems ===\n");

    println!("When plaintiff is partially at fault for their own injury,");
    println!("how does that affect recovery? US states have different rules:\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  THREE SYSTEMS OF COMPARATIVE NEGLIGENCE                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("1. CONTRIBUTORY NEGLIGENCE (Traditional Rule)");
    println!("   • Any fault by plaintiff = complete bar to recovery");
    println!("   • Harsh rule from 19th century");
    println!("   • Only 4 states still use: AL, MD, NC, VA (+ DC)\n");

    println!("2. PURE COMPARATIVE NEGLIGENCE (Modern Majority)");
    println!("   • Damages reduced by plaintiff's % of fault");
    println!("   • Plaintiff recovers even if 99% at fault");
    println!("   • 13 states use this system\n");

    println!("3. MODIFIED COMPARATIVE NEGLIGENCE");
    println!("   a) 50% Bar (12 states):");
    println!("      • Plaintiff recovers if 50% or less at fault");
    println!("   b) 51% Bar (21 states):");
    println!("      • Plaintiff recovers if less than 51% at fault\n");

    println!("{}", "=".repeat(60));
    println!("\n=== HYPOTHETICAL: Car Accident ===\n");

    println!("Facts:");
    println!("  • Plaintiff runs red light");
    println!("  • Defendant speeding (50 mph in 30 mph zone)");
    println!("  • Total damages: $100,000");
    println!("  • Jury finds: Plaintiff 40% at fault, Defendant 60% at fault\n");

    println!("How much can plaintiff recover in each state?\n");

    println!("{}", "=".repeat(60));
    println!("\n1. CALIFORNIA - Pure Comparative Negligence\n");

    println!("System: Pure Comparative Negligence");
    println!("Rule: Damages reduced by plaintiff's percentage of fault");
    println!("\nCalculation:");
    println!("  Total damages: $100,000");
    println!("  Plaintiff's fault: 40%");
    println!("  Recovery: $100,000 × (100% - 40%) = $60,000");
    println!("\n✓ Plaintiff recovers $60,000");
    println!("\nStates using Pure Comparative:");
    println!("  AK, AZ, CA, FL, KY, LA, MS, MO, NM, NY, RI, WA, PR\n");

    println!("{}", "=".repeat(60));
    println!("\n2. TEXAS - Modified Comparative (51% Bar)\n");

    println!("System: Modified Comparative (51% Bar)");
    println!("Rule: Plaintiff recovers only if LESS THAN 51% at fault");
    println!("\nCalculation:");
    println!("  Plaintiff's fault: 40%");
    println!("  40% < 51%? YES → Plaintiff may recover");
    println!("  Recovery: $100,000 × (100% - 40%) = $60,000");
    println!("\n✓ Plaintiff recovers $60,000");
    println!("\nWhat if plaintiff was 51% at fault?");
    println!("  51% ≥ 51%? YES → NO RECOVERY (complete bar)");
    println!("\nStates using 51% Bar:");
    println!("  AR, CO, GA, ID, KS, ME, ND, NE, OK, TN, TX, UT, WV\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARATIVE TABLE ===\n");

    println!("┌────────────┬───────────────────────┬──────────────────┐");
    println!("│ State      │ System                │ 40% Fault Result │");
    println!("├────────────┼───────────────────────┼──────────────────┤");
    println!("│ California │ Pure Comparative      │ $60,000          │");
    println!("│ New York   │ Pure Comparative      │ $60,000          │");
    println!("│ Florida    │ Pure Comparative      │ $60,000          │");
    println!("│ Texas      │ Modified (51% bar)    │ $60,000          │");
    println!("│ Virginia   │ Contributory (old)    │ $0 (barred!)     │");
    println!("└────────────┴───────────────────────┴──────────────────┘\n");

    println!("If Plaintiff was 51% at fault:\n");
    println!("┌────────────┬───────────────────────┬──────────────────┐");
    println!("│ State      │ System                │ 51% Fault Result │");
    println!("├────────────┼───────────────────────┼──────────────────┤");
    println!("│ California │ Pure Comparative      │ $49,000          │");
    println!("│ New York   │ Pure Comparative      │ $49,000          │");
    println!("│ Florida    │ Pure Comparative      │ $49,000          │");
    println!("│ Texas      │ Modified (51% bar)    │ $0 (barred!)     │");
    println!("│ Virginia   │ Contributory (old)    │ $0 (barred!)     │");
    println!("└────────────┴───────────────────────┴──────────────────┘\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARISON WITH CIVIL LAW ===\n");

    println!("Unitary Civil Law Countries:");
    println!("  Japan (民法722条2項): Comparative fault nationwide");
    println!("  Germany (BGB §254): Mitigation based on plaintiff's contribution");
    println!("  France (Code civil 1240): Partial fault reduces damages");
    println!("  → ONE rule for entire country\n");

    println!("United States (Federal Common Law):");
    println!("  → 51 DIFFERENT rules");
    println!("  → Choice of law determines outcome");
    println!("  → Forum shopping incentive");
    println!("  → Complexity from federalism\n");

    println!("✓ Example completed successfully");
}
