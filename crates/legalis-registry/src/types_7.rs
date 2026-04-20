//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Utc};
use legalis_core::EffectType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{
    AuditOperation, DuplicateCandidate, EnrichmentType, HealthStatus, TemporaryAccess,
};
use super::types_6::{AccessUser, AuditResult, BenchmarkResult, StatuteEntry, StatuteStatus};
use super::types_8::{AbacCondition, ComponentHealth};

fn default_true() -> bool {
    true
}

/// Search query for statutes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Full-text search term
    pub text: Option<String>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Filter by jurisdiction
    pub jurisdiction: Option<String>,
    /// Filter by status
    pub status: Option<StatuteStatus>,
    /// Filter by active statutes only
    pub active_only: bool,
    /// Filter by effective date range
    pub effective_date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by expiry date range
    pub expiry_date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by modified date range
    pub modified_date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by version number
    pub version: Option<u32>,
    /// Filter by minimum version
    pub min_version: Option<u32>,
    /// Filter by effect type
    pub effect_type: Option<EffectType>,
    /// Exclude statutes with these tags
    pub exclude_tags: Vec<String>,
    /// Include only statutes that reference these IDs
    pub references: Vec<String>,
    /// Include only statutes with supersedes relationships
    pub has_supersedes: Option<bool>,
}
impl SearchQuery {
    /// Creates a new empty search query.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the text search term.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    /// Adds a tag filter.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    /// Sets the jurisdiction filter.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
    /// Sets the status filter.
    pub fn with_status(mut self, status: StatuteStatus) -> Self {
        self.status = Some(status);
        self
    }
    /// Sets the active-only filter.
    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }
    /// Sets the effective date range filter.
    pub fn with_effective_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.effective_date_range = Some((start, end));
        self
    }
    /// Sets the expiry date range filter.
    pub fn with_expiry_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.expiry_date_range = Some((start, end));
        self
    }
    /// Sets the modified date range filter.
    pub fn with_modified_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.modified_date_range = Some((start, end));
        self
    }
    /// Sets the version filter.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }
    /// Sets the minimum version filter.
    pub fn with_min_version(mut self, min_version: u32) -> Self {
        self.min_version = Some(min_version);
        self
    }
    /// Sets the effect type filter.
    pub fn with_effect_type(mut self, effect_type: EffectType) -> Self {
        self.effect_type = Some(effect_type);
        self
    }
    /// Adds a tag to exclude.
    pub fn exclude_tag(mut self, tag: impl Into<String>) -> Self {
        self.exclude_tags.push(tag.into());
        self
    }
    /// Adds a reference filter (statute must reference this ID).
    pub fn with_reference(mut self, reference_id: impl Into<String>) -> Self {
        self.references.push(reference_id.into());
        self
    }
    /// Filters for statutes that have supersedes relationships.
    pub fn with_supersedes(mut self) -> Self {
        self.has_supersedes = Some(true);
        self
    }
    /// Filters for statutes that don't have supersedes relationships.
    pub fn without_supersedes(mut self) -> Self {
        self.has_supersedes = Some(false);
        self
    }
}
/// Temporal analytics for tracking registry growth and changes over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnalytics {
    /// Number of statutes registered per day (date -> count)
    pub registrations_per_day: HashMap<String, usize>,
    /// Number of updates per day (date -> count)
    pub updates_per_day: HashMap<String, usize>,
    /// Average version count per statute
    pub avg_versions_per_statute: f64,
    /// Statutes with highest version velocity (id, version_count)
    pub most_versioned_statutes: Vec<(String, usize)>,
    /// Growth rate (statutes per day) over the period
    pub growth_rate: f64,
    /// Peak activity date and count
    pub peak_activity_date: Option<(String, usize)>,
}
impl TemporalAnalytics {
    /// Creates a new temporal analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns the total number of registrations across all days.
    pub fn total_registrations(&self) -> usize {
        self.registrations_per_day.values().sum()
    }
    /// Returns the total number of updates across all days.
    pub fn total_updates(&self) -> usize {
        self.updates_per_day.values().sum()
    }
    /// Returns the total activity (registrations + updates).
    pub fn total_activity(&self) -> usize {
        self.total_registrations() + self.total_updates()
    }
}
/// Result of a bulk operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    /// Total items processed
    pub total_processed: usize,
    /// Successful operations
    pub successful: usize,
    /// Failed operations
    pub failed: usize,
    /// Error details by statute ID
    pub errors: HashMap<String, String>,
    /// Duration of the operation
    pub duration_ms: u64,
}
impl BulkOperationResult {
    /// Creates a new empty result.
    pub fn new() -> Self {
        Self {
            total_processed: 0,
            successful: 0,
            failed: 0,
            errors: HashMap::new(),
            duration_ms: 0,
        }
    }
    /// Checks if all operations succeeded.
    pub fn is_all_successful(&self) -> bool {
        self.failed == 0 && self.total_processed > 0
    }
    /// Returns the success rate (0.0-1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total_processed == 0 {
            0.0
        } else {
            self.successful as f64 / self.total_processed as f64
        }
    }
}
/// Similarity measure between two statutes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimilarityScore {
    /// Overall similarity (0.0 - 1.0)
    pub overall: f64,
    /// Title similarity
    pub title: f64,
    /// Content similarity
    pub content: f64,
    /// Metadata similarity
    pub metadata: f64,
}
impl SimilarityScore {
    /// Creates a new similarity score.
    pub fn new(title: f64, content: f64, metadata: f64) -> Self {
        let overall = (title * 0.4 + content * 0.5 + metadata * 0.1).clamp(0.0, 1.0);
        Self {
            overall,
            title,
            content,
            metadata,
        }
    }
    /// Checks if similarity exceeds threshold (likely duplicate).
    pub fn is_likely_duplicate(&self, threshold: f64) -> bool {
        self.overall >= threshold
    }
    /// Checks if similarity suggests possible duplicate.
    pub fn is_possible_duplicate(&self, threshold: f64) -> bool {
        self.overall >= threshold * 0.7
    }
}
/// Result of applying retention policies.
#[derive(Debug, Clone)]
pub struct RetentionResult {
    /// IDs of statutes that were archived
    pub archived_ids: Vec<String>,
    /// Reason for each archival
    pub reasons: HashMap<String, String>,
    /// Total statutes evaluated
    pub total_evaluated: usize,
}
impl RetentionResult {
    /// Creates a new retention result.
    pub fn new(total_evaluated: usize) -> Self {
        Self {
            archived_ids: Vec::new(),
            reasons: HashMap::new(),
            total_evaluated,
        }
    }
    /// Records an archived statute.
    pub fn record_archived(&mut self, statute_id: String, reason: String) {
        self.archived_ids.push(statute_id.clone());
        self.reasons.insert(statute_id, reason);
    }
    /// Returns the number of statutes archived.
    pub fn archived_count(&self) -> usize {
        self.archived_ids.len()
    }
}
/// Result of duplicate detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDetectionResult {
    /// All duplicate candidates found
    pub candidates: Vec<DuplicateCandidate>,
    /// Similarity threshold used
    pub threshold: f64,
    /// Number of statutes analyzed
    pub statutes_analyzed: usize,
    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
}
impl DuplicateDetectionResult {
    /// Creates a new duplicate detection result.
    pub fn new(threshold: f64, statutes_analyzed: usize) -> Self {
        Self {
            candidates: Vec::new(),
            threshold,
            statutes_analyzed,
            detected_at: Utc::now(),
        }
    }
    /// Adds a duplicate candidate.
    pub fn add_candidate(&mut self, candidate: DuplicateCandidate) {
        self.candidates.push(candidate);
    }
    /// Returns only likely duplicates (high confidence).
    pub fn likely_duplicates(&self) -> Vec<&DuplicateCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.similarity.is_likely_duplicate(self.threshold))
            .collect()
    }
    /// Returns possible duplicates (medium confidence).
    pub fn possible_duplicates(&self) -> Vec<&DuplicateCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.similarity.is_possible_duplicate(self.threshold))
            .collect()
    }
    /// Returns total number of duplicate pairs found.
    pub fn total_duplicates(&self) -> usize {
        self.candidates.len()
    }
}
/// A conflict that occurred during merge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeConflict {
    /// Title conflict
    Title { old: String, new: String },
    /// Status conflict
    Status {
        old: StatuteStatus,
        new: StatuteStatus,
    },
    /// Jurisdiction conflict
    Jurisdiction { old: String, new: String },
    /// Effective date conflict
    EffectiveDate {
        old: Option<DateTime<Utc>>,
        new: Option<DateTime<Utc>>,
    },
    /// Expiry date conflict
    ExpiryDate {
        old: Option<DateTime<Utc>>,
        new: Option<DateTime<Utc>>,
    },
}
/// Audit log entry capturing detailed operation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique audit ID
    pub audit_id: Uuid,
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// User or system that performed the operation
    pub actor: String,
    /// Type of operation performed
    pub operation: AuditOperation,
    /// Statute ID affected (if applicable)
    pub statute_id: Option<String>,
    /// Result of the operation
    pub result: AuditResult,
    /// IP address or source identifier
    pub source: Option<String>,
    /// Additional context data
    pub metadata: HashMap<String, String>,
}
impl AuditEntry {
    /// Creates a new audit entry.
    pub fn new(actor: String, operation: AuditOperation, result: AuditResult) -> Self {
        Self {
            audit_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            actor,
            operation,
            statute_id: None,
            result,
            source: None,
            metadata: HashMap::new(),
        }
    }
    /// Builder: Sets the statute ID.
    pub fn with_statute_id(mut self, statute_id: String) -> Self {
        self.statute_id = Some(statute_id);
        self
    }
    /// Builder: Sets the source.
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    /// Builder: Adds metadata.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    /// Checks if the operation was successful.
    pub fn is_success(&self) -> bool {
        matches!(self.result, AuditResult::Success)
    }
    /// Checks if the operation failed.
    pub fn is_failure(&self) -> bool {
        matches!(self.result, AuditResult::Failure { .. })
    }
}
/// Enrichment suggestion for a statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentSuggestion {
    /// Type of enrichment
    pub enrichment_type: EnrichmentType,
    /// Suggested value or action
    pub suggestion: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Reason for suggestion
    pub reason: String,
}
impl EnrichmentSuggestion {
    /// Creates a new enrichment suggestion.
    pub fn new(
        enrichment_type: EnrichmentType,
        suggestion: String,
        confidence: f64,
        reason: String,
    ) -> Self {
        Self {
            enrichment_type,
            suggestion,
            confidence: confidence.clamp(0.0, 1.0),
            reason,
        }
    }
    /// Checks if suggestion meets confidence threshold.
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
}
/// Result of automatic enrichment analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentResult {
    /// Statute ID being enriched
    pub statute_id: String,
    /// List of suggestions
    pub suggestions: Vec<EnrichmentSuggestion>,
    /// Timestamp of analysis
    pub analyzed_at: DateTime<Utc>,
}
impl EnrichmentResult {
    /// Creates a new enrichment result.
    pub fn new(statute_id: String) -> Self {
        Self {
            statute_id,
            suggestions: Vec::new(),
            analyzed_at: Utc::now(),
        }
    }
    /// Adds a suggestion to the result.
    pub fn add_suggestion(&mut self, suggestion: EnrichmentSuggestion) {
        self.suggestions.push(suggestion);
    }
    /// Returns suggestions meeting a confidence threshold.
    pub fn high_confidence_suggestions(&self, threshold: f64) -> Vec<&EnrichmentSuggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.meets_threshold(threshold))
            .collect()
    }
    /// Groups suggestions by type.
    pub fn suggestions_by_type(
        &self,
        enrichment_type: EnrichmentType,
    ) -> Vec<&EnrichmentSuggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.enrichment_type == enrichment_type)
            .collect()
    }
}
/// Benchmark suite for registry operations.
#[derive(Debug, Clone, Default)]
pub struct BenchmarkSuite {
    results: Vec<BenchmarkResult>,
}
impl BenchmarkSuite {
    /// Creates a new benchmark suite.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    /// Adds a benchmark result.
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }
    /// Returns all benchmark results.
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
    /// Exports results to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.results)
    }
    /// Returns a summary of all benchmarks.
    pub fn summary(&self) -> String {
        let mut summary = String::from("Benchmark Results:\n");
        for result in &self.results {
            summary.push_str(&format!("  {}\n", result.summary()));
        }
        summary
    }
}
/// Data sovereignty configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfig {
    /// Primary region where data is stored
    pub primary_region: GeographicRegion,
    /// Allowed replication regions
    pub allowed_regions: Vec<GeographicRegion>,
    /// Enforce strict residency (no cross-region access)
    pub strict_residency: bool,
    /// Require encryption for cross-region transfer
    pub require_encryption: bool,
}
impl DataSovereigntyConfig {
    /// Creates a new data sovereignty configuration.
    pub fn new(primary_region: GeographicRegion) -> Self {
        Self {
            primary_region,
            allowed_regions: Vec::new(),
            strict_residency: false,
            require_encryption: true,
        }
    }
    /// Adds an allowed region for replication.
    pub fn allow_region(mut self, region: GeographicRegion) -> Self {
        if !self.allowed_regions.contains(&region) {
            self.allowed_regions.push(region);
        }
        self
    }
    /// Enables strict residency mode.
    pub fn with_strict_residency(mut self, strict: bool) -> Self {
        self.strict_residency = strict;
        self
    }
    /// Sets encryption requirement.
    pub fn with_encryption_required(mut self, required: bool) -> Self {
        self.require_encryption = required;
        self
    }
    /// Checks if a region is allowed for data storage/access.
    pub fn is_region_allowed(&self, region: &GeographicRegion) -> bool {
        if region == &self.primary_region {
            return true;
        }
        if self.strict_residency {
            return false;
        }
        self.allowed_regions.contains(region) && self.primary_region.allows_transfer_to(region)
    }
}
/// User role in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Viewer - read-only access
    Viewer,
    /// Editor - read and write access
    Editor,
    /// Admin - full access including permissions management
    Admin,
}
impl Role {
    /// Returns permissions granted to this role.
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::Viewer => Permission::read_only(),
            Role::Editor => Permission::editor(),
            Role::Admin => Permission::all(),
        }
    }
    /// Checks if this role has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
    /// Checks if this role is at least the specified level.
    pub fn is_at_least(&self, other: Role) -> bool {
        self >= &other
    }
}
/// Permission types for statute operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read statute content
    Read,
    /// Create new statutes
    Create,
    /// Update existing statutes
    Update,
    /// Delete statutes
    Delete,
    /// Change statute status
    ChangeStatus,
    /// Add/remove tags
    ManageTags,
    /// Add/remove metadata
    ManageMetadata,
    /// Add/remove references
    ManageReferences,
    /// Archive/unarchive statutes
    Archive,
    /// Manage permissions
    ManagePermissions,
    /// Execute bulk operations
    BulkOperations,
    /// Generate reports
    GenerateReports,
}
impl Permission {
    /// Returns all available permissions.
    pub fn all() -> Vec<Permission> {
        vec![
            Permission::Read,
            Permission::Create,
            Permission::Update,
            Permission::Delete,
            Permission::ChangeStatus,
            Permission::ManageTags,
            Permission::ManageMetadata,
            Permission::ManageReferences,
            Permission::Archive,
            Permission::ManagePermissions,
            Permission::BulkOperations,
            Permission::GenerateReports,
        ]
    }
    /// Returns read-only permissions.
    pub fn read_only() -> Vec<Permission> {
        vec![Permission::Read, Permission::GenerateReports]
    }
    /// Returns editor permissions (read + write, no delete/admin).
    pub fn editor() -> Vec<Permission> {
        vec![
            Permission::Read,
            Permission::Create,
            Permission::Update,
            Permission::ChangeStatus,
            Permission::ManageTags,
            Permission::ManageMetadata,
            Permission::ManageReferences,
            Permission::GenerateReports,
        ]
    }
}
/// Comprehensive health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Overall health status
    pub status: HealthStatus,
    /// Timestamp of the check
    pub timestamp: DateTime<Utc>,
    /// Total statutes in registry
    pub statute_count: usize,
    /// Total versions tracked
    pub version_count: usize,
    /// Total events in event store
    pub event_count: usize,
    /// Cache hit rate (0.0-1.0)
    pub cache_hit_rate: f64,
    /// Number of archived statutes
    pub archived_count: usize,
    /// Memory usage estimate (bytes)
    pub memory_estimate_bytes: usize,
    /// Check duration (milliseconds)
    pub check_duration_ms: u64,
    /// Component-specific checks
    pub component_checks: HashMap<String, ComponentHealth>,
}
/// Result of applying retention rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionExecutionResult {
    /// Statutes deleted
    pub deleted: Vec<String>,
    /// Statutes archived
    pub archived: Vec<String>,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
    /// Was this a dry run?
    pub dry_run: bool,
}
impl RetentionExecutionResult {
    /// Creates a new execution result.
    pub fn new(deleted: Vec<String>, archived: Vec<String>, dry_run: bool) -> Self {
        Self {
            deleted,
            archived,
            executed_at: Utc::now(),
            dry_run,
        }
    }
    /// Returns total affected statutes.
    pub fn total_affected(&self) -> usize {
        self.deleted.len() + self.archived.len()
    }
}
/// Access control manager.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessControlManager {
    /// Registered users
    #[serde(default)]
    pub(crate) users: HashMap<String, AccessUser>,
    /// Access policies
    #[serde(default)]
    pub(crate) policies: Vec<AccessPolicy>,
    /// Temporary access grants
    #[serde(default)]
    pub(crate) temporary_grants: Vec<TemporaryAccess>,
    /// Enable/disable access control
    #[serde(default = "default_true")]
    pub(crate) enabled: bool,
}
impl AccessControlManager {
    /// Creates a new access control manager.
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            policies: Vec::new(),
            temporary_grants: Vec::new(),
            enabled: true,
        }
    }
    /// Registers a user.
    pub fn add_user(&mut self, user: AccessUser) {
        self.users.insert(user.user_id.clone(), user);
    }
    /// Gets a user by ID.
    pub fn get_user(&self, user_id: &str) -> Option<&AccessUser> {
        self.users.get(user_id)
    }
    /// Updates a user's role.
    pub fn update_user_role(&mut self, user_id: &str, role: Role) -> bool {
        if let Some(user) = self.users.get_mut(user_id) {
            user.role = role;
            true
        } else {
            false
        }
    }
    /// Adds an access policy.
    pub fn add_policy(&mut self, policy: AccessPolicy) {
        self.policies.push(policy);
        self.policies.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }
    /// Grants temporary access.
    pub fn grant_temporary_access(&mut self, grant: TemporaryAccess) {
        self.temporary_grants.push(grant);
    }
    /// Cleans up expired temporary grants.
    pub fn cleanup_expired_grants(&mut self) {
        self.temporary_grants.retain(|g| g.is_valid());
    }
    /// Checks if a user has permission for an operation.
    pub fn check_permission(
        &self,
        user_id: &str,
        permission: Permission,
        statute_id: Option<&str>,
        statute_entry: Option<&StatuteEntry>,
    ) -> bool {
        if !self.enabled {
            return true;
        }
        let user = match self.get_user(user_id) {
            Some(u) => u,
            None => return false,
        };
        if user.has_permission(permission) {
            return true;
        }
        if let Some(sid) = statute_id {
            for grant in &self.temporary_grants {
                if grant.user_id == user_id
                    && grant.is_valid()
                    && grant.applies_to(sid)
                    && grant.permissions.contains(&permission)
                {
                    return true;
                }
            }
        }
        for policy in &self.policies {
            if let Some(req_role) = policy.required_role
                && !user.role.is_at_least(req_role)
            {
                continue;
            }
            if !policy.conditions_met(&user.attributes, statute_entry) {
                continue;
            }
            if policy.grants(permission) {
                return true;
            }
        }
        false
    }
    /// Lists all active temporary grants for a user.
    pub fn list_user_grants(&self, user_id: &str) -> Vec<&TemporaryAccess> {
        self.temporary_grants
            .iter()
            .filter(|g| g.user_id == user_id && g.is_valid())
            .collect()
    }
    /// Revokes a temporary grant.
    pub fn revoke_grant(&mut self, grant_id: Uuid) -> bool {
        let len_before = self.temporary_grants.len();
        self.temporary_grants.retain(|g| g.grant_id != grant_id);
        self.temporary_grants.len() < len_before
    }
    /// Enables or disables access control.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Returns whether access control is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Returns total number of users.
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
    /// Returns total number of policies.
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }
    /// Returns number of active temporary grants.
    pub fn active_grant_count(&self) -> usize {
        self.temporary_grants
            .iter()
            .filter(|g| g.is_valid())
            .count()
    }
}
/// Access control policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy ID
    pub policy_id: Uuid,
    /// Policy name
    pub name: String,
    /// Required role
    pub required_role: Option<Role>,
    /// Specific permissions granted
    pub permissions: Vec<Permission>,
    /// ABAC conditions
    pub conditions: Vec<AbacCondition>,
    /// Priority (higher = evaluated first)
    pub priority: i32,
    /// Is policy enabled?
    pub enabled: bool,
}
impl AccessPolicy {
    /// Creates a new access policy.
    pub fn new(name: impl Into<String>, permissions: Vec<Permission>) -> Self {
        Self {
            policy_id: Uuid::new_v4(),
            name: name.into(),
            required_role: None,
            permissions,
            conditions: Vec::new(),
            priority: 0,
            enabled: true,
        }
    }
    /// Sets the required role.
    pub fn with_role(mut self, role: Role) -> Self {
        self.required_role = Some(role);
        self
    }
    /// Adds an ABAC condition.
    pub fn with_condition(mut self, condition: AbacCondition) -> Self {
        self.conditions.push(condition);
        self
    }
    /// Sets the priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    /// Checks if the policy grants a specific permission.
    pub fn grants(&self, permission: Permission) -> bool {
        self.enabled && self.permissions.contains(&permission)
    }
    /// Checks if all conditions are met.
    pub fn conditions_met(
        &self,
        user_attrs: &HashMap<String, String>,
        statute_entry: Option<&StatuteEntry>,
    ) -> bool {
        self.conditions
            .iter()
            .all(|c| c.evaluate(user_attrs, statute_entry))
    }
}
/// Geographic region for data sovereignty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// European Union
    EU,
    /// United States
    US,
    /// United Kingdom
    UK,
    /// Asia Pacific
    APAC,
    /// Japan
    Japan,
    /// China
    China,
    /// Custom region
    Custom(String),
}
impl GeographicRegion {
    /// Returns the region code.
    pub fn code(&self) -> String {
        match self {
            GeographicRegion::EU => "EU".to_string(),
            GeographicRegion::US => "US".to_string(),
            GeographicRegion::UK => "UK".to_string(),
            GeographicRegion::APAC => "APAC".to_string(),
            GeographicRegion::Japan => "JP".to_string(),
            GeographicRegion::China => "CN".to_string(),
            GeographicRegion::Custom(s) => s.clone(),
        }
    }
    /// Checks if this region allows data transfer to another region.
    pub fn allows_transfer_to(&self, other: &GeographicRegion) -> bool {
        match (self, other) {
            (GeographicRegion::EU, GeographicRegion::EU) => true,
            (GeographicRegion::EU, GeographicRegion::UK) => true,
            (GeographicRegion::EU, _) => false,
            _ => true,
        }
    }
}
/// Operation metrics for the registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Total number of registrations
    pub registrations: u64,
    /// Total number of updates
    pub updates: u64,
    /// Total number of reads
    pub reads: u64,
    /// Total number of searches
    pub searches: u64,
    /// Total number of deletes (if supported)
    pub deletes: u64,
    /// Total number of status changes
    pub status_changes: u64,
    /// Total number of tag operations
    pub tag_operations: u64,
    /// Total number of metadata operations
    pub metadata_operations: u64,
    /// Total number of cache hits
    pub cache_hits: u64,
    /// Total number of cache misses
    pub cache_misses: u64,
    /// Total number of webhook triggers
    pub webhook_triggers: u64,
    /// Total number of validation failures
    pub validation_failures: u64,
}
impl OperationMetrics {
    /// Creates new empty metrics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
    /// Returns the total number of operations.
    pub fn total_operations(&self) -> u64 {
        self.registrations
            + self.updates
            + self.reads
            + self.searches
            + self.deletes
            + self.status_changes
            + self.tag_operations
            + self.metadata_operations
    }
    /// Resets all metrics to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// Validates that title is not empty.
#[derive(Debug, Clone)]
pub struct NonEmptyTitleRule;
