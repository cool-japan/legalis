//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

use super::functions_2::{ValidationResult, ValidationRule};
use super::types_3::{LogLevel, RegistryEvent, TagAnalytics};
use super::types_5::WebhookEventFilter;
use super::types_6::{
    BatchValidationResult, PiiFieldType, RelationshipAnalytics, StatuteEntry, StatuteStatus,
    WebhookSubscription,
};
use super::types_7::{NonEmptyTitleRule, Permission, SimilarityScore, TemporalAnalytics};
use super::types_8::{ArchivedStatute, LogEntry, MetricEntry, NonEmptyIdRule, ValidationError};

/// Data profile for a field in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldProfile {
    /// Field name
    pub field_name: String,
    /// Total values
    pub total_values: usize,
    /// Null/empty values count
    pub null_count: usize,
    /// Unique values count
    pub unique_count: usize,
    /// Most common values (top 10)
    pub most_common: Vec<(String, usize)>,
    /// Completeness percentage
    pub completeness: f64,
}
impl FieldProfile {
    /// Creates a new field profile.
    pub fn new(field_name: String, total_values: usize) -> Self {
        Self {
            field_name,
            total_values,
            null_count: 0,
            unique_count: 0,
            most_common: Vec::new(),
            completeness: 0.0,
        }
    }
    /// Calculates completeness percentage.
    pub fn calculate_completeness(&mut self) {
        if self.total_values > 0 {
            self.completeness =
                ((self.total_values - self.null_count) as f64 / self.total_values as f64) * 100.0;
        }
    }
}
/// A collection of validation rules.
#[derive(Default)]
pub struct Validator {
    pub(super) rules: Vec<Box<dyn ValidationRule>>,
}
impl Validator {
    /// Creates a new empty validator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a validator with default rules.
    pub fn with_defaults() -> Self {
        let mut validator = Self::new();
        validator.add_rule(Box::new(NonEmptyIdRule));
        validator.add_rule(Box::new(NonEmptyTitleRule));
        validator.add_rule(Box::new(DateValidationRule));
        validator.add_rule(Box::new(TagValidationRule));
        validator
    }
    /// Adds a validation rule.
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) -> &mut Self {
        self.rules.push(rule);
        self
    }
    /// Validates a statute entry against all rules.
    pub fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        for rule in &self.rules {
            rule.validate(entry)?;
        }
        Ok(())
    }
    /// Returns all validation rules.
    pub fn rules(&self) -> &[Box<dyn ValidationRule>] {
        &self.rules
    }
}
impl Validator {
    /// Validates multiple statute entries.
    pub fn validate_batch(&self, entries: &[StatuteEntry]) -> BatchValidationResult {
        let mut errors = HashMap::new();
        let mut valid = 0;
        let mut invalid = 0;
        for entry in entries {
            match self.validate(entry) {
                Ok(()) => valid += 1,
                Err(e) => {
                    invalid += 1;
                    errors.insert(entry.statute.id.clone(), e);
                }
            }
        }
        BatchValidationResult {
            total: entries.len(),
            valid,
            invalid,
            errors,
        }
    }
    /// Validates multiple entries and returns only the valid ones.
    pub fn filter_valid(&self, entries: Vec<StatuteEntry>) -> Vec<StatuteEntry> {
        entries
            .into_iter()
            .filter(|e| self.validate(e).is_ok())
            .collect()
    }
    /// Validates multiple entries and returns only the invalid ones with their errors.
    pub fn filter_invalid(
        &self,
        entries: Vec<StatuteEntry>,
    ) -> Vec<(StatuteEntry, ValidationError)> {
        entries
            .into_iter()
            .filter_map(|e| self.validate(&e).err().map(|err| (e, err)))
            .collect()
    }
}
/// Temporary access grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryAccess {
    /// Grant ID
    pub grant_id: Uuid,
    /// User ID this grant is for
    pub user_id: String,
    /// Statute ID (None for global access)
    pub statute_id: Option<String>,
    /// Permissions granted
    pub permissions: Vec<Permission>,
    /// Grant valid from
    pub valid_from: DateTime<Utc>,
    /// Grant valid until
    pub valid_until: DateTime<Utc>,
    /// Reason for grant
    pub reason: String,
    /// Granted by (user ID)
    pub granted_by: String,
}
impl TemporaryAccess {
    /// Creates a new temporary access grant.
    pub fn new(
        user_id: impl Into<String>,
        permissions: Vec<Permission>,
        duration_hours: i64,
        reason: impl Into<String>,
        granted_by: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            grant_id: Uuid::new_v4(),
            user_id: user_id.into(),
            statute_id: None,
            permissions,
            valid_from: now,
            valid_until: now + chrono::Duration::hours(duration_hours),
            reason: reason.into(),
            granted_by: granted_by.into(),
        }
    }
    /// Sets the statute ID for statute-specific access.
    pub fn for_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.statute_id = Some(statute_id.into());
        self
    }
    /// Checks if the grant is currently valid.
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.valid_from && now <= self.valid_until
    }
    /// Checks if the grant applies to a specific statute.
    pub fn applies_to(&self, statute_id: &str) -> bool {
        self.statute_id
            .as_ref()
            .map(|s| s == statute_id)
            .unwrap_or(true)
    }
    /// Returns remaining time in seconds.
    pub fn remaining_seconds(&self) -> i64 {
        let now = Utc::now();
        if now > self.valid_until {
            0
        } else {
            (self.valid_until - now).num_seconds()
        }
    }
}
/// Validates that tags are not empty and unique.
#[derive(Debug, Clone)]
pub struct TagValidationRule;
/// Health status of the registry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some degradation but functional
    Degraded { issues: Vec<String> },
    /// Critical issues affecting functionality
    Unhealthy { errors: Vec<String> },
}
impl HealthStatus {
    /// Checks if the status is healthy.
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
    /// Checks if the status is degraded.
    pub fn is_degraded(&self) -> bool {
        matches!(self, HealthStatus::Degraded { .. })
    }
    /// Checks if the status is unhealthy.
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy { .. })
    }
}
/// A detected PII instance in statute content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiDetection {
    /// Field type
    pub field_type: PiiFieldType,
    /// Original value (potentially sensitive)
    pub value: String,
    /// Position in text (char offset)
    pub position: usize,
    /// Length of the PII value
    pub length: usize,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}
