//! India Right to Information Act 2005 (RTI Act)
//!
//! This example demonstrates how to use Legalis-RS for India's landmark
//! transparency legislation - the Right to Information Act, 2005.
//!
//! ## RTI Act Overview
//!
//! The RTI Act empowers citizens to:
//! - Request information from public authorities
//! - Receive responses within 30 days (48 hours for life/liberty matters)
//! - Appeal denials to Information Commissions
//!
//! ## Key Sections Modeled
//!
//! - Section 3: Right to information
//! - Section 4: Proactive disclosure obligations
//! - Section 6: Request procedures
//! - Section 7: Response timelines
//! - Section 8: Exemptions from disclosure
//! - Section 19: Appeals process

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;
use legalis_viz::DecisionTree;

/// RTI Act statutes in DSL format
const RTI_STATUTES: &str = r#"
// =============================================================================
// Right to Information Act, 2005 (India)
// =============================================================================

// Section 3 - Right to Information
STATUTE rti-s3-right: "RTI Act s.3 - Right to Information" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS indian_citizen
    THEN GRANT "Right to access information from public authorities"
}

// Section 4 - Proactive Disclosure
STATUTE rti-s4-proactive: "RTI Act s.4 - Suo Motu Disclosure" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS public_authority
    THEN OBLIGATION "Proactively publish organizational info, functions, duties, powers, rules, records, budget, subsidies, concessions, permits"
}

// Section 6 - Request to Obtain Information
STATUTE rti-s6-request: "RTI Act s.6 - Application for Information" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS indian_citizen AND HAS written_application AND HAS fee_paid
    THEN GRANT "Right to submit RTI application to any public authority"

    DISCRETION "No requirement to give reasons for seeking information"
}

// Section 7 - Response Timeline (Standard)
STATUTE rti-s7-timeline-30: "RTI Act s.7 - 30-Day Response" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND NOT HAS life_liberty_matter
    THEN OBLIGATION "Provide information or rejection within 30 days"
}

// Section 7 - Response Timeline (Life/Liberty)
STATUTE rti-s7-timeline-48hr: "RTI Act s.7 - 48-Hour Response" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS life_liberty_matter
    THEN OBLIGATION "Provide information within 48 hours"
}

// Section 7 - Third Party Information
STATUTE rti-s7-third-party: "RTI Act s.7(7) - Third Party Consultation" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS third_party_information AND HAS confidentiality_claimed
    THEN OBLIGATION "Consult third party; decide within 40 days total"

    DISCRETION "Balance third party interests against public interest"
}

// =============================================================================
// Section 8 - Exemptions from Disclosure
// =============================================================================

STATUTE rti-s8-security: "RTI Act s.8(1)(a) - National Security Exemption" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS affects_sovereignty_security
    THEN PROHIBITION "Disclosure of information affecting sovereignty, security, strategic interests"

    DISCRETION "Harm must be demonstrable, not speculative"
}

STATUTE rti-s8-court-contempt: "RTI Act s.8(1)(b) - Court Contempt" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS expressly_forbidden_by_court
    THEN PROHIBITION "Disclosure expressly forbidden by court or tribunal"
}

STATUTE rti-s8-parliament-privilege: "RTI Act s.8(1)(c) - Parliamentary Privilege" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS breach_of_parliament_privilege
    THEN PROHIBITION "Disclosure constituting breach of Parliament/Legislature privilege"
}

STATUTE rti-s8-commercial-confidence: "RTI Act s.8(1)(d) - Commercial Confidence" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS commercial_confidence AND HAS competitive_harm
    THEN PROHIBITION "Disclosure of trade secrets or commercial confidence"

    EXCEPTION WHEN HAS larger_public_interest
}

STATUTE rti-s8-fiduciary: "RTI Act s.8(1)(e) - Fiduciary Relationship" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS fiduciary_relationship
    THEN PROHIBITION "Information in fiduciary capacity"

    EXCEPTION WHEN HAS larger_public_interest
}

STATUTE rti-s8-foreign-govt: "RTI Act s.8(1)(f) - Foreign Government" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS foreign_govt_confidence
    THEN PROHIBITION "Information received in confidence from foreign government"
}

STATUTE rti-s8-endanger-life: "RTI Act s.8(1)(g) - Endanger Life/Safety" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS would_endanger_life_safety
    THEN PROHIBITION "Disclosure endangering life or physical safety of any person"
}

STATUTE rti-s8-investigation: "RTI Act s.8(1)(h) - Investigation Records" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS impede_investigation
    THEN PROHIBITION "Information impeding investigation/prosecution of offenders"
}

STATUTE rti-s8-cabinet: "RTI Act s.8(1)(i) - Cabinet Papers" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS cabinet_papers AND NOT HAS decision_made_public
    THEN PROHIBITION "Cabinet papers including Council of Ministers deliberations"

    EXCEPTION WHEN HAS decision_made_public
}

STATUTE rti-s8-personal-privacy: "RTI Act s.8(1)(j) - Personal Privacy" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS personal_information AND HAS no_public_interest
    THEN PROHIBITION "Personal information with no relationship to public activity"

    EXCEPTION WHEN HAS larger_public_interest
    DISCRETION "Balance privacy against public interest in disclosure"
}

// Section 8(2) - 20-Year Rule
STATUTE rti-s8-20year-rule: "RTI Act s.8(2) - 20-Year Disclosure" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS information_over_20_years
    THEN GRANT "Information over 20 years old must be disclosed"

    EXCEPTION WHEN HAS affects_sovereignty_security
}

