//! Risk analysis for policy outcomes.
//!
//! This module provides tools for analyzing uncertainty and risk in simulation results,
//! including Value at Risk (VaR), Conditional VaR, and various risk metrics.

use crate::{SimResult, SimulationMetrics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Value at Risk (VaR) calculation for policy outcomes.
///
/// VaR represents the maximum expected loss at a given confidence level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueAtRisk {
    /// Confidence level (e.g., 0.95 for 95% VaR)
    pub confidence_level: f64,
    /// VaR value for deterministic ratio
    pub var_deterministic: f64,
    /// VaR value for discretion ratio
    pub var_discretion: f64,
    /// Sample size used for calculation
    pub sample_size: usize,
}

impl ValueAtRisk {
    /// Calculates VaR from multiple simulation runs.
    ///
    /// # Arguments
    /// * `metrics` - Multiple simulation results
    /// * `confidence_level` - Confidence level (e.g., 0.95 for 95% VaR)
    pub fn calculate(metrics: &[SimulationMetrics], confidence_level: f64) -> SimResult<Self> {
        if metrics.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Cannot calculate VaR with empty metrics".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&confidence_level) {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Confidence level must be between 0 and 1".to_string(),
            ));
        }

        let mut det_ratios: Vec<f64> = metrics.iter().map(|m| m.deterministic_ratio()).collect();
        let mut disc_ratios: Vec<f64> = metrics.iter().map(|m| m.discretion_ratio()).collect();

        det_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
        disc_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = ((1.0 - confidence_level) * det_ratios.len() as f64) as usize;
        let index = index.min(det_ratios.len().saturating_sub(1));

        Ok(Self {
            confidence_level,
            var_deterministic: det_ratios[index],
            var_discretion: disc_ratios[index],
            sample_size: metrics.len(),
        })
    }
}

/// Conditional Value at Risk (CVaR), also known as Expected Shortfall.
///
/// CVaR represents the expected loss given that the loss exceeds VaR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalVaR {
    /// Confidence level
    pub confidence_level: f64,
    /// CVaR value for deterministic ratio
    pub cvar_deterministic: f64,
    /// CVaR value for discretion ratio
    pub cvar_discretion: f64,
    /// Sample size used for calculation
    pub sample_size: usize,
}

impl ConditionalVaR {
    /// Calculates CVaR from multiple simulation runs.
    ///
    /// # Arguments
    /// * `metrics` - Multiple simulation results
    /// * `confidence_level` - Confidence level (e.g., 0.95 for 95% CVaR)
    pub fn calculate(metrics: &[SimulationMetrics], confidence_level: f64) -> SimResult<Self> {
        if metrics.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Cannot calculate CVaR with empty metrics".to_string(),
            ));
        }

        let mut det_ratios: Vec<f64> = metrics.iter().map(|m| m.deterministic_ratio()).collect();
        let mut disc_ratios: Vec<f64> = metrics.iter().map(|m| m.discretion_ratio()).collect();

        det_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
        disc_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let cutoff_index = ((1.0 - confidence_level) * det_ratios.len() as f64) as usize;
        let cutoff_index = cutoff_index.min(det_ratios.len().saturating_sub(1));

        let cvar_det = if cutoff_index > 0 {
            det_ratios[..cutoff_index].iter().sum::<f64>() / cutoff_index as f64
        } else {
            det_ratios[0]
        };

        let cvar_disc = if cutoff_index > 0 {
            disc_ratios[..cutoff_index].iter().sum::<f64>() / cutoff_index as f64
        } else {
            disc_ratios[0]
        };

        Ok(Self {
            confidence_level,
            cvar_deterministic: cvar_det,
            cvar_discretion: cvar_disc,
            sample_size: metrics.len(),
        })
    }
}

/// Comprehensive risk metrics for simulation results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    /// Mean deterministic ratio
    pub mean_deterministic: f64,
    /// Mean discretion ratio
    pub mean_discretion: f64,
    /// Standard deviation of deterministic ratio (volatility)
    pub std_dev_deterministic: f64,
    /// Standard deviation of discretion ratio
    pub std_dev_discretion: f64,
    /// Skewness of deterministic ratio
    pub skewness_deterministic: f64,
    /// Skewness of discretion ratio
    pub skewness_discretion: f64,
    /// Kurtosis of deterministic ratio
    pub kurtosis_deterministic: f64,
    /// Kurtosis of discretion ratio
    pub kurtosis_discretion: f64,
    /// Sample size
    pub sample_size: usize,
}

