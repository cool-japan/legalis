//! Quality-assurance metrics over response observations.
//!
//! Where [`crate::calculate_quality_metrics`] scores the *readability* of a
//! single text, this module gates a *stream* of production responses against a
//! configurable suite of [`QaCheck`]s and reports continuous pass-rates - the
//! kind of QA signal a production monitor watches over time. It reuses the
//! existing readability scorer for an aggregate text-quality summary rather than
//! re-implementing it.

use super::ResponseObservation;
use crate::calculate_quality_metrics;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A declarative quality-assurance check applied to a response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QaCheck {
    /// The request must have succeeded.
    Succeeded,
    /// The response body must be non-empty (after trimming).
    NonEmpty,
    /// The response must be at least this many characters.
    MinLength(usize),
    /// The response must be at most this many characters.
    MaxLength(usize),
    /// The request latency must not exceed this many milliseconds.
    MaxLatencyMs(u64),
    /// The response must contain at least one of these (case-insensitive) terms.
    MustContainAny(Vec<String>),
    /// The response must contain none of these (case-insensitive) terms.
    MustNotContain(Vec<String>),
    /// The response must match this regular expression.
    MatchesPattern(String),
    /// The response must not look like a refusal (uses the given markers).
    NoRefusal(Vec<String>),
    /// The response must parse as valid JSON.
    ValidJson,
}

impl QaCheck {
    /// Returns a stable, human-readable name for the check.
    pub fn name(&self) -> String {
        match self {
            QaCheck::Succeeded => "succeeded".to_string(),
            QaCheck::NonEmpty => "non_empty".to_string(),
            QaCheck::MinLength(n) => format!("min_length_{n}"),
            QaCheck::MaxLength(n) => format!("max_length_{n}"),
            QaCheck::MaxLatencyMs(n) => format!("max_latency_{n}ms"),
            QaCheck::MustContainAny(_) => "must_contain_any".to_string(),
            QaCheck::MustNotContain(_) => "must_not_contain".to_string(),
            QaCheck::MatchesPattern(pattern) => format!("matches_pattern[{pattern}]"),
            QaCheck::NoRefusal(_) => "no_refusal".to_string(),
            QaCheck::ValidJson => "valid_json".to_string(),
        }
    }
}

/// The outcome of evaluating one check against one observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QaCheckOutcome {
    /// The check passed.
    Pass,
    /// The check failed.
    Fail,
    /// The check does not apply to this observation (e.g. no response body).
    NotApplicable,
}

impl QaCheckOutcome {
    fn from_bool(passed: bool) -> Self {
        if passed {
            QaCheckOutcome::Pass
        } else {
            QaCheckOutcome::Fail
        }
    }
}

/// Per-check aggregated statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckStats {
    /// The check's name.
    pub name: String,
    /// Number of observations where the check passed.
    pub passed: usize,
    /// Number of observations where the check failed.
    pub failed: usize,
    /// Number of observations where the check did not apply.
    pub not_applicable: usize,
}

impl CheckStats {
    /// Pass rate over *applicable* observations in `[0, 1]` (`1.0` if none apply).
    pub fn pass_rate(&self) -> f64 {
        let applicable = self.passed + self.failed;
        if applicable == 0 {
            1.0
        } else {
            self.passed as f64 / applicable as f64
        }
    }
}

/// An aggregate text-quality summary (averaged readability metrics).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateTextQuality {
    /// Number of responses that contributed text.
    pub sampled_responses: usize,
    /// Average Flesch reading-ease approximation.
    pub avg_readability: f64,
    /// Average vocabulary richness (unique / total words).
    pub avg_vocabulary_richness: f64,
    /// Average word count per response.
    pub avg_word_count: f64,
}

/// The QA report aggregated across a batch of observations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QaReport {
    /// Number of observations evaluated.
    pub total_observations: usize,
    /// Number of observations that passed every applicable check.
    pub fully_passing: usize,
    /// Per-check statistics (sorted by check name).
    pub checks: Vec<CheckStats>,
    /// Sample of ids of observations that failed at least one check (capped).
    pub failing_observation_ids: Vec<String>,
    /// Aggregate readability summary, when any response carried text.
    pub text_quality: Option<AggregateTextQuality>,
}

