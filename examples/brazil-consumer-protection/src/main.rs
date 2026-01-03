//! Brazilian Consumer Defense Code (Codigo de Defesa do Consumidor - CDC)
//!
//! This example demonstrates how to use Legalis-RS for Brazilian consumer
//! protection law. It covers Lei 8.078/1990 (CDC), one of the most
//! comprehensive consumer protection laws in the world.
//!
//! ## CDC Overview
//!
//! The Brazilian Consumer Defense Code (CDC) establishes:
//! - Consumer as "hypersufficient" party deserving protection
//! - Strict liability for product/service defects
//! - Right of withdrawal (7 days for remote purchases)
//! - Abusive clause prohibition
//! - Collective consumer rights
//!
//! ## Key Articles Modeled
//!
//! - Art. 6: Basic consumer rights
//! - Art. 12-14: Product/service liability
//! - Art. 18-20: Product/service defects
//! - Art. 30-35: Offer binding rules
//! - Art. 39: Abusive practices
//! - Art. 49: Right of withdrawal
//! - Art. 51: Abusive clauses

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;
use legalis_viz::DecisionTree;

/// Brazilian CDC statutes in DSL format
const CDC_STATUTES: &str = r#"
// =============================================================================
// CDC - Lei 8.078/1990 - Consumer Defense Code (Brazil)
// Codigo de Defesa do Consumidor
// =============================================================================

// Article 6 - Basic Consumer Rights
STATUTE cdc-art6-basic-rights: "CDC Art. 6 - Direitos Basicos do Consumidor" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS consumer_status
    THEN GRANT "Basic consumer rights: life protection, education, information, fair advertising, contract modification, effective prevention and compensation"
}

// =============================================================================
// Product Liability (Arts. 12-14)
// =============================================================================

// Article 12 - Manufacturer/Producer Liability
STATUTE cdc-art12-product-liability: "CDC Art. 12 - Responsabilidade pelo Fato do Produto" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS product_defect AND HAS damage_occurred AND HAS consumer_status
    THEN OBLIGATION "Manufacturer, producer, constructor, importer liable for damages"

    DISCRETION "Defect existence and causation determination by expert evaluation"
}

// Article 14 - Service Provider Liability
STATUTE cdc-art14-service-liability: "CDC Art. 14 - Responsabilidade pelo Fato do Servico" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS service_defect AND HAS damage_occurred AND HAS consumer_status
    THEN OBLIGATION "Service provider liable for damages caused by defective services"

    EXCEPTION WHEN HAS liberal_professional
    DISCRETION "Liberal professionals (doctors, lawyers) require proof of fault"
}

// =============================================================================
// Product/Service Quality (Arts. 18-20)
// =============================================================================

// Article 18 - Product Quality Defects
STATUTE cdc-art18-product-quality: "CDC Art. 18 - Vicio do Produto" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS product_quality_defect AND HAS consumer_status AND NOT HAS defect_repaired_30_days
    THEN GRANT "Right to: replacement, refund, or proportional price reduction"

    DISCRETION "30-day repair period may be extended to 180 days by agreement"
}

// Article 20 - Service Quality Defects
STATUTE cdc-art20-service-quality: "CDC Art. 20 - Vicio do Servico" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS service_quality_defect AND HAS consumer_status
    THEN GRANT "Right to: service re-execution, refund, or proportional price reduction"
}

// =============================================================================
// Binding Offer (Arts. 30-35)
// =============================================================================

// Article 30 - Binding Advertising
STATUTE cdc-art30-binding-offer: "CDC Art. 30 - Oferta Vinculante" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS public_offer_made AND HAS consumer_status
    THEN OBLIGATION "Supplier bound by sufficiently precise advertising/offer terms"
}

// Article 35 - Non-compliance with Offer
STATUTE cdc-art35-offer-noncompliance: "CDC Art. 35 - Descumprimento da Oferta" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS offer_not_honored AND HAS consumer_status
    THEN GRANT "Consumer may demand: forced compliance, equivalent product/service, or contract rescission with damages"
}

// =============================================================================
// Abusive Practices (Art. 39)
// =============================================================================

STATUTE cdc-art39-tied-selling: "CDC Art. 39 I - Venda Casada Proibida" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS tied_selling_attempted
    THEN PROHIBITION "Conditioning supply on purchase of other product/service (venda casada)"
}

STATUTE cdc-art39-refusal-demand: "CDC Art. 39 II - Recusa de Atendimento" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS demand_refused_without_cause AND HAS stock_available
    THEN PROHIBITION "Refusing to meet consumer demand when able"
}

