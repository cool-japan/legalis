//! Laos Civil Code 2020 - Japan Soft ODA Case Study
//! ປະມວນກົດໝາຍແພ່ງ ສປປ ລາວ
//!
//! This example demonstrates Legalis-RS applied to a real-world legal
//! technical assistance (Soft ODA) success story: Japan's support for
//! Laos's first comprehensive Civil Code.
//!
//! ## Historical Background
//!
//! - **Before 2020**: Laos had fragmented civil legislation inherited from
//!   French colonial period and socialist era
//! - **1996-2020**: Japan's Ministry of Justice (MOJ) provided technical
//!   assistance through JICA for drafting a comprehensive Civil Code
//! - **May 2020**: Laos National Assembly adopted the Civil Code
//! - **December 2020**: Civil Code entered into force
//!
//! ## Japan's Soft ODA Approach
//!
//! Japan's legal technical assistance (法整備支援) involves:
//! 1. Respecting local legal traditions and culture
//! 2. Comparative law analysis (not transplanting Japanese law directly)
//! 3. Long-term capacity building for local legal professionals
//! 4. Supporting drafting while Lao counterparts make final decisions
//!
//! ## The Laos Civil Code Structure
//!
//! Influenced by both Japanese and German civil law traditions:
//! - Part I: General Provisions (総則 / ພາກທົ່ວໄປ)
//! - Part II: Persons (人 / ບຸກຄົນ)
//! - Part III: Things (物 / ຊັບ)
//! - Part IV: Obligations (債権 / ໜີ້)
//! - Part V: Real Rights (物権 / ກຳມະສິດ)
//! - Part VI: Family (家族 / ຄອບຄົວ)
//! - Part VII: Inheritance (相続 / ມູນມໍລະດົກ)
//!
//! ## Reference
//!
//! Japanese Ministry of Justice: <https://www.moj.go.jp/content/001321546.pdf>

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

/// Laos Civil Code 2020 statutes in DSL format
const LAOS_CC_STATUTES: &str = r#"
// =============================================================================
// Laos Civil Code 2020 (ປະມວນກົດໝາຍແພ່ງ)
// Effective: December 2020
// Japan Soft ODA Technical Assistance Case Study
// =============================================================================

// =============================================================================
// Part I: General Provisions (ພາກທົ່ວໄປ)
// =============================================================================

// Article 1 - Purpose of Civil Code
STATUTE la-cc-art1: "Laos CC Art. 1 - Purpose" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS civil_matter
    THEN GRANT "Civil Code regulates civil relations, protects rights and legitimate interests of natural and legal persons"
}

// Article 6 - Good Faith Principle
STATUTE la-cc-art6: "Laos CC Art. 6 - Good Faith (ຄວາມສຸດຈະລິດ)" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS civil_relations
    THEN OBLIGATION "Exercise rights and perform obligations in good faith"

    DISCRETION "Good faith interpretation follows general principles of civil law"
}

// Article 7 - Prohibition of Abuse of Rights
STATUTE la-cc-art7: "Laos CC Art. 7 - Abuse of Rights Prohibition" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS exercise_of_right AND HAS sole_purpose_to_harm_others
    THEN PROHIBITION "Exercise of right solely to harm another is prohibited"
}

// =============================================================================
// Part II: Persons (ບຸກຄົນ)
// =============================================================================

// Article 12 - Legal Capacity of Natural Persons
STATUTE la-cc-art12: "Laos CC Art. 12 - Legal Capacity" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS natural_person
    THEN GRANT "Capacity to have rights and obligations from birth to death"
}

// Article 16 - Capacity to Act (18 years)
STATUTE la-cc-art16: "Laos CC Art. 16 - Capacity to Act" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN AGE >= 18 AND HAS natural_person AND HAS sound_mind
    THEN GRANT "Full capacity to perform juridical acts"
}

// Article 17 - Limited Capacity (15-17 years)
STATUTE la-cc-art17: "Laos CC Art. 17 - Limited Capacity" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN AGE >= 15 AND AGE < 18 AND HAS natural_person
    THEN GRANT "Limited capacity - may perform acts appropriate to age with guardian consent for major transactions"
}

// Article 22 - Legal Persons
STATUTE la-cc-art22: "Laos CC Art. 22 - Legal Persons" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS legal_person_registration
    THEN GRANT "Legal person has capacity for rights and obligations within scope of law and charter"
}

// =============================================================================
// Part IV: Obligations (ໜີ້)
// =============================================================================

