//! KG Limited Partnership Formation Example
//!
//! Demonstrates complete formation of a Limited Partnership (Kommanditgesellschaft - KG)
//! and GmbH & Co. KG under §161-177a HGB.
//!
//! # Legal Context
//!
//! The KG is a partnership with **two types of partners with different liability**:
//! - **General partners (Komplementäre)**: Unlimited personal liability
//! - **Limited partners (Kommanditisten)**: Limited to contribution amount
//!
//! # Key Characteristics
//!
//! - **Minimum partners**: 1 general + 1 limited (§161 Abs. 1 HGB)
//! - **General partners**: Unlimited liability like OHG (§161 Abs. 2 HGB)
//! - **Limited partners**: Liability limited to agreed amount (§171 HGB)
//! - **Management**: Only general partners (§164 HGB)
//! - **Taxation**: Transparent (partnership level)
//! - **Commercial register**: Registration required with liability limits (§162 HGB)
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example kg-limited-partnership
//! ```

use chrono::Utc;
use legalis_de::gmbhg::Capital;
use legalis_de::hgb::{
    FiscalYearEnd, GmbHCoKG, GmbHPartner, KG, LimitedPartner, Partner, PartnerType,
    validate_gmbh_co_kg, validate_kg,
};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║   German KG Formation - Kommanditgesellschaft                 ║");
    println!("║   Limited Partnership under §161-177a HGB                    ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ==========================================================================
    // Example 1: Standard KG with 1 General + 2 Limited Partners
    // ==========================================================================
    println!("📋 Example 1: Standard KG with Mixed Liability Structure");
    println!("{}", "═".repeat(70));

    let kg = KG {
        partnership_name: "Tech Ventures KG".to_string(),
        registered_office: "München".to_string(),
        business_purpose:
            "IT-Beratung, Softwareentwicklung und digitale Transformation für Unternehmen"
                .to_string(),
        general_partners: vec![Partner {
            name: "Max Mustermann".to_string(),
            address: "Maximilianstraße 10, 80539 München".to_string(),
            contribution: Some(Capital::from_euros(20_000)),
            contribution_paid: Some(Capital::from_euros(20_000)),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        }],
        limited_partners: vec![
            LimitedPartner {
                name: "Anna Schmidt".to_string(),
                address: "Leopoldstraße 25, 80802 München".to_string(),
                liability_limit: Capital::from_euros(50_000),
                contribution_paid: Capital::from_euros(50_000),
                partner_type: PartnerType::NaturalPerson,
                has_special_representation: false,
            },
            LimitedPartner {
                name: "Peter Weber".to_string(),
                address: "Sendlinger Straße 15, 80331 München".to_string(),
                liability_limit: Capital::from_euros(30_000),
                contribution_paid: Capital::from_euros(30_000),
                partner_type: PartnerType::NaturalPerson,
                has_special_representation: false,
            },
        ],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
    };

    println!("\n📄 Partnership Agreement (Gesellschaftsvertrag):");
    println!("   Partnership Name:  {}", kg.partnership_name);
    println!("   Registered Office: {}", kg.registered_office);
    println!("   Business Purpose:  {}", kg.business_purpose);

    println!("\n👔 General Partners (Komplementäre) - Unlimited Liability:");
    for (i, partner) in kg.general_partners.iter().enumerate() {
        println!("   {}. {}", i + 1, partner.name);
        println!("      Address:       {}", partner.address);
        if let Some(contribution) = &partner.contribution {
            println!("      Contribution:  €{:.2}", contribution.to_euros());
        }
        println!("      Liability:     Unlimited (§161 Abs. 2 HGB)");
        println!("      Management:    Yes (§164 HGB)");
        println!("      Representation: Yes (§170 HGB)");
    }

    println!("\n👥 Limited Partners (Kommanditisten) - Limited Liability:");
    for (i, partner) in kg.limited_partners.iter().enumerate() {
        println!("   {}. {}", i + 1, partner.name);
        println!("      Address:        {}", partner.address);
        println!(
            "      Liability Limit: €{:.2} (Haftsumme)",
            partner.liability_limit.to_euros()
        );
        println!(
            "      Paid:           €{:.2}",
            partner.contribution_paid.to_euros()
        );
        let remaining_liability =
            partner.liability_limit.amount_cents - partner.contribution_paid.amount_cents;
        println!(
            "      Remaining Liability: €{:.2}",
            (remaining_liability as f64) / 100.0
        );
        println!("      Management:     No (§164 HGB)");
        println!(
            "      Representation: {}",
            if partner.has_special_representation {
                "Yes (special authority)"
            } else {
                "No (§170 HGB)"
            }
        );
    }

    // Calculate total capital
    let general_capital: u64 = kg
        .general_partners
        .iter()
        .filter_map(|p| p.contribution_paid.as_ref())
        .map(|c| c.amount_cents)
        .sum();

    let limited_capital: u64 = kg
        .limited_partners
        .iter()
        .map(|p| p.contribution_paid.amount_cents)
        .sum();

    let total_capital = general_capital + limited_capital;

    println!("\n💰 Financial Summary:");
    println!(
        "   General Partners: €{:.2}",
        (general_capital as f64) / 100.0
    );
    println!(
        "   Limited Partners: €{:.2}",
        (limited_capital as f64) / 100.0
    );
    println!(
        "   Total Capital:    €{:.2}",
        (total_capital as f64) / 100.0
    );

    // Validate KG
    println!("\n🔍 Validating KG Structure...");
    match validate_kg(&kg) {
        Ok(()) => {
            println!("✅ KG Structure: VALID");
            println!("   - At least 1 general partner (§161 Abs. 1 HGB)");
            println!("   - At least 1 limited partner (§161 Abs. 1 HGB)");
            println!("   - Partnership name includes 'KG' suffix (§19 HGB)");
            println!("   - All liability limits are valid (§171 HGB)");
            println!("   - Limited partners cannot manage (§164 HGB)");
        }
        Err(e) => {
            println!("❌ Validation FAILED: {}", e);
            return;
        }
    }

    // ==========================================================================
    // Example 2: GmbH & Co. KG (Limited Liability for All)
    // ==========================================================================
    println!("\n\n📋 Example 2: GmbH & Co. KG (Hybrid Structure)");
    println!("{}", "═".repeat(70));

    let gmbh_co_kg = GmbHCoKG {
        partnership_name: "Verwaltungs GmbH & Co. KG".to_string(),
        registered_office: "Berlin".to_string(),
        business_purpose: "Vermögensverwaltung, Beteiligungen und Immobilienverwaltung".to_string(),
        gmbh_general_partner: GmbHPartner {
            company_name: "Verwaltungs GmbH".to_string(),
            registered_office: "Berlin".to_string(),
            managing_directors: vec!["Dr. Anna Müller".to_string(), "Thomas Schmidt".to_string()],
            share_capital: Capital::from_euros(25_000),
        },
        limited_partners: vec![
            LimitedPartner {
                name: "Investor Alpha GmbH".to_string(),
                address: "Friedrichstraße 100, 10117 Berlin".to_string(),
                liability_limit: Capital::from_euros(500_000),
                contribution_paid: Capital::from_euros(500_000),
                partner_type: PartnerType::LegalEntity,
                has_special_representation: false,
            },
            LimitedPartner {
                name: "Familie Müller".to_string(),
                address: "Kurfürstendamm 50, 10707 Berlin".to_string(),
                liability_limit: Capital::from_euros(200_000),
                contribution_paid: Capital::from_euros(200_000),
                partner_type: PartnerType::NaturalPerson,
                has_special_representation: false,
            },
            LimitedPartner {
                name: "Pension Fund Deutschland".to_string(),
                address: "Unter den Linden 1, 10117 Berlin".to_string(),
                liability_limit: Capital::from_euros(300_000),
                contribution_paid: Capital::from_euros(300_000),
                partner_type: PartnerType::LegalEntity,
                has_special_representation: false,
            },
        ],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
    };

    println!("\n📄 GmbH & Co. KG Structure:");
    println!("   Partnership Name: {}", gmbh_co_kg.partnership_name);
    println!("\n🏢 GmbH General Partner (Komplementär-GmbH):");
    println!(
        "   Company Name:     {}",
        gmbh_co_kg.gmbh_general_partner.company_name
    );
    println!(
        "   Share Capital:    €{:.2}",
        gmbh_co_kg.gmbh_general_partner.share_capital.to_euros()
    );
    println!("   Managing Directors:");
    for (i, director) in gmbh_co_kg
        .gmbh_general_partner
        .managing_directors
        .iter()
        .enumerate()
    {
        println!("   {}. {}", i + 1, director);
    }
    println!("   Liability:        Unlimited (as GmbH)");
    println!("   → But GmbH shareholders have limited liability!");

    println!("\n👥 Limited Partners (Kommanditisten):");
    let total_limited_capital: u64 = gmbh_co_kg
        .limited_partners
        .iter()
        .map(|p| p.contribution_paid.amount_cents)
        .sum();

    for (i, partner) in gmbh_co_kg.limited_partners.iter().enumerate() {
        let percentage = (partner.contribution_paid.amount_cents as f64)
            / (total_limited_capital as f64)
            * 100.0;
        println!("   {}. {}", i + 1, partner.name);
        println!(
            "      Liability Limit: €{:.2} ({:.1}%)",
            partner.liability_limit.to_euros(),
            percentage
        );
        println!("      Type: {:?}", partner.partner_type);
    }

    println!("\n💰 Financial Summary:");
    println!(
        "   GmbH Capital:     €{:.2}",
        gmbh_co_kg.gmbh_general_partner.share_capital.to_euros()
    );
    println!(
        "   Limited Partners: €{:.2}",
        (total_limited_capital as f64) / 100.0
    );
    println!(
        "   Total Capital:    €{:.2}",
        gmbh_co_kg.gmbh_general_partner.share_capital.to_euros()
            + (total_limited_capital as f64) / 100.0
    );

    // Validate GmbH & Co. KG
    println!("\n🔍 Validating GmbH & Co. KG Structure...");
    match validate_gmbh_co_kg(&gmbh_co_kg) {
        Ok(()) => {
            println!("✅ GmbH & Co. KG Structure: VALID");
            println!("   - GmbH has minimum €25,000 capital (GmbHG §5)");
            println!("   - GmbH has managing directors");
            println!("   - At least 1 limited partner");
            println!("   - Partnership name includes 'GmbH & Co. KG'");
            println!("\n   ⭐ Effective Limited Liability for All:");
            println!("      - GmbH shareholders: Limited to share capital");
            println!("      - Kommanditisten: Limited to Haftsumme");
            println!("      - No partner has unlimited personal liability!");
        }
        Err(e) => {
            println!("❌ Validation FAILED: {}", e);
            return;
        }
    }

    // ==========================================================================
    // Example 3: Invalid KG (No Limited Partners)
    // ==========================================================================
    println!("\n\n📋 Example 3: Invalid KG (No Limited Partners)");
    println!("{}", "═".repeat(70));

    let invalid_kg = KG {
        partnership_name: "All General KG".to_string(),
        registered_office: "Hamburg".to_string(),
        business_purpose: "Handelsgeschäfte".to_string(),
        general_partners: vec![
            Partner {
                name: "Partner A".to_string(),
                address: "Hamburg".to_string(),
                contribution: Some(Capital::from_euros(10_000)),
                contribution_paid: Some(Capital::from_euros(10_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
            Partner {
                name: "Partner B".to_string(),
                address: "Hamburg".to_string(),
                contribution: Some(Capital::from_euros(10_000)),
                contribution_paid: Some(Capital::from_euros(10_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            },
        ],
        limited_partners: vec![], // No limited partners!
        formation_date: None,
        fiscal_year_end: None,
    };

    match validate_kg(&invalid_kg) {
        Ok(()) => println!("✅ Valid (unexpected!)"),
        Err(e) => {
            println!("❌ Validation FAILED (expected):");
            println!("   {}", e);
            println!("\n   ⚠️  A KG requires at least 1 limited partner (§161 Abs. 1 HGB).");
            println!(
                "   If all partners have unlimited liability, consider forming an OHG instead."
            );
        }
    }

    // ==========================================================================
    // Formation Summary
    // ==========================================================================
    println!("\n\n📊 KG Formation Summary:");
    println!("{}", "═".repeat(70));
    println!("✅ KG and GmbH & Co. KG formations demonstrated!\n");
    println!("📝 Next Steps for KG Formation:");
    println!("   1. ✍️  Draft partnership agreement (Gesellschaftsvertrag)");
    println!("      → Specify each limited partner's Haftsumme (§171 HGB)");
    println!("      → Define profit/loss distribution");
    println!("      → Clarify management authority (general partners only)");
    println!("\n   2. 📋 Commercial register entry (Handelsregistereintragung)");
    println!("      → Register with liability limits (§162 HGB)");
    println!("      → List all general and limited partners");
    println!("\n   3. 🏢 Business and tax registration");
    println!("      → Same as OHG (Gewerbeamt, Finanzamt)");

    println!("\n⚠️  Important Legal Considerations:");
    println!("{}", "═".repeat(70));
    println!("   📊 Liability Structure:");
    println!("      - General partners: Unlimited personal liability");
    println!("      - Limited partners: Limited to Haftsumme (§171 HGB)");
    println!("      - Once contribution paid, limited partner's liability ends");
    println!("\n   ⚙️  Management Rights:");
    println!("      - Only general partners can manage (§164 HGB)");
    println!("      - Limited partners have no management authority");
    println!("      - Limited partners can monitor and object (§166 HGB)");
    println!("\n   🏦 GmbH & Co. KG Advantages:");
    println!("      - Effective limited liability for all parties");
    println!("      - Transparent taxation like partnership");
    println!("      - Professional management through GmbH");
    println!("      - Common structure for family businesses and holdings");

    println!("\n{}", "═".repeat(70));
    println!("🎉 KG formation examples completed successfully!");
    println!("{}\n", "═".repeat(70));

    // ==========================================================================
    // Comparison Table
    // ==========================================================================
    println!("📊 Comparison: KG vs GmbH & Co. KG vs OHG");
    println!("{}", "═".repeat(70));
    println!("┌────────────────────┬──────────────┬─────────────────┬──────────┐");
    println!("│ Feature            │      KG      │ GmbH & Co. KG   │   OHG    │");
    println!("├────────────────────┼──────────────┼─────────────────┼──────────┤");
    println!("│ General Partners   │ 1+ (unlimited)│ 1 GmbH (limited)│ 2+ (unl.)│");
    println!("│ Limited Partners   │ 1+ (limited) │ 1+ (limited)    │   None   │");
    println!("│ Effective Liability│   Mixed      │ Limited (all)   │Unlimited │");
    println!("│ Management         │ Gen. partners│ GmbH directors  │   All    │");
    println!("│ Min. Capital       │     None     │ €25,000 (GmbH)  │   None   │");
    println!("│ Formation Cost     │     Low      │    Medium       │   Low    │");
    println!("│ Taxation           │ Transparent  │  Transparent    │Transparent│");
    println!("│ Complexity         │    Medium    │     High        │   Low    │");
    println!("│ Common Use Cases   │ Family biz   │ Holdings, funds │Small biz │");
    println!("└────────────────────┴──────────────┴─────────────────┴──────────┘");

    println!("\n📚 Legal References:");
    println!("   - §161-177a HGB: Limited partnership provisions");
    println!("   - §162 HGB: Commercial register entry with liability limits");
    println!("   - §164 HGB: Management by general partners only");
    println!("   - §170 HGB: Limited partners' representation restrictions");
    println!("   - §171 HGB: Liability limit (Haftsumme) and contribution");
    println!("   - §5 GmbHG: GmbH minimum capital (€25,000)\n");
}