STATUTE cdc-art39-unsolicited-products: "CDC Art. 39 III - Produtos Nao Solicitados" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS unsolicited_product_sent
    THEN GRANT "Unsolicited products are treated as free samples - no payment obligation"
}

STATUTE cdc-art39-vulnerability: "CDC Art. 39 IV - Exploracao de Vulnerabilidade" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS vulnerability_exploited AND (HAS elderly_consumer OR HAS child_consumer OR HAS health_condition)
    THEN PROHIBITION "Exploiting consumer weakness, age, health, knowledge, or social condition"

    DISCRETION "Determining exploitation requires case-by-case assessment"
}

// =============================================================================
// Right of Withdrawal (Art. 49)
// =============================================================================

STATUTE cdc-art49-withdrawal: "CDC Art. 49 - Direito de Arrependimento" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS remote_purchase AND HAS within_7_days AND HAS consumer_status
    THEN GRANT "Right to withdraw from contract within 7 days of receipt (direito de arrependimento)"
}

// =============================================================================
// Abusive Clauses (Art. 51)
// =============================================================================

STATUTE cdc-art51-liability-exclusion: "CDC Art. 51 I - Clausula de Exclusao de Responsabilidade" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS liability_exclusion_clause AND HAS consumer_contract
    THEN PROHIBITION "Clauses excluding/reducing supplier liability are null and void"

    EXCEPTION WHEN HAS justified_situation AND HAS negotiable_contract
}

STATUTE cdc-art51-unilateral-termination: "CDC Art. 51 XI - Rescisao Unilateral" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS unilateral_termination_clause AND HAS consumer_contract AND NOT HAS consumer_right_to_terminate
    THEN PROHIBITION "Unilateral termination right for supplier only is null"
}

STATUTE cdc-art51-arbitration: "CDC Art. 51 VII - Arbitragem Compulsoria" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 1990-09-11

    WHEN HAS mandatory_arbitration_clause AND HAS consumer_contract
    THEN PROHIBITION "Mandatory arbitration clauses are null in consumer contracts"
}

// =============================================================================
// Special Protection - Elderly Consumers
// =============================================================================

STATUTE cdc-elderly-priority: "CDC + Estatuto do Idoso - Atendimento Prioritario" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 2003-10-01

    WHEN AGE >= 60 AND HAS consumer_status
    THEN GRANT "Priority service in all consumer establishments"
}

// =============================================================================
// E-commerce Specific (Decreto 7.962/2013)
// =============================================================================

STATUTE ecommerce-info: "Decreto 7.962/2013 - Informacoes Claras" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 2013-03-15

    WHEN HAS ecommerce_transaction AND HAS consumer_status
    THEN OBLIGATION "Clear display of: full business address, CNPJ, contact info, total price, delivery terms"
}

STATUTE ecommerce-right-to-info: "Decreto 7.962/2013 - Sumario do Contrato" {
    JURISDICTION "BR"
    VERSION 1
    EFFECTIVE_DATE 2013-03-15

    WHEN HAS ecommerce_transaction AND HAS consumer_status
    THEN OBLIGATION "Provide contract summary before purchase confirmation"
}
"#;

