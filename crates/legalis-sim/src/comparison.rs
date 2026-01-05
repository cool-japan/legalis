//! Statute comparison and A/B testing tools.
//!
//! This module provides:
//! - Statute version comparison
//! - A/B testing framework
//! - Sensitivity analysis
//! - Impact differential analysis

use crate::metrics::SimulationMetrics;
use legalis_core::{LegalEntity, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comparison result between two statute versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteComparison {
    /// ID of the statute being compared
    pub statute_id: String,
    /// Name of version A
    pub version_a_name: String,
    /// Name of version B
    pub version_b_name: String,
    /// Metrics for version A
    pub metrics_a: SimulationMetrics,
    /// Metrics for version B
    pub metrics_b: SimulationMetrics,
    /// Differential analysis
    pub differential: DifferentialAnalysis,
}

impl StatuteComparison {
    /// Creates a new statute comparison.
    pub fn new(
        statute_id: String,
        version_a_name: String,
        version_b_name: String,
        metrics_a: SimulationMetrics,
        metrics_b: SimulationMetrics,
    ) -> Self {
        let differential = DifferentialAnalysis::compute(&metrics_a, &metrics_b);
        Self {
            statute_id,
            version_a_name,
            version_b_name,
            metrics_a,
            metrics_b,
            differential,
        }
    }

    /// Generates a comparison report.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!(
            "=== Statute Comparison: {} ===\n",
            self.statute_id
        ));
        report.push_str(&format!("Version A: {}\n", self.version_a_name));
        report.push_str(&format!("Version B: {}\n\n", self.version_b_name));

        report.push_str("Metrics:\n");
        report.push_str(&format!(
            "  Deterministic Rate: {:.1}% → {:.1}% ({:+.1}%)\n",
            self.metrics_a.deterministic_ratio() * 100.0,
            self.metrics_b.deterministic_ratio() * 100.0,
            self.differential.deterministic_delta * 100.0
        ));
        report.push_str(&format!(
            "  Discretion Rate: {:.1}% → {:.1}% ({:+.1}%)\n",
            self.metrics_a.discretion_ratio() * 100.0,
            self.metrics_b.discretion_ratio() * 100.0,
            self.differential.discretion_delta * 100.0
        ));
        report.push_str(&format!(
            "  Void Rate: {:.1}% → {:.1}% ({:+.1}%)\n",
            (self.metrics_a.void_count as f64 / self.metrics_a.total_applications as f64) * 100.0,
            (self.metrics_b.void_count as f64 / self.metrics_b.total_applications as f64) * 100.0,
            self.differential.void_delta * 100.0
        ));

        report.push_str(&format!(
            "\nImpact: {}\n",
            self.differential.impact_summary()
        ));

        report
    }

    /// Returns whether version B is an improvement over version A.
    pub fn is_improvement(&self) -> bool {
        // More deterministic and less discretion is generally better
        self.differential.deterministic_delta > 0.0 && self.differential.discretion_delta < 0.0
    }
}

/// Differential analysis between two sets of metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialAnalysis {
    /// Change in deterministic ratio (B - A)
    pub deterministic_delta: f64,
    /// Change in discretion ratio (B - A)
    pub discretion_delta: f64,
    /// Change in void ratio (B - A)
    pub void_delta: f64,
    /// Magnitude of change (0.0 to 1.0)
    pub magnitude: f64,
}

impl DifferentialAnalysis {
    /// Computes differential analysis.
    pub fn compute(metrics_a: &SimulationMetrics, metrics_b: &SimulationMetrics) -> Self {
        let deterministic_delta = metrics_b.deterministic_ratio() - metrics_a.deterministic_ratio();
        let discretion_delta = metrics_b.discretion_ratio() - metrics_a.discretion_ratio();
        let void_a = metrics_a.void_count as f64 / metrics_a.total_applications.max(1) as f64;
        let void_b = metrics_b.void_count as f64 / metrics_b.total_applications.max(1) as f64;
        let void_delta = void_b - void_a;

        let magnitude =
            (deterministic_delta.abs() + discretion_delta.abs() + void_delta.abs()) / 3.0;

        Self {
            deterministic_delta,
            discretion_delta,
            void_delta,
            magnitude,
        }
    }

    /// Generates an impact summary.
    pub fn impact_summary(&self) -> String {
        if self.magnitude < 0.01 {
            "Negligible change".to_string()
        } else if self.magnitude < 0.05 {
            "Minor change".to_string()
        } else if self.magnitude < 0.15 {
            "Moderate change".to_string()
        } else if self.magnitude < 0.30 {
            "Significant change".to_string()
        } else {
            "Major change".to_string()
        }
    }
}

/// A/B test configuration.
pub struct ABTest {
    /// Name of the A/B test
    pub name: String,
    /// Version A statute
    pub version_a: Statute,
    /// Version B statute
    pub version_b: Statute,
    /// Population to test against
    pub population: Vec<Box<dyn LegalEntity>>,
}

