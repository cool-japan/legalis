//! Federal Preemption - Federal vs State Law Conflicts
//!
//! This example demonstrates when federal law preempts (overrides) state law
//! under the Supremacy Clause of the US Constitution.
//!
//! Run with: `cargo run --example federal_preemption`

fn main() {
    println!("=== Federal Preemption: Federal vs State Law ===\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  SUPREMACY CLAUSE (US Constitution Article VI)             ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("US Constitution, Article VI, Clause 2:\n");
    println!("  'This Constitution, and the Laws of the United States which");
    println!("   shall be made in Pursuance thereof... shall be the supreme");
    println!("   Law of the Land; and the Judges in every State shall be bound");
    println!("   thereby, any Thing in the Constitution or Laws of any State");
    println!("   to the Contrary notwithstanding.'\n");

    println!("What this means:");
    println!("  ✓ Federal law is supreme over state law");
    println!("  ✓ When conflict exists, federal law wins");
    println!("  ✓ State law is 'preempted' (invalidated)\n");

    println!("BUT: Federal government has LIMITED powers");
    println!("  • Tenth Amendment: Powers not delegated to US are reserved to states");
    println!("  • Federal law must be within enumerated powers");
    println!("  • Most common: Commerce Clause (regulate interstate commerce)\n");

    println!("{}", "=".repeat(60));
    println!("\n=== THREE TYPES OF PREEMPTION ===\n");

    println!("1. EXPRESS PREEMPTION");
    println!("   • Federal statute explicitly states it preempts state law");
    println!("   • Clear congressional intent");
    println!("   • Example: 'This Act shall supersede all state laws...'\n");

    println!("2. IMPLIED CONFLICT PREEMPTION");
    println!("   • State law conflicts with federal law");
    println!("   • Impossible to comply with both");
    println!("   • OR state law frustrates federal purpose\n");

    println!("3. FIELD PREEMPTION");
    println!("   • Federal law occupies entire regulatory field");
    println!("   • No room for state regulation");
    println!("   • Example: Immigration, foreign affairs\n");

    println!("{}", "=".repeat(60));
    println!("\n=== SCENARIO 1: Express Preemption - ERISA ===\n");

    println!("Employee Retirement Income Security Act (ERISA, 1974):\n");

    println!("Facts:");
    println!("  • Federal law regulating employee benefit plans");
    println!("  • California passes law requiring employers to provide");
    println!("    specific health benefits");
    println!("  • Does federal ERISA preempt California law?\n");

    println!("ERISA § 514(a) - Express Preemption Clause:");
    println!("  '[ERISA] shall supersede any and all State laws insofar as");
    println!("   they may now or hereafter relate to any employee benefit plan...'\n");

    println!("Analysis:");
    println!("  ✓ ERISA contains express preemption language");
    println!("  ✓ Very broad: 'relate to' any benefit plan");
    println!("  ✓ California law relates to employee benefits");
    println!("  → Federal law PREEMPTS state law\n");

    println!("Result:");
    println!("  ✗ California law invalid (preempted)");
    println!("  ✓ Uniform national standard for employee benefits\n");

    println!("{}", "=".repeat(60));
    println!("\n=== SCENARIO 2: Conflict Preemption - Immigration ===\n");

    println!("Arizona SB 1070 (2010) - Immigration Enforcement Law:\n");

    println!("Facts:");
    println!("  • Arizona passes strict immigration enforcement law");
    println!("  • Requires police to check immigration status during stops");
    println!("  • Makes it state crime to be in US illegally");
    println!("  • Does this conflict with federal immigration law?\n");

    println!("Analysis:");
    println!("  ✓ Immigration = federal domain (field preemption)");
    println!("  ✓ State law conflicts with federal discretion");
    println!("  ✓ Obstacle to federal objectives");
    println!("  → Federal law PREEMPTS most of state law\n");

    println!("Arizona v. United States (2012):");
    println!("  • Supreme Court struck down most of SB 1070");
    println!("  • Immigration enforcement = federal domain");
    println!("  • States cannot create parallel enforcement\n");

    println!("{}", "=".repeat(60));
    println!("\n=== SUMMARY TABLE ===\n");

    println!("┌────────────────────┬─────────────┬─────────────────────┐");
    println!("│ Area               │ Preemption? │ Winner              │");
    println!("├────────────────────┼─────────────┼─────────────────────┤");
    println!("│ ERISA benefits     │ YES         │ Federal (express)   │");
    println!("│ Immigration        │ YES         │ Federal (field)     │");
    println!("│ CA emissions       │ NO          │ State (exception)   │");
    println!("│ Drug labels (brand)│ NO          │ State tort law      │");
    println!("└────────────────────┴─────────────┴─────────────────────┘\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARISON WITH OTHER FEDERAL SYSTEMS ===\n");

    println!("United States:");
    println!("  • Dual sovereignty (federal + state)");
    println!("  • Federal powers ENUMERATED (limited)");
    println!("  • State powers RESERVED (default)");
    println!("  • Preemption when federal law supreme\n");

    println!("Germany (Bundesstaat):");
    println!("  • Federal system (Bund + Länder)");
    println!("  • Article 31 GG: Federal law breaks state law");
    println!("  • Similar to US but more cooperative federalism\n");

    println!("Unitary Systems (Japan, France, UK):");
    println!("  • NO federal-state conflict");
    println!("  • National law applies throughout");
    println!("  • No preemption issue (only one sovereign)\n");

    println!("✓ Example completed successfully");
}
