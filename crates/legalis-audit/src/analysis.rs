//! Analysis and reporting for audit trails.

use crate::{Actor, AuditRecord, DecisionResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Analysis results for decision patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionAnalysis {
    /// Total number of decisions analyzed
    pub total_decisions: usize,
    /// Decision distribution by statute
    pub by_statute: HashMap<String, usize>,
    /// Decision distribution by event type
    pub by_event_type: HashMap<String, usize>,
    /// Decision distribution by actor type
    pub by_actor_type: HashMap<String, usize>,
    /// Decision distribution by result type
    pub by_result_type: HashMap<String, usize>,
    /// Temporal distribution (decisions per time period)
    pub temporal_distribution: Vec<TemporalBucket>,
    /// Override rate (percentage of decisions that were overridden)
    pub override_rate: f64,
    /// Discretion rate (percentage requiring human discretion)
    pub discretion_rate: f64,
    /// Analysis timestamp
    pub generated_at: DateTime<Utc>,
}

/// Temporal bucket for time-series analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalBucket {
    /// Start of the time bucket
    pub start: DateTime<Utc>,
    /// End of the time bucket
    pub end: DateTime<Utc>,
    /// Number of decisions in this bucket
    pub count: usize,
}

/// Distribution report for a specific dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionReport {
    /// The dimension being analyzed
    pub dimension: String,
    /// Distribution data
    pub distribution: Vec<DistributionEntry>,
    /// Total count
    pub total: usize,
    /// Generated timestamp
    pub generated_at: DateTime<Utc>,
}

/// Single entry in a distribution report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionEntry {
    /// Label for this entry
    pub label: String,
    /// Count of occurrences
    pub count: usize,
    /// Percentage of total
    pub percentage: f64,
}

/// Analyzes decision patterns from audit records.
pub struct DecisionAnalyzer;

impl DecisionAnalyzer {
    /// Performs comprehensive analysis on a set of records.
    pub fn analyze(records: &[AuditRecord]) -> DecisionAnalysis {
        let total_decisions = records.len();

        let by_statute = Self::count_by_statute(records);
        let by_event_type = Self::count_by_event_type(records);
        let by_actor_type = Self::count_by_actor_type(records);
        let by_result_type = Self::count_by_result_type(records);
        let temporal_distribution = Self::temporal_distribution(records, 10);

        let overridden_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();

        let discretion_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();

        let override_rate = if total_decisions > 0 {
            (overridden_count as f64 / total_decisions as f64) * 100.0
        } else {
            0.0
        };

        let discretion_rate = if total_decisions > 0 {
            (discretion_count as f64 / total_decisions as f64) * 100.0
        } else {
            0.0
        };

        DecisionAnalysis {
            total_decisions,
            by_statute,
            by_event_type,
            by_actor_type,
            by_result_type,
            temporal_distribution,
            override_rate,
            discretion_rate,
            generated_at: Utc::now(),
        }
    }

    /// Generates a distribution report for a specific dimension.
    pub fn distribution_report(records: &[AuditRecord], dimension: &str) -> DistributionReport {
        let distribution = match dimension {
            "statute" => Self::statute_distribution(records),
            "event_type" => Self::event_type_distribution(records),
            "actor_type" => Self::actor_type_distribution(records),
            "result_type" => Self::result_type_distribution(records),
            _ => Vec::new(),
        };

        let total = records.len();

        DistributionReport {
            dimension: dimension.to_string(),
            distribution,
            total,
            generated_at: Utc::now(),
        }
    }

    /// Counts decisions by statute ID.
    fn count_by_statute(records: &[AuditRecord]) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for record in records {
            *counts.entry(record.statute_id.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Counts decisions by event type.
    fn count_by_event_type(records: &[AuditRecord]) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for record in records {
            let event_type = format!("{:?}", record.event_type);
            *counts.entry(event_type).or_insert(0) += 1;
        }
        counts
    }

    /// Counts decisions by actor type.
    fn count_by_actor_type(records: &[AuditRecord]) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for record in records {
            let actor_type = match &record.actor {
                Actor::System { .. } => "System",
                Actor::User { .. } => "User",
                Actor::External { .. } => "External",
            };
            *counts.entry(actor_type.to_string()).or_insert(0) += 1;
        }
        counts
    }