impl PiiDetection {
    /// Creates a new PII detection.
    pub fn new(field_type: PiiFieldType, value: String, position: usize, confidence: f64) -> Self {
        let length = value.len();
        let confidence = confidence.clamp(0.0, 1.0);
        Self {
            field_type,
            value,
            position,
            length,
            confidence,
        }
    }
    /// Returns true if confidence is above threshold.
    pub fn is_confident(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
}
/// Type of data enrichment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnrichmentType {
    /// Auto-tagging based on content
    AutoTag,
    /// Metadata inference
    MetadataInference,
    /// Jurisdiction inference
    JurisdictionInference,
    /// Related statute suggestion
    RelatedStatute,
    /// Category classification
    CategoryClassification,
}
/// Types of auditable operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditOperation {
    /// Register new statute
    Register,
    /// Update existing statute
    Update,
    /// Delete statute
    Delete,
    /// Archive statute
    Archive,
    /// Unarchive statute
    Unarchive,
    /// Change status
    StatusChange {
        from: StatuteStatus,
        to: StatuteStatus,
    },
    /// Add tag
    AddTag { tag: String },
    /// Remove tag
    RemoveTag { tag: String },
    /// Add metadata
    AddMetadata { key: String },
    /// Remove metadata
    RemoveMetadata { key: String },
    /// Export data
    Export { format: String },
    /// Import data
    Import { format: String },
    /// Search operation
    Search { query: String },
    /// Batch operation
    BatchOperation {
        operation_type: String,
        count: usize,
    },
    /// Apply retention policy
    RetentionPolicy,
    /// Create snapshot
    CreateSnapshot,
    /// Restore from snapshot
    RestoreSnapshot { snapshot_id: Uuid },
}
/// Observability collector for logs and metrics.
#[derive(Debug, Clone)]
pub struct ObservabilityCollector {
    logs: VecDeque<LogEntry>,
    pub(super) metrics: VecDeque<MetricEntry>,
    max_logs: usize,
    max_metrics: usize,
    min_log_level: LogLevel,
}
impl ObservabilityCollector {
    /// Creates a new observability collector.
    pub fn new(max_logs: usize, max_metrics: usize, min_log_level: LogLevel) -> Self {
        Self {
            logs: VecDeque::new(),
            metrics: VecDeque::new(),
            max_logs,
            max_metrics,
            min_log_level,
        }
    }
    /// Records a log entry.
    pub fn log(&mut self, entry: LogEntry) {
        if entry.level < self.min_log_level {
            return;
        }
        self.logs.push_back(entry);
        if self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }
    }
    /// Records a metric entry.
    pub fn metric(&mut self, entry: MetricEntry) {
        self.metrics.push_back(entry);
        if self.metrics.len() > self.max_metrics {
            self.metrics.pop_front();
        }
    }
    /// Returns all logs.
    pub fn logs(&self) -> &VecDeque<LogEntry> {
        &self.logs
    }
    /// Returns all metrics.
    pub fn metrics(&self) -> &VecDeque<MetricEntry> {
        &self.metrics
    }
    /// Returns logs filtered by level.
    pub fn logs_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.iter().filter(|e| e.level == level).collect()
    }
    /// Returns logs filtered by operation.
    pub fn logs_by_operation(&self, operation: &str) -> Vec<&LogEntry> {
        self.logs
            .iter()
            .filter(|e| e.operation == operation)
            .collect()
    }
    /// Returns metrics by name.
    pub fn metrics_by_name(&self, name: &str) -> Vec<&MetricEntry> {
        self.metrics.iter().filter(|m| m.name == name).collect()
    }
    /// Clears all logs.
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }
    /// Clears all metrics.
    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }
    /// Exports logs to JSON.
    pub fn export_logs_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.logs)
    }
    /// Exports metrics to JSON.
    pub fn export_metrics_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.metrics)
    }
}
/// Archive for storing removed or superseded statutes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatuteArchive {
    /// Archived statutes by ID
    pub(super) archived: HashMap<String, ArchivedStatute>,
}
impl StatuteArchive {
    /// Creates a new empty archive.
    pub fn new() -> Self {
        Self::default()
    }
    /// Archives a statute.
    pub fn archive(&mut self, entry: StatuteEntry, reason: String) {
        let statute_id = entry.statute.id.clone();
        self.archived.insert(
            statute_id,
            ArchivedStatute {
                entry,
                reason,
                archived_at: Utc::now(),
            },
        );
    }
    /// Retrieves an archived statute.
    pub fn get(&self, statute_id: &str) -> Option<&ArchivedStatute> {
        self.archived.get(statute_id)
    }
    /// Removes a statute from the archive (unarchive).
    pub fn unarchive(&mut self, statute_id: &str) -> Option<ArchivedStatute> {
        self.archived.remove(statute_id)
    }
    /// Lists all archived statute IDs.
    pub fn list_ids(&self) -> Vec<String> {
        self.archived.keys().cloned().collect()
    }
    /// Lists all archived statutes.
    pub fn list_all(&self) -> Vec<&ArchivedStatute> {
        self.archived.values().collect()
    }
    /// Returns the count of archived statutes.
    pub fn count(&self) -> usize {
        self.archived.len()
    }
    /// Searches archived statutes by reason (case-insensitive substring match).
    pub fn search_by_reason(&self, query: &str) -> Vec<&ArchivedStatute> {
        let query_lower = query.to_lowercase();
        self.archived
            .values()
            .filter(|a| a.reason.to_lowercase().contains(&query_lower))
            .collect()
    }
    /// Clears all archived statutes.
    pub fn clear(&mut self) {
        self.archived.clear();
    }
}
/// Errors during registry operations.
#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Statute not found: {0}")]
    StatuteNotFound(String),
    #[error("Version not found: {statute_id} v{version}")]
    VersionNotFound { statute_id: String, version: u32 },
    #[error("Duplicate statute ID: {0}")]
    DuplicateId(String),
    #[error("Circular reference detected: {0}")]
    CircularReference(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Concurrent modification: expected ETag {expected}, got {actual}")]
    ConcurrentModification { expected: String, actual: String },
}
/// A potential duplicate statute pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCandidate {
    /// First statute ID
    pub statute_id_1: String,
    /// Second statute ID
    pub statute_id_2: String,
    /// Similarity score
    pub similarity: SimilarityScore,
    /// Reason for flagging as duplicate
    pub reason: String,
}
impl DuplicateCandidate {
    /// Creates a new duplicate candidate.
    pub fn new(
        statute_id_1: String,
        statute_id_2: String,
        similarity: SimilarityScore,
        reason: String,
    ) -> Self {
        Self {
            statute_id_1,
            statute_id_2,
            similarity,
            reason,
        }
    }
}
/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Time to wait before attempting recovery (seconds)
    pub timeout_secs: i64,
    /// Number of successful requests needed to close circuit
    pub success_threshold: usize,
}
impl CircuitBreakerConfig {
    /// Creates a new circuit breaker config.
    pub fn new(failure_threshold: usize, timeout_secs: i64, success_threshold: usize) -> Self {
        Self {
            failure_threshold,
            timeout_secs,
            success_threshold,
        }
    }
}
/// Validates that effective and expiry dates are logical.
#[derive(Debug, Clone)]
pub struct DateValidationRule;
/// Field projection options for efficient queries.
#[derive(Debug, Clone, Default)]
pub struct FieldProjection {
    /// Include statute ID
    pub include_id: bool,
    /// Include title
    pub include_title: bool,
    /// Include version
    pub include_version: bool,
    /// Include status
    pub include_status: bool,
    /// Include jurisdiction
    pub include_jurisdiction: bool,
    /// Include tags
    pub include_tags: bool,
    /// Include dates
    pub include_dates: bool,
    /// Include metadata
    pub include_metadata: bool,
}
impl FieldProjection {
    /// Creates a projection that includes all fields.
    pub fn all() -> Self {
        Self {
            include_id: true,
            include_title: true,
            include_version: true,
            include_status: true,
            include_jurisdiction: true,
            include_tags: true,
            include_dates: true,
            include_metadata: true,
        }
    }
    /// Creates a projection with only essential fields.
    pub fn essential() -> Self {
        Self {
            include_id: true,
            include_title: true,
            include_version: true,
            include_status: true,
            ..Default::default()
        }
    }
    /// Adds ID to the projection.
    pub fn with_id(mut self) -> Self {
        self.include_id = true;
        self
    }
    /// Adds title to the projection.
    pub fn with_title(mut self) -> Self {
        self.include_title = true;
        self
    }
    /// Adds version to the projection.
    pub fn with_version(mut self) -> Self {
        self.include_version = true;
        self
    }
    /// Adds status to the projection.
    pub fn with_status(mut self) -> Self {
        self.include_status = true;
        self
    }
    /// Adds jurisdiction to the projection.
    pub fn with_jurisdiction(mut self) -> Self {
        self.include_jurisdiction = true;
        self
    }
    /// Adds tags to the projection.
    pub fn with_tags(mut self) -> Self {
        self.include_tags = true;
        self
    }
    /// Adds dates to the projection.
    pub fn with_dates(mut self) -> Self {
        self.include_dates = true;
        self
    }
    /// Adds metadata to the projection.
    pub fn with_metadata(mut self) -> Self {
        self.include_metadata = true;
        self
    }
}
/// Webhook manager for event notifications.
#[derive(Debug, Clone)]
pub struct WebhookManager {
    pub(super) subscriptions: Arc<Mutex<Vec<WebhookSubscription>>>,
}
impl WebhookManager {
    /// Creates a new webhook manager.
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(Vec::new())),
        }
    }
    /// Subscribes to events with a callback.
    pub fn subscribe<F>(
        &self,
        name: Option<String>,
        filter: Option<WebhookEventFilter>,
        callback: F,
    ) -> Uuid
    where
        F: Fn(&RegistryEvent) + Send + Sync + 'static,
    {
        let id = Uuid::new_v4();
        let subscription = WebhookSubscription {
            id,
            name,
            callback: Arc::new(callback),
            event_filter: filter,
        };
        let mut subs = self
            .subscriptions
            .lock()
            .expect("subscriptions mutex poisoned");
        subs.push(subscription);
        id
    }
    /// Unsubscribes a webhook by ID.
    pub fn unsubscribe(&self, id: Uuid) -> bool {
        let mut subs = self
            .subscriptions
            .lock()
            .expect("subscriptions mutex poisoned");
        if let Some(pos) = subs.iter().position(|s| s.id == id) {
            subs.remove(pos);
            true
        } else {
            false
        }
    }
    /// Triggers all matching webhooks for an event.
    pub fn trigger(&self, event: &RegistryEvent) {
        let subs = self
            .subscriptions
            .lock()
            .expect("subscriptions mutex poisoned");
        for subscription in subs.iter() {
            if subscription
                .event_filter
                .as_ref()
                .is_none_or(|filter| filter.matches(event))
            {
                (subscription.callback)(event);
            }
        }
    }
    /// Returns the count of active subscriptions.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions
            .lock()
            .expect("subscriptions mutex poisoned")
            .len()
    }
    /// Clears all subscriptions.
    pub fn clear(&self) {
        self.subscriptions
            .lock()
            .expect("subscriptions mutex poisoned")
            .clear();
    }
    /// Lists all subscription IDs and names.
    pub fn list_subscriptions(&self) -> Vec<(Uuid, Option<String>)> {
        self.subscriptions
            .lock()
            .expect("subscriptions mutex poisoned")
            .iter()
            .map(|s| (s.id, s.name.clone()))
            .collect()
    }
}
/// Represents a change in a field between two statute versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldChange<T: Clone> {
    /// Field value changed from old to new
    Changed { old: T, new: T },
    /// Field was added (only in new version)
    Added { value: T },
    /// Field was removed (only in old version)
    Removed { value: T },
    /// Field unchanged
    Unchanged { value: T },
}
impl<T: Clone + PartialEq> FieldChange<T> {
    /// Creates a field change by comparing old and new values.
    pub fn from_optional(old: Option<&T>, new: Option<&T>) -> Option<Self> {
        match (old, new) {
            (Some(o), Some(n)) if o != n => Some(FieldChange::Changed {
                old: o.clone(),
                new: n.clone(),
            }),
            (Some(o), Some(_)) => Some(FieldChange::Unchanged { value: o.clone() }),
            (None, Some(n)) => Some(FieldChange::Added { value: n.clone() }),
            (Some(o), None) => Some(FieldChange::Removed { value: o.clone() }),
            (None, None) => None,
        }
    }
    /// Creates a field change by comparing required values.
    pub fn from_values(old: &T, new: &T) -> Self {
        if old != new {
            FieldChange::Changed {
                old: old.clone(),
                new: new.clone(),
            }
        } else {
            FieldChange::Unchanged { value: old.clone() }
        }
    }
    /// Returns true if this represents a change.
    pub fn is_changed(&self) -> bool {
        matches!(
            self,
            FieldChange::Changed { .. } | FieldChange::Added { .. } | FieldChange::Removed { .. }
        )
    }
    /// Returns the new value if available.
    pub fn new_value(&self) -> Option<&T> {
        match self {
            FieldChange::Changed { new, .. } => Some(new),
            FieldChange::Added { value } => Some(value),
            FieldChange::Unchanged { value } => Some(value),
            FieldChange::Removed { .. } => None,
        }
    }
}
/// Search cache configuration.
#[derive(Debug, Clone, Copy)]
pub struct SearchCacheConfig {
    /// Maximum number of cached queries
    pub max_entries: usize,
    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: i64,
}
impl SearchCacheConfig {
    /// Creates a new cache config.
    pub fn new(max_entries: usize, ttl_seconds: i64) -> Self {
        Self {
            max_entries,
            ttl_seconds,
        }
    }
    /// Creates a cache config with no TTL (cache indefinitely).
    pub fn no_ttl(max_entries: usize) -> Self {
        Self {
            max_entries,
            ttl_seconds: i64::MAX,
        }
    }
    /// Creates a cache config with short TTL (1 minute).
    pub fn short_lived(max_entries: usize) -> Self {
        Self {
            max_entries,
            ttl_seconds: 60,
        }
    }
    /// Creates a cache config with long TTL (1 hour).
    pub fn long_lived(max_entries: usize) -> Self {
        Self {
            max_entries,
            ttl_seconds: 3600,
        }
    }
}
/// Event store for tracking all changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStore {
    /// All events in chronological order
    pub(super) events: VecDeque<RegistryEvent>,
    /// Maximum number of events to keep (0 = unlimited)
    pub(super) max_events: usize,
}
impl EventStore {
    /// Creates a new event store.
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 0,
        }
    }
    /// Creates a new event store with a maximum size.
    pub fn with_max_events(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
        }
    }
    /// Records an event.
    pub fn record(&mut self, event: RegistryEvent) {
        self.events.push_back(event);
        if self.max_events > 0 && self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }
    /// Returns all events.
    pub fn all_events(&self) -> Vec<&RegistryEvent> {
        self.events.iter().collect()
    }
    /// Returns events for a specific statute.
    pub fn events_for_statute(&self, statute_id: &str) -> Vec<&RegistryEvent> {
        self.events
            .iter()
            .filter(|event| match event {
                RegistryEvent::StatuteRegistered { statute_id: id, .. }
                | RegistryEvent::StatuteUpdated { statute_id: id, .. }
                | RegistryEvent::StatusChanged { statute_id: id, .. }
                | RegistryEvent::TagAdded { statute_id: id, .. }
                | RegistryEvent::TagRemoved { statute_id: id, .. }
                | RegistryEvent::ReferenceAdded { statute_id: id, .. }
                | RegistryEvent::ReferenceRemoved { statute_id: id, .. }
                | RegistryEvent::MetadataUpdated { statute_id: id, .. }
                | RegistryEvent::StatuteDeleted { statute_id: id, .. }
                | RegistryEvent::StatuteArchived { statute_id: id, .. } => id == statute_id,
            })
            .collect()
    }
    /// Returns events within a date range.
    pub fn events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&RegistryEvent> {
        self.events
            .iter()
            .filter(|event| {
                let timestamp = match event {
                    RegistryEvent::StatuteRegistered { timestamp, .. }
                    | RegistryEvent::StatuteUpdated { timestamp, .. }
                    | RegistryEvent::StatusChanged { timestamp, .. }
                    | RegistryEvent::TagAdded { timestamp, .. }
                    | RegistryEvent::TagRemoved { timestamp, .. }
                    | RegistryEvent::ReferenceAdded { timestamp, .. }
                    | RegistryEvent::ReferenceRemoved { timestamp, .. }
                    | RegistryEvent::MetadataUpdated { timestamp, .. }
                    | RegistryEvent::StatuteDeleted { timestamp, .. }
                    | RegistryEvent::StatuteArchived { timestamp, .. } => timestamp,
                };
                timestamp >= &start && timestamp <= &end
            })
            .collect()
    }
    /// Returns the count of events.
    pub fn count(&self) -> usize {
        self.events.len()
    }
    /// Clears all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
