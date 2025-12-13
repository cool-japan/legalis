//! Example: Japanese Civil Code Article 709 - Tort Liability Simulation
//!
//! This example demonstrates how to use Legalis-RS to simulate tort law cases
//! under Article 709 of the Japanese Civil Code.
//!
//! ## Scenarios Tested
//!
//! 1. **Clear Intent (明確な故意)**: Intentional harm with clear liability
//! 2. **Negligence (過失)**: Negligent conduct causing harm
//! 3. **Borderline Case (境界事例)**: Requires judicial discretion
//! 4. **No Tort (不法行為なし)**: No liability (missing requirements)

use legalis_core::{BasicEntity, LegalEntity, LegalResult};
use legalis_jp::article_709;
use legalis_sim::SimEngine;

#[tokio::main]
async fn main() {
    println!("=== 民法第709条 不法行為シミュレーション ===");
    println!("=== Article 709 Tort Liability Simulation ===\n");

    let statute = article_709();

    // Display the statute structure
    println!("📜 Statute Structure:\n");
    println!("{}\n", statute);
    println!("{}", "=".repeat(80));

    // Test different scenarios
    println!("\n🧪 Scenario Testing\n");

    test_scenario_1_intentional_tort();
    test_scenario_2_negligence();
    test_scenario_3_borderline_case();
    test_scenario_4_no_tort();
    test_scenario_5_missing_causation();

    // Population simulation
    println!("\n{}", "=".repeat(80));
    println!("\n🌐 Population Simulation\n");
    run_population_simulation().await;
}

/// Scenario 1: Clear intentional tort (故意による明確な不法行為)
fn test_scenario_1_intentional_tort() {
    println!("📌 Scenario 1: Intentional Tort (故意の不法行為)");
    println!("   Facts: A punched B intentionally, causing injury");
    println!("   事実: Aが故意にBを殴打し、怪我を負わせた\n");

    let mut agent = BasicEntity::new();
    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("negligence", "false".to_string());
    agent.set_attribute("infringement", "true".to_string());
    agent.set_attribute("causation", "true".to_string());
    agent.set_attribute("damages_exist", "true".to_string());

    let result = SimEngine::apply_law(&agent, &article_709());

    print_result("Scenario 1", &result);
    println!();
}

/// Scenario 2: Negligence (過失による不法行為)
fn test_scenario_2_negligence() {
    println!("📌 Scenario 2: Negligent Tort (過失の不法行為)");
    println!("   Facts: Driver failed to check blind spot, causing accident");
    println!("   事実: 運転手が死角確認を怠り、事故を起こした\n");

    let mut agent = BasicEntity::new();
    agent.set_attribute("intent", "false".to_string());
    agent.set_attribute("negligence", "true".to_string());
    agent.set_attribute("infringement", "true".to_string());
    agent.set_attribute("causation", "true".to_string());
    agent.set_attribute("damages_exist", "true".to_string());

    let result = SimEngine::apply_law(&agent, &article_709());

    print_result("Scenario 2", &result);
    println!();
}

/// Scenario 3: Borderline case requiring judicial discretion
fn test_scenario_3_borderline_case() {
    println!("📌 Scenario 3: Borderline Case (境界的事例)");
    println!("   Facts: Unclear if conduct was negligent; requires judicial review");
    println!("   事実: 過失の有無が不明確; 司法判断が必要\n");

    let mut agent = BasicEntity::new();
    agent.set_attribute("intent", "false".to_string());
    agent.set_attribute("negligence", "unclear".to_string()); // Not "true"
    agent.set_attribute("infringement", "true".to_string());
    agent.set_attribute("causation", "true".to_string());
    agent.set_attribute("damages_exist", "true".to_string());

    let result = SimEngine::apply_law(&agent, &article_709());

    print_result("Scenario 3", &result);
    println!();
}

/// Scenario 4: No tort (all requirements not met)
fn test_scenario_4_no_tort() {
    println!("📌 Scenario 4: No Tort (不法行為なし)");
    println!("   Facts: No infringement occurred");
    println!("   事実: 権利侵害なし\n");

    let mut agent = BasicEntity::new();
    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("negligence", "false".to_string());
    agent.set_attribute("infringement", "false".to_string()); // No infringement!
    agent.set_attribute("causation", "true".to_string());
    agent.set_attribute("damages_exist", "true".to_string());

    let result = SimEngine::apply_law(&agent, &article_709());

    print_result("Scenario 4", &result);
    println!();
}

/// Scenario 5: Missing causation
fn test_scenario_5_missing_causation() {
    println!("📌 Scenario 5: Missing Causation (因果関係なし)");
    println!("   Facts: Damages exist but not caused by defendant's conduct");
    println!("   事実: 損害はあるが、被告の行為との因果関係なし\n");

    let mut agent = BasicEntity::new();
    agent.set_attribute("intent", "true".to_string());
    agent.set_attribute("negligence", "false".to_string());
    agent.set_attribute("infringement", "true".to_string());
    agent.set_attribute("causation", "false".to_string()); // No causation!
    agent.set_attribute("damages_exist", "true".to_string());

    let result = SimEngine::apply_law(&agent, &article_709());

    print_result("Scenario 5", &result);
    println!();
}