impl ABTest {
    /// Creates a new A/B test.
    pub fn new(
        name: String,
        version_a: Statute,
        version_b: Statute,
        population: Vec<Box<dyn LegalEntity>>,
    ) -> Self {
        Self {
            name,
            version_a,
            version_b,
            population,
        }
    }

    /// Runs the A/B test and returns comparison.
    pub async fn run(&self) -> StatuteComparison {
        use crate::engine::{LawApplicationResult, SimEngine};

        // Run simulation with version A
        let mut metrics_a = SimulationMetrics::new();
        for agent in &self.population {
            let result = SimEngine::apply_law(agent.as_ref(), &self.version_a);
            metrics_a.record_result(&LawApplicationResult {
                agent_id: agent.id(),
                statute_id: self.version_a.id.clone(),
                result,
            });
        }

        // Run simulation with version B
        let mut metrics_b = SimulationMetrics::new();
        for agent in &self.population {
            let result = SimEngine::apply_law(agent.as_ref(), &self.version_b);
            metrics_b.record_result(&LawApplicationResult {
                agent_id: agent.id(),
                statute_id: self.version_b.id.clone(),
                result,
            });
        }

        StatuteComparison::new(
            self.version_a.id.clone(),
            "Version A".to_string(),
            "Version B".to_string(),
            metrics_a,
            metrics_b,
        )
    }
}

/// Sensitivity analysis for statute parameters.
#[derive(Debug, Clone)]
pub struct SensitivityAnalysis {
    /// Base statute
    pub base_statute: Statute,
    /// Parameter variations to test
    pub variations: Vec<(String, Statute)>,
}

impl SensitivityAnalysis {
    /// Creates a new sensitivity analysis.
    pub fn new(base_statute: Statute) -> Self {
        Self {
            base_statute,
            variations: Vec::new(),
        }
    }

    /// Adds a parameter variation.
    pub fn add_variation(mut self, name: String, statute: Statute) -> Self {
        self.variations.push((name, statute));
        self
    }

    /// Runs sensitivity analysis across all variations.
    pub async fn run(&self, population: &[Box<dyn LegalEntity>]) -> SensitivityResults {
        use crate::engine::{LawApplicationResult, SimEngine};

        let mut results = HashMap::new();

        // Test base statute
        let mut base_metrics = SimulationMetrics::new();
        for entity in population {
            let result = SimEngine::apply_law(entity.as_ref(), &self.base_statute);
            base_metrics.record_result(&LawApplicationResult {
                agent_id: entity.id(),
                statute_id: self.base_statute.id.clone(),
                result,
            });
        }

        results.insert("Base".to_string(), base_metrics.clone());

        // Test each variation
        for (name, statute) in &self.variations {
            let mut metrics = SimulationMetrics::new();
            for entity in population {
                let result = SimEngine::apply_law(entity.as_ref(), statute);
                metrics.record_result(&LawApplicationResult {
                    agent_id: entity.id(),
                    statute_id: statute.id.clone(),
                    result,
                });
            }
            results.insert(name.clone(), metrics);
        }

        SensitivityResults {
            base_metrics,
            variation_metrics: results,
        }
    }
}

/// Results of sensitivity analysis.
#[derive(Debug, Clone)]
pub struct SensitivityResults {
    /// Metrics for base statute
    pub base_metrics: SimulationMetrics,
    /// Metrics for each variation
    pub variation_metrics: HashMap<String, SimulationMetrics>,
}

impl SensitivityResults {
    /// Generates a sensitivity report.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Sensitivity Analysis ===\n\n");

        report.push_str(&format!(
            "Base: D={:.1}% J={:.1}% V={:.1}%\n\n",
            self.base_metrics.deterministic_ratio() * 100.0,
            self.base_metrics.discretion_ratio() * 100.0,
            (self.base_metrics.void_count as f64
                / self.base_metrics.total_applications.max(1) as f64)
                * 100.0
        ));

        report.push_str("Variations:\n");
        for (name, metrics) in &self.variation_metrics {
            if name != "Base" {
                let diff = DifferentialAnalysis::compute(&self.base_metrics, metrics);
                report.push_str(&format!(
                    "  {}: D={:.1}% ({:+.1}%) J={:.1}% ({:+.1}%) - {}\n",
                    name,
                    metrics.deterministic_ratio() * 100.0,
                    diff.deterministic_delta * 100.0,
                    metrics.discretion_ratio() * 100.0,
                    diff.discretion_delta * 100.0,
                    diff.impact_summary()
                ));
            }
        }

