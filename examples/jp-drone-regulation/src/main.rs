//! Japanese Aviation Act - Drone (Unmanned Aircraft) Regulations
//! 航空法における無人航空機規制
//!
//! This example demonstrates how to model Japan's comprehensive drone
//! regulatory framework using Legalis-RS.
//!
//! ## Regulatory Framework
//!
//! Japan's drone regulations have evolved significantly:
//! - 2015/12: Initial regulations (Aviation Act amendments)
//! - 2022/06: Registration system & Remote ID (100g threshold)
//! - 2022/12: Pilot licensing (技能証明) & aircraft certification (機体認証)
//! - 2025/12/18: Abolition of HP掲載機 and 民間技能認証 simplified application
//!
//! ## 2025年12月18日 審査要領改正
//!
//! Major changes effective December 18, 2025:
//! - **HP掲載無人航空機 廃止**: Homepage-listed aircraft no longer simplify applications
//! - **民間技能認証 廃止**: Private skill certifications no longer simplify applications
//! - **国家資格のみ有効**: Only national licenses (技能証明) simplify applications
//! - **機体認証のみ有効**: Only certified aircraft (型式認証/機体認証) simplify applications
//!
//! ## Key Concepts
//!
//! - **無人航空機 (Unmanned Aircraft)**: Drones, UAVs weighing 100g+
//! - **特定飛行 (Specific Flight)**: Flights requiring permission/approval
//! - **技能証明 (Pilot License)**: 一等/二等無人航空機操縦士 (国家資格)
//! - **機体認証 (Aircraft Certification)**: 第一種/第二種
//! - **飛行カテゴリー**: Category I, II, III (Level 4)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

/// Japanese drone regulations in DSL format
const DRONE_STATUTES: &str = r#"
// =============================================================================
// 航空法 - 無人航空機の定義・登録
// Aviation Act - Unmanned Aircraft Definition & Registration
// =============================================================================

// 航空法第2条第22項 - 無人航空機の定義
STATUTE jp-aviation-art2-22: "航空法第2条第22項 - 無人航空機の定義" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS aircraft AND HAS unmanned AND HAS remote_or_auto_control AND weight >= 100
    THEN GRANT "Classified as 無人航空機 (Unmanned Aircraft) under Aviation Act"

    DISCRETION "Weight measured without fuel/battery for some aircraft types"
}

// 航空法第131条の3 - 登録義務
STATUTE jp-aviation-art131-3: "航空法第131条の3 - 無人航空機登録義務" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS unmanned_aircraft AND weight >= 100
    THEN OBLIGATION "Must register with MLIT (国土交通省) before flight"
}

// 航空法第131条の4 - 登録記号の表示
STATUTE jp-aviation-art131-4: "航空法第131条の4 - 登録記号表示義務" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS registered_aircraft
    THEN OBLIGATION "Must display registration number (JU + 10-digit alphanumeric)"
}

// 航空法第131条の6 - リモートID
STATUTE jp-aviation-art131-6: "航空法第131条の6 - リモートID機能" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS registered_aircraft AND NOT HAS remote_id_exemption
    THEN OBLIGATION "Must be equipped with Remote ID function transmitting ID and position"

    EXCEPTION WHEN HAS registered_before_20220620
    DISCRETION "Aircraft registered before 2022/6/20 may have 3-year grace period"
}

// =============================================================================
// 航空法第132条の85 - 特定飛行（許可・承認が必要な飛行）
// Specific Flights Requiring Permission/Approval
// =============================================================================

// 空港等周辺の空域
STATUTE jp-aviation-specific-airport: "特定飛行 - 空港等周辺空域" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS flight_in_airport_vicinity
    THEN OBLIGATION "Requires permission from MLIT (空港周辺は許可必要)"
}

// 150m以上の高度
STATUTE jp-aviation-specific-altitude: "特定飛行 - 高度150m以上" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS flight_altitude_over_150m
    THEN OBLIGATION "Requires permission from MLIT (150m以上は許可必要)"
}

// 人口集中地区（DID）上空
STATUTE jp-aviation-specific-did: "特定飛行 - 人口集中地区上空" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS flight_over_did
    THEN OBLIGATION "Requires permission from MLIT (DID上空は許可必要)"

    DISCRETION "DID boundaries based on national census data"
}

// 夜間飛行
STATUTE jp-aviation-specific-night: "特定飛行 - 夜間飛行" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS night_flight
    THEN OBLIGATION "Requires approval from MLIT (夜間飛行は承認必要)"
}

