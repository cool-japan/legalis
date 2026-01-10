//! Construction Business Act and Real Estate Transactions Act Example
//!
//! Demonstrates licensing and compliance checking for:
//! - Construction Business Act (建設業法 Act No. 100 of 1949)
//! - Real Estate Transactions Act (宅地建物取引業法 Act No. 176 of 1952)
//!
//! Run with:
//! ```bash
//! cargo run --example construction-real-estate-licensing
//! ```

use chrono::{NaiveDate, Utc};
use legalis_jp::construction_real_estate::{
    ConstructionBusinessLicense, ConstructionLicenseType, ConstructionType, LicensedAgent,
    LicensedBroker, Manager, ManagerQualification, Party, Property, PropertyType,
    RealEstateTransaction, TransactionType, validate_construction_license,
    validate_real_estate_transaction,
};

fn main() {
    println!("=== Construction Business Act & Real Estate Transactions Act ===\n");

    // Example 1: Valid construction license
    println!("🏗️  Example 1: General Construction Business License");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_general_construction_license();
    println!();

    // Example 2: Special construction license with higher capital requirements
    println!("🏗️  Example 2: Special Construction Business License");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_special_construction_license();
    println!();

    // Example 3: Invalid construction license (insufficient capital)
    println!("❌ Example 3: Insufficient Capital for Special License");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_insufficient_capital();
    println!();

    // Example 4: Real estate transaction with proper disclosures
    println!("🏠 Example 4: Real Estate Transaction (Article 35)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_real_estate_transaction();
    println!();

    // Example 5: Commission calculation and validation
    println!("💰 Example 5: Broker Commission Validation (Article 46)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_commission_calculation();
    println!();

    // Example 6: Invalid transaction (missing important matters explanation)
    println!("❌ Example 6: Missing Important Matters Explanation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_missing_important_matters();
}

fn example_general_construction_license() {
    let manager = Manager {
        name: "Tanaka Taro (田中太郎)".to_string(),
        qualification: ManagerQualification::FirstClassArchitect,
        certification_number: "ARCH-1-001234".to_string(),
        certification_date: NaiveDate::from_ymd_opt(2020, 4, 1).unwrap(),
    };

    let license = ConstructionBusinessLicense {
        license_number: "東京都知事許可(般-1)第12345号".to_string(),
        business_name: "株式会社サンプル建設 (Sample Construction Co., Ltd.)".to_string(),
        license_type: ConstructionLicenseType::General,
        construction_types: vec![
            ConstructionType::Architecture,
            ConstructionType::Civil,
            ConstructionType::Carpentry,
        ],
        registered_capital_jpy: 10_000_000, // ¥10M
        issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2029, 1, 1).unwrap(), // 5 years
        managers: vec![manager],
    };

    println!("License Number: {}", license.license_number);
    println!("Business Name: {}", license.business_name);
    println!(
        "License Type: {} ({})",
        license.license_type.name_en(),
        license.license_type.name_ja()
    );
    println!(
        "Registered Capital: ¥{} (Min: ¥{})",
        license.registered_capital_jpy,
        license.license_type.minimum_capital()
    );
    println!(
        "Construction Types: {} types authorized",
        license.construction_types.len()
    );
    for ct in &license.construction_types {
        println!("  • {:?}", ct);
    }
    println!("Managers: {} qualified", license.managers.len());
    for mgr in &license.managers {
        println!("  • {} ({:?})", mgr.name, mgr.qualification);
    }

    let validity_years = (license.expiration_date - license.issue_date).num_days() / 365;
    println!("License Validity: {} years (Article 3-3)", validity_years);

    match validate_construction_license(&license) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Construction license is VALID");
                println!("  ✓ Capital requirement met (Article 7)");
                println!("  ✓ Qualified manager present (Article 8)");
                println!("  ✓ 5-year validity period (Article 3-3)");

                if !report.warnings.is_empty() {
                    println!("\n⚠️  Recommendations:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            }
        }
        Err(e) => println!("❌ Validation error: {}", e),
    }
}

fn example_special_construction_license() {
    let manager1 = Manager {
        name: "Suzuki Hanako (鈴木花子)".to_string(),
        qualification: ManagerQualification::CivilEngineer,
        certification_number: "CIVIL-001".to_string(),
        certification_date: NaiveDate::from_ymd_opt(2019, 3, 15).unwrap(),
    };

    let manager2 = Manager {
        name: "Yamada Jiro (山田次郎)".to_string(),
        qualification: ManagerQualification::ConstructionManager,
        certification_number: "CONST-MGR-002".to_string(),
        certification_date: NaiveDate::from_ymd_opt(2021, 6, 1).unwrap(),
    };

    let license = ConstructionBusinessLicense {
        license_number: "国土交通大臣許可(特-1)第67890号".to_string(),
        business_name: "大手建設株式会社 (Major Construction Corp.)".to_string(),
        license_type: ConstructionLicenseType::Special,
        construction_types: vec![
            ConstructionType::Architecture,
            ConstructionType::Civil,
            ConstructionType::Electrical,
            ConstructionType::PlumbingHeating,
        ],
        registered_capital_jpy: 50_000_000, // ¥50M (exceeds ¥20M minimum)
        issue_date: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2028, 6, 1).unwrap(),
        managers: vec![manager1, manager2],
    };

    println!("License Number: {}", license.license_number);
    println!("Business Name: {}", license.business_name);
    println!(
        "License Type: {} ({})",
        license.license_type.name_en(),
        license.license_type.name_ja()
    );
    println!(
        "Registered Capital: ¥{} (Min: ¥{} for Special)",
        license.registered_capital_jpy,
        license.license_type.minimum_capital()
    );
    println!(
        "Construction Types: {} types",
        license.construction_types.len()
    );
    println!("Qualified Managers: {} (Article 8)", license.managers.len());

    match validate_construction_license(&license) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Special construction license is VALID");
                println!("  ✓ ¥20M minimum capital requirement met (Article 7-1)");
                println!("  ✓ Multiple qualified managers (Article 8)");
                println!("  ✓ Authorized for large-scale projects");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_insufficient_capital() {
    let manager = Manager {
        name: "Sato Yuki (佐藤由紀)".to_string(),
        qualification: ManagerQualification::SecondClassArchitect,
        certification_number: "ARCH-2-005678".to_string(),
        certification_date: NaiveDate::from_ymd_opt(2022, 8, 1).unwrap(),
    };

    let license = ConstructionBusinessLicense {
        license_number: "TEST-INVALID-001".to_string(),
        business_name: "Small Construction Inc.".to_string(),
        license_type: ConstructionLicenseType::Special, // Requires ¥20M
        construction_types: vec![ConstructionType::Architecture],
        registered_capital_jpy: 8_000_000, // ❌ Only ¥8M (insufficient!)
        issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2029, 1, 1).unwrap(),
        managers: vec![manager],
    };

    println!("Business Name: {}", license.business_name);
    println!("License Type: Special (requires ¥20M capital)");
    println!("Registered Capital: ¥{} ❌", license.registered_capital_jpy);
    println!(
        "Shortfall: ¥{}",
        20_000_000 - license.registered_capital_jpy
    );

    match validate_construction_license(&license) {
        Ok(report) => {
            if !report.is_valid() {
                println!("\n❌ COMPLIANCE VIOLATION:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
                println!("\n📌 Article 7-1: Special construction business requires ¥20M capital");
                println!("   Solution: Increase capital or apply for General license");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_real_estate_transaction() {
    let agent = LicensedAgent {
        name: "Kobayashi Akira (小林明)".to_string(),
        registration_number: "宅建士登録番号 123456".to_string(),
        registration_date: NaiveDate::from_ymd_opt(2021, 5, 1).unwrap(),
    };

    let broker = LicensedBroker {
        company_name: "Tokyo Real Estate Co., Ltd. (東京不動産株式会社)".to_string(),
        license_number: "東京都知事免許(2)第34567号".to_string(),
        agent,
        commission_jpy: 198_000, // Within Article 46 limits
    };

    let property = Property {
        property_type: PropertyType::LandAndBuilding,
        address: "3-4-5 Shibuya, Shibuya-ku, Tokyo".to_string(),
        area_sqm: 85.5,
        price_jpy: 30_000_000, // ¥30M
        description: Some("3LDK apartment, 8th floor, south facing".to_string()),
    };

    let transaction = RealEstateTransaction {
        transaction_id: "TX-2026-0109-001".to_string(),
        transaction_type: TransactionType::Sale,
        property,
        buyer: Party {
            name: "Watanabe Kenji (渡辺健二)".to_string(),
            address: "Yokohama City, Kanagawa".to_string(),
            contact: Some("090-1234-5678".to_string()),
        },
        seller: Party {
            name: "Nakamura Yumi (中村由美)".to_string(),
            address: "Shibuya-ku, Tokyo".to_string(),
            contact: Some("080-9876-5432".to_string()),
        },
        broker: Some(broker),
        important_matters_explained: true, // ✅ Article 35 compliance
        contract_date: Utc::now().date_naive(),
    };

    println!("Transaction ID: {}", transaction.transaction_id);
    println!("Type: {:?} (売買)", transaction.transaction_type);
    println!("\nProperty:");
    println!("  Type: {:?}", transaction.property.property_type);
    println!("  Address: {}", transaction.property.address);
    println!("  Area: {} m²", transaction.property.area_sqm);
    println!("  Price: ¥{}", transaction.property.price_jpy);

    if let Some(broker) = &transaction.broker {
        println!("\nBroker: {}", broker.company_name);
        println!("  License: {}", broker.license_number);
        println!("  Agent: {}", broker.agent.name);
        println!("  Commission: ¥{}", broker.commission_jpy);
    }

    let max_commission = transaction.calculate_max_commission();
    println!("\nCommission Analysis (Article 46):");
    println!(
        "  Actual: ¥{}",
        transaction.broker.as_ref().unwrap().commission_jpy
    );
    println!("  Maximum Allowed: ¥{}", max_commission);
    println!(
        "  Status: {}",
        if transaction.is_commission_valid() {
            "✅ Within limit"
        } else {
            "❌ Exceeds limit"
        }
    );

    match validate_real_estate_transaction(&transaction) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Real estate transaction is COMPLIANT");
                println!("  ✓ Important matters explained (Article 35)");
                println!("  ✓ Licensed agent present (宅地建物取引士)");
                println!("  ✓ Commission within legal limits (Article 46)");

                if !report.warnings.is_empty() {
                    println!("\n⚠️  Recommendations:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_commission_calculation() {
    println!("Commission Calculation Examples (Article 46)\n");

    let test_cases = vec![
        (1_500_000, "¥1.5M property"),
        (3_000_000, "¥3M property"),
        (5_000_000, "¥5M property"),
        (50_000_000, "¥50M property"),
    ];

    println!("Price Range Rates:");
    println!("  • ¥0-¥2M: 5% + tax");
    println!("  • ¥2M-¥4M: 4% + tax");
    println!("  • Over ¥4M: 3% + tax");
    println!("  • Tax: 10% consumption tax\n");

    for (price, description) in test_cases {
        let property = Property {
            property_type: PropertyType::Building,
            address: "Tokyo".to_string(),
            area_sqm: 50.0,
            price_jpy: price,
            description: None,
        };

        let transaction = RealEstateTransaction {
            transaction_id: format!("CALC-{}", price),
            transaction_type: TransactionType::Sale,
            property,
            buyer: Party {
                name: "Buyer".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            seller: Party {
                name: "Seller".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            broker: None,
            important_matters_explained: true,
            contract_date: Utc::now().date_naive(),
        };

        let max_commission = transaction.calculate_max_commission();
        let percentage = (max_commission as f64 / price as f64) * 100.0;

        println!("{}", description);
        println!("  Max Commission: ¥{} ({:.2}%)", max_commission, percentage);
    }

    println!("\n📌 Note: These are maximum amounts per party (buyer/seller)");
    println!("   Broker may charge up to this amount to each party.");
}

fn example_missing_important_matters() {
    let property = Property {
        property_type: PropertyType::Land,
        address: "Rural Area, Chiba Prefecture".to_string(),
        area_sqm: 200.0,
        price_jpy: 8_000_000,
        description: None,
    };

    let transaction = RealEstateTransaction {
        transaction_id: "TX-INVALID-001".to_string(),
        transaction_type: TransactionType::Sale,
        property,
        buyer: Party {
            name: "Inexperienced Buyer".to_string(),
            address: "Tokyo".to_string(),
            contact: None,
        },
        seller: Party {
            name: "Seller".to_string(),
            address: "Chiba".to_string(),
            contact: None,
        },
        broker: None,
        important_matters_explained: false, // ❌ Violation!
        contract_date: Utc::now().date_naive(),
    };

    println!("Transaction Type: {:?}", transaction.transaction_type);
    println!("Property: {}", transaction.property.address);
    println!("Important Matters Explained: No ❌");

    match validate_real_estate_transaction(&transaction) {
        Ok(report) => {
            if !report.is_valid() {
                println!("\n❌ COMPLIANCE VIOLATION:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
                println!("\n📌 Article 35: Important Matters Explanation (重要事項説明)");
                println!("   Required for ALL real estate transactions");
                println!("   Must be provided by licensed agent (宅地建物取引士)");
                println!("\n   Important matters include:");
                println!("   • Legal restrictions on the property");
                println!("   • Water/sewage facilities");
                println!("   • Building inspection status");
                println!("   • Contract terms and conditions");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}
