//! Consumer Rights Act 2015 - Tiered Remedies Example
//!
//! Demonstrates the three-tier remedy system for goods under CRA 2015.
//!
//! Tier 1: Short-term right to reject (30 days) - s.22
//! Tier 2: Repair or replacement (one attempt) - s.23
//! Tier 3: Price reduction or final rejection - s.24

use chrono::Utc;
use legalis_uk::consumer_rights::*;

fn main() {
    println!("=== Consumer Rights Act 2015 - Tiered Remedies ===\n");
    println!("Demonstrating the three-tier remedy system for goods\n");
    println!("====================================================\n");

    // Example 1: Short-term right to reject (within 30 days)
    example_1_short_term_reject();

    // Example 2: Repair or replacement (after 30 days)
    example_2_repair_or_replacement();

    // Example 3: Price reduction or final rejection (after failed repair)
    example_3_price_reduction_final_reject();

    // Example 4: Satisfactory quality breach
    example_4_satisfactory_quality();

    // Example 5: Fitness for purpose breach
    example_5_fitness_for_purpose();

    // Example 6: Unfair contract terms
    example_6_unfair_terms();
}

fn example_1_short_term_reject() {
    println!("Example 1: Short-Term Right to Reject (Tier 1)");
    println!("===============================================\n");

    let contract = GoodsContract {
        description: "Samsung 55\" 4K TV".to_string(),
        price_gbp: 499.99,
        purchase_date: Utc::now().date_naive() - chrono::Duration::days(15),
        trader: Trader {
            name: "Electronics Warehouse Ltd".to_string(),
            address: "Retail Park, Leeds, LS11 5LN".to_string(),
            contact: "sales@electronics.co.uk".to_string(),
            company_number: Some("12345678".to_string()),
        },
        consumer: Consumer {
            name: "Emma Wilson".to_string(),
            address: "42 Oak Lane, Leeds, LS6 1AA".to_string(),
            contact: "emma@example.com".to_string(),
        },
        statutory_rights: vec![GoodsStatutoryRight::SatisfactoryQuality],
        remedy_stage: None,
    };

    println!("Purchase Details:");
    println!("  Product: {}", contract.description);
    println!("  Price: £{:.2}", contract.price_gbp);
    println!("  Purchase date: {}", contract.purchase_date);
    println!(
        "  Days since purchase: {}",
        (Utc::now().date_naive() - contract.purchase_date).num_days()
    );
    println!();

    println!("Defect Discovered:");
    println!("  TV has multiple dead pixels and backlight bleeding");
    println!();

    if contract.can_short_term_reject() {
        println!("✅ SHORT-TERM RIGHT TO REJECT AVAILABLE (CRA 2015 s.22)");
        println!();
        println!("Consumer Rights:");
        println!("  • Full refund (100% of purchase price)");
        println!("  • No deduction for use");
        println!("  • Must exercise within 30 days of delivery");
        println!(
            "  • Deadline: {}",
            contract.purchase_date + chrono::Duration::days(30)
        );
        println!();
        println!("Trader Obligations:");
        println!("  • Must provide refund within 14 days");
        println!("  • Refund same method as payment");
        println!("  • Consumer may deduct cost of return");
        println!();
    }
}

