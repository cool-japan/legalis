//! Thai Business Laws (Foreign Business Act, BOI)
//!
//! This example demonstrates how to use Legalis-RS for Thai business
//! regulatory compliance, including foreign investment restrictions.
//!
//! ## Thai Business Environment
//!
//! Key legislation for foreign businesses:
//! - Foreign Business Act B.E. 2542 (1999)
//! - Board of Investment (BOI) Promotion
//! - Civil and Commercial Code
//! - Labor Protection Act

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

const TH_BUSINESS_STATUTES: &str = r#"
// =============================================================================
// Foreign Business Act B.E. 2542 (1999) - Thailand
// พ.ร.บ. การประกอบธุรกิจของคนต่างด้าว พ.ศ. 2542
// =============================================================================

// List 1 - Prohibited Activities
STATUTE th-fba-list1: "FBA List 1 - Prohibited for Foreigners" {
    JURISDICTION "TH"
    VERSION 1
    EFFECTIVE_DATE 1999-03-04

    WHEN HAS foreign_entity AND (
        HAS newspaper_business OR HAS radio_tv_business OR
        HAS rice_farming OR HAS land_trading
    )
    THEN PROHIBITION "These activities are absolutely prohibited for foreigners"
}

// List 2 - Restricted (Cabinet Approval)
STATUTE th-fba-list2: "FBA List 2 - Cabinet Approval Required" {
    JURISDICTION "TH"
    VERSION 1
    EFFECTIVE_DATE 1999-03-04

    WHEN HAS foreign_entity AND (
        HAS arms_production OR HAS domestic_transport OR
        HAS thai_antiques_trading
    )
    THEN OBLIGATION "Obtain Cabinet approval (rarely granted)"
}

// List 3 - Thai Competitive Advantage
STATUTE th-fba-list3: "FBA List 3 - License Required" {
    JURISDICTION "TH"
    VERSION 1
    EFFECTIVE_DATE 1999-03-04

    WHEN HAS foreign_entity AND (
        HAS retail_business OR HAS wholesale_business OR
        HAS service_business OR HAS construction_business
    )
    THEN OBLIGATION "Obtain Foreign Business License from Department of Business Development"

    EXCEPTION WHEN HAS boi_promotion
    EXCEPTION WHEN HAS treaty_exemption
    DISCRETION "Minimum capital and Thai employment requirements apply"
}

// =============================================================================
// BOI Investment Promotion Act B.E. 2520
// พ.ร.บ. ส่งเสริมการลงทุน พ.ศ. 2520
// =============================================================================

// BOI Zone Benefits
STATUTE th-boi-zone1: "BOI Zone 1 - Maximum Benefits" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS boi_promoted_activity AND HAS investment_in_special_economic_zone
    THEN GRANT "Up to 13 years corporate tax exemption + import duty exemptions"
}

STATUTE th-boi-general: "BOI General Incentives" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS boi_promoted_activity
    THEN GRANT "3-8 year CIT exemption, import duty exemption, foreign majority ownership permitted"
}

// Foreign Ownership via BOI
STATUTE th-boi-foreign-ownership: "BOI - 100% Foreign Ownership" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS boi_promoted_activity
    THEN GRANT "100% foreign ownership permitted regardless of FBA restrictions"
}

// =============================================================================
// Company Registration Requirements
// =============================================================================

STATUTE th-company-registration: "CCC - Company Registration" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS business_activity_thailand
    THEN OBLIGATION "Register with Department of Business Development"
}

STATUTE th-minimum-shareholders: "CCC - Minimum Shareholders" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS limited_company
    THEN OBLIGATION "Minimum 3 shareholders for private limited company"
}

STATUTE th-thai-majority: "FBA - Thai Majority Requirement" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 1999-03-04

    WHEN HAS foreign_entity AND HAS fba_restricted_activity AND NOT HAS boi_promotion
    THEN OBLIGATION "Thai nationals must hold at least 51% of shares"

    DISCRETION "Nominee arrangements are illegal - genuine Thai investment required"
}

// =============================================================================
// Labor Protection Act B.E. 2541
// พ.ร.บ. คุ้มครองแรงงาน พ.ศ. 2541
// =============================================================================