impl RiskMetrics {
    /// Calculates comprehensive risk metrics from multiple simulations.
    pub fn calculate(metrics: &[SimulationMetrics]) -> SimResult<Self> {
        if metrics.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Cannot calculate risk metrics with empty data".to_string(),
            ));
        }

        let det_ratios: Vec<f64> = metrics.iter().map(|m| m.deterministic_ratio()).collect();
        let disc_ratios: Vec<f64> = metrics.iter().map(|m| m.discretion_ratio()).collect();

        let mean_det = det_ratios.iter().sum::<f64>() / det_ratios.len() as f64;
        let mean_disc = disc_ratios.iter().sum::<f64>() / disc_ratios.len() as f64;

        let variance_det = det_ratios
            .iter()
            .map(|x| (x - mean_det).powi(2))
            .sum::<f64>()
            / det_ratios.len() as f64;
        let variance_disc = disc_ratios
            .iter()
            .map(|x| (x - mean_disc).powi(2))
            .sum::<f64>()
            / disc_ratios.len() as f64;

        let std_dev_det = variance_det.sqrt();
        let std_dev_disc = variance_disc.sqrt();

        let skewness_det = if std_dev_det > 0.0 {
            det_ratios
                .iter()
                .map(|x| ((x - mean_det) / std_dev_det).powi(3))
                .sum::<f64>()
                / det_ratios.len() as f64
        } else {
            0.0
        };

        let skewness_disc = if std_dev_disc > 0.0 {
            disc_ratios
                .iter()
                .map(|x| ((x - mean_disc) / std_dev_disc).powi(3))
                .sum::<f64>()
                / disc_ratios.len() as f64
        } else {
            0.0
        };

        let kurtosis_det = if std_dev_det > 0.0 {
            det_ratios
                .iter()
                .map(|x| ((x - mean_det) / std_dev_det).powi(4))
                .sum::<f64>()
                / det_ratios.len() as f64
                - 3.0 // Excess kurtosis
        } else {
            0.0
        };

        let kurtosis_disc = if std_dev_disc > 0.0 {
            disc_ratios
                .iter()
                .map(|x| ((x - mean_disc) / std_dev_disc).powi(4))
                .sum::<f64>()
                / disc_ratios.len() as f64
                - 3.0 // Excess kurtosis
        } else {
            0.0
        };

        Ok(Self {
            mean_deterministic: mean_det,
            mean_discretion: mean_disc,
            std_dev_deterministic: std_dev_det,
            std_dev_discretion: std_dev_disc,
            skewness_deterministic: skewness_det,
            skewness_discretion: skewness_disc,
            kurtosis_deterministic: kurtosis_det,
            kurtosis_discretion: kurtosis_disc,
            sample_size: metrics.len(),
        })
    }

    /// Returns the coefficient of variation for deterministic ratio.
    pub fn cv_deterministic(&self) -> f64 {
        if self.mean_deterministic > 0.0 {
            self.std_dev_deterministic / self.mean_deterministic
        } else {
            0.0
        }
    }

    /// Returns the coefficient of variation for discretion ratio.
    pub fn cv_discretion(&self) -> f64 {
        if self.mean_discretion > 0.0 {
            self.std_dev_discretion / self.mean_discretion
        } else {
            0.0
        }
    }
}

/// Comprehensive risk analysis report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAnalysisReport {
    /// VaR at 95% confidence level
    pub var_95: ValueAtRisk,
    /// VaR at 99% confidence level
    pub var_99: ValueAtRisk,
    /// CVaR at 95% confidence level
    pub cvar_95: ConditionalVaR,
    /// CVaR at 99% confidence level
    pub cvar_99: ConditionalVaR,
    /// Risk metrics
    pub risk_metrics: RiskMetrics,
    /// Confidence intervals (95%)
    pub confidence_intervals: ConfidenceIntervals,
}

/// Confidence intervals for simulation metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceIntervals {
    /// Confidence level (e.g., 0.95)
    pub confidence_level: f64,
    /// Lower bound for deterministic ratio
    pub deterministic_lower: f64,
    /// Upper bound for deterministic ratio
    pub deterministic_upper: f64,
    /// Lower bound for discretion ratio
    pub discretion_lower: f64,
    /// Upper bound for discretion ratio
    pub discretion_upper: f64,
}

