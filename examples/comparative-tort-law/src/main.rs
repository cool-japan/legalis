//! Comparative Tort Law Simulation: Japan 🇯🇵 Germany 🇩🇪 France 🇫🇷 United States 🇺🇸
//!
//! This example demonstrates how Legalis-RS can be used for comparative legal analysis
//! by simulating the same tort scenario under Japanese, German, French, and US law.
//!
//! ## Laws Compared
//!
//! ### Civil Law Systems (大陸法)
//!
//! - 🇯🇵 **Japanese Civil Code Article 709** (民法第709条)
//!   - General clause: "rights or legally protected interests"
//!   - Medium abstraction level
//!
//! - 🇩🇪 **German BGB § 823 Abs. 1**
//!   - Enumerated protected interests: life, body, health, freedom, property, other rights
//!   - Concrete, limited approach
//!
//! - 🇩🇪 **German BGB § 826**
//!   - Intentional damage contrary to good morals
//!   - Catches conduct not covered by § 823
//!
//! - 🇫🇷 **French Code civil Article 1240** (ex-1382)
//!   - Ultra-abstract: "any act whatever of man causing damage"
//!   - Maximum abstraction level
//!
//! ### Common Law System (英米法)
//!
//! - 🇺🇸 **US Restatement (Second) of Torts**
//!   - § 158: Battery (intentional tort)
//!   - Case-based reasoning from precedents
//!   - Punitive damages available
//!
//! ## Legal Philosophy Spectrum
//!
//! ```text
//! CIVIL LAW
//! Concrete ←―――――――――――――――――――――→ Abstract
//! (Certainty)                    (Flexibility)
//!
//!    🇩🇪 BGB § 823     🇯🇵 Art. 709     🇫🇷 Art. 1240
//!    (Enumeration)     (Medium)         (Universal)
//!
//! COMMON LAW
//!    🇺🇸 Restatement
//!    (Case synthesis)
//! ```
//!
//! ## Comparative Legal Analysis
//!
//! This tool enables empirical study of:
//! - How different legal systems handle the same factual scenario
//! - Civil Law (statute-based) vs Common Law (case-based) approaches
//! - Coverage gaps between enumerated vs general clause approaches
//! - Role of judicial discretion across legal systems
//! - Trade-offs between legal certainty and flexibility
//! - Unique Common Law features (punitive damages, precedent binding)

use legalis_core::{BasicEntity, LegalEntity, LegalResult, Statute};
use legalis_de::{bgb_823_1, bgb_826};
use legalis_fr::article_1240;
use legalis_jp::article_709;
use legalis_sim::{SimEngine, SimulationMetrics};
use legalis_us::{battery_as_statute, iied_as_statute, products_liability_as_statute};

#[tokio::main]
async fn main() {
    println!("╔═════════════════════════════════════════════════════════════════════╗");
    println!("║   Comparative Tort Law: Japan 🇯🇵 Germany 🇩🇪 France 🇫🇷 USA 🇺🇸        ║");
    println!("║   比較不法行為法: 日本・ドイツ・フランス・米国                              ║");
    println!("╚═════════════════════════════════════════════════════════════════════╝\n");

    println!("This simulation evaluates tort scenarios under FOUR legal systems:");
    println!("  • 🇯🇵 Japanese Civil Code Article 709 (民法第709条) - Civil Law");
    println!("  • 🇩🇪 German BGB § 823 Abs. 1 & § 826 - Civil Law");
    println!("  • 🇫🇷 French Code civil Article 1240 (ex-Art. 1382) - Civil Law");
    println!("  • 🇺🇸 US Restatement (Second) of Torts - Common Law\n");

    println!("📚 Legal Traditions:");
    println!("  Civil Law (大陸法):  Statute-based → Deductive reasoning");
    println!("  Common Law (英米法): Case-based   → Analogical reasoning\n");

    println!("🎯 Philosophy: Concrete (🇩🇪) ← Medium (🇯🇵) ← Abstract (🇫🇷) ‖ Precedent (🇺🇸)\n");

    println!("{}\n", "=".repeat(75));

    // Test Case 1: Physical injury by intentional battery
    test_case_1_battery();

    println!("\n{}\n", "=".repeat(75));

    // Test Case 2: Economic loss through fraud
    test_case_2_fraud();

    println!("\n{}\n", "=".repeat(75));

    // Test Case 3: Personality rights violation (defamation)
    test_case_3_personality_rights();

    println!("\n{}\n", "=".repeat(75));

    // Test Case 4: Pure economic loss (negligence, no physical harm)
    test_case_4_pure_economic_loss();

    println!("\n{}\n", "=".repeat(75));

    // Test Case 5: Intentional infliction of emotional distress (IIED)
    test_case_5_emotional_distress();

    println!("\n{}\n", "=".repeat(75));

    // Test Case 6: Defective product causing injury
    test_case_6_product_liability();

    println!("\n{}\n", "=".repeat(75));

    println!("\n{}\n", "=".repeat(75));

    // Comparative analysis: population-based simulation across all four jurisdictions
    println!("\n📊 Comparative Analysis\n");
    run_comparative_simulation().await;
}

