//! UK Employment Law Rights and Obligations
//!
//! This example demonstrates how to use Legalis-RS for UK employment law
//! compliance checking. It covers:
//!
//! - Employment Rights Act 1996
//! - Working Time Regulations 1998
//! - National Minimum Wage Act 1998
//! - Equality Act 2010
//! - Statutory Sick Pay (SSP)
//! - Statutory Maternity/Paternity Leave
//!
//! ## UK Employment Law Context
//!
//! The UK has a comprehensive statutory framework for employment rights,
//! supplemented by common law principles. Post-Brexit, UK retains most
//! EU-derived employment protections but can now diverge.
//!
//! ## Key Legislation Modeled
//!
//! - ERA 1996 s.94: Unfair dismissal protection (2 years' service)
//! - ERA 1996 s.86: Statutory notice periods
//! - WTR 1998: 48-hour week limit, 28 days paid leave
//! - NMWA 1998: Minimum wage by age band
//! - EA 2010: Protected characteristics

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;
use legalis_viz::DecisionTree;

/// UK Employment Law statutes in DSL format
const UK_EMPLOYMENT_STATUTES: &str = r#"
// =============================================================================
// Employment Rights Act 1996 (ERA 1996)
// =============================================================================

// Section 94: Unfair Dismissal Protection
STATUTE era1996-s94-unfair-dismissal: "ERA 1996 s.94 - Unfair Dismissal Protection" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 1996-08-22

    WHEN AGE >= 16 AND HAS employee_status AND HAS two_years_continuous_service
    THEN GRANT "Right not to be unfairly dismissed"

    EXCEPTION WHEN HAS gross_misconduct
    DISCRETION "Reasonableness of dismissal assessed by Employment Tribunal"
}

// Section 86: Statutory Notice Periods
STATUTE era1996-s86-notice-1month: "ERA 1996 s.86 - Notice (1 month to 2 years)" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 1996-08-22

    WHEN HAS employee_status AND HAS one_month_service AND NOT HAS two_years_service
    THEN GRANT "Minimum 1 week statutory notice"
}

STATUTE era1996-s86-notice-2years: "ERA 1996 s.86 - Notice (2+ years)" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 1996-08-22

    WHEN HAS employee_status AND HAS two_years_service
    THEN GRANT "1 week notice per year of service (max 12 weeks)"
}

// Section 1: Written Statement of Particulars
STATUTE era1996-s1-written-statement: "ERA 1996 s.1 - Written Statement" {
    JURISDICTION "GB"
    VERSION 2
    EFFECTIVE_DATE 2020-04-06

    WHEN HAS employee_status OR HAS worker_status
    THEN OBLIGATION "Provide written statement of particulars on day 1"
}

// =============================================================================
// Working Time Regulations 1998 (WTR 1998)
// =============================================================================

// Maximum Weekly Working Time
STATUTE wtr1998-weekly-limit: "WTR 1998 - 48 Hour Week Limit" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 1998-10-01

    WHEN HAS worker_status AND NOT HAS opted_out_of_48_hours
    THEN PROHIBITION "Working more than 48 hours per week (17-week average)"

    EXCEPTION WHEN HAS autonomous_decision_making
    DISCRETION "Some sectors have specific exemptions (transport, offshore)"
}

// Statutory Annual Leave
STATUTE wtr1998-annual-leave: "WTR 1998 - Statutory Annual Leave" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 1998-10-01

    WHEN HAS worker_status
    THEN GRANT "5.6 weeks (28 days) paid annual leave pro rata"
}

// Rest Breaks
STATUTE wtr1998-rest-breaks: "WTR 1998 - Rest Breaks" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 1998-10-01

    WHEN HAS worker_status AND HAS shift_over_6_hours
    THEN GRANT "20 minute uninterrupted rest break"
}

// =============================================================================
// National Minimum Wage Act 1998 (NMWA 1998)
// =============================================================================

// National Living Wage (23+)
STATUTE nmwa1998-nlw-23plus: "NMWA 1998 - National Living Wage (23+)" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-01

    WHEN AGE >= 23 AND HAS worker_status
    THEN GRANT "National Living Wage: GBP 11.44 per hour"
}

// 21-22 Rate
STATUTE nmwa1998-21-22: "NMWA 1998 - Age 21-22 Rate" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-01

    WHEN AGE >= 21 AND AGE <= 22 AND HAS worker_status
    THEN GRANT "Minimum wage: GBP 11.44 per hour"
}