impl QaReport {
    /// Overall QA pass-rate: fraction of observations passing all checks.
    pub fn overall_pass_rate(&self) -> f64 {
        if self.total_observations == 0 {
            1.0
        } else {
            self.fully_passing as f64 / self.total_observations as f64
        }
    }
}

/// Evaluates a suite of [`QaCheck`]s against response observations.
///
/// Regular-expression checks are compiled once at construction; an invalid
/// pattern simply yields a check that never applies (rather than panicking).
pub struct QaEvaluator {
    checks: Vec<QaCheck>,
    compiled: Vec<Option<Regex>>,
    sample_cap: usize,
}

impl QaEvaluator {
    /// Builds an evaluator from a list of checks.
    pub fn new(checks: Vec<QaCheck>) -> Self {
        let compiled = checks
            .iter()
            .map(|check| match check {
                QaCheck::MatchesPattern(pattern) => Regex::new(pattern).ok(),
                _ => None,
            })
            .collect();
        Self {
            checks,
            compiled,
            sample_cap: 50,
        }
    }

    /// A sensible default suite for legal LLM responses.
    pub fn legal_default() -> Self {
        Self::new(vec![
            QaCheck::Succeeded,
            QaCheck::NonEmpty,
            QaCheck::MinLength(40),
            QaCheck::NoRefusal(default_refusal_markers()),
        ])
    }

    /// Sets the cap on retained failing-observation ids.
    pub fn with_sample_cap(mut self, cap: usize) -> Self {
        self.sample_cap = cap;
        self
    }

    /// Returns the configured checks.
    pub fn checks(&self) -> &[QaCheck] {
        &self.checks
    }

    /// Evaluates every check against a single observation.
    pub fn evaluate_one(&self, obs: &ResponseObservation) -> Vec<QaCheckOutcome> {
        self.checks
            .iter()
            .zip(self.compiled.iter())
            .map(|(check, regex)| evaluate_check(check, regex.as_ref(), obs))
            .collect()
    }

    /// Evaluates the suite across a batch and aggregates a [`QaReport`].
    pub fn evaluate(&self, observations: &[ResponseObservation]) -> QaReport {
        let mut passed = vec![0usize; self.checks.len()];
        let mut failed = vec![0usize; self.checks.len()];
        let mut not_applicable = vec![0usize; self.checks.len()];
        let mut fully_passing = 0usize;
        let mut failing_ids = Vec::new();

        for obs in observations {
            let outcomes = self.evaluate_one(obs);
            let mut all_pass = true;
            for (index, outcome) in outcomes.iter().enumerate() {
                match outcome {
                    QaCheckOutcome::Pass => passed[index] += 1,
                    QaCheckOutcome::Fail => {
                        failed[index] += 1;
                        all_pass = false;
                    }
                    QaCheckOutcome::NotApplicable => not_applicable[index] += 1,
                }
            }
            if all_pass {
                fully_passing += 1;
            } else if failing_ids.len() < self.sample_cap {
                failing_ids.push(obs.id.clone());
            }
        }

        let mut checks: Vec<CheckStats> = self
            .checks
            .iter()
            .enumerate()
            .map(|(index, check)| CheckStats {
                name: check.name(),
                passed: passed[index],
                failed: failed[index],
                not_applicable: not_applicable[index],
            })
            .collect();
        checks.sort_by(|a, b| a.name.cmp(&b.name));

        QaReport {
            total_observations: observations.len(),
            fully_passing,
            checks,
            failing_observation_ids: failing_ids,
            text_quality: aggregate_text_quality(observations),
        }
    }
}

/// Returns the default refusal markers used by [`QaCheck::NoRefusal`].
pub fn default_refusal_markers() -> Vec<String> {
    vec![
        "i cannot".to_string(),
        "i can't".to_string(),
        "i am unable".to_string(),
        "i'm unable".to_string(),
        "as an ai".to_string(),
        "unable to assist".to_string(),
    ]
}

