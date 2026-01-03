//! Australian Immigration (Migration Act 1958)
//!
//! This example demonstrates how to use Legalis-RS for Australian visa
//! eligibility assessment under the Migration Act 1958 and Regulations.
//!
//! ## Australian Immigration System
//!
//! Australia's points-based immigration system is one of the most
//! sophisticated in the world, with over 100 visa subclasses.
//!
//! ## Key Visa Categories Modeled
//!
//! - Skilled Migration (Subclass 189, 190, 491)
//! - Student Visa (Subclass 500)
//! - Working Holiday (Subclass 417, 462)
//! - Partner Visa (Subclass 820/801)
//! - Employer Sponsored (Subclass 482, 494)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

const AU_IMMIGRATION_STATUTES: &str = r#"
// =============================================================================
// Migration Act 1958 - Australian Immigration Law
// =============================================================================

// Skilled Independent Visa (Subclass 189)
STATUTE au-visa-189: "Skilled Independent Visa (Subclass 189)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN AGE >= 18 AND AGE <= 44 AND
         HAS skilled_occupation AND
         HAS competent_english AND
         HAS skills_assessment AND
         HAS points_65_or_more AND
         HAS health_requirement AND
         HAS character_requirement
    THEN GRANT "Permanent residency - Skilled Independent pathway"

    DISCRETION "Points test cut-off varies by occupation and invitation round"
}

// Skilled Nominated Visa (Subclass 190)
STATUTE au-visa-190: "Skilled Nominated Visa (Subclass 190)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN AGE >= 18 AND AGE <= 44 AND
         HAS skilled_occupation AND
         HAS state_nomination AND
         HAS competent_english AND
         HAS skills_assessment AND
         HAS points_65_or_more
    THEN GRANT "Permanent residency - State Nominated pathway"
}

// Skilled Work Regional (Subclass 491)
STATUTE au-visa-491: "Skilled Work Regional (Subclass 491)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN AGE >= 18 AND AGE <= 44 AND
         HAS skilled_occupation AND
         (HAS state_nomination OR HAS family_sponsor_regional) AND
         HAS competent_english AND
         HAS skills_assessment AND
         HAS points_65_or_more
    THEN GRANT "5-year provisional visa with PR pathway after 3 years in regional area"
}

// Student Visa (Subclass 500)
STATUTE au-visa-500: "Student Visa (Subclass 500)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS enrolled_cricos_course AND
         HAS genuine_temporary_entrant AND
         HAS financial_capacity AND
         HAS health_insurance AND
         HAS english_requirement_met
    THEN GRANT "Student visa for duration of course"

    DISCRETION "Genuine Temporary Entrant (GTE) assessment is discretionary"
}

// Working Holiday Visa (Subclass 417)
STATUTE au-visa-417: "Working Holiday Visa (Subclass 417)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN AGE >= 18 AND AGE <= 30 AND
         HAS eligible_passport_417 AND
         HAS sufficient_funds AND
         NOT HAS dependent_children AND
         HAS health_requirement
    THEN GRANT "12-month working holiday visa (2nd/3rd year possible with regional work)"
}

// Partner Visa (Subclass 820/801)
STATUTE au-visa-820-801: "Partner Visa (Subclass 820/801)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS australian_partner AND
         HAS genuine_relationship AND
         HAS health_requirement AND
         HAS character_requirement
    THEN GRANT "Temporary (820) then Permanent (801) partner visa"

    DISCRETION "Genuineness of relationship assessed holistically"
}

// Temporary Skill Shortage (Subclass 482)
STATUTE au-visa-482: "Temporary Skill Shortage Visa (Subclass 482)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS employer_nomination AND
         HAS skilled_occupation AND
         HAS labour_market_testing AND
         HAS english_requirement_met AND
         HAS relevant_experience
    THEN GRANT "2-4 year employer sponsored visa with PR pathway"
}

// Employer Nomination Scheme (Subclass 186)
STATUTE au-visa-186: "Employer Nomination Scheme (Subclass 186)" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN AGE <= 44 AND
         HAS employer_nomination AND
         HAS skilled_occupation AND
         HAS skills_assessment AND
         HAS competent_english AND
         HAS 3_years_experience
    THEN GRANT "Permanent residency - Direct Entry stream"
}

// =============================================================================
// Character and Health Requirements
// =============================================================================

STATUTE au-character-s501: "Migration Act s.501 - Character Requirement" {
    JURISDICTION "AU"
    VERSION 1
    EFFECTIVE_DATE 1958-10-01

    WHEN HAS substantial_criminal_record OR HAS security_risk
    THEN PROHIBITION "Visa refused/cancelled on character grounds"

    DISCRETION "Minister has personal discretion in s.501 decisions"
}

STATUTE au-health-pia: "Public Interest Criterion 4005/4007 - Health" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS significant_cost_health_condition AND NOT HAS health_waiver
    THEN PROHIBITION "Visa refused on health grounds (significant cost threshold)"

    EXCEPTION WHEN HAS compelling_circumstances
    DISCRETION "Health waiver available for permanent visas in some cases"
}

// =============================================================================
// Points Test (General Skilled Migration)
// =============================================================================

STATUTE au-points-age: "Points Test - Age" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN AGE >= 25 AND AGE <= 32
    THEN GRANT "30 points for age 25-32"
}