// 18-20 Rate
STATUTE nmwa1998-18-20: "NMWA 1998 - Age 18-20 Rate" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-01

    WHEN AGE >= 18 AND AGE <= 20 AND HAS worker_status
    THEN GRANT "Minimum wage: GBP 8.60 per hour"
}

// Under 18 Rate
STATUTE nmwa1998-under18: "NMWA 1998 - Under 18 Rate" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-01

    WHEN AGE >= 16 AND AGE < 18 AND HAS worker_status
    THEN GRANT "Minimum wage: GBP 6.40 per hour"
}

// Apprentice Rate
STATUTE nmwa1998-apprentice: "NMWA 1998 - Apprentice Rate" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-01

    WHEN HAS apprentice_status AND (AGE < 19 OR HAS first_year_apprentice)
    THEN GRANT "Apprentice rate: GBP 6.40 per hour"
}

// =============================================================================
// Equality Act 2010 (EA 2010)
// =============================================================================

// Protected Characteristics
STATUTE ea2010-protected-characteristics: "EA 2010 - Protected Characteristics" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 2010-10-01

    WHEN HAS worker_status OR HAS job_applicant_status
    THEN GRANT "Protection from discrimination based on 9 protected characteristics"

    DISCRETION "Proportionate means for legitimate aim may justify indirect discrimination"
}

// Reasonable Adjustments for Disability
STATUTE ea2010-reasonable-adjustments: "EA 2010 - Reasonable Adjustments" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 2010-10-01

    WHEN HAS worker_status AND HAS disability_status
    THEN OBLIGATION "Employer must make reasonable adjustments"

    DISCRETION "What is 'reasonable' depends on cost, practicability, resources"
}

// Equal Pay
STATUTE ea2010-equal-pay: "EA 2010 - Equal Pay" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 2010-10-01

    WHEN HAS worker_status
    THEN GRANT "Right to equal pay for equal work regardless of sex"

    DISCRETION "Material factor defence may justify pay differences"
}

// =============================================================================
// Statutory Sick Pay (SSP)
// =============================================================================

STATUTE ssp-eligibility: "SSP - Statutory Sick Pay Eligibility" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-06

    WHEN HAS employee_status AND HAS sick_for_4_plus_days AND
         INCOME >= 123 AND NOT HAS already_receiving_ssp_28_weeks
    THEN GRANT "SSP: GBP 116.75 per week for up to 28 weeks"
}

// =============================================================================
// Statutory Maternity Leave and Pay
// =============================================================================

STATUTE sml-eligibility: "Statutory Maternity Leave" {
    JURISDICTION "GB"
    VERSION 1
    EFFECTIVE_DATE 2024-04-01

    WHEN HAS employee_status AND HAS pregnant_or_new_mother
    THEN GRANT "Up to 52 weeks maternity leave (39 weeks paid)"
}

STATUTE smp-eligibility: "Statutory Maternity Pay" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-07

    WHEN HAS employee_status AND HAS 26_weeks_service_at_15th_week AND
         INCOME >= 123
    THEN GRANT "SMP: 90% of earnings (6 weeks) then GBP 184.03/week (33 weeks)"
}

// =============================================================================
// Statutory Paternity Leave and Pay
// =============================================================================

STATUTE spl-eligibility: "Statutory Paternity Leave" {
    JURISDICTION "GB"
    VERSION 2024
    EFFECTIVE_DATE 2024-04-07

    WHEN HAS employee_status AND HAS 26_weeks_service AND HAS new_father_status
    THEN GRANT "Up to 2 weeks paternity leave"
}
"#;

