//! Will Formalities and Compulsory Portion Example
//!
//! Demonstrates German testamentary succession (§§2064-2338 BGB):
//! - Holographic will requirements (§2247 BGB)
//! - Public will (§2232 BGB)
//! - Testamentary capacity (§2229 BGB)
//! - Compulsory portion calculations (§2303 BGB)
//! - Will validation and common errors

use chrono::NaiveDate;
use legalis_de::bgb::erbrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Will Formalities and Compulsory Portion Examples ===\n");
    println!("Testamentary Succession (Gewillkürte Erbfolge) - BGB Book 5\n");

    // Example 1: Valid Holographic Will
    println!("📋 Example 1: Valid Holographic Will (§2247 BGB)");
    println!("Entirely handwritten and signed by testator\n");

    let deceased = Deceased {
        name: "Hans Mueller".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1950, 3, 15).unwrap(),
        date_of_death: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
        place_of_death: "Berlin".to_string(),
        last_residence: "Berlin".to_string(),
        nationality: "German".to_string(),
    };

    let valid_will = Will {
        testator: deceased.clone(),
        will_type: WillType::Holographic,
        created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        place_of_creation: "Berlin".to_string(),
        is_handwritten: true, // REQUIRED for holographic will
        has_signature: true,  // REQUIRED
        has_date: true,       // Recommended but not mandatory
        beneficiaries: vec![
            WillBeneficiary {
                name: "Maria Mueller".to_string(),
                relationship: RelationshipToDeceased::Spouse,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 3,
                    denominator: 4,
                },
                conditions: vec![],
            },
            WillBeneficiary {
                name: "Thomas Mueller".to_string(),
                relationship: RelationshipToDeceased::Child,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 4,
                },
                conditions: vec![],
            },
        ],
        revoked: false,
        revoked_at: None,
    };

    match validate_holographic_will(&valid_will) {
        Ok(()) => {
            println!("✅ Holographic Will: VALID");
            println!("   - Type: Holographic (Eigenhändiges Testament)");
            println!("   - Handwritten: ✓ (required per §2247 Abs. 1)");
            println!("   - Signature: ✓ (required)");
            println!("   - Date: ✓ (recommended)");
            println!("   - Beneficiaries:");
            for beneficiary in &valid_will.beneficiaries {
                if let Some(decimal) = beneficiary.inheritance_share.as_decimal() {
                    println!("     - {}: {:.0}%", beneficiary.name, decimal * 100.0);
                }
            }
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 2: Invalid Will - Not Handwritten
    println!("\n📋 Example 2: Invalid Will - Not Handwritten");
    println!("Typed or printed will is INVALID for holographic form\n");

    let invalid_will_typed = Will {
        testator: deceased.clone(),
        will_type: WillType::Holographic,
        created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        place_of_creation: "Berlin".to_string(),
        is_handwritten: false, // INVALID - must be handwritten!
        has_signature: true,
        has_date: true,
        beneficiaries: vec![WillBeneficiary {
            name: "Charity Organization".to_string(),
            relationship: RelationshipToDeceased::NotRelated,
            inheritance_share: InheritanceShare::Full,
            conditions: vec![],
        }],
        revoked: false,
        revoked_at: None,
    };

    match validate_holographic_will(&invalid_will_typed) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error: {}", e);
            println!("   → §2247 Abs. 1 BGB requires ENTIRE will be handwritten");
        }
    }

    // Example 3: Invalid Will - Missing Signature
    println!("\n📋 Example 3: Invalid Will - Missing Signature");
    println!("Unsigned will is INVALID\n");

    let invalid_will_unsigned = Will {
        testator: deceased.clone(),
        will_type: WillType::Holographic,
        created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        place_of_creation: "Berlin".to_string(),
        is_handwritten: true,
        has_signature: false, // INVALID - must be signed!
        has_date: true,
        beneficiaries: vec![WillBeneficiary {
            name: "Son".to_string(),
            relationship: RelationshipToDeceased::Child,
            inheritance_share: InheritanceShare::Full,
            conditions: vec![],
        }],
        revoked: false,
        revoked_at: None,
    };

    match validate_holographic_will(&invalid_will_unsigned) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error: {}", e);
            println!("   → Signature is mandatory (§2247 Abs. 1 BGB)");
        }
    }

    // Example 4: Public Will
    println!("\n📋 Example 4: Public Will (§2232 BGB)");
    println!("Will declared before notary or deposited with court\n");

    let public_will = Will {
        testator: deceased.clone(),
        will_type: WillType::Public,
        created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        place_of_creation: "Notary Office Berlin".to_string(),
        is_handwritten: false, // Not required for public will
        has_signature: true,
        has_date: true,
        beneficiaries: vec![WillBeneficiary {
            name: "Daughter".to_string(),
            relationship: RelationshipToDeceased::Child,
            inheritance_share: InheritanceShare::Full,
            conditions: vec!["Must complete university education".to_string()],
        }],
        revoked: false,
        revoked_at: None,
    };

    match validate_will(&public_will) {
        Ok(()) => {
            println!("✅ Public Will: VALID");
            println!("   - Type: Public (Öffentliches Testament)");
            println!("   - Declared before notary (§2232 BGB)");
            println!("   - Handwritten requirement: N/A (not required)");
            println!("   - More formal but clearer than holographic");
            if let Some(beneficiary) = public_will.beneficiaries.first()
                && !beneficiary.conditions.is_empty()
            {
                println!("   - Conditions:");
                for condition in &beneficiary.conditions {
                    println!("     - {}", condition);
                }
            }
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 5: Testamentary Capacity - Age Rules
    println!("\n📋 Example 5: Testamentary Capacity (§2229 BGB)");
    println!("Age requirements for making a valid will\n");

    let ages_to_test = vec![(15, "Under 16"), (17, "Age 16-17"), (25, "Age 18+")];

    for (age, description) in ages_to_test {
        match validate_testamentary_capacity(age) {
            Ok(capacity) => {
                println!("Age {}: {} - {:?}", age, description, capacity);
                match capacity {
                    TestamentaryCapacity::None => {
                        println!("   → No capacity to make will");
                    }
                    TestamentaryCapacity::Limited => {
                        println!("   → Limited capacity (holographic or public only)");
                        println!("   → Cannot make joint wills");
                    }
                    TestamentaryCapacity::Full => {
                        println!("   → Full testamentary capacity");
                    }
                }
            }
            Err(e) => println!("Age {}: {} - Error: {}", age, description, e),
        }
    }

    // Example 6: Compulsory Portion (Pflichtteil)
    println!("\n📋 Example 6: Compulsory Portion Calculation (§2303 BGB)");
    println!("Disinherited child entitled to 1/2 of legal share\n");

    let estate_value = Capital::from_euros(200_000);

    let compulsory_portion = CompulsoryPortion {
        claimant: CompulsoryPortionClaimant {
            name: "Disinherited Son".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1985, 5, 15).unwrap(),
            relationship: RelationshipToDeceased::Child,
        },
        deceased: deceased.clone(),
        estate_value,
        portion: InheritanceShare::Fraction {
            numerator: 1,
            denominator: 4, // 1/2 of 1/2 (if 2 children)
        },
        amount: Capital::from_euros(50_000),
    };

    match validate_compulsory_portion(&compulsory_portion) {
        Ok(()) => {
            println!("✅ Compulsory Portion: VALID");
            println!("   - Claimant: {}", compulsory_portion.claimant.name);
            println!(
                "   - Relationship: {:?}",
                compulsory_portion.claimant.relationship
            );
            println!("   - Estate value: €{:.2}", estate_value.to_euros());
            println!("   - Legal share (if no will): 1/2 (€100,000)");
            println!("   - Compulsory portion: 1/2 of legal share");
            println!(
                "   - Amount: €{:.2}",
                compulsory_portion.calculate_amount().to_euros()
            );
            println!("\n💡 Legal Effect:");
            println!("   - Monetary claim against heirs (not asset claim)");
            println!("   - Cannot be disinherited below compulsory portion");
            println!("   - Must be claimed within 3 years (§2332 BGB)");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 7: Compulsory Portion - Not Entitled
    println!("\n📋 Example 7: No Compulsory Portion - Sibling");
    println!("Siblings are NOT entitled to compulsory portion\n");

    let invalid_claimant = CompulsoryPortionClaimant {
        name: "Sister".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1952, 8, 20).unwrap(),
        relationship: RelationshipToDeceased::Sibling,
    };

    println!("Claimant: {} (Sibling)", invalid_claimant.name);
    println!(
        "Entitled to compulsory portion? {}",
        invalid_claimant.is_entitled()
    );
    println!("\n💡 Only descendants, parents (if no descendants), and spouse");
    println!("   are entitled to compulsory portion (§2303 BGB)");

    // Example 8: Inheritance Contract
    println!("\n📋 Example 8: Inheritance Contract (§2274 BGB)");
    println!("Binding agreement about succession (requires notarization)\n");

    let inheritance_contract = InheritanceContract {
        testator: "Hans Mueller".to_string(),
        beneficiary: "Maria Mueller (spouse)".to_string(),
        contract_date: NaiveDate::from_ymd_opt(2020, 1, 15).unwrap(),
        notarized: true, // REQUIRED per §2276 BGB
        inheritance_share: InheritanceShare::Full,
        is_mutual: true,
        revoked: false,
    };

    match validate_inheritance_contract(&inheritance_contract) {
        Ok(()) => {
            println!("✅ Inheritance Contract: VALID");
            println!("   - Testator: {}", inheritance_contract.testator);
            println!("   - Beneficiary: {}", inheritance_contract.beneficiary);
            println!("   - Notarized: ✓ (mandatory per §2276 BGB)");
            println!("   - Type: Mutual contract");
            println!("\n💡 Key Differences from Will:");
            println!("   - More formal (must be notarized)");
            println!("   - Harder to revoke (contractual nature)");
            println!("   - Often used between spouses");
            println!("   - Binding on both parties");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 9: Certificate of Inheritance
    println!("\n📋 Example 9: Certificate of Inheritance (§2353 BGB)");
    println!("Official document proving heir status\n");

    let certificate = CertificateOfInheritance {
        heir_name: "Maria Mueller".to_string(),
        deceased_name: "Hans Mueller".to_string(),
        inheritance_share: "1/2".to_string(),
        issued_by: "Amtsgericht Berlin (Nachlassgericht)".to_string(),
        issued_at: NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        is_joint_certificate: false,
    };

    println!("Certificate Details:");
    println!("   - Heir: {}", certificate.heir_name);
    println!("   - Deceased: {}", certificate.deceased_name);
    println!("   - Share: {}", certificate.inheritance_share);
    println!("   - Issued by: {}", certificate.issued_by);
    println!("   - Date: {}", certificate.issued_at);
    println!("\n💡 Purpose:");
    println!("   - Proves heir status to third parties");
    println!("   - Required for real estate transfers");
    println!("   - Required for bank account access");
    println!("   - Public faith effect (§2366 BGB)");

    // Summary
    println!("\n=== Summary: German Will Formalities ===");
    println!("\n📝 Will Types (Testamentsformen):");
    println!("   1. Holographic (§2247): Entirely handwritten + signed");
    println!("   2. Public (§2232): Notarized or court-deposited");
    println!("   3. Emergency (§§2249-2251): Exceptional circumstances");
    println!("\n🎂 Testamentary Capacity (§2229):");
    println!("   - Under 16: No capacity");
    println!("   - Age 16-17: Limited capacity (special formalities)");
    println!("   - Age 18+: Full capacity");
    println!("\n⚖️ Compulsory Portion (§2303):");
    println!("   - Entitled: Descendants, parents, spouse");
    println!("   - Amount: 1/2 of legal inheritance share");
    println!("   - Nature: Monetary claim (not asset claim)");
    println!("   - NOT entitled: Siblings, other relatives");
    println!("\n📋 Key Formality Rules:");
    println!("   - Holographic must be ENTIRELY handwritten");
    println!("   - All wills must be signed");
    println!("   - Date recommended but not always required");
    println!("   - Inheritance contracts MUST be notarized");
    println!("   - No witnesses required for holographic wills");
}