impl ConfidenceIntervals {
    /// Calculates confidence intervals using the normal approximation.
    pub fn calculate(metrics: &[SimulationMetrics], confidence_level: f64) -> SimResult<Self> {
        if metrics.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Cannot calculate confidence intervals with empty data".to_string(),
            ));
        }

        let det_ratios: Vec<f64> = metrics.iter().map(|m| m.deterministic_ratio()).collect();
        let disc_ratios: Vec<f64> = metrics.iter().map(|m| m.discretion_ratio()).collect();

        let mean_det = det_ratios.iter().sum::<f64>() / det_ratios.len() as f64;
        let mean_disc = disc_ratios.iter().sum::<f64>() / disc_ratios.len() as f64;

        let std_dev_det = (det_ratios
            .iter()
            .map(|x| (x - mean_det).powi(2))
            .sum::<f64>()
            / det_ratios.len() as f64)
            .sqrt();

        let std_dev_disc = (disc_ratios
            .iter()
            .map(|x| (x - mean_disc).powi(2))
            .sum::<f64>()
            / disc_ratios.len() as f64)
            .sqrt();

        // Z-score for 95% confidence is ~1.96, for 99% is ~2.576
        let z_score = if (confidence_level - 0.95).abs() < 0.01 {
            1.96
        } else if (confidence_level - 0.99).abs() < 0.01 {
            2.576
        } else {
            // Rough approximation for other confidence levels
            (-2.0 * (1.0 - confidence_level).ln()).sqrt()
        };

        let n = metrics.len() as f64;
        let margin_det = z_score * std_dev_det / n.sqrt();
        let margin_disc = z_score * std_dev_disc / n.sqrt();

        Ok(Self {
            confidence_level,
            deterministic_lower: (mean_det - margin_det).clamp(0.0, 1.0),
            deterministic_upper: (mean_det + margin_det).clamp(0.0, 1.0),
            discretion_lower: (mean_disc - margin_disc).clamp(0.0, 1.0),
            discretion_upper: (mean_disc + margin_disc).clamp(0.0, 1.0),
        })
    }
}

impl RiskAnalysisReport {
    /// Generates a comprehensive risk analysis report from multiple simulation runs.
    pub fn generate(metrics: &[SimulationMetrics]) -> SimResult<Self> {
        if metrics.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Cannot generate risk report with empty data".to_string(),
            ));
        }

        Ok(Self {
            var_95: ValueAtRisk::calculate(metrics, 0.95)?,
            var_99: ValueAtRisk::calculate(metrics, 0.99)?,
            cvar_95: ConditionalVaR::calculate(metrics, 0.95)?,
            cvar_99: ConditionalVaR::calculate(metrics, 0.99)?,
            risk_metrics: RiskMetrics::calculate(metrics)?,
            confidence_intervals: ConfidenceIntervals::calculate(metrics, 0.95)?,
        })
    }

    /// Generates a human-readable summary report.
    pub fn summary(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Risk Analysis Report ===\n\n");

        report.push_str("Risk Metrics:\n");
        report.push_str(&format!(
            "  Mean Deterministic: {:.2}% (± {:.2}%)\n",
            self.risk_metrics.mean_deterministic * 100.0,
            self.risk_metrics.std_dev_deterministic * 100.0
        ));
        report.push_str(&format!(
            "  Mean Discretion: {:.2}% (± {:.2}%)\n",
            self.risk_metrics.mean_discretion * 100.0,
            self.risk_metrics.std_dev_discretion * 100.0
        ));
        report.push_str(&format!(
            "  Coefficient of Variation (Det): {:.2}\n",
            self.risk_metrics.cv_deterministic()
        ));

        report.push_str("\n95% Confidence Intervals:\n");
        report.push_str(&format!(
            "  Deterministic: [{:.2}%, {:.2}%]\n",
            self.confidence_intervals.deterministic_lower * 100.0,
            self.confidence_intervals.deterministic_upper * 100.0
        ));
        report.push_str(&format!(
            "  Discretion: [{:.2}%, {:.2}%]\n",
            self.confidence_intervals.discretion_lower * 100.0,
            self.confidence_intervals.discretion_upper * 100.0
        ));

        report.push_str("\nValue at Risk (VaR):\n");
        report.push_str(&format!(
            "  95% VaR (Deterministic): {:.2}%\n",
            self.var_95.var_deterministic * 100.0
        ));
        report.push_str(&format!(
            "  99% VaR (Deterministic): {:.2}%\n",
            self.var_99.var_deterministic * 100.0
        ));

        report.push_str("\nConditional VaR (CVaR):\n");
        report.push_str(&format!(
            "  95% CVaR (Deterministic): {:.2}%\n",
            self.cvar_95.cvar_deterministic * 100.0
        ));
        report.push_str(&format!(
            "  99% CVaR (Deterministic): {:.2}%\n",
            self.cvar_99.cvar_deterministic * 100.0
        ));

        report.push_str(&format!(
            "\nSample Size: {}\n",
            self.risk_metrics.sample_size
        ));

        report
    }
}