// Section 8(3) - Public Interest Override
STATUTE rti-s8-public-interest: "RTI Act s.8(3) - Public Interest Override" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_application_received AND HAS larger_public_interest AND HAS exempted_category
    THEN GRANT "Public interest in disclosure outweighs harm - must disclose"

    DISCRETION "Weighing public interest is a judicial discretion matter"
}

// =============================================================================
// Section 19 - Appeals
// =============================================================================

STATUTE rti-s19-first-appeal: "RTI Act s.19(1) - First Appeal" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS rti_decision_received AND HAS within_30_days AND HAS aggrieved
    THEN GRANT "Right to first appeal to officer senior to PIO"
}

STATUTE rti-s19-second-appeal: "RTI Act s.19(3) - Second Appeal" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS first_appeal_decided AND HAS within_90_days AND HAS still_aggrieved
    THEN GRANT "Right to second appeal to Central/State Information Commission"
}

// Section 20 - Penalties
STATUTE rti-s20-penalty: "RTI Act s.20 - Penalty for Non-Compliance" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS pio_failed_to_respond AND NOT HAS reasonable_cause
    THEN OBLIGATION "PIO liable for penalty of Rs. 250/day (max Rs. 25,000)"
}

// BPL Fee Waiver
STATUTE rti-bpl-fee-waiver: "RTI Act - BPL Fee Waiver" {
    JURISDICTION "IN"
    VERSION 1
    EFFECTIVE_DATE 2005-10-12

    WHEN HAS indian_citizen AND HAS below_poverty_line
    THEN GRANT "Exemption from RTI application fees"
}
"#;

/// Creates an RTI scenario entity
fn create_rti_scenario(name: &str, attributes: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());

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
    println!("   RIGHT TO INFORMATION ACT, 2005 - Legalis-RS Demo");
    println!("   Empowering Citizens Through Transparency");
    println!("{}\n", "=".repeat(80));

    // Step 1: Parse RTI statutes
    println!("Step 1: Parsing RTI Act sections from DSL...\n");
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(RTI_STATUTES)?;
    println!("   Parsed {} RTI provisions:", statutes.len());
    for statute in &statutes {
        println!("   - {} ({})", statute.id, statute.title);
    }
    println!();

    // Step 2: Verify statute consistency
    println!("Step 2: Verifying RTI Act consistency...\n");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    if result.passed {
        println!("   [OK] All RTI provisions passed verification");
    }
    println!();

    // Step 3: Test RTI scenarios
    println!("Step 3: Evaluating RTI application scenarios...\n");

    let scenarios = vec![
        (
            "Standard RTI Application",
            vec![
                ("indian_citizen", true),
                ("written_application", true),
                ("fee_paid", true),
                ("rti_application_received", true),
            ],
        ),
        (
            "Life/Liberty Matter - Urgent",
            vec![
                ("indian_citizen", true),
                ("rti_application_received", true),
                ("life_liberty_matter", true),
            ],
        ),
        (
            "BPL Applicant - Fee Waiver",
            vec![
                ("indian_citizen", true),
                ("below_poverty_line", true),
                ("written_application", true),
            ],
        ),
        (
            "National Security Exemption Claim",
            vec![
                ("rti_application_received", true),
                ("affects_sovereignty_security", true),
            ],
        ),
        (
            "Personal Privacy vs Public Interest",
            vec![
                ("rti_application_received", true),
                ("personal_information", true),
                ("larger_public_interest", true),
            ],
        ),
        (
            "20-Year-Old Information",
            vec![
                ("rti_application_received", true),
                ("information_over_20_years", true),
            ],
        ),
        (
            "First Appeal Filed",
            vec![
                ("rti_decision_received", true),
                ("within_30_days", true),
                ("aggrieved", true),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (scenario_name, attrs) in &scenarios {
        let entity = create_rti_scenario(scenario_name, attrs);
        let entity_id = entity.id();

        println!("   === {} ===", scenario_name);

        for statute in &statutes {
            let applicable = check_applicability(&entity, statute);
            if applicable {
                println!("   [+] {} applies", statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "rti-checker".to_string(),
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
        println!();
    }

    // Step 4: Visualization
    println!("Step 4: Decision tree for RTI application...\n");
    if let Some(s3) = statutes.iter().find(|s| s.id == "rti-s3-right") {
        match DecisionTree::from_statute(s3) {
            Ok(tree) => println!("{}", tree.to_ascii()),
            Err(e) => println!("   Warning: {:?}", e),
        }
    }
    println!();

    // Step 5: Simulation
    println!("Step 5: Running RTI simulation (500 applications)...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;

    println!("   Simulation Results:");
    println!("   Total applications: {}", metrics.total_applications);
    println!("   Deterministic: {}", metrics.deterministic_count);
    println!("   Requires IC discretion: {}", metrics.discretion_count);
    println!();

    // Summary
    println!("{}", "=".repeat(80));
    println!("   RTI ACT DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Key RTI Principles:");
    println!("   - Maximum disclosure, minimum exemptions");
    println!("   - 30-day response timeline (48 hours for life/liberty)");
    println!("   - Public interest override for exemptions");
    println!("   - Two-tier appeal: First Appeal + Information Commission");
    println!("   - Penalty for non-compliance (Rs. 250/day)");
    println!();
    println!("   Enforcement: Central/State Information Commissions");
    println!();

    Ok(())
}

fn check_applicability(entity: &BasicEntity, statute: &Statute) -> bool {
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
