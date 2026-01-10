//! Contract Breach and Damages Example (Pflichtverletzung und Schadensersatz)
//!
//! Demonstrates breach of contract and damages claims under German BGB
//! (§§280-283, §§323-326 BGB).

use chrono::Utc;
use legalis_de::bgb::schuldrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Contract Law - Breach and Damages ===\n");
    println!("BGB Schuldrecht - Pflichtverletzung und Schadensersatz\n");

    // =========================================================================
    // Example 1: Non-Performance (Nichterfüllung)
    // =========================================================================
    println!("📋 Example 1: Non-Performance with Damages Claim");
    println!("----------------------------------------\n");

    let breach = Breach {
        contract_id: "C-2024-001".to_string(),
        breaching_party: "Seller GmbH".to_string(),
        breach_type: BreachType::NonPerformance,
        occurred_at: Utc::now(),
        fault: FaultLevel::OrdinaryNegligence,
        description: "Failed to deliver goods within agreed timeframe".to_string(),
    };

    println!("Breach Details:");
    println!("  Contract: {}", breach.contract_id);
    println!("  Breaching party: {}", breach.breaching_party);
    println!("  Type: Non-Performance (Nichterfüllung)");
    println!("  Fault: Ordinary Negligence (einfache Fahrlässigkeit)");
    println!("  Description: {}", breach.description);
    println!();

    match validate_breach(&breach) {
        Ok(()) => println!("✅ Breach established per §280 BGB"),
        Err(e) => println!("❌ Breach validation failed: {}", e),
    }
    println!();

    // Create damages claim
    let damages_claim = DamagesClaim {
        contract_id: Some("C-2024-001".to_string()),
        claimant: "Buyer AG".to_string(),
        respondent: "Seller GmbH".to_string(),
        legal_basis: DamagesLegalBasis::GeneralBreach,
        damage_types: vec![DamageType::Positive, DamageType::Consequential],
        amount_claimed: Capital::from_euros(25_000),
        fault_proven: true,
        causation_proven: true,
    };

    println!("Damages Claim (§280 BGB):");
    println!("  Claimant: {}", damages_claim.claimant);
    println!("  Respondent: {}", damages_claim.respondent);
    println!("  Amount: €{:.2}", damages_claim.amount_claimed.to_euros());
    println!("  Legal basis: General breach (§280 Abs. 1 BGB)");
    println!();

    match validate_damages_claim(&damages_claim) {
        Ok(()) => {
            println!("✅ Damages claim valid!");
            println!("   Requirements met:");
            println!("   ✓ Schuldverhältnis (obligation relationship) exists");
            println!("   ✓ Pflichtverletzung (breach of duty)");
            println!("   ✓ Verschulden (fault) proven");
            println!("   ✓ Schaden (damage) established");
            println!("   ✓ Kausalität (causation) proven");
        }
        Err(e) => println!("❌ Damages claim invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Delay in Performance (Verzug)
    // =========================================================================
    println!("📋 Example 2: Delay in Performance (Verzug §286 BGB)");
    println!("----------------------------------------\n");

    let delay_breach = Breach {
        contract_id: "C-2024-002".to_string(),
        breaching_party: "Construction Co. KG".to_string(),
        breach_type: BreachType::Delay,
        occurred_at: Utc::now(),
        fault: FaultLevel::OrdinaryNegligence,
        description: "Construction delayed by 60 days beyond agreed completion date".to_string(),
    };

    println!("Delay Situation:");
    println!("  Breaching party: {}", delay_breach.breaching_party);
    println!("  Delay: 60 days beyond agreed date");
    println!("  Consequences: Additional rental costs for temporary housing");
    println!();

    let delay_damages = DamagesClaim {
        contract_id: Some("C-2024-002".to_string()),
        claimant: "Homeowner".to_string(),
        respondent: "Construction Co. KG".to_string(),
        legal_basis: DamagesLegalBasis::Delay,
        damage_types: vec![DamageType::Positive, DamageType::Consequential],
        amount_claimed: Capital::from_euros(12_000),
        fault_proven: true,
        causation_proven: true,
    };

    println!("Damages for Delay (§280 Abs. 2, §286 BGB):");
    println!("  Temporary rental costs: €12,000");
    println!("  Causation: Directly caused by construction delay");
    println!();

    match validate_damages_claim(&delay_damages) {
        Ok(()) => {
            println!("✅ Delay damages claim valid!");
            println!("   §286 BGB: Debtor in default (Schuldnerverzug)");
            println!("   §280 Abs. 2 BGB: Damages for delay");
            println!("   Claimant entitled to compensation for delay damages");
        }
        Err(e) => println!("❌ Claim invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 3: Damages in Lieu of Performance (§281 BGB)
    // =========================================================================
    println!("📋 Example 3: Damages in Lieu of Performance (§281 BGB)");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  Seller failed to deliver specialized machinery");
    println!("  Buyer set grace period (Nachfrist): 14 days");
    println!("  Grace period expired without performance");
    println!("  Buyer now seeks damages instead of performance");
    println!();

    let remedy = Remedy {
        contract_id: "C-2024-003".to_string(),
        claimant: "Manufacturing GmbH".to_string(),
        respondent: "Machinery Supplier AG".to_string(),
        remedy_type: RemedyType::DamagesInLieu,
        damages_amount: Some(Capital::from_euros(150_000)),
        grace_period_days: Some(14),
        grace_period_expired: true,
    };

    println!("Remedy Request:");
    println!("  Type: Damages in lieu of performance");
    println!(
        "  Grace period: {} days (expired)",
        remedy.grace_period_days.unwrap()
    );
    println!(
        "  Amount: €{:.2}",
        remedy.damages_amount.as_ref().unwrap().to_euros()
    );
    println!();

    match validate_remedy(&remedy) {
        Ok(()) => {
            println!("✅ Damages in lieu claim valid per §281 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Grace period set (Nachfrist)");
            println!("   ✓ Grace period expired without performance");
            println!("   ✓ Buyer released from obligation to accept performance");
            println!("   ✓ Entitled to full compensation (Schadensersatz statt der Leistung)");
        }
        Err(e) => println!("❌ Remedy invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 4: Termination After Grace Period (§323 BGB)
    // =========================================================================
    println!("📋 Example 4: Termination for Non-Performance (Rücktritt §323 BGB)");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  Service provider failed to complete work");
    println!("  Client set reasonable grace period: 21 days");
    println!("  No performance after grace period");
    println!("  Client exercises right to terminate contract");
    println!();

    let termination = Termination {
        contract_id: "C-2024-004".to_string(),
        terminating_party: "Client Corp.".to_string(),
        grounds: TerminationGrounds::NonPerformanceAfterGracePeriod,
        grace_period_set_and_expired: true,
        declared_at: Utc::now(),
        effective: true,
    };

    println!("Termination:");
    println!("  Terminating party: {}", termination.terminating_party);
    println!("  Grounds: Non-performance after grace period");
    println!("  Grace period: Set and expired");
    println!();

    match validate_termination(&termination) {
        Ok(()) => {
            println!("✅ Termination valid per §323 Abs. 1 BGB!");
            println!("   Effects (§§346-354 BGB):");
            println!("   - Mutual obligations to return performance (Rückgewähr)");
            println!("   - Contract obligations cease");
            println!("   - Damages claim may still exist (§325 BGB)");
            println!("   - Use of item must be compensated");
        }
        Err(e) => println!("❌ Termination invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 5: Immediate Termination (No Grace Period Required)
    // =========================================================================
    println!("📋 Example 5: Immediate Termination - Serious Breach");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  Contractor seriously and finally refuses to perform");
    println!("  No grace period required (§323 Abs. 2 Nr. 1 BGB)");
    println!();

    let immediate_termination = Termination {
        contract_id: "C-2024-005".to_string(),
        terminating_party: "Property Owner".to_string(),
        grounds: TerminationGrounds::RefusalToPerform,
        grace_period_set_and_expired: false, // No grace period needed!
        declared_at: Utc::now(),
        effective: true,
    };

    println!("Termination:");
    println!("  Grounds: Serious and final refusal to perform");
    println!("  Grace period: Not required");
    println!();

    match validate_termination(&immediate_termination) {
        Ok(()) => {
            println!("✅ Immediate termination valid per §323 Abs. 2 Nr. 1 BGB!");
            println!("   Grace period unnecessary when:");
            println!("   - Debtor seriously and finally refuses to perform");
            println!("   - Performance impossible (§323 Abs. 4 BGB)");
            println!("   - Fixed-date transaction deadline missed (§323 Abs. 2 Nr. 2 BGB)");
            println!("   - Special circumstances justify immediate termination");
        }
        Err(e) => println!("❌ Termination invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 6: Minor Breach (No Termination Allowed)
    // =========================================================================
    println!("📋 Example 6: Minor Breach - Termination Excluded");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  Delivery 2 days late (contract value: €100,000)");
    println!("  Breach is minor (unerhebliche Pflichtverletzung)");
    println!("  §323 Abs. 5 S. 2 BGB excludes termination");
    println!();

    let minor_termination = Termination {
        contract_id: "C-2024-006".to_string(),
        terminating_party: "Buyer".to_string(),
        grounds: TerminationGrounds::MinorBreach,
        grace_period_set_and_expired: true,
        declared_at: Utc::now(),
        effective: false,
    };

    match validate_termination(&minor_termination) {
        Ok(()) => println!("✅ Termination valid"),
        Err(e) => {
            println!("❌ Termination invalid: {}", e);
            println!("   §323 Abs. 5 S. 2 BGB: Minor breach excludes termination");
            println!("   Remedy available: Damages for delay (§286 BGB)");
            println!("   But: No right to terminate contract");
            println!("   Rationale: Proportionality principle");
        }
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: German Contract Breach Remedies ===");
    println!();
    println!("§280 BGB - General Damages:");
    println!("  Requirements: Breach + Fault + Damage + Causation");
    println!("  Types: §280 Abs. 1 (general), §280 Abs. 2 (delay)");
    println!();
    println!("§281 BGB - Damages in Lieu:");
    println!("  Requires: Grace period set and expired");
    println!("  Effect: Damages instead of performance");
    println!();
    println!("§323 BGB - Termination (Rücktritt):");
    println!("  Standard: Grace period required (§323 Abs. 1)");
    println!("  Exceptions: Refusal, impossibility, fixed-date (§323 Abs. 2)");
    println!("  Limitation: Minor breach excluded (§323 Abs. 5 S. 2)");
    println!();
    println!("§§346-354 BGB - Effects of Termination:");
    println!("  - Mutual return obligations");
    println!("  - Value compensation for use");
    println!("  - Damages claims may coexist (§325 BGB)");
    println!();
    println!("✅ All examples demonstrate correct BGB breach remedies!");
}