// 目視外飛行（BVLOS）
STATUTE jp-aviation-specific-bvlos: "特定飛行 - 目視外飛行" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS beyond_visual_line_of_sight
    THEN OBLIGATION "Requires approval from MLIT (目視外飛行は承認必要)"
}

// 人・物件との距離30m未満
STATUTE jp-aviation-specific-proximity: "特定飛行 - 人・物件との距離" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS flight_within_30m_of_people_or_property
    THEN OBLIGATION "Requires approval (人・物件から30m未満は承認必要)"
}

// 催し場所上空
STATUTE jp-aviation-specific-event: "特定飛行 - 催し場所上空" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS flight_over_event_venue
    THEN OBLIGATION "Requires approval (催し場所上空は承認必要)"
}

// 危険物輸送
STATUTE jp-aviation-specific-hazmat: "特定飛行 - 危険物輸送" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS transporting_hazardous_materials
    THEN OBLIGATION "Requires approval (危険物輸送は承認必要)"
}

// 物件投下
STATUTE jp-aviation-specific-dropping: "特定飛行 - 物件投下" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS dropping_objects
    THEN OBLIGATION "Requires approval (物件投下は承認必要)"
}

// =============================================================================
// 飛行カテゴリー・技能証明・機体認証
// Flight Categories, Pilot Licensing, Aircraft Certification
// =============================================================================

// カテゴリーI飛行
STATUTE jp-aviation-category1: "飛行カテゴリーI" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS unmanned_aircraft AND NOT HAS specific_flight
    THEN GRANT "Category I: No permission/approval required, follow basic rules"
}

// カテゴリーII飛行
STATUTE jp-aviation-category2: "飛行カテゴリーII" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS specific_flight AND NOT HAS third_party_airspace
    THEN OBLIGATION "Category II: Requires permission/approval or 二等技能証明+第二種機体認証"

    DISCRETION "With 二等技能証明 and 第二種機体認証, permission may be simplified"
}

// カテゴリーIII飛行（レベル4）
STATUTE jp-aviation-category3: "飛行カテゴリーIII（レベル4飛行）" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS specific_flight AND HAS third_party_airspace
    THEN OBLIGATION "Category III (Level 4): Requires 一等技能証明 + 第一種機体認証"

    DISCRETION "Level 4 enables BVLOS over populated areas for delivery, inspection, etc."
}

// 一等無人航空機操縦士
STATUTE jp-aviation-pilot-class1: "一等無人航空機操縦士" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS age_16_or_above AND HAS passed_class1_exam AND HAS no_disqualification
    THEN GRANT "一等無人航空機操縦士技能証明 (Class 1 Pilot License)"

    DISCRETION "Required for Category III (Level 4) flights"
}

// 二等無人航空機操縦士
STATUTE jp-aviation-pilot-class2: "二等無人航空機操縦士" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS age_16_or_above AND HAS passed_class2_exam AND HAS no_disqualification
    THEN GRANT "二等無人航空機操縦士技能証明 (Class 2 Pilot License)"

    DISCRETION "Simplifies approval process for Category II flights"
}

// 第一種機体認証
STATUTE jp-aviation-cert-class1: "第一種機体認証" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS aircraft_inspection_passed AND HAS class1_safety_standards
    THEN GRANT "第一種機体認証 (Class 1 Aircraft Certification)"

    DISCRETION "Required for Category III flights over third parties"
}

// 第二種機体認証
STATUTE jp-aviation-cert-class2: "第二種機体認証" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-12-05

    WHEN HAS aircraft_inspection_passed AND HAS class2_safety_standards
    THEN GRANT "第二種機体認証 (Class 2 Aircraft Certification)"

    DISCRETION "Simplifies approval for Category II flights"
}

// =============================================================================
// 飛行ルール・その他
// Flight Rules & Other Provisions
// =============================================================================

// 飲酒等の禁止
STATUTE jp-aviation-no-alcohol: "航空法第132条の87 - 飲酒等の禁止" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS operating_aircraft AND HAS under_influence
    THEN PROHIBITION "Operating unmanned aircraft while intoxicated is prohibited"
}

// 飛行前確認
STATUTE jp-aviation-preflight: "航空法第132条の88 - 飛行前確認" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS unmanned_aircraft AND HAS planning_flight
    THEN OBLIGATION "Must conduct pre-flight inspection and confirm airspace status"
}

// 衝突予防
STATUTE jp-aviation-collision: "航空法第132条の89 - 衝突予防義務" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS unmanned_aircraft AND HAS in_flight
    THEN OBLIGATION "Must avoid collision with manned aircraft; yield right of way"
}

