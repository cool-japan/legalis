//! Valid GmbH Formation Example
//!
//! Demonstrates complete formation of a standard GmbH with €50,000 capital,
//! two shareholders, and managing director validation.
//!
//! # Legal Context
//!
//! This example shows a typical GmbH formation scenario:
//! - €50,000 capital (above minimum of €25,000)
//! - Two shareholders with different contributions
//! - 100% capital paid upfront (exceeds 50% requirement)
//! - One managing director with sole representation authority
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example gmbh-formation-valid
//! ```

use chrono::{NaiveDate, Utc};
use legalis_de::gmbhg::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║     German GmbH Formation Example - GmbH-Gründung             ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ==========================================================================
    // Example 1: Standard GmbH with €50,000 capital
    // ==========================================================================
    println!("📋 Example 1: Standard GmbH (€50,000 Stammkapital)");
    println!("{}", "═".repeat(70));

    let articles = ArticlesOfAssociation {
        company_name: "Tech Solutions GmbH".to_string(),

        registered_office: RegisteredOffice {
            city: "Berlin".to_string(),
            full_address: Some("Alexanderplatz 1, 10178 Berlin".to_string()),
        },

        business_purpose:
            "Entwicklung und Vertrieb von Softwarelösungen sowie IT-Beratung und Consulting"
                .to_string(),

        share_capital: Capital::from_euros(50_000),

        share_structure: vec![
            ShareAllocation {
                shareholder: Shareholder {
                    name: "Max Mustermann".to_string(),
                    address: "Musterstraße 1, 10115 Berlin".to_string(),
                    shareholder_type: ShareholderType::NaturalPerson,
                },
                nominal_amount_cents: 3_000_000,    // €30,000 (60%)
                contribution_paid_cents: 3_000_000, // 100% paid
            },
            ShareAllocation {
                shareholder: Shareholder {
                    name: "Erika Musterfrau".to_string(),
                    address: "Beispielweg 5, 80331 München".to_string(),
                    shareholder_type: ShareholderType::NaturalPerson,
                },
                nominal_amount_cents: 2_000_000,    // €20,000 (40%)
                contribution_paid_cents: 2_000_000, // 100% paid
            },
        ],

        duration: Some(Duration::Unlimited),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        formation_date: Some(Utc::now()),
        resolution_requirements: None,
    };

    println!("\n📄 Articles of Association (Gesellschaftsvertrag):");
    println!("   Company Name:     {}", articles.company_name);
    println!("   Registered Office: {}", articles.registered_office.city);
    println!("   Business Purpose: {}", articles.business_purpose);
    println!(
        "   Share Capital:    €{:.2}",
        articles.share_capital.to_euros()
    );
    println!("\n👥 Shareholders (Gesellschafter):");
    for (i, share) in articles.share_structure.iter().enumerate() {
        println!("   {}. {}", i + 1, share.shareholder.name);
        println!(
            "      Nominal Amount: €{:.2} ({:.1}%)",
            (share.nominal_amount_cents as f64) / 100.0,
            (share.nominal_amount_cents as f64) / (articles.share_capital.amount_cents as f64)
                * 100.0
        );
        println!(
            "      Paid:          €{:.2} ({:.1}% of nominal)",
            (share.contribution_paid_cents as f64) / 100.0,
            (share.contribution_paid_cents as f64) / (share.nominal_amount_cents as f64) * 100.0
        );
        println!("      Address:       {}", share.shareholder.address);
    }

    // Validate articles
    println!("\n🔍 Validating Articles of Association...");
    match validate_articles_of_association(&articles, CompanyType::GmbH) {
        Ok(()) => {
            println!("✅ Articles of Association: VALID");
            println!("   - Capital meets §5 GmbHG requirement (€25,000 minimum)");
            println!("   - Initial contribution exceeds §7 Abs. 2 requirement (€12,500 or 50%)");
            println!("   - Company name includes required 'GmbH' suffix (§4 GmbHG)");
            println!("   - All mandatory elements per §3 GmbHG present");
        }
        Err(e) => {
            println!("❌ Validation FAILED: {}", e);
            return;
        }
    }

    // ==========================================================================
    // Managing Directors (Geschäftsführer)
    // ==========================================================================
    println!("\n👔 Managing Directors (Geschäftsführer):");
    println!("{}", "═".repeat(70));

    let directors = ManagingDirectors {
        directors: vec![ManagingDirector {
            name: "Max Mustermann".to_string(),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1980, 5, 15).unwrap()),
            address: "Musterstraße 1, 10115 Berlin".to_string(),
            appointment_date: Utc::now(),
            representation_authority: RepresentationAuthority::Sole,
            has_capacity: true,
        }],
    };

    println!("\n   Name:           {}", directors.directors[0].name);
    println!(
        "   Date of Birth:  {}",
        directors.directors[0]
            .date_of_birth
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "Not specified".to_string())
    );
    println!("   Address:        {}", directors.directors[0].address);
    println!(
        "   Authority:      {:?}",
        directors.directors[0].representation_authority
    );
    println!(
        "   Legal Capacity: {}",
        if directors.directors[0].has_capacity {
            "Yes"
        } else {
            "No"
        }
    );

    println!("\n🔍 Validating Managing Directors...");
    match validate_managing_directors(&directors) {
        Ok(()) => {
            println!("✅ Managing Directors: VALID");
            println!("   - At least one director appointed (§6 Abs. 3 GmbHG)");
            println!("   - Natural person with full capacity (§6 Abs. 2 S. 2 GmbHG)");
            println!("   - All required information provided");
        }
        Err(e) => {
            println!("❌ Managing Directors FAILED: {}", e);
            return;
        }
    }

    // ==========================================================================
    // Formation Summary
    // ==========================================================================
    println!("\n📊 Formation Summary:");
    println!("{}", "═".repeat(70));
    println!("✅ All validations passed!");
    println!("\n📝 Next Steps (Nächste Schritte):");
    println!("   1. ✍️  Notarization of articles (Notarielle Beurkundung)");
    println!("      → Articles must be certified by a German notary (§2 GmbHG)");
    println!("\n   2. 💰 Capital contribution (Einlageleistung)");
    println!("      → Transfer €50,000 to company bank account");
    println!("      → At least €25,000 (50%) or €12,500 before registration (§7 Abs. 2)");
    println!("\n   3. 📋 Commercial register entry (Handelsregistereintragung)");
    println!("      → File application with local court (Amtsgericht)");
    println!("      → GmbH acquires legal personality upon registration (§11 GmbHG)");
    println!("\n   4. 🏢 Business registration (Gewerbeanmeldung)");
    println!("      → Register business with local trade office (Gewerbeamt)");
    println!("\n   5. 🔢 Tax registration (Steuerliche Anmeldung)");
    println!("      → Register with tax office (Finanzamt)");
    println!("      → Obtain tax ID (Steuernummer)");

    println!("\n{}", "═".repeat(70));
    println!("🎉 GmbH formation example completed successfully!");
    println!("{}\n", "═".repeat(70));

    // ==========================================================================
    // Example 2: Capital Validation Edge Cases
    // ==========================================================================
    println!("\n📋 Example 2: Capital Validation Edge Cases");
    println!("{}", "═".repeat(70));

    // Exactly minimum (€25,000)
    let min_capital = Capital::from_euros(25_000);
    println!("\n💶 Testing €25,000 (exactly minimum):");
    match validate_capital(&min_capital, CompanyType::GmbH) {
        Ok(()) => println!("   ✅ Valid - exactly meets §5 GmbHG requirement"),
        Err(e) => println!("   ❌ Invalid: {}", e),
    }

    // Below minimum (€24,999)
    let below_min = Capital::from_euros(24_999);
    println!("\n💶 Testing €24,999 (€1 below minimum):");
    match validate_capital(&below_min, CompanyType::GmbH) {
        Ok(()) => println!("   ✅ Valid (unexpected!)"),
        Err(_e) => println!("   ❌ Invalid (expected): Capital below minimum"),
    }

    // High capital (€1,000,000)
    let high_capital = Capital::from_euros(1_000_000);
    println!("\n💶 Testing €1,000,000 (high capital):");
    match validate_capital(&high_capital, CompanyType::GmbH) {
        Ok(()) => println!("   ✅ Valid - no maximum limit for GmbH"),
        Err(e) => println!("   ❌ Invalid: {}", e),
    }

    println!("\n{}", "═".repeat(70));
    println!("Example 2 completed.\n");
}