/// Test Case 1: Battery (Physical Assault)
///
/// Facts: A intentionally punched B, causing bodily injury.
fn test_case_1_battery() {
    println!("📋 Test Case 1: Battery (暴行 / Körperverletzung / Voie de fait)");
    println!("   Facts: A intentionally punched B, causing bodily injury");
    println!("   事実: Aが故意にBを殴打し、身体傷害を生じさせた\n");

    let mut agent = BasicEntity::new();

    // Common facts
    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("negligence", "false".to_string());
    agent.set_attribute("infringement", "true".to_string()); // JP: rights violated
    agent.set_attribute("illegality", "true".to_string()); // JP
    agent.set_attribute("damage", "true".to_string());
    agent.set_attribute("causation", "true".to_string());

    // German-specific: § 823(1) enumerated interests
    agent.set_attribute("life_violated", "false".to_string());
    agent.set_attribute("body_violated", "true".to_string()); // Körperverletzung
    agent.set_attribute("health_violated", "true".to_string());

    // French-specific
    agent.set_attribute("faute_intentionnelle", "true".to_string());
    agent.set_attribute("dommage", "true".to_string());
    agent.set_attribute("lien_causalite", "true".to_string());

    // US-specific (Restatement battery)
    agent.set_attribute("voluntary_act", "true".to_string());
    agent.set_attribute("intent_harmful_contact", "true".to_string());
    agent.set_attribute("harmful_contact", "true".to_string());

    // Evaluate under all four systems
    println!("  🇯🇵 Japan (Article 709):");
    let result_jp = SimEngine::apply_law(&agent, &article_709());
    print_result(&result_jp);

    println!("\n  🇩🇪 Germany (BGB § 823(1)):");
    let result_de = SimEngine::apply_law(&agent, &bgb_823_1());
    print_result(&result_de);

    println!("\n  🇫🇷 France (Article 1240):");
    let result_fr = SimEngine::apply_law(&agent, &article_1240());
    print_result(&result_fr);

    println!("\n  🇺🇸 USA (Restatement § 158 Battery):");
    let result_us = SimEngine::apply_law(&agent, &battery_as_statute());
    print_result(&result_us);

    println!("\n  ✅ Consensus: All systems impose liability for intentional battery.");
    println!("     Civil Law: Protected interest (body/health) violated + fault + causation");
    println!(
        "     Common Law: Intent + harmful contact (no need for 'protected interest' analysis)"
    );
}

