//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::functions::RegistryResult;
use super::types::{FieldChange, PiiDetection, RegistryError, TenantStats};
use super::types_3::{BackupMetadata, RegistryEvent};
use super::types_4::StatuteRegistry;
use super::types_6::{LineageOperation, PiiFieldType, StatuteEntry, StatuteStatus};
use super::types_8::{MaskingStrategy, PiiScanResult, QualityScore, StatuteSummary};

/// Incremental backup containing only changes since last backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalBackup {
    /// Base snapshot ID this incremental is built upon
    pub base_snapshot_id: Uuid,
    /// When this incremental was created
    pub created_at: DateTime<Utc>,
    /// Events since the base snapshot
    pub delta_events: Vec<RegistryEvent>,
    /// Statutes added or modified since base
    pub changed_statutes: Vec<StatuteEntry>,
    /// IDs of statutes deleted since base
    pub deleted_statute_ids: Vec<String>,
}
impl IncrementalBackup {
    /// Creates a new incremental backup.
    pub fn new(base_snapshot_id: Uuid) -> Self {
        Self {
            base_snapshot_id,
            created_at: Utc::now(),
            delta_events: Vec::new(),
            changed_statutes: Vec::new(),
            deleted_statute_ids: Vec::new(),
        }
    }
    /// Returns the total number of changes.
    pub fn change_count(&self) -> usize {
        self.delta_events.len() + self.changed_statutes.len() + self.deleted_statute_ids.len()
    }
}
/// A backup of the registry state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryBackup {
    /// All current statutes
    pub statutes: Vec<StatuteEntry>,
    /// All version history
    pub versions: HashMap<String, HashMap<u32, StatuteEntry>>,
    /// Event history
    pub events: Vec<RegistryEvent>,
    /// Backup metadata
    pub metadata: BackupMetadata,
}
/// PII detector and handler.
#[derive(Debug, Clone)]
pub struct PiiDetector {
    /// Enable/disable PII detection
    pub(super) enabled: bool,
    /// Minimum confidence threshold
    pub(super) min_confidence: f64,
    /// Masking strategy
    pub(crate) masking_strategy: MaskingStrategy,
}
impl PiiDetector {
    /// Creates a new PII detector with default settings.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the minimum confidence threshold.
    pub fn with_min_confidence(mut self, threshold: f64) -> Self {
        self.min_confidence = threshold.clamp(0.0, 1.0);
        self
    }
    /// Sets the masking strategy.
    pub fn with_masking_strategy(mut self, strategy: MaskingStrategy) -> Self {
        self.masking_strategy = strategy;
        self
    }
    /// Enables or disables PII detection.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Scans statute content for PII.
    pub fn scan(&self, statute_id: &str, content: &str) -> PiiScanResult {
        if !self.enabled {
            return PiiScanResult::new(statute_id.to_string(), Vec::new());
        }
        let mut detections = Vec::new();
        if let Some(email_regex) = Self::email_pattern() {
            for (idx, _) in content.match_indices(&email_regex) {
                if let Some(end) = content[idx..].find(|c: char| c.is_whitespace()) {
                    let email = &content[idx..idx + end];
                    if email.contains('@') {
                        detections.push(PiiDetection::new(
                            PiiFieldType::Email,
                            email.to_string(),
                            idx,
                            0.9,
                        ));
                    }
                }
            }
        }
        for (idx, _) in content.match_indices(char::is_numeric) {
            let rest = &content[idx..];
            if let Some(number) = Self::extract_phone_number(rest)
                && number.len() >= 10
            {
                detections.push(PiiDetection::new(
                    PiiFieldType::PhoneNumber,
                    number.clone(),
                    idx,
                    0.8,
                ));
            }
        }
        for (idx, _) in content.match_indices(char::is_numeric) {
            if let Some(ip) = Self::extract_ip_address(&content[idx..]) {
                detections.push(PiiDetection::new(
                    PiiFieldType::IpAddress,
                    ip.clone(),
                    idx,
                    0.95,
                ));
            }
        }
        PiiScanResult::new(statute_id.to_string(), detections)
    }
    /// Masks PII in content based on detection results.
    pub fn mask(&self, content: &str, scan_result: &PiiScanResult) -> String {
        let mut masked = content.to_string();
        let mut offset = 0i32;
        let mut sorted_detections = scan_result.detections.clone();
        sorted_detections.sort_by_key(|d| d.position);
        for detection in sorted_detections.iter() {
            if !detection.is_confident(self.min_confidence) {
                continue;
            }
            let pos = (detection.position as i32 + offset) as usize;
            let masked_value = self.apply_masking(&detection.value, &detection.field_type);
            let original_len = detection.length;
            let new_len = masked_value.len();
            if pos + original_len <= masked.len() {
                masked.replace_range(pos..pos + original_len, &masked_value);
                offset += new_len as i32 - original_len as i32;
            }
        }
        masked
    }
    /// Applies masking strategy to a value.
    fn apply_masking(&self, value: &str, field_type: &PiiFieldType) -> String {
        match self.masking_strategy {
            MaskingStrategy::Asterisks => "*".repeat(value.len().min(8)),
            MaskingStrategy::Redacted => "[REDACTED]".to_string(),
            MaskingStrategy::TypeMarker => format!("[{:?}]", field_type).to_uppercase(),
            MaskingStrategy::Hash => format!("[HASH:{}]", value.len()),
            MaskingStrategy::Partial => {
                if value.len() <= 4 {
                    "*".repeat(value.len())
                } else {
                    let chars: Vec<char> = value.chars().collect();
                    let mut result = String::new();
                    for (i, ch) in chars.iter().enumerate() {
                        if i == 0 || i == chars.len() - 1 {
                            result.push(*ch);
                        } else {
                            result.push('*');
                        }
                    }
                    result
                }
            }
        }
    }
    fn email_pattern() -> Option<&'static str> {
        Some("@")
    }
    fn extract_phone_number(text: &str) -> Option<String> {
        let mut number = String::new();
        for ch in text.chars().take(15) {
            if ch.is_numeric() || ch == '-' || ch == '(' || ch == ')' || ch == ' ' {
                number.push(ch);
            } else {
                break;
            }
        }
        if number.chars().filter(|c| c.is_numeric()).count() >= 10 {
            Some(number.trim().to_string())
        } else {
            None
        }
    }
    fn extract_ip_address(text: &str) -> Option<String> {
        let parts: Vec<&str> = text.split('.').take(4).collect();
        if parts.len() == 4 {
            let ip: String = parts.join(".");
            if ip.chars().all(|c| c.is_numeric() || c == '.') {
                return Some(ip);
            }
        }
        None
    }
}
/// Ranking configuration for search results.
#[derive(Debug, Clone, Copy)]
pub struct RankingConfig {
    /// Weight for title matches (default: 3.0)
    pub title_weight: f64,
    /// Weight for ID matches (default: 2.0)
    pub id_weight: f64,
    /// Weight for tag matches (default: 1.5)
    pub tag_weight: f64,
    /// Weight for jurisdiction matches (default: 1.0)
    pub jurisdiction_weight: f64,
    /// Boost for exact matches (default: 2.0)
    pub exact_match_boost: f64,
}
impl RankingConfig {
    /// Creates a new ranking configuration.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the title weight.
    pub fn with_title_weight(mut self, weight: f64) -> Self {
        self.title_weight = weight;
        self
    }
    /// Sets the ID weight.
    pub fn with_id_weight(mut self, weight: f64) -> Self {
        self.id_weight = weight;
        self
    }
    /// Sets the tag weight.
    pub fn with_tag_weight(mut self, weight: f64) -> Self {
        self.tag_weight = weight;
        self
    }
    /// Sets the jurisdiction weight.
    pub fn with_jurisdiction_weight(mut self, weight: f64) -> Self {
        self.jurisdiction_weight = weight;
        self
    }
    /// Sets the exact match boost.
    pub fn with_exact_match_boost(mut self, boost: f64) -> Self {
        self.exact_match_boost = boost;
        self
    }
}
/// Quality assessment for a statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    /// Statute ID being assessed
    pub statute_id: String,
    /// Quality score
    pub score: QualityScore,
    /// Issues found
    pub issues: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
}
impl QualityAssessment {
    /// Creates a new quality assessment.
    pub fn new(statute_id: String, score: QualityScore) -> Self {
        Self {
            statute_id,
            score,
            issues: Vec::new(),
            suggestions: Vec::new(),
            assessed_at: Utc::now(),
        }
    }
    /// Adds an issue to the assessment.
    pub fn with_issue(mut self, issue: String) -> Self {
        self.issues.push(issue);
        self
    }
    /// Adds a suggestion to the assessment.
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestions.push(suggestion);
        self
    }
    /// Checks if the assessment has any issues.
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}
/// Metric type for observability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter that only increases
    Counter { value: u64 },
    /// Gauge that can increase or decrease
    Gauge { value: f64 },
    /// Histogram of values
    Histogram { values: Vec<f64> },
    /// Timing measurement in microseconds
    Timing { duration_us: u64 },
}
/// Data retention rule for automatic cleanup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataRetentionRule {
    /// Retain for a specific number of days
    RetainForDays(u32),
    /// Retain until a specific date
    RetainUntil(DateTime<Utc>),
    /// Retain indefinitely
    RetainIndefinitely,
    /// Delete after statute becomes inactive for N days
    DeleteInactiveAfterDays(u32),
    /// Archive after N days instead of deleting
    ArchiveAfterDays(u32),
}
/// Audit report format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditReportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Plain text format
    Text,
    /// HTML format
    Html,
}
/// Represents differences between two statute entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDiff {
    /// Statute ID
    pub statute_id: String,
    /// Old version number
    pub old_version: u32,
    /// New version number
    pub new_version: u32,
    /// Title changes
    pub title: Option<FieldChange<String>>,
    /// Status changes
    pub status: Option<FieldChange<StatuteStatus>>,
    /// Effective date changes
    pub effective_date: Option<FieldChange<DateTime<Utc>>>,
    /// Expiry date changes
    pub expiry_date: Option<FieldChange<DateTime<Utc>>>,
    /// Jurisdiction changes
    pub jurisdiction: Option<FieldChange<String>>,
    /// Tags added
    pub tags_added: Vec<String>,
    /// Tags removed
    pub tags_removed: Vec<String>,
    /// Metadata added
    pub metadata_added: HashMap<String, String>,
    /// Metadata removed
    pub metadata_removed: HashMap<String, String>,
    /// Metadata changed
    pub metadata_changed: HashMap<String, (String, String)>,
    /// References added
    pub references_added: Vec<String>,
    /// References removed
    pub references_removed: Vec<String>,
    /// Supersedes added
    pub supersedes_added: Vec<String>,
    /// Supersedes removed
    pub supersedes_removed: Vec<String>,
    /// Whether the statute content itself changed
    pub content_changed: bool,
}
impl StatuteDiff {
    /// Computes the difference between two statute entries.
    pub fn compute(old: &StatuteEntry, new: &StatuteEntry) -> Self {
        let old_tags: HashSet<_> = old.tags.iter().collect();
        let new_tags: HashSet<_> = new.tags.iter().collect();
        let tags_added: Vec<_> = new_tags
            .difference(&old_tags)
            .map(|s| (*s).clone())
            .collect();
        let tags_removed: Vec<_> = old_tags
            .difference(&new_tags)
            .map(|s| (*s).clone())
            .collect();
        let mut metadata_added = HashMap::new();
        let mut metadata_removed = HashMap::new();
        let mut metadata_changed = HashMap::new();
        for (key, new_val) in &new.metadata {
            match old.metadata.get(key) {
                Some(old_val) if old_val != new_val => {
                    metadata_changed.insert(key.clone(), (old_val.clone(), new_val.clone()));
                }
                None => {
                    metadata_added.insert(key.clone(), new_val.clone());
                }
                _ => {}
            }
        }
        for (key, old_val) in &old.metadata {
            if !new.metadata.contains_key(key) {
                metadata_removed.insert(key.clone(), old_val.clone());
            }
        }
        let old_refs: HashSet<_> = old.references.iter().collect();
        let new_refs: HashSet<_> = new.references.iter().collect();
        let references_added: Vec<_> = new_refs
            .difference(&old_refs)
            .map(|s| (*s).clone())
            .collect();
        let references_removed: Vec<_> = old_refs
            .difference(&new_refs)
            .map(|s| (*s).clone())
            .collect();
        let old_supersedes: HashSet<_> = old.supersedes.iter().collect();
        let new_supersedes: HashSet<_> = new.supersedes.iter().collect();
        let supersedes_added: Vec<_> = new_supersedes
            .difference(&old_supersedes)
            .map(|s| (*s).clone())
            .collect();
        let supersedes_removed: Vec<_> = old_supersedes
            .difference(&new_supersedes)
            .map(|s| (*s).clone())
            .collect();
        let content_changed = serde_json::to_string(&old.statute).unwrap_or_default()
            != serde_json::to_string(&new.statute).unwrap_or_default();
        StatuteDiff {
            statute_id: new.statute.id.clone(),
            old_version: old.version,
            new_version: new.version,
            title: FieldChange::from_values(&old.statute.title, &new.statute.title)
                .is_changed()
                .then(|| FieldChange::from_values(&old.statute.title, &new.statute.title)),
            status: FieldChange::from_values(&old.status, &new.status)
                .is_changed()
                .then(|| FieldChange::from_values(&old.status, &new.status)),
            effective_date: FieldChange::from_optional(
                old.effective_date.as_ref(),
                new.effective_date.as_ref(),
            ),
            expiry_date: FieldChange::from_optional(
                old.expiry_date.as_ref(),
                new.expiry_date.as_ref(),
            ),
            jurisdiction: FieldChange::from_values(&old.jurisdiction, &new.jurisdiction)
                .is_changed()
                .then(|| FieldChange::from_values(&old.jurisdiction, &new.jurisdiction)),
            tags_added,
            tags_removed,
            metadata_added,
            metadata_removed,
            metadata_changed,
            references_added,
            references_removed,
            supersedes_added,
            supersedes_removed,
            content_changed,
        }
    }
    /// Returns true if there are any changes.
    pub fn has_changes(&self) -> bool {
        self.title.as_ref().is_some_and(|c| c.is_changed())
            || self.status.as_ref().is_some_and(|c| c.is_changed())
            || self.effective_date.as_ref().is_some_and(|c| c.is_changed())
            || self.expiry_date.as_ref().is_some_and(|c| c.is_changed())
            || self.jurisdiction.as_ref().is_some_and(|c| c.is_changed())
            || !self.tags_added.is_empty()
            || !self.tags_removed.is_empty()
            || !self.metadata_added.is_empty()
            || !self.metadata_removed.is_empty()
            || !self.metadata_changed.is_empty()
            || !self.references_added.is_empty()
            || !self.references_removed.is_empty()
            || !self.supersedes_added.is_empty()
            || !self.supersedes_removed.is_empty()
            || self.content_changed
    }
    /// Returns a human-readable summary of changes.
    pub fn summary(&self) -> String {
        let mut changes = Vec::new();
        if self.title.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("title");
        }
        if self.status.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("status");
        }
        if self.effective_date.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("effective date");
        }
        if self.expiry_date.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("expiry date");
        }
        if self.jurisdiction.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("jurisdiction");
        }
        if !self.tags_added.is_empty() || !self.tags_removed.is_empty() {
            changes.push("tags");
        }
        if !self.metadata_added.is_empty()
            || !self.metadata_removed.is_empty()
            || !self.metadata_changed.is_empty()
        {
            changes.push("metadata");
        }
        if !self.references_added.is_empty() || !self.references_removed.is_empty() {
            changes.push("references");
        }
        if !self.supersedes_added.is_empty() || !self.supersedes_removed.is_empty() {
            changes.push("supersedes");
        }
        if self.content_changed {
            changes.push("content");
        }
        if changes.is_empty() {
            "No changes".to_string()
        } else {
            format!("Changed: {}", changes.join(", "))
        }
    }
}
/// Compliance dashboard metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    /// Dashboard ID
    pub dashboard_id: Uuid,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Total statutes under management
    pub total_statutes: usize,
    /// Statutes with PII detected
    pub statutes_with_pii: usize,
    /// Statutes subject to retention
    pub statutes_pending_retention: usize,
    /// Average quality score
    pub avg_quality_score: f64,
    /// Statutes below quality threshold
    pub low_quality_count: usize,
    /// Total audit events
    pub total_audit_events: usize,
    /// Failed audit events
    pub failed_operations: usize,
    /// Data sovereignty violations
    pub sovereignty_violations: usize,
    /// Compliance rate (0.0-1.0)
    pub compliance_rate: f64,
}
impl ComplianceDashboard {
    /// Creates a new compliance dashboard.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        total_statutes: usize,
        statutes_with_pii: usize,
        statutes_pending_retention: usize,
        avg_quality_score: f64,
        low_quality_count: usize,
        total_audit_events: usize,
        failed_operations: usize,
        sovereignty_violations: usize,
    ) -> Self {
        let compliance_rate = if total_statutes > 0 {
            let compliant =
                total_statutes.saturating_sub(low_quality_count + sovereignty_violations);
            compliant as f64 / total_statutes as f64
        } else {
            1.0
        };
        Self {
            dashboard_id: Uuid::new_v4(),
            generated_at: Utc::now(),
            total_statutes,
            statutes_with_pii,
            statutes_pending_retention,
            avg_quality_score,
            low_quality_count,
            total_audit_events,
            failed_operations,
            sovereignty_violations,
            compliance_rate,
        }
    }
    /// Returns true if compliance rate meets threshold.
    pub fn meets_compliance_threshold(&self, threshold: f64) -> bool {
        self.compliance_rate >= threshold
    }
    /// Exports dashboard to JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}
