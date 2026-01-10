//! Tort Claim Example (Unerlaubte Handlung nach §823 Abs. 1 BGB)
//!
//! Demonstrates tort claims under German BGB using the builder pattern for §823 Abs. 1 BGB.

use chrono::Utc;
use legalis_de::bgb::unerlaubte_handlungen::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Tort Law - §823 Abs. 1 BGB Examples ===\n");
    println!("BGB Unerlaubte Handlungen - Tort Claims with Builder Pattern\n");

    // =========================================================================
    // Example 1: Traffic Accident - Bodily Injury
    // =========================================================================
    println!("📋 Example 1: Traffic Accident (Verkehrsunfall)");
    println!("----------------------------------------\n");

    let claim = TortClaim823_1Builder::new()
        .tortfeasor("Hans Müller", "Frankfurt am Main")
        .injured_party("Anna Schmidt", "Berlin")
        .protected_interest(ProtectedInterest::Body)
        .violation_direct_injury("Broken leg from car collision", "Severe")
        .verschulden(Verschulden::EinfacheFahrlassigkeit)
        .widerrechtlich(true)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(8_000))
        .damages_pain_suffering(Capital::from_euros(5_000))
        .damages_lost_income(Capital::from_euros(3_000))
        .causation_established(true)
        .build()
        .unwrap();

    println!("Traffic Accident Claim:");
    println!("  Tortfeasor: {}", claim.tortfeasor.name);
    println!("  Injured party: {}", claim.injured_party.name);
    println!(
        "  Protected interest: {:?} (§823 Abs. 1 BGB)",
        claim.protected_interest
    );
    println!("  Fault level: {:?}", claim.verschulden);
    println!();
    println!("Damages Breakdown:");
    println!(
        "  Medical expenses: €{:.2}",
        claim.damages.medical_expenses.as_ref().unwrap().to_euros()
    );
    println!(
        "  Pain & suffering (§253 BGB): €{:.2}",
        claim
            .damages
            .pain_and_suffering
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!(
        "  Lost income (§252 BGB): €{:.2}",
        claim.damages.lost_income.as_ref().unwrap().to_euros()
    );
    println!("  Total: €{:.2}", claim.damages.total.to_euros());
    println!();

    match validate_tort_claim_823_1(&claim) {
        Ok(()) => {
            println!("✅ Tort claim valid per §823 Abs. 1 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Protected interest violated (Body - Körper)");
            println!("   ✓ Fault established (ordinary negligence)");
            println!("   ✓ Unlawfulness (Widerrechtlichkeit)");
            println!("   ✓ Causation established (Kausalität)");
            println!("   ✓ Damage proven (Schaden)");
        }
        Err(e) => println!("❌ Claim invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Property Damage - Negligent Destruction
    // =========================================================================
    println!("📋 Example 2: Property Damage (Sachbeschädigung)");
    println!("----------------------------------------\n");

    let property_claim = TortClaim823_1Builder::new()
        .tortfeasor_legal_entity("Construction Co. GmbH", "Munich")
        .injured_party("Max Mustermann", "Augsburg")
        .protected_interest(ProtectedInterest::Property)
        .violation_property_damage(
            "Excavator damaged garden fence and shed",
            Capital::from_euros(14_000),
        )
        .verschulden(Verschulden::GrobeFahrlassigkeit)
        .widerrechtlich(true)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(12_000))
        .damages_consequential(Capital::from_euros(2_000))
        .causation_established(true)
        .notes("Construction company failed to verify property boundaries")
        .build()
        .unwrap();

    println!("Property Damage Claim:");
    println!(
        "  Tortfeasor: {} (Legal entity)",
        property_claim.tortfeasor.name
    );
    println!("  Injured party: {}", property_claim.injured_party.name);
    println!(
        "  Protected interest: {:?} (§823 Abs. 1 BGB)",
        property_claim.protected_interest
    );
    println!("  Fault level: Gross negligence (grobe Fahrlässigkeit)");
    println!();
    println!("Damages:");
    println!(
        "  Property damage: €{:.2}",
        property_claim
            .damages
            .property_damage
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!(
        "  Consequential damages: €{:.2}",
        property_claim
            .damages
            .consequential_damages
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!("  Total: €{:.2}", property_claim.damages.total.to_euros());
    println!();

    match validate_tort_claim_823_1(&property_claim) {
        Ok(()) => {
            println!("✅ Property damage claim valid!");
            println!("   Gross negligence (grobe Fahrlässigkeit) established");
            println!("   Property (Eigentum) protected under §823 Abs. 1 BGB");
        }
        Err(e) => println!("❌ Claim invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 3: Claim with Justification (Self-Defense)
    // =========================================================================
    println!("📋 Example 3: Justification - Self-Defense (Notwehr)");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  Defendant struck attacker during attempted robbery");
    println!("  Attacker claims bodily injury");
    println!("  Defendant invokes self-defense (§32 StGB - Notwehr)");
    println!();

    let mut justified_claim = TortClaim823_1Builder::new()
        .tortfeasor("Defender", "Hamburg")
        .injured_party("Attacker", "Hamburg")
        .protected_interest(ProtectedInterest::Body)
        .violation_direct_injury("Bruised face from defensive strike", "Minor")
        .verschulden(Verschulden::Vorsatz)
        .widerrechtlich(true)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(500))
        .causation_established(true)
        .build()
        .unwrap();

    // Add justification ground
    justified_claim.justification = Some(Justification::SelfDefense);

    println!("Attempted Claim:");
    println!("  Protected interest: Body (Körper)");
    println!("  Damage: €{:.2}", justified_claim.damages.total.to_euros());
    println!("  Justification: Self-defense (Notwehr)");
    println!();

    match validate_tort_claim_823_1(&justified_claim) {
        Ok(()) => println!("✅ Claim valid (unexpected)"),
        Err(e) => {
            println!("❌ Claim fails: {}", e);
            println!();
            println!("Explanation:");
            println!("  Unlawfulness (Widerrechtlichkeit) is negated by self-defense");
            println!("  §32 StGB permits necessary defensive action against unlawful attack");
            println!("  Tort claim under §823 Abs. 1 BGB therefore fails");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 4: Personality Rights Violation
    // =========================================================================
    println!("📋 Example 4: Personality Rights (Allgemeines Persönlichkeitsrecht)");
    println!("----------------------------------------\n");

    let personality_claim = TortClaim823_1Builder::new()
        .tortfeasor_legal_entity("Tabloid Press GmbH", "Berlin")
        .injured_party("Celebrity", "Munich")
        .protected_interest(ProtectedInterest::OtherRight)
        .violation_other_rights("Unauthorized publication of private photos violating general personality right")
        .verschulden(Verschulden::Vorsatz)
        .widerrechtlich(true)
        .incident_date(Utc::now())
        .damages_pain_suffering(Capital::from_euros(20_000))
        .causation_established(true)
        .notes("General personality right (allgemeines Persönlichkeitsrecht) protected as 'sonstiges Recht'")
        .build()
        .unwrap();

    println!("Personality Rights Claim:");
    println!("  Tortfeasor: {}", personality_claim.tortfeasor.name);
    println!("  Injured party: {}", personality_claim.injured_party.name);
    println!("  Right violated: General personality right (Art. 2 Abs. 1, Art. 1 Abs. 1 GG)");
    println!("  Fault: Intent (Vorsatz)");
    println!();
    println!("Damages:");
    println!(
        "  Pain & suffering (§253 Abs. 2 BGB): €{:.2}",
        personality_claim
            .damages
            .pain_and_suffering
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!();

    match validate_tort_claim_823_1(&personality_claim) {
        Ok(()) => {
            println!("✅ Personality rights claim valid!");
            println!("   'Sonstiges Recht' includes general personality right");
            println!("   §253 Abs. 2 BGB allows pain & suffering for personality rights");
            println!("   Derived from Art. 2 Abs. 1, Art. 1 Abs. 1 GG");
        }
        Err(e) => println!("❌ Claim invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 5: Intentional Tort under §826 BGB
    // =========================================================================
    println!("📋 Example 5: Intentional Immoral Conduct (§826 BGB)");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  Competitor deliberately spread false business information");
    println!("  Intent to harm victim's business reputation");
    println!("  Conduct contrary to good morals (sittenwidrig)");
    println!();

    let immoral_claim = TortClaim826 {
        tortfeasor: TortParty {
            name: "Rival Company AG".to_string(),
            address: Some("Frankfurt".to_string()),
            is_natural_person: false,
        },
        injured_party: TortParty {
            name: "Victim GmbH".to_string(),
            address: Some("Berlin".to_string()),
            is_natural_person: false,
        },
        conduct: "Deliberately spread false rumors about financial insolvency".to_string(),
        sittenwidrig: true,
        schadigungsvorsatz: true,
        incident_date: Utc::now(),
        damages: DamageClaim {
            property_damage: Some(Capital::from_euros(50_000)),
            personal_injury: None,
            pain_and_suffering: None,
            lost_income: Some(Capital::from_euros(100_000)),
            medical_expenses: None,
            consequential_damages: Some(Capital::from_euros(30_000)),
            total: Capital::from_euros(180_000),
        },
        causation_established: true,
        notes: Some(
            "§826 BGB applies - broader than §823 Abs. 1, covers pure economic loss".to_string(),
        ),
    };

    println!("§826 BGB Claim:");
    println!("  Tortfeasor: {}", immoral_claim.tortfeasor.name);
    println!("  Injured party: {}", immoral_claim.injured_party.name);
    println!("  Conduct: {}", immoral_claim.conduct);
    println!("  Intent to harm: Yes (Schädigungsvorsatz)");
    println!("  Contrary to good morals: Yes (Sittenwidrigkeit)");
    println!();
    println!("Damages:");
    println!(
        "  Lost business: €{:.2}",
        immoral_claim
            .damages
            .lost_income
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!(
        "  Property damage: €{:.2}",
        immoral_claim
            .damages
            .property_damage
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!(
        "  Consequential: €{:.2}",
        immoral_claim
            .damages
            .consequential_damages
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!("  Total: €{:.2}", immoral_claim.damages.total.to_euros());
    println!();

    match validate_tort_claim_826(&immoral_claim) {
        Ok(()) => {
            println!("✅ §826 BGB claim valid!");
            println!("   Requirements met:");
            println!("   ✓ Intent to cause damage (Schädigungsvorsatz)");
            println!("   ✓ Contrary to good morals (Sittenwidrigkeit)");
            println!("   ✓ Causation established");
            println!("   ✓ Damage proven");
            println!();
            println!("   §826 BGB advantages over §823 Abs. 1:");
            println!("   - No enumerated protected interests required");
            println!("   - Covers pure economic loss");
            println!("   - Broader protection for intentional wrongful conduct");
        }
        Err(e) => println!("❌ Claim invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 6: Invalid Claim - No Fault Proven
    // =========================================================================
    println!("📋 Example 6: Invalid Claim - No Fault (Kein Verschulden)");
    println!("----------------------------------------\n");

    let no_fault_claim = TortClaim823_1Builder::new()
        .tortfeasor("Driver", "Munich")
        .injured_party("Pedestrian", "Munich")
        .protected_interest(ProtectedInterest::Body)
        .violation_direct_injury("Minor collision", "Minor")
        .verschulden(Verschulden::KeinVerschulden)
        .widerrechtlich(true)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(1_000))
        .causation_established(true)
        .notes("Unavoidable accident (unabwendbares Ereignis)")
        .build()
        .unwrap();

    println!("Scenario: Unavoidable accident with no fault");
    println!();

    match validate_tort_claim_823_1(&no_fault_claim) {
        Ok(()) => println!("✅ Claim valid (unexpected)"),
        Err(e) => {
            println!("❌ Claim fails: {}", e);
            println!();
            println!("Explanation:");
            println!("  §823 Abs. 1 BGB requires fault (Verschulden)");
            println!("  No fault = no liability (kein Verschulden = keine Haftung)");
            println!("  Exception: Strict liability statutes (Gefährdungshaftung)");
            println!("  Example: §7 StVG (motor vehicle liability) - not in §823 Abs. 1");
        }
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: German Tort Law (BGB) ===");
    println!();
    println!("Requirements for Tort Liability:");
    println!("  1. Violation of protected interest:");
    println!("     - Life (Leben)");
    println!("     - Body (Koerper)");
    println!("     - Health (Gesundheit)");
    println!("     - Freedom (Freiheit)");
    println!("     - Property (Eigentum)");
    println!("     - Other right (sonstiges Recht)");
    println!();
    println!("  2. Fault (Verschulden):");
    println!("     - Intent (Vorsatz)");
    println!("     - Gross negligence (grobe Fahrlaessigkeit)");
    println!("     - Ordinary negligence (einfache Fahrlaessigkeit)");
    println!();
    println!("  3. Unlawfulness (Widerrechtlichkeit):");
    println!("     - Presumed when protected interest violated");
    println!("     - Negated by justification grounds");
    println!();
    println!("  4. Causation (Kausalitaet):");
    println!("     - Factual causation (conditio sine qua non)");
    println!("     - Legal causation (Adaequanztheorie)");
    println!();
    println!("  5. Damage (Schaden):");
    println!("     - Property damage (Vermoegensschaden)");
    println!("     - Personal injury");
    println!("     - Pain and suffering (limited cases)");
    println!();
    println!("Alternative: Intentional Immoral Conduct:");
    println!("  - Broader protection");
    println!("  - Covers pure economic loss");
    println!("  - Requires: Intent + Contrary to good morals");
    println!();
    println!("All examples demonstrate correct BGB tort law principles!");
}
