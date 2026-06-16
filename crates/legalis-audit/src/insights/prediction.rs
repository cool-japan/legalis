//! Pattern-based outcome prediction via a first-order Markov model.
//!
//! Audit decisions about a single subject (or under a single statute) form a
//! sequence; the *transition frequencies* between successive outcomes encode a
//! great deal about how a matter is likely to evolve. This module learns a
//! first-order Markov model — a Laplace-smoothed transition matrix over
//! [`OutcomeCategory`] states — and uses it to:
//!
//! - predict the most likely *next* outcome for an in-flight subject;
//! - flag *improbable transitions* that have already occurred (a transition the
//!   learned model assigns near-zero probability is, by definition, surprising);
//! - flag *outcome drift*, where a recent population's outcome mix diverges from
//!   the long-run baseline (measured by total-variation distance).
//!
//! This is deliberately distinct from [`crate::predictive`], which forecasts
//! discrete compliance *violations*; here we model the *sequence dynamics* of
//! ordinary decision outcomes.

use crate::insights::finding::{AuditFinding, BlastRadius, FindingKind, Likelihood, Severity};
use crate::{AuditError, AuditRecord, AuditResult, DecisionResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// A coarse outcome class derived from a decision record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OutcomeCategory {
    /// A deterministic decision that approved / granted something.
    Approved,
    /// A deterministic decision that denied / rejected something.
    Denied,
    /// Any other deterministic effect.
    OtherDeterministic,
    /// A decision routed to human discretion.
    Discretionary,
    /// A decision overridden by a human.
    Overridden,
    /// A decision voided due to a logical error.
    Void,
}

impl OutcomeCategory {
    /// Every category, in a stable order (used as the smoothing support).
    pub const ALL: [OutcomeCategory; 6] = [
        OutcomeCategory::Approved,
        OutcomeCategory::Denied,
        OutcomeCategory::OtherDeterministic,
        OutcomeCategory::Discretionary,
        OutcomeCategory::Overridden,
        OutcomeCategory::Void,
    ];

    /// Classifies a record's [`DecisionResult`] into an outcome category.
    pub fn from_record(record: &AuditRecord) -> Self {
        match &record.result {
            DecisionResult::Deterministic { effect_applied, .. } => {
                let lower = effect_applied.to_lowercase();
                if lower.contains("approv") || lower.contains("grant") || lower.contains("allow") {
                    OutcomeCategory::Approved
                } else if lower.contains("den")
                    || lower.contains("reject")
                    || lower.contains("refus")
                    || lower.contains("block")
                {
                    OutcomeCategory::Denied
                } else {
                    OutcomeCategory::OtherDeterministic
                }
            }
            DecisionResult::RequiresDiscretion { .. } => OutcomeCategory::Discretionary,
            DecisionResult::Overridden { .. } => OutcomeCategory::Overridden,
            DecisionResult::Void { .. } => OutcomeCategory::Void,
        }
    }

    /// Returns `true` for outcomes that generally warrant follow-up.
    pub fn is_adverse(self) -> bool {
        matches!(
            self,
            OutcomeCategory::Denied | OutcomeCategory::Overridden | OutcomeCategory::Void
        )
    }

    /// A stable lower-case label.
    pub fn label(self) -> &'static str {
        match self {
            OutcomeCategory::Approved => "approved",
            OutcomeCategory::Denied => "denied",
            OutcomeCategory::OtherDeterministic => "other_deterministic",
            OutcomeCategory::Discretionary => "discretionary",
            OutcomeCategory::Overridden => "overridden",
            OutcomeCategory::Void => "void",
        }
    }
}

/// Determines how records are grouped into ordered sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SequenceKey {
    /// One sequence per subject.
    Subject,
    /// One sequence per statute.
    Statute,
}

