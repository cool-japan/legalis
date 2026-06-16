//! A/B test result analysis with statistical significance testing.
//!
//! Groups response observations (and any linked user feedback) by their
//! [`ResponseObservation::variant`] label and compares two variants on the
//! production metrics that matter - success rate, latency, cost and user rating -
//! using proper hypothesis tests from [`super::stats`]: a two-proportion z-test
//! for success rate and Welch's unequal-variance t-test for the continuous
//! metrics. Each comparison reports an effect size, a p-value, whether it is
//! significant at the configured alpha, and which variant won.
//!
//! This complements the text-quality A/B helper [`crate::compare_variants`]
//! (which scores BLEU/ROUGE between two strings) by analysing *live operational*
//! metrics rather than reference-based text similarity.

use super::{
    FeedbackSignal, ResponseObservation, TwoProportionTest, WelchTTest, join_feedback, mean,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Summary metrics for one experiment variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantMetrics {
    /// Variant label.
    pub name: String,
    /// Number of requests routed to the variant.
    pub requests: usize,
    /// Number of successful requests.
    pub successes: usize,
    /// Success rate in `[0, 1]`.
    pub success_rate: f64,
    /// Mean latency over successful requests (ms).
    pub mean_latency_ms: f64,
    /// Mean cost per request (USD).
    pub mean_cost: f64,
    /// Number of user ratings linked to the variant.
    pub rating_count: usize,
    /// Mean user rating (1-5), when any feedback is linked.
    pub mean_rating: Option<f64>,
}

/// One metric comparison between variant A and variant B.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricComparison {
    /// Name of the metric being compared.
    pub metric: String,
    /// Variant A's value.
    pub value_a: f64,
    /// Variant B's value.
    pub value_b: f64,
    /// Absolute difference (`B - A`).
    pub difference: f64,
    /// Relative change as a percent of A (`0.0` when A is zero).
    pub relative_change_pct: f64,
    /// The statistical test used.
    pub test: String,
    /// Two-sided p-value, when a test could be run.
    pub p_value: Option<f64>,
    /// Whether the difference is significant at the experiment's alpha.
    pub significant: bool,
    /// Whether a lower value is better for this metric.
    pub lower_is_better: bool,
    /// The winning variant's label, when the difference is significant.
    pub winner: Option<String>,
}

/// The full result of analysing an A/B experiment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbAnalysis {
    /// Experiment name.
    pub experiment: String,
    /// Significance level used.
    pub alpha: f64,
    /// Metrics for variant A.
    pub variant_a: VariantMetrics,
    /// Metrics for variant B.
    pub variant_b: VariantMetrics,
    /// Per-metric comparisons.
    pub comparisons: Vec<MetricComparison>,
    /// The overall winning variant, when one is decisively ahead.
    pub overall_winner: Option<String>,
    /// A human-readable recommendation.
    pub recommendation: String,
}

impl AbAnalysis {
    /// Returns the comparison for a named metric, if present.
    pub fn comparison(&self, metric: &str) -> Option<&MetricComparison> {
        self.comparisons.iter().find(|c| c.metric == metric)
    }

    /// Returns the number of metrics on which a variant significantly won.
    pub fn significant_wins(&self, variant: &str) -> usize {
        self.comparisons
            .iter()
            .filter(|c| c.winner.as_deref() == Some(variant))
            .count()
    }
}

/// An A/B experiment: a labelled collection of observations and feedback.
pub struct Experiment {
    name: String,
    alpha: f64,
    observations: Vec<ResponseObservation>,
    feedback: Vec<FeedbackSignal>,
}

