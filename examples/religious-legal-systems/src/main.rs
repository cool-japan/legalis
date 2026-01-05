//! Comparative Religious Legal Systems - Academic Analysis
//!
//! This example demonstrates how Legalis-RS can model religious legal systems
//! from a purely academic, comparative law perspective.
//!
//! ## Scope and Approach
//!
//! This module focuses on:
//! - **Procedural structures** (courts, tribunals, appeals)
//! - **Jurisdiction boundaries** (where religious law applies)
//! - **Interface with secular law** (recognition, enforcement)
//! - **Financial instruments** (Islamic finance, widely used globally)
//!
//! This is NOT a doctrinal analysis. We examine these systems as functioning
//! legal orders, similar to how comparative law scholars study them.
//!
//! ## Systems Covered
//!
//! 1. **Canon Law** - Catholic Church procedural law (marriage tribunals)
//! 2. **Islamic Finance** - Sharia-compliant financial instruments
//! 3. **Jewish Personal Status** - Israeli Rabbinical Court jurisdiction
//! 4. **India Personal Law** - Secular codification of religious personal law
//! 5. **Thai Sangha Law** - Buddhist ecclesiastical organization

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_verifier::StatuteVerifier;

/// Religious legal system rules in DSL format (academic/structural focus)
const RELIGIOUS_LAW_STATUTES: &str = r#"
// =============================================================================
// CANON LAW - Procedural Aspects (1983 Code of Canon Law)
// Catholic Church's internal legal system - focus on marriage tribunal process
// =============================================================================

// Canon 1671 - Marriage Case Jurisdiction
STATUTE canon-1671: "Canon 1671 - Marriage Tribunal Jurisdiction" {
    JURISDICTION "CANON"
    VERSION 1983
    EFFECTIVE_DATE 1983-11-27

    WHEN HAS marriage_nullity_case AND HAS catholic_party
    THEN GRANT "Diocesan tribunal has first instance jurisdiction"

    DISCRETION "Forum selection: place of marriage, domicile of respondent, or where most proofs available"
}

// Canon 1673 - Competent Tribunal
STATUTE canon-1673: "Canon 1673 - Competent Forum" {
    JURISDICTION "CANON"
    VERSION 2015
    EFFECTIVE_DATE 2015-12-08

    WHEN HAS marriage_nullity_case AND HAS both_parties_catholic
    THEN GRANT "Tribunal of either party's domicile is competent"

    DISCRETION "Mitis Iudex reforms (2015) simplified procedures"
}

// Canon 1683 - Briefer Process
STATUTE canon-1683: "Canon 1683 - Processus Brevior" {
    JURISDICTION "CANON"
    VERSION 2015
    EFFECTIVE_DATE 2015-12-08

    WHEN HAS marriage_nullity_case AND HAS both_parties_consent AND HAS clear_evidence
    THEN GRANT "Bishop may decide case directly through briefer process"

    DISCRETION "Introduced by Pope Francis in Mitis Iudex Dominus Iesus (2015)"
}

// Canon 1628 - Right of Appeal
STATUTE canon-1628: "Canon 1628 - Appellate Rights" {
    JURISDICTION "CANON"
    VERSION 1983
    EFFECTIVE_DATE 1983-11-27

    WHEN HAS tribunal_decision AND HAS aggrieved_party
    THEN GRANT "Right to appeal to higher tribunal (Metropolitan or Roman Rota)"
}

// =============================================================================
// ISLAMIC FINANCE - Sharia-Compliant Financial Instruments
// Widely used in global banking (Malaysia, UAE, UK, etc.)
// =============================================================================

// Murabaha - Cost-Plus Financing
STATUTE islamic-murabaha: "Murabaha Contract Structure" {
    JURISDICTION "ISLAMIC-FINANCE"
    VERSION 1
    EFFECTIVE_DATE 2000-01-01

    WHEN HAS financing_request AND HAS asset_purchase AND HAS disclosed_cost AND HAS disclosed_markup
    THEN GRANT "Murabaha contract valid: bank purchases asset, sells to client at disclosed markup"

    DISCRETION "Must avoid gharar (uncertainty) and ensure genuine asset transfer"
}

