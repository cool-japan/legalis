//! Mexican Tax Law (Codigo Fiscal de la Federacion)
//!
//! This example demonstrates how to use Legalis-RS for Mexican tax
//! compliance under the Codigo Fiscal and related laws.
//!
//! ## Mexican Tax System
//!
//! Key taxes:
//! - ISR (Income Tax)
//! - IVA (Value Added Tax)
//! - IEPS (Special Tax on Production)

use std::collections::HashMap;

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use legalis_core::{BasicEntity, ComparisonOp, Condition, LegalEntity, Statute};
use legalis_dsl::LegalDslParser;
use legalis_sim::{PopulationBuilder, SimEngine};
use legalis_verifier::StatuteVerifier;

const MX_TAX_STATUTES: &str = r#"
// =============================================================================
// Codigo Fiscal de la Federacion (Mexico)
// =============================================================================

// RFC Registration
STATUTE mx-cff-rfc: "CFF Art. 27 - RFC Registration" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS taxable_activity OR AGE >= 18
    THEN OBLIGATION "Register with SAT and obtain RFC (Registro Federal de Contribuyentes)"
}

// CFDI Invoicing
STATUTE mx-cfdi-invoicing: "CFF Art. 29 - Digital Invoicing (CFDI)" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS business_activity AND HAS sale_or_service
    THEN OBLIGATION "Issue CFDI (Comprobante Fiscal Digital por Internet) for all transactions"
}

// =============================================================================
// Ley del Impuesto Sobre la Renta (Income Tax Law)
// =============================================================================

// Individual Tax Brackets
STATUTE mx-isr-individual: "LISR - Individual Income Tax" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS resident_individual AND HAS taxable_income
    THEN OBLIGATION "Pay ISR on income (1.92% to 35% progressive rates)"
}

// Corporate Tax
STATUTE mx-isr-corporate: "LISR Art. 9 - Corporate Income Tax" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS corporate_entity AND HAS taxable_income
    THEN OBLIGATION "Pay 30% corporate income tax on net profit"
}

// RESICO (Simplified Trust Regime)
STATUTE mx-resico-individual: "LISR - RESICO Individual" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS resident_individual AND INCOME <= 3500000 AND HAS professional_activity
    THEN GRANT "RESICO regime: simplified 1%-2.5% tax on gross income"
}

// Dividends
STATUTE mx-isr-dividends: "LISR - Dividend Withholding" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS dividend_distribution AND NOT HAS from_cufin
    THEN OBLIGATION "10% additional ISR on distributed dividends"
}

// =============================================================================
// Ley del IVA (Value Added Tax Law)
// =============================================================================

// Standard IVA Rate
STATUTE mx-iva-standard: "LIVA - Standard 16% Rate" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS sale_or_service AND NOT HAS iva_exempt AND NOT HAS border_zone
    THEN OBLIGATION "Charge 16% IVA on sales and services"
}

// Border Zone Rate
STATUTE mx-iva-border: "LIVA - Border Zone 8% Rate" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS sale_or_service AND HAS border_zone_business
    THEN GRANT "Reduced 8% IVA rate in northern border zone"
}

// Zero Rate
STATUTE mx-iva-zero: "LIVA Art. 2-A - Zero Rate" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN (HAS basic_food_sale OR HAS medicine_sale OR HAS export_sale)
    THEN GRANT "0% IVA rate (with right to credit input IVA)"
}

// IVA Exemptions
STATUTE mx-iva-exempt: "LIVA Art. 15 - Exemptions" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS education_service OR HAS medical_service OR HAS residential_rent
    THEN GRANT "IVA exempt (no IVA charged, no input credit)"
}

// =============================================================================
// IEPS (Special Tax on Production and Services)
// =============================================================================

STATUTE mx-ieps-fuel: "LIEPS - Fuel Tax" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS fuel_sale
    THEN OBLIGATION "IEPS on gasoline, diesel (variable quota)"
}

STATUTE mx-ieps-sugar: "LIEPS - Sugar Tax" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS sugary_beverage_sale
    THEN OBLIGATION "IEPS of 1 peso per liter on sugary beverages"
}

