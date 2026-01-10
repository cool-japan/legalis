//! EU Trademark Registration Example
//!
//! Demonstrates EU Trademark Regulation (EU) 2017/1001 validation.

use legalis_eu::intellectual_property::*;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║     EU Trademark Registration Validation                 ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Scenario 1: Valid distinctive word mark
    println!("━━━ Scenario 1: Distinctive Word Mark ━━━\n");

    let trademark1 = EuTrademark::new()
        .with_mark_text("INNOVATECH")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("InnovaTech GmbH")
        .add_nice_class(9)
        .unwrap() // Electronics
        .add_nice_class(42)
        .unwrap() // IT services
        .add_goods_services("Computer software, mobile applications")
        .add_goods_services("Software development services");

    match trademark1.validate() {
        Ok(validation) => {
            println!("✅ Trademark: INNOVATECH");
            println!("   Type: Word mark");
            println!("   Classes: 9 (Goods), 42 (Services)");
            println!("   Registrable: {}", validation.is_registrable);
            println!(
                "   Distinctiveness: {}\n",
                validation.distinctiveness_established
            );
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Scenario 2: Descriptive mark without secondary meaning
    println!("━━━ Scenario 2: Descriptive Mark (Rejected) ━━━\n");

    let trademark2 = EuTrademark::new()
        .with_mark_text("FAST SOFTWARE")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("Example Corp")
        .add_nice_class(9)
        .unwrap()
        .with_descriptive(true); // Mark is descriptive

    match trademark2.validate() {
        Ok(_) => println!("✅ Trademark registered"),
        Err(e) => {
            println!("❌ Trademark: FAST SOFTWARE");
            println!("   Rejection reason: {}", e);
            println!("   Legal basis: Article 7(1)(b) - Lack of distinctiveness\n");
        }
    }

    // Scenario 3: Descriptive mark with acquired distinctiveness
    println!("━━━ Scenario 3: Descriptive Mark with Secondary Meaning ━━━\n");

    let trademark3 = EuTrademark::new()
        .with_mark_text("WINDOWS")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("Software Giant Corp")
        .add_nice_class(9)
        .unwrap()
        .with_descriptive(true)
        .with_secondary_meaning(true); // Acquired distinctiveness

    match trademark3.validate() {
        Ok(_validation) => {
            println!("✅ Trademark: WINDOWS");
            println!("   Status: Registrable despite being descriptive");
            println!("   Reason: Acquired distinctiveness (secondary meaning)");
            println!("   Article 7(3): Mark has become distinctive through use\n");
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Scenario 4: Generic mark (always rejected)
    println!("━━━ Scenario 4: Generic Mark (Rejected) ━━━\n");

    let trademark4 = EuTrademark::new()
        .with_mark_text("COMPUTER")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("Example GmbH")
        .add_nice_class(9)
        .unwrap()
        .with_generic(true);

    match trademark4.validate() {
        Ok(_) => println!("✅ Trademark registered"),
        Err(e) => {
            println!("❌ Trademark: COMPUTER");
            println!("   Rejection reason: {}", e);
            println!("   Legal basis: Article 7(1)(d) - Generic terms cannot be monopolized\n");
        }
    }

    // Scenario 5: Invalid Nice class
    println!("━━━ Scenario 5: Invalid Nice Class ━━━\n");

    let result = EuTrademark::new()
        .with_mark_text("EXAMPLE")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("Example Corp")
        .add_nice_class(99); // Invalid - Nice classes are 1-45

    match result {
        Ok(_) => println!("✅ Valid Nice class"),
        Err(e) => {
            println!("❌ Invalid Nice classification");
            println!("   Error: {}", e);
            println!("   Valid range: Classes 1-45\n");
        }
    }

    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║               EU TRADEMARK SUMMARY                        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("EU Trademark Regulation (EU) 2017/1001\n");

    println!("Key Requirements:");
    println!("  ✅ Distinctive (Article 7) - not descriptive or generic");
    println!("  ✅ Valid Nice Classification (Classes 1-45)");
    println!("  ✅ Clear goods/services specification");
    println!("  ✅ Valid representation of the mark\n");

    println!("Grounds for Refusal (Article 7):");
    println!("  (a) Not capable of graphical representation");
    println!("  (b) Lack of distinctiveness");
    println!("  (c) Descriptive of goods/services");
    println!("  (d) Generic/customary terms");
    println!("  (e) Deceptive marks");
    println!("  (f) Contrary to public policy/morality\n");

    println!("Overcoming Descriptiveness:");
    println!("  Article 7(3): Mark may become distinctive through use");
    println!("  Requires proof of secondary meaning (acquired distinctiveness)\n");

    println!("Protection:");
    println!("  Duration: 10 years from filing date");
    println!("  Renewable: Indefinitely (every 10 years)");
    println!("  Coverage: All 27 EU member states + unitary right\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
