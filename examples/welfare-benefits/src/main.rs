//! Welfare Benefits Eligibility System
//!
//! This example demonstrates how to use Legalis-RS to build a comprehensive
//! welfare benefits eligibility determination system. It showcases:
//!
//! - Parsing statutes from DSL
//! - Evaluating eligibility for multiple benefit programs
//! - Running population simulations
//! - Generating visualizations
//! - Maintaining audit trails
//! - Verifying statute consistency

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;
use legalis_viz::DecisionTree;

/// The welfare benefits statutes in DSL format.
const WELFARE_STATUTES: &str = r#"
// Basic Welfare Assistance Program
// Provides fundamental support for low-income individuals

STATUTE basic-welfare: "Basic Welfare Assistance" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN INCOME <= 30000
    THEN GRANT "Monthly welfare payment of $500"

    DISCRETION "Case workers may adjust based on local cost of living"
}

// Senior Citizens Pension Supplement
// Additional support for elderly citizens

STATUTE senior-pension: "Senior Citizens Pension Supplement" {
    JURISDICTION "US"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 65 AND INCOME <= 50000
    THEN GRANT "Monthly pension supplement of $300"
}

// Child Support Benefit
// Support for families with dependent children

STATUTE child-support: "Child Support Benefit" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS dependent-children AND INCOME <= 60000
    THEN GRANT "Per-child monthly benefit of $200"

    DISCRETION "Additional support available for special needs children"
}

// Disability Assistance
// Support for individuals with disabilities

STATUTE disability-assist: "Disability Assistance Program" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS disability AND INCOME <= 45000
    THEN GRANT "Monthly disability benefit of $600"
}

// Emergency Housing Assistance
// Temporary housing support for those at risk

STATUTE housing-assist: "Emergency Housing Assistance" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS housing-insecure AND INCOME <= 35000
    THEN GRANT "Emergency housing voucher"

    DISCRETION "Priority given to families with children"
}

// Healthcare Subsidy
// Support for healthcare costs

STATUTE healthcare-subsidy: "Healthcare Cost Subsidy" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN INCOME <= 40000 OR (AGE >= 65 AND INCOME <= 60000)
    THEN GRANT "Healthcare premium subsidy"
}

// Unemployment Bridge Benefit
// Temporary support during job search

STATUTE unemployment-bridge: "Unemployment Bridge Benefit" {
    JURISDICTION "US"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS unemployed AND NOT HAS receiving-unemployment-insurance
    THEN GRANT "Bridge benefit of $400 per month"
}
"#;

