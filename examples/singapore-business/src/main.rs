//! Singapore Business Regulations
//!
//! This example demonstrates how to use Legalis-RS for Singapore business
//! regulatory compliance. It covers:
//!
//! - Companies Act (Cap. 50)
//! - Personal Data Protection Act 2012 (PDPA)
//! - Employment Act (Cap. 91)
//! - ACRA Registration Requirements
//!
//! ## Singapore Business Environment
//!
//! Singapore consistently ranks as one of the easiest places to do business.
//! Clear, predictable regulations and efficient enforcement are key factors.

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

/// Singapore business statutes in DSL format
const SG_BUSINESS_STATUTES: &str = r#"
// =============================================================================
// Companies Act (Cap. 50) - Singapore
// =============================================================================

// Company Registration
STATUTE sg-companies-registration: "Companies Act - Registration Requirement" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2006-01-30

    WHEN HAS business_activity_in_singapore
    THEN OBLIGATION "Register with ACRA within 14 days of commencing business"
}

// Minimum Directors
STATUTE sg-companies-directors: "Companies Act s.145 - Minimum Directors" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2006-01-30

    WHEN HAS private_company
    THEN OBLIGATION "At least 1 director ordinarily resident in Singapore"
}

// Company Secretary
STATUTE sg-companies-secretary: "Companies Act s.171 - Company Secretary" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2006-01-30

    WHEN HAS registered_company
    THEN OBLIGATION "Appoint company secretary within 6 months of incorporation"
}

// Annual Returns
STATUTE sg-companies-annual-return: "Companies Act s.197 - Annual Return" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2006-01-30

    WHEN HAS registered_company
    THEN OBLIGATION "File annual return with ACRA within 30 days of AGM"
}

// =============================================================================
// Personal Data Protection Act 2012 (PDPA)
// =============================================================================

// Consent Obligation
STATUTE sg-pdpa-consent: "PDPA s.13 - Consent Obligation" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2014-07-02

    WHEN HAS collects_personal_data AND HAS organization_status
    THEN OBLIGATION "Obtain individual's consent before collecting, using, disclosing data"

    EXCEPTION WHEN HAS legitimate_interest_exception
    EXCEPTION WHEN HAS business_improvement_exception
}

// Purpose Limitation
STATUTE sg-pdpa-purpose: "PDPA s.18 - Purpose Limitation" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2014-07-02

    WHEN HAS organization_status AND HAS personal_data_collected
    THEN PROHIBITION "Using data for purposes not notified to individual"
}

// Access Obligation
STATUTE sg-pdpa-access: "PDPA s.21 - Access to Personal Data" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2014-07-02

    WHEN HAS data_access_request AND HAS organization_status
    THEN OBLIGATION "Provide individual with their personal data upon request"
}

// Correction Obligation
STATUTE sg-pdpa-correction: "PDPA s.22 - Correction of Data" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2014-07-02

    WHEN HAS data_correction_request AND HAS error_in_data
    THEN OBLIGATION "Correct personal data upon request unless proper to decline"
}

// Data Breach Notification
STATUTE sg-pdpa-breach: "PDPA s.26B - Data Breach Notification" {
    JURISDICTION "SG"
    VERSION 2020
    EFFECTIVE_DATE 2021-02-01

    WHEN HAS notifiable_data_breach AND HAS organization_status
    THEN OBLIGATION "Notify PDPC and affected individuals within 3 days of assessment"
}

// Do Not Call Registry
STATUTE sg-pdpa-dnc: "PDPA Part IX - Do Not Call Registry" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2014-01-02

    WHEN HAS marketing_message AND HAS singapore_number_registered_dnc
    THEN PROHIBITION "Sending marketing messages to DNC-registered numbers"
}

// =============================================================================
// Employment Act (Cap. 91)
// =============================================================================

// Salary Payment
STATUTE sg-employment-salary: "Employment Act s.20 - Salary Payment" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 1968-08-15

    WHEN HAS employee_status AND HAS covered_by_employment_act
    THEN OBLIGATION "Pay salary within 7 days after end of salary period"
}

// CPF Contributions
STATUTE sg-cpf-contribution: "CPF Act - Employer Contribution" {
    JURISDICTION "SG"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employer_status AND HAS singapore_employee_or_pr
    THEN OBLIGATION "Contribute to CPF (up to 17% employer, 20% employee)"
}

