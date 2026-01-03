//! Canadian Healthcare (Canada Health Act)
//!
//! This example demonstrates how to use Legalis-RS for Canadian healthcare
//! eligibility under the Canada Health Act and provincial health plans.
//!
//! ## Canadian Healthcare System
//!
//! Canada has a publicly funded, single-payer healthcare system known as
//! "Medicare." The Canada Health Act sets national standards, while provinces
//! administer their own health insurance plans.
//!
//! ## Key Legislation
//!
//! - Canada Health Act (1984)
//! - Provincial Health Insurance Plans (OHIP, MSP, RAMQ, etc.)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

const CA_HEALTHCARE_STATUTES: &str = r#"
// =============================================================================
// Canada Health Act (1984)
// =============================================================================

// Section 10 - Program Criteria
STATUTE cha-s10-public-admin: "CHA s.10(a) - Public Administration" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS provincial_health_plan
    THEN OBLIGATION "Plan administered by public authority on non-profit basis"
}

STATUTE cha-s10-comprehensiveness: "CHA s.10(b) - Comprehensiveness" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS provincial_health_plan
    THEN OBLIGATION "Cover all insured health services provided by hospitals and physicians"
}

STATUTE cha-s10-universality: "CHA s.10(c) - Universality" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS provincial_health_plan
    THEN OBLIGATION "Cover 100% of insured persons on uniform terms and conditions"
}

STATUTE cha-s10-portability: "CHA s.10(d) - Portability" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS provincial_health_plan AND HAS moving_between_provinces
    THEN GRANT "Coverage during 3-month waiting period by previous province"
}

STATUTE cha-s10-accessibility: "CHA s.10(e) - Accessibility" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS provincial_health_plan
    THEN PROHIBITION "User charges or extra-billing that impedes access"
}

// =============================================================================
// Provincial Health Insurance Eligibility
// =============================================================================

// Ontario Health Insurance Plan (OHIP)
STATUTE ca-ohip-eligibility: "OHIP - Ontario Health Insurance" {
    JURISDICTION "CA-ON"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS ontario_resident AND HAS primary_residence_ontario AND
         HAS present_153_days_year AND
         (HAS canadian_citizen OR HAS permanent_resident OR HAS valid_work_permit)
    THEN GRANT "OHIP coverage for insured health services"
}

STATUTE ca-ohip-newborn: "OHIP - Newborn Coverage" {
    JURISDICTION "CA-ON"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS newborn_child AND HAS parent_ohip_eligible
    THEN GRANT "Immediate OHIP coverage from birth"
}

// British Columbia Medical Services Plan (MSP)
STATUTE ca-msp-eligibility: "MSP - BC Medical Services Plan" {
    JURISDICTION "CA-BC"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS bc_resident AND
         (HAS canadian_citizen OR HAS permanent_resident OR HAS valid_work_permit_6_months)
    THEN GRANT "MSP coverage (no premiums since 2020)"
}

// Quebec RAMQ
STATUTE ca-ramq-eligibility: "RAMQ - Quebec Health Insurance" {
    JURISDICTION "CA-QC"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS quebec_resident AND
         (HAS canadian_citizen OR HAS permanent_resident OR HAS valid_work_permit)
    THEN GRANT "RAMQ coverage for insured health services"
}

// Alberta Health Care Insurance Plan (AHCIP)
STATUTE ca-ahcip-eligibility: "AHCIP - Alberta Health Care" {
    JURISDICTION "CA-AB"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS alberta_resident AND
         (HAS canadian_citizen OR HAS permanent_resident OR HAS valid_work_permit)
    THEN GRANT "Alberta health care coverage"
}

// =============================================================================
// Waiting Period Rules
// =============================================================================

STATUTE ca-waiting-period: "Provincial Health - 3-Month Waiting Period" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS new_provincial_resident AND NOT HAS completed_waiting_period
    THEN OBLIGATION "Maximum 3-month waiting period for new residents"

    EXCEPTION WHEN HAS newborn_child
    EXCEPTION WHEN HAS returning_canadian
}

// =============================================================================
// Excluded Services
// =============================================================================

STATUTE ca-excluded-dental: "CHA - Dental Services Exclusion" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS dental_service AND NOT HAS hospital_based_dental
    THEN PROHIBITION "Not covered under public health insurance"
}