/// Test Case 2: Economic Loss through Fraud
///
/// Facts: A fraudulently induced B to invest, causing pure economic loss (no physical harm).
fn test_case_2_fraud() {
    println!("📋 Test Case 2: Fraud (詐欺 / Betrug / Fraude)");
    println!("   Facts: A fraudulently induced B to invest, causing pure economic loss");
    println!("   事実: Aが詐欺的にBを投資に誘導し、純粋経済損失を生じさせた\n");

    let mut agent = BasicEntity::new();

    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("negligence", "false".to_string());
    agent.set_attribute("infringement", "true".to_string()); // JP: property rights
    agent.set_attribute("illegality", "true".to_string());
    agent.set_attribute("damage", "true".to_string());
    agent.set_attribute("causation", "true".to_string());

    // German-specific: NOT covered by § 823(1) (no enumerated interest violated)
    // Only property (Eigentum) is listed, but economic loss ≠ property damage
    agent.set_attribute("life_violated", "false".to_string());
    agent.set_attribute("body_violated", "false".to_string());
    agent.set_attribute("property_violated", "false".to_string()); // No property damaged
    agent.set_attribute("freedom_violated", "false".to_string());

    // German § 826: Intentional + good morals
    agent.set_attribute("intentional_harm", "true".to_string());
    agent.set_attribute("contra_bonos_mores", "true".to_string()); // Fraud violates good morals

    // French
    agent.set_attribute("faute_intentionnelle", "true".to_string());
    agent.set_attribute("dommage", "true".to_string());
    agent.set_attribute("lien_causalite", "true".to_string());

    // US: Fraud is covered under general tort principles (misrepresentation)
    // Not modeled in Restatement battery, but would be separate tort
    agent.set_attribute("voluntary_act", "true".to_string());
    agent.set_attribute("intent_harmful_contact", "false".to_string()); // No battery here

    println!("  🇯🇵 Japan (Article 709):");
    let result_jp = SimEngine::apply_law(&agent, &article_709());
    print_result(&result_jp);

    println!("\n  🇩🇪 Germany (BGB § 823(1)):");
    let result_de_823 = SimEngine::apply_law(&agent, &bgb_823_1());
    print_result(&result_de_823);

    println!("\n  🇩🇪 Germany (BGB § 826 - good morals):");
    let result_de_826 = SimEngine::apply_law(&agent, &bgb_826());
    print_result(&result_de_826);

    println!("\n  🇫🇷 France (Article 1240):");
    let result_fr = SimEngine::apply_law(&agent, &article_1240());
    print_result(&result_fr);

    println!("\n  🇺🇸 USA (Battery):");
    let result_us = SimEngine::apply_law(&agent, &battery_as_statute());
    print_result(&result_us);

    println!("\n  ⚠️  Structural Gap:");
    println!("     🇩🇪 § 823(1): ❌ No liability (pure economic loss not enumerated)");
    println!("     🇩🇪 § 826:    ✅ Liability (catches fraud via good morals)");
    println!("     🇯🇵🇫🇷🇺🇸:      ✅ All cover fraud (general clauses / separate torts)");
    println!("\n     This shows the cost of German enumeration: gaps require § 826 backstop.");
}

/// Test Case 3: Personality Rights Violation (Defamation)
///
/// Facts: A published false statements damaging B's reputation.
fn test_case_3_personality_rights() {
    println!("📋 Test Case 3: Defamation (名誉毀損 / Ehrbeleidigung / Diffamation)");
    println!("   Facts: A published false statements damaging B's reputation");
    println!("   事実: Aが虚偽の陳述を公表し、Bの名誉を毀損した\n");

    let mut agent = BasicEntity::new();

    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("infringement", "true".to_string()); // JP: personality rights
    agent.set_attribute("illegality", "true".to_string());
    agent.set_attribute("damage", "true".to_string());
    agent.set_attribute("causation", "true".to_string());

    // German: "sonstiges Recht" (other rights) includes personality rights
    agent.set_attribute("other_right_violated", "true".to_string());

    // French
    agent.set_attribute("faute_intentionnelle", "true".to_string());
    agent.set_attribute("dommage", "true".to_string());
    agent.set_attribute("lien_causalite", "true".to_string());

    // US: Defamation is separate tort (not battery)
    agent.set_attribute("voluntary_act", "true".to_string());
    agent.set_attribute("intent_harmful_contact", "false".to_string());

    println!("  🇯🇵 Japan (Article 709):");
    let result_jp = SimEngine::apply_law(&agent, &article_709());
    print_result(&result_jp);

    println!("\n  🇩🇪 Germany (BGB § 823(1) - sonstiges Recht):");
    let result_de = SimEngine::apply_law(&agent, &bgb_823_1());
    print_result(&result_de);

    println!("\n  🇫🇷 France (Article 1240):");
    let result_fr = SimEngine::apply_law(&agent, &article_1240());
    print_result(&result_fr);

    println!("\n  🇺🇸 USA (Battery - not applicable):");
    let result_us = SimEngine::apply_law(&agent, &battery_as_statute());
    print_result(&result_us);

    println!("\n  ✅ Consensus (Civil Law): All three systems recognize personality rights.");
    println!("     🇩🇪: Via 'sonstiges Recht' (catch-all in enumeration)");
    println!("     🇯🇵: Via 'legally protected interests'");
    println!("     🇫🇷: Via 'tout dommage' (any damage)");
    println!("\n  🇺🇸 Note: Defamation is a SEPARATE tort in Common Law (not battery).");
    println!(
        "     This shows Common Law's categorical approach: different torts, different rules."
    );
}