impl Experiment {
    /// Creates a new experiment with the default alpha of `0.05`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alpha: 0.05,
            observations: Vec::new(),
            feedback: Vec::new(),
        }
    }

    /// Sets the significance level (clamped to a sane open interval).
    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(1e-6, 0.5);
        self
    }

    /// Adds one observation.
    pub fn add_observation(&mut self, observation: ResponseObservation) {
        self.observations.push(observation);
    }

    /// Adds many observations.
    pub fn add_observations(
        &mut self,
        observations: impl IntoIterator<Item = ResponseObservation>,
    ) {
        self.observations.extend(observations);
    }

    /// Adds a feedback signal.
    pub fn add_feedback(&mut self, signal: FeedbackSignal) {
        self.feedback.push(signal);
    }

    /// Returns the distinct variant labels present, with their request counts.
    pub fn variant_counts(&self) -> BTreeMap<String, usize> {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for obs in &self.observations {
            if let Some(variant) = &obs.variant {
                *counts.entry(variant.clone()).or_insert(0) += 1;
            }
        }
        counts
    }

    /// Analyses two named variants, returning `None` if either is absent.
    pub fn analyze(&self, variant_a: &str, variant_b: &str) -> Option<AbAnalysis> {
        let obs_a: Vec<&ResponseObservation> = self.variant_observations(variant_a);
        let obs_b: Vec<&ResponseObservation> = self.variant_observations(variant_b);
        if obs_a.is_empty() || obs_b.is_empty() {
            return None;
        }

        let ratings_a = self.variant_ratings(variant_a);
        let ratings_b = self.variant_ratings(variant_b);

        let metrics_a = variant_metrics(variant_a, &obs_a, &ratings_a);
        let metrics_b = variant_metrics(variant_b, &obs_b, &ratings_b);

        let mut comparisons = Vec::new();
        comparisons.push(self.compare_success_rate(&obs_a, &obs_b, variant_a, variant_b));
        comparisons.push(self.compare_latency(&obs_a, &obs_b, variant_a, variant_b));
        comparisons.push(self.compare_cost(&obs_a, &obs_b, variant_a, variant_b));
        if let Some(rating_comparison) =
            self.compare_ratings(&ratings_a, &ratings_b, variant_a, variant_b)
        {
            comparisons.push(rating_comparison);
        }

        let (overall_winner, recommendation) =
            decide_winner(variant_a, variant_b, &comparisons, &metrics_a, &metrics_b);

        Some(AbAnalysis {
            experiment: self.name.clone(),
            alpha: self.alpha,
            variant_a: metrics_a,
            variant_b: metrics_b,
            comparisons,
            overall_winner,
            recommendation,
        })
    }

    /// Analyses the two most-trafficked variants automatically.
    pub fn analyze_top_two(&self) -> Option<AbAnalysis> {
        let counts = self.variant_counts();
        let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        if ranked.len() < 2 {
            return None;
        }
        self.analyze(&ranked[0].0, &ranked[1].0)
    }

    fn variant_observations(&self, variant: &str) -> Vec<&ResponseObservation> {
        self.observations
            .iter()
            .filter(|obs| obs.variant.as_deref() == Some(variant))
            .collect()
    }

    /// Returns the user ratings (1-5) linked to a variant via feedback joins.
    fn variant_ratings(&self, variant: &str) -> Vec<f64> {
        let joined = join_feedback(&self.observations, &self.feedback);
        joined
            .into_iter()
            .filter(|(obs, _)| obs.variant.as_deref() == Some(variant))
            .filter_map(|(_, signal)| signal.rating.map(|rating| rating.value() as f64))
            .collect()
    }

    fn compare_success_rate(
        &self,
        obs_a: &[&ResponseObservation],
        obs_b: &[&ResponseObservation],
        name_a: &str,
        name_b: &str,
    ) -> MetricComparison {
        let successes_a = obs_a.iter().filter(|obs| obs.is_success()).count();
        let successes_b = obs_b.iter().filter(|obs| obs.is_success()).count();
        let rate_a = successes_a as f64 / obs_a.len() as f64;
        let rate_b = successes_b as f64 / obs_b.len() as f64;

        let test = TwoProportionTest::run(successes_a, obs_a.len(), successes_b, obs_b.len());
        let p_value = test.map(|t| t.p_value);
        self.build_comparison(
            "success_rate",
            "two_proportion_z",
            rate_a,
            rate_b,
            p_value,
            false,
            name_a,
            name_b,
        )
    }

    fn compare_latency(
        &self,
        obs_a: &[&ResponseObservation],
        obs_b: &[&ResponseObservation],
        name_a: &str,
        name_b: &str,
    ) -> MetricComparison {
        let lat_a: Vec<f64> = obs_a
            .iter()
            .filter(|obs| obs.is_success())
            .map(|obs| obs.latency_ms as f64)
            .collect();
        let lat_b: Vec<f64> = obs_b
            .iter()
            .filter(|obs| obs.is_success())
            .map(|obs| obs.latency_ms as f64)
            .collect();
        let p_value = WelchTTest::run(&lat_a, &lat_b).map(|t| t.p_value);
        self.build_comparison(
            "latency_ms",
            "welch_t",
            mean(&lat_a),
            mean(&lat_b),
            p_value,
            true,
            name_a,
            name_b,
        )
    }

    fn compare_cost(
        &self,
        obs_a: &[&ResponseObservation],
        obs_b: &[&ResponseObservation],
        name_a: &str,
        name_b: &str,
    ) -> MetricComparison {
        let cost_a: Vec<f64> = obs_a.iter().map(|obs| obs.cost_or_zero()).collect();
        let cost_b: Vec<f64> = obs_b.iter().map(|obs| obs.cost_or_zero()).collect();
        let p_value = WelchTTest::run(&cost_a, &cost_b).map(|t| t.p_value);
        self.build_comparison(
            "cost_usd",
            "welch_t",
            mean(&cost_a),
            mean(&cost_b),
            p_value,
            true,
            name_a,
            name_b,
        )
    }

    fn compare_ratings(
        &self,
        ratings_a: &[f64],
        ratings_b: &[f64],
        name_a: &str,
        name_b: &str,
    ) -> Option<MetricComparison> {
        if ratings_a.is_empty() && ratings_b.is_empty() {
            return None;
        }
        let p_value = WelchTTest::run(ratings_a, ratings_b).map(|t| t.p_value);
        Some(self.build_comparison(
            "user_rating",
            "welch_t",
            mean(ratings_a),
            mean(ratings_b),
            p_value,
            false,
            name_a,
            name_b,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn build_comparison(
        &self,
        metric: &str,
        test: &str,
        value_a: f64,
        value_b: f64,
        p_value: Option<f64>,
        lower_is_better: bool,
        name_a: &str,
        name_b: &str,
    ) -> MetricComparison {
        let difference = value_b - value_a;
        let relative_change_pct = if value_a.abs() > f64::EPSILON {
            difference / value_a * 100.0
        } else {
            0.0
        };
        let significant = p_value.map(|p| p < self.alpha).unwrap_or(false);
        let winner = if significant {
            let b_better = if lower_is_better {
                value_b < value_a
            } else {
                value_b > value_a
            };
            Some(if b_better {
                name_b.to_string()
            } else {
                name_a.to_string()
            })
        } else {
            None
        };

        MetricComparison {
            metric: metric.to_string(),
            value_a,
            value_b,
            difference,
            relative_change_pct,
            test: test.to_string(),
            p_value,
            significant,
            lower_is_better,
            winner,
        }
    }
}

/// Builds the per-variant metric summary.
fn variant_metrics(
    name: &str,
    observations: &[&ResponseObservation],
    ratings: &[f64],
) -> VariantMetrics {
    let requests = observations.len();
    let successes = observations.iter().filter(|obs| obs.is_success()).count();
    let success_rate = if requests == 0 {
        0.0
    } else {
        successes as f64 / requests as f64
    };
    let latencies: Vec<f64> = observations
        .iter()
        .filter(|obs| obs.is_success())
        .map(|obs| obs.latency_ms as f64)
        .collect();
    let costs: Vec<f64> = observations.iter().map(|obs| obs.cost_or_zero()).collect();
    let mean_rating = if ratings.is_empty() {
        None
    } else {
        Some(mean(ratings))
    };

    VariantMetrics {
        name: name.to_string(),
        requests,
        successes,
        success_rate,
        mean_latency_ms: mean(&latencies),
        mean_cost: mean(&costs),
        rating_count: ratings.len(),
        mean_rating,
    }
}

/// Decides the overall winner by counting significant metric wins.
fn decide_winner(
    name_a: &str,
    name_b: &str,
    comparisons: &[MetricComparison],
    metrics_a: &VariantMetrics,
    metrics_b: &VariantMetrics,
) -> (Option<String>, String) {
    let wins_a = comparisons
        .iter()
        .filter(|c| c.winner.as_deref() == Some(name_a))
        .count();
    let wins_b = comparisons
        .iter()
        .filter(|c| c.winner.as_deref() == Some(name_b))
        .count();

    if wins_a == 0 && wins_b == 0 {
        return (
            None,
            format!(
                "No statistically significant difference between '{name_a}' and '{name_b}'; \
                 keep collecting data before deciding."
            ),
        );
    }

    if wins_a > wins_b {
        (
            Some(name_a.to_string()),
            format!(
                "Variant '{name_a}' wins {wins_a} significant metric(s) vs {wins_b} for '{name_b}' \
                 (success rate {:.1}% vs {:.1}%); prefer '{name_a}'.",
                metrics_a.success_rate * 100.0,
                metrics_b.success_rate * 100.0
            ),
        )
    } else if wins_b > wins_a {
        (
            Some(name_b.to_string()),
            format!(
                "Variant '{name_b}' wins {wins_b} significant metric(s) vs {wins_a} for '{name_a}' \
                 (success rate {:.1}% vs {:.1}%); prefer '{name_b}'.",
                metrics_b.success_rate * 100.0,
                metrics_a.success_rate * 100.0
            ),
        )
    } else {
        // Tie on win count: break by significant success-rate difference.
        let success_winner = comparisons
            .iter()
            .find(|c| c.metric == "success_rate" && c.significant)
            .and_then(|c| c.winner.clone());
        match success_winner {
            Some(winner) => (
                Some(winner.clone()),
                format!("Variants tie on metric wins; '{winner}' is preferred on success rate."),
            ),
            None => (
                None,
                "Variants are evenly matched across significant metrics; no clear winner."
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human_feedback::Rating;

    fn obs(variant: &str, success: bool, latency: u64, cost: f64) -> ResponseObservation {
        let base = ResponseObservation::new("openai", "gpt-4")
            .with_variant(variant)
            .with_latency(latency)
            .with_cost(cost);
        if success {
            base
        } else {
            base.with_error(super::super::ErrorCategory::Timeout)
        }
    }

    #[test]
    fn test_variant_counts_and_missing() {
        let mut experiment = Experiment::new("test");
        experiment.add_observation(obs("a", true, 100, 0.01));
        experiment.add_observation(obs("b", true, 100, 0.01));
        let counts = experiment.variant_counts();
        assert_eq!(counts.get("a"), Some(&1));
        assert!(experiment.analyze("a", "missing").is_none());
    }

    #[test]
    fn test_success_rate_significant() {
        let mut experiment = Experiment::new("success");
        // A: 95/100 success; B: 60/100 success - a large, significant gap.
        for index in 0..100 {
            experiment.add_observation(obs("a", index >= 5, 100, 0.01));
            experiment.add_observation(obs("b", index >= 40, 100, 0.01));
        }
        let analysis = experiment.analyze("a", "b").expect("both present");
        let success = analysis.comparison("success_rate").expect("present");
        assert!(success.significant);
        assert_eq!(success.winner.as_deref(), Some("a"));
        assert_eq!(analysis.overall_winner.as_deref(), Some("a"));
        assert!(analysis.significant_wins("a") >= 1);
    }

    #[test]
    fn test_latency_lower_is_better() {
        let mut experiment = Experiment::new("latency");
        for index in 0..60 {
            // Both always succeed; B is consistently much faster.
            experiment.add_observation(obs("a", true, 1000 + (index % 5) as u64, 0.01));
            experiment.add_observation(obs("b", true, 200 + (index % 5) as u64, 0.01));
        }
        let analysis = experiment.analyze("a", "b").expect("present");
        let latency = analysis.comparison("latency_ms").expect("present");
        assert!(latency.lower_is_better);
        assert!(latency.significant);
        // B is faster, so B wins the latency metric.
        assert_eq!(latency.winner.as_deref(), Some("b"));
    }

    #[test]
    fn test_no_significant_difference() {
        let mut experiment = Experiment::new("noise");
        for index in 0..50 {
            experiment.add_observation(obs("a", index != 0, 300, 0.02));
            experiment.add_observation(obs("b", index != 1, 305, 0.02));
        }
        let analysis = experiment.analyze("a", "b").expect("present");
        let success = analysis.comparison("success_rate").expect("present");
        assert!(!success.significant);
        assert!(analysis.overall_winner.is_none());
        assert!(
            analysis
                .recommendation
                .contains("No statistically significant")
        );
    }

    #[test]
    fn test_rating_comparison_with_feedback() {
        let mut experiment = Experiment::new("ratings");
        // Build variant-tagged observations with known ids, then rate them.
        // Use a realistic spread (not constants) so variance is well-defined.
        for index in 0..20 {
            let id_a = format!("a-{index}");
            let id_b = format!("b-{index}");
            experiment.add_observation(obs("a", true, 100, 0.01).with_id(&id_a));
            experiment.add_observation(obs("b", true, 100, 0.01).with_id(&id_b));
            let rating_a = if index % 2 == 0 {
                Rating::Excellent
            } else {
                Rating::Good
            };
            let rating_b = if index % 2 == 0 {
                Rating::Poor
            } else {
                Rating::VeryPoor
            };
            experiment.add_feedback(FeedbackSignal::new(id_a).with_rating(rating_a));
            experiment.add_feedback(FeedbackSignal::new(id_b).with_rating(rating_b));
        }
        let analysis = experiment.analyze("a", "b").expect("present");
        let rating = analysis.comparison("user_rating").expect("present");
        assert!(rating.significant);
        assert_eq!(rating.winner.as_deref(), Some("a"));
        assert_eq!(analysis.variant_a.rating_count, 20);
        assert!(
            analysis.variant_a.mean_rating.unwrap_or(0.0)
                > analysis.variant_b.mean_rating.unwrap_or(0.0)
        );
    }

    #[test]
    fn test_analyze_top_two() {
        let mut experiment = Experiment::new("auto");
        for _ in 0..30 {
            experiment.add_observation(obs("control", true, 100, 0.01));
        }
        for _ in 0..25 {
            experiment.add_observation(obs("treatment", true, 100, 0.01));
        }
        for _ in 0..2 {
            experiment.add_observation(obs("rare", true, 100, 0.01));
        }
        let analysis = experiment.analyze_top_two().expect("two variants");
        let names = [
            analysis.variant_a.name.clone(),
            analysis.variant_b.name.clone(),
        ];
        assert!(names.contains(&"control".to_string()));
        assert!(names.contains(&"treatment".to_string()));
        assert!(!names.contains(&"rare".to_string()));
    }
}
