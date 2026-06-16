//! Anomaly detection over response observations.
//!
//! This module finds anomalous *responses* (as opposed to the cost anomalies in
//! [`crate::cost_analytics`]): latency spikes, cost spikes, response-length
//! outliers, empty / truncated bodies and apparent refusals. Statistical
//! detectors use robust median/MAD z-scores by default so that a few extreme
//! points do not mask the rest, with a classic mean/standard-deviation mode
//! available. A streaming [`StreamingAnomalyMonitor`] provides online detection
//! via exponentially-weighted moving statistics.
//!
//! Severity is expressed with the crate's existing [`crate::AnomalySeverity`].

use super::{ResponseObservation, mean, percentile_sorted, population_std_dev, scaled_mad};
use crate::AnomalySeverity;
use serde::{Deserialize, Serialize};

/// The kind of response anomaly that was detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnomalyKind {
    /// A successful request that took unusually long.
    LatencySpike,
    /// A request whose estimated cost was unusually high.
    CostSpike,
    /// A response whose length was an outlier (too short or too long).
    ResponseLengthOutlier,
    /// A successful request that returned an empty body.
    EmptyResponse,
    /// A response that appears to have been cut off mid-sentence.
    TruncatedResponse,
    /// A response that appears to be a refusal / non-answer.
    RefusalDetected,
}

impl AnomalyKind {
    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            AnomalyKind::LatencySpike => "latency_spike",
            AnomalyKind::CostSpike => "cost_spike",
            AnomalyKind::ResponseLengthOutlier => "response_length_outlier",
            AnomalyKind::EmptyResponse => "empty_response",
            AnomalyKind::TruncatedResponse => "truncated_response",
            AnomalyKind::RefusalDetected => "refusal_detected",
        }
    }
}

/// A single detected anomaly tied back to its observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseAnomaly {
    /// The id of the offending [`ResponseObservation`].
    pub observation_id: String,
    /// What kind of anomaly this is.
    pub kind: AnomalyKind,
    /// How severe it is.
    pub severity: AnomalySeverity,
    /// The observed value (latency ms, cost, char count, ...).
    pub observed: f64,
    /// The expected / baseline value for that metric.
    pub expected: f64,
    /// The (robust) z-score, when the detector is statistical.
    pub z_score: Option<f64>,
    /// A human-readable description.
    pub description: String,
}

/// Configuration for [`ResponseAnomalyDetector`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyConfig {
    /// z-score at or above which a point is a `Medium` anomaly.
    pub medium_threshold: f64,
    /// z-score at or above which a point is a `High` anomaly.
    pub high_threshold: f64,
    /// Use robust median/MAD statistics instead of mean/standard-deviation.
    pub robust: bool,
    /// Minimum number of points before statistical detection runs.
    pub min_samples: usize,
    /// Enable latency-spike detection.
    pub detect_latency: bool,
    /// Enable cost-spike detection.
    pub detect_cost: bool,
    /// Enable response-length outlier detection.
    pub detect_length: bool,
    /// Enable empty / truncated / refusal content detection.
    pub detect_content: bool,
    /// Lowercase substrings that mark a response as a refusal.
    pub refusal_markers: Vec<String>,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            medium_threshold: 3.0,
            high_threshold: 5.0,
            robust: true,
            min_samples: 8,
            detect_latency: true,
            detect_cost: true,
            detect_length: true,
            detect_content: true,
            refusal_markers: vec![
                "i cannot".to_string(),
                "i can't".to_string(),
                "i am unable".to_string(),
                "i'm unable".to_string(),
                "i am not able".to_string(),
                "as an ai".to_string(),
                "i am sorry, but".to_string(),
                "i'm sorry, but".to_string(),
                "cannot provide legal advice".to_string(),
                "unable to assist".to_string(),
            ],
        }
    }
}

/// Detects anomalies across a batch of response observations.
#[derive(Debug, Clone, Default)]
pub struct ResponseAnomalyDetector {
    config: AnomalyConfig,
}

impl ResponseAnomalyDetector {
    /// Creates a detector with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a detector with custom configuration.
    pub fn with_config(config: AnomalyConfig) -> Self {
        Self { config }
    }

