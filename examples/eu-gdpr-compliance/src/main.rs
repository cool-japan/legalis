//! EU GDPR (General Data Protection Regulation) Compliance Checker
//!
//! This example demonstrates how to use Legalis-RS to build a GDPR compliance
//! verification system. It showcases:
//!
//! - Data processing lawfulness verification (Article 6)
//! - Data subject rights compliance (Articles 12-23)
//! - Data breach notification requirements (Articles 33-34)
//! - Cross-border transfer rules (Articles 44-49)
//! - Controller/Processor obligations
//!
//! ## GDPR Overview
//!
//! The General Data Protection Regulation (EU) 2016/679 is a regulation in EU law
//! on data protection and privacy. It applies to all EU member states and has
//! extraterritorial effect on organizations processing EU residents' data.
//!
//! ## Key Articles Modeled
//!
//! - Article 6: Lawfulness of processing (6 legal bases)
//! - Article 7: Conditions for consent
//! - Article 15: Right of access
//! - Article 17: Right to erasure ("right to be forgotten")
//! - Article 20: Right to data portability
//! - Article 33: Data breach notification to supervisory authority
//! - Article 35: Data Protection Impact Assessment (DPIA)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;
use legalis_viz::DecisionTree;

/// GDPR statutes in DSL format
const GDPR_STATUTES: &str = r#"
// =============================================================================
// GDPR Article 6: Lawfulness of Processing
// =============================================================================

// Article 6(1)(a) - Consent
STATUTE gdpr-art6-consent: "GDPR Art. 6(1)(a) - Consent-based Processing" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS data_subject_consent AND HAS specific_purpose AND HAS freely_given
    THEN GRANT "Lawful processing based on consent"

    DISCRETION "Consent must be freely given, specific, informed and unambiguous"
}

// Article 6(1)(b) - Contract Performance
STATUTE gdpr-art6-contract: "GDPR Art. 6(1)(b) - Contract Performance" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS contractual_relationship AND HAS processing_necessary_for_contract
    THEN GRANT "Lawful processing for contract performance"
}

// Article 6(1)(c) - Legal Obligation
STATUTE gdpr-art6-legal: "GDPR Art. 6(1)(c) - Legal Obligation" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS legal_obligation_to_process
    THEN GRANT "Lawful processing for legal obligation compliance"
}

// Article 6(1)(d) - Vital Interests
STATUTE gdpr-art6-vital: "GDPR Art. 6(1)(d) - Vital Interests" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS vital_interests_at_stake AND HAS life_threatening_situation
    THEN GRANT "Lawful processing to protect vital interests"

    DISCRETION "Determining 'vital interests' requires case-by-case assessment"
}

// Article 6(1)(f) - Legitimate Interests
STATUTE gdpr-art6-legitimate: "GDPR Art. 6(1)(f) - Legitimate Interests" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS legitimate_interest AND NOT HAS overriding_data_subject_interests
    THEN GRANT "Lawful processing based on legitimate interests"

    DISCRETION "Balancing test: controller's interests vs data subject's rights"
}

// =============================================================================
// GDPR Data Subject Rights (Articles 12-23)
// =============================================================================

// Article 15 - Right of Access
STATUTE gdpr-art15-access: "GDPR Art. 15 - Right of Access" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS data_subject_request AND HAS identity_verified
    THEN OBLIGATION "Provide copy of personal data within 30 days"

    EXCEPTION WHEN HAS manifestly_unfounded_request
    DISCRETION "May extend response time by 60 days for complex requests"
}

// Article 17 - Right to Erasure
STATUTE gdpr-art17-erasure: "GDPR Art. 17 - Right to Erasure" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS erasure_request AND (
        HAS data_no_longer_necessary OR
        HAS consent_withdrawn OR
        HAS unlawful_processing
    )
    THEN OBLIGATION "Erase personal data without undue delay"

    EXCEPTION WHEN HAS legal_retention_requirement
    EXCEPTION WHEN HAS public_interest_archiving
    DISCRETION "Balancing right to erasure against freedom of expression"
}

// Article 20 - Right to Data Portability
STATUTE gdpr-art20-portability: "GDPR Art. 20 - Right to Data Portability" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS portability_request AND HAS automated_processing AND (
        HAS data_subject_consent OR HAS contractual_relationship
    )
    THEN OBLIGATION "Provide data in structured, machine-readable format"
}