fn example_2_repair_or_replacement() {
    println!("Example 2: Repair or Replacement (Tier 2)");
    println!("==========================================\n");

    let contract = GoodsContract {
        description: "Bosch Washing Machine WAN28100GB".to_string(),
        price_gbp: 379.00,
        purchase_date: Utc::now().date_naive() - chrono::Duration::days(45),
        trader: Trader {
            name: "Appliance Direct".to_string(),
            address: "Industrial Estate, Manchester, M17 1LP".to_string(),
            contact: "support@appliancedirect.co.uk".to_string(),
            company_number: Some("87654321".to_string()),
        },
        consumer: Consumer {
            name: "James Smith".to_string(),
            address: "15 Park View, Manchester, M20 2AA".to_string(),
            contact: "james@example.com".to_string(),
        },
        statutory_rights: vec![GoodsStatutoryRight::SatisfactoryQuality],
        remedy_stage: None,
    };

    println!("Purchase Details:");
    println!("  Product: {}", contract.description);
    println!("  Price: £{:.2}", contract.price_gbp);
    println!("  Purchase date: {}", contract.purchase_date);
    println!(
        "  Days since purchase: {}",
        (Utc::now().date_naive() - contract.purchase_date).num_days()
    );
    println!();

    println!("Defect Discovered:");
    println!("  Drum not spinning, error code E18 (water pump fault)");
    println!();

    if !contract.can_short_term_reject() {
        println!("❌ Short-term right to reject EXPIRED (30 days passed)");
        println!();
        println!("✅ REPAIR OR REPLACEMENT AVAILABLE (CRA 2015 s.23)");
        println!();
        println!("Consumer Choice (s.23(2)):");
        println!("  Option A: REPAIR the goods");
        println!("    • Trader must repair within reasonable time");
        println!("    • Must not cause significant inconvenience");
        println!("    • Free of charge");
        println!();
        println!("  Option B: REPLACEMENT goods");
        println!("    • Trader must replace with equivalent");
        println!("    • Must not cause significant inconvenience");
        println!("    • Free of charge");
        println!();
        println!("Conditions (s.23(3)):");
        println!("  • Remedy must be POSSIBLE (not impossible)");
        println!("  • Must not be DISPROPORTIONATE");
        println!("    (compared to other available remedy)");
        println!();
        println!("Recommendation: Request REPAIR (pump replacement)");
        println!("  Likely less disruptive than full replacement");
        println!();
    }
}

fn example_3_price_reduction_final_reject() {
    println!("Example 3: Price Reduction or Final Rejection (Tier 3)");
    println!("=======================================================\n");

    println!("Scenario: Laptop with keyboard defect");
    println!("  • Purchase price: £800");
    println!("  • Repair attempted: Keyboard replacement (failed)");
    println!("  • Defect persists: Keys still sticking after repair");
    println!();

    println!("✅ FINAL REMEDY AVAILABLE (CRA 2015 s.24)");
    println!();
    println!("Triggers for Final Remedy (s.24(5)):");
    println!("  ✓ Repair/replacement not possible (or disproportionate)");
    println!("  ✓ Trader failed to repair/replace within reasonable time");
    println!("  ✓ Repair/replacement would cause significant inconvenience");
    println!();

    println!("Consumer Choice:");
    println!();
    println!("  Option A: PRICE REDUCTION (s.24(5))");
    println!("    • Appropriate reduction reflecting loss in value");
    println!("    • Keep the goods");
    println!("    • Formula: Reduction = (Defect severity %) × Purchase price");
    println!("    • Example: 15% severity = £120 reduction");
    println!();
    println!("  Option B: FINAL RIGHT TO REJECT (s.24(8))");
    println!("    • Reject goods and get refund");
    println!("    • Deduction for use MAY apply");
    println!("    • Formula: Refund = Price - Deduction for use");
    println!("    • Example: £800 - £150 use = £650 refund");
    println!();

    println!("Deduction for Use (s.24(10)):");
    println!("  Only if consumer has had use of goods");
    println!("  Factors considered:");
    println!("    • Length of time goods were conforming");
    println!("    • Use consumer has had");
    println!("    • Nature of goods");
    println!();
}

fn example_4_satisfactory_quality() {
    println!("Example 4: Breach of Satisfactory Quality (s.9)");
    println!("================================================\n");

    println!("Product: Nike Air Max Trainers");
    println!("Price: £110");
    println!("Defect: Sole separated from upper after 2 weeks");
    println!();

    let result = validate_satisfactory_quality(
        "Nike Air Max Trainers",
        "Sole separated from upper after 2 weeks of normal use",
        110.0,
        false,
    );

    match result {
        Err(e) => {
            println!("❌ STATUTORY RIGHT BREACHED");
            println!();
            println!("{}", e);
            println!();
        }
        Ok(_) => println!("✅ Satisfactory quality met"),
    }

    println!("CRA 2015 s.9(2) - Quality Factors:");
    println!("  ✓ Fitness for common purposes");
    println!("  ✓ Appearance and finish");
    println!("  ✓ Freedom from minor defects");
    println!("  ✓ Safety");
    println!("  ✓ Durability ← FAILED");
    println!();
}

