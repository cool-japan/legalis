//! Generic categorical pattern-analysis engine.
//!
//! [`PatternAnalyzer`] is a deterministic, offline engine for discovering
//! repeatable patterns in a corpus of [`LegalEvent`]s by treating each event's
//! `category` as an *outcome* and its `attributes` as *dimensions*. It powers
//! several v0.5.7 analytics items as a single generic engine over
//! caller-supplied data:
//!
//! * **Judge decision pattern analysis** - set the outcome to the disposition
//!   and group by the `judge` attribute to see each judge's outcome
//!   distribution and how it deviates from the baseline.
//! * **Settlement pattern recognition** - set the outcome to "settled"/"tried"
//!   and group / segment by claim type, value band, jurisdiction, etc.
//!
//! It computes outcome base rates, per-segment conditional distributions, the
//! statistical *association* between an attribute value and an outcome (support,
//! confidence, lift and pointwise mutual information), and the Shannon entropy /
//! most-predictive segments. No machine-learned model and no LLM are involved.

use super::LegalEvent;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A discrete distribution over outcome labels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcomeDistribution {
    /// Total number of observations.
    pub total: usize,
    /// Count per outcome label (ordered by label).
    pub counts: BTreeMap<String, usize>,
    /// Probability per outcome label (counts normalised by total).
    pub probabilities: BTreeMap<String, f64>,
    /// Shannon entropy of the distribution, in bits.
    pub entropy_bits: f64,
}

impl OutcomeDistribution {
    /// Builds a distribution from outcome counts.
    pub fn from_counts(counts: BTreeMap<String, usize>) -> Self {
        let total: usize = counts.values().sum();
        let mut probabilities = BTreeMap::new();
        let mut entropy = 0.0;
        if total > 0 {
            for (label, &count) in &counts {
                let p = count as f64 / total as f64;
                probabilities.insert(label.clone(), p);
                if p > 0.0 {
                    entropy -= p * p.log2();
                }
            }
        }
        Self {
            total,
            counts,
            probabilities,
            entropy_bits: entropy,
        }
    }

    /// Returns the probability of an outcome label (0.0 if unseen).
    pub fn probability(&self, label: &str) -> f64 {
        self.probabilities.get(label).copied().unwrap_or(0.0)
    }

    /// Returns the most likely outcome label and its probability, if any.
    pub fn mode(&self) -> Option<(String, f64)> {
        self.probabilities
            .iter()
            .max_by(|a, b| {
                a.1.partial_cmp(b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| b.0.cmp(a.0))
            })
            .map(|(label, &p)| (label.clone(), p))
    }
}

/// The conditional outcome distribution for one value of a grouping attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentPattern {
    /// The grouping attribute key (e.g. `judge`).
    pub attribute: String,
    /// The attribute value defining this segment (e.g. `Smith`).
    pub value: String,
    /// Outcome distribution conditioned on this segment.
    pub distribution: OutcomeDistribution,
    /// How much this segment's entropy is *reduced* versus the baseline (the
    /// information gain, in bits): higher means the segment is more predictive.
    pub information_gain_bits: f64,
}

/// A measured association between an attribute value and an outcome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcomeAssociation {
    /// The grouping attribute key.
    pub attribute: String,
    /// The attribute value.
    pub value: String,
    /// The outcome label.
    pub outcome: String,
    /// Support: P(value AND outcome) over the whole corpus.
    pub support: f64,
    /// Confidence: P(outcome | value).
    pub confidence: f64,
    /// Lift: confidence / base rate of the outcome (>1 means positively
    /// associated, <1 negatively).
    pub lift: f64,
    /// Pointwise mutual information between value and outcome, in bits.
    pub pmi_bits: f64,
    /// Number of observations supporting the association.
    pub count: usize,
}

/// Configuration for pattern mining.
#[derive(Debug, Clone)]
pub struct PatternOptions {
    /// Minimum number of observations a segment / association must have to be
    /// reported (filters out noise from tiny samples).
    pub min_support_count: usize,
    /// Maximum number of associations to return, ranked by absolute log-lift.
    pub top_associations: usize,
}

impl Default for PatternOptions {
    fn default() -> Self {
        Self {
            min_support_count: 1,
            top_associations: 50,
        }
    }
}

/// A complete pattern-analysis report for one grouping attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternReport {
    /// The grouping attribute analysed.
    pub attribute: String,
    /// Baseline (marginal) outcome distribution across all events.
    pub baseline: OutcomeDistribution,
    /// Per-segment conditional distributions (ordered by attribute value).
    pub segments: Vec<SegmentPattern>,
    /// Notable outcome associations (ranked by strength).
    pub associations: Vec<OutcomeAssociation>,
}

