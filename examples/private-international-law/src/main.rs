//! Private International Law / Conflict of Laws (国際私法)
//!
//! This example demonstrates how to use Legalis-RS for Private International
//! Law (PIL) / Conflict of Laws analysis across multiple jurisdictions.
//!
//! ## What is Private International Law?
//!
//! Private International Law addresses three key questions:
//! 1. **Jurisdiction**: Which court has authority to hear the case?
//! 2. **Choice of Law**: Which country's law applies to the dispute?
//! 3. **Recognition & Enforcement**: Will foreign judgments be recognized?
//!
//! ## Key Instruments Modeled
//!
//! - Japanese Act on General Rules for Application of Laws (法の適用に関する通則法)
//! - EU Rome I Regulation (Contractual Obligations)
//! - EU Rome II Regulation (Non-Contractual Obligations)
//! - Hague Conventions (Service, Evidence, Child Abduction)
//! - New York Convention (Arbitral Awards)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_verifier::StatuteVerifier;

/// Private International Law rules in DSL format
const PIL_STATUTES: &str = r#"
// =============================================================================
// Japan: Act on General Rules for Application of Laws (法の適用に関する通則法)
// =============================================================================

// Article 4 - Personal Status (Capacity)
STATUTE jp-horei-art4: "Japan PIL Art. 4 - Legal Capacity" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS capacity_question AND HAS natural_person
    THEN GRANT "Legal capacity governed by law of nationality (本国法)"

    EXCEPTION WHEN HAS transaction_in_japan AND HAS foreign_incapacity_unknown
    DISCRETION "Exception if person would have capacity under Japanese law and transacted in Japan"
}

// Article 7 - Contract Choice of Law
STATUTE jp-horei-art7: "Japan PIL Art. 7 - Contract Governing Law" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS contract_dispute AND HAS express_choice_of_law
    THEN GRANT "Law chosen by parties governs the contract (当事者自治)"
}

// Article 8 - Contract: No Choice of Law
STATUTE jp-horei-art8: "Japan PIL Art. 8 - Contract Closest Connection" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS contract_dispute AND NOT HAS express_choice_of_law
    THEN GRANT "Law most closely connected to contract applies (characteristic performer's habitual residence)"

    DISCRETION "Determining closest connection requires case-by-case analysis"
}

// Article 17 - Tort
STATUTE jp-horei-art17: "Japan PIL Art. 17 - Tort Governing Law" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS tort_claim
    THEN GRANT "Law of place of result occurrence (結果発生地法) governs"

    EXCEPTION WHEN HAS result_unforeseeable
    DISCRETION "If result location unforeseeable, law of acting location applies"
}

// Article 24 - Marriage Formation
STATUTE jp-horei-art24: "Japan PIL Art. 24 - Marriage Requirements" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS marriage_question AND HAS substantive_requirements
    THEN GRANT "Each party's national law governs substantive requirements for marriage"
}

// Article 25 - Marriage Effects
STATUTE jp-horei-art25: "Japan PIL Art. 25 - Marriage Effects" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS marriage_effects_question
    THEN GRANT "Common nationality > common habitual residence > closest connection"

    DISCRETION "Cascade through connecting factors in order"
}

// Article 36 - Succession
STATUTE jp-horei-art36: "Japan PIL Art. 36 - Succession" {
    JURISDICTION "JP-PIL"
    VERSION 2006
    EFFECTIVE_DATE 2007-01-01

    WHEN HAS succession_question
    THEN GRANT "Succession governed by decedent's national law at death"
}

// =============================================================================
// EU Rome I Regulation (Contractual Obligations)
// =============================================================================

// Article 3 - Freedom of Choice
STATUTE eu-rome1-art3: "Rome I Art. 3 - Freedom of Choice" {
    JURISDICTION "EU-PIL"
    VERSION 2008
    EFFECTIVE_DATE 2009-12-17

    WHEN HAS contract_dispute AND HAS eu_court AND HAS express_choice_of_law
    THEN GRANT "Contract governed by law chosen by parties"
}

