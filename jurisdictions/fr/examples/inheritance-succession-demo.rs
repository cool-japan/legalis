//! Inheritance and Succession Example
//!
//! Demonstrates French inheritance law (Code civil - Droit des successions)
//! including succession rules, wills, and reserved portions (réserve héréditaire).

use chrono::NaiveDate;
use legalis_fr::inheritance::*;

fn main() {
    println!("=== French Inheritance Law Example ===\n");
    println!("Code civil - Droit des successions (Articles 720-894)\n");

    // Example 1: Simple succession with one child
    println!("📋 Example 1: Legal Succession with One Child");
    println!("─────────────────────────────────────────────\n");

    let deceased = Person::new("Jean Dupont".to_string(), 75);
    let child = Person::new("Marie Dupont".to_string(), 45);

    let heir = Heir::new(child, Relationship::Child)
        .with_reserved_portion(0.5) // 1 child = 1/2 reserved (Article 913)
        .with_actual_share(1.0); // Sole heir receives entire estate

    let succession = Succession::new(
        deceased.clone(),
        NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
    )
    .with_last_domicile("Paris, France".to_string())
    .with_heir(heir.clone());

    match validate_succession(&succession) {
        Ok(()) => {
            println!("✅ Succession: VALID");
            println!("   Deceased: {}", deceased.name);
            println!("   Domicile: {}", succession.last_domicile);
            println!(
                "   Death date: {} (Article 720 - succession opens)",
                succession.death_date
            );
            println!("\n   Heir: {}", heir.person.name);
            println!("   Relationship: {:?}", heir.relationship);
            println!("   Reserved portion: 50% (Article 913)");
            println!("   Actual share: 100% (sole heir)");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 2: Succession with multiple children
    println!("📋 Example 2: Succession with Three Children");
    println!("─────────────────────────────────────────\n");

    let deceased2 = Person::new("Pierre Martin".to_string(), 80);
    let child1 = Person::new("Sophie Martin".to_string(), 55);
    let child2 = Person::new("Thomas Martin".to_string(), 52);
    let child3 = Person::new("Claire Martin".to_string(), 48);

    // 3 children = 3/4 reserved total (Article 913)
    // Each child gets 1/3 of estate
    let heir1 = Heir::new(child1, Relationship::Child)
        .with_reserved_portion(0.25) // 1/4 each reserved
        .with_actual_share(1.0 / 3.0);

    let heir2 = Heir::new(child2, Relationship::Child)
        .with_reserved_portion(0.25)
        .with_actual_share(1.0 / 3.0);

    let heir3 = Heir::new(child3, Relationship::Child)
        .with_reserved_portion(0.25)
        .with_actual_share(1.0 / 3.0);

    let succession2 = Succession::new(
        deceased2.clone(),
        NaiveDate::from_ymd_opt(2024, 11, 1).unwrap(),
    )
    .with_last_domicile("Lyon, France".to_string())
    .with_heir(heir1.clone())
    .with_heir(heir2.clone())
    .with_heir(heir3.clone());

    match validate_succession(&succession2) {
        Ok(()) => {
            println!("✅ Succession: VALID");
            println!("   Deceased: {}", deceased2.name);
            println!("   Death date: {}", succession2.death_date);
            println!("\n   Total reserved portion: 75% (3 children - Article 913)");
            println!("   Available portion: 25% (quotité disponible)");
            println!("\n   Heirs (equal shares):");
            for (i, heir) in succession2.heirs.iter().enumerate() {
                println!(
                    "   {}. {}: {:.1}%",
                    i + 1,
                    heir.person.name,
                    heir.actual_share * 100.0
                );
            }
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 3: Succession with will (testament)
    println!("📋 Example 3: Succession with Holographic Will");
    println!("───────────────────────────────────────────────\n");

    let _testator = Person::new("Marie Leclerc".to_string(), 70);
    let sole_child = Person::new("Antoine Leclerc".to_string(), 40);
    let friend = Person::new("Claire Bernard".to_string(), 65);

    // Holographic will (testament olographe) - Article 970
    let will = Will::new(
        WillType::Holographic {
            handwritten: true,
            dated: true,
            signed: true,
        },
        "Marie Leclerc".to_string(),
        NaiveDate::from_ymd_opt(2023, 3, 15).unwrap(),
    )
    .with_disposition(Disposition {
        beneficiary: sole_child.name.clone(),
        disposition_type: DispositionType::General,
        description: "Reserved portion to my child".to_string(),
        value: Some(250_000), // €250,000 (50% of estate)
    })
    .with_disposition(Disposition {
        beneficiary: friend.name.clone(),
        disposition_type: DispositionType::General,
        description: "Available portion to my friend".to_string(),
        value: Some(250_000), // €250,000 (50% of estate)
    });

    match validate_will(&will) {
        Ok(()) => {
            println!("✅ Will: VALID");
            println!("   Type: Holographic (Article 970)");
            println!("   Testator: {}", will.testator);
            println!("   Date: {}", will.date);
            println!("\n   Requirements met:");
            if let WillType::Holographic {
                handwritten,
                dated,
                signed,
            } = will.will_type
            {
                println!("     ✅ Entirely handwritten: {}", handwritten);
                println!("     ✅ Dated: {}", dated);
                println!("     ✅ Signed: {}", signed);
            }
            println!("\n   Dispositions:");
            for disp in &will.dispositions {
                println!(
                    "     - To {}: {:?}",
                    disp.beneficiary, disp.disposition_type
                );
            }
            println!("\n   ✅ Respects reserved portion (1 child = 50%)");
        }
        Err(e) => println!("❌ Invalid will: {}", e),
    }

    println!("\n");

    // Example 4: Authentic will (testament authentique)
    println!("📋 Example 4: Authentic Will with Notary");
    println!("────────────────────────────────────────\n");

    let authentic_will = Will::new(
        WillType::Authentic {
            notary: "Maître Jean Dubois, Notaire à Paris".to_string(),
            witnesses: vec!["Bernard Laurent".to_string(), "Catherine Petit".to_string()],
        },
        "Robert Bernard".to_string(),
        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
    )
    .with_disposition(Disposition {
        beneficiary: "Hôpital Sainte-Anne".to_string(),
        disposition_type: DispositionType::Specific,
        description: "Charitable bequest - Donation of €100,000 to hospital".to_string(),
        value: Some(100_000), // €100,000
    });

    match validate_will(&authentic_will) {
        Ok(()) => {
            println!("✅ Authentic Will: VALID");
            println!("   Type: Authentic (Article 971)");
            if let WillType::Authentic { notary, witnesses } = &authentic_will.will_type {
                println!("   Notary: {}", notary);
                println!("   Witnesses: {} and {}", witnesses[0], witnesses[1]);
            }
            println!("\n   Advantages:");
            println!("     ✅ Maximum legal security");
            println!("     ✅ Difficult to contest");
            println!("     ✅ Preserved by notary");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Summary
    println!("════════════════════════════════════════════════════════");
    println!("📊 Summary: French Inheritance Law");
    println!("════════════════════════════════════════════════════════\n");

    println!("🏛️  Legal Succession (Succession légale):");
    println!("   - Article 720: Succession opens at death");
    println!("   - Article 731-746: Order of heirs");
    println!("   - Children inherit equally\n");

    println!("📜 Will Types:");
    println!("   1. Holographic (olographe) - Article 970");
    println!("      Must be handwritten, dated, and signed");
    println!("   2. Authentic (authentique) - Article 971");
    println!("      Notarized with two witnesses");
    println!("   3. Mystic (mystique) - Article 976");
    println!("      Sealed and presented to notary\n");

    println!("🛡️  Reserved Portion (Réserve héréditaire - Article 913):");
    println!("   - 1 child:  50% reserved, 50% available");
    println!("   - 2 children: 66.7% reserved, 33.3% available");
    println!("   - 3+ children: 75% reserved, 25% available");
    println!("   Protects descendants' inheritance rights\n");

    println!("🔑 Key Principles:");
    println!("   - Strong forced heirship (vs. Anglo-American law)");
    println!("   - Reserved portions mandatory (Article 913-917)");
    println!("   - Limited testamentary freedom");
    println!("   - Will formalities strictly enforced");
}
