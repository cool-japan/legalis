//! South Korean Labor Standards Act (근로기준법)
//!
//! This example demonstrates how to use Legalis-RS for Korean labor law
//! compliance checking under the Labor Standards Act.
//!
//! ## Korean Labor Law Context
//!
//! South Korea has strong worker protections including:
//! - 52-hour workweek limit
//! - Mandatory severance pay
//! - Strong dismissal protections
//! - Comprehensive social insurance

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

const KR_LABOR_STATUTES: &str = r#"
// =============================================================================
// 근로기준법 (Labor Standards Act) - South Korea
// =============================================================================

// Article 2 - Employee Status
STATUTE kr-lsa-art2-employee: "Labor Standards Act Art. 2 - Employee Definition" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS provides_labor AND HAS wages_received AND HAS subordinate_relationship
    THEN GRANT "Employee status under Labor Standards Act"

    DISCRETION "Subordination assessed based on actual working relationship, not contract form"
}

// Article 17 - Written Employment Contract
STATUTE kr-lsa-art17-contract: "Labor Standards Act Art. 17 - Employment Contract" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status
    THEN OBLIGATION "Employer must provide written statement of working conditions"
}

// Article 23 - Dismissal Restrictions
STATUTE kr-lsa-art23-dismissal: "Labor Standards Act Art. 23 - Unfair Dismissal" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND NOT HAS justifiable_reason
    THEN PROHIBITION "Dismissal without justifiable reason is void"

    DISCRETION "Justifiable reason determination by Labor Relations Commission"
}

// Article 26 - Advance Notice of Dismissal
STATUTE kr-lsa-art26-notice: "Labor Standards Act Art. 26 - 30-Day Notice" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS to_be_dismissed
    THEN OBLIGATION "30 days advance notice or 30 days pay in lieu"

    EXCEPTION WHEN HAS probation_under_3_months
}

// Article 36 - Wage Payment Principles
STATUTE kr-lsa-art36-wages: "Labor Standards Act Art. 36 - Wage Payment" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status
    THEN OBLIGATION "Pay full wages in currency, directly to worker, at least monthly"
}

// Article 43 - Minimum Wage
STATUTE kr-minimum-wage-2024: "Minimum Wage Act - 2024 Rate" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status
    THEN GRANT "Minimum wage: 9,860 KRW per hour (2024)"
}

// =============================================================================
// Working Hours (Article 50-53)
// =============================================================================

// Article 50 - Standard Working Hours
STATUTE kr-lsa-art50-hours: "Labor Standards Act Art. 50 - Working Hours" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status
    THEN PROHIBITION "Working hours exceeding 40 hours/week, 8 hours/day"

    EXCEPTION WHEN HAS worker_consent_overtime
}

// Article 53 - Extended Working Hours Limit
STATUTE kr-lsa-art53-overtime-limit: "Labor Standards Act Art. 53 - 52-Hour Week" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS company_5_or_more_workers
    THEN PROHIBITION "Total working hours (including OT) exceeding 52 hours/week"
}

// Article 56 - Overtime Premium
STATUTE kr-lsa-art56-overtime-pay: "Labor Standards Act Art. 56 - Overtime Premium" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS overtime_worked
    THEN GRANT "50% premium for overtime, night work, and holiday work"
}

// =============================================================================
// Leave Entitlements
// =============================================================================

// Article 60 - Annual Leave
STATUTE kr-lsa-art60-annual-leave: "Labor Standards Act Art. 60 - Annual Leave" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS one_year_service AND HAS 80_percent_attendance
    THEN GRANT "15 days paid annual leave (increases with tenure)"
}

// Maternity Leave
STATUTE kr-maternity-leave: "Labor Standards Act - Maternity Leave" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS pregnant_employee
    THEN GRANT "90 days maternity leave (45 days paid by employer, 45 days by insurance)"
}

// Paternity Leave
STATUTE kr-paternity-leave: "Equal Employment Act - Paternity Leave" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS spouse_gave_birth
    THEN GRANT "10 days paid paternity leave"
}

// =============================================================================
// Severance Pay (근로자퇴직급여보장법)
// =============================================================================

STATUTE kr-severance-pay: "Retirement Benefit Security Act - Severance" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status AND HAS one_year_or_more_service
    THEN GRANT "Severance pay: 30 days wages per year of service"
}

// =============================================================================
// Social Insurance (4대보험)
// =============================================================================

STATUTE kr-social-insurance: "Four Major Social Insurances" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status
    THEN OBLIGATION "Enrollment in: National Pension, Health Insurance, Employment Insurance, Industrial Accident Insurance"
}

// =============================================================================
// Special Categories
// =============================================================================

STATUTE kr-fixed-term-limit: "Fixed-term Employment Act - 2-Year Limit" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS fixed_term_contract AND HAS exceeded_2_years
    THEN GRANT "Deemed as permanent employee after 2 years"
}

STATUTE kr-dispatched-worker: "Worker Dispatch Act - Protections" {
    JURISDICTION "KR"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS dispatched_worker_status AND HAS exceeded_2_years
    THEN GRANT "Right to request direct employment by user company"
}
"#;

fn create_worker(name: &str, attrs: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("name", name.to_string());
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
    println!("   KOREAN LABOR STANDARDS ACT (근로기준법) - Legalis-RS Demo");
    println!("   Worker Rights and Employer Obligations");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(KR_LABOR_STATUTES)?;
    println!(
        "Step 1: Parsed {} labor provisions (노동법 조항)\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Verification {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Evaluating worker scenarios...\n");

    let workers = vec![
        (
            "정규직 근로자 (Regular Employee)",
            vec![
                ("employee_status", true),
                ("provides_labor", true),
                ("wages_received", true),
                ("subordinate_relationship", true),
                ("one_year_service", true),
                ("80_percent_attendance", true),
                ("company_5_or_more_workers", true),
            ],
        ),
        (
            "기간제 근로자 (Fixed-term)",
            vec![
                ("employee_status", true),
                ("fixed_term_contract", true),
                ("exceeded_2_years", true),
            ],
        ),
        (
            "파견 근로자 (Dispatched)",
            vec![
                ("dispatched_worker_status", true),
                ("exceeded_2_years", true),
            ],
        ),
        (
            "임산부 근로자 (Pregnant)",
            vec![("employee_status", true), ("pregnant_employee", true)],
        ),
        (
            "신규 입사자 (New Hire)",
            vec![
                ("employee_status", true),
                ("probation_under_3_months", true),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, attrs) in &workers {
        let entity = create_worker(name, attrs);
        println!("   === {} ===", name);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                println!("   [+] {}", statute.id);
                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "kr-labor".to_string(),
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
        "   Total: {} | Deterministic: {} | 노동위원회 판단 필요: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   한국 노동법 DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!("\n   주요 근로자 보호:");
    println!("   - 52시간 근무제 (52-hour workweek)");
    println!("   - 퇴직금 (Severance pay)");
    println!("   - 해고 제한 (Dismissal restrictions)");
    println!("   - 4대 보험 (Four social insurances)");
    println!("\n   시행기관: 고용노동부 (Ministry of Employment and Labor)\n");

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
                _ => false,
            })
            .unwrap_or(false),
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => evaluate_condition(entity, l) && evaluate_condition(entity, r),
        Condition::Or(l, r) => evaluate_condition(entity, l) || evaluate_condition(entity, r),
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true,
    }
}