// Sukuk - Islamic Bonds
STATUTE islamic-sukuk: "Sukuk Issuance Requirements" {
    JURISDICTION "ISLAMIC-FINANCE"
    VERSION 1
    EFFECTIVE_DATE 2000-01-01

    WHEN HAS capital_raising AND HAS underlying_asset AND HAS profit_sharing_structure
    THEN GRANT "Sukuk issuance permitted: certificates represent ownership in tangible assets"

    DISCRETION "Must be backed by real assets; pure debt instruments not permitted"
}

// Ijara - Islamic Leasing
STATUTE islamic-ijara: "Ijara Lease Structure" {
    JURISDICTION "ISLAMIC-FINANCE"
    VERSION 1
    EFFECTIVE_DATE 2000-01-01

    WHEN HAS lease_arrangement AND HAS lessor_ownership AND HAS defined_usufruct
    THEN GRANT "Ijara valid: lessor retains ownership, transfers usufruct for rent"
}

// Takaful - Islamic Insurance
STATUTE islamic-takaful: "Takaful Cooperative Insurance" {
    JURISDICTION "ISLAMIC-FINANCE"
    VERSION 1
    EFFECTIVE_DATE 2000-01-01

    WHEN HAS insurance_need AND HAS mutual_contribution AND HAS tabarru_donation
    THEN GRANT "Takaful structure valid: mutual risk-sharing through donations to common pool"

    DISCRETION "Participants share surplus; operator acts as wakeel (agent) or mudarib (manager)"
}

// Riba Prohibition - Interest
STATUTE islamic-riba: "Riba Prohibition in Transactions" {
    JURISDICTION "ISLAMIC-FINANCE"
    VERSION 1
    EFFECTIVE_DATE 600-01-01

    WHEN HAS loan_transaction AND HAS predetermined_interest
    THEN PROHIBITION "Riba (interest/usury) not permitted; use profit-sharing alternatives"

    DISCRETION "Distinction between riba al-fadl and riba al-nasi'a"
}

// =============================================================================
// ISRAEL - Rabbinical Court Jurisdiction
// Personal status matters under religious court jurisdiction
// =============================================================================

// Rabbinical Courts Jurisdiction Law 1953
STATUTE israel-rabbinic-marriage: "Rabbinical Courts - Marriage Jurisdiction" {
    JURISDICTION "IL"
    VERSION 1953
    EFFECTIVE_DATE 1953-01-01

    WHEN HAS marriage_of_jews AND HAS in_israel
    THEN OBLIGATION "Marriage and divorce of Jews in Israel under exclusive Rabbinical Court jurisdiction"

    DISCRETION "Civil marriage not available domestically; foreign marriages recognized"
}

// Rabbinical Courts - Divorce
STATUTE israel-rabbinic-divorce: "Rabbinical Courts - Divorce (Get)" {
    JURISDICTION "IL"
    VERSION 1953
    EFFECTIVE_DATE 1953-01-01

    WHEN HAS jewish_couple AND HAS divorce_request AND HAS in_israel
    THEN OBLIGATION "Divorce requires religious divorce (get) from Rabbinical Court"
}

// Get Refusal Sanctions
STATUTE israel-get-refusal: "Sanctions for Get Refusal" {
    JURISDICTION "IL"
    VERSION 1995
    EFFECTIVE_DATE 1995-01-01

    WHEN HAS get_refusal AND HAS rabbinical_court_order
    THEN GRANT "Civil sanctions available: travel restrictions, professional license suspension"

    DISCRETION "Balances religious law requirements with protection of agunot (chained women)"
}

// =============================================================================
// INDIA - Personal Law System
// Secular state with codified religious personal law
// =============================================================================

// Hindu Marriage Act 1955
STATUTE india-hindu-marriage: "Hindu Marriage Act 1955 - Conditions" {
    JURISDICTION "IN"
    VERSION 1955
    EFFECTIVE_DATE 1955-05-18

    WHEN HAS hindu_marriage AND (
        HAS neither_spouse_existing_marriage AND
        HAS mental_capacity AND
        HAS minimum_age_compliance
    )
    THEN GRANT "Hindu marriage valid under Hindu Marriage Act 1955"

    DISCRETION "Sapinda and prohibited relationship rules apply"
}