// 緊急用務空域
STATUTE jp-aviation-emergency: "緊急用務空域での飛行制限" {
    JURISDICTION "JP"
    VERSION 2021
    EFFECTIVE_DATE 2021-06-01

    WHEN HAS flight_in_emergency_airspace
    THEN PROHIBITION "Flight prohibited in emergency operation airspace (災害等の緊急用務空域)"

    DISCRETION "Airspace designated during disasters, accidents, firefighting operations"
}

// 100g未満の機体
STATUTE jp-aviation-under100g: "100g未満の機体" {
    JURISDICTION "JP"
    VERSION 2022
    EFFECTIVE_DATE 2022-06-20

    WHEN HAS aircraft AND HAS unmanned AND weight < 100
    THEN GRANT "Not classified as 無人航空機; registration not required, but airspace rules still apply"

    DISCRETION "Some local ordinances (条例) may impose additional restrictions"
}

// 小型無人機等飛行禁止法（重要施設周辺）
STATUTE jp-drone-security: "小型無人機等飛行禁止法" {
    JURISDICTION "JP"
    VERSION 2016
    EFFECTIVE_DATE 2016-04-07

    WHEN HAS flight_near_important_facility AND (
        HAS near_parliament OR HAS near_pm_residence OR
        HAS near_imperial_palace OR HAS near_embassy OR
        HAS near_nuclear_facility OR HAS near_airport
    )
    THEN PROHIBITION "Flight prohibited near important facilities (対象施設周辺地域)"

    DISCRETION "Separate from Aviation Act; enforced by police"
}

// =============================================================================
// 2025年12月18日 審査要領改正 - 申請手続簡略化要件の変更
// December 18, 2025 Amendment - Changes to Simplified Application Requirements
// =============================================================================

// HP掲載無人航空機制度の廃止
STATUTE jp-aviation-2025-hp-aircraft-abolished: "HP掲載無人航空機制度廃止" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS category2_application AND HAS hp_listed_aircraft_only AND NOT HAS type_certification
    THEN PROHIBITION "HP掲載無人航空機による申請簡略化は廃止 (December 2025)"

    DISCRETION "型式認証機または機体認証機のみが申請簡略化の対象"
}

// 民間技能認証制度の廃止
STATUTE jp-aviation-2025-private-cert-abolished: "民間技能認証制度廃止" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS category2_application AND HAS private_skill_certification AND NOT HAS national_pilot_license
    THEN PROHIBITION "民間技能認証による申請簡略化は廃止 (December 2025)"

    DISCRETION "国家資格（一等・二等技能証明）のみが申請簡略化の対象"
}

// 民間飛行マニュアル制度の廃止
STATUTE jp-aviation-2025-private-manual-abolished: "HP掲載飛行マニュアル制度廃止" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS category2_application AND HAS hp_listed_flight_manual
    THEN PROHIBITION "HP掲載講習団体等の飛行マニュアル使用による簡略化は廃止"

    DISCRETION "独自の飛行マニュアル作成が必要"
}

// 2025年12月18日以降の申請簡略化要件（機体）
STATUTE jp-aviation-2025-aircraft-simplification: "申請簡略化要件（機体）2025年12月以降" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS category2_application AND (HAS type_certification OR HAS aircraft_certification)
    THEN GRANT "型式認証機または機体認証機は申請書類の一部省略可能"

    DISCRETION "機体に関する資料の提出省略"
}

// 2025年12月18日以降の申請簡略化要件（操縦者）
STATUTE jp-aviation-2025-pilot-simplification: "申請簡略化要件（操縦者）2025年12月以降" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS category2_application AND HAS national_pilot_license
    THEN GRANT "国家資格（技能証明）保有者は申請書類の一部省略可能"

    DISCRETION "操縦者に関する資料の提出省略"
}

// 既存許可の取扱い（2025年12月18日以降）
STATUTE jp-aviation-2025-existing-permits: "既存許可の取扱い（2025年12月以降）" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS existing_permit_with_hp_aircraft OR HAS existing_permit_with_private_cert
    THEN OBLIGATION "複製・変更・更新申請不可、新規申請が必要"

    DISCRETION "DIPS 2.0での複製機能等が使用不可となる"
}

