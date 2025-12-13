//! Simulation metrics collection and reporting.

use crate::engine::LawApplicationResult;
use legalis_core::LegalResult;
use std::collections::HashMap;
use uuid::Uuid;

/// Metrics collected during simulation.
#[derive(Debug, Clone, Default)]
pub struct SimulationMetrics {
    /// Total number of law applications
    pub total_applications: usize,
    /// Number of deterministic results
    pub deterministic_count: usize,
    /// Number of discretionary results
    pub discretion_count: usize,
    /// Number of void results
    pub void_count: usize,
    /// Per-statute metrics
    pub statute_metrics: HashMap<String, StatuteMetrics>,
    /// Agents that triggered discretionary review
    pub discretion_agents: Vec<Uuid>,
}

impl SimulationMetrics {
    /// Creates a new metrics collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a single law application result.
    pub fn record_result(&mut self, result: &LawApplicationResult) {
        self.total_applications += 1;

        let statute_metrics = self
            .statute_metrics
            .entry(result.statute_id.clone())
            .or_default();

        statute_metrics.total += 1;

        match &result.result {
            LegalResult::Deterministic(_) => {
                self.deterministic_count += 1;
                statute_metrics.deterministic += 1;
            }
            LegalResult::JudicialDiscretion { .. } => {
                self.discretion_count += 1;
                statute_metrics.discretion += 1;
                self.discretion_agents.push(result.agent_id);
            }
            LegalResult::Void { .. } => {
                self.void_count += 1;
                statute_metrics.void += 1;
            }
        }
    }

    /// Returns the percentage of deterministic outcomes.
    pub fn deterministic_ratio(&self) -> f64 {
        if self.total_applications == 0 {
            0.0
        } else {
            self.deterministic_count as f64 / self.total_applications as f64
        }
    }

    /// Returns the percentage of discretionary outcomes.
    pub fn discretion_ratio(&self) -> f64 {
        if self.total_applications == 0 {
            0.0
        } else {
            self.discretion_count as f64 / self.total_applications as f64
        }
    }

    /// Generates a summary report.
    pub fn summary(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Simulation Summary ===\n");
        report.push_str(&format!(
            "Total applications: {}\n",
            self.total_applications
        ));
        report.push_str(&format!(
            "Deterministic: {} ({:.1}%)\n",
            self.deterministic_count,
            self.deterministic_ratio() * 100.0
        ));
        report.push_str(&format!(
            "Discretionary: {} ({:.1}%)\n",
            self.discretion_count,
            self.discretion_ratio() * 100.0
        ));
        report.push_str(&format!("Void: {}\n", self.void_count));
        report.push_str("\n=== Per-Statute Breakdown ===\n");

        for (statute_id, metrics) in &self.statute_metrics {
            report.push_str(&format!(
                "{}: D={} / J={} / V={}\n",
                statute_id, metrics.deterministic, metrics.discretion, metrics.void
            ));
        }

        report
    }
}

/// Metrics for a single statute.
#[derive(Debug, Clone, Default)]
pub struct StatuteMetrics {
    /// Total applications of this statute
    pub total: usize,
    /// Deterministic outcomes
    pub deterministic: usize,
    /// Discretionary outcomes
    pub discretion: usize,
    /// Void outcomes
    pub void: usize,
}

impl StatuteMetrics {
    /// Returns the effectiveness ratio (deterministic / total).
    pub fn effectiveness(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.deterministic as f64 / self.total as f64
        }
    }

    /// Returns the ambiguity ratio (discretion / total).
    pub fn ambiguity(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.discretion as f64 / self.total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::Effect;

    #[test]
    fn test_metrics_recording() {
        let mut metrics = SimulationMetrics::new();

        metrics.record_result(&LawApplicationResult {
            agent_id: Uuid::new_v4(),
            statute_id: "test-1".to_string(),
            result: LegalResult::Deterministic(Effect::new(
                legalis_core::EffectType::Grant,
                "Test",
            )),
        });

        metrics.record_result(&LawApplicationResult {
            agent_id: Uuid::new_v4(),
            statute_id: "test-1".to_string(),
            result: LegalResult::JudicialDiscretion {
                issue: "Test issue".to_string(),
                context_id: Uuid::new_v4(),
                narrative_hint: None,
            },
        });

        assert_eq!(metrics.total_applications, 2);
        assert_eq!(metrics.deterministic_count, 1);
        assert_eq!(metrics.discretion_count, 1);
        assert!((metrics.deterministic_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_statute_metrics() {
        let metrics = StatuteMetrics {
            total: 100,
            deterministic: 80,
            discretion: 15,
            void: 5,
        };

        assert!((metrics.effectiveness() - 0.8).abs() < f64::EPSILON);
        assert!((metrics.ambiguity() - 0.15).abs() < f64::EPSILON);
    }
}