// Article 4 - Applicable Law Absent Choice
STATUTE eu-rome1-art4: "Rome I Art. 4 - Applicable Law" {
    JURISDICTION "EU-PIL"
    VERSION 2008
    EFFECTIVE_DATE 2009-12-17

    WHEN HAS contract_dispute AND HAS eu_court AND NOT HAS express_choice_of_law
    THEN GRANT "Sale: seller's habitual residence; Service: provider's habitual residence"

    DISCRETION "Manifestly closer connection escape clause available"
}

// Article 6 - Consumer Contracts
STATUTE eu-rome1-art6: "Rome I Art. 6 - Consumer Protection" {
    JURISDICTION "EU-PIL"
    VERSION 2008
    EFFECTIVE_DATE 2009-12-17

    WHEN HAS consumer_contract AND HAS eu_court AND HAS professional_directed_activity
    THEN GRANT "Consumer's habitual residence law applies; choice cannot deprive consumer of mandatory protections"
}

// Article 8 - Employment Contracts
STATUTE eu-rome1-art8: "Rome I Art. 8 - Individual Employment" {
    JURISDICTION "EU-PIL"
    VERSION 2008
    EFFECTIVE_DATE 2009-12-17

    WHEN HAS employment_contract AND HAS eu_court
    THEN GRANT "Habitual place of work; choice cannot deprive employee of mandatory protections"
}

// =============================================================================
// EU Rome II Regulation (Non-Contractual Obligations)
// =============================================================================

// Article 4 - General Rule for Torts
STATUTE eu-rome2-art4: "Rome II Art. 4 - Tort General Rule" {
    JURISDICTION "EU-PIL"
    VERSION 2007
    EFFECTIVE_DATE 2009-01-11

    WHEN HAS tort_claim AND HAS eu_court
    THEN GRANT "Law of country where damage occurs (lex loci damni)"

    EXCEPTION WHEN HAS common_habitual_residence
}

// Article 5 - Product Liability
STATUTE eu-rome2-art5: "Rome II Art. 5 - Product Liability" {
    JURISDICTION "EU-PIL"
    VERSION 2007
    EFFECTIVE_DATE 2009-01-11

    WHEN HAS product_liability_claim AND HAS eu_court
    THEN GRANT "Cascade: victim's habitual residence (if marketed there) > place of acquisition > place of damage"
}

// Article 14 - Choice of Law for Torts
STATUTE eu-rome2-art14: "Rome II Art. 14 - Party Autonomy" {
    JURISDICTION "EU-PIL"
    VERSION 2007
    EFFECTIVE_DATE 2009-01-11

    WHEN HAS tort_claim AND HAS eu_court AND HAS post_dispute_choice
    THEN GRANT "Parties may choose applicable law after dispute arises"
}

// =============================================================================
// Hague Conventions
// =============================================================================

// Hague Service Convention 1965
STATUTE hague-service: "Hague Service Convention 1965" {
    JURISDICTION "INTL"
    VERSION 1965
    EFFECTIVE_DATE 1969-02-10

    WHEN HAS service_of_process AND HAS cross_border AND HAS contracting_states
    THEN OBLIGATION "Service through Central Authority; local methods may be used if destination state permits"
}

// Hague Child Abduction Convention 1980
STATUTE hague-abduction: "Hague Child Abduction Convention 1980" {
    JURISDICTION "INTL"
    VERSION 1980
    EFFECTIVE_DATE 1983-12-01

    WHEN HAS child_wrongfully_removed AND HAS habitual_residence_contracting_state
    THEN OBLIGATION "Return child to habitual residence unless exceptions apply"

    EXCEPTION WHEN HAS grave_risk_of_harm
    EXCEPTION WHEN HAS child_objects_and_mature
    DISCRETION "Grave risk and child's objection are discretionary defenses"
}

// =============================================================================
// New York Convention (Arbitral Awards)
// =============================================================================

STATUTE nyc-art3: "New York Convention 1958 Art. III - Recognition" {
    JURISDICTION "INTL"
    VERSION 1958
    EFFECTIVE_DATE 1959-06-07

    WHEN HAS foreign_arbitral_award AND HAS nyc_contracting_state
    THEN OBLIGATION "Recognize and enforce arbitral awards made in other contracting states"
}

