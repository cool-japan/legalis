//! Historical Soviet Legal System (USSR 1922-1991)
//!
//! This example demonstrates how Legalis-RS can be used for historical
//! and comparative legal research by modeling the Soviet legal system.
//!
//! ## Historical Context
//!
//! Soviet law (Советское право) was a unique legal system that:
//! - Rejected the distinction between public and private law
//! - Emphasized collective ownership over individual rights
//! - Used law as an instrument of socialist transformation
//! - Influenced legal systems across Eastern Europe, Central Asia, and beyond
//!
//! ## Why Model Historical Legal Systems?
//!
//! 1. **Comparative Law Research**: Understanding how different legal philosophies
//!    operationalize concepts like property, contract, and rights
//! 2. **Legal Transplantation Studies**: Many post-Soviet states still have
//!    legal structures influenced by Soviet law
//! 3. **Legal History**: Preserving and analyzing defunct legal systems
//! 4. **Soft ODA Context**: Understanding recipient countries' legal heritage
//!
//! ## Key Soviet Legal Codes Modeled
//!
//! - 1922 Civil Code (Гражданский кодекс РСФСР)
//! - 1926 Criminal Code
//! - 1964 Fundamentals of Civil Legislation (Основы)
//! - 1977 Constitution (Конституция СССР)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

/// Soviet legal statutes in DSL format (historical reconstruction)
const SOVIET_STATUTES: &str = r#"
// =============================================================================
// USSR Constitution 1977 (Конституция СССР 1977)
// Historical legal system - for research purposes only
// =============================================================================

// Article 1 - Socialist State
STATUTE ussr-const-art1: "USSR Constitution 1977 Art. 1 - Socialist State" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS soviet_territory
    THEN GRANT "USSR is a socialist state of the whole people"

    DISCRETION "Definition of 'socialist state' evolved through Party interpretation"
}

// Article 6 - Leading Role of CPSU
STATUTE ussr-const-art6: "USSR Constitution 1977 Art. 6 - CPSU Leading Role" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1990-03-14

    WHEN HAS soviet_territory
    THEN OBLIGATION "CPSU is the leading and guiding force of Soviet society"

    DISCRETION "Removed in 1990 constitutional amendment"
}

// Article 10 - Socialist Ownership
STATUTE ussr-const-art10: "USSR Constitution 1977 Art. 10 - Socialist Property" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS means_of_production
    THEN GRANT "Foundation of economic system is socialist ownership: state and collective-farm-cooperative"
}

// Article 13 - Personal Property
STATUTE ussr-const-art13: "USSR Constitution 1977 Art. 13 - Personal Property" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS soviet_citizen
    THEN GRANT "Right to personal property: earned income, savings, dwelling, household articles, articles of personal use and convenience"

    DISCRETION "Personal property distinguished from 'private property' (means of production)"
}

// Article 40 - Right to Work
STATUTE ussr-const-art40: "USSR Constitution 1977 Art. 40 - Right to Work" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS soviet_citizen AND HAS able_to_work
    THEN GRANT "Guaranteed right to work: choice of trade/profession, job placement, payment according to quantity and quality of work"
}

// Article 41 - Rest and Leisure
STATUTE ussr-const-art41: "USSR Constitution 1977 Art. 41 - Rest and Leisure" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS soviet_worker
    THEN GRANT "Right to rest: 41-hour workweek (workers), 36-hour (hazardous), annual paid leave, rest homes, sanatoria"
}

// Article 42 - Health Protection
STATUTE ussr-const-art42: "USSR Constitution 1977 Art. 42 - Free Healthcare" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS soviet_citizen
    THEN GRANT "Free qualified medical care through state health services"
}

// Article 45 - Education
STATUTE ussr-const-art45: "USSR Constitution 1977 Art. 45 - Free Education" {
    JURISDICTION "SU"
    VERSION 1977
    EFFECTIVE_DATE 1977-10-07
    EXPIRY_DATE 1991-12-26

    WHEN HAS soviet_citizen
    THEN GRANT "Free education at all levels, compulsory secondary education"
}

// =============================================================================
// Civil Code RSFSR 1964 (Гражданский кодекс РСФСР 1964)
// =============================================================================

// Article 1 - Civil Legislation Tasks
STATUTE rsfsr-gk-art1: "RSFSR Civil Code 1964 Art. 1 - Tasks" {
    JURISDICTION "SU-RSFSR"
    VERSION 1964
    EFFECTIVE_DATE 1964-10-01
    EXPIRY_DATE 1995-01-01

    WHEN HAS civil_relations
    THEN OBLIGATION "Civil legislation regulates property and related personal non-property relations for building communism"
}