/// Tuning parameters for outcome prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomePredictionConfig {
    /// How to group records into sequences.
    pub key: SequenceKey,
    /// Laplace (additive) smoothing constant applied to transition counts.
    pub smoothing_alpha: f64,
    /// Minimum observed transitions from a state for predictions / improbable
    /// flags originating at that state to be trusted.
    pub min_support: usize,
    /// Probability at or below which an observed transition is "improbable".
    pub improbable_threshold: f64,
    /// Total-variation distance above which outcome drift is flagged.
    pub drift_threshold: f64,
}

impl Default for OutcomePredictionConfig {
    fn default() -> Self {
        Self {
            key: SequenceKey::Subject,
            smoothing_alpha: 0.5,
            min_support: 5,
            improbable_threshold: 0.02,
            drift_threshold: 0.3,
        }
    }
}

/// A learned first-order transition model over outcome categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionModel {
    /// `transitions[from][to]` = observed count of `from -> to`.
    pub transitions: HashMap<OutcomeCategory, HashMap<OutcomeCategory, usize>>,
    /// Total transitions observed out of each state.
    pub from_totals: HashMap<OutcomeCategory, usize>,
    /// Overall occurrence count of each category (the long-run baseline).
    pub overall: HashMap<OutcomeCategory, usize>,
    /// Total category occurrences across all sequences.
    pub total_observations: usize,
    /// Smoothing constant baked into the model.
    pub alpha: f64,
}

impl TransitionModel {
    /// Smoothed probability of transitioning `from -> to`.
    pub fn probability(&self, from: OutcomeCategory, to: OutcomeCategory) -> f64 {
        let count = self
            .transitions
            .get(&from)
            .and_then(|row| row.get(&to))
            .copied()
            .unwrap_or(0) as f64;
        let total = self.from_totals.get(&from).copied().unwrap_or(0) as f64;
        let support = OutcomeCategory::ALL.len() as f64;
        (count + self.alpha) / (total + self.alpha * support)
    }

    /// Number of transitions observed out of `from`.
    pub fn support(&self, from: OutcomeCategory) -> usize {
        self.from_totals.get(&from).copied().unwrap_or(0)
    }

    /// The full smoothed next-state distribution from `from`, sorted by
    /// descending probability.
    pub fn next_distribution(&self, from: OutcomeCategory) -> Vec<(OutcomeCategory, f64)> {
        let mut dist: Vec<(OutcomeCategory, f64)> = OutcomeCategory::ALL
            .iter()
            .map(|&to| (to, self.probability(from, to)))
            .collect();
        dist.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        dist
    }

    /// The long-run baseline distribution over categories.
    pub fn baseline_distribution(&self) -> HashMap<OutcomeCategory, f64> {
        let mut dist = HashMap::new();
        if self.total_observations == 0 {
            return dist;
        }
        for &cat in OutcomeCategory::ALL.iter() {
            let count = self.overall.get(&cat).copied().unwrap_or(0) as f64;
            dist.insert(cat, count / self.total_observations as f64);
        }
        dist
    }
}

/// A predicted next outcome for a sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomePrediction {
    /// The sequence this prediction belongs to (subject id or statute id).
    pub sequence_id: String,
    /// The conditioning (most recent) state.
    pub from_state: OutcomeCategory,
    /// The most likely next outcome.
    pub predicted: OutcomeCategory,
    /// The probability of `predicted`.
    pub probability: f64,
    /// The full next-state distribution, descending.
    pub distribution: Vec<(OutcomeCategory, f64)>,
    /// Observed transitions out of `from_state` (the evidential support).
    pub support: usize,
    /// Confidence in `[0, 1]` blending probability and support.
    pub confidence: f64,
}

/// Learns and applies a first-order outcome transition model.
#[derive(Debug, Clone)]
pub struct OutcomePredictor {
    config: OutcomePredictionConfig,
    model: Option<TransitionModel>,
}

impl OutcomePredictor {
    /// Creates a predictor with the default configuration.
    pub fn new() -> Self {
        Self::with_config(OutcomePredictionConfig::default())
    }