/// Creates an employee entity
fn create_employee(
    name: &str,
    age: u32,
    weekly_earnings: u64,
    attributes: &[(&str, bool)],
) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("name", name.to_string());
    entity.set_attribute("age", age.to_string());
    entity.set_attribute("income", weekly_earnings.to_string()); // Weekly earnings

    for (key, value) in attributes {
        if *value {
            entity.set_attribute(key, "true".to_string());
        }
    }

    entity
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   UK EMPLOYMENT LAW RIGHTS CHECKER - Legalis-RS Demo");
    println!("   Employment Rights Act 1996 | Working Time Regulations 1998");
    println!("   National Minimum Wage Act 1998 | Equality Act 2010");
    println!("{}\n", "=".repeat(80));

    // Step 1: Parse UK employment statutes
    println!("Step 1: Parsing UK employment law statutes from DSL...\n");
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(UK_EMPLOYMENT_STATUTES)?;
    println!("   Parsed {} statutory provisions:", statutes.len());
    for statute in &statutes {
        println!("   - {} ({})", statute.id, statute.title);
    }
    println!();

    // Step 2: Verify statute consistency
    println!("Step 2: Verifying statutory consistency...\n");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    if result.passed {
        println!("   [OK] All statutes passed verification");
    } else {
        for error in &result.errors {
            println!("   [ERROR] {:?}", error);
        }
    }
    println!();

    // Step 3: Test employee scenarios
    println!("Step 3: Evaluating employee rights scenarios...\n");

    let scenarios = vec![
        // Scenario 1: Full-time employee, 3 years service, age 35
        (
            "Senior Full-Time Employee",
            35u32,
            800u64, // Weekly earnings
            vec![
                ("employee_status", true),
                ("worker_status", true),
                ("two_years_continuous_service", true),
                ("two_years_service", true),
                ("one_month_service", true),
            ],
        ),
        // Scenario 2: Young worker, 19, part-time
        (
            "Young Part-Time Worker",
            19,
            250,
            vec![("worker_status", true), ("one_month_service", true)],
        ),
        // Scenario 3: Pregnant employee
        (
            "Pregnant Employee",
            28,
            600,
            vec![
                ("employee_status", true),
                ("worker_status", true),
                ("pregnant_or_new_mother", true),
                ("26_weeks_service_at_15th_week", true),
                ("one_month_service", true),
            ],
        ),
        // Scenario 4: Disabled employee requiring adjustments
        (
            "Disabled Employee",
            42,
            700,
            vec![
                ("employee_status", true),
                ("worker_status", true),
                ("disability_status", true),
                ("two_years_continuous_service", true),
                ("two_years_service", true),
            ],
        ),
        // Scenario 5: Apprentice under 19
        (
            "Young Apprentice",
            17,
            200,
            vec![
                ("worker_status", true),
                ("apprentice_status", true),
                ("first_year_apprentice", true),
            ],
        ),
        // Scenario 6: Zero-hours worker with long shifts
        (
            "Zero-Hours Worker",
            25,
            300,
            vec![("worker_status", true), ("shift_over_6_hours", true)],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (scenario_name, age, earnings, attrs) in &scenarios {
        let entity = create_employee(scenario_name, *age, *earnings, attrs);
        let entity_id = entity.id();

        println!(
            "   === {} (Age: {}, Weekly: GBP {}) ===",
            scenario_name, age, earnings
        );

        let mut applicable_rights = Vec::new();

        for statute in &statutes {
            let eligible = check_eligibility(&entity, statute);

            if eligible {
                applicable_rights.push(statute.id.clone());
                println!("   [+] {}", statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "uk-employment-checker".to_string(),
                    },
                    statute.id.clone(),
                    entity_id,
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

        println!("   Total rights applicable: {}", applicable_rights.len());
        println!();
    }

    // Step 4: Visualization
    println!("Step 4: Generating decision tree for unfair dismissal protection...\n");
    if let Some(unfair_dismissal) = statutes
        .iter()
        .find(|s| s.id == "era1996-s94-unfair-dismissal")
    {
        match DecisionTree::from_statute(unfair_dismissal) {
            Ok(tree) => {
                let ascii = tree.to_ascii();
                println!("{}", ascii);
            }
            Err(e) => println!("   Warning: Could not generate tree: {:?}", e),
        }
    }
    println!();

    // Step 5: Population simulation
    println!("Step 5: Running workforce simulation (500 employees)...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;

    println!("   Simulation Results:");
    println!("   -------------------------------------------");
    println!(
        "   Total eligibility checks: {}",
        metrics.total_applications
    );
    println!("   Deterministic outcomes: {}", metrics.deterministic_count);
    println!(
        "   Tribunal discretion needed: {}",
        metrics.discretion_count
    );
    println!();

    // Summary
    println!("{}", "=".repeat(80));
    println!("   UK EMPLOYMENT LAW DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Key UK Employment Rights Demonstrated:");
    println!("   - Unfair dismissal protection (ERA 1996 s.94)");
    println!("   - Statutory notice periods (ERA 1996 s.86)");
    println!("   - Working time limits (WTR 1998)");
    println!("   - National Minimum Wage bands (NMWA 1998)");
    println!("   - Discrimination protection (EA 2010)");
    println!("   - Family-friendly rights (SMP, SPL)");
    println!();
    println!("   Enforcement Bodies:");
    println!("   - Employment Tribunal (individual claims)");
    println!("   - ACAS (conciliation)");
    println!("   - HMRC (NMW enforcement)");
    println!("   - EHRC (equality matters)");
    println!();

    Ok(())
}

/// Checks if an entity is eligible for a statutory right
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

/// Evaluates a single condition against an entity
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
        _ => true,
    }
}