    /// Returns a reference to the configuration.
    pub fn config(&self) -> &AnomalyConfig {
        &self.config
    }

    /// Detects all anomalies across the given observations.
    pub fn detect(&self, observations: &[ResponseObservation]) -> Vec<ResponseAnomaly> {
        let mut anomalies = Vec::new();
        let successful: Vec<&ResponseObservation> =
            observations.iter().filter(|obs| obs.is_success()).collect();

        if self.config.detect_latency {
            let values: Vec<f64> = successful.iter().map(|obs| obs.latency_ms as f64).collect();
            self.detect_statistical(
                &successful,
                &values,
                AnomalyKind::LatencySpike,
                true,
                "latency (ms)",
                &mut anomalies,
            );
        }

        if self.config.detect_cost {
            let values: Vec<f64> = successful.iter().map(|obs| obs.cost_or_zero()).collect();
            self.detect_statistical(
                &successful,
                &values,
                AnomalyKind::CostSpike,
                true,
                "cost (USD)",
                &mut anomalies,
            );
        }

        if self.config.detect_length {
            let values: Vec<f64> = successful
                .iter()
                .map(|obs| obs.response_chars as f64)
                .collect();
            self.detect_statistical(
                &successful,
                &values,
                AnomalyKind::ResponseLengthOutlier,
                false,
                "response length (chars)",
                &mut anomalies,
            );
        }

        if self.config.detect_content {
            for obs in &successful {
                self.detect_content_anomalies(obs, &mut anomalies);
            }
        }

        anomalies
    }

    /// Runs one statistical detector over a metric column.
    ///
    /// `positive_only` flags only upward deviations (spikes); otherwise both
    /// tails are flagged (outliers).
    fn detect_statistical(
        &self,
        observations: &[&ResponseObservation],
        values: &[f64],
        kind: AnomalyKind,
        positive_only: bool,
        metric_label: &str,
        out: &mut Vec<ResponseAnomaly>,
    ) {
        if values.len() < self.config.min_samples {
            return;
        }
        let (center, spread) = if self.config.robust {
            let mut sorted = values.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let med = percentile_sorted(&sorted, 50.0);
            let mad = scaled_mad(values);
            if mad > 0.0 {
                (med, mad)
            } else {
                // The scaled MAD collapses to zero when a majority of points are
                // identical; fall back to mean/standard-deviation so that a sharp
                // spike against an otherwise-constant baseline is still caught.
                (mean(values), population_std_dev(values))
            }
        } else {
            (mean(values), population_std_dev(values))
        };
        if spread <= 0.0 {
            return;
        }

        for (obs, &value) in observations.iter().zip(values.iter()) {
            let z = (value - center) / spread;
            if positive_only && z <= 0.0 {
                continue;
            }
            let magnitude = z.abs();
            if let Some(severity) = self.severity_for(magnitude) {
                out.push(ResponseAnomaly {
                    observation_id: obs.id.clone(),
                    kind,
                    severity,
                    observed: value,
                    expected: center,
                    z_score: Some(z),
                    description: format!(
                        "{} of {:.4} deviates {:.1} sigma from baseline {:.4}",
                        metric_label, value, z, center
                    ),
                });
            }
        }
    }

    /// Detects per-response content anomalies (empty, truncated, refusal).
    fn detect_content_anomalies(&self, obs: &ResponseObservation, out: &mut Vec<ResponseAnomaly>) {
        let text = match obs.response_text.as_deref() {
            Some(text) => text,
            None => return,
        };
        let trimmed = text.trim();

        if trimmed.is_empty() {
            out.push(ResponseAnomaly {
                observation_id: obs.id.clone(),
                kind: AnomalyKind::EmptyResponse,
                severity: AnomalySeverity::High,
                observed: 0.0,
                expected: 1.0,
                z_score: None,
                description: "successful request returned an empty response body".to_string(),
            });
            return;
        }

        let lower = trimmed.to_lowercase();
        if self
            .config
            .refusal_markers
            .iter()
            .any(|marker| lower.contains(marker.as_str()))
        {
            out.push(ResponseAnomaly {
                observation_id: obs.id.clone(),
                kind: AnomalyKind::RefusalDetected,
                severity: AnomalySeverity::Low,
                observed: trimmed.chars().count() as f64,
                expected: trimmed.chars().count() as f64,
                z_score: None,
                description: "response appears to be a refusal or non-answer".to_string(),
            });
        }

        if Self::looks_truncated(trimmed) {
            out.push(ResponseAnomaly {
                observation_id: obs.id.clone(),
                kind: AnomalyKind::TruncatedResponse,
                severity: AnomalySeverity::Medium,
                observed: trimmed.chars().count() as f64,
                expected: trimmed.chars().count() as f64,
                z_score: None,
                description: "response appears to be cut off mid-sentence".to_string(),
            });
        }
    }