    /// Creates a predictor with a custom configuration.
    pub fn with_config(config: OutcomePredictionConfig) -> Self {
        Self {
            config,
            model: None,
        }
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &OutcomePredictionConfig {
        &self.config
    }

    /// Returns the learned model, if training has happened.
    pub fn model(&self) -> Option<&TransitionModel> {
        self.model.as_ref()
    }

    /// Returns `true` once [`OutcomePredictor::train`] has succeeded.
    pub fn is_trained(&self) -> bool {
        self.model.is_some()
    }

    /// Learns the transition model from historical records.
    pub fn train(&mut self, records: &[AuditRecord]) -> AuditResult<()> {
        if records.is_empty() {
            return Err(AuditError::InvalidRecord(
                "cannot train outcome predictor on an empty corpus".to_string(),
            ));
        }

        let sequences = self.build_sequences(records);
        let mut transitions: HashMap<OutcomeCategory, HashMap<OutcomeCategory, usize>> =
            HashMap::new();
        let mut from_totals: HashMap<OutcomeCategory, usize> = HashMap::new();
        let mut overall: HashMap<OutcomeCategory, usize> = HashMap::new();
        let mut total_observations = 0usize;

        for (_, seq) in sequences.iter() {
            for window in seq.windows(2) {
                let from = window[0].1;
                let to = window[1].1;
                *transitions.entry(from).or_default().entry(to).or_insert(0) += 1;
                *from_totals.entry(from).or_insert(0) += 1;
            }
            for (_, cat, _) in seq.iter() {
                *overall.entry(*cat).or_insert(0) += 1;
                total_observations += 1;
            }
        }

        self.model = Some(TransitionModel {
            transitions,
            from_totals,
            overall,
            total_observations,
            alpha: self.config.smoothing_alpha,
        });
        Ok(())
    }

    /// Predicts the next outcome given an explicit outcome history.
    ///
    /// Returns `None` when the history is empty (no conditioning state).
    pub fn predict_next(
        &self,
        sequence_id: impl Into<String>,
        history: &[OutcomeCategory],
    ) -> AuditResult<Option<OutcomePrediction>> {
        let model = self.require_model()?;
        let Some(&from_state) = history.last() else {
            return Ok(None);
        };
        let distribution = model.next_distribution(from_state);
        let (predicted, probability) = distribution.first().copied().unwrap_or((from_state, 0.0));
        let support = model.support(from_state);
        let confidence = prediction_confidence(probability, support);

        Ok(Some(OutcomePrediction {
            sequence_id: sequence_id.into(),
            from_state,
            predicted,
            probability,
            distribution,
            support,
            confidence,
        }))
    }

    /// Predicts the next outcome for one subject from raw records.
    pub fn predict_for_subject(
        &self,
        records: &[AuditRecord],
        subject_id: Uuid,
    ) -> AuditResult<Option<OutcomePrediction>> {
        self.require_model()?;
        let mut seq: Vec<(chrono::DateTime<chrono::Utc>, OutcomeCategory)> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .map(|r| (r.timestamp, OutcomeCategory::from_record(r)))
            .collect();
        seq.sort_by_key(|(ts, _)| *ts);
        let history: Vec<OutcomeCategory> = seq.into_iter().map(|(_, c)| c).collect();
        self.predict_next(subject_id.to_string(), &history)
    }

    /// Produces a prediction for every sequence with at least two states,
    /// sorted with the highest-confidence adverse predictions first.
    pub fn predict_all(&self, records: &[AuditRecord]) -> AuditResult<Vec<OutcomePrediction>> {
        self.require_model()?;
        let sequences = self.build_sequences(records);
        let mut predictions = Vec::new();
        for (id, seq) in sequences {
            if seq.len() < 2 {
                continue;
            }
            let history: Vec<OutcomeCategory> = seq.into_iter().map(|(_, c, _)| c).collect();
            if let Some(pred) = self.predict_next(id, &history)? {
                predictions.push(pred);
            }
        }
        predictions.sort_by(|a, b| {
            let a_key = a.confidence * if a.predicted.is_adverse() { 1.0 } else { 0.5 };
            let b_key = b.confidence * if b.predicted.is_adverse() { 1.0 } else { 0.5 };
            b_key.total_cmp(&a_key)
        });
        Ok(predictions)
    }

    /// Flags outcome categories that recur via transitions the model considers
    /// improbable, aggregating one finding per surprising `from -> to` pair.
    pub fn detect_improbable_transitions(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<AuditFinding>> {
        let model = self.require_model()?;
        let sequences = self.build_sequences(records);

        // Aggregate evidence per surprising (from, to) pair.
        let mut aggregated: HashMap<(OutcomeCategory, OutcomeCategory), Vec<Uuid>> = HashMap::new();
        for (_, seq) in sequences.iter() {
            for window in seq.windows(2) {
                let (from, from_id) = (window[0].1, window[0].2);
                let (to, to_id) = (window[1].1, window[1].2);
                if model.support(from) < self.config.min_support {
                    continue;
                }
                if model.probability(from, to) <= self.config.improbable_threshold {
                    let entry = aggregated.entry((from, to)).or_default();
                    entry.push(from_id);
                    entry.push(to_id);
                }
            }
        }

        let mut findings = Vec::new();
        for ((from, to), ids) in aggregated {
            let prob = model.probability(from, to);
            let occurrences = ids.len() / 2;
            let unique: HashSet<Uuid> = ids.iter().copied().collect();
            let confidence = (1.0 - prob).clamp(0.0, 1.0);
            let severity = if to.is_adverse() {
                Severity::High
            } else {
                Severity::Medium
            };
            let blast = BlastRadius::from_counts(unique.len(), occurrences.max(1), 1);
            let finding = AuditFinding::new(
                FindingKind::ImprobableTransition,
                format!("Improbable transition {} -> {}", from.label(), to.label()),
                severity,
                Likelihood::from_confidence(confidence),
                blast,
            )
            .with_description(format!(
                "The transition {} -> {} occurred {} time(s) yet the learned model assigns it only {:.4} probability, indicating an unexpected decision pathway.",
                from.label(),
                to.label(),
                occurrences,
                prob
            ))
            .with_evidence(unique.into_iter().collect())
            .with_metric("transition_probability", prob)
            .with_metric("occurrences", occurrences as f64);
            findings.push(finding);
        }

        findings.sort_by(|a, b| {
            let pa = a
                .metrics
                .get("transition_probability")
                .copied()
                .unwrap_or(1.0);
            let pb = b
                .metrics
                .get("transition_probability")
                .copied()
                .unwrap_or(1.0);
            pa.total_cmp(&pb)
        });
        Ok(findings)
    }

    /// Compares the outcome mix of `recent` records against the learned
    /// baseline; emits a drift finding when the total-variation distance
    /// exceeds the configured threshold.
    pub fn detect_outcome_drift(
        &self,
        recent: &[AuditRecord],
    ) -> AuditResult<Option<AuditFinding>> {
        let model = self.require_model()?;
        if recent.len() < self.config.min_support {
            return Ok(None);
        }

        let mut recent_counts: HashMap<OutcomeCategory, usize> = HashMap::new();
        let mut subjects: HashSet<Uuid> = HashSet::new();
        let mut statutes: HashSet<String> = HashSet::new();
        for record in recent {
            *recent_counts
                .entry(OutcomeCategory::from_record(record))
                .or_insert(0) += 1;
            subjects.insert(record.subject_id);
            statutes.insert(record.statute_id.clone());
        }
        let total = recent.len() as f64;
        let baseline = model.baseline_distribution();

        // Total-variation distance between the two distributions.
        let mut tvd = 0.0;
        for &cat in OutcomeCategory::ALL.iter() {
            let recent_p = recent_counts.get(&cat).copied().unwrap_or(0) as f64 / total;
            let base_p = baseline.get(&cat).copied().unwrap_or(0.0);
            tvd += (recent_p - base_p).abs();
        }
        tvd *= 0.5;

        if tvd < self.config.drift_threshold {
            return Ok(None);
        }

        let confidence = (tvd / (2.0 * self.config.drift_threshold)).clamp(0.0, 1.0);
        let evidence: Vec<Uuid> = recent.iter().take(100).map(|r| r.id).collect();
        let blast = BlastRadius::from_counts(recent.len(), subjects.len(), statutes.len());
        let finding = AuditFinding::new(
            FindingKind::OutcomeDrift,
            "Outcome distribution drift",
            Severity::Medium,
            Likelihood::from_confidence(confidence),
            blast,
        )
        .with_description(format!(
            "The recent outcome mix diverges from the historical baseline (total-variation distance = {tvd:.3}), suggesting a shift in decision behaviour."
        ))
        .with_evidence(evidence)
        .with_metric("total_variation_distance", tvd)
        .with_metric("sample_size", total);

        Ok(Some(finding))
    }

    fn require_model(&self) -> AuditResult<&TransitionModel> {
        self.model.as_ref().ok_or_else(|| {
            AuditError::InvalidRecord("outcome predictor must be trained first".to_string())
        })
    }

    /// Groups records into time-ordered `(timestamp, category, record id)`
    /// sequences keyed by the configured dimension.
    fn build_sequences(
        &self,
        records: &[AuditRecord],
    ) -> HashMap<String, Vec<(chrono::DateTime<chrono::Utc>, OutcomeCategory, Uuid)>> {
        let mut sequences: HashMap<
            String,
            Vec<(chrono::DateTime<chrono::Utc>, OutcomeCategory, Uuid)>,
        > = HashMap::new();
        for record in records {
            let key = match self.config.key {
                SequenceKey::Subject => record.subject_id.to_string(),
                SequenceKey::Statute => record.statute_id.clone(),
            };
            sequences.entry(key).or_default().push((
                record.timestamp,
                OutcomeCategory::from_record(record),
                record.id,
            ));
        }
        for seq in sequences.values_mut() {
            seq.sort_by_key(|(ts, _, _)| *ts);
        }
        sequences
    }
}

impl Default for OutcomePredictor {
    fn default() -> Self {
        Self::new()
    }
}

/// Blends a prediction's probability with its evidential support so that a
/// confident-looking prediction backed by little data is appropriately
/// discounted.
fn prediction_confidence(probability: f64, support: usize) -> f64 {
    let support_factor = support as f64 / (support as f64 + 5.0);
    (probability * support_factor).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, EventType};
    use chrono::{Duration, Utc};
    use std::collections::HashMap as StdHashMap;

    fn record(subject: Uuid, statute: &str, effect: &str, offset_secs: i64) -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now() + Duration::seconds(offset_secs),
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: statute.to_string(),
            subject_id: subject,
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: effect.to_string(),
                parameters: StdHashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_outcome_categorization() {
        let s = Uuid::new_v4();
        assert_eq!(
            OutcomeCategory::from_record(&record(s, "x", "approved", 0)),
            OutcomeCategory::Approved
        );
        assert_eq!(
            OutcomeCategory::from_record(&record(s, "x", "denied", 0)),
            OutcomeCategory::Denied
        );
        assert_eq!(
            OutcomeCategory::from_record(&record(s, "x", "escalate", 0)),
            OutcomeCategory::OtherDeterministic
        );
        assert!(OutcomeCategory::Denied.is_adverse());
        assert!(!OutcomeCategory::Approved.is_adverse());
    }

    #[test]
    fn test_train_and_transition_probabilities() {
        let mut predictor = OutcomePredictor::new();
        let mut records = Vec::new();
        // 20 subjects all following approved -> approved.
        for _ in 0..20 {
            let s = Uuid::new_v4();
            records.push(record(s, "stat", "approved", 0));
            records.push(record(s, "stat", "approved", 10));
        }
        predictor.train(&records).expect("train ok");
        let model = predictor.model().expect("model");
        // approved -> approved should dominate.
        let p_aa = model.probability(OutcomeCategory::Approved, OutcomeCategory::Approved);
        let p_ad = model.probability(OutcomeCategory::Approved, OutcomeCategory::Denied);
        assert!(p_aa > p_ad);
        assert!(p_aa > 0.8);
    }

    #[test]
    fn test_predict_next_distribution_sums_to_one() {
        let mut predictor = OutcomePredictor::new();
        let mut records = Vec::new();
        for _ in 0..10 {
            let s = Uuid::new_v4();
            records.push(record(s, "stat", "approved", 0));
            records.push(record(s, "stat", "denied", 10));
        }
        predictor.train(&records).expect("train ok");
        let pred = predictor
            .predict_next("seq", &[OutcomeCategory::Approved])
            .expect("ok")
            .expect("some");
        let sum: f64 = pred.distribution.iter().map(|(_, p)| p).sum();
        assert!((sum - 1.0).abs() < 1e-9);
        assert_eq!(pred.from_state, OutcomeCategory::Approved);
    }

    #[test]
    fn test_predict_next_empty_history() {
        let mut predictor = OutcomePredictor::new();
        predictor
            .train(&[record(Uuid::new_v4(), "s", "approved", 0)])
            .expect("train ok");
        let result = predictor.predict_next("seq", &[]).expect("ok");
        assert!(result.is_none());
    }

    #[test]
    fn test_predict_for_subject() {
        let mut predictor = OutcomePredictor::new();
        let subject = Uuid::new_v4();
        let mut records = Vec::new();
        for _ in 0..15 {
            let s = Uuid::new_v4();
            records.push(record(s, "stat", "approved", 0));
            records.push(record(s, "stat", "approved", 5));
        }
        records.push(record(subject, "stat", "approved", 100));
        predictor.train(&records).expect("train ok");

        let pred = predictor
            .predict_for_subject(&records, subject)
            .expect("ok")
            .expect("some");
        assert_eq!(pred.predicted, OutcomeCategory::Approved);
        assert!(pred.confidence > 0.0);
    }

    #[test]
    fn test_detect_outcome_drift() {
        let mut predictor = OutcomePredictor::new();
        // Baseline: nearly all approvals.
        let mut training = Vec::new();
        for _ in 0..50 {
            let s = Uuid::new_v4();
            training.push(record(s, "stat", "approved", 0));
            training.push(record(s, "stat", "approved", 5));
        }
        predictor.train(&training).expect("train ok");

        // Recent: nearly all denials -> large drift.
        let recent: Vec<AuditRecord> = (0..30)
            .map(|i| record(Uuid::new_v4(), "stat", "denied", i))
            .collect();
        let drift = predictor.detect_outcome_drift(&recent).expect("ok");
        assert!(drift.is_some());
        let finding = drift.expect("some");
        assert_eq!(finding.kind, FindingKind::OutcomeDrift);
        assert!(finding.metrics["total_variation_distance"] > 0.3);
    }

    #[test]
    fn test_detect_improbable_transition() {
        let mut predictor = OutcomePredictor::new();
        // Strong approved -> approved regime, lots of support, so that a single
        // approved -> void transition is diluted below the improbability floor.
        let mut records = Vec::new();
        for _ in 0..80 {
            let s = Uuid::new_v4();
            records.push(record(s, "stat", "approved", 0));
            records.push(record(s, "stat", "approved", 5));
        }
        // One subject takes a surprising approved -> void path.
        let odd = Uuid::new_v4();
        records.push(record(odd, "stat", "approved", 1000));
        let mut void_rec = record(odd, "stat", "", 2000);
        void_rec.result = DecisionResult::Void {
            reason: "logic error".to_string(),
        };
        records.push(void_rec);

        predictor.train(&records).expect("train ok");
        let findings = predictor
            .detect_improbable_transitions(&records)
            .expect("ok");
        assert!(
            findings
                .iter()
                .any(|f| f.kind == FindingKind::ImprobableTransition)
        );
    }

    #[test]
    fn test_untrained_errors() {
        let predictor = OutcomePredictor::new();
        assert!(
            predictor
                .predict_next("s", &[OutcomeCategory::Approved])
                .is_err()
        );
        assert!(predictor.detect_outcome_drift(&[]).is_err());
    }
}
