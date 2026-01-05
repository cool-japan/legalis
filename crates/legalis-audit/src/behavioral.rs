//! Behavioral pattern recognition for decision-making analysis.
//!
//! This module analyzes decision-making patterns to identify behavioral characteristics,
//! trends, and potential issues in automated and human decision processes.

use crate::{Actor, AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for behavioral pattern recognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralConfig {
    /// Minimum pattern occurrence threshold
    pub min_pattern_occurrences: usize,
    /// Temporal window for pattern detection (in hours)
    pub temporal_window_hours: i64,
    /// Enable actor behavior analysis
    pub enable_actor_analysis: bool,
    /// Enable temporal pattern analysis
    pub enable_temporal_analysis: bool,
    /// Enable decision flow analysis
    pub enable_flow_analysis: bool,
}

impl Default for BehavioralConfig {
    fn default() -> Self {
        Self {
            min_pattern_occurrences: 5,
            temporal_window_hours: 168, // 1 week
            enable_actor_analysis: true,
            enable_temporal_analysis: true,
            enable_flow_analysis: true,
        }
    }
}

/// Detected behavioral pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    /// Pattern identifier
    pub pattern_id: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Pattern description
    pub description: String,
    /// Frequency of pattern occurrence
    pub occurrence_count: usize,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Associated actors
    pub actors: Vec<String>,
    /// Associated statutes
    pub statutes: Vec<String>,
    /// Time range of pattern
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
    /// Pattern characteristics
    pub characteristics: HashMap<String, String>,
}

/// Type of behavioral pattern.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatternType {
    /// Consistent behavior pattern
    Consistent,
    /// Periodic behavior pattern
    Periodic,
    /// Escalating behavior pattern
    Escalating,
    /// Anomalous behavior pattern
    Anomalous,
    /// Sequential decision pattern
    Sequential,
    /// Parallel decision pattern
    Parallel,
    /// Override tendency pattern
    OverrideTendency,
    /// Time-based pattern
    TimeBased,
}

/// Actor behavior profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBehaviorProfile {
    /// Actor identifier
    pub actor_id: String,
    /// Total decisions made
    pub total_decisions: usize,
    /// Override rate
    pub override_rate: f64,
    /// Discretionary rate
    pub discretionary_rate: f64,
    /// Most active hours
    pub peak_hours: Vec<u32>,
    /// Most active days
    pub peak_days: Vec<String>,
    /// Favorite statutes
    pub frequent_statutes: Vec<(String, usize)>,
    /// Average decision time
    pub avg_decision_time_seconds: f64,
    /// Behavioral patterns
    pub patterns: Vec<BehavioralPattern>,
}

/// Temporal pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPattern {
    /// Pattern name
    pub name: String,
    /// Time slot (e.g., "Monday 9-10 AM")
    pub time_slot: String,
    /// Decision count in this slot
    pub decision_count: usize,
    /// Percentage of total decisions
    pub percentage: f64,
    /// Characteristics
    pub characteristics: HashMap<String, f64>,
}

/// Decision flow pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionFlowPattern {
    /// Flow identifier
    pub flow_id: String,
    /// Statute sequence
    pub statute_sequence: Vec<String>,
    /// Occurrence count
    pub occurrence_count: usize,
    /// Average duration
    pub avg_duration_seconds: f64,
    /// Success rate
    pub success_rate: f64,
}

/// Behavioral pattern recognizer.
pub struct BehavioralRecognizer {
    config: BehavioralConfig,
}

impl BehavioralRecognizer {
    /// Creates a new behavioral recognizer with default configuration.
    pub fn new() -> Self {
        Self::with_config(BehavioralConfig::default())
    }

    /// Creates a new behavioral recognizer with custom configuration.
    pub fn with_config(config: BehavioralConfig) -> Self {
        Self { config }
    }