/// Statistics for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantStats {
    /// Number of statutes
    pub statute_count: usize,
    /// Number of events
    pub event_count: usize,
    /// Number of active statutes
    pub active_statute_count: usize,
    /// Number of unique tags
    pub tag_count: usize,
    /// Number of unique jurisdictions
    pub jurisdiction_count: usize,
}
/// Cached analytics with timestamp for TTL.
#[derive(Debug, Clone)]
pub(crate) struct CachedAnalytics {
    pub(crate) temporal: Option<(TemporalAnalytics, DateTime<Utc>)>,
    pub(crate) relationship: Option<(RelationshipAnalytics, DateTime<Utc>)>,
    pub(crate) tag: Option<(TagAnalytics, DateTime<Utc>)>,
    pub(crate) activity: Option<(ActivityAnalytics, DateTime<Utc>)>,
    pub(crate) cache_duration_secs: i64,
}
impl CachedAnalytics {
    pub(crate) fn new(cache_duration_secs: i64) -> Self {
        Self {
            temporal: None,
            relationship: None,
            tag: None,
            activity: None,
            cache_duration_secs,
        }
    }
    pub(crate) fn is_valid(timestamp: DateTime<Utc>, duration_secs: i64) -> bool {
        let now = Utc::now();
        (now - timestamp).num_seconds() < duration_secs
    }
    pub(crate) fn get_temporal(&self) -> Option<&TemporalAnalytics> {
        self.temporal.as_ref().and_then(|(analytics, timestamp)| {
            if Self::is_valid(*timestamp, self.cache_duration_secs) {
                Some(analytics)
            } else {
                None
            }
        })
    }
    pub(crate) fn set_temporal(&mut self, analytics: TemporalAnalytics) {
        self.temporal = Some((analytics, Utc::now()));
    }
    pub(crate) fn get_relationship(&self) -> Option<&RelationshipAnalytics> {
        self.relationship
            .as_ref()
            .and_then(|(analytics, timestamp)| {
                if Self::is_valid(*timestamp, self.cache_duration_secs) {
                    Some(analytics)
                } else {
                    None
                }
            })
    }
    pub(crate) fn set_relationship(&mut self, analytics: RelationshipAnalytics) {
        self.relationship = Some((analytics, Utc::now()));
    }
    pub(crate) fn get_tag(&self) -> Option<&TagAnalytics> {
        self.tag.as_ref().and_then(|(analytics, timestamp)| {
            if Self::is_valid(*timestamp, self.cache_duration_secs) {
                Some(analytics)
            } else {
                None
            }
        })
    }
    pub(crate) fn set_tag(&mut self, analytics: TagAnalytics) {
        self.tag = Some((analytics, Utc::now()));
    }
    pub(crate) fn get_activity(&self) -> Option<&ActivityAnalytics> {
        self.activity.as_ref().and_then(|(analytics, timestamp)| {
            if Self::is_valid(*timestamp, self.cache_duration_secs) {
                Some(analytics)
            } else {
                None
            }
        })
    }
    pub(crate) fn set_activity(&mut self, analytics: ActivityAnalytics) {
        self.activity = Some((analytics, Utc::now()));
    }
    pub(crate) fn clear(&mut self) {
        self.temporal = None;
        self.relationship = None;
        self.tag = None;
        self.activity = None;
    }
}
/// Activity analytics for tracking modification patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAnalytics {
    /// Most modified statutes (id, modification_count)
    pub most_modified: Vec<(String, usize)>,
    /// Recently modified statutes (id, last_modified_date)
    pub recently_modified: Vec<(String, DateTime<Utc>)>,
    /// Least modified statutes (id, last_modified_date)
    pub least_modified: Vec<(String, DateTime<Utc>)>,
    /// Statutes by status change frequency (id, status_change_count)
    pub frequent_status_changes: Vec<(String, usize)>,
    /// Average modification frequency (days between modifications)
    pub avg_modification_frequency_days: f64,
}
impl ActivityAnalytics {
    /// Creates a new activity analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns statutes modified within the last N days.
    pub fn modified_within_days(&self, days: i64) -> Vec<String> {
        let threshold = Utc::now() - chrono::Duration::days(days);
        self.recently_modified
            .iter()
            .filter(|(_, date)| *date > threshold)
            .map(|(id, _)| id.clone())
            .collect()
    }
}