/// Comparative risk analysis between multiple statutes.
#[derive(Debug, Clone)]
pub struct ComparativeRiskAnalysis {
    /// Risk reports for each statute
    pub reports: HashMap<String, RiskAnalysisReport>,
}

impl ComparativeRiskAnalysis {
    /// Creates a new comparative analysis.
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
        }
    }

    /// Adds a statute's risk analysis.
    pub fn add_statute(
        mut self,
        statute_id: String,
        metrics: &[SimulationMetrics],
    ) -> SimResult<Self> {
        let report = RiskAnalysisReport::generate(metrics)?;
        self.reports.insert(statute_id, report);
        Ok(self)
    }

    /// Identifies the statute with lowest risk (highest mean deterministic, lowest volatility).
    pub fn lowest_risk_statute(&self) -> Option<(&String, &RiskAnalysisReport)> {
        self.reports.iter().min_by(|(_, a), (_, b)| {
            let score_a =
                a.risk_metrics.mean_deterministic - a.risk_metrics.std_dev_deterministic * 2.0;
            let score_b =
                b.risk_metrics.mean_deterministic - b.risk_metrics.std_dev_deterministic * 2.0;
            score_a.partial_cmp(&score_b).unwrap()
        })
    }

    /// Identifies the statute with highest expected return (mean deterministic).
    pub fn highest_return_statute(&self) -> Option<(&String, &RiskAnalysisReport)> {
        self.reports.iter().max_by(|(_, a), (_, b)| {
            a.risk_metrics
                .mean_deterministic
                .partial_cmp(&b.risk_metrics.mean_deterministic)
                .unwrap()
        })
    }

    /// Generates a comparative report.
    pub fn comparative_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Comparative Risk Analysis ===\n\n");

        for (statute_id, risk_report) in &self.reports {
            report.push_str(&format!("Statute: {}\n", statute_id));
            report.push_str(&format!(
                "  Mean: {:.2}%, Volatility: {:.2}%, CV: {:.2}\n",
                risk_report.risk_metrics.mean_deterministic * 100.0,
                risk_report.risk_metrics.std_dev_deterministic * 100.0,
                risk_report.risk_metrics.cv_deterministic()
            ));
            report.push_str(&format!(
                "  95% VaR: {:.2}%, 95% CVaR: {:.2}%\n\n",
                risk_report.var_95.var_deterministic * 100.0,
                risk_report.cvar_95.cvar_deterministic * 100.0
            ));
        }

        if let Some((statute_id, _)) = self.lowest_risk_statute() {
            report.push_str(&format!("Lowest Risk: {}\n", statute_id));
        }

        if let Some((statute_id, _)) = self.highest_return_statute() {
            report.push_str(&format!("Highest Expected Return: {}\n", statute_id));
        }

        report
    }
}

impl Default for ComparativeRiskAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metrics(n: usize, base_det: f64, volatility: f64) -> Vec<SimulationMetrics> {
        use crate::engine::LawApplicationResult;
        use legalis_core::{Effect, EffectType, LegalResult};
        use uuid::Uuid;

        (0..n)
            .map(|i| {
                let mut metrics = SimulationMetrics::new();
                let det_ratio = base_det + (i as f64 / n as f64 - 0.5) * volatility;
                let det_count = (det_ratio * 100.0) as usize;
                let disc_count = 100 - det_count;

                for _ in 0..det_count {
                    metrics.record_result(&LawApplicationResult {
                        agent_id: Uuid::new_v4(),
                        statute_id: "test".to_string(),
                        result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "Test")),
                    });
                }

                for _ in 0..disc_count {
                    metrics.record_result(&LawApplicationResult {
                        agent_id: Uuid::new_v4(),
                        statute_id: "test".to_string(),
                        result: LegalResult::JudicialDiscretion {
                            issue: "Test".to_string(),
                            context_id: Uuid::new_v4(),
                            narrative_hint: None,
                        },
                    });
                }