STATUTE mx-ieps-tobacco: "LIEPS - Tobacco Tax" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS tobacco_product_sale
    THEN OBLIGATION "IEPS on tobacco products (160% + quota)"
}

// =============================================================================
// Compliance Obligations
// =============================================================================

STATUTE mx-annual-declaration: "CFF - Annual Tax Return" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS taxpayer_status
    THEN OBLIGATION "File annual tax return (individuals: April, corporations: March)"
}

STATUTE mx-monthly-provisional: "LISR - Monthly Provisional Payments" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS business_activity OR HAS professional_activity
    THEN OBLIGATION "Make monthly provisional ISR payments by 17th of following month"
}

STATUTE mx-withholding-salaries: "LISR - Salary Withholding" {
    JURISDICTION "MX"
    VERSION 2024
    EFFECTIVE_DATE 2024-01-01

    WHEN HAS employer_status AND HAS salary_payments
    THEN OBLIGATION "Withhold ISR from employee salaries per tax tables"
}
"#;

fn create_taxpayer(name: &str, income: Option<u64>, attrs: &[(&str, bool)]) -> BasicEntity {
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
    println!("   CODIGO FISCAL DE LA FEDERACION - Legalis-RS Demo");
    println!("   Mexican Tax Law: ISR | IVA | IEPS");
    println!("{}\n", "=".repeat(80));

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(MX_TAX_STATUTES)?;
    println!("Paso 1: Parsed {} disposiciones fiscales\n", statutes.len());

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);
    println!(
        "Paso 2: Verificacion {}\n",
        if result.passed { "APROBADA" } else { "FALLIDA" }
    );

    println!("Paso 3: Evaluando obligaciones fiscales...\n");

    let taxpayers = vec![
        (
            "Empresa Grande (Corporation)",
            None,
            vec![
                ("corporate_entity", true),
                ("taxable_income", true),
                ("sale_or_service", true),
                ("business_activity", true),
                ("taxpayer_status", true),
                ("employer_status", true),
                ("salary_payments", true),
            ],
        ),
        (
            "Profesionista RESICO",
            Some(2000000u64),
            vec![
                ("resident_individual", true),
                ("taxable_income", true),
                ("professional_activity", true),
                ("taxpayer_status", true),
            ],
        ),
        (
            "Exportador",
            None,
            vec![
                ("corporate_entity", true),
                ("export_sale", true),
                ("business_activity", true),
                ("taxpayer_status", true),
            ],
        ),
        (
            "Negocio Zona Fronteriza",
            None,
            vec![
                ("corporate_entity", true),
                ("border_zone_business", true),
                ("sale_or_service", true),
                ("taxpayer_status", true),
            ],
        ),
        (
            "Servicio Medico",
            None,
            vec![
                ("resident_individual", true),
                ("medical_service", true),
                ("professional_activity", true),
                ("taxpayer_status", true),
            ],
        ),
    ];

    let mut audit_trail = AuditTrail::new();

    for (name, income, attrs) in &taxpayers {
        let entity = create_taxpayer(name, *income, attrs);
        println!("   === {} ===", name);

        for statute in &statutes {
            if check_applicability(&entity, statute) {
                println!("   [+] {}", statute.id);
                let record = AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "mx-tax".to_string(),
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

    println!("Paso 4: Simulacion poblacional...\n");
    let population = PopulationBuilder::new().generate_random(500).build();
    let engine = SimEngine::new(statutes.clone(), population);
    let metrics = engine.run_simulation().await;
    println!(
        "   Total: {} | Deterministicos: {} | Discrecion: {}\n",
        metrics.total_applications, metrics.deterministic_count, metrics.discretion_count
    );

    println!("{}", "=".repeat(80));
    println!("   DEMO FISCAL MEXICANO COMPLETADO");
    println!("{}", "=".repeat(80));
    println!("\n   Principales Impuestos:");
    println!("   - ISR: Impuesto Sobre la Renta (1.92%-35% / 30% corp)");
    println!("   - IVA: Impuesto al Valor Agregado (16% / 8% frontera)");
    println!("   - IEPS: Impuesto Especial sobre Produccion y Servicios");
    println!("\n   Autoridad: SAT (Servicio de Administracion Tributaria)\n");

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