// Article 92 - State Ownership Objects
STATUTE rsfsr-gk-art92: "RSFSR Civil Code 1964 Art. 92 - State Property" {
    JURISDICTION "SU-RSFSR"
    VERSION 1964
    EFFECTIVE_DATE 1964-10-01
    EXPIRY_DATE 1995-01-01

    WHEN HAS land OR HAS natural_resources OR HAS basic_means_of_production
    THEN GRANT "Exclusive state ownership (cannot be privately owned)"
}

// Article 105 - Personal Ownership Right
STATUTE rsfsr-gk-art105: "RSFSR Civil Code 1964 Art. 105 - Personal Property Scope" {
    JURISDICTION "SU-RSFSR"
    VERSION 1964
    EFFECTIVE_DATE 1964-10-01
    EXPIRY_DATE 1995-01-01

    WHEN HAS soviet_citizen AND HAS personal_property
    THEN GRANT "Personal ownership: labor income, savings, dwelling house, household economy, personal use items"

    DISCRETION "Property cannot be used for unearned income"
}

// Article 106 - Dwelling House Size Limit
STATUTE rsfsr-gk-art106: "RSFSR Civil Code 1964 Art. 106 - House Size Limit" {
    JURISDICTION "SU-RSFSR"
    VERSION 1964
    EFFECTIVE_DATE 1964-10-01
    EXPIRY_DATE 1995-01-01

    WHEN HAS personal_dwelling AND HAS exceeds_60_sqm
    THEN PROHIBITION "Single family cannot own dwelling exceeding 60 square meters living space"
}

// =============================================================================
// Labor Code 1971 (Кодекс законов о труде РСФСР)
// =============================================================================

STATUTE rsfsr-kzot-workweek: "RSFSR Labor Code 1971 - Working Hours" {
    JURISDICTION "SU-RSFSR"
    VERSION 1971
    EFFECTIVE_DATE 1971-04-01
    EXPIRY_DATE 2002-02-01

    WHEN HAS soviet_worker
    THEN GRANT "41-hour workweek (36 hours for hazardous conditions, 24 hours for ages 15-16)"
}

STATUTE rsfsr-kzot-leave: "RSFSR Labor Code 1971 - Annual Leave" {
    JURISDICTION "SU-RSFSR"
    VERSION 1971
    EFFECTIVE_DATE 1971-04-01
    EXPIRY_DATE 2002-02-01

    WHEN HAS soviet_worker
    THEN GRANT "Minimum 15 working days paid annual leave"
}

STATUTE rsfsr-kzot-dismissal: "RSFSR Labor Code 1971 - Dismissal Restrictions" {
    JURISDICTION "SU-RSFSR"
    VERSION 1971
    EFFECTIVE_DATE 1971-04-01
    EXPIRY_DATE 2002-02-01

    WHEN HAS soviet_worker AND HAS dismissal_proposed
    THEN OBLIGATION "Trade union committee consent required for dismissal (except specific grounds)"

    DISCRETION "Strong employment protection - near-impossible to dismiss workers"
}

// =============================================================================
// Criminal Code 1960 (Уголовный кодекс РСФСР 1960)
// =============================================================================

STATUTE rsfsr-uk-anti-soviet: "RSFSR Criminal Code 1960 Art. 70 - Anti-Soviet Agitation" {
    JURISDICTION "SU-RSFSR"
    VERSION 1960
    EFFECTIVE_DATE 1960-10-27
    EXPIRY_DATE 1991-12-26

    WHEN HAS anti_soviet_propaganda OR HAS undermining_soviet_power
    THEN OBLIGATION "Criminal liability: 6 months to 7 years (first offense), 3 to 10 years (repeat)"

    DISCRETION "Broadly interpreted; used against dissidents"
}

STATUTE rsfsr-uk-parasitism: "RSFSR Criminal Code 1960 Art. 209 - Parasitism" {
    JURISDICTION "SU-RSFSR"
    VERSION 1960
    EFFECTIVE_DATE 1960-10-27
    EXPIRY_DATE 1991-04-05

    WHEN HAS able_bodied AND NOT HAS socially_useful_work AND HAS prolonged_unemployment
    THEN OBLIGATION "Criminal liability for 'parasitic way of life' (тунеядство)"

    DISCRETION "Notable use: Brodsky trial 1964"
}

STATUTE rsfsr-uk-speculation: "RSFSR Criminal Code 1960 Art. 154 - Speculation" {
    JURISDICTION "SU-RSFSR"
    VERSION 1960
    EFFECTIVE_DATE 1960-10-27
    EXPIRY_DATE 1991-12-26

    WHEN HAS buying_for_resale AND HAS profit_motive
    THEN PROHIBITION "Criminal liability for speculation (buying for resale at profit)"

    DISCRETION "Private entrepreneurship criminalized"
}