/// Creates a consumer scenario entity
fn create_consumer_scenario(name: &str, age: u32, attributes: &[(&str, bool)]) -> BasicEntity {
    let mut entity = BasicEntity::new();
    entity.set_attribute("scenario_name", name.to_string());
    entity.set_attribute("age", age.to_string());

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
    println!("   CODIGO DE DEFESA DO CONSUMIDOR (CDC) - Legalis-RS Demo");
    println!("   Brazilian Consumer Defense Code - Lei 8.078/1990");
    println!("{}\n", "=".repeat(80));

    // Step 1: Parse CDC statutes
    println!("Etapa 1: Parsing CDC articles from DSL...\n");
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(CDC_STATUTES)?;
    println!("   Parsed {} CDC provisions:", statutes.len());
    for statute in &statutes {
        println!("   - {} ({})", statute.id, statute.title);
    }
    println!();

    // Step 2: Verify statute consistency
    println!("Etapa 2: Verifying CDC article consistency...\n");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    if result.passed {
        println!("   [OK] All CDC articles passed verification");
    } else {
        for error in &result.errors {
            println!("   [ERROR] {:?}", error);
        }
    }
    println!();

    // Step 3: Test consumer scenarios
    println!("Etapa 3: Evaluating consumer protection scenarios...\n");

    let scenarios = vec![
        // Scenario 1: Online purchase with defective product
        (
            "Compra Online com Produto Defeituoso",
            35u32,
            vec![
                ("consumer_status", true),
                ("remote_purchase", true),
                ("within_7_days", true),
                ("product_quality_defect", true),
                ("ecommerce_transaction", true),
            ],
        ),
        // Scenario 2: Elderly consumer exploited
        (
            "Idoso Vitima de Pratica Abusiva",
            72,
            vec![
                ("consumer_status", true),
                ("elderly_consumer", true),
                ("vulnerability_exploited", true),
            ],
        ),
        // Scenario 3: Tied selling attempt
        (
            "Tentativa de Venda Casada",
            28,
            vec![("consumer_status", true), ("tied_selling_attempted", true)],
        ),
        // Scenario 4: Service defect causing damage
        (
            "Servico Defeituoso com Dano",
            45,
            vec![
                ("consumer_status", true),
                ("service_defect", true),
                ("damage_occurred", true),
            ],
        ),
        // Scenario 5: Offer not honored
        (
            "Oferta Nao Cumprida",
            33,
            vec![
                ("consumer_status", true),
                ("public_offer_made", true),
                ("offer_not_honored", true),
            ],
        ),
        // Scenario 6: Abusive contract clause
        (
            "Clausula Abusiva em Contrato",
            40,
            vec![
                ("consumer_status", true),
                ("consumer_contract", true),
                ("liability_exclusion_clause", true),
                ("mandatory_arbitration_clause", true),
            ],
        ),
        // Scenario 7: Unsolicited product received
        (
            "Produto Nao Solicitado Recebido",
            55,
            vec![
                ("consumer_status", true),
                ("unsolicited_product_sent", true),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (scenario_name, age, attrs) in &scenarios {
        let entity = create_consumer_scenario(scenario_name, *age, attrs);
        let entity_id = entity.id();

        println!("   === {} (Idade: {}) ===", scenario_name, age);

        let mut applicable_rights = Vec::new();
        let mut prohibitions = Vec::new();

        for statute in &statutes {
            let applicable = check_applicability(&entity, statute);

            if applicable {
                let effect_type = &statute.effect.effect_type;
                if format!("{:?}", effect_type).contains("Prohibition") {
                    prohibitions.push(statute.id.clone());
                    println!("   [X] {} - PRATICA PROIBIDA", statute.id);
                } else {
                    applicable_rights.push(statute.id.clone());
                    println!("   [+] {} - DIREITO APLICAVEL", statute.id);
                }

                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "cdc-checker".to_string(),
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

        println!(
            "   Direitos: {} | Proibicoes violadas: {}",
            applicable_rights.len(),
            prohibitions.len()
        );
        println!();
    }

    // Step 4: Visualization
    println!("Etapa 4: Generating decision tree for direito de arrependimento...\n");
    if let Some(withdrawal) = statutes.iter().find(|s| s.id == "cdc-art49-withdrawal") {
        match DecisionTree::from_statute(withdrawal) {
            Ok(tree) => {
                let ascii = tree.to_ascii();
                println!("{}", ascii);
            }
            Err(e) => println!("   Warning: Could not generate tree: {:?}", e),
        }
    }
    println!();

    // Step 5: Population simulation
    println!("Etapa 5: Running consumer simulation (500 transactions)...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;

    println!("   Resultados da Simulacao:");
    println!("   -------------------------------------------");
    println!(
        "   Total transacoes avaliadas: {}",
        metrics.total_applications
    );
    println!(
        "   Resultados deterministicos: {}",
        metrics.deterministic_count
    );
    println!(
        "   Requer analise caso-a-caso: {}",
        metrics.discretion_count
    );
    println!();

    // Summary
    println!("{}", "=".repeat(80));
    println!("   CDC DEMO COMPLETE");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Principais Direitos do Consumidor Demonstrados:");
    println!("   - Direito de arrependimento (7 dias - Art. 49)");
    println!("   - Responsabilidade objetiva (Arts. 12-14)");
    println!("   - Vicios de produto/servico (Arts. 18-20)");
    println!("   - Oferta vinculante (Arts. 30-35)");
    println!("   - Praticas abusivas proibidas (Art. 39)");
    println!("   - Clausulas abusivas nulas (Art. 51)");
    println!();
    println!("   Orgaos de Defesa do Consumidor:");
    println!("   - PROCON (estadual/municipal)");
    println!("   - SENACON (federal)");
    println!("   - Ministerio Publico");
    println!("   - Juizados Especiais Civeis");
    println!();
    println!("   O CDC e considerado uma das leis de protecao ao");
    println!("   consumidor mais avancadas do mundo!");
    println!();

    Ok(())
}

/// Checks if a CDC provision applies to the scenario
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