STATUTE au-points-english-superior: "Points Test - Superior English" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS superior_english
    THEN GRANT "20 points for superior English (IELTS 8+)"
}

STATUTE au-points-experience-overseas: "Points Test - Overseas Experience" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS overseas_experience_8_years
    THEN GRANT "15 points for 8+ years overseas skilled employment"
}

STATUTE au-points-australian-study: "Points Test - Australian Study" {
    JURISDICTION "AU"
    VERSION 2024
    EFFECTIVE_DATE 2024-07-01

    WHEN HAS australian_study_requirement
    THEN GRANT "5 points for 2+ years study in Australia"
}
"#;

fn create_applicant(name: &str, age: u32, attrs: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("name", name.to_string());
    entity.set_attribute("age", age.to_string());
    for (key, value) in attrs {
        if *value {
            entity.set_attribute(key, "true".to_string());
        }
    }
    entity
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   AUSTRALIAN IMMIGRATION LAW - Legalis-RS Demo");
    println!("   Migration Act 1958 | Visa Eligibility Assessment");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(AU_IMMIGRATION_STATUTES)?;
    println!("Step 1: Parsed {} visa provisions\n", statutes.len());

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Verification {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Evaluating visa applicants...\n");

    let applicants = vec![
        (
            "Skilled Worker (Age 30)",
            30u32,
            vec![
                ("skilled_occupation", true),
                ("competent_english", true),
                ("skills_assessment", true),
                ("points_65_or_more", true),
                ("health_requirement", true),
                ("character_requirement", true),
            ],
        ),
        (
            "International Student",
            22,
            vec![
                ("enrolled_cricos_course", true),
                ("genuine_temporary_entrant", true),
                ("financial_capacity", true),
                ("health_insurance", true),
                ("english_requirement_met", true),
            ],
        ),
        (
            "Working Holiday Maker",
            25,
            vec![
                ("eligible_passport_417", true),
                ("sufficient_funds", true),
                ("health_requirement", true),
            ],
        ),
        (
            "Partner Visa Applicant",
            35,
            vec![
                ("australian_partner", true),
                ("genuine_relationship", true),
                ("health_requirement", true),
                ("character_requirement", true),
            ],
        ),
        (
            "Over-Age Skilled Worker",
            48,
            vec![
                ("skilled_occupation", true),
                ("competent_english", true),
                ("points_65_or_more", true),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, age, attrs) in &applicants {
        let entity = create_applicant(name, *age, attrs);
        println!("   === {} (Age: {}) ===", name, age);

        let mut eligible_visas = Vec::new();
        for statute in &statutes {
            if check_applicability(&entity, statute) && statute.id.starts_with("au-visa-") {
                eligible_visas.push(statute.id.clone());
                println!("   [+] {} - ELIGIBLE", statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "au-immigration".to_string(),
                    },
                    statute.id.clone(),
                    entity.id(),
                    DecisionContext::default(),
                    DecisionResult::Deterministic {
                        effect_applied: statute.effect.description.clone(),
                        parameters: HashMap::new(),
                    },
                    None,
                );
                let _ = audit_trail.record(record);
            }
        }
        if eligible_visas.is_empty() {
            println!("   [!] No visa eligibility found");
        }
        println!();
    }

    println!("Step 4: Running population simulation...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;
    println!(
        "   Total: {} | Deterministic: {} | Case Officer discretion: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   AUSTRALIAN IMMIGRATION DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!("\n   Key Immigration Pathways:");
    println!("   - Skilled Migration (189/190/491) - Points-tested");
    println!("   - Employer Sponsored (482/186/494)");
    println!("   - Family (Partner 820/801, Parent, Child)");
    println!("   - Student (500) and Working Holiday (417/462)");
    println!("\n   Decision Maker: Department of Home Affairs\n");

    Ok(())
}

fn check_applicability(entity: &BasicEntity, statute: &Statute) -> bool {
    statute
        .preconditions
        .iter()
        .all(|c| evaluate_condition(entity, c))
}

fn evaluate_condition(entity: &BasicEntity, condition: &Condition) -> bool {
    match condition {
        Condition::Age { operator, value } => entity
            .get_attribute("age")
            .and_then(|s| s.parse::<u32>().ok())
            .map(|age| match operator {
                ComparisonOp::GreaterOrEqual => age >= *value,
                ComparisonOp::GreaterThan => age > *value,
                ComparisonOp::LessOrEqual => age <= *value,
                ComparisonOp::LessThan => age < *value,
                ComparisonOp::Equal => age == *value,
                ComparisonOp::NotEqual => age != *value,
            })
            .unwrap_or(false),
        Condition::Income { operator, value } => entity
            .get_attribute("income")
            .and_then(|s| s.parse::<u64>().ok())
            .map(|income| match operator {
                ComparisonOp::GreaterOrEqual => income >= *value,
                ComparisonOp::GreaterThan => income > *value,
                ComparisonOp::LessOrEqual => income <= *value,
                ComparisonOp::LessThan => income < *value,
                ComparisonOp::Equal => income == *value,
                ComparisonOp::NotEqual => income != *value,
            })
            .unwrap_or(false),
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => evaluate_condition(entity, l) && evaluate_condition(entity, r),
        Condition::Or(l, r) => evaluate_condition(entity, l) || evaluate_condition(entity, r),
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true,
    }
}