    /// Heuristic: a non-trivial response that does not end in terminal
    /// punctuation or a closing bracket is likely truncated.
    fn looks_truncated(text: &str) -> bool {
        if text.chars().count() < 40 {
            return false;
        }
        match text.chars().last() {
            Some(last) => !matches!(last, '.' | '!' | '?' | '"' | ')' | ']' | '}' | '`' | ':'),
            None => false,
        }
    }

    /// Maps a z-score magnitude to a severity, or `None` if it is not anomalous.
    fn severity_for(&self, magnitude: f64) -> Option<AnomalySeverity> {
        if magnitude >= self.config.high_threshold {
            Some(AnomalySeverity::High)
        } else if magnitude >= self.config.medium_threshold {
            Some(AnomalySeverity::Medium)
        } else {
            None
        }
    }
}

/// An online anomaly monitor using exponentially-weighted moving statistics.
///
/// Suited to streaming a single metric (typically latency) where retaining a
/// full history is undesirable. After warming up over `warmup` points it flags
/// any value whose distance from the running EWMA mean exceeds `threshold`
/// running standard deviations.
#[derive(Debug, Clone)]
pub struct StreamingAnomalyMonitor {
    alpha: f64,
    threshold: f64,
    warmup: usize,
    count: usize,
    ewma_mean: f64,
    ewma_var: f64,
}

impl StreamingAnomalyMonitor {
    /// Creates a new streaming monitor.
    ///
    /// `alpha` is the EWMA smoothing factor in `(0, 1]` (higher = more reactive),
    /// `threshold` is the z-score cut-off and `warmup` is the number of points to
    /// observe before flagging.
    pub fn new(alpha: f64, threshold: f64, warmup: usize) -> Self {
        Self {
            alpha: alpha.clamp(f64::EPSILON, 1.0),
            threshold,
            warmup: warmup.max(1),
            count: 0,
            ewma_mean: 0.0,
            ewma_var: 0.0,
        }
    }

    /// Feeds one value and returns its z-score if it is anomalous.
    ///
    /// The point is scored against the *established* baseline (the statistics
    /// before this point) so that a large spike cannot inflate its own variance
    /// and hide itself. The running statistics are then always updated, including
    /// for flagged points, so a sustained shift is eventually learned rather than
    /// alerted on indefinitely.
    pub fn update(&mut self, value: f64) -> Option<f64> {
        self.count += 1;
        if self.count == 1 {
            self.ewma_mean = value;
            self.ewma_var = 0.0;
            return None;
        }

        let diff = value - self.ewma_mean;
        let baseline_std = self.ewma_var.sqrt();
        let z = if self.count > self.warmup && baseline_std > 0.0 {
            Some(diff / baseline_std)
        } else {
            None
        };

        // West's online EWMA mean/variance update (applied after scoring).
        let incr = self.alpha * diff;
        self.ewma_mean += incr;
        self.ewma_var = (1.0 - self.alpha) * (self.ewma_var + diff * incr);

        match z {
            Some(z) if z.abs() >= self.threshold => Some(z),
            _ => None,
        }
    }

    /// Returns the current EWMA mean.
    pub fn current_mean(&self) -> f64 {
        self.ewma_mean
    }

    /// Returns the number of points seen so far.
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenUsage;

    fn latency_obs(latency: u64) -> ResponseObservation {
        ResponseObservation::new("openai", "gpt-4").with_latency(latency)
    }

