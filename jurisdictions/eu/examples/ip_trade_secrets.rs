//! EU Trade Secrets Example - Directive (EU) 2016/943
//!
//! Demonstrates trade secret protection validation and misappropriation analysis under EU law.

use legalis_eu::intellectual_property::{
    AcquisitionMethod, TradeSecret, TradeSecretCharacteristics,
};

fn main() {
    println!("=== EU Trade Secrets Validation Examples ===\n");
    println!(
        "Directive (EU) 2016/943 on the protection of undisclosed know-how and business information\n"
    );

    // Scenario 1: Valid trade secret with adequate protection
    println!("Scenario 1: Well-Protected Trade Secret");
    println!("----------------------------------------");
    let algorithm = TradeSecret::new()
        .with_description("Proprietary compression algorithm reducing data size by 80%")
        .with_holder("DataTech Solutions GmbH")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: true,              // Not publicly known
            has_commercial_value: true,   // Provides competitive advantage
            reasonable_steps_taken: true, // Protected by measures
        })
        .add_protective_measure("Non-disclosure agreements with all employees")
        .add_protective_measure("Access control: source code on air-gapped system")
        .add_protective_measure("Encryption at rest with AES-256")
        .add_protective_measure("Need-to-know access policy")
        .add_protective_measure("Exit interviews reminding of confidentiality obligations");

    match algorithm.validate() {
        Ok(validation) => {
            println!("✅ PROTECTED as trade secret under Directive 2016/943");
            println!("   Three-part test (Article 2(1)):");
            println!("     ✓ (a) Information is secret");
            println!("     ✓ (b) Has commercial value because secret");
            println!("     ✓ (c) Reasonable steps taken to keep secret");
            println!(
                "\n   Protective measures adequate: {}",
                validation.protective_measures_adequate
            );
            if validation.recommendations.is_empty() {
                println!("   No additional recommendations");
            }
        }
        Err(e) => println!("❌ Not protectable: {}", e),
    }

    // Scenario 2: Trade secret lacking commercial value
    println!("\n\nScenario 2: Information Lacking Commercial Value");
    println!("-------------------------------------------------");
    let personal_info = TradeSecret::new()
        .with_description("Employee's personal preferences for coffee")
        .with_holder("Individual")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: true,
            has_commercial_value: false, // No commercial value
            reasonable_steps_taken: true,
        });

    match personal_info.validate() {
        Ok(_) => println!("✅ Protected"),
        Err(e) => {
            println!("❌ NOT PROTECTED as trade secret");
            println!("   Reason: {}", e);
            println!(
                "   Article 2(1)(b): Information must have commercial value because it is secret"
            );
        }
    }

    // Scenario 3: Information not kept secret
    println!("\n\nScenario 3: Insufficient Protective Measures");
    println!("---------------------------------------------");
    let recipe = TradeSecret::new()
        .with_description("Restaurant's signature sauce recipe")
        .with_holder("Family Restaurant")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: true,
            has_commercial_value: true,
            reasonable_steps_taken: false, // No protective measures
        });

    match recipe.validate() {
        Ok(_) => println!("✅ Protected"),
        Err(e) => {
            println!("❌ NOT PROTECTED as trade secret");
            println!("   Reason: {}", e);
            println!("   Article 2(1)(c): Must take reasonable steps to keep information secret");
            println!("\n   Examples of reasonable steps:");
            println!("     - Non-disclosure agreements");
            println!("     - Physical access restrictions");
            println!("     - Password protection");
            println!("     - Marking documents as 'Confidential'");
        }
    }

    // Scenario 4: Publicly known information
    println!("\n\nScenario 4: Information Already Public");
    println!("---------------------------------------");
    let public_info = TradeSecret::new()
        .with_description("Information available in published patent")
        .with_holder("Company")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: false, // Publicly available
            has_commercial_value: true,
            reasonable_steps_taken: true,
        });

    match public_info.validate() {
        Ok(_) => println!("✅ Protected"),
        Err(e) => {
            println!("❌ NOT PROTECTED as trade secret");
            println!("   Reason: {}", e);
            println!("   Article 2(1)(a): Information must not be generally known");
            println!("   Once published in patent, no longer secret");
        }
    }

    // Scenario 5: Insufficient protective measures (warning)
    println!("\n\nScenario 5: Minimal Protection (Needs Improvement)");
    println!("---------------------------------------------------");
    let customer_list = TradeSecret::new()
        .with_description("Curated customer list with contact details")
        .with_holder("Sales Co")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: true,
            has_commercial_value: true,
            reasonable_steps_taken: true,
        })
        .add_protective_measure("Basic NDA with employees");

    match customer_list.validate() {
        Ok(validation) => {
            println!("⚠️  TECHNICALLY PROTECTED, but protection is weak");
            println!("   Three-part test passed, but:");
            if !validation.recommendations.is_empty() {
                for rec in &validation.recommendations {
                    println!("   Recommendation: {}", rec);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Misappropriation Analysis
    println!("\n\n=== Misappropriation Analysis (Articles 3-4) ===\n");

    let protected_secret = TradeSecret::new()
        .with_description("Manufacturing process for high-efficiency solar panels")
        .with_holder("GreenEnergy Corp")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: true,
            has_commercial_value: true,
            reasonable_steps_taken: true,
        })
        .add_protective_measure("Restricted access to facility")
        .add_protective_measure("NDA with all personnel")
        .add_protective_measure("Confidentiality clauses in employment contracts");

    // Scenario 6: Unlawful acquisition - unauthorized access
    println!("Scenario 6: Unlawful Acquisition via Unauthorized Access");
    println!("---------------------------------------------------------");
    let analysis1 =
        protected_secret.analyze_misappropriation(AcquisitionMethod::UnauthorizedAccess);

    println!("Acquisition method: Competitor broke into facility and photographed equipment");
    println!("Analysis:");
    if analysis1.is_unlawful {
        println!("  ❌ UNLAWFUL ACQUISITION (Article 4)");
        println!("  Applicable articles:");
        for article in &analysis1.applicable_articles {
            println!("    - {}", article);
        }
        println!("\n  Available remedies:");
        for remedy in &analysis1.remedies_available {
            println!("    - {}", remedy);
        }
    }

    // Scenario 7: Unlawful acquisition - breach of NDA
    println!("\n\nScenario 7: Unlawful Acquisition via Breach of Confidentiality");
    println!("---------------------------------------------------------------");
    let analysis2 = protected_secret.analyze_misappropriation(AcquisitionMethod::Breach);

    println!("Acquisition method: Former employee disclosed information despite NDA");
    println!("Analysis:");
    if analysis2.is_unlawful {
        println!("  ❌ UNLAWFUL ACQUISITION (Article 4(3)(b))");
        println!("  Breach of confidentiality agreement");
        println!("\n  Holder can seek:");
        for remedy in &analysis2.remedies_available {
            println!("    - {}", remedy);
        }
    }

    // Scenario 8: Lawful acquisition - independent discovery
    println!("\n\nScenario 8: Lawful Acquisition via Independent Discovery");
    println!("---------------------------------------------------------");
    let analysis3 =
        protected_secret.analyze_misappropriation(AcquisitionMethod::IndependentDiscovery);

    println!("Acquisition method: Competitor independently developed same process through R&D");
    println!("Analysis:");
    if !analysis3.is_unlawful {
        println!("  ✅ LAWFUL ACQUISITION (Article 3(1)(a))");
        println!("  Independent discovery or creation is permitted");
        println!("  No remedies available to original holder");
        for article in &analysis3.applicable_articles {
            println!("    {}", article);
        }
    }

    // Scenario 9: Lawful acquisition - reverse engineering
    println!("\n\nScenario 9: Lawful Acquisition via Reverse Engineering");
    println!("-------------------------------------------------------");
    let analysis4 =
        protected_secret.analyze_misappropriation(AcquisitionMethod::ReverseEngineering);

    println!("Acquisition method: Competitor purchased product and reverse-engineered it");
    println!("Analysis:");
    if !analysis4.is_unlawful {
        println!("  ✅ LAWFUL ACQUISITION (Article 3(1)(b))");
        println!("  Reverse engineering of lawfully acquired product is permitted");
        println!("  Directive explicitly protects this right");
        println!(
            "  Note: If product was acquired unlawfully, reverse engineering would also be unlawful"
        );
    }

    // Scenario 10: Lawful acquisition - observation of public product
    println!("\n\nScenario 10: Lawful Acquisition via Observation");
    println!("------------------------------------------------");
    let analysis5 =
        protected_secret.analyze_misappropriation(AcquisitionMethod::ObservationOfPublicProduct);

    println!("Acquisition method: Observed publicly displayed product at trade show");
    println!("Analysis:");
    if !analysis5.is_unlawful {
        println!("  ✅ LAWFUL ACQUISITION (Article 3(1)(b))");
        println!("  Observation or examination of publicly available product is permitted");
        println!("  Holder cannot prevent this");
    }

    println!("\n\n=== Summary ===");
    println!("EU Trade Secrets Directive 2016/943 provides:");
    println!("\n1. Three-Part Test for Protection (Article 2(1)):");
    println!("   a) Information is secret (not generally known)");
    println!("   b) Has commercial value because it is secret");
    println!("   c) Subject to reasonable steps to keep it secret");
    println!("\n2. Lawful Acquisition (Article 3):");
    println!("   • Independent discovery or creation");
    println!("   • Reverse engineering of lawfully acquired products");
    println!("   • Observation of products in public circulation");
    println!("\n3. Unlawful Acquisition (Article 4):");
    println!("   • Unauthorized access to documents/materials");
    println!("   • Breach of confidentiality agreement");
    println!("   • Inducing breach of confidentiality");
    println!("\n4. Remedies (Articles 12-13):");
    println!("   • Injunctions to prevent use/disclosure");
    println!("   • Damages for economic harm");
    println!("   • Recall or destruction of infringing goods");
    println!("\nBest practices for protection:");
    println!("  ✓ Implement multiple protective measures");
    println!("  ✓ Use NDAs with employees and business partners");
    println!("  ✓ Restrict physical and digital access");
    println!("  ✓ Mark documents as confidential");
    println!("  ✓ Conduct exit interviews");
    println!("  ✓ Regular review and update of security measures");
}