STATUTE nyc-art5: "New York Convention 1958 Art. V - Refusal Grounds" {
    JURISDICTION "INTL"
    VERSION 1958
    EFFECTIVE_DATE 1959-06-07

    WHEN HAS foreign_arbitral_award AND (
        HAS incapacity_of_party OR
        HAS invalid_arbitration_agreement OR
        HAS no_proper_notice OR
        HAS beyond_scope_of_submission OR
        HAS improper_tribunal_composition
    )
    THEN GRANT "Recognition may be refused on listed grounds"

    DISCRETION "Court has discretion even if ground established"
}

// =============================================================================
// Public Policy Exception (Ordre Public)
// =============================================================================

STATUTE ordre-public: "Public Policy Exception (All Systems)" {
    JURISDICTION "INTL"
    VERSION 1
    EFFECTIVE_DATE 1900-01-01

    WHEN HAS foreign_law_applicable AND HAS manifestly_incompatible_public_policy
    THEN PROHIBITION "Foreign law not applied if manifestly contrary to forum's public policy"

    DISCRETION "Public policy narrowly construed; international vs domestic public policy"
}

// =============================================================================
// Renvoi
// =============================================================================

STATUTE renvoi: "Renvoi Doctrine" {
    JURISDICTION "INTL"
    VERSION 1
    EFFECTIVE_DATE 1900-01-01

    WHEN HAS foreign_law_reference AND HAS foreign_pil_refers_back
    THEN GRANT "Whether to accept renvoi (reference back) depends on forum's approach"

    DISCRETION "Japan generally rejects renvoi; France accepts in some areas"
}
"#;

fn create_pil_scenario(name: &str, attrs: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
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
    println!("   PRIVATE INTERNATIONAL LAW / CONFLICT OF LAWS");
    println!("   国際私法 - Cross-Border Legal Analysis with Legalis-RS");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(PIL_STATUTES)?;
    println!(
        "Step 1: Parsed {} PIL rules across jurisdictions\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Consistency check {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Analyzing cross-border scenarios...\n");

    let scenarios = vec![
        (
            "Contract with Choice of Law Clause",
            vec![
                ("contract_dispute", true),
                ("express_choice_of_law", true),
                ("eu_court", true),
            ],
        ),
        (
            "Consumer E-commerce Dispute (EU)",
            vec![
                ("contract_dispute", true),
                ("consumer_contract", true),
                ("eu_court", true),
                ("professional_directed_activity", true),
            ],
        ),
        (
            "Cross-Border Tort (Product Liability)",
            vec![
                ("tort_claim", true),
                ("product_liability_claim", true),
                ("eu_court", true),
            ],
        ),
        (
            "International Marriage (Japan PIL)",
            vec![
                ("marriage_question", true),
                ("substantive_requirements", true),
                ("natural_person", true),
            ],
        ),
        (
            "Cross-Border Succession",
            vec![("succession_question", true)],
        ),
        (
            "Foreign Arbitral Award Enforcement",
            vec![
                ("foreign_arbitral_award", true),
                ("nyc_contracting_state", true),
            ],
        ),
        (
            "Child Abduction Case",
            vec![
                ("child_wrongfully_removed", true),
                ("habitual_residence_contracting_state", true),
            ],
        ),
        (
            "Service of Process Abroad",
            vec![
                ("service_of_process", true),
                ("cross_border", true),
                ("contracting_states", true),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, attrs) in &scenarios {
        let entity = create_pil_scenario(name, attrs);
        println!("   === {} ===", name);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                println!("   [+] {} applies", statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "pil-analyzer".to_string(),
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

    println!("{}", "=".repeat(80));
    println!("   PRIVATE INTERNATIONAL LAW DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Key Connecting Factors:");
    println!("   - Nationality (本国法)");
    println!("   - Habitual Residence (常居所)");
    println!("   - Place of Acting/Result (行為地/結果発生地)");
    println!("   - Party Autonomy (当事者自治)");
    println!("   - Closest Connection (最密接関係地)");
    println!();
    println!("   Major Instruments:");
    println!("   - Japan: 法の適用に関する通則法 (2006)");
    println!("   - EU: Rome I & II Regulations");
    println!("   - Hague Conventions (Service, Child Abduction)");
    println!("   - New York Convention (Arbitration)");
    println!();

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