// Muslim Personal Law (Shariat) Application Act 1937
STATUTE india-muslim-personal: "Muslim Personal Law Application" {
    JURISDICTION "IN"
    VERSION 1937
    EFFECTIVE_DATE 1937-10-07

    WHEN HAS muslim_party AND HAS personal_matter AND (
        HAS succession_question OR HAS marriage_question OR HAS divorce_question OR HAS maintenance_question
    )
    THEN GRANT "Muslim Personal Law (Shariat) applies to personal matters"

    DISCRETION "State-level variations exist"
}

// Special Marriage Act 1954 - Secular Option
STATUTE india-special-marriage: "Special Marriage Act 1954" {
    JURISDICTION "IN"
    VERSION 1954
    EFFECTIVE_DATE 1954-01-01

    WHEN HAS marriage_request AND HAS secular_option_chosen
    THEN GRANT "Civil marriage under Special Marriage Act available regardless of religion"

    DISCRETION "Provides secular alternative to religious personal law"
}

// =============================================================================
// THAILAND - Sangha Act (Buddhist Ecclesiastical Law)
// Monastic organization and discipline
// =============================================================================

// Sangha Act B.E. 2505 (1962)
STATUTE thai-sangha-structure: "Sangha Act - Organizational Structure" {
    JURISDICTION "TH"
    VERSION 1962
    EFFECTIVE_DATE 1962-12-25

    WHEN HAS buddhist_sangha_matter AND HAS thailand_jurisdiction
    THEN GRANT "Sangha Supreme Council has authority over ecclesiastical affairs"

    DISCRETION "Supreme Patriarch (Sangharaja) heads the Thai Sangha"
}

// Sangha Discipline
STATUTE thai-sangha-discipline: "Sangha Act - Monastic Discipline" {
    JURISDICTION "TH"
    VERSION 1962
    EFFECTIVE_DATE 1962-12-25

    WHEN HAS monk_misconduct AND HAS thailand_jurisdiction
    THEN OBLIGATION "Ecclesiastical courts handle monastic discipline matters"

    DISCRETION "Vinaya (monastic rules) applied through Sangha court system"
}

// Ordination Requirements
STATUTE thai-sangha-ordination: "Sangha Act - Ordination" {
    JURISDICTION "TH"
    VERSION 1962
    EFFECTIVE_DATE 1962-12-25

    WHEN HAS ordination_candidate AND HAS age_20_or_above AND HAS parental_consent AND HAS no_disqualification
    THEN GRANT "May receive full ordination (upasampada) as bhikkhu"

    DISCRETION "Novice ordination (samanera) available from age 7"
}
"#;

fn create_scenario(name: &str, attrs: &[&str]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
    for attr in attrs {
        entity.set_attribute(attr, "true".to_string());
    }
    entity
}

fn check_applicability(entity: &BasicEntity, statute: &Statute) -> bool {
    statute
        .preconditions
        .iter()
        .all(|c| evaluate_condition(entity, c))
}