/// Creates a test citizen with attributes.
fn create_citizen(name: &str, age: u32, income: u64, attributes: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("name", name.to_string());
    entity.set_attribute("age", age.to_string());
    entity.set_attribute("income", income.to_string());

    for (key, value) in attributes {
        if *value {
            entity.set_attribute(key, "true".to_string());
        }
    }

    entity
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     WELFARE BENEFITS ELIGIBILITY SYSTEM - Legalis-RS Demo    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Step 1: Parse the welfare statutes
    println!("Step 1: Parsing welfare statutes from DSL...\n");
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(WELFARE_STATUTES)?;
    println!("   Parsed {} statutes:", statutes.len());
    for statute in &statutes {
        println!("   - {} ({})", statute.id, statute.title);
    }
    println!();

    // Step 2: Verify statutes for consistency
    println!("Step 2: Verifying statute consistency...\n");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    if result.passed {
        println!("   [OK] All statutes passed verification");
    } else {
        println!("   [FAIL] Verification failed:");
        for error in &result.errors {
            println!("     - {:?}", error);
        }
    }
    for warning in &result.warnings {
        println!("   [WARN] {}", warning);
    }
    println!();

    // Step 3: Create test citizens
    println!("Step 3: Creating test citizens for eligibility evaluation...\n");
    let test_citizens = vec![
        (
            "Alice Johnson",
            72,
            35000u64,
            vec![("dependent-children", false)],
        ),
        (
            "Bob Smith",
            35,
            25000,
            vec![("dependent-children", true), ("disability", false)],
        ),
        (
            "Carol Williams",
            28,
            22000,
            vec![("disability", true), ("housing-insecure", true)],
        ),
        ("David Brown", 45, 55000, vec![("dependent-children", true)]),
        (
            "Eva Martinez",
            68,
            48000,
            vec![
                ("unemployed", true),
                ("receiving-unemployment-insurance", false),
            ],
        ),
    ];

    for (name, age, income, _) in &test_citizens {
        println!("   {} : Age {}, Income ${}", name, age, income);
    }
    println!();

    // Step 4: Evaluate eligibility and record in audit trail
    println!("Step 4: Evaluating eligibility for each citizen...\n");
    let mut audit_trail = AuditTrail::new();

    for (name, age, income, attrs) in &test_citizens {
        let citizen = create_citizen(name, *age, *income, attrs);
        let citizen_id = citizen.id();

        println!("   === {} ===", name);

        for statute in &statutes {
            let eligible = check_eligibility(&citizen, statute);

            if eligible {
                println!("   [+] {} - ELIGIBLE", statute.id);

                // Record in audit trail
                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "welfare-system".to_string(),
                    },
                    statute.id.clone(),
                    citizen_id,
                    DecisionContext::default(),
                    DecisionResult::Deterministic {
                        effect_applied: statute.effect.description.clone(),
                        parameters: HashMap::new(),
                    },
                    None, // Previous hash will be set by audit trail
                );
                let _ = audit_trail.record(record);
            } else {
                println!("   [-] {} - Not eligible", statute.id);
            }
        }
        println!();
    }

    // Step 5: Generate visualizations
    println!("Step 5: Generating decision tree visualization...\n");
    if let Some(first_statute) = statutes.first() {
        match DecisionTree::from_statute(first_statute) {
            Ok(tree) => {
                let ascii = tree.to_ascii();
                println!("{}", ascii);
            }
            Err(e) => println!("   Warning: Could not generate tree: {:?}", e),
        }
    }
    println!();

    // Step 6: Run population simulation
    println!("Step 6: Running population simulation (500 citizens)...\n");

    // Generate random population
    let population = PopulationBuilder::new().generate_random(500).build();

    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;

    println!("   Simulation Results:");
    println!("   -------------------------------------------");
    println!("   Total applications: {}", metrics.total_applications);
    println!("   Deterministic outcomes: {}", metrics.deterministic_count);
    println!("   Discretionary outcomes: {}", metrics.discretion_count);
    println!();

    // Step 7: Audit trail integrity
    println!("Step 7: Verifying audit trail integrity...\n");
    match audit_trail.verify_integrity() {
        Ok(true) => {
            println!("   [OK] Audit trail integrity verified");
            println!("   Total records: {}", audit_trail.count());
        }
        Ok(false) => {
            println!("   [FAIL] Audit trail integrity check returned false");
        }
        Err(e) => {
            println!("   [FAIL] Audit trail integrity check failed: {:?}", e);
        }
    }
    println!();

    // Summary
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                      DEMO COMPLETE                           ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  This example demonstrated:                                  ║");
    println!("║  - DSL parsing for legal statutes                            ║");
    println!("║  - Statute verification                                      ║");
    println!("║  - Individual eligibility evaluation                         ║");
    println!("║  - Decision tree visualization                               ║");
    println!("║  - Population simulation                                     ║");
    println!("║  - Audit trail with integrity verification                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}

/// Checks if an entity is eligible for a statute by evaluating conditions.
fn check_eligibility(entity: &BasicEntity, statute: &Statute) -> bool {
    if statute.preconditions.is_empty() {
        return true;
    }

    for condition in &statute.preconditions {
        if !evaluate_condition(entity, condition) {
            return false;
        }
    }
    true
}

/// Evaluates a single condition against an entity.
fn evaluate_condition(entity: &BasicEntity, condition: &Condition) -> bool {
    match condition {
        Condition::Age { operator, value } => {
            if let Some(age_str) = entity.get_attribute("age") {
                if let Ok(age) = age_str.parse::<u32>() {
                    return match operator {
                        ComparisonOp::GreaterOrEqual => age >= *value,
                        ComparisonOp::GreaterThan => age > *value,
                        ComparisonOp::LessOrEqual => age <= *value,
                        ComparisonOp::LessThan => age < *value,
                        ComparisonOp::Equal => age == *value,
                        ComparisonOp::NotEqual => age != *value,
                    };
                }
            }
            false
        }
        Condition::Income { operator, value } => {
            if let Some(income_str) = entity.get_attribute("income") {
                if let Ok(income) = income_str.parse::<u64>() {
                    return match operator {
                        ComparisonOp::GreaterOrEqual => income >= *value,
                        ComparisonOp::GreaterThan => income > *value,
                        ComparisonOp::LessOrEqual => income <= *value,
                        ComparisonOp::LessThan => income < *value,
                        ComparisonOp::Equal => income == *value,
                        ComparisonOp::NotEqual => income != *value,
                    };
                }
            }
            false
        }
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(left, right) => {
            evaluate_condition(entity, left) && evaluate_condition(entity, right)
        }
        Condition::Or(left, right) => {
            evaluate_condition(entity, left) || evaluate_condition(entity, right)
        }
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true, // Default to true for other conditions
    }
}