/// Test Case 4: Pure Economic Loss (Negligence)
///
/// Facts: A negligently gave bad investment advice, causing B's economic loss.
fn test_case_4_pure_economic_loss() {
    println!("📋 Test Case 4: Pure Economic Loss by Negligence (過失による純粋経済損失)");
    println!("   Facts: A negligently gave bad investment advice, causing B's economic loss");
    println!("   事実: Aが過失により誤った投資助言をし、Bに経済的損失を生じさせた\n");

    let mut agent = BasicEntity::new();

    agent.set_attribute("intent", "false".to_string());
    agent.set_attribute("negligence", "true".to_string());
    agent.set_attribute("infringement", "true".to_string()); // Arguably
    agent.set_attribute("illegality", "true".to_string()); // Judicial discretion
    agent.set_attribute("damage", "true".to_string());
    agent.set_attribute("causation", "true".to_string());

    // German: No enumerated interest violated (negligence + economic loss)
    agent.set_attribute("property_violated", "false".to_string());

    // French
    agent.set_attribute("faute_negligence", "true".to_string());
    agent.set_attribute("dommage", "true".to_string());
    agent.set_attribute("lien_causalite", "true".to_string());

    // US: Negligent misrepresentation exists but not battery
    agent.set_attribute("voluntary_act", "true".to_string());
    agent.set_attribute("intent_harmful_contact", "false".to_string());

    println!("  🇯🇵 Japan (Article 709):");
    let result_jp = SimEngine::apply_law(&agent, &article_709());
    print_result(&result_jp);

    println!("\n  🇩🇪 Germany (BGB § 823(1)):");
    let result_de = SimEngine::apply_law(&agent, &bgb_823_1());
    print_result(&result_de);

    println!("\n  🇫🇷 France (Article 1240):");
    let result_fr = SimEngine::apply_law(&agent, &article_1240());
    print_result(&result_fr);

    println!("\n  🇺🇸 USA (Battery - not applicable):");
    let result_us = SimEngine::apply_law(&agent, &battery_as_statute());
    print_result(&result_us);

    println!("\n  ⚖️  Judicial Discretion Zone:");
    println!("     🇯🇵: Requires discretion (was 'legally protected interest' violated?)");
    println!("     🇩🇪: ❌ Not covered by § 823(1) (no enumerated interest)");
    println!("     🇫🇷: ✅ Likely covered (faute + dommage suffice)");
    println!("     🇺🇸: Separate tort (negligent misrepresentation) - not battery");
    println!("\n     French law is most plaintiff-friendly; German law most restrictive.");
}

/// Test Case 5: Intentional Infliction of Emotional Distress
///
/// Facts: A engaged in extreme and outrageous conduct causing B severe emotional distress.
fn test_case_5_emotional_distress() {
    println!("📋 Test Case 5: Intentional Infliction of Emotional Distress (IIED)");
    println!("   Facts: A engaged in extreme and outrageous conduct causing severe distress");
    println!("   事実: Aが極端かつ非道な行為により、Bに重大な精神的苦痛を生じさせた\n");

    let mut agent = BasicEntity::new();

    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("infringement", "true".to_string()); // Mental health
    agent.set_attribute("illegality", "true".to_string());
    agent.set_attribute("damage", "true".to_string());
    agent.set_attribute("causation", "true".to_string());

    // German: "Gesundheit" (health) can include mental health
    agent.set_attribute("health_violated", "true".to_string());

    // French
    agent.set_attribute("faute_intentionnelle", "true".to_string());
    agent.set_attribute("dommage", "true".to_string()); // Moral damage
    agent.set_attribute("lien_causalite", "true".to_string());

    // US: IIED-specific conditions
    agent.set_attribute("extreme_outrageous_conduct", "true".to_string());
    agent.set_attribute("intent_emotional_distress", "true".to_string());
    agent.set_attribute("severe_emotional_distress", "true".to_string());

    println!("  🇯🇵 Japan (Article 709 + Article 710 non-pecuniary damages):");
    let result_jp = SimEngine::apply_law(&agent, &article_709());
    print_result(&result_jp);

    println!("\n  🇩🇪 Germany (BGB § 823(1) - Gesundheit):");
    let result_de = SimEngine::apply_law(&agent, &bgb_823_1());
    print_result(&result_de);

    println!("\n  🇫🇷 France (Article 1240 - dommage moral):");
    let result_fr = SimEngine::apply_law(&agent, &article_1240());
    print_result(&result_fr);

    println!("\n  🇺🇸 USA (Restatement § 46 IIED):");
    let result_us = SimEngine::apply_law(&agent, &iied_as_statute());
    print_result(&result_us);

    println!("\n  📖 Common Law Specificity:");
    println!("     IIED is a SEPARATE intentional tort with HIGH threshold:");
    println!("     • Conduct must be 'extreme and outrageous' (beyond all decency)");
    println!("     • Distress must be 'severe' (not mere upset)");
    println!("     Civil Law systems fold this into general tort provisions.");
}