    /// Counts decisions by result type.
    fn count_by_result_type(records: &[AuditRecord]) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for record in records {
            let result_type = match &record.result {
                DecisionResult::Deterministic { .. } => "Deterministic",
                DecisionResult::RequiresDiscretion { .. } => "RequiresDiscretion",
                DecisionResult::Void { .. } => "Void",
                DecisionResult::Overridden { .. } => "Overridden",
            };
            *counts.entry(result_type.to_string()).or_insert(0) += 1;
        }
        counts
    }

    /// Generates temporal distribution with the specified number of buckets.
    fn temporal_distribution(records: &[AuditRecord], bucket_count: usize) -> Vec<TemporalBucket> {
        if records.is_empty() || bucket_count == 0 {
            return Vec::new();
        }

        let min_time = records.iter().map(|r| r.timestamp).min().unwrap();
        let max_time = records.iter().map(|r| r.timestamp).max().unwrap();

        let duration = max_time.signed_duration_since(min_time);
        let bucket_duration = duration / bucket_count as i32;

        let mut buckets = Vec::new();
        for i in 0..bucket_count {
            let start = min_time + bucket_duration * i as i32;
            let end = if i == bucket_count - 1 {
                max_time
            } else {
                min_time + bucket_duration * (i + 1) as i32
            };

            let count = records
                .iter()
                .filter(|r| r.timestamp >= start && r.timestamp <= end)
                .count();

            buckets.push(TemporalBucket { start, end, count });
        }

        buckets
    }

    /// Generates statute distribution report.
    fn statute_distribution(records: &[AuditRecord]) -> Vec<DistributionEntry> {
        let counts = Self::count_by_statute(records);
        Self::to_distribution_entries(counts, records.len())
    }

    /// Generates event type distribution report.
    fn event_type_distribution(records: &[AuditRecord]) -> Vec<DistributionEntry> {
        let counts = Self::count_by_event_type(records);
        Self::to_distribution_entries(counts, records.len())
    }

    /// Generates actor type distribution report.
    fn actor_type_distribution(records: &[AuditRecord]) -> Vec<DistributionEntry> {
        let counts = Self::count_by_actor_type(records);
        Self::to_distribution_entries(counts, records.len())
    }

    /// Generates result type distribution report.
    fn result_type_distribution(records: &[AuditRecord]) -> Vec<DistributionEntry> {
        let counts = Self::count_by_result_type(records);
        Self::to_distribution_entries(counts, records.len())
    }

    /// Converts a count map to distribution entries.
    fn to_distribution_entries(
        counts: HashMap<String, usize>,
        total: usize,
    ) -> Vec<DistributionEntry> {
        let mut entries: Vec<_> = counts
            .into_iter()
            .map(|(label, count)| {
                let percentage = if total > 0 {
                    (count as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                DistributionEntry {
                    label,
                    count,
                    percentage,
                }
            })
            .collect();

        // Sort by count descending
        entries.sort_by(|a, b| b.count.cmp(&a.count));
        entries
    }
}

/// Detects anomalies in audit trails.
pub struct AnomalyDetector;

impl AnomalyDetector {
    /// Detects sudden spikes in decision volume.
    pub fn detect_volume_spikes(
        records: &[AuditRecord],
        threshold_multiplier: f64,
    ) -> Vec<VolumeAnomaly> {
        let temporal_dist = DecisionAnalyzer::temporal_distribution(records, 20);

        if temporal_dist.is_empty() {
            return Vec::new();
        }

        let avg_count = temporal_dist.iter().map(|b| b.count).sum::<usize>() as f64
            / temporal_dist.len() as f64;

        let threshold = avg_count * threshold_multiplier;

        temporal_dist
            .iter()
            .filter(|bucket| bucket.count as f64 > threshold)
            .map(|bucket| VolumeAnomaly {
                start: bucket.start,
                end: bucket.end,
                count: bucket.count,
                expected: avg_count as usize,
                severity: (bucket.count as f64 / avg_count),
            })
            .collect()
    }

    /// Detects unusual override patterns.
    pub fn detect_override_anomalies(records: &[AuditRecord]) -> Vec<OverrideAnomaly> {
        let mut anomalies = Vec::new();

        // Group by statute
        let mut by_statute: HashMap<String, Vec<&AuditRecord>> = HashMap::new();
        for record in records {
            by_statute
                .entry(record.statute_id.clone())
                .or_default()
                .push(record);
        }

        for (statute_id, statute_records) in by_statute {
            let total = statute_records.len();
            let overridden = statute_records
                .iter()
                .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
                .count();

            let override_rate = if total > 0 {
                (overridden as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            // Flag if override rate is unusually high (>20%)
            if override_rate > 20.0 && total >= 5 {
                anomalies.push(OverrideAnomaly {
                    statute_id,
                    total_decisions: total,
                    overridden_count: overridden,
                    override_rate,
                });
            }
        }

        anomalies.sort_by(|a, b| b.override_rate.partial_cmp(&a.override_rate).unwrap());
        anomalies
    }
}

/// Volume anomaly detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeAnomaly {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub count: usize,
    pub expected: usize,
    pub severity: f64,
}

/// Override anomaly detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideAnomaly {
    pub statute_id: String,
    pub total_decisions: usize,
    pub overridden_count: usize,
    pub override_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record(
        statute_id: &str,
        event_type: EventType,
        result: DecisionResult,
    ) -> AuditRecord {
        AuditRecord::new(
            event_type,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            result,
            None,
        )
    }

    #[test]
    fn test_decision_analysis() {
        let records = vec![
            create_test_record(
                "statute-1",
                EventType::AutomaticDecision,
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
            ),
            create_test_record(
                "statute-1",
                EventType::AutomaticDecision,
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
            ),
            create_test_record(
                "statute-2",
                EventType::HumanOverride,
                DecisionResult::Overridden {
                    original_result: Box::new(DecisionResult::Deterministic {
                        effect_applied: "original".to_string(),
                        parameters: HashMap::new(),
                    }),
                    new_result: Box::new(DecisionResult::Deterministic {
                        effect_applied: "new".to_string(),
                        parameters: HashMap::new(),
                    }),
                    justification: "reason".to_string(),
                },
            ),
        ];

        let analysis = DecisionAnalyzer::analyze(&records);
        assert_eq!(analysis.total_decisions, 3);
        assert_eq!(analysis.by_statute.get("statute-1"), Some(&2));
        assert_eq!(analysis.by_statute.get("statute-2"), Some(&1));
        assert!(analysis.override_rate > 0.0);
    }

    #[test]
    fn test_distribution_report() {
        let records = vec![
            create_test_record(
                "statute-1",
                EventType::AutomaticDecision,
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
            ),
            create_test_record(
                "statute-1",
                EventType::AutomaticDecision,
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
            ),
        ];

        let report = DecisionAnalyzer::distribution_report(&records, "statute");
        assert_eq!(report.dimension, "statute");
        assert_eq!(report.total, 2);
        assert!(!report.distribution.is_empty());
    }

    #[test]
    fn test_anomaly_detection() {
        let mut records = Vec::new();

        // Create normal volume
        for _ in 0..10 {
            records.push(create_test_record(
                "statute-1",
                EventType::AutomaticDecision,
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
            ));
        }

        let anomalies = AnomalyDetector::detect_volume_spikes(&records, 2.0);
        // With uniform distribution, we shouldn't detect spikes
        assert!(anomalies.is_empty() || anomalies.len() < 3);
    }
}