STATUTE th-minimum-wage: "Labor Protection - Minimum Wage 2024" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status_thailand
    THEN GRANT "Minimum wage: 330-370 THB/day (varies by province)"
}

STATUTE th-working-hours: "Labor Protection - Working Hours" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status_thailand
    THEN PROHIBITION "Working hours exceeding 8 hours/day, 48 hours/week"
}

STATUTE th-annual-leave: "Labor Protection - Annual Leave" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status_thailand AND HAS one_year_service
    THEN GRANT "Minimum 6 days paid annual leave"
}

STATUTE th-severance-pay: "Labor Protection - Severance Pay" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employee_status_thailand AND HAS employment_terminated_by_employer
    THEN GRANT "Severance pay: 30-400 days wages based on tenure"
}

STATUTE th-foreign-worker-quota: "Foreign Employment - 4:1 Ratio" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS foreign_employees AND HAS thai_company
    THEN OBLIGATION "Maintain minimum 4 Thai employees per 1 foreign employee"

    EXCEPTION WHEN HAS boi_promotion
}

// =============================================================================
// Tax Obligations
// =============================================================================

STATUTE th-corporate-tax: "Revenue Code - Corporate Income Tax" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS company_thailand AND HAS net_profit
    THEN OBLIGATION "Pay 20% corporate income tax (SME rates may apply)"
}

STATUTE th-vat: "Revenue Code - VAT" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS business_activity_thailand AND INCOME >= 1800000
    THEN OBLIGATION "Register for and charge 7% VAT"
}

STATUTE th-withholding-tax: "Revenue Code - Withholding Tax" {
    JURISDICTION "TH"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS payment_to_contractor OR HAS payment_of_rent
    THEN OBLIGATION "Withhold tax at source (1-15% depending on payment type)"
}
"#;

fn create_business(name: &str, income: Option<u64>, attrs: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("name", name.to_string());
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
    println!("   THAI BUSINESS LAWS - Legalis-RS Demo");
    println!("   Foreign Business Act | BOI | Labor Protection");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(TH_BUSINESS_STATUTES)?;
    println!(
        "Step 1: Parsed {} Thai business provisions\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Verification {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Evaluating business scenarios...\n");

    let businesses = vec![
        (
            "Foreign Retail Business (No BOI)",
            None,
            vec![
                ("foreign_entity", true),
                ("retail_business", true),
                ("business_activity_thailand", true),
                ("fba_restricted_activity", true),
                ("company_thailand", true),
                ("net_profit", true),
            ],
        ),
        (
            "BOI Promoted Manufacturing",
            Some(5000000u64),
            vec![
                ("foreign_entity", true),
                ("boi_promoted_activity", true),
                ("business_activity_thailand", true),
                ("company_thailand", true),
            ],
        ),
        (
            "BOI in Special Economic Zone",
            Some(10000000u64),
            vec![
                ("foreign_entity", true),
                ("boi_promoted_activity", true),
                ("investment_in_special_economic_zone", true),
                ("business_activity_thailand", true),
            ],
        ),
        (
            "Thai Employer",
            Some(2500000u64),
            vec![
                ("thai_company", true),
                ("business_activity_thailand", true),
                ("company_thailand", true),
                ("net_profit", true),
                ("foreign_employees", true),
                ("employee_status_thailand", true),
            ],
        ),
        (
            "Prohibited Activity Attempt",
            None,
            vec![("foreign_entity", true), ("land_trading", true)],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, income, attrs) in &businesses {
        let entity = create_business(name, *income, attrs);
        println!("   === {} ===", name);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                let effect = format!("{:?}", statute.effect.effect_type);
                let symbol = if effect.contains("Prohibition") {
                    "[X]"
                } else {
                    "[+]"
                };
                println!("   {} {}", symbol, statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "th-business".to_string(),
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
    println!("   THAI BUSINESS DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!("\n   Key Points for Foreign Investors:");
    println!("   - FBA List 3 activities require license or BOI promotion");
    println!("   - BOI promotion enables 100% foreign ownership");
    println!("   - 4:1 Thai-to-foreign employee ratio (unless BOI exempt)");
    println!("   - 51% Thai ownership required without exemption");
    println!("\n   Authorities:");
    println!("   - DBD (Department of Business Development)");
    println!("   - BOI (Board of Investment)");
    println!("   - Ministry of Labour\n");

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
