//! Company Formation Example (会社設立の例)
//!
//! This example demonstrates how to create and validate articles of incorporation
//! for a Japanese stock company (株式会社 - Kabushiki-gaisha).
//!
//! # Usage
//! ```bash
//! cargo run --example company-formation-kaisha
//! ```

use chrono::Utc;
use legalis_jp::commercial_law::*;

fn main() {
    println!("=== Japanese Company Formation Example ===\n");
    println!("会社設立の例 - Company Formation Example\n");

    // Example 1: Valid Stock Company Formation
    println!("📋 Example 1: Valid Stock Company (株式会社)");
    println!("{}", "=".repeat(60));

    let valid_articles = ArticlesOfIncorporation {
        company_name: "テクノロジーソリューションズ株式会社".to_string(),
        business_purposes: vec![
            "ソフトウェアの開発及び販売".to_string(),
            "ITコンサルティング業務".to_string(),
            "クラウドサービスの提供".to_string(),
            "前各号に附帯する一切の業務".to_string(),
        ],
        head_office_location: "東京都渋谷区渋谷1丁目1番1号".to_string(),
        authorized_shares: Some(10_000),
        capital: Capital::new(10_000_000), // ¥10,000,000 capital
        fiscal_year_end_month: 3,          // March (typical for Japanese companies)
        incorporators: vec![
            Incorporator {
                name: "山田太郎".to_string(),
                address: "東京都渋谷区".to_string(),
                shares_subscribed: Some(7_000),
                investment_amount_jpy: 7_000_000,
            },
            Incorporator {
                name: "佐藤花子".to_string(),
                address: "東京都新宿区".to_string(),
                shares_subscribed: Some(3_000),
                investment_amount_jpy: 3_000_000,
            },
        ],
        establishment_date: Some(Utc::now()),
    };

    println!("Company Name: {}", valid_articles.company_name);
    println!("Capital: {}", valid_articles.capital);
    println!("Authorized Shares: {:?}", valid_articles.authorized_shares);
    println!(
        "Fiscal Year End: {}月",
        valid_articles.fiscal_year_end_month
    );
    println!("\nBusiness Purposes:");
    for (i, purpose) in valid_articles.business_purposes.iter().enumerate() {
        println!("  {}. {}", i + 1, purpose);
    }
    println!("\nIncorporators:");
    for inc in &valid_articles.incorporators {
        println!(
            "  • {} (Investment: ¥{})",
            inc.name, inc.investment_amount_jpy
        );
    }

    // Validate
    match validate_articles_of_incorporation(&valid_articles, CompanyType::StockCompany) {
        Ok(()) => println!("\n✅ Validation Result: PASSED - Articles are valid!"),
        Err(e) => println!("\n❌ Validation Result: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 2: LLC (Limited Liability Company) Formation
    println!("📋 Example 2: LLC Formation (合同会社)");
    println!("{}", "=".repeat(60));

    let llc_articles = ArticlesOfIncorporation {
        company_name: "グローバルソリューションズ合同会社".to_string(),
        business_purposes: vec![
            "経営コンサルティング業務".to_string(),
            "マーケティング支援サービス".to_string(),
        ],
        head_office_location: "大阪府大阪市北区梅田1丁目1番1号".to_string(),
        authorized_shares: None,          // LLCs don't have shares
        capital: Capital::new(3_000_000), // ¥3,000,000 capital
        fiscal_year_end_month: 12,        // December
        incorporators: vec![Incorporator {
            name: "鈴木一郎".to_string(),
            address: "大阪府大阪市".to_string(),
            shares_subscribed: None,
            investment_amount_jpy: 3_000_000,
        }],
        establishment_date: Some(Utc::now()),
    };

    println!("Company Name: {}", llc_articles.company_name);
    println!("Capital: {}", llc_articles.capital);
    println!("Fiscal Year End: {}月", llc_articles.fiscal_year_end_month);

    match validate_articles_of_incorporation(&llc_articles, CompanyType::LLC) {
        Ok(()) => println!("\n✅ Validation Result: PASSED - LLC articles are valid!"),
        Err(e) => println!("\n❌ Validation Result: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 3: Invalid Formation (Missing Company Type Suffix)
    println!("📋 Example 3: Invalid Formation - Missing Suffix");
    println!("{}", "=".repeat(60));

    let invalid_articles = ArticlesOfIncorporation {
        company_name: "テクノロジーソリューションズ".to_string(), // Missing "株式会社"!
        business_purposes: vec!["ソフトウェア開発".to_string()],
        head_office_location: "東京都".to_string(),
        authorized_shares: Some(10_000),
        capital: Capital::new(1_000_000),
        fiscal_year_end_month: 3,
        incorporators: vec![Incorporator {
            name: "Test".to_string(),
            address: "Tokyo".to_string(),
            shares_subscribed: Some(10_000),
            investment_amount_jpy: 1_000_000,
        }],
        establishment_date: None,
    };

    println!("Company Name: {} (INVALID)", invalid_articles.company_name);

    match validate_articles_of_incorporation(&invalid_articles, CompanyType::StockCompany) {
        Ok(()) => println!("\n✅ Validation Result: PASSED"),
        Err(e) => println!(
            "\n❌ Validation Result: FAILED (as expected)\n   Error: {}",
            e
        ),
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 4: Capital Classification
    println!("📋 Example 4: Capital Classification");
    println!("{}", "=".repeat(60));

    let capitals = vec![
        Capital::new(50_000_000),  // Small company
        Capital::new(100_000_000), // Small company threshold
        Capital::new(500_000_000), // Below large company
        Capital::new(600_000_000), // Large company
    ];

    for capital in capitals {
        println!("\nCapital: {}", capital);
        println!("  Small Company (≤¥100M): {}", capital.is_small_company());
        println!("  Large Company (>¥500M): {}", capital.is_large_company());
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 5: Commercial Statutory Interest Calculation
    println!("📋 Example 5: Commercial Statutory Interest (6% p.a.)");
    println!("{}", "=".repeat(60));

    let principal = 10_000_000; // ¥10,000,000
    let periods = vec![30, 90, 180, 365]; // days

    println!("\nPrincipal: ¥{}", principal);
    println!(
        "Statutory Rate: {}% per annum",
        COMMERCIAL_STATUTORY_INTEREST_RATE * 100.0
    );

    for days in periods {
        let interest = calculate_commercial_statutory_interest(principal, days);
        println!("  {} days: ¥{}", days, interest);
    }

    println!("\n{}", "=".repeat(60));
    println!("\n✨ Company Formation Examples Complete!");
    println!("   All validations demonstrate proper compliance with");
    println!("   the Companies Act (会社法) and Commercial Code (商法).\n");
}
