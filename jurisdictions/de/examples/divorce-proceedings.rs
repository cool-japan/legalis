//! Divorce Proceedings and Maintenance Example
//!
//! Demonstrates divorce requirements, maintenance obligations, and parental custody
//! under German BGB §§1564-1698.

use chrono::{NaiveDate, Utc};
use legalis_de::bgb::familienrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Family Law - Divorce Proceedings ===\n");
    println!("BGB Familienrecht - Scheidung und Unterhalt\n");

    // Create a valid marriage first
    let spouse1 = Person {
        name: "Hans Mueller".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1985, 3, 10).unwrap(),
        place_of_birth: "Berlin".to_string(),
        nationality: "German".to_string(),
        gender: Gender::Male,
        address: "Musterstrasse 1, 10115 Berlin".to_string(),
    };

    let spouse2 = Person {
        name: "Maria Mueller".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1987, 7, 22).unwrap(),
        place_of_birth: "Munich".to_string(),
        nationality: "German".to_string(),
        gender: Gender::Female,
        address: "Beispielweg 5, 80331 Munich".to_string(),
    };

    let marriage = Marriage {
        spouse1: spouse1.clone(),
        spouse2: spouse2.clone(),
        marriage_date: NaiveDate::from_ymd_opt(2015, 6, 15).unwrap(),
        place_of_marriage: "Berlin".to_string(),
        registrar_office: "Standesamt Berlin-Mitte".to_string(),
        status: MarriageStatus::Valid,
        property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
        impediments: vec![],
    };

    // =========================================================================
    // Example 1: Valid Divorce with Mutual Consent (§1566 Abs. 1 BGB)
    // =========================================================================
    println!("Example 1: Divorce with Mutual Consent (1 year separation)");
    println!("----------------------------------------\n");

    let divorce = Divorce {
        marriage: marriage.clone(),
        filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        separation_date: NaiveDate::from_ymd_opt(2022, 12, 1).unwrap(), // 13 months before filing
        ground: DivorceGround::MarriageBreakdown,
        mutual_consent: true,
        divorce_decree_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
        accrued_gains_equalization: None,
        pension_equalization: None,
    };

    println!("Divorce Details:");
    println!(
        "  Spouses: {} and {}",
        divorce.marriage.spouse1.name, divorce.marriage.spouse2.name
    );
    println!("  Marriage Date: {}", divorce.marriage.marriage_date);
    println!("  Separation Date: {}", divorce.separation_date);
    println!("  Filing Date: {}", divorce.filing_date);
    println!(
        "  Separation Period: {} months",
        divorce.separation_period_months()
    );
    println!("  Mutual Consent: Yes");
    println!();

    match validate_divorce(&divorce) {
        Ok(()) => {
            println!("✅ Divorce valid per §§1564-1566 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Marriage breakdown (Ehescheitern) - §1565 BGB");
            println!("   ✓ Separation period: 1 year with mutual consent (§1566 Abs. 1)");
            println!();
            println!("   §1566 Abs. 1 BGB: Marriage presumed irretrievably broken");
            println!("   after 1 year separation if both spouses consent to divorce");
            println!();
            println!("   Divorce consequences:");
            println!("   - Marriage dissolved (Ehe aufgelöst)");
            println!("   - Accrued gains equalization (Zugewinnausgleich) if applicable");
            println!("   - Pension equalization (Versorgungsausgleich) required");
            println!("   - Post-marital maintenance may be owed (§§1569-1586)");
        }
        Err(e) => println!("❌ Divorce invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Divorce Without Consent (§1566 Abs. 2 BGB)
    // =========================================================================
    println!("Example 2: Divorce Without Consent (3 years separation)");
    println!("----------------------------------------\n");

    let no_consent_divorce = Divorce {
        marriage: marriage.clone(),
        filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        separation_date: NaiveDate::from_ymd_opt(2020, 10, 1).unwrap(), // 39 months before filing
        ground: DivorceGround::MarriageBreakdown,
        mutual_consent: false, // One spouse opposes divorce
        divorce_decree_date: Some(NaiveDate::from_ymd_opt(2024, 8, 1).unwrap()),
        accrued_gains_equalization: None,
        pension_equalization: None,
    };

    println!("Scenario: One spouse opposes divorce");
    println!(
        "  Separation Period: {} months",
        no_consent_divorce.separation_period_months()
    );
    println!("  Mutual Consent: No");
    println!();

    match validate_divorce(&no_consent_divorce) {
        Ok(()) => {
            println!("✅ Divorce valid per §1566 Abs. 2 BGB!");
            println!("   §1566 Abs. 2: Marriage presumed irretrievably broken");
            println!("   after 3 years separation WITHOUT consent requirement");
            println!();
            println!("   Rationale: Even if one spouse wishes to continue marriage,");
            println!("   3-year separation demonstrates definitive breakdown");
        }
        Err(e) => println!("❌ Divorce invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 3: Invalid Divorce - Insufficient Separation (§1566 BGB)
    // =========================================================================
    println!("Example 3: Invalid Divorce - Insufficient Separation Period");
    println!("----------------------------------------\n");

    let insufficient_separation = Divorce {
        marriage: marriage.clone(),
        filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        separation_date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(), // Only 6 months
        ground: DivorceGround::MarriageBreakdown,
        mutual_consent: true, // Requires 12 months
        divorce_decree_date: None,
        accrued_gains_equalization: None,
        pension_equalization: None,
    };

    println!("Scenario: Attempt to divorce after only 6 months separation");
    println!("  Required with consent: 12 months");
    println!(
        "  Actual: {} months",
        insufficient_separation.separation_period_months()
    );
    println!();

    match validate_divorce(&insufficient_separation) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Divorce fails: {}", e);
            println!();
            println!("   §1566 BGB: Separation Period Requirements");
            println!("   - 1 year (12 months) with mutual consent");
            println!("   - 3 years (36 months) without consent");
            println!();
            println!("   Policy: Separation period allows for reconciliation");
            println!("   and ensures marriage is truly irretrievably broken");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 4: Post-Marital Maintenance (§§1569-1586 BGB)
    // =========================================================================
    println!("Example 4: Post-Marital Maintenance (Nachehelicher Unterhalt)");
    println!("----------------------------------------\n");

    let maintenance = PostMaritalMaintenance {
        claimant: spouse2.clone(),
        obligor: spouse1.clone(),
        ground: MaintenanceGround::ChildCare, // §1570 BGB
        monthly_amount: Capital::from_euros(1_500),
        start_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(), // After divorce decree
        end_date: Some(NaiveDate::from_ymd_opt(2027, 6, 1).unwrap()), // Limited to 3 years
        limited_duration: true,                                   // §1578b BGB
    };

    println!("Maintenance Details:");
    println!("  Obligor: {} (paying spouse)", maintenance.obligor.name);
    println!(
        "  Claimant: {} (receiving spouse)",
        maintenance.claimant.name
    );
    println!("  Ground: Child Care (§1570 BGB)");
    println!(
        "  Monthly Amount: EUR {:.2}",
        maintenance.monthly_amount.to_euros()
    );
    println!("  Duration: Limited to 3 years (§1578b BGB)");
    println!();

    match validate_post_marital_maintenance(&maintenance) {
        Ok(()) => {
            println!("✅ Post-marital maintenance valid per §§1569-1586 BGB!");
            println!();
            println!("   §1570 BGB - Child Care Maintenance:");
            println!("   Divorced spouse caring for joint child entitled to maintenance");
            println!();
            println!("   Other maintenance grounds (§§1571-1576 BGB):");
            println!("   - Age (§1571)");
            println!("   - Illness (§1572)");
            println!("   - Unemployment (§1573)");
            println!("   - Additional training (§1575)");
            println!("   - Equity (§1576)");
            println!();
            println!("   §1578b BGB - Temporal Limitation:");
            println!("   Court may limit maintenance duration based on:");
            println!("   - Marriage duration");
            println!("   - Childcare needs");
            println!("   - Expectation of self-support");
        }
        Err(e) => println!("❌ Maintenance invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 5: Parental Custody (§§1626-1698 BGB)
    // =========================================================================
    println!("Example 5: Parental Custody (Elterliche Sorge)");
    println!("----------------------------------------\n");

    let child = Person {
        name: "Sophie Mueller".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(2018, 3, 15).unwrap(), // 6 years old
        place_of_birth: "Berlin".to_string(),
        nationality: "German".to_string(),
        gender: Gender::Female,
        address: "With mother".to_string(),
    };

    let custody = ParentalCustody {
        child: child.clone(),
        custody_holders: vec![spouse1.clone(), spouse2.clone()],
        custody_type: CustodyType::Joint, // DEFAULT for married parents
        established_date: child.date_of_birth,
    };

    println!("Custody Details:");
    println!(
        "  Child: {}, born {}",
        custody.child.name, custody.child.date_of_birth
    );
    println!(
        "  Child Age: {} years",
        child.age_at(Utc::now().date_naive())
    );
    println!("  Custody Type: Joint Custody (gemeinsame Sorge)");
    println!(
        "  Custody Holders: {} and {}",
        custody.custody_holders[0].name, custody.custody_holders[1].name
    );
    println!();

    match validate_parental_custody(&custody) {
        Ok(()) => {
            println!("✅ Parental custody valid per §§1626-1698 BGB!");
            println!();
            println!("   §1626 BGB - Parental Custody:");
            println!("   Parents have duty and right to care for child");
            println!();
            println!("   Joint Custody (gemeinsame Sorge):");
            println!("   - DEFAULT for married parents");
            println!("   - Continues after divorce unless court orders otherwise");
            println!("   - Both parents make major decisions together");
            println!("   - Day-to-day decisions by parent with whom child lives");
            println!();
            println!("   Sole Custody (Alleinsorge):");
            println!("   - Court may award if joint custody not in child's best interest");
            println!("   - One parent has exclusive decision-making authority");
            println!();
            println!("   Custody ends: When child reaches 18 (majority)");
        }
        Err(e) => println!("❌ Custody invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 6: Maintenance Obligation (§§1601-1615 BGB)
    // =========================================================================
    println!("Example 6: Parent-Child Maintenance Obligation");
    println!("----------------------------------------\n");

    let child_maintenance = MaintenanceObligation {
        obligor: spouse1.clone(),
        beneficiary: child.clone(),
        relationship: MaintenanceRelationship::ParentToChild,
        monthly_amount: Capital::from_euros(500),
        start_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
        end_date: None, // Until child reaches adulthood or completes education
    };

    println!("Child Maintenance:");
    println!("  Obligor: {} (parent)", child_maintenance.obligor.name);
    println!(
        "  Beneficiary: {} (child)",
        child_maintenance.beneficiary.name
    );
    println!(
        "  Monthly Amount: EUR {:.2}",
        child_maintenance.monthly_amount.to_euros()
    );
    println!();

    match validate_maintenance_obligation(&child_maintenance) {
        Ok(()) => {
            println!("✅ Maintenance obligation valid per §§1601-1615 BGB!");
            println!();
            println!("   §1601 BGB - Maintenance Obligation:");
            println!("   Relatives in direct line obligated to provide maintenance");
            println!();
            println!("   Parent-to-Child Maintenance:");
            println!("   - Both parents obligated (§1601)");
            println!("   - Duty extends until child can support themselves");
            println!("   - Usually: Until completion of first vocational training");
            println!("   - Amount determined by Düsseldorf Table (Düsseldorfer Tabelle)");
            println!();
            println!("   Priority Order (§1609 BGB):");
            println!("   1. Minor children and children in education (up to age 21)");
            println!("   2. Parent caring for children under 3");
            println!("   3. Other relatives");
        }
        Err(e) => println!("❌ Maintenance invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: German Divorce and Maintenance Law ===");
    println!();
    println!("§§1564-1587 BGB - Divorce:");
    println!("  Ground: Marriage breakdown (Ehescheitern - §1565)");
    println!("  Separation periods:");
    println!("  - 1 year with mutual consent (§1566 Abs. 1)");
    println!("  - 3 years without consent (§1566 Abs. 2)");
    println!();
    println!("§§1569-1586 BGB - Post-Marital Maintenance:");
    println!("  Grounds: Child care, age, illness, unemployment, training, equity");
    println!("  Temporal limitation possible (§1578b)");
    println!();
    println!("§§1601-1615 BGB - Maintenance Obligations:");
    println!("  Between: Parents-children, spouses, other relatives");
    println!("  Priority: Minor children first");
    println!();
    println!("§§1626-1698 BGB - Parental Custody:");
    println!("  DEFAULT: Joint custody for married parents");
    println!("  Continues after divorce unless court orders otherwise");
    println!("  Ends: When child reaches 18 (adulthood)");
    println!();
    println!("All examples demonstrate correct BGB family law application!");
}