    /// Analyzes behavioral patterns in audit records.
    pub fn analyze(&self, records: &[AuditRecord]) -> AuditResult<Vec<BehavioralPattern>> {
        let mut patterns = Vec::new();

        if self.config.enable_actor_analysis {
            patterns.extend(self.detect_actor_patterns(records)?);
        }

        if self.config.enable_temporal_analysis {
            patterns.extend(self.detect_temporal_patterns(records)?);
        }

        if self.config.enable_flow_analysis {
            patterns.extend(self.detect_flow_patterns(records)?);
        }

        // Filter by minimum occurrences
        patterns.retain(|p| p.occurrence_count >= self.config.min_pattern_occurrences);

        // Sort by confidence descending
        patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        Ok(patterns)
    }

    /// Generates actor behavior profiles.
    pub fn profile_actors(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<ActorBehaviorProfile>> {
        let mut actor_records: HashMap<String, Vec<&AuditRecord>> = HashMap::new();

        // Group records by actor
        for record in records {
            let actor_id = Self::get_actor_id(&record.actor);
            actor_records.entry(actor_id).or_default().push(record);
        }

        let mut profiles = Vec::new();

        for (actor_id, actor_recs) in actor_records {
            let profile = self.create_actor_profile(&actor_id, actor_recs)?;
            profiles.push(profile);
        }

        // Sort by total decisions descending
        profiles.sort_by(|a, b| b.total_decisions.cmp(&a.total_decisions));

        Ok(profiles)
    }

    /// Detects temporal patterns.
    pub fn detect_temporal_patterns_detailed(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<TemporalPattern>> {
        let mut hourly_patterns: HashMap<(u32, u32), usize> = HashMap::new(); // (day_of_week, hour) -> count

        for record in records {
            let day = record.timestamp.weekday().num_days_from_monday();
            let hour = record.timestamp.time().hour();
            *hourly_patterns.entry((day, hour)).or_insert(0) += 1;
        }

        let total = records.len();
        let mut patterns = Vec::new();

        for ((day, hour), count) in hourly_patterns {
            if count >= self.config.min_pattern_occurrences {
                let day_name = match day {
                    0 => "Monday",
                    1 => "Tuesday",
                    2 => "Wednesday",
                    3 => "Thursday",
                    4 => "Friday",
                    5 => "Saturday",
                    6 => "Sunday",
                    _ => "Unknown",
                };

                let pattern = TemporalPattern {
                    name: format!("{} {}:00", day_name, hour),
                    time_slot: format!("{} {:02}:00-{:02}:00", day_name, hour, hour + 1),
                    decision_count: count,
                    percentage: if total > 0 {
                        (count as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    },
                    characteristics: HashMap::new(),
                };

                patterns.push(pattern);
            }
        }

        // Sort by decision count descending
        patterns.sort_by(|a, b| b.decision_count.cmp(&a.decision_count));

        Ok(patterns)
    }

    /// Detects decision flow patterns.
    pub fn detect_decision_flows(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<DecisionFlowPattern>> {
        // Group records by subject to track decision flows
        let mut subject_flows: HashMap<Uuid, Vec<&AuditRecord>> = HashMap::new();

        for record in records {
            subject_flows
                .entry(record.subject_id)
                .or_default()
                .push(record);
        }

        let mut flow_sequences: HashMap<Vec<String>, Vec<f64>> = HashMap::new(); // sequence -> durations

        for (_, mut subject_recs) in subject_flows {
            // Sort by timestamp
            subject_recs.sort_by_key(|r| r.timestamp);

            if subject_recs.len() < 2 {
                continue;
            }

            // Extract sequence of statutes
            let sequence: Vec<String> = subject_recs.iter().map(|r| r.statute_id.clone()).collect();

            // Calculate duration
            let start = subject_recs.first().unwrap().timestamp;
            let end = subject_recs.last().unwrap().timestamp;
            let duration = (end - start).num_seconds() as f64;

            flow_sequences.entry(sequence).or_default().push(duration);
        }

        let mut patterns = Vec::new();

        for (sequence, durations) in flow_sequences {
            if durations.len() >= self.config.min_pattern_occurrences {
                let avg_duration = durations.iter().sum::<f64>() / durations.len() as f64;

                let pattern = DecisionFlowPattern {
                    flow_id: Uuid::new_v4().to_string(),
                    statute_sequence: sequence,
                    occurrence_count: durations.len(),
                    avg_duration_seconds: avg_duration,
                    success_rate: 1.0, // Simplified
                };

                patterns.push(pattern);
            }
        }

        // Sort by occurrence count descending
        patterns.sort_by(|a, b| b.occurrence_count.cmp(&a.occurrence_count));

        Ok(patterns)
    }

    /// Detects actor-specific behavioral patterns.
    fn detect_actor_patterns(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<BehavioralPattern>> {
        let mut patterns = Vec::new();
        let mut actor_records: HashMap<String, Vec<&AuditRecord>> = HashMap::new();

        // Group by actor
        for record in records {
            let actor_id = Self::get_actor_id(&record.actor);
            actor_records.entry(actor_id).or_default().push(record);
        }

        for (actor_id, actor_recs) in actor_records {
            // Check for override tendency
            let override_count = actor_recs
                .iter()
                .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
                .count();

            let override_rate = override_count as f64 / actor_recs.len() as f64;

            if override_rate > 0.3 && override_count >= self.config.min_pattern_occurrences {
                let time_range = (
                    actor_recs.iter().map(|r| r.timestamp).min().unwrap(),
                    actor_recs.iter().map(|r| r.timestamp).max().unwrap(),
                );

                let mut characteristics = HashMap::new();
                characteristics.insert(
                    "override_rate".to_string(),
                    format!("{:.2}%", override_rate * 100.0),
                );
                characteristics.insert("total_overrides".to_string(), override_count.to_string());

                patterns.push(BehavioralPattern {
                    pattern_id: Uuid::new_v4().to_string(),
                    pattern_type: PatternType::OverrideTendency,
                    description: format!("Actor {} shows high override tendency", actor_id),
                    occurrence_count: override_count,
                    confidence: override_rate.min(1.0),
                    actors: vec![actor_id.clone()],
                    statutes: Vec::new(),
                    time_range,
                    characteristics,
                });
            }
        }

        Ok(patterns)
    }

    /// Detects temporal behavioral patterns.
    fn detect_temporal_patterns(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<BehavioralPattern>> {
        let mut patterns = Vec::new();

        // Detect night-time activity patterns
        let night_records: Vec<_> = records
            .iter()
            .filter(|r| {
                let hour = r.timestamp.time().hour();
                !(6..22).contains(&hour)
            })
            .collect();

        if night_records.len() >= self.config.min_pattern_occurrences {
            let percentage = (night_records.len() as f64 / records.len() as f64) * 100.0;

            if percentage > 10.0 {
                let time_range = (
                    night_records.iter().map(|r| r.timestamp).min().unwrap(),
                    night_records.iter().map(|r| r.timestamp).max().unwrap(),
                );

                let mut characteristics = HashMap::new();
                characteristics.insert("percentage".to_string(), format!("{:.2}%", percentage));
                characteristics.insert("count".to_string(), night_records.len().to_string());

                patterns.push(BehavioralPattern {
                    pattern_id: Uuid::new_v4().to_string(),
                    pattern_type: PatternType::TimeBased,
                    description: "Significant night-time activity detected".to_string(),
                    occurrence_count: night_records.len(),
                    confidence: (percentage / 100.0).min(1.0),
                    actors: Vec::new(),
                    statutes: Vec::new(),
                    time_range,
                    characteristics,
                });
            }
        }

        Ok(patterns)
    }

    /// Detects decision flow patterns.
    fn detect_flow_patterns(&self, records: &[AuditRecord]) -> AuditResult<Vec<BehavioralPattern>> {
        let mut patterns = Vec::new();

        // Detect rapid decision sequences
        let mut sorted_records: Vec<_> = records.iter().collect();
        sorted_records.sort_by_key(|r| r.timestamp);

        let mut rapid_sequences = 0;
        for window in sorted_records.windows(2) {
            let time_diff = (window[1].timestamp - window[0].timestamp).num_seconds();
            if time_diff < 5 {
                // Less than 5 seconds
                rapid_sequences += 1;
            }
        }

        if rapid_sequences >= self.config.min_pattern_occurrences {
            let mut characteristics = HashMap::new();
            characteristics.insert("rapid_sequences".to_string(), rapid_sequences.to_string());

            patterns.push(BehavioralPattern {
                pattern_id: Uuid::new_v4().to_string(),
                pattern_type: PatternType::Sequential,
                description: "Rapid decision sequence pattern detected".to_string(),
                occurrence_count: rapid_sequences,
                confidence: 0.8,
                actors: Vec::new(),
                statutes: Vec::new(),
                time_range: (
                    sorted_records.first().unwrap().timestamp,
                    sorted_records.last().unwrap().timestamp,
                ),
                characteristics,
            });
        }

        Ok(patterns)
    }

    /// Creates an actor behavior profile.
    fn create_actor_profile(
        &self,
        actor_id: &str,
        records: Vec<&AuditRecord>,
    ) -> AuditResult<ActorBehaviorProfile> {
        let total_decisions = records.len();

        // Calculate override rate
        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();
        let override_rate = override_count as f64 / total_decisions as f64;

        // Calculate discretionary rate
        let discretionary_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();
        let discretionary_rate = discretionary_count as f64 / total_decisions as f64;

        // Find peak hours
        let mut hour_counts: HashMap<u32, usize> = HashMap::new();
        for record in &records {
            *hour_counts
                .entry(record.timestamp.time().hour())
                .or_insert(0) += 1;
        }
        let mut peak_hours: Vec<_> = hour_counts.into_iter().collect();
        peak_hours.sort_by(|a, b| b.1.cmp(&a.1));
        let peak_hours: Vec<u32> = peak_hours.into_iter().take(3).map(|(h, _)| h).collect();

        // Find peak days
        let mut day_counts: HashMap<u32, usize> = HashMap::new();
        for record in &records {
            *day_counts
                .entry(record.timestamp.weekday().num_days_from_monday())
                .or_insert(0) += 1;
        }
        let mut peak_days_vec: Vec<_> = day_counts.into_iter().collect();
        peak_days_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let peak_days: Vec<String> = peak_days_vec
            .into_iter()
            .take(3)
            .map(|(d, _)| {
                match d {
                    0 => "Monday",
                    1 => "Tuesday",
                    2 => "Wednesday",
                    3 => "Thursday",
                    4 => "Friday",
                    5 => "Saturday",
                    6 => "Sunday",
                    _ => "Unknown",
                }
                .to_string()
            })
            .collect();

        // Find frequent statutes
        let mut statute_counts: HashMap<String, usize> = HashMap::new();
        for record in &records {
            *statute_counts.entry(record.statute_id.clone()).or_insert(0) += 1;
        }
        let mut frequent_statutes: Vec<_> = statute_counts.into_iter().collect();
        frequent_statutes.sort_by(|a, b| b.1.cmp(&a.1));
        frequent_statutes.truncate(5);

        // Calculate average decision time (simplified)
        let avg_decision_time_seconds = 1.0; // Placeholder

        Ok(ActorBehaviorProfile {
            actor_id: actor_id.to_string(),
            total_decisions,
            override_rate,
            discretionary_rate,
            peak_hours,
            peak_days,
            frequent_statutes,
            avg_decision_time_seconds,
            patterns: Vec::new(),
        })
    }

    /// Extracts actor ID from Actor enum.
    fn get_actor_id(actor: &Actor) -> String {
        match actor {
            Actor::System { component } => format!("system:{}", component),
            Actor::User { user_id, .. } => format!("user:{}", user_id),
            Actor::External { system_id } => format!("external:{}", system_id),
        }
    }

    /// Returns the configuration.
    pub fn config(&self) -> &BehavioralConfig {
        &self.config
    }
}

impl Default for BehavioralRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, EventType};
    use chrono::Duration;
    use std::collections::HashMap;

    fn create_test_record(hours_ago: i64, actor_component: &str, is_override: bool) -> AuditRecord {
        let timestamp = Utc::now() - Duration::hours(hours_ago);

        let result = if is_override {
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "denied".to_string(),
                    parameters: HashMap::new(),
                }),
                justification: "test override".to_string(),
            }
        } else {
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            }
        };

        AuditRecord {
            id: Uuid::new_v4(),
            timestamp,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: actor_component.to_string(),
            },
            statute_id: "test-statute".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result,
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_behavioral_recognizer_creation() {
        let recognizer = BehavioralRecognizer::new();
        assert_eq!(recognizer.config().min_pattern_occurrences, 5);
    }

    #[test]
    fn test_analyze_patterns() {
        // Use custom config with lower threshold so test data triggers pattern detection
        let mut config = BehavioralConfig::default();
        config.min_pattern_occurrences = 2;
        let recognizer = BehavioralRecognizer::with_config(config);

        // Create more records with override pattern
        let records: Vec<_> = (0..20)
            .map(|i| create_test_record(i, "component-1", i % 3 == 0))
            .collect();

        let patterns = recognizer.analyze(&records).unwrap();
        // Should detect some patterns with lower threshold
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_profile_actors() {
        let recognizer = BehavioralRecognizer::new();

        let records: Vec<_> = (0..20)
            .map(|i| {
                let component = if i % 2 == 0 {
                    "component-1"
                } else {
                    "component-2"
                };
                create_test_record(i, component, false)
            })
            .collect();

        let profiles = recognizer.profile_actors(&records).unwrap();
        assert_eq!(profiles.len(), 2);
        assert_eq!(
            profiles[0].total_decisions + profiles[1].total_decisions,
            20
        );
    }

    #[test]
    fn test_detect_temporal_patterns() {
        // Use custom config with lower threshold
        let mut config = BehavioralConfig::default();
        config.min_pattern_occurrences = 2;
        let recognizer = BehavioralRecognizer::with_config(config);

        // Create records that share the same (day_of_week, hour) slot
        // Use hours 0, 0, 24, 24 (same day and hour slot)
        let mut records = Vec::new();
        for _ in 0..3 {
            records.push(create_test_record(0, "component-1", false)); // Same hour slot
        }
        for _ in 0..3 {
            records.push(create_test_record(168, "component-1", false)); // 1 week ago, same day/hour
        }

        let patterns = recognizer
            .detect_temporal_patterns_detailed(&records)
            .unwrap();
        // Should detect temporal patterns when records share time slots
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_detect_decision_flows() {
        // Use custom config with lower threshold
        let mut config = BehavioralConfig::default();
        config.min_pattern_occurrences = 2;
        let recognizer = BehavioralRecognizer::with_config(config);

        // Create multiple subjects with the same flow pattern to meet threshold
        let mut records = Vec::new();
        for _ in 0..5 {
            let subject_id = Uuid::new_v4();
            for i in 0..3 {
                let mut record = create_test_record(i, "component-1", false);
                record.subject_id = subject_id;
                records.push(record);
            }
        }

        let flows = recognizer.detect_decision_flows(&records).unwrap();
        // Should detect decision flows with lower threshold
        assert!(!flows.is_empty());
    }

    #[test]
    fn test_get_actor_id() {
        let system = Actor::System {
            component: "test".to_string(),
        };
        assert_eq!(BehavioralRecognizer::get_actor_id(&system), "system:test");

        let user = Actor::User {
            user_id: "user123".to_string(),
            role: "admin".to_string(),
        };
        assert_eq!(BehavioralRecognizer::get_actor_id(&user), "user:user123");

        let external = Actor::External {
            system_id: "ext123".to_string(),
        };
        assert_eq!(
            BehavioralRecognizer::get_actor_id(&external),
            "external:ext123"
        );
    }
}