STATUTE ca-excluded-vision: "CHA - Vision Care Exclusion" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS routine_vision_care AND AGE >= 18 AND AGE <= 64
    THEN PROHIBITION "Routine eye exams not covered for adults 18-64"
}

STATUTE ca-excluded-prescription: "CHA - Prescription Drugs Exclusion" {
    JURISDICTION "CA"
    VERSION 1
    EFFECTIVE_DATE 1984-04-01

    WHEN HAS outpatient_prescription AND NOT HAS provincial_pharmacare_eligible
    THEN PROHIBITION "Outpatient prescriptions not universally covered"
}

// =============================================================================
// Special Coverage
// =============================================================================

STATUTE ca-seniors-pharmacare: "Provincial Pharmacare - Seniors" {
    JURISDICTION "CA"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 65 AND HAS provincial_resident
    THEN GRANT "Enhanced pharmacare coverage for seniors (varies by province)"
}

STATUTE ca-indigenous-nihb: "NIHB - Non-Insured Health Benefits" {
    JURISDICTION "CA"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS first_nations_or_inuit_status
    THEN GRANT "NIHB coverage: dental, vision, mental health, medical transportation, prescriptions"
}

STATUTE ca-refugee-ifhp: "IFHP - Interim Federal Health Program" {
    JURISDICTION "CA"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS refugee_claimant OR HAS protected_person
    THEN GRANT "IFHP coverage during refugee process"
}
"#;

fn create_resident(name: &str, age: u32, attrs: &[(&str, bool)]) -> BasicEntity {
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
    println!("   CANADIAN HEALTHCARE - Legalis-RS Demo");
    println!("   Canada Health Act | Provincial Health Plans");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(CA_HEALTHCARE_STATUTES)?;
    println!("Step 1: Parsed {} healthcare provisions\n", statutes.len());

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Verification {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Evaluating healthcare eligibility...\n");

    let residents = vec![
        (
            "Ontario Citizen",
            45u32,
            vec![
                ("ontario_resident", true),
                ("primary_residence_ontario", true),
                ("present_153_days_year", true),
                ("canadian_citizen", true),
                ("provincial_resident", true),
            ],
        ),
        (
            "BC Permanent Resident",
            35,
            vec![
                ("bc_resident", true),
                ("permanent_resident", true),
                ("provincial_resident", true),
            ],
        ),
        (
            "Quebec Senior",
            72,
            vec![
                ("quebec_resident", true),
                ("canadian_citizen", true),
                ("provincial_resident", true),
            ],
        ),
        (
            "New Immigrant (Waiting Period)",
            28,
            vec![
                ("ontario_resident", true),
                ("permanent_resident", true),
                ("new_provincial_resident", true),
            ],
        ),
        (
            "First Nations Member",
            50,
            vec![
                ("first_nations_or_inuit_status", true),
                ("canadian_citizen", true),
                ("provincial_resident", true),
            ],
        ),
        ("Refugee Claimant", 32, vec![("refugee_claimant", true)]),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, age, attrs) in &residents {
        let entity = create_resident(name, *age, attrs);
        println!("   === {} (Age: {}) ===", name, age);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                let effect_type = format!("{:?}", statute.effect.effect_type);
                if !effect_type.contains("Prohibition") {
                    println!("   [+] {} - COVERED", statute.id);
                }

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "ca-healthcare".to_string(),
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
        println!();
    }

    println!("Step 4: Running population simulation...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;
    println!(
        "   Total: {} | Deterministic: {} | Discretion: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   CANADIAN HEALTHCARE DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!("\n   Five Principles of Medicare (CHA):");
    println!("   1. Public Administration");
    println!("   2. Comprehensiveness");
    println!("   3. Universality");
    println!("   4. Portability");
    println!("   5. Accessibility");
    println!("\n   Provincial Plans: OHIP, MSP, RAMQ, AHCIP, etc.\n");

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
            .map(|i| match operator {
                ComparisonOp::GreaterOrEqual => i >= *value,
                ComparisonOp::GreaterThan => i > *value,
                ComparisonOp::LessOrEqual => i <= *value,
                ComparisonOp::LessThan => i < *value,
                ComparisonOp::Equal => i == *value,
                ComparisonOp::NotEqual => i != *value,
            })
            .unwrap_or(false),
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => evaluate_condition(entity, l) && evaluate_condition(entity, r),
        Condition::Or(l, r) => evaluate_condition(entity, l) || evaluate_condition(entity, r),
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true,
    }
}
