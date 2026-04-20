//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{CircuitBreakerConfig, FieldProfile};
use super::types_5::RetentionRule;
use super::types_6::StatuteStatus;

/// Difference between two registries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryDifference {
    /// Timestamp of comparison
    pub compared_at: DateTime<Utc>,
    /// Statutes only in left registry
    pub only_in_left: Vec<String>,
    /// Statutes only in right registry
    pub only_in_right: Vec<String>,
    /// Statutes in both but with differences
    pub different_statutes: Vec<StatuteDifferenceDetail>,
    /// Statutes that are identical
    pub identical_statutes: Vec<String>,
}
impl RegistryDifference {
    /// Creates a new empty registry difference.
    pub fn new() -> Self {
        Self {
            compared_at: Utc::now(),
            only_in_left: Vec::new(),
            only_in_right: Vec::new(),
            different_statutes: Vec::new(),
            identical_statutes: Vec::new(),
        }
    }
    /// Returns the total number of differences found.
    pub fn difference_count(&self) -> usize {
        self.only_in_left.len() + self.only_in_right.len() + self.different_statutes.len()
    }
    /// Checks if the registries are identical.
    pub fn is_identical(&self) -> bool {
        self.difference_count() == 0
    }
    /// Returns a summary of the comparison.
    pub fn summary(&self) -> String {
        format!(
            "Only in left: {}, Only in right: {}, Different: {}, Identical: {}",
            self.only_in_left.len(),
            self.only_in_right.len(),
            self.different_statutes.len(),
            self.identical_statutes.len()
        )
    }
}
/// Details of differences in a specific statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDifferenceDetail {
    /// Statute ID
    pub statute_id: String,
    /// Fields that differ
    pub differing_fields: Vec<String>,
    /// Version in left registry
    pub left_version: u32,
    /// Version in right registry
    pub right_version: u32,
}
/// Configuration for retention policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetentionPolicy {
    /// Rules to apply for archiving
    pub(super) rules: Vec<RetentionRule>,
    /// Whether to automatically apply retention on operations
    pub(super) auto_apply: bool,
}
impl RetentionPolicy {
    /// Creates a new empty retention policy.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enables automatic application of retention rules.
    pub fn with_auto_apply(mut self) -> Self {
        self.auto_apply = true;
        self
    }
    /// Adds a retention rule.
    pub fn add_rule(mut self, rule: RetentionRule) -> Self {
        self.rules.push(rule);
        self
    }
    /// Returns all rules.
    pub fn rules(&self) -> &[RetentionRule] {
        &self.rules
    }
    /// Checks if auto-apply is enabled.
    pub fn is_auto_apply(&self) -> bool {
        self.auto_apply
    }
}
/// Tag analytics for analyzing tag usage patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagAnalytics {
    /// Tag frequency (tag -> count)
    pub tag_frequency: HashMap<String, usize>,
    /// Tag co-occurrence (tag1 -> tag2 -> count)
    pub tag_cooccurrence: HashMap<String, HashMap<String, usize>>,
    /// Most used tags (tag, count)
    pub most_used_tags: Vec<(String, usize)>,
    /// Least used tags (tag, count)
    pub least_used_tags: Vec<(String, usize)>,
    /// Average tags per statute
    pub avg_tags_per_statute: f64,
}
impl TagAnalytics {
    /// Creates a new tag analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns the total number of unique tags.
    pub fn unique_tag_count(&self) -> usize {
        self.tag_frequency.len()
    }
    /// Returns the total tag usage across all statutes.
    pub fn total_tag_usage(&self) -> usize {
        self.tag_frequency.values().sum()
    }
    /// Gets tags that commonly appear together with the given tag.
    pub fn related_tags(&self, tag: &str, min_occurrences: usize) -> Vec<(String, usize)> {
        self.tag_cooccurrence
            .get(tag)
            .map(|cooccur| {
                let mut pairs: Vec<_> = cooccur
                    .iter()
                    .filter(|&(_, count)| *count >= min_occurrences)
                    .map(|(t, c)| (t.clone(), *c))
                    .collect();
                pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
                pairs
            })
            .unwrap_or_default()
    }
}
/// Configuration for bulk operations.
#[derive(Debug, Clone)]
pub struct BulkConfig {
    /// Batch size for processing
    pub batch_size: usize,
    /// Whether to continue on error
    pub continue_on_error: bool,
    /// Maximum parallel operations
    pub max_parallelism: usize,
}
impl BulkConfig {
    /// Creates a new bulk config.
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            ..Default::default()
        }
    }
    /// Builder: Sets continue on error.
    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }
    /// Builder: Sets max parallelism.
    pub fn with_max_parallelism(mut self, max_parallelism: usize) -> Self {
        self.max_parallelism = max_parallelism;
        self
    }
}
/// Circuit breaker state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are rejected
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}
/// Events that can occur in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryEvent {
    /// A new statute was registered
    StatuteRegistered {
        registry_id: Uuid,
        statute_id: String,
        jurisdiction: String,
        timestamp: DateTime<Utc>,
    },
    /// A statute was updated
    StatuteUpdated {
        statute_id: String,
        old_version: u32,
        new_version: u32,
        timestamp: DateTime<Utc>,
    },
    /// A statute's status was changed
    StatusChanged {
        statute_id: String,
        old_status: StatuteStatus,
        new_status: StatuteStatus,
        timestamp: DateTime<Utc>,
    },
    /// A tag was added to a statute
    TagAdded {
        statute_id: String,
        tag: String,
        timestamp: DateTime<Utc>,
    },
    /// A tag was removed from a statute
    TagRemoved {
        statute_id: String,
        tag: String,
        timestamp: DateTime<Utc>,
    },
    /// A reference was added
    ReferenceAdded {
        statute_id: String,
        referenced_id: String,
        timestamp: DateTime<Utc>,
    },
    /// A reference was removed
    ReferenceRemoved {
        statute_id: String,
        referenced_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Metadata was updated
    MetadataUpdated {
        statute_id: String,
        key: String,
        old_value: Option<String>,
        new_value: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// A statute was deleted
    StatuteDeleted {
        statute_id: String,
        jurisdiction: String,
        version: u32,
        timestamp: DateTime<Utc>,
    },
    /// A statute was archived
    StatuteArchived {
        statute_id: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}
/// Metadata for a registry backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Timestamp when the backup was created
    pub created_at: DateTime<Utc>,
    /// Version of the backup format
    pub format_version: String,
    /// Total number of statutes
    pub statute_count: usize,
    /// Total number of events
    pub event_count: usize,
    /// Description or notes
    pub description: Option<String>,
}
/// Log level for observability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}
/// Circuit breaker for fault tolerance.
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    pub(super) state: CircuitState,
    pub(super) failure_count: usize,
    pub(super) success_count: usize,
    pub(super) last_failure_time: Option<DateTime<Utc>>,
}
impl CircuitBreaker {
    /// Creates a new circuit breaker.
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }
    /// Records a successful operation.
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                    self.last_failure_time = None;
                }
            }
            CircuitState::Open => {
                self.state = CircuitState::Closed;
                self.failure_count = 0;
                self.success_count = 0;
            }
        }
    }
    /// Records a failed operation.
    pub fn record_failure(&mut self) {
        self.last_failure_time = Some(Utc::now());
        match self.state {
            CircuitState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.success_count = 0;
            }
            CircuitState::Open => {}
        }
    }
    /// Checks if a request is allowed.
    pub fn is_request_allowed(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    let now = Utc::now();
                    let timeout = chrono::Duration::seconds(self.config.timeout_secs);
                    if now - last_failure >= timeout {
                        self.state = CircuitState::HalfOpen;
                        self.success_count = 0;
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }
    /// Returns the current state.
    pub fn state(&self) -> &CircuitState {
        &self.state
    }
    /// Returns the failure count.
    pub fn failure_count(&self) -> usize {
        self.failure_count
    }
    /// Resets the circuit breaker.
    pub fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.last_failure_time = None;
    }
    /// Forces the circuit to open.
    pub fn force_open(&mut self) {
        self.state = CircuitState::Open;
        self.last_failure_time = Some(Utc::now());
    }
}
/// Comprehensive data profile for the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProfile {
    /// Total statutes profiled
    pub total_statutes: usize,
    /// Field profiles
    pub field_profiles: HashMap<String, FieldProfile>,
    /// Average quality score
    pub average_quality: f64,
    /// Quality distribution (grade -> count)
    pub quality_distribution: HashMap<char, usize>,
    /// Status distribution
    pub status_distribution: HashMap<StatuteStatus, usize>,
    /// Jurisdiction distribution
    pub jurisdiction_distribution: HashMap<String, usize>,
    /// Tag usage patterns
    pub tag_patterns: HashMap<String, usize>,
    /// Profiling timestamp
    pub profiled_at: DateTime<Utc>,
}
impl DataProfile {
    /// Creates a new data profile.
    pub fn new(total_statutes: usize) -> Self {
        Self {
            total_statutes,
            field_profiles: HashMap::new(),
            average_quality: 0.0,
            quality_distribution: HashMap::new(),
            status_distribution: HashMap::new(),
            jurisdiction_distribution: HashMap::new(),
            tag_patterns: HashMap::new(),
            profiled_at: Utc::now(),
        }
    }
    /// Adds a field profile.
    pub fn add_field_profile(&mut self, profile: FieldProfile) {
        self.field_profiles
            .insert(profile.field_name.clone(), profile);
    }
    /// Gets the completeness of a field.
    pub fn field_completeness(&self, field_name: &str) -> Option<f64> {
        self.field_profiles.get(field_name).map(|p| p.completeness)
    }
    /// Exports the profile to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
