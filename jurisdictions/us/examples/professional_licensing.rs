//! Professional Licensing Across US States
//!
//! This example demonstrates how professional licenses (attorneys, physicians)
//! vary across US states and the challenges of interstate practice.
//!
//! Run with: `cargo run --example professional_licensing`

fn main() {
    println!("=== Professional Licensing Across US States ===\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ATTORNEY LICENSING: BAR ADMISSION                         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("To practice law in the US:");
    println!("  • Must be admitted to state bar");
    println!("  • Each state has own requirements");
    println!("  • NO automatic nationwide license");
    println!("  • Federal courts have separate admission\n");

    println!("Typical Requirements:");
    println!("  1. Law degree (J.D.) from ABA-accredited school");
    println!("  2. Pass bar examination");
    println!("  3. Pass character and fitness review");
    println!("  4. Take oath of admission\n");

    println!("{}", "=".repeat(60));
    println!("\n=== UNIFORM BAR EXAM (UBE) ===\n");

    println!("What is the UBE?");
    println!("  • Standardized bar exam (since 2011)");
    println!("  • Developed by National Conference of Bar Examiners (NCBE)");
    println!("  • Portable score across UBE jurisdictions");
    println!("  • Currently adopted by 41 jurisdictions\n");

    println!("UBE Components:");
    println!("  1. Multistate Essay Exam (MEE) - 6 essays");
    println!("  2. Multistate Performance Test (MPT) - 2 tasks");
    println!("  3. Multistate Bar Exam (MBE) - 200 multiple choice\n");

    println!("UBE Scoring:");
    println!("  • Maximum score: 400 points");
    println!("  • Passing score varies by jurisdiction:");
    println!("    - New York: 266");
    println!("    - California: Does NOT use UBE (own exam)");
    println!("    - Texas: Does NOT use UBE (own exam)");
    println!("    - Illinois: 266\n");

    println!("Non-UBE States (own bar exams):");
    println!("  • California (notoriously difficult)");
    println!("  • Texas");
    println!("  • Florida");
    println!("  • Louisiana (Civil Law - unique exam)\n");

    println!("{}", "=".repeat(60));
    println!("\n=== CALIFORNIA BAR EXAM - The Most Difficult ===\n");

    println!("Why California is different:");
    println!("  • Does NOT use UBE");
    println!("  • Own exam covering California-specific law");
    println!("  • Historically lowest pass rate in US");
    println!("  • 2-day exam covering state and federal law\n");

    println!("Unique California Features:");
    println!("  • Tests California-specific law (community property)");
    println!("  • Allows law office study (no law school required!)");
    println!("  • Baby Bar exam for non-ABA students");
    println!("  • Pass rate: ~40-50% (vs ~70-75% for UBE states)\n");

    println!("{}", "=".repeat(60));
    println!("\n=== PHYSICIAN LICENSING ===\n");

    println!("Medical License Requirements:");
    println!("  1. Medical degree (M.D. or D.O.)");
    println!("  2. Residency training");
    println!("  3. Pass USMLE (United States Medical Licensing Examination)");
    println!("  4. State medical board license");
    println!("  5. DEA registration (for prescribing)\n");

    println!("State Medical Licenses:");
    println!("  • Each state issues own license");
    println!("  • NO automatic reciprocity");
    println!("  • Must apply to each state separately\n");

    println!("{}", "=".repeat(60));
    println!("\n=== INTERSTATE MEDICAL LICENSURE COMPACT (IMLC) ===\n");

    println!("What is IMLC?");
    println!("  • Expedited licensure for physicians");
    println!("  • Member states recognize each other's licenses");
    println!("  • Streamlined application process");
    println!("  • Currently: 40 states + territories\n");

    println!("How it works:");
    println!("  1. Physician has license in 'State of Principal License' (SPL)");
    println!("  2. Meets eligibility criteria");
    println!("  3. Applies through IMLC for license in other member states");
    println!("  4. Expedited review (avg 10-30 days vs 3-6 months)\n");

    println!("Major states NOT in IMLC:");
    println!("  • California");
    println!("  • New York");
    println!("  • Texas");
    println!("  • Florida\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARISON TABLE ===\n");

    println!("┌─────────────┬──────────────┬───────────────┬──────────────┐");
    println!("│ Profession  │ Exam         │ Reciprocity   │ Interstate   │");
    println!("├─────────────┼──────────────┼───────────────┼──────────────┤");
    println!("│ Attorney    │ State bar    │ Limited       │ UBE (41)     │");
    println!("│             │ or UBE       │ varies        │              │");
    println!("├─────────────┼──────────────┼───────────────┼──────────────┤");
    println!("│ Physician   │ USMLE        │ None → must   │ IMLC (40)    │");
    println!("│             │ (national)   │ apply each    │              │");
    println!("└─────────────┴──────────────┴───────────────┴──────────────┘\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARISON WITH UNITARY SYSTEMS ===\n");

    println!("Civil Law Countries:");
    println!("  Japan:");
    println!("    • 司法試験 (bar exam) - nationwide license");
    println!("    • 医師国家試験 (medical exam) - nationwide license");
    println!("    → ONE license for entire country\n");

    println!("  Germany:");
    println!("    • Staatsexamen - nationwide license");
    println!("    → Practice anywhere in Germany + EU\n");

    println!("United States:");
    println!("  • 51 separate licensing regimes (50 states + DC)");
    println!("  • Must obtain license in each state");
    println!("  • Compacts reduce but don't eliminate burden");
    println!("  → Complexity from federalism\n");

    println!("✓ Example completed successfully");
}
