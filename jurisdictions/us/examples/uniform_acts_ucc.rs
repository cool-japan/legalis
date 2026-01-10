//! Uniform Commercial Code (UCC) - Adoption and Variations
//!
//! This example demonstrates the Uniform Commercial Code and how it aims
//! to create consistency across US states despite being adopted differently.
//!
//! Run with: `cargo run --example uniform_acts_ucc`

use legalis_us::uniform_acts::UCCArticle;

fn main() {
    println!("=== Uniform Commercial Code (UCC) Across US States ===\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  WHAT IS THE UCC?                                          ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("The Uniform Commercial Code (UCC):");
    println!("  • Model statute for commercial transactions");
    println!("  • Drafted by National Conference of Commissioners on");
    println!("    Uniform State Laws (NCCUSL) + American Law Institute (ALI)");
    println!("  • First published: 1952");
    println!("  • Goal: Create uniform commercial law across all 50 states\n");

    println!("Why needed?");
    println!("  ✓ Interstate commerce requires predictability");
    println!("  ✓ 50 different commercial laws = chaos");
    println!("  ✓ Businesses need consistent rules\n");

    println!("UCC Articles:\n");

    for article in UCCArticle::all() {
        println!("  Article {}: {}", article.number(), article.name());
    }

    println!("\n{}", "=".repeat(60));
    println!("\n=== UCC ADOPTION STATUS ===\n");

    println!("All 50 states + DC have adopted UCC, but with variations:\n");

    println!("California:");
    println!("  ✓ Adopted all UCC articles");
    println!("  • California Commercial Code (Cal. Com. Code)");
    println!("  • State-specific variations in Article 2 (Sales)");

    println!("\nNew York:");
    println!("  ✓ Adopted all UCC articles");
    println!("  • N.Y. U.C.C. Law");
    println!("  • Important financial center, influential interpretations");

    println!("\nTexas:");
    println!("  ✓ Adopted all UCC articles");
    println!("  • Texas Business & Commerce Code");
    println!("  • Some variations in secured transactions");

    println!("\nLouisiana (SPECIAL CASE):");
    println!("  ✓ Partial adoption");
    println!("  • Only Civil Law state in US");
    println!("  • Did NOT adopt Article 2 (Sales) until much later");
    println!("  • Maintains civilian approach to commercial law\n");

    println!("{}", "=".repeat(60));
    println!("\n=== EXAMPLE: UCC ARTICLE 2 - SALES ===\n");

    println!("Scenario: Sale of Defective Machinery");
    println!("  • Buyer: Company in New York");
    println!("  • Seller: Company in California");
    println!("  • Goods: Manufacturing equipment ($500,000)");
    println!("  • Problem: Equipment doesn't work as promised\n");

    println!("UCC Article 2 Warranty Provisions:\n");

    println!("§ 2-314: IMPLIED WARRANTY OF MERCHANTABILITY");
    println!("  Goods must be:");
    println!("  ✓ Fit for ordinary purposes");
    println!("  ✓ Adequately packaged");
    println!("  ✓ Conform to label/description");
    println!("  → Applies in ALL states (with minor variations)\n");

    println!("§ 2-315: IMPLIED WARRANTY OF FITNESS FOR PARTICULAR PURPOSE");
    println!("  If seller knows:");
    println!("  • Particular purpose buyer needs goods for");
    println!("  • Buyer relies on seller's expertise");
    println!("  → Warranty that goods will fit that purpose\n");

    println!("§ 2-316: DISCLAIMER OF WARRANTIES");
    println!("  Seller can disclaim implied warranties if:");
    println!("  • Conspicuous writing");
    println!("  • Mentions 'merchantability' by name");
    println!("  • OR 'as is' / 'with all faults' language\n");

    println!("Uniformity Achieved:");
    println!("  ✓ Same rules in New York and California");
    println!("  ✓ Predictable outcome for interstate commerce");
    println!("  ✓ Businesses can plan transactions\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARISON WITH CIVIL LAW ===\n");

    println!("Civil Law Countries:");
    println!("  France: Code de commerce (1807) - nationwide");
    println!("  Germany: HGB (Handelsgesetzbuch) - nationwide");
    println!("  Japan: 商法 (Commercial Code) - nationwide");
    println!("  → ONE code, binding across entire country\n");

    println!("United States (Federal System):");
    println!("  • NO national commercial code");
    println!("  • UCC is MODEL statute (persuasive)");
    println!("  • Each state adopts and modifies");
    println!("  • Result: 51 versions with variations");
    println!("  • Federal government has limited commerce power\n");

    println!("Why US doesn't have federal commercial code:");
    println!("  • Tenth Amendment: Powers not delegated to federal government");
    println!("    are reserved to states");
    println!("  • Contract law traditionally state matter");
    println!("  • States jealously guard sovereignty");
    println!("  • UCC compromise: Uniform model, state adoption\n");

    println!("✓ Example completed successfully");
}
