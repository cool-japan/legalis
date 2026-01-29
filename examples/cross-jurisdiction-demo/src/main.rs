//! Cross-Jurisdiction Demonstration
//!
//! This example proves that Legalis-RS is a GENERIC legal computation engine,
//! not country-specific code.
//!
//! The SAME engine processes:
//! - Japanese law (Civil Law system)
//! - German law (Civil Law system)
//! - US law (Common Law system)
//! - EU regulations (Sui generis supranational law)
//!
//! Key Insight:
//! - The Condition/Statute/Effect model is jurisdiction-agnostic
//! - Only the DATA (legal rules) changes, not the ENGINE
//! - This is true "Law as Code" - universal legal computation

use anyhow::Result;
use legalis_core::{AttributeBasedContext, ComparisonOp, Condition, Effect, EffectType, Statute};
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("🌍 Cross-Jurisdiction Demonstration");
    println!("   Proving: ONE ENGINE handles ALL legal systems\n");

    println!("═══════════════════════════════════════════════════════════");
    println!("  The Universal Legal Computation Engine");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📚 Testing the SAME Condition::Age across 4 jurisdictions:\n");

    // Japan: 民法第731条（婚姻適齢）
    demo_japan_marriage_age()?;

    // Germany: BGB §1303 (Ehemündigkeit)
    demo_germany_marriage_age()?;

    // USA: Marriage Age (varies by state, using California as example)
    demo_usa_marriage_age()?;

    // EU: GDPR Age of Consent for Digital Services (Article 8)
    demo_eu_digital_consent_age()?;

    println!("\n═══════════════════════════════════════════════════════════");
    println!("  Proof of Genericity");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("✅ All 4 jurisdictions use:");
    println!("   • Same type: Condition::Age {{ operator, value }}");
    println!("   • Same evaluation engine: evaluate_simple()");
    println!("   • Same result type: bool\n");

    println!("🔑 KEY INSIGHT:");
    println!("   This is NOT 4 different codebases for 4 countries.");
    println!("   This is ONE GENERIC ENGINE with 4 DATA INPUTS.\n");

    println!("💡 What changes between jurisdictions:");
    println!("   ❌ NOT the engine code");
    println!("   ❌ NOT the evaluation logic");
    println!("   ✅ ONLY the legal rule values (18 vs 16 vs 13)\n");

    println!("🚀 This proves:");
    println!("   Legalis-RS is a UNIVERSAL legal computation platform.");
    println!("   Add new jurisdiction = Add new DATA file.");
    println!("   NO code changes needed!");

    Ok(())
}

/// Japan: Marriage Age (民法第731条)
fn demo_japan_marriage_age() -> Result<()> {
    println!("▼ Japan (日本) - Civil Law System");
    println!("   Law: 民法第731条（婚姻適齢）");
    println!("   Rule: 18歳以上\n");

    let japan_marriage = Statute::new(
        "jp-minpo-731",
        "民法第731条",
        Effect::new(EffectType::Grant, "婚姻可能"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let test_17 = create_context(17);
    let test_18 = create_context(18);

    println!(
        "   Testing: 17歳 → {}",
        if evaluate(&japan_marriage, &test_17)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "   Testing: 18歳 → {}",
        if evaluate(&japan_marriage, &test_18)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!();

    Ok(())
}

/// Germany: Marriage Age (BGB §1303)
fn demo_germany_marriage_age() -> Result<()> {
    println!("▼ Germany (Deutschland) - Civil Law System");
    println!("   Law: BGB §1303 (Ehemündigkeit)");
    println!("   Rule: 18 Jahre oder älter\n");

    let germany_marriage = Statute::new(
        "de-bgb-1303",
        "BGB §1303",
        Effect::new(EffectType::Grant, "Eheschließung zulässig"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let test_17 = create_context(17);
    let test_18 = create_context(18);

    println!(
        "   Testing: 17 Jahre → {}",
        if evaluate(&germany_marriage, &test_17)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "   Testing: 18 Jahre → {}",
        if evaluate(&germany_marriage, &test_18)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!();

    Ok(())
}

/// USA: Marriage Age (California Family Code §301)
fn demo_usa_marriage_age() -> Result<()> {
    println!("▼ USA (California) - Common Law System");
    println!("   Law: California Family Code §301");
    println!("   Rule: Age 18 or above\n");

    let usa_marriage = Statute::new(
        "us-ca-fam-301",
        "CA Family Code §301",
        Effect::new(EffectType::Grant, "Marriage permitted"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let test_17 = create_context(17);
    let test_18 = create_context(18);

    println!(
        "   Testing: 17 years → {}",
        if evaluate(&usa_marriage, &test_17)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "   Testing: 18 years → {}",
        if evaluate(&usa_marriage, &test_18)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!();

    Ok(())
}

/// EU: GDPR Age of Consent for Digital Services (Article 8)
fn demo_eu_digital_consent_age() -> Result<()> {
    println!("▼ EU - Supranational Regulation");
    println!("   Law: GDPR Article 8 (Age of Consent for Digital Services)");
    println!("   Rule: 13歳以上 (with parental consent exceptions)\n");

    let eu_digital_consent = Statute::new(
        "eu-gdpr-8",
        "GDPR Article 8",
        Effect::new(EffectType::Grant, "Digital service consent valid"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 13, // Minimum age for digital service consent (some MS raise to 16)
    });

    let test_12 = create_context(12);
    let test_13 = create_context(13);

    println!(
        "   Testing: 12 years → {}",
        if evaluate(&eu_digital_consent, &test_12)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "   Testing: 13 years → {}",
        if evaluate(&eu_digital_consent, &test_13)? {
            "✅"
        } else {
            "❌"
        }
    );
    println!();

    Ok(())
}

/// Create simple context with age
fn create_context(age: u32) -> AttributeBasedContext {
    let mut attrs = HashMap::new();
    attrs.insert("age".to_string(), age.to_string());
    AttributeBasedContext::new(attrs)
}

/// Evaluate statute (wrapper)
fn evaluate(statute: &Statute, context: &AttributeBasedContext) -> Result<bool> {
    for condition in &statute.preconditions {
        if !condition.evaluate_simple(context)? {
            return Ok(false);
        }
    }
    Ok(true)
}