/// Test Case 6: Defective Product Causing Injury
///
/// Facts: Manufacturer sold defective product that injured consumer.
fn test_case_6_product_liability() {
    println!("📋 Test Case 6: Products Liability (製造物責任 / Produkthaftung)");
    println!("   Facts: Manufacturer sold defective product that injured consumer");
    println!("   事実: 製造業者が欠陥製品を販売し、消費者が負傷した\n");

    let mut agent = BasicEntity::new();

    // Civil Law: Typically covered by separate product liability statutes
    agent.set_attribute("negligence", "true".to_string());
    agent.set_attribute("infringement", "true".to_string());
    agent.set_attribute("illegality", "true".to_string());
    agent.set_attribute("damage", "true".to_string());
    agent.set_attribute("causation", "true".to_string());
    agent.set_attribute("body_violated", "true".to_string());
    agent.set_attribute("health_violated", "true".to_string());

    // French
    agent.set_attribute("faute_negligence", "true".to_string());
    agent.set_attribute("dommage", "true".to_string());
    agent.set_attribute("lien_causalite", "true".to_string());

    // US: Strict liability under § 402A
    agent.set_attribute("commercial_seller", "true".to_string());
    agent.set_attribute("design_defect", "true".to_string());
    agent.set_attribute("unreasonably_dangerous", "true".to_string());
    agent.set_attribute("no_substantial_change", "true".to_string());
    agent.set_attribute("physical_harm", "true".to_string());

    println!("  🇯🇵 Japan (Article 709 - but see製造物責任法PL Act 1994):");
    let result_jp = SimEngine::apply_law(&agent, &article_709());
    print_result(&result_jp);

    println!("\n  🇩🇪 Germany (BGB § 823(1) - but see Produkthaftungsgesetz 1989):");
    let result_de = SimEngine::apply_law(&agent, &bgb_823_1());
    print_result(&result_de);

    println!("\n  🇫🇷 France (Article 1240 - but see EU Directive 1985):");
    let result_fr = SimEngine::apply_law(&agent, &article_1240());
    print_result(&result_fr);

    println!("\n  🇺🇸 USA (Restatement § 402A - Strict Liability):");
    let result_us = SimEngine::apply_law(&agent, &products_liability_as_statute());
    print_result(&result_us);

    println!("\n  🏛️  Legal Development Path:");
    println!("     Common Law → Case law (Greenman 1963) → Restatement § 402A");
    println!("     Civil Law  → EU Directive (1985) → National statutes");
    println!("\n  🔍 Key Difference:");
    println!("     US: STRICT LIABILITY (no fault required) developed through cases");
    println!("     Civil Law: Enacted strict liability statutes after US case law development");
}

/// Helper function to print legal results
fn print_result(result: &LegalResult<legalis_core::Effect>) {
    match result {
        LegalResult::Deterministic(effect) => {
            println!(
                "     ✅ Deterministic: {} - {}",
                effect.effect_type, effect.description
            );
        }
        LegalResult::JudicialDiscretion { issue, .. } => {
            println!("     ⚖️  Judicial Discretion Required: {}", issue);
        }
        LegalResult::Void { reason } => {
            println!("     ❌ Void: {}", reason);
        }
    }
}