fn print_result(_scenario: &str, result: &LegalResult<legalis_core::Effect>) {
    match result {
        LegalResult::Deterministic(effect) => {
            println!("   ✅ Result: DETERMINISTIC");
            println!("   Effect: {}", effect);
            println!("   ⚖️  Outcome: Tortfeasor is LIABLE for damages (損害賠償責任あり)");
        }
        LegalResult::JudicialDiscretion {
            issue,
            narrative_hint,
            ..
        } => {
            println!("   ⚠️  Result: REQUIRES JUDICIAL DISCRETION");
            println!("   Issue: {}", issue);
            if let Some(hint) = narrative_hint {
                println!("   Guidance:");
                for line in hint.lines() {
                    println!("     {}", line);
                }
            }
        }
        LegalResult::Void { reason } => {
            println!("   ❌ Result: NO LIABILITY (Precondition not met)");
            println!("   Reason: {}", reason);
        }
    }
}

/// Run a population simulation with mixed cases
async fn run_population_simulation() {
    println!("Running simulation with 5 agents across different tort scenarios...\n");

    // Create population with different profiles
    let mut population: Vec<Box<dyn LegalEntity>> = Vec::new();

    // Agent 1: Clear intentional tort
    let mut agent1 = BasicEntity::new();
    agent1.set_attribute("name", "Agent 1 (Intentional)".to_string());
    agent1.set_attribute("intent", "true".to_string());
    agent1.set_attribute("infringement", "true".to_string());
    agent1.set_attribute("causation", "true".to_string());
    agent1.set_attribute("damages_exist", "true".to_string());
    population.push(Box::new(agent1));

    // Agent 2: Negligence
    let mut agent2 = BasicEntity::new();
    agent2.set_attribute("name", "Agent 2 (Negligence)".to_string());
    agent2.set_attribute("negligence", "true".to_string());
    agent2.set_attribute("infringement", "true".to_string());
    agent2.set_attribute("causation", "true".to_string());
    agent2.set_attribute("damages_exist", "true".to_string());
    population.push(Box::new(agent2));

    // Agent 3: Borderline (unclear negligence)
    let mut agent3 = BasicEntity::new();
    agent3.set_attribute("name", "Agent 3 (Borderline)".to_string());
    agent3.set_attribute("negligence", "unclear".to_string());
    agent3.set_attribute("infringement", "true".to_string());
    agent3.set_attribute("causation", "true".to_string());
    agent3.set_attribute("damages_exist", "true".to_string());
    population.push(Box::new(agent3));

    // Agent 4: No tort (no infringement)
    let mut agent4 = BasicEntity::new();
    agent4.set_attribute("name", "Agent 4 (No Infringement)".to_string());
    agent4.set_attribute("intent", "true".to_string());
    agent4.set_attribute("infringement", "false".to_string());
    agent4.set_attribute("causation", "true".to_string());
    agent4.set_attribute("damages_exist", "true".to_string());
    population.push(Box::new(agent4));

    // Agent 5: No causation
    let mut agent5 = BasicEntity::new();
    agent5.set_attribute("name", "Agent 5 (No Causation)".to_string());
    agent5.set_attribute("intent", "true".to_string());
    agent5.set_attribute("infringement", "true".to_string());
    agent5.set_attribute("causation", "false".to_string());
    agent5.set_attribute("damages_exist", "true".to_string());
    population.push(Box::new(agent5));

    let statutes = vec![article_709()];
    let engine = SimEngine::new(statutes, population);

    println!("Population size: {}", engine.population_size());
    println!("Statutes: {}", engine.statute_count());
    println!();

    let metrics = engine.run_simulation().await;

    println!("📊 Simulation Results:\n");
    println!("Total applications: {}", metrics.total_applications);
    println!("Deterministic outcomes: {}", metrics.deterministic_count);
    println!("Judicial discretion required: {}", metrics.discretion_count);
    println!("Void/Invalid cases: {}", metrics.void_count);

    let rate = metrics.deterministic_ratio();
    println!("\nDeterministic rate: {:.1}%", rate * 100.0);

    println!("\n💡 Interpretation:");
    println!(
        "   - {} cases have clear liability (deterministic)",
        metrics.deterministic_count
    );
    println!(
        "   - {} cases require judicial review (discretion)",
        metrics.discretion_count
    );
    println!("   - {} cases have no liability (void)", metrics.void_count);
    println!("\n   This demonstrates Legalis-RS's core philosophy:");
    println!("   \"計算可能性と裁量の分離\" (Separation of computation and discretion)");
}
