//! Contract breach and damages calculation example
//!
//! Demonstrates French contract law Articles 1217, 1224, 1231

use legalis_fr::contract::{
    BreachType, Contract, ContractType, ValidityDefect, calculate_contract_damages,
    calculate_damages_with_force_majeure, validate_breach_claim, validate_contract_validity,
};

fn main() {
    println!("=== French Contract Law Example ===\n");
    println!("Code civil - Contract law (2016 reform)\n");

    // Example 1: Valid sale contract
    println!("📋 Example 1: Valid Sale Contract (Article 1128)");
    println!("────────────────────────────────────────────");

    let contract = Contract::new()
        .with_type(ContractType::Sale {
            price: 50_000,
            subject: "Machine industrielle".to_string(),
        })
        .with_parties(vec![
            "Acheteur SARL TechSolutions".to_string(),
            "Vendeur SA EquipmentCorp".to_string(),
        ])
        .with_consent(true)
        .with_good_faith(true);

    match validate_contract_validity(&contract) {
        Ok(_) => {
            println!("✅ Contract is valid (Article 1128: consent + capacity + lawful content)")
        }
        Err(e) => println!("❌ Contract invalid: {}", e),
    }

    println!();

    // Example 2: Contract with breach - calculate damages
    println!("💰 Example 2: Contract Breach - Damages Calculation (Article 1231)");
    println!("───────────────────────────────────────────────────────────────");

    let contract_with_breach = Contract::new()
        .with_type(ContractType::Sale {
            price: 50_000,
            subject: "Machine industrielle".to_string(),
        })
        .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
        .with_consent(true)
        .with_breach(BreachType::NonPerformance)
        .with_contract_value(50_000)
        .with_actual_loss(45_000); // Buyer had to purchase elsewhere at €95,000

    // Validate breach claim
    match validate_breach_claim(&contract_with_breach) {
        Ok(_) => println!("✅ Breach claim is valid"),
        Err(e) => println!("❌ Invalid claim: {}", e),
    }

    // Calculate damages without penalty clause
    let damages = calculate_contract_damages(
        50_000, // Contract value
        45_000, // Actual loss (damnum emergens)
        None,   // No penalty clause
    );

    println!("\nDamages calculation (Article 1231):");
    println!("  Original contract price: €50,000");
    println!("  Price paid elsewhere: €95,000");
    println!("  Actual loss (damnum emergens): €45,000");
    println!("  ➜ Damages awarded: €{}", damages);
    println!("\n  Note: French contract law does not require proof of fault,");
    println!("        only breach of obligation (unlike tort law Article 1240)");

    println!();

    // Example 3: Contract with penalty clause
    println!("⚖️  Example 3: Penalty Clause (Clause pénale - Article 1231-5)");
    println!("──────────────────────────────────────────────────────────");

    let _contract_with_penalty = Contract::new()
        .with_type(ContractType::Service {
            description: "Construction project".to_string(),
            remuneration: 500_000,
        })
        .with_parties(vec!["Client".to_string(), "Contractor".to_string()])
        .with_consent(true)
        .with_breach(BreachType::DelayedPerformance)
        .with_contract_value(500_000)
        .with_actual_loss(80_000)
        .with_penalty_clause(100_000); // €100,000 penalty for delay

    let damages_with_penalty = calculate_contract_damages(
        500_000,
        80_000,
        Some(100_000), // Penalty clause takes precedence
    );

    println!("Contract value: €500,000");
    println!("Actual loss: €80,000");
    println!("Penalty clause: €100,000");
    println!("➜ Damages awarded: €{}", damages_with_penalty);
    println!("\nNote: Penalty clause takes precedence (Article 1231-5),");
    println!("      but courts may reduce excessive penalties (modération judiciaire)");

    println!();

    // Example 4: Force majeure exemption
    println!("🌪️  Example 4: Force Majeure Exemption (Article 1218)");
    println!("──────────────────────────────────────────────────");

    println!("Scenario: COVID-19 pandemic prevents performance\n");

    // Without force majeure - liable
    let damages_no_fm = calculate_damages_with_force_majeure(
        100_000, // Contract value
        80_000,  // Actual loss
        None,    // No penalty
        false,   // No force majeure
    );

    println!("Without force majeure:");
    println!("  ➜ Damages: €{}", damages_no_fm);

    // With force majeure - not liable
    let damages_with_fm = calculate_damages_with_force_majeure(
        100_000, // Contract value
        80_000,  // Actual loss
        None,    // No penalty
        true,    // Force majeure applies
    );

    println!("\nWith force majeure (COVID-19 government lockdown):");
    println!("  ➜ Damages: €{}", damages_with_fm);
    println!("  Debtor is exempt from liability (Article 1218)");

    println!();

    // Example 5: Invalid contract - validity defect
    println!("❌ Example 5: Invalid Contract - Fraud (Dol - Article 1137)");
    println!("────────────────────────────────────────────────────────");

    let invalid_contract = Contract::new()
        .with_type(ContractType::Sale {
            price: 100_000,
            subject: "Antique painting".to_string(),
        })
        .with_parties(vec!["Buyer".to_string(), "Fraudulent Seller".to_string()])
        .with_consent(true)
        .with_validity_defect(ValidityDefect::Fraud {
            by_contracting_party: true,
            description: "Seller falsely claimed painting was authentic Monet".to_string(),
        });

    match validate_contract_validity(&invalid_contract) {
        Ok(_) => println!("✅ Contract is valid"),
        Err(e) => {
            println!("❌ Contract is invalid: {}", e);
            println!("\nExplanation: Fraud (dol) vitiates consent (Article 1137).");
            println!("The contract can be annulled (nullité).");
        }
    }

    println!();

    // Summary
    println!("════════════════════════════════════════════════════════");
    println!("📚 Summary - French Contract Law Key Principles");
    println!("════════════════════════════════════════════════════════");
    println!();
    println!("🔹 Article 1103: Binding force - contracts have force of law");
    println!("🔹 Article 1128: Three validity requirements:");
    println!("   1. Consent  2. Capacity  3. Lawful & certain content");
    println!("🔹 Article 1217: Five breach remedies:");
    println!("   1. Exception  2. Specific performance  3. Price reduction");
    println!("   4. Termination  5. Damages");
    println!("🔹 Article 1231: Damages for breach (no fault required)");
    println!("🔹 Article 1218: Force majeure exempts from liability");
    println!("🔹 Article 1231-5: Penalty clauses (but courts may reduce)");
    println!();
    println!("Comparison with Japanese law (民法):");
    println!("  - Similar: No fault required for breach (仏1231条 vs 日415条)");
    println!("  - Similar: Specific performance preferred (仏1221条 vs 日414条)");
    println!("  - Difference: France explicitly lists 3 validity requirements");
    println!();
}