/// Mines categorical patterns from a corpus of legal events.
#[derive(Debug, Clone, Default)]
pub struct PatternAnalyzer {
    options: PatternOptions,
}

impl PatternAnalyzer {
    /// Creates an analyzer with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an analyzer with explicit options.
    pub fn with_options(options: PatternOptions) -> Self {
        Self { options }
    }

    /// Computes the baseline outcome distribution (over event categories).
    pub fn baseline(&self, events: &[LegalEvent]) -> OutcomeDistribution {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for event in events {
            *counts.entry(event.category.clone()).or_insert(0) += 1;
        }
        OutcomeDistribution::from_counts(counts)
    }

    /// Computes per-segment conditional outcome distributions for one grouping
    /// attribute, with each segment's information gain relative to the baseline.
    pub fn segment_by(&self, events: &[LegalEvent], attribute: &str) -> Vec<SegmentPattern> {
        let baseline = self.baseline(events);
        let mut grouped: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
        for event in events {
            if let Some(value) = event.attributes.get(attribute) {
                *grouped
                    .entry(value.clone())
                    .or_default()
                    .entry(event.category.clone())
                    .or_insert(0) += 1;
            }
        }

        grouped
            .into_iter()
            .filter(|(_, counts)| counts.values().sum::<usize>() >= self.options.min_support_count)
            .map(|(value, counts)| {
                let distribution = OutcomeDistribution::from_counts(counts);
                let information_gain_bits = baseline.entropy_bits - distribution.entropy_bits;
                SegmentPattern {
                    attribute: attribute.to_string(),
                    value,
                    distribution,
                    information_gain_bits,
                }
            })
            .collect()
    }

