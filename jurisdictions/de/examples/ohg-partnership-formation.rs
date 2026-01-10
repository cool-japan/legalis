//! OHG Partnership Formation Example
//!
//! Demonstrates complete formation of a General Partnership (Offene Handelsgesellschaft - OHG)
//! under §105-160 HGB.
//!
//! # Legal Context
//!
//! The OHG is a partnership where all partners have **unlimited personal liability**
//! for partnership debts (§128 HGB). It is commonly used for:
//! - Small to medium-sized businesses
//! - Professional partnerships (law firms, consulting firms)
//! - Family businesses where partners trust each other
//!
//! # Key Characteristics
//!
//! - **Minimum partners**: 2 (§105 Abs. 1 HGB)
//! - **Liability**: Unlimited and joint for all partners (§128 HGB)
//! - **Management**: Each partner has management authority (§114 HGB)
//! - **Representation**: Each partner can represent the partnership (§125 HGB)
//! - **Profit sharing**: Equal unless otherwise agreed (§121 HGB)
//! - **Commercial register**: Registration required (§106 HGB)
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example ohg-partnership-formation
//! ```

use chrono::Utc;
use legalis_de::gmbhg::Capital;
use legalis_de::hgb::{FiscalYearEnd, OHG, Partner, PartnerType, validate_ohg};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║   German OHG Formation - Offene Handelsgesellschaft          ║");
    println!("║   General Partnership under §105-160 HGB                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ==========================================================================
    // Example 1: Standard OHG with Two Partners
    // ==========================================================================
    println!("📋 Example 1: Standard OHG with Two Partners");
    println!("{}", "═".repeat(70));

    let ohg = OHG {
        partnership_name: "Mustermann & Schmidt OHG".to_string(),
        registered_office: "Berlin".to_string(),
        business_purpose: "Softwareentwicklung, IT-Beratung und digitale Transformation für mittelständische Unternehmen".to_string(),
        partners: vec![
            Partner {
                name: "Max Mustermann".to_string(),
                address: "Musterstraße 1, 10115 Berlin".to_string(),
                contribution: Some(Capital::from_euros(15_000)),
                contribution_paid: Some(Capital::from_euros(15_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
            Partner {
                name: "Erika Schmidt".to_string(),
                address: "Beispielweg 5, 20095 Hamburg".to_string(),
                contribution: Some(Capital::from_euros(15_000)),
                contribution_paid: Some(Capital::from_euros(15_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
        ],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        unlimited_liability: true,
    };

    println!("\n📄 Partnership Agreement (Gesellschaftsvertrag):");
    println!("   Partnership Name:  {}", ohg.partnership_name);
    println!("   Registered Office: {}", ohg.registered_office);
    println!("   Business Purpose:  {}", ohg.business_purpose);
    println!("\n👥 Partners (Gesellschafter):");
    for (i, partner) in ohg.partners.iter().enumerate() {
        println!("   {}. {}", i + 1, partner.name);
        println!("      Address:       {}", partner.address);
        if let Some(contribution) = &partner.contribution {
            println!("      Contribution:  €{:.2}", contribution.to_euros());
        }
        if let Some(paid) = &partner.contribution_paid {
            println!("      Paid:          €{:.2}", paid.to_euros());
        }
        println!(
            "      Management:    {}",
            if partner.has_management_authority {
                "Yes (§114 HGB)"
            } else {
                "No"
            }
        );
        println!(
            "      Representation: {}",
            if partner.has_representation_authority {
                "Yes (§125 HGB)"
            } else {
                "No"
            }
        );
        println!("      Liability:     Unlimited (§128 HGB)");
    }

    // Calculate total contributions
    let total_contributions: u64 = ohg
        .partners
        .iter()
        .filter_map(|p| p.contribution_paid.as_ref())
        .map(|c| c.amount_cents)
        .sum();

    println!("\n💰 Financial Summary:");
    println!(
        "   Total Contributions: €{:.2}",
        (total_contributions as f64) / 100.0
    );
    println!(
        "   Partnership Assets:  €{:.2} (initial capital)",
        (total_contributions as f64) / 100.0
    );

    // Validate OHG
    println!("\n🔍 Validating OHG Structure...");
    match validate_ohg(&ohg) {
        Ok(()) => {
            println!("✅ OHG Structure: VALID");
            println!("   - Minimum 2 partners requirement met (§105 Abs. 1 HGB)");
            println!("   - All partners have unlimited liability (§128 HGB)");
            println!("   - Partnership name includes 'OHG' suffix (§19 HGB)");
            println!("   - Business purpose is valid and meaningful");
            println!("   - Each partner has management authority (§114 HGB)");
            println!("   - Each partner can represent the partnership (§125 HGB)");
        }
        Err(e) => {
            println!("❌ Validation FAILED: {}", e);
            return;
        }
    }

    // ==========================================================================
    // Example 2: OHG with Three Partners (Different Contributions)
    // ==========================================================================
    println!("\n\n📋 Example 2: OHG with Three Partners (Unequal Contributions)");
    println!("{}", "═".repeat(70));

    let ohg_unequal = OHG {
        partnership_name: "Tech Innovators OHG".to_string(),
        registered_office: "München".to_string(),
        business_purpose: "Entwicklung und Vertrieb innovativer Softwarelösungen im Bereich künstliche Intelligenz".to_string(),
        partners: vec![
            Partner {
                name: "Dr. Anna Müller".to_string(),
                address: "Maximilianstraße 10, 80539 München".to_string(),
                contribution: Some(Capital::from_euros(50_000)),
                contribution_paid: Some(Capital::from_euros(50_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
            Partner {
                name: "Peter Schmidt".to_string(),
                address: "Leopoldstraße 25, 80802 München".to_string(),
                contribution: Some(Capital::from_euros(30_000)),
                contribution_paid: Some(Capital::from_euros(30_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
            Partner {
                name: "Lisa Weber".to_string(),
                address: "Sendlinger Straße 15, 80331 München".to_string(),
                contribution: Some(Capital::from_euros(20_000)),
                contribution_paid: Some(Capital::from_euros(20_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
        ],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        unlimited_liability: true,
    };

    println!("\n   Partnership Name: {}", ohg_unequal.partnership_name);
    println!("   Partners: {}", ohg_unequal.partners.len());

    let total_unequal: u64 = ohg_unequal
        .partners
        .iter()
        .filter_map(|p| p.contribution_paid.as_ref())
        .map(|c| c.amount_cents)
        .sum();

    println!("\n   Contribution Breakdown:");
    for (i, partner) in ohg_unequal.partners.iter().enumerate() {
        if let Some(contribution) = &partner.contribution {
            let percentage = (contribution.amount_cents as f64) / (total_unequal as f64) * 100.0;
            println!(
                "   {}. {}: €{:.2} ({:.1}%)",
                i + 1,
                partner.name,
                contribution.to_euros(),
                percentage
            );
        }
    }
    println!("   Total Capital: €{:.2}", (total_unequal as f64) / 100.0);

    match validate_ohg(&ohg_unequal) {
        Ok(()) => println!("\n✅ OHG with unequal contributions: VALID"),
        Err(e) => println!("\n❌ Validation FAILED: {}", e),
    }

    // ==========================================================================
    // Example 3: Invalid OHG (Insufficient Partners)
    // ==========================================================================
    println!("\n\n📋 Example 3: Invalid OHG (Only 1 Partner)");
    println!("{}", "═".repeat(70));

    let invalid_ohg = OHG {
        partnership_name: "Solo Business OHG".to_string(),
        registered_office: "Frankfurt am Main".to_string(),
        business_purpose: "Unternehmensberatung".to_string(),
        partners: vec![Partner {
            name: "Hans Mueller".to_string(),
            address: "Zeil 1, 60313 Frankfurt am Main".to_string(),
            contribution: Some(Capital::from_euros(25_000)),
            contribution_paid: Some(Capital::from_euros(25_000)),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        }],
        formation_date: None,
        fiscal_year_end: None,
        unlimited_liability: true,
    };

    println!("\n   Partnership Name: {}", invalid_ohg.partnership_name);
    println!(
        "   Partners: {} (requires minimum 2!)",
        invalid_ohg.partners.len()
    );

    match validate_ohg(&invalid_ohg) {
        Ok(()) => println!("\n✅ Valid (unexpected!)"),
        Err(e) => {
            println!("\n❌ Validation FAILED (expected):");
            println!("   {}", e);
            println!("\n   ⚠️  An OHG requires at least 2 partners per §105 Abs. 1 HGB.");
            println!("   For a single-person business, consider:");
            println!("   - Einzelunternehmen (sole proprietorship)");
            println!("   - GmbH (limited liability company)");
            println!("   - UG (haftungsbeschränkt) (mini-GmbH)");
        }
    }

    // ==========================================================================
    // Formation Summary
    // ==========================================================================
    println!("\n\n📊 OHG Formation Summary:");
    println!("{}", "═".repeat(70));
    println!("✅ Valid OHG formations demonstrated!\n");
    println!("📝 Next Steps (Nächste Schritte):");
    println!("   1. ✍️  Draft partnership agreement (Gesellschaftsvertrag)");
    println!("      → Include all mandatory elements per §105 HGB");
    println!("      → Define profit/loss distribution (default: equal per §121 HGB)");
    println!("      → Specify management rules if deviating from §114 HGB");
    println!("\n   2. 📋 Commercial register entry (Handelsregistereintragung)");
    println!("      → File application with local court (Amtsgericht)");
    println!("      → Submit partnership agreement and partner information");
    println!("      → OHG acquires legal personality upon registration (§123 HGB)");
    println!("\n   3. 🏢 Business registration (Gewerbeanmeldung)");
    println!("      → Register with local trade office (Gewerbeamt)");
    println!("\n   4. 🔢 Tax registration (Steuerliche Anmeldung)");
    println!("      → Register with tax office (Finanzamt)");
    println!("      → OHG is subject to transparent taxation (income flows to partners)");

    println!("\n⚠️  Important Legal Considerations:");
    println!("{}", "═".repeat(70));
    println!("   🔴 Unlimited Liability (Unbeschränkte Haftung):");
    println!("      - All partners liable with personal assets (§128 HGB)");
    println!("      - Joint and several liability for partnership debts");
    println!("      - Creditors can pursue any partner for full debt amount");
    println!("\n   ⚙️  Management and Representation:");
    println!("      - Each partner has management authority (§114 HGB)");
    println!("      - Each partner can represent the partnership (§125 HGB)");
    println!("      - Partnership agreement can modify these defaults");
    println!("\n   💰 Profit and Loss Allocation:");
    println!("      - Default: Equal distribution (§121 HGB)");
    println!("      - Can be modified in partnership agreement");
    println!("      - Transparent taxation: income taxed at partner level");

    println!("\n{}", "═".repeat(70));
    println!("🎉 OHG formation examples completed successfully!");
    println!("{}\n", "═".repeat(70));

    // ==========================================================================
    // Comparison: OHG vs Other Legal Forms
    // ==========================================================================
    println!("📊 Comparison: OHG vs Other Legal Forms");
    println!("{}", "═".repeat(70));
    println!("┌───────────────────┬──────────┬──────────┬──────────┬──────────┐");
    println!("│ Feature           │   OHG    │    KG    │   GmbH   │    UG    │");
    println!("├───────────────────┼──────────┼──────────┼──────────┼──────────┤");
    println!("│ Min. Partners     │    2     │    2     │    1     │    1     │");
    println!("│ Liability         │ Unlimited│  Mixed   │ Limited  │ Limited  │");
    println!("│ Min. Capital      │   None   │   None   │ €25,000  │   €1     │");
    println!("│ Formation Cost    │   Low    │   Low    │  Medium  │  Medium  │");
    println!("│ Taxation          │Transparent│Transparent│ Corporate│Corporate│");
    println!("│ Complexity        │   Low    │  Medium  │  Medium  │  Medium  │");
    println!("└───────────────────┴──────────┴──────────┴──────────┴──────────┘");

    println!("\n📚 Legal References:");
    println!("   - §105-160 HGB: General provisions for OHG");
    println!("   - §114 HGB: Management authority of partners");
    println!("   - §121 HGB: Profit and loss distribution");
    println!("   - §125 HGB: Representation authority");
    println!("   - §128 HGB: Unlimited liability of partners\n");
}