    #[test]
    fn test_latency_spike_detection() {
        let mut observations: Vec<ResponseObservation> =
            (0..30).map(|_| latency_obs(100)).collect();
        let spike = latency_obs(100_000).with_id("spike");
        observations.push(spike);

        let detector = ResponseAnomalyDetector::new();
        let anomalies = detector.detect(&observations);
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == AnomalyKind::LatencySpike && a.observation_id == "spike")
        );
    }

    #[test]
    fn test_no_anomalies_in_uniform_data() {
        let observations: Vec<ResponseObservation> = (0..30).map(|_| latency_obs(100)).collect();
        let detector = ResponseAnomalyDetector::new();
        let anomalies = detector.detect(&observations);
        assert!(
            anomalies
                .iter()
                .all(|a| a.kind != AnomalyKind::LatencySpike)
        );
    }

    #[test]
    fn test_cost_spike_detection() {
        let mut observations: Vec<ResponseObservation> =
            (0..20).map(|_| latency_obs(100).with_cost(0.01)).collect();
        observations.push(latency_obs(100).with_cost(50.0).with_id("expensive"));

        let detector = ResponseAnomalyDetector::new();
        let anomalies = detector.detect(&observations);
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == AnomalyKind::CostSpike && a.observation_id == "expensive")
        );
    }

    #[test]
    fn test_empty_and_refusal_detection() {
        let empty = ResponseObservation::new("openai", "gpt-4")
            .with_id("empty")
            .with_response("   ");
        let refusal = ResponseObservation::new("openai", "gpt-4")
            .with_id("refusal")
            .with_response("I cannot provide legal advice on this matter.");

        let detector = ResponseAnomalyDetector::new();
        let anomalies = detector.detect(&[empty, refusal]);
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == AnomalyKind::EmptyResponse && a.observation_id == "empty")
        );
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == AnomalyKind::RefusalDetected && a.observation_id == "refusal")
        );
    }

    #[test]
    fn test_truncated_response_detection() {
        let truncated = ResponseObservation::new("openai", "gpt-4")
            .with_id("trunc")
            .with_response(
                "The contract is governed by the laws of the State of New York and the parties agree to",
            );
        let complete = ResponseObservation::new("openai", "gpt-4")
            .with_id("ok")
            .with_response("The contract is governed by the laws of the State of New York.");

        let detector = ResponseAnomalyDetector::new();
        let anomalies = detector.detect(&[truncated, complete]);
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == AnomalyKind::TruncatedResponse && a.observation_id == "trunc")
        );
        assert!(
            !anomalies
                .iter()
                .any(|a| a.kind == AnomalyKind::TruncatedResponse && a.observation_id == "ok")
        );
    }

    #[test]
    fn test_failures_excluded_from_statistics() {
        // Failures should not pollute the latency baseline.
        let mut observations: Vec<ResponseObservation> =
            (0..20).map(|_| latency_obs(100)).collect();
        observations.push(latency_obs(99_999).failed("timeout").with_id("failed"));

        let detector = ResponseAnomalyDetector::new();
        let anomalies = detector.detect(&observations);
        assert!(anomalies.iter().all(|a| a.observation_id != "failed"));
    }

    #[test]
    fn test_streaming_monitor() {
        let mut monitor = StreamingAnomalyMonitor::new(0.2, 3.0, 10);
        // Warm up on a steady signal with slight noise.
        for index in 0..40 {
            let value = 100.0 + (index % 3) as f64;
            let _ = monitor.update(value);
        }
        // A massive jump should be flagged.
        let flagged = monitor.update(10_000.0);
        assert!(flagged.is_some());
        assert!(monitor.count() > 10);
    }

    #[test]
    fn test_classic_mode_config() {
        let config = AnomalyConfig {
            robust: false,
            ..AnomalyConfig::default()
        };
        let mut observations: Vec<ResponseObservation> = (0..20)
            .map(|_| latency_obs(100).with_usage(TokenUsage::new(10, 10)))
            .collect();
        observations.push(latency_obs(5000).with_id("spike"));
        let detector = ResponseAnomalyDetector::with_config(config);
        let anomalies = detector.detect(&observations);
        assert!(anomalies.iter().any(|a| a.observation_id == "spike"));
    }
}