fn evaluate_condition(entity: &BasicEntity, condition: &Condition) -> bool {
    match condition {
        Condition::HasAttribute { key } => entity.get_attribute(key).is_some(),
        Condition::And(l, r) => evaluate_condition(entity, l) && evaluate_condition(entity, r),
        Condition::Or(l, r) => evaluate_condition(entity, l) || evaluate_condition(entity, r),
        Condition::Not(inner) => !evaluate_condition(entity, inner),
        _ => true,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   COMPARATIVE RELIGIOUS LEGAL SYSTEMS");
    println!("   Academic Analysis of Religious Law as Legal Systems");
    println!("{}", "=".repeat(80));
    println!();
    println!("   NOTE: This is a purely academic/structural analysis.");
    println!("   We examine procedural aspects, jurisdiction, and secular law interfaces.");
    println!("   No doctrinal positions are taken.");
    println!();

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(RELIGIOUS_LAW_STATUTES)?;
    println!(
        "Step 1: Parsed {} religious legal system provisions\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Structural consistency check {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    // Group statutes by jurisdiction for analysis
    println!("Step 3: Jurisdiction Analysis\n");

    let jurisdictions = [
        ("CANON", "Canon Law (Catholic Church)"),
        ("ISLAMIC-FINANCE", "Islamic Finance"),
        ("IL", "Israel (Rabbinical Courts)"),
        ("IN", "India (Personal Law)"),
        ("TH", "Thailand (Sangha Law)"),
    ];

    for (code, name) in &jurisdictions {
        let count = statutes
            .iter()
            .filter(|s| s.jurisdiction.as_deref() == Some(*code))
            .count();
        println!("   {} [{}]: {} provisions", name, code, count);
    }
    println!();

    // Test scenarios
    println!("Step 4: Scenario Analysis\n");

    let scenarios: Vec<(&str, Vec<&str>)> = vec![
        (
            "Catholic Marriage Nullity Case",
            vec![
                "marriage_nullity_case",
                "catholic_party",
                "both_parties_catholic",
            ],
        ),
        (
            "Briefer Process Eligibility",
            vec![
                "marriage_nullity_case",
                "both_parties_consent",
                "clear_evidence",
            ],
        ),
        (
            "Islamic Home Financing (Murabaha)",
            vec![
                "financing_request",
                "asset_purchase",
                "disclosed_cost",
                "disclosed_markup",
            ],
        ),
        (
            "Sukuk Bond Issuance",
            vec![
                "capital_raising",
                "underlying_asset",
                "profit_sharing_structure",
            ],
        ),
        (
            "Takaful Insurance Pool",
            vec!["insurance_need", "mutual_contribution", "tabarru_donation"],
        ),
        (
            "Jewish Divorce in Israel",
            vec!["jewish_couple", "divorce_request", "in_israel"],
        ),
        (
            "Hindu Marriage (India)",
            vec![
                "hindu_marriage",
                "neither_spouse_existing_marriage",
                "mental_capacity",
                "minimum_age_compliance",
            ],
        ),
        (
            "Secular Marriage Option (India)",
            vec!["marriage_request", "secular_option_chosen"],
        ),
        (
            "Thai Buddhist Ordination",
            vec![
                "ordination_candidate",
                "age_20_or_above",
                "parental_consent",
                "no_disqualification",
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, attrs) in &scenarios {
        let entity = create_scenario(name, attrs);
        println!("   === {} ===", name);

        let applicable: Vec<_> = statutes
            .iter()
            .filter(|s| check_applicability(&entity, s))
            .collect();

        for statute in &applicable {
            println!("   [+] {} applies", statute.id);

            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "religious-law-analyzer".to_string(),
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

        if applicable.is_empty() {
            println!("   (no matching provisions)");
        }
        println!();
    }

    // Summary
    println!("{}", "=".repeat(80));
    println!("   COMPARATIVE ANALYSIS COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Key Observations:");
    println!();
    println!("   1. JURISDICTION MODELS:");
    println!("      - Exclusive: Israel (personal status)");
    println!("      - Parallel: India (religious + secular options)");
    println!("      - Internal: Canon Law, Sangha (within religious community)");
    println!("      - Transactional: Islamic Finance (global markets)");
    println!();
    println!("   2. SECULAR LAW INTERFACE:");
    println!("      - Recognition: Foreign marriages (Israel)");
    println!("      - Codification: Hindu Marriage Act (India)");
    println!("      - Enforcement: Get refusal sanctions (Israel)");
    println!("      - Regulation: Islamic finance standards (Malaysia, UAE, UK)");
    println!();
    println!("   3. PROCEDURAL FEATURES:");
    println!("      - Appellate systems (Canon Law, Sangha)");
    println!("      - Alternative dispute resolution");
    println!("      - Specialized tribunals");
    println!();
    println!("   Academic Resources:");
    println!("      - Journal of Law and Religion");
    println!("      - Oxford Handbook of Law and Religion");
    println!("      - AAOIFI Standards (Islamic Finance)");
    println!();

    Ok(())
}
