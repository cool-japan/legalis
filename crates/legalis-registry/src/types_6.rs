//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

use super::functions::WebhookCallback;
use super::types_5::{
    AuditReportFormat, LineageEntry, MergeStrategy, RegistryBackup, WebhookEventFilter,
};
use super::types_7::{AuditEntry, MergeConflict, Permission, Role};
use super::types_8::ValidationError;

/// Audit report configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportConfig {
    /// Report title
    pub title: String,
    /// Start date filter
    pub start_date: Option<DateTime<Utc>>,
    /// End date filter
    pub end_date: Option<DateTime<Utc>>,
    /// Include operations
    pub include_operations: bool,
    /// Include events
    pub include_events: bool,
    /// Include quality metrics
    pub include_quality: bool,
    /// Include PII scan results
    pub include_pii_scans: bool,
    /// Report format
    pub format: AuditReportFormat,
}
impl AuditReportConfig {
    /// Creates a new audit report configuration.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }
    /// Sets the date range.
    pub fn with_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_date = Some(start);
        self.end_date = Some(end);
        self
    }
    /// Sets what to include in the report.
    pub fn with_sections(
        mut self,
        operations: bool,
        events: bool,
        quality: bool,
        pii_scans: bool,
    ) -> Self {
        self.include_operations = operations;
        self.include_events = events;
        self.include_quality = quality;
        self.include_pii_scans = pii_scans;
        self
    }
    /// Sets the report format.
    pub fn with_format(mut self, format: AuditReportFormat) -> Self {
        self.format = format;
        self
    }
}
/// Validates that jurisdiction is valid.
#[derive(Debug, Clone)]
pub struct ValidJurisdictionRule {
    /// Allowed jurisdictions
    pub allowed: HashSet<String>,
}
impl ValidJurisdictionRule {
    /// Creates a new jurisdiction validation rule.
    pub fn new(allowed: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            allowed: allowed.into_iter().map(|s| s.into()).collect(),
        }
    }
}
/// Aggregation functions for grouping and counting.
#[derive(Debug, Clone)]
pub struct AggregationResult {
    /// Group key -> count
    pub counts: HashMap<String, usize>,
    /// Total items aggregated
    pub total: usize,
}
impl AggregationResult {
    /// Creates a new aggregation result.
    pub fn new(counts: HashMap<String, usize>) -> Self {
        let total = counts.values().sum();
        Self { counts, total }
    }
    /// Returns the count for a specific group.
    pub fn get_count(&self, key: &str) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }
    /// Returns all groups sorted by count (descending).
    pub fn sorted_by_count(&self) -> Vec<(String, usize)> {
        let mut pairs: Vec<_> = self.counts.iter().map(|(k, &v)| (k.clone(), v)).collect();
        pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
        pairs
    }
    /// Returns the percentage for a specific group.
    pub fn percentage(&self, key: &str) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.get_count(key) as f64 / self.total as f64) * 100.0
    }
}
/// Webhook subscription.
#[derive(Clone)]
pub struct WebhookSubscription {
    /// Unique ID for this subscription
    pub id: Uuid,
    /// Optional name/description
    pub name: Option<String>,
    /// Callback function
    pub(crate) callback: WebhookCallback,
    /// Filter: only trigger for specific event types
    pub event_filter: Option<WebhookEventFilter>,
}
/// Statistics about the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of statutes
    pub total_statutes: usize,
    /// Total number of versions across all statutes
    pub total_versions: usize,
    /// Total number of events
    pub total_events: usize,
    /// Total number of unique tags
    pub total_tags: usize,
    /// Total number of jurisdictions
    pub total_jurisdictions: usize,
    /// Count by status
    pub by_status: HashMap<StatuteStatus, usize>,
    /// Count by jurisdiction
    pub by_jurisdiction: HashMap<String, usize>,
}
/// Performance benchmark result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Number of iterations
    pub iterations: usize,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
    /// Average duration per operation in microseconds
    pub avg_duration_us: f64,
    /// Operations per second
    pub ops_per_sec: f64,
    /// Minimum duration in microseconds
    pub min_duration_us: u64,
    /// Maximum duration in microseconds
    pub max_duration_us: u64,
}
impl BenchmarkResult {
    /// Creates a new benchmark result.
    pub fn new(name: String, iterations: usize, durations_us: Vec<u64>) -> Self {
        let total_duration_us: u64 = durations_us.iter().sum();
        let total_duration_ms = total_duration_us / 1000;
        let avg_duration_us = total_duration_us as f64 / iterations as f64;
        let ops_per_sec = 1_000_000.0 / avg_duration_us;
        let min_duration_us = *durations_us.iter().min().unwrap_or(&0);
        let max_duration_us = *durations_us.iter().max().unwrap_or(&0);
        Self {
            name,
            iterations,
            total_duration_ms,
            avg_duration_us,
            ops_per_sec,
            min_duration_us,
            max_duration_us,
        }
    }
    /// Returns a formatted summary.
    pub fn summary(&self) -> String {
        format!(
            "{}: {:.2} ops/sec, avg: {:.2}µs, min: {}µs, max: {}µs ({} iterations)",
            self.name,
            self.ops_per_sec,
            self.avg_duration_us,
            self.min_duration_us,
            self.max_duration_us,
            self.iterations
        )
    }
}
/// Type of lineage operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageOperation {
    /// Created from scratch
    Created,
    /// Imported from external source
    Imported { source: String },
    /// Derived from another statute
    Derived { parent_id: String },
    /// Merged from multiple statutes
    Merged { source_ids: Vec<String> },
    /// Enriched by automatic process
    Enriched { enrichment_type: String },
    /// Validated by validation rule
    Validated { rule_name: String },
    /// Transformed by custom logic
    Transformed { transformation: String },
}
/// PII (Personally Identifiable Information) field types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PiiFieldType {
    /// Name of a person
    Name,
    /// Email address
    Email,
    /// Phone number
    PhoneNumber,
    /// Social security number or national ID
    NationalId,
    /// Physical address
    Address,
    /// Date of birth
    DateOfBirth,
    /// IP address
    IpAddress,
    /// Custom PII type
    Custom(String),
}
/// User with access control attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessUser {
    /// User ID
    pub user_id: String,
    /// Display name
    pub display_name: String,
    /// Primary role
    pub role: Role,
    /// User attributes for ABAC
    pub attributes: HashMap<String, String>,
    /// Directly assigned permissions (overrides role)
    pub direct_permissions: Vec<Permission>,
}
impl AccessUser {
    /// Creates a new user with a role.
    pub fn new(user_id: impl Into<String>, display_name: impl Into<String>, role: Role) -> Self {
        Self {
            user_id: user_id.into(),
            display_name: display_name.into(),
            role,
            attributes: HashMap::new(),
            direct_permissions: Vec::new(),
        }
    }
    /// Adds a user attribute.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
    /// Adds a direct permission.
    pub fn with_permission(mut self, permission: Permission) -> Self {
        if !self.direct_permissions.contains(&permission) {
            self.direct_permissions.push(permission);
        }
        self
    }
    /// Gets all permissions (role + direct).
    pub fn all_permissions(&self) -> Vec<Permission> {
        let mut perms = self.role.permissions();
        for p in &self.direct_permissions {
            if !perms.contains(p) {
                perms.push(*p);
            }
        }
        perms
    }
    /// Checks if user has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.all_permissions().contains(&permission)
    }
}
/// Data lineage tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    /// All lineage entries
    pub(super) entries: Vec<LineageEntry>,
    /// Maximum entries to keep (for memory management)
    max_entries: usize,
}
impl DataLineage {
    /// Creates a new data lineage tracker.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    /// Records a lineage entry.
    pub fn record(&mut self, entry: LineageEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }
    }
    /// Gets lineage history for a statute.
    pub fn get_lineage(&self, statute_id: &str) -> Vec<&LineageEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id == statute_id)
            .collect()
    }
    /// Gets lineage entries by operation type.
    pub fn get_by_operation(&self, operation_type: &str) -> Vec<&LineageEntry> {
        self.entries
            .iter()
            .filter(|e| match &e.operation {
                LineageOperation::Created => operation_type == "Created",
                LineageOperation::Imported { .. } => operation_type == "Imported",
                LineageOperation::Derived { .. } => operation_type == "Derived",
                LineageOperation::Merged { .. } => operation_type == "Merged",
                LineageOperation::Enriched { .. } => operation_type == "Enriched",
                LineageOperation::Validated { .. } => operation_type == "Validated",
                LineageOperation::Transformed { .. } => operation_type == "Transformed",
            })
            .collect()
    }
    /// Gets lineage entries by actor.
    pub fn get_by_actor(&self, actor: &str) -> Vec<&LineageEntry> {
        self.entries.iter().filter(|e| e.actor == actor).collect()
    }
    /// Gets lineage entries in a time range.
    pub fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&LineageEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
    /// Traces the full provenance chain for a statute.
    pub fn trace_provenance(&self, statute_id: &str) -> Vec<&LineageEntry> {
        let mut provenance = Vec::new();
        let mut current_ids = vec![statute_id.to_string()];
        let mut visited = HashSet::new();
        while let Some(id) = current_ids.pop() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id.clone());
            for entry in self.get_lineage(&id) {
                provenance.push(entry);
                match &entry.operation {
                    LineageOperation::Derived { parent_id } if !visited.contains(parent_id) => {
                        current_ids.push(parent_id.clone());
                    }
                    LineageOperation::Merged { source_ids } => {
                        for source_id in source_ids {
                            if !visited.contains(source_id) {
                                current_ids.push(source_id.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        provenance.sort_by_key(|e| e.timestamp);
        provenance
    }
    /// Exports lineage to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.entries)
    }
    /// Clears all lineage entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Returns total number of lineage entries.
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}
/// A search result with relevance scoring.
#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    /// The statute entry
    pub entry: &'a StatuteEntry,
    /// Relevance score (0.0 - 1.0, higher is better)
    pub score: f64,
    /// Match highlights (field -> matched text)
    pub highlights: HashMap<String, Vec<String>>,
}
impl<'a> SearchResult<'a> {
    /// Creates a new search result with a given score.
    pub fn new(entry: &'a StatuteEntry, score: f64) -> Self {
        Self {
            entry,
            score: score.clamp(0.0, 1.0),
            highlights: HashMap::new(),
        }
    }
    /// Adds a highlight for a field.
    pub fn add_highlight(&mut self, field: String, matched: String) {
        self.highlights.entry(field).or_default().push(matched);
    }
    /// Gets highlights for a specific field.
    pub fn get_highlights(&self, field: &str) -> Option<&Vec<String>> {
        self.highlights.get(field)
    }
}
/// Rate limit configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: usize,
    /// Time window in seconds
    pub window_secs: i64,
    /// Whether to enable rate limiting
    pub enabled: bool,
}
impl RateLimitConfig {
    /// Creates a new rate limit config.
    pub fn new(max_requests: usize, window_secs: i64) -> Self {
        Self {
            max_requests,
            window_secs,
            enabled: true,
        }
    }
    /// Disables rate limiting.
    pub fn disabled() -> Self {
        Self {
            max_requests: 0,
            window_secs: 0,
            enabled: false,
        }
    }
    /// Builder: Sets enabled flag.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
/// Result of a merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// The merged statute entry
    pub entry: StatuteEntry,
    /// Conflicts that were resolved
    pub conflicts: Vec<MergeConflict>,
    /// Whether the merge was automatic or had conflicts
    pub has_conflicts: bool,
}
impl MergeResult {
    /// Returns true if the merge was successful without conflicts.
    pub fn is_clean(&self) -> bool {
        !self.has_conflicts
    }
}
/// Result of batch validation.
#[derive(Debug, Clone)]
pub struct BatchValidationResult {
    /// Total number of entries validated
    pub total: usize,
    /// Number of valid entries
    pub valid: usize,
    /// Number of invalid entries
    pub invalid: usize,
    /// Validation errors by statute ID
    pub errors: HashMap<String, ValidationError>,
}
impl BatchValidationResult {
    /// Returns true if all entries are valid.
    pub fn is_all_valid(&self) -> bool {
        self.invalid == 0
    }
    /// Returns the validation success rate (0.0 to 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.valid as f64 / self.total as f64
        }
    }
}
/// A versioned statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteEntry {
    /// Unique registry ID
    pub registry_id: Uuid,
    /// The statute data
    pub statute: Statute,
    /// Version number
    pub version: u32,
    /// ETag for optimistic concurrency control
    pub etag: String,
    /// Status
    pub status: StatuteStatus,
    /// Effective date
    pub effective_date: Option<DateTime<Utc>>,
    /// Expiry date
    pub expiry_date: Option<DateTime<Utc>>,
    /// Parent statute (for amendments)
    pub amends: Option<String>,
    /// Statutes this one supersedes
    pub supersedes: Vec<String>,
    /// References to other statutes
    pub references: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Metadata
    pub metadata: HashMap<String, String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}