/// Build a population of entities with varied tort scenario attributes and run a simulation.
///
/// Each entity's attributes are assigned deterministically from its index to ensure
/// reproducible scenario variation across intent, negligence, damage, and jurisdiction-specific
/// conditions.
pub async fn build_and_run_sim(
    statutes: Vec<Statute>,
    population_size: usize,
) -> SimulationMetrics {
    let mut population: Vec<Box<dyn LegalEntity>> = Vec::with_capacity(population_size);

    for i in 0..population_size {
        let mut entity = BasicEntity::new();

        let has_intent = i % 3 == 0;
        let has_negligence = i % 2 == 0;
        let has_damage = i % 10 != 0; // 90% have damage

        // Shared attributes
        entity.set_attribute("intent", has_intent.to_string());
        entity.set_attribute("negligence", has_negligence.to_string());
        entity.set_attribute("damage", has_damage.to_string());
        entity.set_attribute("causation", "true".to_string());
        entity.set_attribute("infringement", (i % 4 != 0).to_string());
        entity.set_attribute("illegality", (i % 5 != 0).to_string());

        // German-specific (§ 823(1) enumerated interests)
        entity.set_attribute("body_violated", (i % 3 == 0).to_string());
        entity.set_attribute("health_violated", (i % 4 == 0).to_string());
        entity.set_attribute("intentional_harm", has_intent.to_string());
        entity.set_attribute("contra_bonos_mores", (i % 6 == 0).to_string());

        // French-specific
        entity.set_attribute("faute_intentionnelle", has_intent.to_string());
        entity.set_attribute("faute_negligence", has_negligence.to_string());
        entity.set_attribute("dommage", has_damage.to_string());
        entity.set_attribute("lien_causalite", "true".to_string());

        // US battery / IIED
        entity.set_attribute("voluntary_act", "true".to_string());
        entity.set_attribute("intent_harmful_contact", has_intent.to_string());
        entity.set_attribute("harmful_contact", (i % 3 == 0).to_string());
        entity.set_attribute("extreme_outrageous_conduct", (i % 5 == 0).to_string());
        entity.set_attribute("intent_emotional_distress", has_intent.to_string());
        entity.set_attribute("severe_emotional_distress", (i % 7 == 0).to_string());

        // US products liability (§ 402A)
        entity.set_attribute("commercial_seller", "true".to_string());
        entity.set_attribute("design_defect", (i % 4 == 0).to_string());
        entity.set_attribute("unreasonably_dangerous", (i % 4 == 0).to_string());
        entity.set_attribute("no_substantial_change", "true".to_string());
        entity.set_attribute("physical_harm", has_damage.to_string());

        population.push(Box::new(entity));
    }

    SimEngine::new(statutes, population).run_simulation().await
}

