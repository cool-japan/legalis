//! Patent Application Validation Example (特許出願検証例)
//!
//! This example demonstrates patent application validation according to
//! the Patent Act (特許法).
//!
//! # Usage
//! ```bash
//! cargo run --example patent-application-validator
//! ```

use chrono::{Duration, Utc};
use legalis_jp::intellectual_property::*;

fn main() {
    println!("=== Patent Application Validation Example ===\n");
    println!("特許出願検証例 - Patent Application Validation\n");

    // Example 1: Valid Patent Application
    println!("📋 Example 1: Valid Patent Application (有効な特許出願)");
    println!("{}", "=".repeat(70));

    let valid_application = PatentApplication {
        application_number: "2020-123456".to_string(),
        filing_date: Utc::now(),
        title: "エネルギー効率改善装置 (Energy Efficiency Improvement Device)".to_string(),
        inventors: vec!["山田太郎".to_string(), "佐藤花子".to_string()],
        applicants: vec!["テクノロジー株式会社".to_string()],
        category: InventionCategory::Product,
        claims: vec![
            "1. エネルギー消費を30%削減する装置であって、センサーと制御ユニットを備える。"
                .to_string(),
            "2. 前記制御ユニットは、機械学習アルゴリズムにより最適化を行う。".to_string(),
        ],
        abstract_text:
            "本発明は、機械学習技術を用いてエネルギー消費を最適化する新規な装置に関する。"
                .to_string(),
        priority_date: None,
        examination_requested: true,
    };

    println!(
        "Application Number: {}",
        valid_application.application_number
    );
    println!("Title: {}", valid_application.title);
    println!("Inventors: {}", valid_application.inventors.join(", "));
    println!("Category: {:?}", valid_application.category);
    println!("Claims: {} items", valid_application.claims.len());
    println!(
        "Examination Requested: {}",
        valid_application.examination_requested
    );

    match validate_patent_application(&valid_application) {
        Ok(()) => println!("\n✅ Validation: PASSED - Application meets all requirements!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 2: Patent with Priority Claim
    println!("📋 Example 2: Patent with Priority Claim (優先権主張)");
    println!("{}", "=".repeat(70));

    let priority_application = PatentApplication {
        application_number: "2021-234567".to_string(),
        filing_date: Utc::now(),
        title: "Novel Semiconductor Device".to_string(),
        inventors: vec!["Researcher A".to_string()],
        applicants: vec!["Tech Corp".to_string()],
        category: InventionCategory::Product,
        claims: vec!["A semiconductor device comprising novel structure".to_string()],
        abstract_text:
            "This invention relates to a novel semiconductor device with improved characteristics"
                .to_string(),
        priority_date: Some(Utc::now() - Duration::days(300)), // 10 months ago
        examination_requested: false,
    };

    println!(
        "Application Number: {}",
        priority_application.application_number
    );
    println!(
        "Filing Date: {}",
        priority_application.filing_date.format("%Y-%m-%d")
    );
    if let Some(priority_date) = priority_application.priority_date {
        println!("Priority Date: {}", priority_date.format("%Y-%m-%d"));
        let days_diff = (priority_application.filing_date - priority_date).num_days();
        println!("Priority Period: {} days (must be ≤365)", days_diff);
    }

    match validate_patent_application(&priority_application) {
        Ok(()) => println!("\n✅ Validation: PASSED - Priority claim is valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Invalid Application - Insufficient Disclosure
    println!("📋 Example 3: Invalid Application - Insufficient Disclosure");
    println!("{}", "=".repeat(70));

    let invalid_application = PatentApplication {
        application_number: "2020-999999".to_string(),
        filing_date: Utc::now(),
        title: "Test Device".to_string(),
        inventors: vec!["Inventor".to_string()],
        applicants: vec!["Company".to_string()],
        category: InventionCategory::Product,
        claims: vec!["A device".to_string()], // Too vague!
        abstract_text: "Device".to_string(),  // Too short!
        priority_date: None,
        examination_requested: false,
    };

    println!(
        "Application Number: {}",
        invalid_application.application_number
    );
    println!(
        "Claims: {} (INSUFFICIENT)",
        invalid_application.claims.len()
    );
    println!(
        "Abstract Length: {} chars (INSUFFICIENT)",
        invalid_application.abstract_text.len()
    );

    match validate_patent_application(&invalid_application) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED (as expected)\n   Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 4: Patent Grant Validation
    println!("📋 Example 4: Valid Patent Grant (特許査定)");
    println!("{}", "=".repeat(70));

    let patent_grant = PatentGrant {
        patent_number: "特許第6000000号".to_string(),
        grant_date: Utc::now() - Duration::days(365 * 3), // 3 years ago
        application: valid_application.clone(),
        annual_fees_paid_until_year: 3,
    };

    println!("Patent Number: {}", patent_grant.patent_number);
    println!("Grant Date: {}", patent_grant.grant_date.format("%Y-%m-%d"));
    println!(
        "Years Since Grant: {:.1}",
        (Utc::now() - patent_grant.grant_date).num_days() as f64 / 365.0
    );
    println!(
        "Annual Fees Paid Until: Year {}",
        patent_grant.annual_fees_paid_until_year
    );
    println!(
        "Annual Fees Current: {}",
        patent_grant.are_annual_fees_current()
    );
    println!("Patent Valid: {}", patent_grant.is_valid());

    match validate_patent_grant(&patent_grant) {
        Ok(()) => println!("\n✅ Validation: PASSED - Patent is currently valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 5: Expired Patent
    println!("📋 Example 5: Expired Patent (特許権期間満了)");
    println!("{}", "=".repeat(70));

    let old_application = PatentApplication {
        application_number: "1990-000001".to_string(),
        filing_date: Utc::now() - Duration::days(365 * 25), // 25 years ago (expired!)
        title: "Old Invention".to_string(),
        inventors: vec!["Old Inventor".to_string()],
        applicants: vec!["Old Company".to_string()],
        category: InventionCategory::Method,
        claims: vec!["An old method for processing".to_string()],
        abstract_text: "This invention relates to an old method".to_string(),
        priority_date: None,
        examination_requested: true,
    };

    let expired_grant = PatentGrant {
        patent_number: "特許第3000000号".to_string(),
        grant_date: Utc::now() - Duration::days(365 * 23),
        application: old_application.clone(),
        annual_fees_paid_until_year: 20,
    };

    println!("Patent Number: {}", expired_grant.patent_number);
    println!(
        "Filing Date: {}",
        old_application.filing_date.format("%Y-%m-%d")
    );
    println!(
        "Years Since Filing: {:.1}",
        old_application.years_since_filing()
    );
    println!("Protection Period: {} years", PATENT_PROTECTION_YEARS);
    println!(
        "Is Expired: {}",
        old_application.is_protection_expired(expired_grant.grant_date)
    );

    match validate_patent_grant(&expired_grant) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED (as expected)\n   Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 6: Patent Infringement Check
    println!("📋 Example 6: Patent Infringement Check (特許権侵害チェック)");
    println!("{}", "=".repeat(70));

    let product_description = "A device with sensor and control unit using machine learning algorithms to optimize energy consumption and reduce usage by 30%";

    println!("Patent Claims:");
    for (i, claim) in patent_grant.application.claims.iter().enumerate() {
        println!("  {}. {}", i + 1, claim);
    }
    println!("\nProduct Description:");
    println!("  {}", product_description);

    match check_patent_infringement(&patent_grant, product_description) {
        Ok(()) => println!("\n✅ No obvious infringement detected"),
        Err(e) => println!("\n⚠️  Potential infringement:\n   {}", e),
    }

    println!("\n{}", "=".repeat(70));
    println!("\n✨ Patent Application Validation Examples Complete!");
    println!("   All examples demonstrate proper compliance with");
    println!("   the Patent Act (特許法) requirements.\n");
}
