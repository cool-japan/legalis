//! Products Liability - Restatement § 402A
//!
//! This example demonstrates strict liability for defective products
//! under the Restatement (Second) of Torts § 402A.
//!
//! Run with: `cargo run --example products_liability`

use legalis_us::restatement::section_402a_products_liability;

fn main() {
    println!("=== Products Liability: Restatement (Second) § 402A ===\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  STRICT LIABILITY FOR DEFECTIVE PRODUCTS                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let section_402a = section_402a_products_liability();

    println!("Case Rule: {}", section_402a.name);
    println!("Citation: Restatement (Second) of Torts § 402A (1965)");
    println!("Effect: {}\n", section_402a.effect.description);

    println!("\n{}", "=".repeat(60));
    println!("\n=== THE RULE ===\n");

    println!("Restatement (Second) of Torts § 402A (1965):\n");
    println!("(1) One who sells any product in a defective condition");
    println!("    unreasonably dangerous to the user or consumer or to his");
    println!("    property is subject to liability for physical harm thereby");
    println!("    caused to the ultimate user or consumer, or to his property, if");

    println!("\n    (a) the seller is engaged in the business of selling such");
    println!("        a product, and");
    println!("\n    (b) it is expected to and does reach the user or consumer");
    println!("        without substantial change in the condition in which");
    println!("        it is sold.\n");

    println!("(2) The rule stated in Subsection (1) applies although");
    println!("\n    (a) the seller has exercised all possible care in the");
    println!("        preparation and sale of his product, and");
    println!("\n    (b) the user or consumer has not bought the product from");
    println!("        or entered into any contractual relation with the seller.\n");

    println!("{}", "=".repeat(60));
    println!("\n=== KEY ELEMENTS ===\n");

    println!("1. SELLER IN THE BUSINESS");
    println!("   • Must be merchant/commercial seller");
    println!("   • NOT casual/occasional sellers");
    println!("   • Example: Manufacturer, wholesaler, retailer ✓");
    println!("   • Example: Individual selling used car ✗\n");

    println!("2. DEFECTIVE CONDITION");
    println!("   Three types of defects:\n");

    println!("   a) Manufacturing Defect:");
    println!("      • Product deviates from intended design");
    println!("      • Example: Car with missing brake pad");
    println!("      • Example: Bottle with crack\n");

    println!("   b) Design Defect:");
    println!("      • Entire product line designed unsafely");
    println!("      • Example: SUV with high rollover risk");
    println!("      • Example: Ladder with unstable base\n");

    println!("   c) Failure to Warn:");
    println!("      • Inadequate warnings about dangers");
    println!("      • Example: Medication without side effect warning");
    println!("      • Example: Chemical without hazard label\n");

    println!("3. UNREASONABLY DANGEROUS");
    println!("   • Beyond what ordinary consumer expects");
    println!("   • Product dangerous beyond contemplation of user");
    println!("   • Not merely that product could be safer\n");

    println!("4. CAUSATION");
    println!("   • Defect must cause the harm");
    println!("   • Physical harm to person or property");
    println!("   • Economic loss alone usually insufficient\n");

    println!("5. REACHES USER WITHOUT CHANGE");
    println!("   • Product in same condition as when sold");
    println!("   • No substantial alteration by third parties\n");

    println!("{}", "=".repeat(60));
    println!("\n=== STRICT LIABILITY - What Does It Mean? ===\n");

    println!("Strict Liability:");
    println!("  ✓ NO need to prove negligence (fault)");
    println!("  ✓ NO need to prove intent");
    println!("  ✓ Seller liable even if exercised ALL possible care");
    println!("  ✓ Focus on PRODUCT, not seller's conduct\n");

    println!("Comparison with Other Theories:\n");

    println!("┌──────────────────┬───────────┬────────────┬────────────┐");
    println!("│ Theory           │ Fault?    │ Privity?   │ Difficulty │");
    println!("├──────────────────┼───────────┼────────────┼────────────┤");
    println!("│ Negligence       │ Required  │ Not needed │ Hard       │");
    println!("│ Warranty         │ Not needed│ Required   │ Medium     │");
    println!("│ Strict Liability │ Not needed│ Not needed │ Easy       │");
    println!("└──────────────────┴───────────┴────────────┴────────────┘\n");

    println!("Why Strict Liability?");
    println!("  Policy Justifications:");
    println!("  1. Risk spreading: Manufacturers can insure/spread costs");
    println!("  2. Incentive for safety: Encourages safer products");
    println!("  3. Consumer protection: Easier recovery for victims");
    println!("  4. Privity barrier: No contract needed with manufacturer\n");

    println!("{}", "=".repeat(60));
    println!("\n=== HISTORICAL EVOLUTION ===\n");

    println!("1842: Winterbottom v. Wright (England)");
    println!("  • Privity requirement: Only parties in contract can sue");
    println!("  • Harsh rule limited manufacturer liability\n");

    println!("1916: MacPherson v. Buick Motor Co. (Cardozo, NY)");
    println!("  • Eliminated privity for negligence claims");
    println!("  • BUT still required proving fault\n");

    println!("1932: Donoghue v. Stevenson (UK House of Lords)");
    println!("  • 'Neighbor principle' - duty to ultimate consumer");
    println!("  • Influenced US law development\n");

    println!("1963: Greenman v. Yuba Power Products (California)");
    println!("  • First strict liability case in US");
    println!("  • Plaintiff injured by defective power tool");
    println!("  • Court: Strict liability without fault\n");

    println!("1965: Restatement (Second) § 402A");
    println!("  • ALI codifies strict liability");
    println!("  • Widely adopted across US");
    println!("  • Now majority rule in all 50 states\n");

    println!("{}", "=".repeat(60));
    println!("\n=== HYPOTHETICAL SCENARIO ===\n");

    println!("Facts:");
    println!("  • Alice buys a blender from RetailCo");
    println!("  • Blender manufactured by BlenderCorp");
    println!("  • While blending, glass jar shatters");
    println!("  • Glass cuts Alice's hand (nerve damage)");
    println!("  • Investigation: Glass had microscopic crack from manufacturing\n");

    println!("Analysis under § 402A:\n");

    println!("1. Seller in business?");
    println!("   ✓ YES - BlenderCorp is manufacturer");
    println!("   ✓ YES - RetailCo is retailer");
    println!("   → Both potentially liable\n");

    println!("2. Defective condition?");
    println!("   ✓ YES - Manufacturing defect (microscopic crack)");
    println!("   → Deviates from intended design\n");

    println!("3. Unreasonably dangerous?");
    println!("   ✓ YES - Consumer doesn't expect jar to shatter");
    println!("   ✓ YES - Dangerous beyond ordinary use risks\n");

    println!("4. Causation?");
    println!("   ✓ YES - Crack caused shattering");
    println!("   ✓ YES - Shattering caused injury\n");

    println!("5. Reached user without change?");
    println!("   ✓ YES - Alice used it as intended");
    println!("   ✓ NO alteration or misuse\n");

    println!("Result:");
    println!("  ✓ BlenderCorp strictly liable");
    println!("  ✓ RetailCo also strictly liable (can seek indemnity from BlenderCorp)");
    println!("  ✓ Alice recovers even though no one was 'negligent'");
    println!("  ✓ NO need to prove BlenderCorp failed to inspect\n");

    println!("Defenses:");
    println!("  ✗ 'We used reasonable care' - NOT a defense (strict liability)");
    println!("  ✗ 'No privity with Alice' - NOT a defense (§ 402A eliminates)");
    println!("  ✓ Product misuse - Could be defense");
    println!("  ✓ Comparative fault - Reduces damages in some states\n");

    println!("{}", "=".repeat(60));
    println!("\n=== FAMOUS PRODUCTS LIABILITY CASES ===\n");

    println!("1. Greenman v. Yuba Power Products (1963)");
    println!("   • Defective power tool");
    println!("   • First strict liability case");
    println!("   • No privity, no fault needed\n");

    println!("2. Grimshaw v. Ford Motor Co. (1981)");
    println!("   • Ford Pinto gas tank design");
    println!("   • Design defect - prone to explosion in rear-end collisions");
    println!("   • $125 million punitive damages (reduced to $3.5M)\n");

    println!("3. Liebeck v. McDonald's (1994)");
    println!("   • Famous 'hot coffee' case");
    println!("   • Coffee at 180-190°F caused third-degree burns");
    println!("   • Failure to warn + product too dangerous");
    println!("   • Often misunderstood - plaintiff had severe injuries\n");

    println!("4. Wyeth v. Levine (2009)");
    println!("   • Pharmaceutical failure to warn");
    println!("   • Inadequate warning led to amputation");
    println!("   • State tort law NOT preempted by FDA approval\n");

    println!("{}", "=".repeat(60));
    println!("\n=== COMPARISON WITH OTHER LEGAL SYSTEMS ===\n");

    println!("United States (§ 402A):");
    println!("  ✓ Strict liability - no fault required");
    println!("  ✓ Widely adopted across all 50 states");
    println!("  ✓ Pro-consumer protection\n");

    println!("European Union (Product Liability Directive 85/374/EEC):");
    println!("  ✓ Strict liability for defective products");
    println!("  ✓ Harmonized across EU");
    println!("  ✓ Development risk defense available");
    println!("  ✗ Consumer bears burden of proof on defect\n");

    println!("Japan (製造物責任法 - PL Law 1994):");
    println!("  ✓ Strict liability introduced 1994");
    println!("  ✓ Based on EU Directive");
    println!("  ✗ Traditional culture: reluctant to sue");
    println!("  ✗ Fewer cases than US despite similar law\n");

    println!("Germany (ProdHaftG - Product Liability Act 1990):");
    println!("  ✓ Strict liability");
    println!("  ✓ Implements EU Directive");
    println!("  ✓ Development risk defense (state of art)");
    println!("  ✗ More protective of manufacturers than US\n");

    println!("Key Differences:");
    println!("  • US: Most pro-plaintiff, easiest recovery");
    println!("  • EU: Balance between consumer protection and innovation");
    println!("  • Japan: Law similar to EU but cultural factors reduce litigation\n");

    println!("{}", "=".repeat(60));
    println!("\n=== RESTATEMENT (THIRD) - MODERN EVOLUTION ===\n");

    println!("Restatement (Third) of Torts: Products Liability (1998):");
    println!("  • Updates § 402A with modern approach");
    println!("  • Distinguishes three defect types more clearly");
    println!("  • Risk-utility test for design defects");
    println!("  • States slowly adopting\n");

    println!("Key Changes:");
    println!("  § 402A (Second): 'Unreasonably dangerous' standard");
    println!("  Restatement (Third): Separate tests for each defect type\n");

    println!("  Manufacturing defect: Departs from intended design");
    println!("  Design defect: Reasonable alternative design available");
    println!("  Warning defect: Foreseeable risks not warned\n");

    println!("Status:");
    println!("  • Many states still use § 402A (Second)");
    println!("  • Some adopted Third Restatement");
    println!("  • Mixed approaches across US\n");

    println!("✓ Example completed successfully");
}