fn example_5_fitness_for_purpose() {
    println!("Example 5: Breach of Fitness for Particular Purpose (s.10)");
    println!("===========================================================\n");

    println!("Scenario:");
    println!("  Customer: \"I need a laptop for heavy video editing\"");
    println!("  Salesperson: \"This model is perfect for that\"");
    println!("  Product: Dell Inspiron 15 (4GB RAM, integrated graphics)");
    println!("  Reality: Crashes when running video editing software");
    println!();

    let result = validate_fit_for_purpose(
        "Heavy video editing (Adobe Premiere Pro)",
        false,
        "Laptop has insufficient RAM (4GB) and no dedicated GPU. \
         Crashes when rendering video. Requires minimum 16GB RAM and \
         dedicated graphics card for video editing.",
    );

    match result {
        Err(e) => {
            println!("❌ STATUTORY RIGHT BREACHED");
            println!();
            println!("{}", e);
            println!();
        }
        Ok(_) => println!("✅ Fit for purpose"),
    }

    println!("CRA 2015 s.10 Requirements:");
    println!("  1. Consumer made known particular purpose ✓");
    println!("  2. Consumer relied on trader's skill/judgment ✓");
    println!("  3. Goods must be fit for that purpose ✗ FAILED");
    println!();
}

fn example_6_unfair_terms() {
    println!("Example 6: Unfair Contract Terms (Part 2)");
    println!("==========================================\n");

    println!("Term 1: Exclusion of Liability for Death/Injury");
    println!("Term text: \"We exclude all liability for death or personal injury\"");
    println!();

    let assessment1 = UnfairTermAssessment {
        contrary_to_good_faith: true,
        significant_imbalance: true,
        detriment_to_consumer: true,
        on_grey_list: Some(GreyListItem::ExcludeLiabilityDeathInjury),
        transparent_and_prominent: false,
    };

    match validate_unfair_term(
        &assessment1,
        "We exclude all liability for death or personal injury",
    ) {
        Err(_) => {
            println!("❌ UNFAIR TERM - NOT BINDING ON CONSUMER");
            println!();
            println!("Grey List Item: Schedule 2 Para 1");
            println!("  Attempts to exclude/limit liability for death/personal injury");
            println!("  resulting from act/omission of trader");
            println!();
            println!("Effect: Term has NO LEGAL EFFECT (s.62(1))");
            println!();
        }
        Ok(_) => println!("✅ Term is fair"),
    }

    println!("Term 2: Automatic Contract Renewal");
    println!("Term text: \"Contract automatically renews unless you cancel 90 days in advance\"");
    println!();

    let assessment2 = UnfairTermAssessment {
        contrary_to_good_faith: true,
        significant_imbalance: true,
        detriment_to_consumer: true,
        on_grey_list: Some(GreyListItem::AutomaticRenewal),
        transparent_and_prominent: false,
    };

    match validate_unfair_term(
        &assessment2,
        "Contract automatically renews unless you cancel 90 days in advance",
    ) {
        Err(_) => {
            println!("❌ UNFAIR TERM - NOT BINDING ON CONSUMER");
            println!();
            println!("Grey List Item: Schedule 2 Para 8");
            println!("  Automatically extending contract unless consumer");
            println!("  objects in unreasonably short time");
            println!();
            println!("Effect: Term has NO LEGAL EFFECT (s.62(1))");
            println!();
        }
        Ok(_) => println!("✅ Term is fair"),
    }

    println!("CRA 2015 s.62 - Unfairness Test:");
    println!("  A term is unfair if:");
    println!("    • Contrary to good faith, AND");
    println!("    • Causes significant imbalance, AND");
    println!("    • To detriment of consumer");
    println!();
    println!("  Exception: Core terms (price/subject matter)");
    println!("    IF transparent AND prominent");
    println!();
}