        report
    }

    /// Finds the best performing variation.
    pub fn best_variation(&self) -> Option<String> {
        self.variation_metrics
            .iter()
            .filter(|(name, _)| *name != "Base")
            .max_by(|(_, metrics_a), (_, metrics_b)| {
                metrics_a
                    .deterministic_ratio()
                    .partial_cmp(&metrics_b.deterministic_ratio())
                    .unwrap()
            })
            .map(|(name, _)| name.clone())
    }
}

/// Batch simulation runner for executing multiple scenarios efficiently.
#[derive(Default)]
pub struct BatchSimulationRunner {
    /// Named scenarios to run
    scenarios: Vec<(String, Vec<Statute>)>,
}

impl BatchSimulationRunner {
    /// Creates a new batch runner.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a scenario to run.
    pub fn add_scenario(mut self, name: impl Into<String>, statutes: Vec<Statute>) -> Self {
        self.scenarios.push((name.into(), statutes));
        self
    }

    /// Runs all scenarios with the given population and returns results.
    pub async fn run_all(self, population: &[&dyn LegalEntity]) -> BatchSimulationResults {
        let mut results = HashMap::new();

        for (name, statutes) in self.scenarios {
            let mut metrics = crate::SimulationMetrics::new();
            for entity in population {
                for statute in &statutes {
                    let result = crate::SimEngine::apply_law(*entity, statute);
                    metrics.record_result(&crate::LawApplicationResult {
                        agent_id: entity.id(),
                        statute_id: statute.id.clone(),
                        result,
                    });
                }
            }
            results.insert(name, metrics);
        }

        BatchSimulationResults { results }
    }
}

/// Results from batch simulation.
#[derive(Debug, Clone)]
pub struct BatchSimulationResults {
    /// Results for each scenario
    pub results: HashMap<String, SimulationMetrics>,
}

impl BatchSimulationResults {
    /// Generates a comparison report across all scenarios.
    pub fn comparison_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Batch Simulation Results ===\n\n");

        let mut sorted_scenarios: Vec<_> = self.results.iter().collect();
        sorted_scenarios.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (name, metrics) in sorted_scenarios {
            report.push_str(&format!(
                "{}: {} applications | D={:.1}% | J={:.1}% | V={:.1}%\n",
                name,
                metrics.total_applications,
                metrics.deterministic_ratio() * 100.0,
                metrics.discretion_ratio() * 100.0,
                (metrics.void_count as f64 / metrics.total_applications.max(1) as f64) * 100.0
            ));
        }

        report
    }

    /// Finds the scenario with highest deterministic ratio.
    pub fn best_scenario(&self) -> Option<(&String, &SimulationMetrics)> {
        self.results.iter().max_by(|(_, a), (_, b)| {
            a.deterministic_ratio()
                .partial_cmp(&b.deterministic_ratio())
                .unwrap()
        })
    }

    /// Exports results to a structured format for further analysis.
    pub fn to_table(&self) -> Vec<BatchResultRow> {
        self.results
            .iter()
            .map(|(name, metrics)| BatchResultRow {
                scenario_name: name.clone(),
                total_applications: metrics.total_applications,
                deterministic_count: metrics.deterministic_count,
                discretion_count: metrics.discretion_count,
                void_count: metrics.void_count,
                deterministic_ratio: metrics.deterministic_ratio(),
                discretion_ratio: metrics.discretion_ratio(),
            })
            .collect()
    }
}