/// Computes an aggregate readability summary over responses carrying text.
fn aggregate_text_quality(observations: &[ResponseObservation]) -> Option<AggregateTextQuality> {
    let texts: Vec<&str> = observations
        .iter()
        .filter_map(|obs| obs.response_text.as_deref())
        .filter(|text| !text.trim().is_empty())
        .collect();
    if texts.is_empty() {
        return None;
    }
    let count = texts.len() as f64;
    let mut readability = 0.0;
    let mut richness = 0.0;
    let mut words = 0.0;
    for text in &texts {
        let metrics = calculate_quality_metrics(text);
        readability += metrics.readability;
        richness += metrics.vocabulary_richness;
        words += metrics.word_count as f64;
    }
    Some(AggregateTextQuality {
        sampled_responses: texts.len(),
        avg_readability: readability / count,
        avg_vocabulary_richness: richness / count,
        avg_word_count: words / count,
    })
}

/// Evaluates a single check against a single observation.
fn evaluate_check(
    check: &QaCheck,
    regex: Option<&Regex>,
    obs: &ResponseObservation,
) -> QaCheckOutcome {
    match check {
        QaCheck::Succeeded => QaCheckOutcome::from_bool(obs.is_success()),
        QaCheck::MaxLatencyMs(limit) => QaCheckOutcome::from_bool(obs.latency_ms <= *limit),
        QaCheck::NonEmpty => match obs.response_text.as_deref() {
            Some(text) => QaCheckOutcome::from_bool(!text.trim().is_empty()),
            None => QaCheckOutcome::NotApplicable,
        },
        QaCheck::MinLength(min) => match obs.response_text.as_deref() {
            Some(text) => QaCheckOutcome::from_bool(text.chars().count() >= *min),
            None => QaCheckOutcome::NotApplicable,
        },
        QaCheck::MaxLength(max) => match obs.response_text.as_deref() {
            Some(text) => QaCheckOutcome::from_bool(text.chars().count() <= *max),
            None => QaCheckOutcome::NotApplicable,
        },
        QaCheck::MustContainAny(terms) => match obs.response_text.as_deref() {
            Some(text) => {
                let lower = text.to_lowercase();
                QaCheckOutcome::from_bool(
                    terms
                        .iter()
                        .any(|term| lower.contains(&term.to_lowercase())),
                )
            }
            None => QaCheckOutcome::NotApplicable,
        },
        QaCheck::MustNotContain(terms) => match obs.response_text.as_deref() {
            Some(text) => {
                let lower = text.to_lowercase();
                QaCheckOutcome::from_bool(
                    !terms
                        .iter()
                        .any(|term| lower.contains(&term.to_lowercase())),
                )
            }
            None => QaCheckOutcome::NotApplicable,
        },
        QaCheck::NoRefusal(markers) => match obs.response_text.as_deref() {
            Some(text) => {
                let lower = text.to_lowercase();
                QaCheckOutcome::from_bool(
                    !markers
                        .iter()
                        .any(|marker| lower.contains(&marker.to_lowercase())),
                )
            }
            None => QaCheckOutcome::NotApplicable,
        },
        QaCheck::MatchesPattern(_) => match (regex, obs.response_text.as_deref()) {
            (Some(regex), Some(text)) => QaCheckOutcome::from_bool(regex.is_match(text)),
            (None, _) => QaCheckOutcome::NotApplicable,
            (_, None) => QaCheckOutcome::NotApplicable,
        },
        QaCheck::ValidJson => match obs.response_text.as_deref() {
            Some(text) => QaCheckOutcome::from_bool(
                serde_json::from_str::<serde_json::Value>(text.trim()).is_ok(),
            ),
            None => QaCheckOutcome::NotApplicable,
        },
    }
}