// カテゴリーII飛行の許可・承認（2025年12月改正後）
STATUTE jp-aviation-category2-2025: "カテゴリーII飛行（2025年12月改正後）" {
    JURISDICTION "JP"
    VERSION 2025
    EFFECTIVE_DATE 2025-12-18

    WHEN HAS specific_flight AND NOT HAS third_party_airspace
    THEN OBLIGATION "Category II: 許可・承認が必要。国家資格+機体認証で申請簡略化"

    DISCRETION "民間資格のみでは申請簡略化不可（2025年12月18日以降）"
}
"#;

fn create_drone_scenario(name: &str, attrs: &[(&str, &str)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
    for (key, value) in attrs {
        entity.set_attribute(key, value.to_string());
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
        Condition::Age { operator, value } => entity
            .get_attribute("weight")
            .and_then(|s| s.parse::<u32>().ok())
            .map(|w| match operator {
                ComparisonOp::GreaterOrEqual => w >= *value,
                ComparisonOp::LessThan => w < *value,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   JAPANESE AVIATION ACT - DRONE REGULATIONS");
    println!("   航空法 無人航空機規制 - Legalis-RS Analysis");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Regulatory Timeline:");
    println!("   - 2015/12: Initial drone regulations enacted");
    println!("   - 2022/06: Registration system + Remote ID (100g threshold)");
    println!("   - 2022/12: Pilot licensing (技能証明) + Aircraft certification (機体認証)");
    println!("   - 2025/12/18: HP掲載機・民間技能認証による申請簡略化廃止");
    println!();
    println!("   [!] 2025年12月18日 審査要領改正:");
    println!("   - HP掲載無人航空機 → 廃止（型式認証/機体認証のみ有効）");
    println!("   - 民間技能認証 → 廃止（国家資格のみ有効）");
    println!("   - 既存許可の複製・変更・更新 → 新規申請必要");
    println!();

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(DRONE_STATUTES)?;
    println!(
        "Step 1: Parsed {} drone regulation provisions\n",
        statutes.len()
    );

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Step 2: Consistency check {}\n",
        if result.passed { "PASSED" } else { "FAILED" }
    );

    println!("Step 3: Analyzing drone flight scenarios...\n");

    let scenarios: Vec<(&str, Vec<(&str, &str)>)> = vec![
        (
            "Hobby Drone (250g) - Park Flight",
            vec![
                ("aircraft", "true"),
                ("unmanned", "true"),
                ("remote_or_auto_control", "true"),
                ("weight", "250"),
                ("unmanned_aircraft", "true"),
                ("registered_aircraft", "true"),
                ("planning_flight", "true"),
            ],
        ),
        (
            "Commercial Drone - DID Area Inspection",
            vec![
                ("unmanned_aircraft", "true"),
                ("weight", "2000"),
                ("registered_aircraft", "true"),
                ("specific_flight", "true"),
                ("flight_over_did", "true"),
                ("planning_flight", "true"),
                ("in_flight", "true"),
            ],
        ),
        (
            "Delivery Drone - Level 4 (BVLOS over city)",
            vec![
                ("unmanned_aircraft", "true"),
                ("weight", "5000"),
                ("registered_aircraft", "true"),
                ("specific_flight", "true"),
                ("third_party_airspace", "true"),
                ("beyond_visual_line_of_sight", "true"),
                ("flight_over_did", "true"),
            ],
        ),
        (
            "Night Photography - Event Coverage",
            vec![
                ("unmanned_aircraft", "true"),
                ("weight", "1500"),
                ("registered_aircraft", "true"),
                ("specific_flight", "true"),
                ("night_flight", "true"),
                ("flight_over_event_venue", "true"),
            ],
        ),
        (
            "Agricultural Drone - Pesticide Spraying",
            vec![
                ("unmanned_aircraft", "true"),
                ("weight", "10000"),
                ("registered_aircraft", "true"),
                ("specific_flight", "true"),
                ("dropping_objects", "true"),
                ("transporting_hazardous_materials", "true"),
            ],
        ),
        (
            "Toy Drone (80g) - Backyard",
            vec![
                ("aircraft", "true"),
                ("unmanned", "true"),
                ("weight", "80"),
                ("planning_flight", "true"),
            ],
        ),
        (
            "Class 1 Pilot Certification",
            vec![
                ("age_16_or_above", "true"),
                ("passed_class1_exam", "true"),
                ("no_disqualification", "true"),
            ],
        ),
        (
            "Class 1 Aircraft Certification",
            vec![
                ("aircraft_inspection_passed", "true"),
                ("class1_safety_standards", "true"),
            ],
        ),
        (
            "Flight Near Airport",
            vec![
                ("unmanned_aircraft", "true"),
                ("registered_aircraft", "true"),
                ("flight_in_airport_vicinity", "true"),
            ],
        ),
        (
            "Flight Near Parliament (Security Zone)",
            vec![
                ("unmanned_aircraft", "true"),
                ("flight_near_important_facility", "true"),
                ("near_parliament", "true"),
            ],
        ),
        // 2025年12月改正関連シナリオ
        (
            "[2025改正] 民間資格のみ保有者 - DID飛行申請",
            vec![
                ("category2_application", "true"),
                ("private_skill_certification", "true"),
                ("hp_listed_aircraft_only", "true"),
                ("flight_over_did", "true"),
            ],
        ),
        (
            "[2025改正] 国家資格+機体認証保有者 - DID飛行申請",
            vec![
                ("category2_application", "true"),
                ("national_pilot_license", "true"),
                ("aircraft_certification", "true"),
                ("flight_over_did", "true"),
            ],
        ),
        (
            "[2025改正] 既存許可（HP掲載機使用）の更新",
            vec![
                ("existing_permit_with_hp_aircraft", "true"),
                ("category2_application", "true"),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, attrs) in &scenarios {
        let entity = create_drone_scenario(name, attrs);
        println!("   === {} ===", name);

        let applicable: Vec<_> = statutes
            .iter()
            .filter(|s| check_applicability(&entity, s))
            .collect();

        for statute in &applicable {
            let effect_type = match statute.effect.effect_type {
                legalis_core::EffectType::Grant => "[GRANT]",
                legalis_core::EffectType::Prohibition => "[PROHIBITION]",
                legalis_core::EffectType::Obligation => "[OBLIGATION]",
                _ => "[EFFECT]",
            };
            println!("   {} {}: {}", effect_type, statute.id, statute.title);

            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "drone-regulation-analyzer".to_string(),
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
            println!("   (no specific regulations apply)");
        }
        println!();
    }

    // Population simulation
    println!("{}", "-".repeat(80));
    println!("Step 4: Population Simulation - Drone Operators\n");

    let population = PopulationBuilder::new().generate_random(500).build();
    let sim_statutes: Vec<_> = statutes.iter().take(5).cloned().collect();
    let engine = SimEngine::new(sim_statutes, population);
    let metrics = engine.run_simulation().await;
    println!("{}", metrics.summary());

    // Summary
    println!("{}", "=".repeat(80));
    println!("   JAPANESE DRONE REGULATION SUMMARY");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Weight Classifications:");
    println!("   - Under 100g: Not 無人航空機, no registration (some rules still apply)");
    println!("   - 100g+: 無人航空機, registration + Remote ID required");
    println!();
    println!("   Flight Categories:");
    println!("   - Category I: Non-specific flights (basic rules only)");
    println!("   - Category II: Specific flights (permission/approval needed)");
    println!("   - Category III: Level 4 (一等技能証明 + 第一種機体認証 required)");
    println!();
    println!("   Specific Flight Triggers (特定飛行):");
    println!("   - Airport vicinity, altitude >150m, DID areas");
    println!("   - Night flight, BVLOS, proximity <30m");
    println!("   - Events, hazmat, object dropping");
    println!();
    println!("   Licensing System (2022/12~):");
    println!("   - 一等無人航空機操縦士: Required for Level 4");
    println!("   - 二等無人航空機操縦士: Simplifies Category II approval");
    println!();
    println!("   [!] 2025年12月18日改正 - 申請簡略化要件:");
    println!("   ┌─────────────────────────────────────────────────────────┐");
    println!("   │ 廃止される制度:                                        │");
    println!("   │   - HP掲載無人航空機による申請簡略化                   │");
    println!("   │   - 民間技能認証による申請簡略化                       │");
    println!("   │   - HP掲載飛行マニュアルの使用                         │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    println!("   │ 有効な申請簡略化要件:                                  │");
    println!("   │   - 機体: 型式認証機 or 機体認証機                     │");
    println!("   │   - 操縦者: 国家資格（一等/二等技能証明）              │");
    println!("   └─────────────────────────────────────────────────────────┘");
    println!();
    println!("   Key Authorities:");
    println!("   - 国土交通省 (MLIT): Registration, permissions, approvals");
    println!("   - DIPS 2.0: Online system for flight plans and approvals");
    println!();
    println!("   References:");
    println!("   - https://www.mlit.go.jp/koku/koku_tk10_000003.html");
    println!("   - https://www.mlit.go.jp/koku/koku_fr10_000042.html");
    println!();

    Ok(())
}