/// Pagination parameters.
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    /// Page number (0-indexed)
    pub page: usize,
    /// Items per page
    pub per_page: usize,
}
impl Pagination {
    /// Creates new pagination parameters.
    pub fn new(page: usize, per_page: usize) -> Self {
        Self { page, per_page }
    }
    /// Creates pagination for the first page.
    pub fn first(per_page: usize) -> Self {
        Self { page: 0, per_page }
    }
    /// Returns pagination for the next page.
    pub fn next(&self) -> Self {
        Self {
            page: self.page + 1,
            per_page: self.per_page,
        }
    }
    /// Returns pagination for the previous page (saturating at 0).
    pub fn prev(&self) -> Self {
        Self {
            page: self.page.saturating_sub(1),
            per_page: self.per_page,
        }
    }
    /// Sets the page number.
    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }
    /// Sets the items per page.
    pub fn with_per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page;
        self
    }
    /// Returns the offset for the current page.
    pub fn offset(&self) -> usize {
        self.page * self.per_page
    }
    /// Returns the limit for the current page.
    pub fn limit(&self) -> usize {
        self.per_page
    }
}
/// Multi-tenant registry manager.
///
/// Allows managing multiple isolated registries for different tenants.
#[derive(Debug)]
pub struct MultiTenantRegistry {
    /// Registry for each tenant
    tenants: HashMap<String, StatuteRegistry>,
    /// Default tenant ID (if any)
    default_tenant: Option<String>,
}
impl MultiTenantRegistry {
    /// Creates a new multi-tenant registry.
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
            default_tenant: None,
        }
    }
    /// Creates a new multi-tenant registry with a default tenant.
    pub fn with_default_tenant(tenant_id: impl Into<String>) -> Self {
        let tenant_id = tenant_id.into();
        let mut tenants = HashMap::new();
        tenants.insert(tenant_id.clone(), StatuteRegistry::new());
        Self {
            tenants,
            default_tenant: Some(tenant_id),
        }
    }
    /// Creates a new tenant registry.
    pub fn create_tenant(&mut self, tenant_id: impl Into<String>) -> RegistryResult<()> {
        let tenant_id = tenant_id.into();
        if self.tenants.contains_key(&tenant_id) {
            return Err(RegistryError::DuplicateId(format!(
                "Tenant '{}' already exists",
                tenant_id
            )));
        }
        self.tenants.insert(tenant_id, StatuteRegistry::new());
        Ok(())
    }
    /// Deletes a tenant registry.
    pub fn delete_tenant(&mut self, tenant_id: &str) -> RegistryResult<()> {
        self.tenants.remove(tenant_id).ok_or_else(|| {
            RegistryError::StatuteNotFound(format!("Tenant '{}' not found", tenant_id))
        })?;
        if self.default_tenant.as_deref() == Some(tenant_id) {
            self.default_tenant = None;
        }
        Ok(())
    }
    /// Gets a mutable reference to a tenant's registry.
    pub fn get_tenant_mut(&mut self, tenant_id: &str) -> RegistryResult<&mut StatuteRegistry> {
        self.tenants.get_mut(tenant_id).ok_or_else(|| {
            RegistryError::StatuteNotFound(format!("Tenant '{}' not found", tenant_id))
        })
    }
    /// Gets a reference to a tenant's registry.
    pub fn get_tenant(&self, tenant_id: &str) -> RegistryResult<&StatuteRegistry> {
        self.tenants.get(tenant_id).ok_or_else(|| {
            RegistryError::StatuteNotFound(format!("Tenant '{}' not found", tenant_id))
        })
    }
    /// Gets a mutable reference to the default tenant's registry.
    pub fn get_default_mut(&mut self) -> RegistryResult<&mut StatuteRegistry> {
        let tenant_id = self
            .default_tenant
            .as_ref()
            .ok_or_else(|| RegistryError::InvalidOperation("No default tenant set".to_string()))?
            .clone();
        self.get_tenant_mut(&tenant_id)
    }
    /// Gets a reference to the default tenant's registry.
    pub fn get_default(&self) -> RegistryResult<&StatuteRegistry> {
        let tenant_id = self
            .default_tenant
            .as_ref()
            .ok_or_else(|| RegistryError::InvalidOperation("No default tenant set".to_string()))?;
        self.get_tenant(tenant_id)
    }
    /// Lists all tenant IDs.
    pub fn list_tenants(&self) -> Vec<&String> {
        self.tenants.keys().collect()
    }
    /// Returns the number of tenants.
    pub fn tenant_count(&self) -> usize {
        self.tenants.len()
    }
    /// Checks if a tenant exists.
    pub fn has_tenant(&self, tenant_id: &str) -> bool {
        self.tenants.contains_key(tenant_id)
    }
    /// Sets the default tenant.
    pub fn set_default_tenant(&mut self, tenant_id: impl Into<String>) -> RegistryResult<()> {
        let tenant_id = tenant_id.into();
        if !self.tenants.contains_key(&tenant_id) {
            return Err(RegistryError::StatuteNotFound(format!(
                "Tenant '{}' not found",
                tenant_id
            )));
        }
        self.default_tenant = Some(tenant_id);
        Ok(())
    }
    /// Exports a tenant's registry to a backup.
    pub fn export_tenant(
        &self,
        tenant_id: &str,
        description: Option<String>,
    ) -> RegistryResult<String> {
        let registry = self.get_tenant(tenant_id)?;
        registry
            .export_backup(description)
            .map_err(|e| RegistryError::InvalidOperation(format!("Export failed: {}", e)))
    }
    /// Imports a backup into a tenant's registry.
    pub fn import_tenant(&mut self, tenant_id: &str, json: &str) -> RegistryResult<()> {
        let registry = self.get_tenant_mut(tenant_id)?;
        registry
            .import_backup(json)
            .map_err(|e| RegistryError::InvalidOperation(format!("Import failed: {}", e)))
    }
    /// Clones a tenant's registry to a new tenant.
    pub fn clone_tenant(
        &mut self,
        source_tenant_id: &str,
        new_tenant_id: impl Into<String>,
    ) -> RegistryResult<()> {
        let new_tenant_id = new_tenant_id.into();
        if self.tenants.contains_key(&new_tenant_id) {
            return Err(RegistryError::DuplicateId(format!(
                "Tenant '{}' already exists",
                new_tenant_id
            )));
        }
        let backup_json = self.export_tenant(source_tenant_id, None)?;
        self.create_tenant(&new_tenant_id)?;
        self.import_tenant(&new_tenant_id, &backup_json)?;
        Ok(())
    }
    /// Returns statistics for all tenants.
    pub fn tenant_statistics(&self) -> HashMap<String, TenantStats> {
        self.tenants
            .iter()
            .map(|(id, registry)| {
                let stats = TenantStats {
                    statute_count: registry.count(),
                    event_count: registry.event_count(),
                    active_statute_count: registry.list_active().len(),
                    tag_count: registry.all_tags().len(),
                    jurisdiction_count: registry.all_jurisdictions().len(),
                };
                (id.clone(), stats)
            })
            .collect()
    }
    /// Returns summaries for all tenants (lazy loading).
    pub fn list_tenant_summaries(&self) -> HashMap<String, Vec<StatuteSummary>> {
        self.tenants
            .iter()
            .map(|(id, registry)| {
                let summaries = registry.list_summaries();
                (id.clone(), summaries)
            })
            .collect()
    }
}
/// Retention policy rule for auto-archiving statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionRule {
    /// Archive statutes that have expired
    ExpiredStatutes { reason: String },
    /// Archive statutes older than specified days since effective date
    OlderThanDays { days: i64, reason: String },
    /// Archive statutes with specific status
    ByStatus {
        status: StatuteStatus,
        reason: String,
    },
    /// Archive statutes superseded by others
    SupersededStatutes { reason: String },
    /// Archive statutes not modified within specified days
    InactiveForDays { days: i64, reason: String },
}
/// Lazy loading configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct LazyLoadConfig {
    /// Load statute content on demand
    pub lazy_content: bool,
    /// Load version history on demand
    pub lazy_versions: bool,
    /// Load events on demand
    pub lazy_events: bool,
}
impl LazyLoadConfig {
    /// Creates a config with all lazy loading enabled.
    pub fn all() -> Self {
        Self {
            lazy_content: true,
            lazy_versions: true,
            lazy_events: true,
        }
    }
    /// Creates a config with all lazy loading disabled.
    pub fn none() -> Self {
        Self::default()
    }
}
/// Strategy for resolving conflicts during merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Prefer the older version's values
    PreferOld,
    /// Prefer the newer version's values
    PreferNew,
    /// Fail if there are conflicts
    FailOnConflict,
    /// Merge both values (for collections)
    MergeBoth,
}
/// Lineage entry tracking data provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEntry {
    /// Unique lineage ID
    pub lineage_id: Uuid,
    /// Statute ID this lineage applies to
    pub statute_id: String,
    /// Operation performed
    pub operation: LineageOperation,
    /// Timestamp of operation
    pub timestamp: DateTime<Utc>,
    /// Actor who performed operation (user, system, etc.)
    pub actor: String,
    /// Additional context
    pub context: HashMap<String, String>,
}
impl LineageEntry {
    /// Creates a new lineage entry.
    pub fn new(statute_id: String, operation: LineageOperation, actor: String) -> Self {
        Self {
            lineage_id: Uuid::new_v4(),
            statute_id,
            operation,
            timestamp: Utc::now(),
            actor,
            context: HashMap::new(),
        }
    }
    /// Adds context to the lineage entry.
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
}
/// Data retention configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    /// Retention rules
    #[serde(default)]
    rules: Vec<DataRetentionRule>,
    /// Auto-apply retention rules
    #[serde(default)]
    auto_apply: bool,
    /// Dry-run mode (don't actually delete)
    #[serde(default)]
    dry_run: bool,
}
impl DataRetentionConfig {
    /// Creates a new retention configuration.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a retention rule.
    pub fn add_rule(mut self, rule: DataRetentionRule) -> Self {
        self.rules.push(rule);
        self
    }
    /// Enables auto-apply mode.
    pub fn with_auto_apply(mut self, auto_apply: bool) -> Self {
        self.auto_apply = auto_apply;
        self
    }
    /// Enables dry-run mode.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
    /// Returns all rules.
    pub fn rules(&self) -> &[DataRetentionRule] {
        &self.rules
    }
    /// Returns whether auto-apply is enabled.
    pub fn is_auto_apply(&self) -> bool {
        self.auto_apply
    }
    /// Returns whether dry-run mode is enabled.
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}
/// Filter for webhook events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventFilter {
    /// Only trigger on statute registration
    StatuteRegistered,
    /// Only trigger on statute updates
    StatuteUpdated,
    /// Only trigger on status changes
    StatusChanged,
    /// Only trigger on tag operations
    TagOperations,
    /// Only trigger on reference operations
    ReferenceOperations,
    /// Only trigger on metadata updates
    MetadataUpdated,
    /// Trigger on any event
    All,
}
impl WebhookEventFilter {
    /// Checks if an event matches this filter.
    pub fn matches(&self, event: &RegistryEvent) -> bool {
        match self {
            Self::All => true,
            Self::StatuteRegistered => {
                matches!(event, RegistryEvent::StatuteRegistered { .. })
            }
            Self::StatuteUpdated => matches!(event, RegistryEvent::StatuteUpdated { .. }),
            Self::StatusChanged => matches!(event, RegistryEvent::StatusChanged { .. }),
            Self::TagOperations => {
                matches!(
                    event,
                    RegistryEvent::TagAdded { .. } | RegistryEvent::TagRemoved { .. }
                )
            }
            Self::ReferenceOperations => {
                matches!(
                    event,
                    RegistryEvent::ReferenceAdded { .. } | RegistryEvent::ReferenceRemoved { .. }
                )
            }
            Self::MetadataUpdated => {
                matches!(event, RegistryEvent::MetadataUpdated { .. })
            }
        }
    }
}