impl StatuteEntry {
    /// Creates a new statute entry.
    pub fn new(statute: Statute, jurisdiction: impl Into<String>) -> Self {
        let now = Utc::now();
        let registry_id = Uuid::new_v4();
        let etag = Self::generate_etag(&registry_id, 1, &now);
        Self {
            registry_id,
            statute,
            version: 1,
            etag,
            status: StatuteStatus::Draft,
            effective_date: None,
            expiry_date: None,
            amends: None,
            supersedes: Vec::new(),
            references: Vec::new(),
            tags: Vec::new(),
            jurisdiction: jurisdiction.into(),
            metadata: HashMap::new(),
            created_at: now,
            modified_at: now,
        }
    }
    /// Generates an ETag for optimistic concurrency control.
    fn generate_etag(registry_id: &Uuid, version: u32, modified_at: &DateTime<Utc>) -> String {
        format!(
            "{}-v{}-{}",
            registry_id,
            version,
            modified_at.timestamp_nanos_opt().unwrap_or(0)
        )
    }
    /// Updates the ETag after modification.
    pub(crate) fn update_etag(&mut self) {
        self.etag = Self::generate_etag(&self.registry_id, self.version, &self.modified_at);
    }
    /// Sets the effective date.
    pub fn with_effective_date(mut self, date: DateTime<Utc>) -> Self {
        self.effective_date = Some(date);
        self
    }
    /// Sets the status.
    pub fn with_status(mut self, status: StatuteStatus) -> Self {
        self.status = status;
        self
    }
    /// Adds a reference.
    pub fn with_reference(mut self, statute_id: impl Into<String>) -> Self {
        self.references.push(statute_id.into());
        self
    }
    /// Adds a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    /// Sets the expiry date.
    pub fn with_expiry_date(mut self, date: DateTime<Utc>) -> Self {
        self.expiry_date = Some(date);
        self
    }
    /// Sets the parent statute (for amendments).
    pub fn with_amends(mut self, statute_id: impl Into<String>) -> Self {
        self.amends = Some(statute_id.into());
        self
    }
    /// Adds a superseded statute.
    pub fn with_supersedes(mut self, statute_id: impl Into<String>) -> Self {
        self.supersedes.push(statute_id.into());
        self
    }
    /// Adds metadata key-value pair.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = jurisdiction.into();
        self
    }
    /// Returns whether this statute is currently active.
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.status == StatuteStatus::Active
            && self.effective_date.is_none_or(|d| d <= now)
            && self.expiry_date.is_none_or(|d| d > now)
    }
}
impl StatuteEntry {
    /// Merges another statute entry into this one using the specified strategy.
    ///
    /// This is useful for reconciling concurrent modifications.
    pub fn merge(&self, other: &StatuteEntry, strategy: MergeStrategy) -> MergeResult {
        let mut merged = self.clone();
        let mut conflicts = Vec::new();
        if self.statute.title != other.statute.title {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.statute.title = other.statute.title.clone();
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::Title {
                        old: self.statute.title.clone(),
                        new: other.statute.title.clone(),
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.statute.title = other.statute.title.clone();
                }
            }
        }
        if self.status != other.status {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.status = other.status;
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::Status {
                        old: self.status,
                        new: other.status,
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.status = other.status;
                }
            }
        }
        if self.jurisdiction != other.jurisdiction {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.jurisdiction = other.jurisdiction.clone();
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::Jurisdiction {
                        old: self.jurisdiction.clone(),
                        new: other.jurisdiction.clone(),
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.jurisdiction = other.jurisdiction.clone();
                }
            }
        }
        if self.effective_date != other.effective_date {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.effective_date = other.effective_date;
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::EffectiveDate {
                        old: self.effective_date,
                        new: other.effective_date,
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.effective_date = other.effective_date;
                }
            }
        }
        if self.expiry_date != other.expiry_date {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.expiry_date = other.expiry_date;
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::ExpiryDate {
                        old: self.expiry_date,
                        new: other.expiry_date,
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.expiry_date = other.expiry_date;
                }
            }
        }
        let old_tags: HashSet<_> = self.tags.iter().cloned().collect();
        let new_tags: HashSet<_> = other.tags.iter().cloned().collect();
        merged.tags = old_tags.union(&new_tags).cloned().collect();
        match strategy {
            MergeStrategy::PreferOld => {}
            MergeStrategy::PreferNew => {
                merged.metadata = other.metadata.clone();
            }
            MergeStrategy::MergeBoth => {
                for (k, v) in &other.metadata {
                    merged.metadata.insert(k.clone(), v.clone());
                }
            }
            MergeStrategy::FailOnConflict => {
                for (k, v) in &other.metadata {
                    if !merged.metadata.contains_key(k) {
                        merged.metadata.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        let old_refs: HashSet<_> = self.references.iter().cloned().collect();
        let new_refs: HashSet<_> = other.references.iter().cloned().collect();
        merged.references = old_refs.union(&new_refs).cloned().collect();
        let old_super: HashSet<_> = self.supersedes.iter().cloned().collect();
        let new_super: HashSet<_> = other.supersedes.iter().cloned().collect();
        merged.supersedes = old_super.union(&new_super).cloned().collect();
        merged.modified_at = Utc::now();
        merged.update_etag();
        MergeResult {
            entry: merged,
            has_conflicts: !conflicts.is_empty(),
            conflicts,
        }
    }
}
/// Audit trail manager for tracking all operations.
#[derive(Debug, Clone)]
pub struct AuditTrail {
    pub(super) entries: VecDeque<AuditEntry>,
    max_entries: usize,
    pub(super) enabled: bool,
}
impl AuditTrail {
    /// Creates a new audit trail with maximum entries.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
            enabled: true,
        }
    }
    /// Records an audit entry.
    pub fn record(&mut self, entry: AuditEntry) {
        if !self.enabled {
            return;
        }
        self.entries.push_back(entry);
        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }
    /// Enables audit logging.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disables audit logging.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Checks if audit logging is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Returns all audit entries.
    pub fn entries(&self) -> &VecDeque<AuditEntry> {
        &self.entries
    }
    /// Returns entries for a specific actor.
    pub fn entries_by_actor(&self, actor: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.actor == actor).collect()
    }
    /// Returns entries for a specific statute.
    pub fn entries_by_statute(&self, statute_id: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id.as_deref() == Some(statute_id))
            .collect()
    }
    /// Returns entries within a time range.
    pub fn entries_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
    /// Returns entries by operation type.
    pub fn entries_by_operation(&self, operation_type: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| format!("{:?}", e.operation).contains(operation_type))
            .collect()
    }
    /// Returns only successful operations.
    pub fn successful_operations(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.is_success()).collect()
    }
    /// Returns only failed operations.
    pub fn failed_operations(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.is_failure()).collect()
    }
    /// Returns the total number of entries.
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    /// Clears all audit entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Exports audit trail to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.entries)
    }
}
/// Relationship analytics for analyzing statute dependencies and supersession chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipAnalytics {
    /// Most referenced statutes (id, reference_count)
    pub most_referenced: Vec<(String, usize)>,
    /// Statutes with most dependencies (id, dependency_count)
    pub most_dependencies: Vec<(String, usize)>,
    /// Supersession chains (root_id -> chain of superseded IDs)
    pub supersession_chains: HashMap<String, Vec<String>>,
    /// Orphaned statutes (no references to or from other statutes)
    pub orphaned_statutes: Vec<String>,
    /// Average references per statute
    pub avg_references_per_statute: f64,
}
impl RelationshipAnalytics {
    /// Creates a new relationship analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns the longest supersession chain length.
    pub fn max_chain_length(&self) -> usize {
        self.supersession_chains
            .values()
            .map(|chain| chain.len())
            .max()
            .unwrap_or(0)
    }
    /// Returns the total number of relationships.
    pub fn total_relationships(&self) -> usize {
        self.most_referenced.iter().map(|(_, count)| count).sum()
    }
}
/// A point-in-time snapshot of the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySnapshot {
    /// Snapshot ID
    pub snapshot_id: Uuid,
    /// When the snapshot was created
    pub created_at: DateTime<Utc>,
    /// Full registry backup
    pub backup: RegistryBackup,
    /// Snapshot description
    pub description: Option<String>,
}
impl RegistrySnapshot {
    /// Creates a new snapshot from a backup.
    pub fn new(backup: RegistryBackup, description: Option<String>) -> Self {
        Self {
            snapshot_id: Uuid::new_v4(),
            created_at: Utc::now(),
            backup,
            description,
        }
    }
}
/// Status of a statute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatuteStatus {
    /// Being drafted
    Draft,
    /// Under review
    UnderReview,
    /// Approved but not yet effective
    Approved,
    /// Currently in force
    Active,
    /// No longer in force
    Repealed,
    /// Replaced by another statute
    Superseded,
}
/// Dependency graph for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Root statute ID
    pub root_id: String,
    /// Forward dependencies: statute_id -> set of statutes it depends on
    pub dependencies: HashMap<String, HashSet<String>>,
    /// Reverse dependencies: statute_id -> set of statutes that depend on it
    pub reverse_dependencies: HashMap<String, HashSet<String>>,
}
impl DependencyGraph {
    /// Returns all statutes that the root depends on (directly or indirectly).
    pub fn all_dependencies(&self) -> HashSet<String> {
        let mut all_deps = HashSet::new();
        for deps in self.dependencies.values() {
            all_deps.extend(deps.iter().cloned());
        }
        all_deps
    }
    /// Returns all statutes that depend on the root (directly or indirectly).
    pub fn all_dependents(&self) -> HashSet<String> {
        self.reverse_dependencies
            .get(&self.root_id)
            .cloned()
            .unwrap_or_default()
    }
    /// Returns the depth of the dependency tree.
    pub fn depth(&self) -> usize {
        self.calculate_depth(&self.root_id, &mut HashSet::new())
    }
    fn calculate_depth(&self, statute_id: &str, visited: &mut HashSet<String>) -> usize {
        if visited.contains(statute_id) {
            return 0;
        }
        visited.insert(statute_id.to_string());
        if let Some(deps) = self.dependencies.get(statute_id) {
            if deps.is_empty() {
                return 1;
            }
            deps.iter()
                .map(|dep| self.calculate_depth(dep, visited))
                .max()
                .unwrap_or(0)
                + 1
        } else {
            1
        }
    }
}
/// Result of an audited operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditResult {
    /// Operation succeeded
    Success,
    /// Operation failed with error message
    Failure { error: String },
    /// Operation partially succeeded
    PartialSuccess { succeeded: usize, failed: usize },
}