/// A single row in the batch results table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResultRow {
    pub scenario_name: String,
    pub total_applications: usize,
    pub deterministic_count: usize,
    pub discretion_count: usize,
    pub void_count: usize,
    pub deterministic_ratio: f64,
    pub discretion_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{BasicEntity, ComparisonOp, Condition, Effect, EffectType};

    fn create_test_population(size: usize) -> Vec<Box<dyn LegalEntity>> {
        (0..size)
            .map(|i| {
                let mut entity = BasicEntity::new();
                entity.set_attribute("age", ((i % 80) + 18).to_string());
                entity.set_attribute("income", ((i % 10) * 10000 + 20000).to_string());
                Box::new(entity) as Box<dyn LegalEntity>
            })
            .collect()
    }

    #[tokio::test]
    async fn test_ab_test() {
        let version_a = Statute::new(
            "voting-age",
            "Voting Age V1",
            Effect::new(EffectType::Grant, "Can vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let version_b = Statute::new(
            "voting-age",
            "Voting Age V2",
            Effect::new(EffectType::Grant, "Can vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let population = create_test_population(100);
        let ab_test = ABTest::new(
            "Voting Age Test".to_string(),
            version_a,
            version_b,
            population,
        );

        let comparison = ab_test.run().await;

        // Version A (age 18) should have more deterministic results
        assert!(
            comparison.metrics_a.deterministic_count > comparison.metrics_b.deterministic_count
        );
        assert!(comparison.differential.magnitude > 0.0);
    }

    #[tokio::test]
    async fn test_sensitivity_analysis() {
        let base = Statute::new(
            "tax",
            "Tax Law",
            Effect::new(EffectType::Obligation, "Pay tax"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::GreaterThan,
            value: 50000,
        });

        let variation1 = Statute::new(
            "tax",
            "Tax Law - Lower Threshold",
            Effect::new(EffectType::Obligation, "Pay tax"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::GreaterThan,
            value: 30000,
        });

        let variation2 = Statute::new(
            "tax",
            "Tax Law - Higher Threshold",
            Effect::new(EffectType::Obligation, "Pay tax"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::GreaterThan,
            value: 70000,
        });

        let analysis = SensitivityAnalysis::new(base)
            .add_variation("Lower threshold (30k)".to_string(), variation1)
            .add_variation("Higher threshold (70k)".to_string(), variation2);

        let population = create_test_population(100);
        let results = analysis.run(&population).await;

        assert!(results.variation_metrics.len() >= 3); // Base + 2 variations
        assert!(results.best_variation().is_some());
    }

    #[test]
    fn test_differential_analysis() {
        let mut metrics_a = SimulationMetrics::new();
        let mut metrics_b = SimulationMetrics::new();

        use crate::engine::LawApplicationResult;
        use uuid::Uuid;

        // Metrics A: 80% deterministic
        for _ in 0..80 {
            metrics_a.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: legalis_core::LegalResult::Deterministic(Effect::new(
                    EffectType::Grant,
                    "Test",
                )),
            });
        }
        for _ in 0..20 {
            metrics_a.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: legalis_core::LegalResult::JudicialDiscretion {
                    issue: "Test".to_string(),
                    context_id: Uuid::new_v4(),
                    narrative_hint: None,
                },
            });
        }

        // Metrics B: 90% deterministic
        for _ in 0..90 {
            metrics_b.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: legalis_core::LegalResult::Deterministic(Effect::new(
                    EffectType::Grant,
                    "Test",
                )),
            });
        }
        for _ in 0..10 {
            metrics_b.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: legalis_core::LegalResult::JudicialDiscretion {
                    issue: "Test".to_string(),
                    context_id: Uuid::new_v4(),
                    narrative_hint: None,
                },
            });
        }

        let diff = DifferentialAnalysis::compute(&metrics_a, &metrics_b);

        assert!((diff.deterministic_delta - 0.1).abs() < 0.01); // +10% deterministic
        assert!((diff.discretion_delta + 0.1).abs() < 0.01); // -10% discretion
        assert!(diff.magnitude > 0.0);
    }

    #[tokio::test]
    async fn test_batch_simulation_runner() {
        let population = create_test_population(50);
        let pop_refs: Vec<&dyn LegalEntity> = population.iter().map(|e| e.as_ref()).collect();

        let statute1 = Statute::new(
            "test1",
            "Test Statute 1",
            Effect::new(EffectType::Grant, "Grant 1"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let statute2 = Statute::new(
            "test2",
            "Test Statute 2",
            Effect::new(EffectType::Grant, "Grant 2"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let runner = BatchSimulationRunner::new()
            .add_scenario("Scenario 1", vec![statute1])
            .add_scenario("Scenario 2", vec![statute2]);

        let results = runner.run_all(&pop_refs).await;

        assert_eq!(results.results.len(), 2);
        assert!(results.results.contains_key("Scenario 1"));
        assert!(results.results.contains_key("Scenario 2"));
        assert!(results.best_scenario().is_some());
    }

    #[test]
    fn test_batch_results_to_table() {
        let mut results = HashMap::new();
        let mut metrics = SimulationMetrics::new();
        metrics.total_applications = 100;
        metrics.deterministic_count = 80;
        metrics.discretion_count = 15;
        metrics.void_count = 5;

        results.insert("Test Scenario".to_string(), metrics);

        let batch_results = BatchSimulationResults { results };
        let table = batch_results.to_table();

        assert_eq!(table.len(), 1);
        assert_eq!(table[0].scenario_name, "Test Scenario");
        assert_eq!(table[0].total_applications, 100);
        assert_eq!(table[0].deterministic_count, 80);
    }

    #[test]
    fn test_batch_results_comparison_report() {
        let mut results = HashMap::new();
        let mut metrics1 = SimulationMetrics::new();
        metrics1.total_applications = 100;
        metrics1.deterministic_count = 80;

        let mut metrics2 = SimulationMetrics::new();
        metrics2.total_applications = 100;
        metrics2.deterministic_count = 90;

        results.insert("Scenario A".to_string(), metrics1);
        results.insert("Scenario B".to_string(), metrics2);

        let batch_results = BatchSimulationResults { results };
        let report = batch_results.comparison_report();

        assert!(report.contains("Batch Simulation Results"));
        assert!(report.contains("Scenario A"));
        assert!(report.contains("Scenario B"));
    }
}