// Article 201 - Definition of Obligation
STATUTE la-cc-art201: "Laos CC Art. 201 - Obligation Definition" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS creditor_debtor_relationship
    THEN GRANT "Obligation is legal relation where debtor must perform specific conduct for creditor"
}

// Article 215 - Contract Formation
STATUTE la-cc-art215: "Laos CC Art. 215 - Contract Formation" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS offer AND HAS acceptance AND HAS meeting_of_minds
    THEN GRANT "Contract formed when offer and acceptance correspond"
}

// Article 225 - Contract Void for Illegality
STATUTE la-cc-art225: "Laos CC Art. 225 - Illegal Contract" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS contract AND HAS illegal_object_or_cause
    THEN PROHIBITION "Contract void if object or cause violates law or public order"
}

// Article 256 - Non-Performance of Obligation
STATUTE la-cc-art256: "Laos CC Art. 256 - Non-Performance" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS non_performance AND HAS attributable_to_debtor
    THEN OBLIGATION "Debtor liable for damages caused by non-performance"

    EXCEPTION WHEN HAS force_majeure
}

// Article 289 - Tort Liability
STATUTE la-cc-art289: "Laos CC Art. 289 - Tort (ການລະເມີດ)" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS unlawful_act AND HAS fault AND HAS damage AND HAS causation
    THEN OBLIGATION "Person who causes damage by unlawful act must compensate"

    DISCRETION "Fault includes intent and negligence"
}

// =============================================================================
// Part V: Real Rights (ກຳມະສິດ)
// =============================================================================

// Article 310 - Ownership Definition
STATUTE la-cc-art310: "Laos CC Art. 310 - Ownership" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS owner_status
    THEN GRANT "Owner has right to possess, use, enjoy fruits, and dispose of property within legal limits"
}

// Article 336 - Co-ownership
STATUTE la-cc-art336: "Laos CC Art. 336 - Co-ownership" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS co_ownership
    THEN GRANT "Each co-owner may use entire thing; major decisions require majority; disposal requires unanimity"
}

// Note: Land ownership in Laos is complex due to socialist land tenure system
STATUTE la-cc-land: "Laos CC - Land Use Rights" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS land_use_right
    THEN GRANT "Land use rights recognized; land remains state property per Constitution"

    DISCRETION "Harmonization with Land Law and Constitution required"
}

// =============================================================================
// Part VI: Family (ຄອບຄົວ)
// =============================================================================

// Article 401 - Marriage Requirements
STATUTE la-cc-art401: "Laos CC Art. 401 - Marriage Age" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN AGE >= 18 AND HAS consent_to_marry
    THEN GRANT "May marry at age 18 with free consent"

    EXCEPTION WHEN AGE >= 15 AND HAS parental_consent AND HAS exceptional_circumstances
}

// Article 410 - Marriage Property Regime
STATUTE la-cc-art410: "Laos CC Art. 410 - Marital Property" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS married_couple
    THEN GRANT "Community property regime applies unless parties agree otherwise"
}

// Article 432 - Divorce Grounds
STATUTE la-cc-art432: "Laos CC Art. 432 - Divorce" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS marriage AND (HAS mutual_consent OR HAS breakdown_of_marriage)
    THEN GRANT "Divorce by mutual consent or court judgment on grounds of irretrievable breakdown"

    DISCRETION "Court assesses breakdown based on specific grounds listed in law"
}

// =============================================================================
// Part VII: Inheritance (ມູນມໍລະດົກ)
// =============================================================================

// Article 501 - Inheritance Opening
STATUTE la-cc-art501: "Laos CC Art. 501 - Inheritance Opens at Death" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS death_of_person
    THEN GRANT "Inheritance opens at moment of death at decedent's last domicile"
}

// Article 510 - Intestate Succession Order
STATUTE la-cc-art510: "Laos CC Art. 510 - Succession Order" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS intestate_succession
    THEN GRANT "Order: (1) descendants, (2) parents, (3) siblings, (4) grandparents, (5) further ascendants"
}

// Article 530 - Will Requirements
STATUTE la-cc-art530: "Laos CC Art. 530 - Will" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN AGE >= 18 AND HAS sound_mind AND HAS will_document
    THEN GRANT "Person may dispose of property by will within legal limits"
}

// Article 540 - Forced Heirship
STATUTE la-cc-art540: "Laos CC Art. 540 - Legitime (Reserved Portion)" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS forced_heir AND HAS will_disposing_over_reserved
    THEN GRANT "Forced heirs (descendants, spouse) entitled to reserved portion"

    DISCRETION "Reserved portion calculation varies; may be reduced in exceptional cases"
}