/// Run population-based comparative simulation across all four jurisdictions.
async fn run_comparative_simulation() {
    let num_simulations: usize = 1000;

    let statutes_jp = vec![article_709()];
    let statutes_de = vec![bgb_823_1(), bgb_826()];
    let statutes_fr = vec![article_1240()];
    let statutes_us = vec![
        battery_as_statute(),
        iied_as_statute(),
        products_liability_as_statute(),
    ];

    let (metrics_jp, metrics_de, metrics_fr, metrics_us) = tokio::join!(
        build_and_run_sim(statutes_jp, num_simulations),
        build_and_run_sim(statutes_de, num_simulations),
        build_and_run_sim(statutes_fr, num_simulations),
        build_and_run_sim(statutes_us, num_simulations),
    );

    println!(
        "Simulated {} random tort scenarios per jurisdiction:\n",
        num_simulations
    );

    println!("┌────────────────┬──────────────┬─────────────┬──────────┐");
    println!("│ System         │ Deterministic│ Discretion  │ Coverage │");
    println!("├────────────────┼──────────────┼─────────────┼──────────┤");

    let coverage_jp =
        (metrics_jp.deterministic_count as f64 / metrics_jp.total_applications as f64) * 100.0;
    println!(
        "│ 🇯🇵 Japan 709    │ {:>12} │ {:>11} │ {:>6.1}%  │",
        metrics_jp.deterministic_count, metrics_jp.discretion_count, coverage_jp
    );

    let coverage_de =
        (metrics_de.deterministic_count as f64 / metrics_de.total_applications as f64) * 100.0;
    println!(
        "│ 🇩🇪 Germany BGB  │ {:>12} │ {:>11} │ {:>6.1}%  │",
        metrics_de.deterministic_count, metrics_de.discretion_count, coverage_de
    );

    let coverage_fr =
        (metrics_fr.deterministic_count as f64 / metrics_fr.total_applications as f64) * 100.0;
    println!(
        "│ 🇫🇷 France 1240  │ {:>12} │ {:>11} │ {:>6.1}%  │",
        metrics_fr.deterministic_count, metrics_fr.discretion_count, coverage_fr
    );

    let coverage_us =
        (metrics_us.deterministic_count as f64 / metrics_us.total_applications as f64) * 100.0;
    println!(
        "│ 🇺🇸 USA Restate. │ {:>12} │ {:>11} │ {:>6.1}%  │",
        metrics_us.deterministic_count, metrics_us.discretion_count, coverage_us
    );

    println!("└────────────────┴──────────────┴─────────────┴──────────┘\n");

    println!("📊 Observations:");
    println!("   • Coverage: How many scenarios resulted in deterministic liability");
    println!("   • French law (abstract) likely has HIGHEST coverage");
    println!("   • German law (enumerated) likely has LOWEST coverage (gaps)");
    println!("   • US law (battery only) has LOW coverage (specific tort, not general)");
    println!("   • Japanese law (medium) falls in between\n");

    println!("🔬 Legal Philosophy Implications:");
    println!("   Concrete (🇩🇪):  High certainty, low flexibility → Gaps require § 826");
    println!("   Medium (🇯🇵):   Balance between certainty and flexibility");
    println!("   Abstract (🇫🇷):  Low certainty, high flexibility → Max victim protection");
    println!("   Precedent (🇺🇸): Case-specific rules → Analogical reasoning required\n");

    println!("🏛️  Civil Law vs Common Law:");
    println!("   Civil Law:    General tort provisions cover wide range");
    println!("   Common Law:   Separate torts (battery, negligence, IIED, etc.)");
    println!("                 Each with distinct elements and precedents");
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_de::{bgb_823_1, bgb_826};
    use legalis_fr::article_1240;
    use legalis_jp::article_709;
    use legalis_us::{battery_as_statute, iied_as_statute, products_liability_as_statute};

    #[tokio::test]
    async fn test_jp_simulation_produces_results() {
        let metrics = build_and_run_sim(vec![article_709()], 10).await;
        assert!(
            metrics.total_applications > 0,
            "Japan simulation must apply statutes to at least one entity"
        );
    }

    #[tokio::test]
    async fn test_de_simulation_produces_results() {
        let metrics = build_and_run_sim(vec![bgb_823_1(), bgb_826()], 10).await;
        assert!(
            metrics.total_applications > 0,
            "Germany simulation must apply statutes to at least one entity"
        );
    }

    #[tokio::test]
    async fn test_fr_simulation_produces_results() {
        let metrics = build_and_run_sim(vec![article_1240()], 10).await;
        assert!(
            metrics.total_applications > 0,
            "France simulation must apply statutes to at least one entity"
        );
    }

    #[tokio::test]
    async fn test_us_simulation_produces_results() {
        let metrics = build_and_run_sim(
            vec![
                battery_as_statute(),
                iied_as_statute(),
                products_liability_as_statute(),
            ],
            10,
        )
        .await;
        assert!(
            metrics.total_applications > 0,
            "US simulation must apply statutes to at least one entity"
        );
    }

    #[tokio::test]
    async fn test_all_jurisdictions_in_parallel() {
        let (metrics_jp, metrics_de, metrics_fr, metrics_us) = tokio::join!(
            build_and_run_sim(vec![article_709()], 10),
            build_and_run_sim(vec![bgb_823_1(), bgb_826()], 10),
            build_and_run_sim(vec![article_1240()], 10),
            build_and_run_sim(
                vec![
                    battery_as_statute(),
                    iied_as_statute(),
                    products_liability_as_statute()
                ],
                10
            ),
        );

        assert!(
            metrics_jp.total_applications > 0,
            "JP total_applications must be > 0"
        );
        assert!(
            metrics_de.total_applications > 0,
            "DE total_applications must be > 0"
        );
        assert!(
            metrics_fr.total_applications > 0,
            "FR total_applications must be > 0"
        );
        assert!(
            metrics_us.total_applications > 0,
            "US total_applications must be > 0"
        );
    }
}