// =============================================================================
// GDPR Data Breach Notification (Articles 33-34)
// =============================================================================

// Article 33 - Notification to Supervisory Authority
STATUTE gdpr-art33-breach-authority: "GDPR Art. 33 - Breach Notification to Authority" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS personal_data_breach AND NOT HAS unlikely_risk_to_rights
    THEN OBLIGATION "Notify supervisory authority within 72 hours"

    DISCRETION "Assessing 'risk to rights and freedoms' requires expertise"
}

// Article 34 - Notification to Data Subject
STATUTE gdpr-art34-breach-subject: "GDPR Art. 34 - Breach Notification to Data Subject" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS personal_data_breach AND HAS high_risk_to_rights
    THEN OBLIGATION "Notify affected data subjects without undue delay"

    EXCEPTION WHEN HAS encryption_applied
    EXCEPTION WHEN HAS subsequent_measures_eliminating_risk
    DISCRETION "Determining 'high risk' involves probability and severity assessment"
}

// =============================================================================
// GDPR Data Protection Impact Assessment (Article 35)
// =============================================================================

STATUTE gdpr-art35-dpia: "GDPR Art. 35 - Data Protection Impact Assessment" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS high_risk_processing AND (
        HAS systematic_monitoring OR
        HAS large_scale_special_categories OR
        HAS automated_decision_making
    )
    THEN OBLIGATION "Conduct Data Protection Impact Assessment before processing"

    DISCRETION "DPO consultation required for risk mitigation measures"
}

// =============================================================================
// GDPR Cross-Border Transfers (Articles 44-49)
// =============================================================================

// Article 45 - Adequacy Decision
STATUTE gdpr-art45-adequacy: "GDPR Art. 45 - Transfer based on Adequacy Decision" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS transfer_to_third_country AND HAS adequacy_decision_exists
    THEN GRANT "Lawful cross-border transfer based on adequacy"
}

// Article 46 - Appropriate Safeguards (SCCs, BCRs)
STATUTE gdpr-art46-safeguards: "GDPR Art. 46 - Transfer with Appropriate Safeguards" {
    JURISDICTION "EU"
    VERSION 1
    EFFECTIVE_DATE 2018-05-25

    WHEN HAS transfer_to_third_country AND NOT HAS adequacy_decision_exists AND (
        HAS standard_contractual_clauses OR
        HAS binding_corporate_rules OR
        HAS approved_certification
    )
    THEN GRANT "Lawful cross-border transfer with safeguards"

    DISCRETION "Transfer Impact Assessment may be required post-Schrems II"
}
"#;

