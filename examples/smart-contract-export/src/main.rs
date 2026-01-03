//! Smart Contract Export Example
//!
//! This example demonstrates how to use `legalis-chain` to generate
//! smart contracts from legal statutes across multiple blockchain platforms.
//!
//! ## Supported Platforms (24+)
//!
//! - **EVM**: Solidity, Vyper, ZkSync Era, Base, Scroll, Linea
//! - **WASM**: Rust/WASM, Ink! (Substrate), CosmWasm
//! - **Move**: Aptos, Sui
//! - **Others**: Cairo (StarkNet), Solana, NEAR, TON, Algorand
//!
//! ## Key Concept
//!
//! Only **deterministic** statutes (without judicial discretion) can be
//! converted to smart contracts. This aligns with the Legalis philosophy:
//! "Governance as Code, Justice as Narrative"

use legalis_chain::{ContractGenerator, TargetPlatform};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

fn create_adult_rights_statute() -> Statute {
    Statute::new(
        "adult-rights-act",
        "Adult Rights Act 2024",
        Effect::new(EffectType::Grant, "Full legal capacity and voting rights"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("INTL")
}

fn create_welfare_eligibility_statute() -> Statute {
    Statute::new(
        "welfare-eligibility",
        "Welfare Benefits Eligibility",
        Effect::new(EffectType::Grant, "Eligible for welfare benefits"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 30000,
        }),
    ))
    .with_jurisdiction("INTL")
}

fn create_senior_discount_statute() -> Statute {
    Statute::new(
        "senior-discount",
        "Senior Citizen Discount Act",
        Effect::new(EffectType::Grant, "Eligible for 20% senior discount"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 65,
    })
    .with_jurisdiction("INTL")
}

fn print_contract_preview(source: &str, max_lines: usize) {
    let lines: Vec<&str> = source.lines().collect();
    let display_lines = lines.len().min(max_lines);
    for line in lines.iter().take(display_lines) {
        println!("      {}", line);
    }
    if lines.len() > max_lines {
        println!("      ... ({} more lines)", lines.len() - max_lines);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   SMART CONTRACT EXPORT - Legalis-Chain Demo");
    println!("   法令からスマートコントラクトを生成");
    println!("{}", "=".repeat(80));
    println!();

    // Create test statutes
    let statutes = vec![
        create_adult_rights_statute(),
        create_welfare_eligibility_statute(),
        create_senior_discount_statute(),
    ];

    println!("Step 1: Created {} test statutes\n", statutes.len());
    for statute in &statutes {
        println!("   - {} ({})", statute.title, statute.id);
    }
    println!();

    // Define target platforms to demonstrate
    let platforms = vec![
        (TargetPlatform::Solidity, "Solidity (Ethereum/EVM)"),
        (TargetPlatform::RustWasm, "Rust/WASM"),
        (TargetPlatform::Ink, "Ink! (Substrate/Polkadot)"),
        (TargetPlatform::Move, "Move (Aptos/Sui)"),
        (TargetPlatform::Cairo, "Cairo (StarkNet)"),
        (TargetPlatform::Vyper, "Vyper (Pythonic EVM)"),
    ];

    println!(
        "Step 2: Generating contracts for {} platforms\n",
        platforms.len()
    );

    // Generate contracts for the first statute on multiple platforms
    let demo_statute = &statutes[0];
    println!(
        "   Target Statute: {} ({})\n",
        demo_statute.title, demo_statute.id
    );

    for (platform, name) in &platforms {
        println!("   {}", "-".repeat(70));
        println!("   Platform: {}", name);
        println!("   {}", "-".repeat(70));

        let generator = ContractGenerator::new(*platform);
        match generator.generate(demo_statute) {
            Ok(contract) => {
                println!("   Contract Name: {}", contract.name);
                println!("   Platform: {:?}", contract.platform);
                if contract.abi.is_some() {
                    println!("   ABI: Available");
                }
                println!("\n   Source Preview:");
                print_contract_preview(&contract.source, 15);
            }
            Err(e) => {
                println!("   Error: {:?}", e);
            }
        }
        println!();
    }

    // Batch generation demo
    println!("{}", "=".repeat(80));
    println!("Step 3: Batch Contract Generation\n");

    let solidity_gen = ContractGenerator::new(TargetPlatform::Solidity);
    let batch_results = solidity_gen.generate_batch(&statutes);

    println!("   Generated {} Solidity contracts:\n", batch_results.len());
    for result in batch_results {
        match result {
            Ok(contract) => {
                let lines = contract.source.lines().count();
                println!("   [OK] {} - {} lines", contract.name, lines);
            }
            Err(e) => {
                println!("   [ERR] {:?}", e);
            }
        }
    }

    // Platform comparison summary
    println!();
    println!("{}", "=".repeat(80));
    println!("   PLATFORM COMPARISON SUMMARY");
    println!("{}", "=".repeat(80));
    println!();
    println!("   | Platform       | VM Type  | Language   | Use Case                    |");
    println!("   |----------------|----------|------------|------------------------------|");
    println!("   | Solidity       | EVM      | Solidity   | Ethereum, Polygon, BSC       |");
    println!("   | Vyper          | EVM      | Python-ish | Security-focused EVM         |");
    println!("   | Rust/WASM      | WASM     | Rust       | General WASM platforms       |");
    println!("   | Ink!           | WASM     | Rust       | Substrate/Polkadot           |");
    println!("   | Move           | MoveVM   | Move       | Aptos, Sui                   |");
    println!("   | Cairo          | Cairo VM | Cairo      | StarkNet (ZK L2)             |");
    println!("   | CosmWasm       | WASM     | Rust       | Cosmos ecosystem             |");
    println!("   | Solana         | BPF      | Rust       | Solana                       |");
    println!("   | NEAR           | WASM     | Rust       | NEAR Protocol                |");
    println!("   | TON (FunC)     | TVM      | FunC       | TON/Telegram                 |");
    println!();
    println!("   Key Insight:");
    println!("   Only deterministic statutes (no JudicialDiscretion) can be converted.");
    println!("   This enforces the Legalis principle: 'Code is Law, but Law preserves Narrative'");
    println!();

    Ok(())
}