                metrics
            })
            .collect()
    }

    #[test]
    fn test_value_at_risk() {
        let metrics = create_test_metrics(100, 0.7, 0.2);
        let var = ValueAtRisk::calculate(&metrics, 0.95).unwrap();

        assert_eq!(var.confidence_level, 0.95);
        assert!(var.var_deterministic > 0.0);
        assert!(var.var_deterministic < 1.0);
        assert_eq!(var.sample_size, 100);
    }

    #[test]
    fn test_conditional_var() {
        let metrics = create_test_metrics(100, 0.7, 0.2);
        let cvar = ConditionalVaR::calculate(&metrics, 0.95).unwrap();

        assert_eq!(cvar.confidence_level, 0.95);
        assert!(cvar.cvar_deterministic > 0.0);
        assert!(cvar.cvar_deterministic < 1.0);
        assert_eq!(cvar.sample_size, 100);
    }

    #[test]
    fn test_risk_metrics() {
        let metrics = create_test_metrics(100, 0.7, 0.2);
        let risk = RiskMetrics::calculate(&metrics).unwrap();

        assert!((risk.mean_deterministic - 0.7).abs() < 0.1);
        assert!(risk.std_dev_deterministic > 0.0);
        assert_eq!(risk.sample_size, 100);
        assert!(risk.cv_deterministic() >= 0.0);
    }

    #[test]
    fn test_confidence_intervals() {
        let metrics = create_test_metrics(100, 0.7, 0.1);
        let ci = ConfidenceIntervals::calculate(&metrics, 0.95).unwrap();

        assert_eq!(ci.confidence_level, 0.95);
        assert!(ci.deterministic_lower < ci.deterministic_upper);
        assert!(ci.discretion_lower < ci.discretion_upper);
        assert!(ci.deterministic_lower <= 1.0);
        assert!(ci.deterministic_upper >= 0.0);
    }

    #[test]
    fn test_risk_analysis_report() {
        let metrics = create_test_metrics(100, 0.7, 0.2);
        let report = RiskAnalysisReport::generate(&metrics).unwrap();

        assert_eq!(report.var_95.confidence_level, 0.95);
        assert_eq!(report.var_99.confidence_level, 0.99);
        assert_eq!(report.cvar_95.confidence_level, 0.95);
        assert_eq!(report.cvar_99.confidence_level, 0.99);

        let summary = report.summary();
        assert!(summary.contains("Risk Analysis Report"));
        assert!(summary.contains("Mean Deterministic"));
        assert!(summary.contains("Value at Risk"));
    }

    #[test]
    fn test_comparative_risk_analysis() {
        let metrics_a = create_test_metrics(50, 0.8, 0.1);
        let metrics_b = create_test_metrics(50, 0.6, 0.3);

        let analysis = ComparativeRiskAnalysis::new()
            .add_statute("statute_a".to_string(), &metrics_a)
            .unwrap()
            .add_statute("statute_b".to_string(), &metrics_b)
            .unwrap();

        assert_eq!(analysis.reports.len(), 2);
        assert!(analysis.lowest_risk_statute().is_some());
        assert!(analysis.highest_return_statute().is_some());

        let report = analysis.comparative_report();
        assert!(report.contains("Comparative Risk Analysis"));
        assert!(report.contains("statute_a"));
        assert!(report.contains("statute_b"));
    }

    #[test]
    fn test_empty_metrics_error() {
        let empty: Vec<SimulationMetrics> = vec![];
        assert!(ValueAtRisk::calculate(&empty, 0.95).is_err());
        assert!(ConditionalVaR::calculate(&empty, 0.95).is_err());
        assert!(RiskMetrics::calculate(&empty).is_err());
        assert!(RiskAnalysisReport::generate(&empty).is_err());
    }

    #[test]
    fn test_invalid_confidence_level() {
        let metrics = create_test_metrics(10, 0.7, 0.1);
        assert!(ValueAtRisk::calculate(&metrics, 1.5).is_err());
        assert!(ValueAtRisk::calculate(&metrics, -0.1).is_err());
    }

    #[test]
    fn test_var_99_higher_than_var_95() {
        let metrics = create_test_metrics(100, 0.7, 0.3);
        let var_95 = ValueAtRisk::calculate(&metrics, 0.95).unwrap();
        let var_99 = ValueAtRisk::calculate(&metrics, 0.99).unwrap();

        // 99% VaR should be more conservative (lower) than 95% VaR
        assert!(var_99.var_deterministic <= var_95.var_deterministic);
    }

    #[test]
    fn test_cvar_more_conservative_than_var() {
        let metrics = create_test_metrics(100, 0.7, 0.3);
        let var = ValueAtRisk::calculate(&metrics, 0.95).unwrap();
        let cvar = ConditionalVaR::calculate(&metrics, 0.95).unwrap();

        // CVaR should be more conservative (lower or equal) than VaR
        assert!(cvar.cvar_deterministic <= var.var_deterministic);
    }
}