// Annual Leave
STATUTE sg-employment-annual-leave: "Employment Act s.43 - Annual Leave" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 1968-08-15

    WHEN HAS employee_status AND HAS covered_by_employment_act AND HAS 3_months_service
    THEN GRANT "Minimum 7 days paid annual leave (increasing with tenure)"
}

// Sick Leave
STATUTE sg-employment-sick-leave: "Employment Act s.89 - Sick Leave" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 1968-08-15

    WHEN HAS employee_status AND HAS covered_by_employment_act AND HAS 6_months_service
    THEN GRANT "14 days outpatient sick leave, 60 days hospitalization leave"
}

// =============================================================================
// Foreign Worker Requirements
// =============================================================================

STATUTE sg-foreign-worker-permit: "Employment of Foreign Manpower Act - Work Permit" {
    JURISDICTION "SG"
    VERSION 1
    EFFECTIVE_DATE 2007-07-01

    WHEN HAS foreign_worker AND NOT HAS valid_work_pass
    THEN PROHIBITION "Employment of foreign worker without valid work pass"
}

STATUTE sg-ep-requirement: "MOM - Employment Pass Requirement" {
    JURISDICTION "SG"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS foreign_professional AND INCOME >= 5000
    THEN GRANT "Eligible for Employment Pass (subject to COMPASS framework)"

    DISCRETION "COMPASS points-based assessment for EP applications"
}
"#;

fn create_business_scenario(
    name: &str,
    attrs: &[(&str, bool)],
    income: Option<u64>,
) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
    if let Some(inc) = income {
        entity.set_attribute("income", inc.to_string());
    }
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
    println!("   SINGAPORE BUSINESS REGULATIONS - Legalis-RS Demo");
    println!("   Companies Act | PDPA | Employment Act");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(SG_BUSINESS_STATUTES)?;
    println!(
        "Step 1: Parsed {} Singapore business regulations\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Verification {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Evaluating business scenarios...\n");

    let scenarios = vec![
        (
            "New Company Registration",
            vec![
                ("business_activity_in_singapore", true),
                ("private_company", true),
            ],
            None,
        ),
        (
            "PDPA Data Collection",
            vec![
                ("organization_status", true),
                ("collects_personal_data", true),
            ],
            None,
        ),
        (
            "Data Breach Incident",
            vec![
                ("organization_status", true),
                ("notifiable_data_breach", true),
            ],
            None,
        ),
        (
            "Employment Pass Application",
            vec![("foreign_professional", true)],
            Some(6000u64),
        ),
        (
            "CPF Contribution Due",
            vec![
                ("employer_status", true),
                ("singapore_employee_or_pr", true),
            ],
            None,
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, attrs, income) in &scenarios {
        let entity = create_business_scenario(name, attrs, *income);
        println!("   === {} ===", name);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                println!("   [+] {}", statute.id);
                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "sg-business".to_string(),
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

    println!("Step 4: Running simulation...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;
    println!(
        "   Total: {} | Deterministic: {} | Discretion: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   SINGAPORE BUSINESS DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!("\n   Key Regulatory Bodies:");
    println!("   - ACRA (Company Registration)");
    println!("   - PDPC (Data Protection)");
    println!("   - MOM (Employment/Foreign Workers)");
    println!("   - CPF Board (Social Security)\n");

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
            .map(|v| compare(v as u64, *value as u64, operator))
            .unwrap_or(false),
        Condition::Income { operator, value } => entity
            .get_attribute("income")
            .and_then(|s| s.parse::<u64>().ok())
            .map(|v| compare(v, *value, operator))
            .unwrap_or(false),
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => evaluate_condition(entity, l) && evaluate_condition(entity, r),
        Condition::Or(l, r) => evaluate_condition(entity, l) || evaluate_condition(entity, r),
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true,
    }
}

fn compare(actual: u64, expected: u64, op: &ComparisonOp) -> bool {
    match op {
        ComparisonOp::GreaterOrEqual => actual >= expected,
        ComparisonOp::GreaterThan => actual > expected,
        ComparisonOp::LessOrEqual => actual <= expected,
        ComparisonOp::LessThan => actual < expected,
        ComparisonOp::Equal => actual == expected,
        ComparisonOp::NotEqual => actual != expected,
    }
}