// =============================================================================
// Transition Provisions
// =============================================================================

STATUTE la-cc-transition: "Laos CC - Transition" {
    JURISDICTION "LA"
    VERSION 2020
    EFFECTIVE_DATE 2020-12-01

    WHEN HAS existing_civil_relation AND HAS before_civil_code
    THEN GRANT "Existing relations continue; Civil Code applies to new matters from effective date"

    DISCRETION "Prior laws remain applicable for matters arising before December 2020"
}
"#;

fn create_laos_scenario(name: &str, age: u32, attrs: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
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
    println!("   LAOS CIVIL CODE 2020 - Japan Soft ODA Case Study");
    println!("   ປະມວນກົດໝາຍແພ່ງ ສປປ ລາວ");
    println!("   Japan Legal Technical Assistance Success Story");
    println!("{}\n", "=".repeat(80));

    println!("   Historical Background:");
    println!("   - 1996-2020: Japan MOJ/JICA technical assistance");
    println!("   - May 2020: Adopted by Lao National Assembly");
    println!("   - December 2020: Entered into force");
    println!("   - First comprehensive Civil Code in Laos history\n");

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(LAOS_CC_STATUTES)?;
    println!(
        "Step 1: Parsed {} Laos Civil Code provisions\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Internal consistency {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Evaluating civil law scenarios...\n");

    let scenarios = vec![
        (
            "Adult with Full Capacity",
            25u32,
            vec![
                ("natural_person", true),
                ("sound_mind", true),
                ("civil_matter", true),
            ],
        ),
        (
            "Minor with Limited Capacity (16 years)",
            16,
            vec![("natural_person", true), ("sound_mind", true)],
        ),
        (
            "Contract Formation",
            30,
            vec![
                ("natural_person", true),
                ("offer", true),
                ("acceptance", true),
                ("meeting_of_minds", true),
            ],
        ),
        (
            "Tort Case (Traffic Accident)",
            35,
            vec![
                ("natural_person", true),
                ("unlawful_act", true),
                ("fault", true),
                ("damage", true),
                ("causation", true),
            ],
        ),
        (
            "Marriage Application",
            20,
            vec![("natural_person", true), ("consent_to_marry", true)],
        ),
        (
            "Inheritance Case",
            50,
            vec![("death_of_person", true), ("intestate_succession", true)],
        ),
        (
            "Land Use Rights",
            40,
            vec![("natural_person", true), ("land_use_right", true)],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, age, attrs) in &scenarios {
        let entity = create_laos_scenario(name, *age, attrs);
        println!("   === {} (Age: {}) ===", name, age);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                println!("   [+] {}", statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "laos-cc".to_string(),
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
        "   Total: {} | Deterministic: {} | Judicial Discretion: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   LAOS CIVIL CODE DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Laos Civil Code Structure:");
    println!("   - Part I: General Provisions (ພາກທົ່ວໄປ)");
    println!("   - Part II: Persons (ບຸກຄົນ)");
    println!("   - Part III: Things (ຊັບ)");
    println!("   - Part IV: Obligations (ໜີ້)");
    println!("   - Part V: Real Rights (ກຳມະສິດ)");
    println!("   - Part VI: Family (ຄອບຄົວ)");
    println!("   - Part VII: Inheritance (ມູນມໍລະດົກ)");
    println!();
    println!("   Japan Soft ODA Principles:");
    println!("   1. Respect for local legal traditions");
    println!("   2. Comparative law approach (not transplantation)");
    println!("   3. Long-term capacity building");
    println!("   4. Local ownership of final decisions");
    println!();
    println!("   Other Japan Legal Assistance Recipients:");
    println!("   - Vietnam (Civil Code 2015)");
    println!("   - Cambodia (Civil Code 2007)");
    println!("   - Mongolia, Indonesia, Myanmar, etc.");
    println!();
    println!("   Reference: https://www.moj.go.jp/content/001321546.pdf");
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
                ComparisonOp::GreaterThan => age > *value,
                ComparisonOp::LessOrEqual => age <= *value,
                ComparisonOp::LessThan => age < *value,
                ComparisonOp::Equal => age == *value,
                ComparisonOp::NotEqual => age != *value,
            })
            .unwrap_or(false),
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => evaluate_condition(entity, l) && evaluate_condition(entity, r),
        Condition::Or(l, r) => evaluate_condition(entity, l) || evaluate_condition(entity, r),
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true,
    }
}