/// Returns the failing checks grouped by name with their failure counts.
///
/// A convenience over a [`QaReport`] for surfacing the worst offenders.
pub fn worst_failing_checks(report: &QaReport) -> BTreeMap<String, usize> {
    report
        .checks
        .iter()
        .filter(|stats| stats.failed > 0)
        .map(|stats| (stats.name.clone(), stats.failed))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(text: &str) -> ResponseObservation {
        ResponseObservation::new("openai", "gpt-4").with_response(text)
    }

    #[test]
    fn test_basic_checks() {
        let evaluator = QaEvaluator::new(vec![
            QaCheck::Succeeded,
            QaCheck::NonEmpty,
            QaCheck::MinLength(10),
        ]);
        let good = ok("This is a sufficiently long response.");
        let outcomes = evaluator.evaluate_one(&good);
        assert!(outcomes.iter().all(|o| *o == QaCheckOutcome::Pass));

        let short = ok("hi");
        let outcomes = evaluator.evaluate_one(&short);
        assert_eq!(outcomes[2], QaCheckOutcome::Fail);
    }

    #[test]
    fn test_report_aggregation() {
        let evaluator = QaEvaluator::new(vec![QaCheck::MinLength(20)]);
        let observations = vec![
            ok("This response is definitely long enough to pass."),
            ok("This one too is plenty long enough."),
            ok("too short"),
        ];
        let report = evaluator.evaluate(&observations);
        assert_eq!(report.total_observations, 3);
        assert_eq!(report.fully_passing, 2);
        assert!((report.overall_pass_rate() - 2.0 / 3.0).abs() < 1e-9);
        assert_eq!(report.checks.len(), 1);
        assert_eq!(report.checks[0].passed, 2);
        assert_eq!(report.checks[0].failed, 1);
        assert_eq!(report.failing_observation_ids.len(), 1);
    }

    #[test]
    fn test_no_refusal_check() {
        let evaluator = QaEvaluator::legal_default();
        let refusal = ok("I cannot help with that request.");
        let report = evaluator.evaluate(&[refusal]);
        let refusal_stats = report
            .checks
            .iter()
            .find(|c| c.name == "no_refusal")
            .expect("present");
        assert_eq!(refusal_stats.failed, 1);
    }

    #[test]
    fn test_valid_json_check() {
        let evaluator = QaEvaluator::new(vec![QaCheck::ValidJson]);
        let valid = ok("{\"verdict\": \"liable\"}");
        let invalid = ok("not json at all");
        assert_eq!(evaluator.evaluate_one(&valid)[0], QaCheckOutcome::Pass);
        assert_eq!(evaluator.evaluate_one(&invalid)[0], QaCheckOutcome::Fail);
    }

    #[test]
    fn test_pattern_check_and_invalid_pattern() {
        let evaluator = QaEvaluator::new(vec![QaCheck::MatchesPattern(r"\d{4}".to_string())]);
        let has_year = ok("Decided in 2024 by the court.");
        let no_year = ok("Decided recently by the court.");
        assert_eq!(evaluator.evaluate_one(&has_year)[0], QaCheckOutcome::Pass);
        assert_eq!(evaluator.evaluate_one(&no_year)[0], QaCheckOutcome::Fail);

        // An invalid regex degrades to NotApplicable, never panics.
        let broken = QaEvaluator::new(vec![QaCheck::MatchesPattern("(".to_string())]);
        assert_eq!(
            broken.evaluate_one(&has_year)[0],
            QaCheckOutcome::NotApplicable
        );
    }

    #[test]
    fn test_not_applicable_when_no_text() {
        let evaluator = QaEvaluator::new(vec![QaCheck::NonEmpty, QaCheck::Succeeded]);
        let no_text = ResponseObservation::new("openai", "gpt-4").with_latency(100);
        let outcomes = evaluator.evaluate_one(&no_text);
        assert_eq!(outcomes[0], QaCheckOutcome::NotApplicable);
        assert_eq!(outcomes[1], QaCheckOutcome::Pass);
    }

    #[test]
    fn test_text_quality_and_worst_checks() {
        let evaluator = QaEvaluator::new(vec![QaCheck::MinLength(1000)]);
        let report = evaluator.evaluate(&[ok("A reasonably readable legal sentence here.")]);
        assert!(report.text_quality.is_some());
        let worst = worst_failing_checks(&report);
        assert_eq!(worst.get("min_length_1000"), Some(&1));
    }
}