/// Creates a data processing scenario entity
fn create_processing_scenario(name: &str, attributes: &[(&str, bool)]) -> BasicEntity {
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
    println!("   EU GDPR COMPLIANCE CHECKER - Legalis-RS Demo");
    println!("   General Data Protection Regulation (EU) 2016/679");
    println!("{}\n", "=".repeat(80));

    // Step 1: Parse GDPR statutes
    println!("Step 1: Parsing GDPR articles from DSL...\n");
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(GDPR_STATUTES)?;
    println!("   Parsed {} GDPR articles:", statutes.len());
    for statute in &statutes {
        println!("   - {} ({})", statute.id, statute.title);
    }
    println!();

    // Step 2: Verify statute consistency
    println!("Step 2: Verifying GDPR article consistency...\n");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    if result.passed {
        println!("   [OK] All GDPR articles passed verification");
    } else {
        for error in &result.errors {
            println!("   [ERROR] {:?}", error);
        }
    }
    println!();

    // Step 3: Test scenarios
    println!("Step 3: Evaluating compliance scenarios...\n");

    let scenarios = vec![
        // Scenario 1: Marketing email with valid consent
        (
            "Marketing Campaign with Consent",
            vec![
                ("data_subject_consent", true),
                ("specific_purpose", true),
                ("freely_given", true),
            ],
        ),
        // Scenario 2: Employee payroll processing
        (
            "Employee Payroll Processing",
            vec![
                ("contractual_relationship", true),
                ("processing_necessary_for_contract", true),
            ],
        ),
        // Scenario 3: Data subject access request
        (
            "Data Subject Access Request",
            vec![("data_subject_request", true), ("identity_verified", true)],
        ),
        // Scenario 4: Data breach with high risk
        (
            "High-Risk Data Breach",
            vec![
                ("personal_data_breach", true),
                ("high_risk_to_rights", true),
            ],
        ),
        // Scenario 5: Cross-border transfer to US (post-Privacy Shield)
        (
            "Transfer to US with SCCs",
            vec![
                ("transfer_to_third_country", true),
                ("adequacy_decision_exists", false), // No adequacy for US
                ("standard_contractual_clauses", true),
            ],
        ),
        // Scenario 6: AI-based profiling requiring DPIA
        (
            "AI Profiling System",
            vec![
                ("high_risk_processing", true),
                ("automated_decision_making", true),
                ("large_scale_special_categories", false),
            ],
        ),
        // Scenario 7: Right to erasure request
        (
            "Right to Be Forgotten Request",
            vec![
                ("erasure_request", true),
                ("consent_withdrawn", true),
                ("legal_retention_requirement", false),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (scenario_name, attrs) in &scenarios {
        let entity = create_processing_scenario(scenario_name, attrs);
        let entity_id = entity.id();

        println!("   === {} ===", scenario_name);
        println!(
            "   Attributes: {:?}",
            attrs
                .iter()
                .filter(|(_, v)| *v)
                .map(|(k, _)| *k)
                .collect::<Vec<_>>()
        );

        let mut applicable_articles = Vec::new();

        for statute in &statutes {
            let eligible = check_compliance(&entity, statute);

            if eligible {
                applicable_articles.push(statute.id.clone());
                println!("   [+] {} - COMPLIANT", statute.id);

                // Record in audit trail
                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "gdpr-compliance-checker".to_string(),
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

        if applicable_articles.is_empty() {
            println!("   [!] No applicable legal basis found - POTENTIAL NON-COMPLIANCE");
        }
        println!();
    }

    // Step 4: Generate visualization
    println!("Step 4: Generating decision tree for consent-based processing...\n");
    if let Some(consent_statute) = statutes.iter().find(|s| s.id == "gdpr-art6-consent") {
        match DecisionTree::from_statute(consent_statute) {
            Ok(tree) => {
                let ascii = tree.to_ascii();
                println!("{}", ascii);
            }
            Err(e) => println!("   Warning: Could not generate tree: {:?}", e),
        }
    }
    println!();

    // Step 5: Population simulation
    println!("Step 5: Running compliance simulation (500 data processing operations)...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;

    println!("   Simulation Results:");
    println!("   -------------------------------------------");
    println!(
        "   Total operations evaluated: {}",
        metrics.total_applications
    );
    println!(
        "   Deterministic (clear compliance): {}",
        metrics.deterministic_count
    );
    println!(
        "   Requires DPO review (discretionary): {}",
        metrics.discretion_count
    );
    println!();

    // Step 6: Audit trail verification
    println!("Step 6: Verifying audit trail integrity (GDPR Art. 5(2) accountability)...\n");
    match audit_trail.verify_integrity() {
        Ok(true) => {
            println!("   [OK] Audit trail integrity verified");
            println!(
                "   Total compliance decisions recorded: {}",
                audit_trail.count()
            );
        }
        Ok(false) => println!("   [FAIL] Audit trail integrity check failed"),
        Err(e) => println!("   [ERROR] {:?}", e),
    }
    println!();

    // Summary
    println!("{}", "=".repeat(80));
    println!("   GDPR COMPLIANCE DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Key GDPR Principles Demonstrated:");
    println!("   - Lawfulness of processing (Article 6 legal bases)");
    println!("   - Data subject rights (Articles 15, 17, 20)");
    println!("   - Breach notification requirements (Articles 33-34)");
    println!("   - Data Protection Impact Assessment (Article 35)");
    println!("   - Cross-border transfer rules (Articles 45-46)");
    println!("   - Accountability principle (Article 5(2))");
    println!();
    println!("   Member States: All 27 EU countries + EEA (Norway, Iceland, Liechtenstein)");
    println!();

    Ok(())
}

/// Checks if an entity complies with a GDPR statute
fn check_compliance(entity: &BasicEntity, statute: &Statute) -> bool {
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
