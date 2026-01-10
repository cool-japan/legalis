//! UG Formation Example (Mini-GmbH)
//!
//! Demonstrates formation of UG (Unternehmergesellschaft - haftungsbeschränkt)
//! with €1 minimum capital, showing the difference from standard GmbH and
//! reserve accumulation requirements.
//!
//! # Legal Context
//!
//! The UG is a "mini-GmbH" introduced to allow low-capital company formation:
//! - Minimum capital: €1 (vs €25,000 for GmbH)
//! - Maximum capital: €24,999 (at €25,000 it becomes a GmbH)
//! - Reserve requirement: 25% of annual profits (§5a GmbHG)
//! - Full payment required immediately (no partial payment like GmbH)
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example ug-formation-mini-gmbh
//! ```

use chrono::Utc;
use legalis_de::gmbhg::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║   UG (haftungsbeschränkt) Formation - Mini-GmbH-Gründung      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ==========================================================================
    // Example 1: UG with €1 minimum capital
    // ==========================================================================
    println!("📋 Example 1: UG with €1 Capital (Minimum)");
    println!("{}", "═".repeat(70));

    let articles_min = ArticlesOfAssociation {
        company_name: "Startup Ventures UG (haftungsbeschränkt)".to_string(),

        registered_office: RegisteredOffice {
            city: "Hamburg".to_string(),
            full_address: Some("Reeperbahn 1, 20359 Hamburg".to_string()),
        },

        business_purpose: "Online-Handel und E-Commerce-Dienstleistungen sowie Dropshipping"
            .to_string(),

        share_capital: Capital::from_euros(1), // Minimum €1

        share_structure: vec![ShareAllocation {
            shareholder: Shareholder {
                name: "Anna Schmidt".to_string(),
                address: "Reeperbahn 1, 20359 Hamburg".to_string(),
                shareholder_type: ShareholderType::NaturalPerson,
            },
            nominal_amount_cents: 100,    // €1
            contribution_paid_cents: 100, // Fully paid (required for UG)
        }],

        duration: Some(Duration::Unlimited),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        formation_date: Some(Utc::now()),
        resolution_requirements: None,
    };

    println!("\n📄 Articles of Association (Gesellschaftsvertrag):");
    println!("   Company Name:     {}", articles_min.company_name);
    println!(
        "   Registered Office: {}",
        articles_min.registered_office.city
    );
    println!("   Business Purpose: {}", articles_min.business_purpose);
    println!(
        "   Share Capital:    €{:.2}",
        articles_min.share_capital.to_euros()
    );

    println!("\n👤 Shareholder (Gesellschafter):");
    println!(
        "   Name:    {}",
        articles_min.share_structure[0].shareholder.name
    );
    println!(
        "   Capital: €{:.2} (100% ownership)",
        (articles_min.share_structure[0].nominal_amount_cents as f64) / 100.0
    );
    println!(
        "   Paid:    €{:.2} (fully paid)",
        (articles_min.share_structure[0].contribution_paid_cents as f64) / 100.0
    );

    println!("\n🔍 Validating UG Formation...");
    match validate_articles_of_association(&articles_min, CompanyType::UG) {
        Ok(()) => {
            println!("✅ UG Formation Valid!");
            println!("   - Capital meets §5a GmbHG minimum (€1)");
            println!("   - Capital below GmbH threshold (€25,000)");
            println!("   - Company name includes required 'UG (haftungsbeschränkt)' suffix");
            println!("   - Capital fully paid (UG requirement)");
        }
        Err(e) => {
            println!("❌ Validation Failed: {}", e);
            return;
        }
    }

    // UG-specific requirements
    println!("\n⚠️  Important UG-Specific Requirements (§5a GmbHG):");
    println!("{}", "─".repeat(70));
    println!("   1. 📊 Reserve Accumulation (Rücklagenbildung):");
    println!("      → 25% of annual net profits MUST be allocated to reserves");
    println!("      → Continues until reserves reach €25,000");
    println!("      → Then may convert to regular GmbH");
    println!("\n   2. 📝 Company Name:");
    println!("      → MUST include 'UG (haftungsbeschränkt)' or");
    println!("        'Unternehmergesellschaft (haftungsbeschränkt)'");
    println!("      → Cannot use just 'GmbH'");
    println!("\n   3. 💰 Full Payment:");
    println!("      → Entire capital must be paid immediately");
    println!("      → No partial payment allowed (unlike GmbH §7 Abs. 2)");
    println!("\n   4. 🔄 Conversion Path:");
    println!("      → Once reserves reach €25,000, may convert to GmbH");
    println!("      → Change company name from 'UG' to 'GmbH'");
    println!("      → No longer subject to reserve requirement");

    // ==========================================================================
    // Example 2: UG with €10,000 capital (typical startup)
    // ==========================================================================
    println!("\n\n📋 Example 2: UG with €10,000 Capital (Typical Startup)");
    println!("{}", "═".repeat(70));

    let articles_typical = ArticlesOfAssociation {
        company_name: "TechStart UG (haftungsbeschränkt)".to_string(),

        registered_office: RegisteredOffice {
            city: "München".to_string(),
            full_address: None,
        },

        business_purpose: "Softwareentwicklung, App-Entwicklung und digitale Beratung".to_string(),

        share_capital: Capital::from_euros(10_000),

        share_structure: vec![
            ShareAllocation {
                shareholder: Shareholder {
                    name: "Max Müller".to_string(),
                    address: "München, Germany".to_string(),
                    shareholder_type: ShareholderType::NaturalPerson,
                },
                nominal_amount_cents: 600_000, // €6,000 (60%)
                contribution_paid_cents: 600_000,
            },
            ShareAllocation {
                shareholder: Shareholder {
                    name: "Lisa Weber".to_string(),
                    address: "München, Germany".to_string(),
                    shareholder_type: ShareholderType::NaturalPerson,
                },
                nominal_amount_cents: 400_000, // €4,000 (40%)
                contribution_paid_cents: 400_000,
            },
        ],

        duration: Some(Duration::Unlimited),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        formation_date: Some(Utc::now()),
        resolution_requirements: None,
    };

    println!(
        "\n   Capital: €{:.2}",
        articles_typical.share_capital.to_euros()
    );
    println!(
        "   Shareholders: {}",
        articles_typical.share_structure.len()
    );

    match validate_articles_of_association(&articles_typical, CompanyType::UG) {
        Ok(()) => println!("   ✅ Valid UG formation with €10,000"),
        Err(e) => println!("   ❌ Invalid: {}", e),
    }

    // Reserve calculation example
    println!("\n💡 Reserve Accumulation Example (§5a GmbHG):");
    println!("{}", "─".repeat(70));
    println!("   Scenario: Annual profit of €20,000");
    println!("\n   Year 1:");
    println!("      Annual Profit:     €20,000");
    println!("      Reserve (25%):     €5,000  → Total reserves: €5,000");
    println!("      Distributable:     €15,000");
    println!("\n   Year 2:");
    println!("      Annual Profit:     €20,000");
    println!("      Reserve (25%):     €5,000  → Total reserves: €10,000");
    println!("      Distributable:     €15,000");
    println!("\n   Year 3:");
    println!("      Annual Profit:     €20,000");
    println!("      Reserve (25%):     €5,000  → Total reserves: €15,000");
    println!("      Distributable:     €15,000");
    println!("\n   Year 4:");
    println!("      Annual Profit:     €20,000");
    println!("      Reserve (25%):     €5,000  → Total reserves: €20,000");
    println!("      Distributable:     €15,000");
    println!("\n   Year 5:");
    println!("      Annual Profit:     €20,000");
    println!("      Reserve (25%):     €5,000  → Total reserves: €25,000 ✓");
    println!("      Distributable:     €15,000");
    println!("\n   ✅ Reserves reached €25,000 → May convert to GmbH!");

    // ==========================================================================
    // Example 3: Invalid UG (capital too high)
    // ==========================================================================
    println!("\n\n📋 Example 3: Invalid UG (Capital Exceeds Limit)");
    println!("{}", "═".repeat(70));

    let invalid_ug = ArticlesOfAssociation {
        company_name: "Large Startup UG (haftungsbeschränkt)".to_string(),
        registered_office: RegisteredOffice {
            city: "Berlin".to_string(),
            full_address: None,
        },
        business_purpose: "Software development".to_string(),
        share_capital: Capital::from_euros(25_000), // Too high for UG!
        share_structure: vec![ShareAllocation {
            shareholder: Shareholder {
                name: "Test User".to_string(),
                address: "Berlin".to_string(),
                shareholder_type: ShareholderType::NaturalPerson,
            },
            nominal_amount_cents: 2_500_000,
            contribution_paid_cents: 2_500_000,
        }],
        duration: Some(Duration::Unlimited),
        fiscal_year_end: None,
        formation_date: None,
        resolution_requirements: None,
    };

    println!("\n   Attempting UG formation with €25,000 capital...");
    match validate_articles_of_association(&invalid_ug, CompanyType::UG) {
        Ok(()) => println!("   ✅ Valid (unexpected!)"),
        Err(GmbHError::UGCapitalExceedsLimit { actual_euros }) => {
            println!("   ❌ Expected Error:");
            println!("      → Capital €{:.2} exceeds UG limit", actual_euros);
            println!("      → At €25,000, company must be formed as GmbH");
            println!("      → UG maximum is €24,999.99");
        }
        Err(e) => println!("   ❌ Unexpected error: {}", e),
    }

    // ==========================================================================
    // Example 4: Capital validation edge cases
    // ==========================================================================
    println!("\n\n📋 Example 4: Capital Validation Edge Cases");
    println!("{}", "═".repeat(70));

    // €1 (minimum)
    let cap_1 = Capital::from_euros(1);
    println!("\n💶 €1 (minimum):");
    match validate_capital(&cap_1, CompanyType::UG) {
        Ok(()) => println!("   ✅ Valid - meets §5a GmbHG minimum"),
        Err(e) => println!("   ❌ {}", e),
    }

    // €24,999 (maximum)
    let cap_max = Capital::from_euros(24_999);
    println!("\n💶 €24,999 (maximum for UG):");
    match validate_capital(&cap_max, CompanyType::UG) {
        Ok(()) => println!("   ✅ Valid - just under GmbH threshold"),
        Err(e) => println!("   ❌ {}", e),
    }

    // €24,999.99 (absolute maximum)
    let cap_max_cents = Capital::from_cents(2_499_999);
    println!("\n💶 €24,999.99 (absolute maximum):");
    match validate_capital(&cap_max_cents, CompanyType::UG) {
        Ok(()) => println!("   ✅ Valid - highest possible UG capital"),
        Err(e) => println!("   ❌ {}", e),
    }

    // €25,000 (too high - must be GmbH)
    let cap_gmbh = Capital::from_euros(25_000);
    println!("\n💶 €25,000 (becomes GmbH):");
    match validate_capital(&cap_gmbh, CompanyType::UG) {
        Ok(()) => println!("   ✅ Valid (unexpected)"),
        Err(GmbHError::UGCapitalExceedsLimit { .. }) => {
            println!("   ❌ Expected: Exceeds UG limit, must be GmbH")
        }
        Err(e) => println!("   ❌ {}", e),
    }

    // ==========================================================================
    // Summary
    // ==========================================================================
    println!("\n{}", "═".repeat(70));
    println!("📊 UG vs GmbH Comparison:");
    println!("{}", "═".repeat(70));
    println!("┌─────────────────────────┬──────────────────┬──────────────────┐");
    println!("│ Aspect                  │ UG               │ GmbH             │");
    println!("├─────────────────────────┼──────────────────┼──────────────────┤");
    println!("│ Minimum Capital         │ €1               │ €25,000          │");
    println!("│ Maximum Capital         │ €24,999          │ None             │");
    println!("│ Initial Payment         │ 100%             │ 50% or €12,500   │");
    println!("│ Reserve Requirement     │ 25% of profits   │ None             │");
    println!("│ Name Suffix             │ UG (haft.)       │ GmbH             │");
    println!("│ Conversion Path         │ → GmbH at €25k   │ N/A              │");
    println!("└─────────────────────────┴──────────────────┴──────────────────┘");

    println!("\n{}", "═".repeat(70));
    println!("🎉 UG formation examples completed successfully!");
    println!("{}\n", "═".repeat(70));
}