// =============================================================================
// Comparative: Kolkhoz Charter (Примерный устав колхоза 1969)
// =============================================================================

STATUTE ussr-kolkhoz-membership: "Model Kolkhoz Charter 1969 - Membership" {
    JURISDICTION "SU"
    VERSION 1969
    EFFECTIVE_DATE 1969-11-28
    EXPIRY_DATE 1991-12-26

    WHEN AGE >= 16 AND HAS kolkhoz_application AND HAS general_meeting_approval
    THEN GRANT "Kolkhoz membership with rights and obligations"
}

STATUTE ussr-kolkhoz-private-plot: "Model Kolkhoz Charter 1969 - Private Plot" {
    JURISDICTION "SU"
    VERSION 1969
    EFFECTIVE_DATE 1969-11-28
    EXPIRY_DATE 1991-12-26

    WHEN HAS kolkhoz_household
    THEN GRANT "Private plot up to 0.5 hectares; may keep 1 cow, pigs, sheep, poultry for personal use"
}
"#;

fn create_soviet_scenario(name: &str, year: u32, attrs: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
    entity.set_attribute("year", year.to_string());
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
    println!("   SOVIET LEGAL SYSTEM (1922-1991) - Legalis-RS Historical Analysis");
    println!("   Советское право - For Comparative Legal Research");
    println!("{}\n", "=".repeat(80));

    println!("   DISCLAIMER: This is a HISTORICAL legal system reconstruction");
    println!("   for academic and comparative law research purposes.");
    println!("   The USSR dissolved on December 26, 1991.\n");

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(SOVIET_STATUTES)?;
    println!(
        "Step 1: Parsed {} Soviet legal provisions\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Internal consistency check {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Analyzing historical scenarios...\n");

    let scenarios = vec![
        (
            "Soviet Worker (1980)",
            1980u32,
            vec![
                ("soviet_citizen", true),
                ("soviet_worker", true),
                ("able_to_work", true),
                ("soviet_territory", true),
            ],
        ),
        (
            "Kolkhoz Farmer (1975)",
            1975,
            vec![
                ("soviet_citizen", true),
                ("kolkhoz_household", true),
                ("kolkhoz_application", true),
                ("general_meeting_approval", true),
            ],
        ),
        (
            "Property Owner Scenario",
            1985,
            vec![
                ("soviet_citizen", true),
                ("personal_property", true),
                ("personal_dwelling", true),
            ],
        ),
        (
            "Excess Property (Violation)",
            1970,
            vec![
                ("soviet_citizen", true),
                ("personal_dwelling", true),
                ("exceeds_60_sqm", true),
            ],
        ),
        (
            "Entrepreneur (Illegal)",
            1982,
            vec![
                ("soviet_citizen", true),
                ("buying_for_resale", true),
                ("profit_motive", true),
            ],
        ),
        (
            "Dissident (Criminalized)",
            1978,
            vec![("soviet_citizen", true), ("anti_soviet_propaganda", true)],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, year, attrs) in &scenarios {
        let entity = create_soviet_scenario(name, *year, attrs);
        println!("   === {} (Year: {}) ===", name, year);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                let effect = format!("{:?}", statute.effect.effect_type);
                let symbol = if effect.contains("Prohibition") {
                    "[X] VIOLATION"
                } else {
                    "[+] APPLIES"
                };
                println!("   {} {}", symbol, statute.id);

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "soviet-law-research".to_string(),
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

    println!("Step 4: Running historical simulation...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;
    println!(
        "   Total: {} | Deterministic: {} | Discretionary: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   SOVIET LAW ANALYSIS COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Key Characteristics of Soviet Law:");
    println!("   - Socialist property (state/collective) vs personal property");
    println!("   - Right to work (and OBLIGATION to work - anti-parasitism)");
    println!("   - Free healthcare and education");
    println!("   - Criminalization of private enterprise ('speculation')");
    println!("   - Political crimes (anti-Soviet agitation)");
    println!("   - Strong labor protections (trade union consent for dismissal)");
    println!();
    println!("   Historical Significance:");
    println!("   - Influenced legal systems of 15 Soviet republics");
    println!("   - Model for Eastern Bloc countries (1945-1989)");
    println!("   - Legacy visible in post-Soviet legal transitions");
    println!("   - Important for understanding Soft ODA context in Central Asia");
    println!();
    println!("   Related Countries for Soft ODA:");
    println!("   - Central Asia: Kazakhstan, Uzbekistan, Kyrgyzstan, Tajikistan, Turkmenistan");
    println!("   - Caucasus: Georgia, Armenia, Azerbaijan");
    println!("   - Eastern Europe: Moldova, Ukraine, Belarus");
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
