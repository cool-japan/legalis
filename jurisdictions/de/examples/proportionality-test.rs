//! Proportionality Test Example (Verhältnismäßigkeitsprüfung)
//!
//! Demonstrates the three-step proportionality test for basic rights restrictions:
//! 1. Suitability (Geeignetheit)
//! 2. Necessity (Erforderlichkeit)
//! 3. Proportionality stricto sensu (Angemessenheit)

use chrono::NaiveDate;
use legalis_de::grundgesetz::*;

fn main() {
    println!("=== Proportionality Test Examples (Verhältnismäßigkeitsprüfung) ===\n");
    println!("Three-Step Test for Justifying Basic Rights Restrictions\n");

    let authority = PublicAuthority {
        name: "Bundestag".to_string(),
        authority_type: AuthorityType::Legislative,
        level: FederalLevel::Federal,
    };

    // Example 1: Valid - All three prongs satisfied
    println!("📋 Example 1: Public Assembly Permit Requirement");
    println!("Art. 8 GG restriction - PASSES proportionality test\n");

    let valid_test = ProportionalityTest {
        restriction: RightsRestriction {
            restricting_authority: authority.clone(),
            legal_basis: "Assembly Act §14 - Permit requirement".to_string(),
            restriction_type: RestrictionType::PermitRequirement,
            date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            justification: "Protection of public order and safety".to_string(),
        },
        legitimate_purpose: "Prevent violence and coordinate with traffic".to_string(),
        suitable: SuitabilityAssessment {
            is_suitable: true,
            reasoning: "Advance notice allows police planning and route coordination".to_string(),
        },
        necessary: NecessityAssessment {
            is_necessary: true,
            alternative_measures: vec![],
            reasoning: "No less restrictive means to achieve same level of public safety"
                .to_string(),
        },
        proportionate_stricto_sensu: ProportionalityStrictoSensu {
            is_proportionate: true,
            public_interest: "Public order, safety of participants and bystanders".to_string(),
            private_interest: "Freedom of assembly".to_string(),
            balancing:
                "Minor administrative burden outweighed by significant public safety benefits"
                    .to_string(),
        },
    };

    match validate_proportionality_test(&valid_test) {
        Ok(()) => {
            println!("✅ Proportionality Test: PASSED");
            println!("\n   Step 1: Suitability (Geeignetheit) ✓");
            println!("   - Question: Can measure achieve the purpose?");
            println!("   - Result: {}", valid_test.suitable.reasoning);
            println!("\n   Step 2: Necessity (Erforderlichkeit) ✓");
            println!("   - Question: Is there a less restrictive alternative?");
            println!("   - Result: {}", valid_test.necessary.reasoning);
            println!("\n   Step 3: Proportionality Stricto Sensu (Angemessenheit) ✓");
            println!("   - Question: Does public benefit outweigh private burden?");
            println!(
                "   - Public interest: {}",
                valid_test.proportionate_stricto_sensu.public_interest
            );
            println!(
                "   - Private interest: {}",
                valid_test.proportionate_stricto_sensu.private_interest
            );
            println!(
                "   - Balancing: {}",
                valid_test.proportionate_stricto_sensu.balancing
            );
            println!("\n   ✅ CONCLUSION: Restriction is constitutional");
        }
        Err(e) => println!("❌ Test Failed: {}", e),
    }

    // Example 2: Fails - Not Suitable
    println!("\n📋 Example 2: Total Ban on Political Demonstrations");
    println!("FAILS suitability - Measure doesn't achieve legitimate purpose\n");

    let unsuitable_test = ProportionalityTest {
        restriction: RightsRestriction {
            restricting_authority: authority.clone(),
            legal_basis: "Hypothetical Law - Total demonstration ban".to_string(),
            restriction_type: RestrictionType::Prohibition,
            date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            justification: "Prevent some instances of violence".to_string(),
        },
        legitimate_purpose: "Reduce isolated incidents of violence".to_string(),
        suitable: SuitabilityAssessment {
            is_suitable: false,
            reasoning: "Total ban eliminates peaceful assemblies too - not suitable".to_string(),
        },
        necessary: NecessityAssessment {
            is_necessary: false,
            alternative_measures: vec![
                "Targeted police presence".to_string(),
                "Permit with conditions".to_string(),
            ],
            reasoning: "Many less restrictive alternatives available".to_string(),
        },
        proportionate_stricto_sensu: ProportionalityStrictoSensu {
            is_proportionate: false,
            public_interest: "Prevent isolated violence".to_string(),
            private_interest: "Core democratic right to assemble".to_string(),
            balancing: "Total ban grossly disproportionate to minimal risk".to_string(),
        },
    };

    match validate_proportionality_test(&unsuitable_test) {
        Ok(()) => println!("✅ Test Passed (unexpected)"),
        Err(e) => {
            println!("❌ Proportionality Test: FAILED");
            println!("   Error: {}", e);
            println!("\n   ❌ Step 1: Suitability - FAILED");
            println!("   - {}", unsuitable_test.suitable.reasoning);
            println!("\n   💡 When first step fails, measure is unconstitutional");
            println!("   - No need to check necessity or proportionality stricto sensu");
        }
    }

    // Example 3: Passes Suitability, Fails Necessity
    println!("\n📋 Example 3: Complete Social Media Ban for Minors");
    println!("PASSES suitability but FAILS necessity\n");

    let unnecessary_test = ProportionalityTest {
        restriction: RightsRestriction {
            restricting_authority: authority.clone(),
            legal_basis: "Hypothetical Youth Protection Act".to_string(),
            restriction_type: RestrictionType::Prohibition,
            date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            justification: "Protect minors from online dangers".to_string(),
        },
        legitimate_purpose: "Protect minors from cyberbullying and harmful content".to_string(),
        suitable: SuitabilityAssessment {
            is_suitable: true,
            reasoning: "Complete ban would eliminate online risks for minors".to_string(),
        },
        necessary: NecessityAssessment {
            is_necessary: false,
            alternative_measures: vec![
                "Parental controls".to_string(),
                "Age verification".to_string(),
                "Content filtering".to_string(),
                "Supervised accounts".to_string(),
            ],
            reasoning: "Many less restrictive alternatives achieve similar protection".to_string(),
        },
        proportionate_stricto_sensu: ProportionalityStrictoSensu {
            is_proportionate: false,
            public_interest: "Youth protection".to_string(),
            private_interest: "Freedom of expression and information".to_string(),
            balancing: "Complete ban excessive when targeted measures suffice".to_string(),
        },
    };

    match validate_proportionality_test(&unnecessary_test) {
        Ok(()) => println!("✅ Test Passed (unexpected)"),
        Err(e) => {
            println!("❌ Proportionality Test: FAILED");
            println!("   Error: {}", e);
            println!("\n   ✓ Step 1: Suitability - PASSED");
            println!("   ❌ Step 2: Necessity - FAILED");
            println!("   - Less restrictive alternatives available:");
            for alt in &unnecessary_test.necessary.alternative_measures {
                println!("     • {}", alt);
            }
            println!("\n   💡 Necessity requires choosing LEAST restrictive means");
            println!("   - If milder alternative achieves same goal, measure fails");
        }
    }

    // Summary
    println!("\n=== Summary: Proportionality Test (Verhältnismäßigkeitsprüfung) ===");
    println!("\n📊 Three-Step Framework:");
    println!("\n1️⃣  SUITABILITY (Geeignetheit)");
    println!("   - Can the measure achieve the legitimate purpose?");
    println!("   - Standard: Measure must promote the goal (not perfect achievement)");
    println!("   - If NO → Measure unconstitutional");
    println!("\n2️⃣  NECESSITY (Erforderlichkeit)");
    println!("   - Is there a less restrictive alternative?");
    println!("   - Standard: Measure must be LEAST restrictive means");
    println!("   - Alternative must be:");
    println!("     • Equally effective");
    println!("     • Less burdensome on rights");
    println!("   - If less restrictive alternative exists → Measure unconstitutional");
    println!("\n3️⃣  PROPORTIONALITY STRICTO SENSU (Angemessenheit)");
    println!("   - Does public benefit outweigh private burden?");
    println!("   - Standard: Balancing of interests (Abwägung)");
    println!("   - Factors:");
    println!("     • Importance of public interest");
    println!("     • Severity of rights restriction");
    println!("     • Availability of compensation");
    println!("   - If burden > benefit → Measure unconstitutional");
    println!("\n✅ ALL three steps must be satisfied");
    println!("❌ Failure of ANY step = Measure unconstitutional");
    println!("\n💡 Applies to ALL basic rights restrictions in Germany");
}
