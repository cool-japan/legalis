//! Marriage Formation and Property Regimes Example
//!
//! Demonstrates marriage formation, property regimes, and accrued gains calculations
//! under German BGB §§1303-1390.

use chrono::NaiveDate;
use legalis_de::bgb::familienrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Family Law - Marriage Formation ===\n");
    println!("BGB Familienrecht - Eheschließung und Güterrecht\n");

    // =========================================================================
    // Example 1: Valid Marriage Formation (§§1303-1311 BGB)
    // =========================================================================
    println!("Example 1: Valid Marriage Formation");
    println!("----------------------------------------\n");

    let spouse1 = Person {
        name: "Hans Mueller".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1990, 5, 15).unwrap(),
        place_of_birth: "Berlin".to_string(),
        nationality: "German".to_string(),
        gender: Gender::Male,
        address: "Musterstrasse 1, 10115 Berlin".to_string(),
    };

    let spouse2 = Person {
        name: "Maria Schmidt".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1992, 8, 20).unwrap(),
        place_of_birth: "Munich".to_string(),
        nationality: "German".to_string(),
        gender: Gender::Female,
        address: "Beispielweg 5, 80331 Munich".to_string(),
    };

    let marriage = Marriage {
        spouse1: spouse1.clone(),
        spouse2: spouse2.clone(),
        marriage_date: NaiveDate::from_ymd_opt(2020, 6, 15).unwrap(),
        place_of_marriage: "Berlin".to_string(),
        registrar_office: "Standesamt Berlin-Mitte".to_string(),
        status: MarriageStatus::Valid,
        property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
        impediments: vec![],
    };

    println!("Marriage Details:");
    println!(
        "  Spouse 1: {}, born {}",
        marriage.spouse1.name, marriage.spouse1.date_of_birth
    );
    println!(
        "  Spouse 2: {}, born {}",
        marriage.spouse2.name, marriage.spouse2.date_of_birth
    );
    println!("  Marriage Date: {}", marriage.marriage_date);
    println!("  Place: {}", marriage.place_of_marriage);
    println!("  Registrar: {}", marriage.registrar_office);
    println!("  Property Regime: Community of Accrued Gains (DEFAULT)");
    println!();

    match validate_marriage(&marriage) {
        Ok(()) => {
            println!("✅ Marriage formation valid per §§1303-1311 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Both parties at least 18 years old (§1303 BGB)");
            println!("   ✓ No existing marriage (§1306 BGB)");
            println!("   ✓ Not closely related (§1307 BGB)");
            println!("   ✓ Both have legal capacity (§1304 BGB)");
            println!("   ✓ Ceremony before registrar (§1310 BGB)");
            println!();
            println!(
                "   Spouse 1 age at marriage: {} years",
                spouse1.age_at(marriage.marriage_date)
            );
            println!(
                "   Spouse 2 age at marriage: {} years",
                spouse2.age_at(marriage.marriage_date)
            );
            println!("   Marriage duration: {} years", marriage.duration_years());
        }
        Err(e) => println!("❌ Marriage invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Invalid Marriage - Below Minimum Age (§1303 BGB)
    // =========================================================================
    println!("Example 2: Invalid Marriage - Below Minimum Age");
    println!("----------------------------------------\n");

    let minor_spouse = Person {
        name: "Young Person".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(2008, 1, 1).unwrap(), // Only 16 years old
        place_of_birth: "Berlin".to_string(),
        nationality: "German".to_string(),
        gender: Gender::Female,
        address: "Test Street 1".to_string(),
    };

    let invalid_marriage = Marriage {
        spouse1: Person {
            name: "Adult Person".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            place_of_birth: "Berlin".to_string(),
            nationality: "German".to_string(),
            gender: Gender::Male,
            address: "Test Street 2".to_string(),
        },
        spouse2: minor_spouse.clone(),
        marriage_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        place_of_marriage: "Berlin".to_string(),
        registrar_office: "Standesamt Berlin".to_string(),
        status: MarriageStatus::Valid,
        property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
        impediments: vec![],
    };

    println!("Scenario: Attempt to marry with party below 18 years");
    println!(
        "  Party age: {} years",
        minor_spouse.age_at(invalid_marriage.marriage_date)
    );
    println!();

    match validate_marriage(&invalid_marriage) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Marriage fails: {}", e);
            println!();
            println!("   §1303 BGB: Minimum marriage age is 18 years");
            println!("   Previously: 16 with parental consent (abolished 2017)");
            println!("   Result: Marriage is void (nichtig)");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 3: Matrimonial Property Agreement (§§1408-1410 BGB)
    // =========================================================================
    println!("Example 3: Matrimonial Property Agreement (Ehevertrag)");
    println!("----------------------------------------\n");

    let property_agreement = MatrimonialPropertyAgreement {
        spouses: (spouse1.clone(), spouse2.clone()),
        agreement_date: NaiveDate::from_ymd_opt(2020, 5, 1).unwrap(), // Before marriage
        notarized: true,                                              // REQUIRED per §1410 BGB
        chosen_regime: MatrimonialPropertyRegime::SeparationOfProperty,
        special_provisions: vec![
            "No equalization of accrued gains upon divorce".to_string(),
            "Each spouse retains full control over their assets".to_string(),
        ],
    };

    println!("Agreement Details:");
    println!(
        "  Parties: {} and {}",
        property_agreement.spouses.0.name, property_agreement.spouses.1.name
    );
    println!("  Date: {}", property_agreement.agreement_date);
    println!(
        "  Notarized: {} (REQUIRED)",
        if property_agreement.notarized {
            "Yes"
        } else {
            "No"
        }
    );
    println!("  Chosen Regime: Separation of Property (Gütertrennung)");
    println!();

    match validate_matrimonial_property_agreement(&property_agreement) {
        Ok(()) => {
            println!("✅ Matrimonial property agreement valid per §§1408-1410 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Notarized (§1410 BGB) - MANDATORY");
            println!("   ✓ Agreement between spouses or future spouses");
            println!("   ✓ Valid chosen regime");
            println!();
            println!("   Effect: DEFAULT regime (community of accrued gains)");
            println!("           replaced by separation of property");
            println!("   Result: Upon divorce, no equalization of accrued gains");
        }
        Err(e) => println!("❌ Agreement invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 4: Accrued Gains Calculation (§§1372-1390 BGB)
    // =========================================================================
    println!("Example 4: Accrued Gains Equalization (Zugewinnausgleich)");
    println!("----------------------------------------\n");

    println!("Scenario: Marriage ended by divorce after 10 years");
    println!("  Property regime: Community of accrued gains (DEFAULT)");
    println!();

    // Spouse 1 assets at marriage start
    let spouse1_initial = Assets {
        real_estate_value: Capital::from_euros(100_000),
        movable_property_value: Capital::from_euros(20_000),
        bank_accounts: Capital::from_euros(10_000),
        securities: Capital::from_euros(0),
        business_interests: Capital::from_euros(0),
        other_assets: Capital::from_euros(0),
        liabilities: Capital::from_euros(30_000),
    };

    // Spouse 1 assets at marriage end
    let spouse1_final = Assets {
        real_estate_value: Capital::from_euros(200_000),
        movable_property_value: Capital::from_euros(30_000),
        bank_accounts: Capital::from_euros(50_000),
        securities: Capital::from_euros(20_000),
        business_interests: Capital::from_euros(0),
        other_assets: Capital::from_euros(0),
        liabilities: Capital::from_euros(50_000),
    };

    // Spouse 2 assets at marriage start
    let spouse2_initial = Assets {
        real_estate_value: Capital::from_euros(0),
        movable_property_value: Capital::from_euros(10_000),
        bank_accounts: Capital::from_euros(5_000),
        securities: Capital::from_euros(0),
        business_interests: Capital::from_euros(0),
        other_assets: Capital::from_euros(0),
        liabilities: Capital::from_euros(0),
    };

    // Spouse 2 assets at marriage end
    let spouse2_final = Assets {
        real_estate_value: Capital::from_euros(0),
        movable_property_value: Capital::from_euros(15_000),
        bank_accounts: Capital::from_euros(30_000),
        securities: Capital::from_euros(10_000),
        business_interests: Capital::from_euros(0),
        other_assets: Capital::from_euros(0),
        liabilities: Capital::from_euros(0),
    };

    let calculation = AccruedGainsCalculation {
        spouse1_initial_assets: spouse1_initial.clone(),
        spouse1_final_assets: spouse1_final.clone(),
        spouse2_initial_assets: spouse2_initial.clone(),
        spouse2_final_assets: spouse2_final.clone(),
        marriage_start_date: NaiveDate::from_ymd_opt(2015, 6, 15).unwrap(),
        marriage_end_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
    };

    println!("Spouse 1 Assets:");
    println!(
        "  Initial (2015): EUR {:.2}",
        spouse1_initial.net_value().to_euros()
    );
    println!(
        "    - Real estate: EUR {:.2}",
        spouse1_initial.real_estate_value.to_euros()
    );
    println!(
        "    - Movables: EUR {:.2}",
        spouse1_initial.movable_property_value.to_euros()
    );
    println!(
        "    - Bank accounts: EUR {:.2}",
        spouse1_initial.bank_accounts.to_euros()
    );
    println!(
        "    - Liabilities: EUR {:.2}",
        spouse1_initial.liabilities.to_euros()
    );
    println!(
        "  Final (2025): EUR {:.2}",
        spouse1_final.net_value().to_euros()
    );
    println!(
        "  Accrued Gain (§1373 BGB): EUR {:.2}",
        calculation.spouse1_accrued_gain().to_euros()
    );
    println!();

    println!("Spouse 2 Assets:");
    println!(
        "  Initial (2015): EUR {:.2}",
        spouse2_initial.net_value().to_euros()
    );
    println!(
        "  Final (2025): EUR {:.2}",
        spouse2_final.net_value().to_euros()
    );
    println!(
        "  Accrued Gain (§1373 BGB): EUR {:.2}",
        calculation.spouse2_accrued_gain().to_euros()
    );
    println!();

    let (claimant, equalization_amount) = calculation.equalization_claim();

    match validate_accrued_gains_calculation(&calculation) {
        Ok(()) => {
            println!("✅ Accrued gains calculation valid per §§1372-1390 BGB!");
            println!();
            println!("   Equalization Calculation (§1378 BGB):");
            println!(
                "   - Spouse 1 gain: EUR {:.2}",
                calculation.spouse1_accrued_gain().to_euros()
            );
            println!(
                "   - Spouse 2 gain: EUR {:.2}",
                calculation.spouse2_accrued_gain().to_euros()
            );
            println!(
                "   - Difference: EUR {:.2}",
                (calculation.spouse1_accrued_gain().to_euros()
                    - calculation.spouse2_accrued_gain().to_euros())
                .abs()
            );
            println!();
            println!("   Result: {:?} receives equalization claim", claimant);
            println!(
                "   Amount: EUR {:.2} (half of difference)",
                equalization_amount.to_euros()
            );
            println!();
            println!("   Policy Rationale:");
            println!("   - Recognizes both spouses' contributions to marriage");
            println!("   - Compensates spouse with lower economic gain");
            println!("   - Aims for fair distribution of marital wealth increase");
        }
        Err(e) => println!("❌ Calculation invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 5: Marriage with Impediment (§1306 BGB)
    // =========================================================================
    println!("Example 5: Marriage with Existing Marriage Impediment");
    println!("----------------------------------------\n");

    let bigamous_marriage = Marriage {
        spouse1: Person {
            name: "Already Married Person".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1985, 1, 1).unwrap(),
            place_of_birth: "Berlin".to_string(),
            nationality: "German".to_string(),
            gender: Gender::Male,
            address: "Test Street".to_string(),
        },
        spouse2: Person {
            name: "New Partner".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            place_of_birth: "Munich".to_string(),
            nationality: "German".to_string(),
            gender: Gender::Female,
            address: "Test Avenue".to_string(),
        },
        marriage_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        place_of_marriage: "Hamburg".to_string(),
        registrar_office: "Standesamt Hamburg".to_string(),
        status: MarriageStatus::Invalid,
        property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
        impediments: vec![MarriageImpediment::ExistingMarriage],
    };

    println!("Scenario: One party already married");
    println!();

    match validate_marriage(&bigamous_marriage) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Marriage fails: {}", e);
            println!();
            println!("   §1306 BGB: Existing Marriage Impediment");
            println!("   A marriage cannot be contracted if one party is already married");
            println!();
            println!("   Effect: Marriage is void (nichtig)");
            println!("   Germany recognizes only monogamous marriage");
            println!("   Previous marriage must be dissolved (divorce or death)");
        }
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: German Marriage Law ===");
    println!();
    println!("§§1303-1311 BGB - Marriage Formation:");
    println!("  Requirements:");
    println!("  - Both parties at least 18 years old (§1303)");
    println!("  - No existing marriage (§1306)");
    println!("  - Not closely related (§1307)");
    println!("  - Both have legal capacity (§1304)");
    println!("  - Civil ceremony before registrar (§1310)");
    println!();
    println!("§§1363-1390 BGB - Property Regimes:");
    println!("  1. Community of Accrued Gains (DEFAULT):");
    println!("     - Separate property during marriage");
    println!("     - Equalization upon divorce/death");
    println!("  2. Separation of Property:");
    println!("     - Complete separation, no equalization");
    println!("  3. Community of Property (rare):");
    println!("     - Joint ownership");
    println!();
    println!("§§1408-1410 BGB - Matrimonial Property Agreement:");
    println!("  - Must be notarized (MANDATORY)");
    println!("  - Can choose different property regime");
    println!("  - Can include special provisions");
    println!();
    println!("All examples demonstrate correct BGB family law application!");
}
