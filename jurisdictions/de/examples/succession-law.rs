//! Legal Succession Law Example (Gesetzliche Erbfolge)
//!
//! Demonstrates German intestate succession under BGB Book 5 (§§1922-1936):
//! - Order system (First/Second/Third/Fourth orders)
//! - Spouse inheritance with property regimes
//! - Right of representation
//! - Acceptance and renunciation decisions

use chrono::NaiveDate;
use legalis_de::bgb::erbrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Legal Succession Law Examples ===\n");
    println!("Gesetzliche Erbfolge nach BGB Buch 5 (§§1922-1936)\n");

    // Example 1: First Order Succession - Two Children
    println!("📋 Example 1: First Order - Two Children (§1924 BGB)");
    println!("Deceased has 2 children, no will, estate worth €100,000\n");

    let deceased1 = Deceased {
        name: "Hans Mueller".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1950, 3, 15).unwrap(),
        date_of_death: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
        place_of_death: "Berlin".to_string(),
        last_residence: "Alexanderplatz 5, 10178 Berlin".to_string(),
        nationality: "German".to_string(),
    };

    println!(
        "Deceased: {} (age at death: {})",
        deceased1.name,
        deceased1.age_at_death()
    );

    let succession1 = LegalSuccession {
        deceased: deceased1.clone(),
        heirs: vec![
            Heir {
                name: "Maria Mueller".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1980, 5, 20).unwrap(),
                relationship: RelationshipToDeceased::Child,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 2,
                },
                is_statutory_heir: true,
            },
            Heir {
                name: "Thomas Mueller".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1982, 8, 10).unwrap(),
                relationship: RelationshipToDeceased::Child,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 2,
                },
                is_statutory_heir: true,
            },
        ],
        succession_order: SuccessionOrder::First,
        spouse_inheritance: None,
    };

    match validate_legal_succession(&succession1) {
        Ok(()) => {
            println!("✅ Legal Succession: VALID");
            println!("   - Order: First (Descendants)");
            println!("   - Heirs: {} statutory heirs", succession1.heirs.len());
            for heir in &succession1.heirs {
                if let Some(decimal) = heir.inheritance_share.as_decimal() {
                    println!(
                        "     - {}: {:.0}% (€{:.2})",
                        heir.name,
                        decimal * 100.0,
                        decimal * 100_000.0
                    );
                }
            }
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 2: First Order with Spouse (Community of Accrued Gains)
    println!("\n📋 Example 2: First Order with Spouse (§1931 BGB)");
    println!("Deceased has spouse + 2 children, community of accrued gains\n");

    let succession2 = LegalSuccession {
        deceased: deceased1.clone(),
        heirs: vec![
            Heir {
                name: "Maria Mueller".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1980, 5, 20).unwrap(),
                relationship: RelationshipToDeceased::Child,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 4,
                },
                is_statutory_heir: true,
            },
            Heir {
                name: "Thomas Mueller".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1982, 8, 10).unwrap(),
                relationship: RelationshipToDeceased::Child,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 4,
                },
                is_statutory_heir: true,
            },
        ],
        succession_order: SuccessionOrder::First,
        spouse_inheritance: Some(SpouseInheritance {
            spouse_name: "Erika Mueller".to_string(),
            marriage_date: NaiveDate::from_ymd_opt(1975, 6, 15).unwrap(),
            matrimonial_property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
            share: InheritanceShare::Fraction {
                numerator: 1,
                denominator: 2, // 1/4 basic + 1/4 accrued gains bonus
            },
        }),
    };

    match validate_legal_succession(&succession2) {
        Ok(()) => {
            println!("✅ Legal Succession with Spouse: VALID");
            println!(
                "   - Spouse: {} (1/2 = 50%)",
                succession2.spouse_inheritance.as_ref().unwrap().spouse_name
            );
            println!("     - 1/4 basic share + 1/4 accrued gains bonus");
            println!("     - Property regime: Community of Accrued Gains (default)");
            println!("   - Children: 2 children share remaining 1/2");
            for heir in &succession2.heirs {
                if let Some(decimal) = heir.inheritance_share.as_decimal() {
                    println!(
                        "     - {}: {:.0}% (€{:.2})",
                        heir.name,
                        decimal * 100.0,
                        decimal * 100_000.0
                    );
                }
            }
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 3: Second Order - Parents and Siblings
    println!("\n📋 Example 3: Second Order - Parents (§1925 BGB)");
    println!("Deceased has no descendants, parents still alive\n");

    let deceased3 = Deceased {
        name: "Julia Schmidt".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1985, 7, 20).unwrap(),
        date_of_death: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
        place_of_death: "Munich".to_string(),
        last_residence: "Munich".to_string(),
        nationality: "German".to_string(),
    };

    let succession3 = LegalSuccession {
        deceased: deceased3,
        heirs: vec![
            Heir {
                name: "Father Schmidt".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1955, 3, 10).unwrap(),
                relationship: RelationshipToDeceased::Parent,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 2,
                },
                is_statutory_heir: true,
            },
            Heir {
                name: "Mother Schmidt".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1958, 9, 5).unwrap(),
                relationship: RelationshipToDeceased::Parent,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 2,
                },
                is_statutory_heir: true,
            },
        ],
        succession_order: SuccessionOrder::Second,
        spouse_inheritance: None,
    };

    match validate_legal_succession(&succession3) {
        Ok(()) => {
            println!("✅ Second Order Succession: VALID");
            println!("   - Order: Second (Parents and their descendants)");
            println!("   - No descendants → parents inherit");
            println!("   - Both parents alive → 50% each");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 4: Right of Representation
    println!("\n📋 Example 4: Right of Representation (§1924 Abs. 2 BGB)");
    println!("Deceased's child predeceased, grandchildren inherit by representation\n");

    let representation = RightOfRepresentation {
        original_heir: "Maria Mueller (deceased)".to_string(),
        original_heir_deceased: true,
        representing_heirs: vec!["Grandson 1".to_string(), "Grandson 2".to_string()],
    };

    println!("Original heir: {}", representation.original_heir);
    println!("Representing heirs (grandchildren):");
    for heir in &representation.representing_heirs {
        println!("  - {}", heir);
    }
    println!("\n💡 Legal Effect:");
    println!("   - Grandchildren inherit in place of deceased parent");
    println!("   - They split their parent's share equally (1/4 each)");
    println!("   - This is 'succession by representation' (Eintrittsrecht)");

    // Example 5: Inheritance Decision - Acceptance
    println!("\n📋 Example 5: Inheritance Acceptance (§1942 BGB)");
    println!("Heir accepts inheritance within 6-week deadline\n");

    let decision_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    let deadline = NaiveDate::from_ymd_opt(2024, 3, 7).unwrap(); // 6 weeks later

    let acceptance_decision = InheritanceDecision {
        heir: Heir {
            name: "Maria Mueller".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 5, 20).unwrap(),
            relationship: RelationshipToDeceased::Child,
            inheritance_share: InheritanceShare::Fraction {
                numerator: 1,
                denominator: 2,
            },
            is_statutory_heir: true,
        },
        decision: InheritanceDecisionType::Acceptance,
        decision_date,
        deadline,
    };

    println!("Heir: {}", acceptance_decision.heir.name);
    println!("Decision: Acceptance (Annahme)");
    println!("Decision date: {}", acceptance_decision.decision_date);
    println!(
        "Deadline: {} (6 weeks from knowledge)",
        acceptance_decision.deadline
    );
    println!("\n💡 Legal Effect:");
    println!("   - Heir accepts all assets AND liabilities");
    println!("   - Becomes universal successor (Gesamtrechtsnachfolge)");
    println!("   - Cannot later renounce (decision is final)");

    // Example 6: Inheritance Decision - Renunciation
    println!("\n📋 Example 6: Inheritance Renunciation (§1943 BGB)");
    println!("Heir renounces inheritance due to insolvency\n");

    let renunciation_decision = InheritanceDecision {
        heir: Heir {
            name: "Thomas Mueller".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1982, 8, 10).unwrap(),
            relationship: RelationshipToDeceased::Child,
            inheritance_share: InheritanceShare::Fraction {
                numerator: 1,
                denominator: 2,
            },
            is_statutory_heir: true,
        },
        decision: InheritanceDecisionType::Renunciation,
        decision_date,
        deadline,
    };

    println!("Heir: {}", renunciation_decision.heir.name);
    println!("Decision: Renunciation (Ausschlagung)");
    println!("Reason: Estate is insolvent (liabilities > assets)");
    println!("\n💡 Legal Effect:");
    println!("   - Heir is treated as if they predeceased testator");
    println!("   - Inheritance passes to next in line");
    println!("   - Heir's children may inherit by representation");
    println!("   - Cannot be withdrawn (§1955 BGB)");

    // Example 7: Estate Calculation
    println!("\n📋 Example 7: Estate Net Value Calculation (Nachlass)");
    println!("Estate with assets and liabilities\n");

    let estate = Estate {
        deceased: deceased1.clone(),
        total_value: Capital::from_euros(250_000),
        assets: vec![
            Asset {
                description: "Family home in Berlin".to_string(),
                asset_type: AssetType::RealEstate,
                value: Capital::from_euros(200_000),
            },
            Asset {
                description: "Bank accounts".to_string(),
                asset_type: AssetType::BankAccount,
                value: Capital::from_euros(30_000),
            },
            Asset {
                description: "Car and personal property".to_string(),
                asset_type: AssetType::MovableProperty,
                value: Capital::from_euros(20_000),
            },
        ],
        liabilities: vec![
            Liability {
                description: "Mortgage on family home".to_string(),
                creditor: "Deutsche Bank".to_string(),
                amount: Capital::from_euros(80_000),
            },
            Liability {
                description: "Credit card debt".to_string(),
                creditor: "Various creditors".to_string(),
                amount: Capital::from_euros(5_000),
            },
        ],
        net_value: Capital::from_euros(165_000),
    };

    let calculated_net = estate.calculate_net_value();

    println!("Assets:");
    for asset in &estate.assets {
        println!("  + {:?}: €{:.2}", asset.asset_type, asset.value.to_euros());
    }
    println!(
        "  = Total Assets: €{:.2}",
        estate
            .assets
            .iter()
            .map(|a| a.value.to_euros())
            .sum::<f64>()
    );

    println!("\nLiabilities:");
    for liability in &estate.liabilities {
        println!(
            "  - {}: €{:.2}",
            liability.description,
            liability.amount.to_euros()
        );
    }
    println!(
        "  = Total Liabilities: €{:.2}",
        estate
            .liabilities
            .iter()
            .map(|l| l.amount.to_euros())
            .sum::<f64>()
    );

    println!("\n✅ Net Estate Value: €{:.2}", calculated_net.to_euros());
    println!("   - This is the amount heirs will inherit");
    println!("   - Each heir receives their share of net value");

    // Summary
    println!("\n=== Summary: German Legal Succession Law ===");
    println!("\n📚 Order System (Ordnungssystem):");
    println!("   1. First Order (§1924): Descendants (children, grandchildren)");
    println!("   2. Second Order (§1925): Parents and their descendants");
    println!("   3. Third Order (§1926): Grandparents and their descendants");
    println!("   4. Fourth Order (§1928): Great-grandparents");
    println!("\n💍 Spouse Inheritance (§1931):");
    println!("   - With First Order: 1/2 (community of accrued gains)");
    println!("   - With Second Order: 3/4 (community of accrued gains)");
    println!("   - Alone: 100% (no relatives)");
    println!("\n⚖️ Key Principles:");
    println!("   - Higher orders exclude lower orders completely");
    println!("   - Children inherit equally by heads (per capita)");
    println!("   - Right of representation for deceased heirs");
    println!("   - 6-week deadline for acceptance/renunciation");
    println!("   - Heirs inherit both assets AND liabilities");
}
