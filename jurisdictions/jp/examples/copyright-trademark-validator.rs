//! Copyright and Trademark Validation Example (著作権・商標検証例)
//!
//! This example demonstrates copyright and trademark validation according to
//! the Copyright Act (著作権法) and Trademark Act (商標法).
//!
//! # Usage
//! ```bash
//! cargo run --example copyright-trademark-validator
//! ```

use chrono::{Duration, Utc};
use legalis_jp::intellectual_property::*;

fn main() {
    println!("=== Copyright & Trademark Validation Example ===\n");
    println!("著作権・商標検証例 - Copyright & Trademark Validation\n");

    // ========================================================================
    // PART 1: COPYRIGHT LAW (著作権法)
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: COPYRIGHT LAW (著作権法)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 1: Valid Literary Work
    println!("📚 Example 1: Literary Work (言語の著作物)");
    println!("{}", "=".repeat(70));

    let novel = CopyrightedWork {
        title: "春の物語 (Spring Story)".to_string(),
        authors: vec!["作家名 (Author Name)".to_string()],
        category: WorkCategory::Literary,
        creation_date: Utc::now() - Duration::days(365),
        first_publication_date: Some(Utc::now() - Duration::days(300)),
        copyright_holder: "出版社株式会社 (Publisher Inc)".to_string(),
        is_work_for_hire: false,
        derivative_source: None,
    };

    println!("Title: {}", novel.title);
    println!("Author(s): {}", novel.authors.join(", "));
    println!("Category: {:?}", novel.category);
    println!("Copyright Holder: {}", novel.copyright_holder);
    println!("Work for Hire: {}", novel.is_work_for_hire);
    println!(
        "Protection Period: {} years after author's death",
        COPYRIGHT_PROTECTION_YEARS
    );

    match validate_copyrighted_work(&novel) {
        Ok(()) => println!("\n✅ Validation: PASSED - Work is copyrightable!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 2: Program Work (Software)
    println!("💻 Example 2: Program Work (プログラムの著作物)");
    println!("{}", "=".repeat(70));

    let software = CopyrightedWork {
        title: "TaskManager Pro v2.0".to_string(),
        authors: vec!["Development Team".to_string()],
        category: WorkCategory::Program,
        creation_date: Utc::now() - Duration::days(730),
        first_publication_date: Some(Utc::now() - Duration::days(700)),
        copyright_holder: "ソフトウェア株式会社".to_string(),
        is_work_for_hire: true, // Work for hire
        derivative_source: None,
    };

    println!("Title: {}", software.title);
    println!("Category: {:?}", software.category);
    println!(
        "Work for Hire: {} (職務著作 - Article 15)",
        software.is_work_for_hire
    );
    println!("Copyright Holder: {}", software.copyright_holder);

    match validate_copyrighted_work(&software) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Fair Use - Quotation
    println!("📝 Example 3: Fair Use - Quotation (引用 - Article 32)");
    println!("{}", "=".repeat(70));

    let quotation_use = "This research paper quotes three paragraphs from the original work \
                         for the purpose of critical analysis and commentary";

    println!("Use Description:");
    println!("  {}", quotation_use);
    println!("\nFair Use Type: Quotation (Article 32)");
    println!("Requirements:");
    println!("  ✓ For criticism, research, or reporting");
    println!("  ✓ Quoted portion is clearly distinguished");
    println!("  ✓ Source is properly attributed");

    match validate_fair_use(FairUseType::Quotation, quotation_use) {
        Ok(()) => println!("\n✅ Fair Use: VALID - Quotation appears proper!"),
        Err(e) => println!("\n❌ Fair Use: INVALID - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 4: Invalid Fair Use - Commercial Use
    println!("❌ Example 4: Invalid Fair Use - Commercial Purpose");
    println!("{}", "=".repeat(70));

    let commercial_use = "Using copyrighted music for commercial advertisement and public broadcasting to promote products";

    println!("Use Description:");
    println!("  {}", commercial_use);
    println!("\nClaimed Fair Use: Private Use (Article 30)");
    println!("Issue: Commercial and public use violates private use exception");

    match validate_fair_use(FairUseType::PrivateUse, commercial_use) {
        Ok(()) => println!("\n✅ Fair Use: VALID"),
        Err(e) => println!("\n❌ Fair Use: INVALID (as expected)\n   Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 5: Copyright Infringement Claim
    println!("⚖️  Example 5: Copyright Infringement Claim (著作権侵害主張)");
    println!("{}", "=".repeat(70));

    let infringement = CopyrightInfringement {
        original_work: novel.clone(),
        alleged_infringing_work: "Unauthorized reproduction and distribution of the novel"
            .to_string(),
        rights_infringed: vec![EconomicRight::Reproduction, EconomicRight::Distribution],
        infringement_date: Utc::now(),
        fair_use_claim: None,
        estimated_damages_jpy: Some(5_000_000),
    };

    println!("Original Work: {}", infringement.original_work.title);
    println!(
        "Alleged Infringement: {}",
        infringement.alleged_infringing_work
    );
    println!("Rights Infringed:");
    for right in &infringement.rights_infringed {
        println!("  - {:?}", right);
    }
    if let Some(damages) = infringement.estimated_damages_jpy {
        println!("Estimated Damages: ¥{}", damages);
    }

    match validate_copyright_infringement(&infringement) {
        Ok(()) => println!("\n✅ Infringement claim appears valid for legal action"),
        Err(e) => println!("\n❌ Infringement claim issue: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // ========================================================================
    // PART 2: TRADEMARK LAW (商標法)
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: TRADEMARK LAW (商標法)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 6: Valid Trademark Application
    println!("®️  Example 6: Valid Trademark Application (商標出願)");
    println!("{}", "=".repeat(70));

    let trademark_app = TrademarkApplication {
        application_number: "商願2020-123456".to_string(),
        filing_date: Utc::now(),
        trademark_representation: "INNOVATECH™".to_string(),
        trademark_type: TrademarkType::Word,
        applicant: "イノベーション株式会社".to_string(),
        designated_classes: vec![9, 42], // Software & IT services
        designated_goods_services: vec![
            "コンピュータソフトウェア (Computer software)".to_string(),
            "ITコンサルティング (IT consulting)".to_string(),
        ],
    };

    println!("Application Number: {}", trademark_app.application_number);
    println!("Trademark: {}", trademark_app.trademark_representation);
    println!("Type: {:?}", trademark_app.trademark_type);
    println!("Applicant: {}", trademark_app.applicant);
    println!(
        "Designated Classes: {:?} (Nice Classification)",
        trademark_app.designated_classes
    );
    println!("Designated Goods/Services:");
    for goods in &trademark_app.designated_goods_services {
        println!("  - {}", goods);
    }

    match validate_trademark_application(&trademark_app) {
        Ok(()) => println!("\n✅ Validation: PASSED - Application meets requirements!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 7: Trademark Registration
    println!("📋 Example 7: Trademark Registration (商標登録)");
    println!("{}", "=".repeat(70));

    let trademark_reg = TrademarkRegistration {
        registration_number: "登録第6000000号".to_string(),
        registration_date: Utc::now() - Duration::days(365 * 3), // 3 years ago
        application: trademark_app.clone(),
        renewal_count: 0,
        last_renewal_date: None,
    };

    println!("Registration Number: {}", trademark_reg.registration_number);
    println!(
        "Registration Date: {}",
        trademark_reg.registration_date.format("%Y-%m-%d")
    );
    println!("Renewal Period: {} years", TRADEMARK_RENEWAL_YEARS);
    println!(
        "Years Until Renewal: {:.1}",
        trademark_reg.years_until_renewal()
    );
    println!("Is Valid: {}", trademark_reg.is_valid());

    match validate_trademark_registration(&trademark_reg) {
        Ok(()) => println!("\n✅ Validation: PASSED - Registration is currently valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 8: Trademark Similarity Assessment
    println!("🔍 Example 8: Trademark Similarity Assessment (商標類似性評価)");
    println!("{}", "=".repeat(70));

    let test_cases = vec![
        ("ACME", "ACME", "Identical marks"),
        ("ACME", "ACNE", "Highly similar marks"),
        ("ACME", "ACMA", "Similar marks"),
        ("ACME", "WXYZ", "Not similar marks"),
        ("INNOVATECH", "INNOVATEK", "Highly similar marks"),
    ];

    for (mark1, mark2, description) in test_cases {
        let similarity = assess_trademark_similarity(mark1, mark2);
        println!("\n{} vs {}: {:?}", mark1, mark2, similarity);
        println!("  Description: {}", description);

        if matches!(
            similarity,
            SimilarityLevel::Identical | SimilarityLevel::HighlySimilar
        ) {
            println!("  ⚠️  Risk: Confusion likely - may constitute infringement");
        }
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 9: Invalid Trademark - Merely Descriptive
    println!("❌ Example 9: Invalid Trademark - Merely Descriptive (記述的商標)");
    println!("{}", "=".repeat(70));

    let descriptive_app = TrademarkApplication {
        application_number: "商願2020-999999".to_string(),
        filing_date: Utc::now(),
        trademark_representation: "BEST".to_string(), // Too descriptive!
        trademark_type: TrademarkType::Word,
        applicant: "Company".to_string(),
        designated_classes: vec![35],
        designated_goods_services: vec!["Business services".to_string()],
    };

    println!(
        "Trademark: {} (PROBLEMATIC)",
        descriptive_app.trademark_representation
    );
    println!("Issue: Merely descriptive term lacks distinctiveness");
    println!("Article 3-1-3: Marks consisting solely of common descriptions");

    match validate_trademark_application(&descriptive_app) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED (as expected)\n   Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 10: Invalid Nice Classification
    println!("❌ Example 10: Invalid Nice Classification (無効な区分指定)");
    println!("{}", "=".repeat(70));

    let invalid_class_app = TrademarkApplication {
        application_number: "商願2020-888888".to_string(),
        filing_date: Utc::now(),
        trademark_representation: "VALIDMARK".to_string(),
        trademark_type: TrademarkType::Word,
        applicant: "Company".to_string(),
        designated_classes: vec![50], // Invalid! Must be 1-45
        designated_goods_services: vec!["Goods".to_string()],
    };

    println!("Trademark: {}", invalid_class_app.trademark_representation);
    println!(
        "Designated Classes: {:?} (INVALID)",
        invalid_class_app.designated_classes
    );
    println!("Valid Range: 1-45 (Nice Classification)");

    match validate_trademark_application(&invalid_class_app) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED (as expected)\n   Error: {}", e),
    }

    println!("\n{}", "=".repeat(70));
    println!("\n✨ Copyright & Trademark Validation Examples Complete!");
    println!("   All examples demonstrate proper compliance with:");
    println!("   - Copyright Act (著作権法)");
    println!("   - Trademark Act (商標法)\n");
}