    /// Mines outcome associations for one grouping attribute.
    ///
    /// For every (attribute value, outcome) pair meeting the support threshold
    /// it computes support, confidence, lift and PMI, then returns the strongest
    /// associations ranked by `|ln(lift)|` (most over- or under-represented).
    pub fn associations(&self, events: &[LegalEvent], attribute: &str) -> Vec<OutcomeAssociation> {
        let total = events.len();
        if total == 0 {
            return Vec::new();
        }
        let baseline = self.baseline(events);

        // Joint and marginal counts.
        let mut joint: BTreeMap<(String, String), usize> = BTreeMap::new();
        let mut value_totals: BTreeMap<String, usize> = BTreeMap::new();
        for event in events {
            if let Some(value) = event.attributes.get(attribute) {
                *joint
                    .entry((value.clone(), event.category.clone()))
                    .or_insert(0) += 1;
                *value_totals.entry(value.clone()).or_insert(0) += 1;
            }
        }

        let total_f = total as f64;
        let mut associations: Vec<OutcomeAssociation> = joint
            .into_iter()
            .filter(|(_, count)| *count >= self.options.min_support_count)
            .map(|((value, outcome), count)| {
                let value_total = value_totals.get(&value).copied().unwrap_or(count);
                let base_rate = baseline.probability(&outcome).max(1e-12);
                let support = count as f64 / total_f;
                let confidence = if value_total > 0 {
                    count as f64 / value_total as f64
                } else {
                    0.0
                };
                let lift = confidence / base_rate;
                // PMI = log2( P(v,o) / (P(v) P(o)) ).
                let p_value = value_total as f64 / total_f;
                let pmi_bits = if p_value > 0.0 && base_rate > 0.0 {
                    (support / (p_value * base_rate)).log2()
                } else {
                    0.0
                };
                OutcomeAssociation {
                    attribute: attribute.to_string(),
                    value,
                    outcome,
                    support,
                    confidence,
                    lift,
                    pmi_bits,
                    count,
                }
            })
            .collect();

        associations.sort_by(|a, b| {
            let la = a.lift.max(1e-12).ln().abs();
            let lb = b.lift.max(1e-12).ln().abs();
            lb.partial_cmp(&la)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.count.cmp(&a.count))
                .then_with(|| a.value.cmp(&b.value))
                .then_with(|| a.outcome.cmp(&b.outcome))
        });
        associations.truncate(self.options.top_associations);
        associations
    }

    /// Produces a full [`PatternReport`] for one grouping attribute.
    pub fn analyze(&self, events: &[LegalEvent], attribute: &str) -> PatternReport {
        PatternReport {
            attribute: attribute.to_string(),
            baseline: self.baseline(events),
            segments: self.segment_by(events, attribute),
            associations: self.associations(events, attribute),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn decision(id: &str, judge: &str, outcome: &str) -> LegalEvent {
        let ts = Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .expect("valid");
        LegalEvent::new(id, ts, outcome).with_attribute("judge", judge)
    }

    /// A corpus where Judge Strict denies most motions and Judge Lenient grants.
    fn judge_corpus() -> Vec<LegalEvent> {
        vec![
            decision("1", "Strict", "denied"),
            decision("2", "Strict", "denied"),
            decision("3", "Strict", "denied"),
            decision("4", "Strict", "granted"),
            decision("5", "Lenient", "granted"),
            decision("6", "Lenient", "granted"),
            decision("7", "Lenient", "granted"),
            decision("8", "Lenient", "denied"),
        ]
    }

    #[test]
    fn test_baseline_distribution() {
        let analyzer = PatternAnalyzer::new();
        let dist = analyzer.baseline(&judge_corpus());
        assert_eq!(dist.total, 8);
        // 4 denied, 4 granted => entropy 1 bit, each p = 0.5.
        assert!((dist.probability("denied") - 0.5).abs() < 1e-9);
        assert!((dist.probability("granted") - 0.5).abs() < 1e-9);
        assert!((dist.entropy_bits - 1.0).abs() < 1e-9);
        let (mode, p) = dist.mode().expect("mode");
        assert!(p >= 0.5);
        assert!(mode == "denied" || mode == "granted");
    }

    #[test]
    fn test_segment_distributions_and_information_gain() {
        let analyzer = PatternAnalyzer::new();
        let segments = analyzer.segment_by(&judge_corpus(), "judge");
        assert_eq!(segments.len(), 2);
        let strict = segments
            .iter()
            .find(|s| s.value == "Strict")
            .expect("strict");
        // Strict denies 3/4.
        assert!((strict.distribution.probability("denied") - 0.75).abs() < 1e-9);
        // Conditioning on judge reduces entropy below the 1-bit baseline.
        assert!(strict.information_gain_bits > 0.0);
        assert!(strict.distribution.entropy_bits < 1.0);
    }

    #[test]
    fn test_associations_lift() {
        let analyzer = PatternAnalyzer::new();
        let assocs = analyzer.associations(&judge_corpus(), "judge");
        assert!(!assocs.is_empty());
        // Strict->denied should have lift > 1 (over-represented vs 50% base).
        let strict_denied = assocs
            .iter()
            .find(|a| a.value == "Strict" && a.outcome == "denied")
            .expect("strict denied assoc");
        assert!(strict_denied.lift > 1.0);
        assert!(strict_denied.pmi_bits > 0.0);
        assert!((strict_denied.confidence - 0.75).abs() < 1e-9);
        // Lenient->denied should be under-represented (lift < 1).
        let lenient_denied = assocs
            .iter()
            .find(|a| a.value == "Lenient" && a.outcome == "denied")
            .expect("lenient denied assoc");
        assert!(lenient_denied.lift < 1.0);
        assert!(lenient_denied.pmi_bits < 0.0);
    }

    #[test]
    fn test_settlement_pattern_use_case() {
        // Generic engine reused for settlement recognition: outcome = settled/tried,
        // grouped by claim value band.
        let analyzer = PatternAnalyzer::new();
        let ts = Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .expect("valid");
        let mk = |id: &str, band: &str, outcome: &str| {
            LegalEvent::new(id, ts, outcome).with_attribute("value_band", band)
        };
        let events = vec![
            mk("1", "high", "settled"),
            mk("2", "high", "settled"),
            mk("3", "high", "settled"),
            mk("4", "low", "tried"),
            mk("5", "low", "tried"),
            mk("6", "low", "settled"),
        ];
        let report = analyzer.analyze(&events, "value_band");
        let high = report
            .segments
            .iter()
            .find(|s| s.value == "high")
            .expect("high band");
        assert!((high.distribution.probability("settled") - 1.0).abs() < 1e-9);
        let high_settled = report
            .associations
            .iter()
            .find(|a| a.value == "high" && a.outcome == "settled")
            .expect("assoc");
        assert!(high_settled.lift > 1.0);
    }

    #[test]
    fn test_min_support_filters_noise() {
        let analyzer = PatternAnalyzer::with_options(PatternOptions {
            min_support_count: 3,
            top_associations: 50,
        });
        let mut events = judge_corpus();
        // Add a rare judge with a single observation; should be filtered.
        events.push(decision("9", "Rare", "granted"));
        let segments = analyzer.segment_by(&events, "judge");
        assert!(segments.iter().all(|s| s.value != "Rare"));
    }

    #[test]
    fn test_empty_corpus() {
        let analyzer = PatternAnalyzer::new();
        assert_eq!(analyzer.baseline(&[]).total, 0);
        assert!(analyzer.associations(&[], "judge").is_empty());
        assert!(analyzer.segment_by(&[], "judge").is_empty());
    }
}
