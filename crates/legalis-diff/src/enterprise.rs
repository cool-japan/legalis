//! Enterprise diff management for large-scale statute operations.
//!
//! This module provides functionality for:
//! - Diff archiving and retention
//! - Diff search and discovery
//! - Audit trail for diff operations
//! - Diff analytics dashboard data
//! - Role-based diff access control

use crate::StatuteDiff;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An archived diff with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedDiff {
    /// The diff itself.
    pub diff: StatuteDiff,
    /// When it was archived.
    pub archived_at: DateTime<Utc>,
    /// Who archived it.
    pub archived_by: String,
    /// Retention policy.
    pub retention_policy: RetentionPolicy,
    /// Tags for organization.
    pub tags: Vec<String>,
    /// Arbitrary metadata.
    pub metadata: HashMap<String, String>,
}

/// Retention policy for archived diffs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionPolicy {
    /// Keep indefinitely.
    Permanent,
    /// Keep for a specified number of years.
    Years(u32),
    /// Keep until a specific date.
    UntilDate(DateTime<Utc>),
    /// Keep until manually deleted.
    Manual,
}

impl RetentionPolicy {
    /// Checks if a diff should be retained at the given time.
    pub fn should_retain(&self, archived_at: DateTime<Utc>, current_time: DateTime<Utc>) -> bool {
        match self {
            RetentionPolicy::Permanent | RetentionPolicy::Manual => true,
            RetentionPolicy::Years(years) => {
                let retention_end = archived_at + Duration::days((years * 365) as i64);
                current_time < retention_end
            }
            RetentionPolicy::UntilDate(date) => current_time < *date,
        }
    }
}

/// A diff archive for storing and managing diffs.
#[derive(Debug, Clone, Default)]
pub struct DiffArchive {
    /// Archived diffs indexed by statute ID.
    diffs: HashMap<String, Vec<ArchivedDiff>>,
}

impl DiffArchive {
    /// Creates a new diff archive.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::enterprise::DiffArchive;
    ///
    /// let archive = DiffArchive::new();
    /// assert_eq!(archive.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            diffs: HashMap::new(),
        }
    }

    /// Archives a diff.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, enterprise::{DiffArchive, RetentionPolicy}};
    /// use chrono::Utc;
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let mut archive = DiffArchive::new();
    /// archive.archive(diff_result, "admin", RetentionPolicy::Years(7), vec!["important".to_string()]);
    /// assert_eq!(archive.count(), 1);
    /// ```
    pub fn archive(
        &mut self,
        diff: StatuteDiff,
        archived_by: &str,
        retention_policy: RetentionPolicy,
        tags: Vec<String>,
    ) {
        let statute_id = diff.statute_id.clone();
        let archived = ArchivedDiff {
            diff,
            archived_at: Utc::now(),
            archived_by: archived_by.to_string(),
            retention_policy,
            tags,
            metadata: HashMap::new(),
        };

        self.diffs
            .entry(statute_id)
            .or_insert_with(Vec::new)
            .push(archived);
    }

    /// Gets all diffs for a statute.
    pub fn get_diffs(&self, statute_id: &str) -> Vec<&ArchivedDiff> {
        self.diffs
            .get(statute_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Searches for diffs by tag.
    pub fn search_by_tag(&self, tag: &str) -> Vec<&ArchivedDiff> {
        self.diffs
            .values()
            .flatten()
            .filter(|d| d.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Searches for diffs by date range.
    pub fn search_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&ArchivedDiff> {
        self.diffs
            .values()
            .flatten()
            .filter(|d| d.archived_at >= start && d.archived_at <= end)
            .collect()
    }

    /// Purges expired diffs based on retention policies.
    pub fn purge_expired(&mut self, current_time: DateTime<Utc>) -> usize {
        let mut purged = 0;

        for diffs in self.diffs.values_mut() {
            let original_len = diffs.len();
            diffs.retain(|d| {
                d.retention_policy
                    .should_retain(d.archived_at, current_time)
            });
            purged += original_len - diffs.len();
        }

        purged
    }

    /// Gets the total number of archived diffs.
    pub fn count(&self) -> usize {
        self.diffs.values().map(|v| v.len()).sum()
    }
}

/// Access control role for diff operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Administrator with full access.
    Admin,
    /// Can view and create diffs.
    Editor,
    /// Can only view diffs.
    Viewer,
    /// Custom role with specific permissions.
    Custom(String),
}

/// Permission for diff operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Can view diffs.
    ViewDiff,
    /// Can create diffs.
    CreateDiff,
    /// Can archive diffs.
    ArchiveDiff,
    /// Can delete diffs.
    DeleteDiff,
    /// Can modify retention policies.
    ModifyRetention,
    /// Can manage access control.
    ManageAccess,
}

impl Role {
    /// Checks if this role has a specific permission.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::enterprise::{Role, Permission};
    ///
    /// let admin = Role::Admin;
    /// assert!(admin.has_permission(Permission::ViewDiff));
    /// assert!(admin.has_permission(Permission::DeleteDiff));
    ///
    /// let viewer = Role::Viewer;
    /// assert!(viewer.has_permission(Permission::ViewDiff));
    /// assert!(!viewer.has_permission(Permission::DeleteDiff));
    /// ```
    pub fn has_permission(&self, permission: Permission) -> bool {
        match self {
            Role::Admin => true, // Admins have all permissions
            Role::Editor => matches!(
                permission,
                Permission::ViewDiff | Permission::CreateDiff | Permission::ArchiveDiff
            ),
            Role::Viewer => matches!(permission, Permission::ViewDiff),
            Role::Custom(_) => false, // Custom roles need explicit permission mapping
        }
    }
}

/// Access control list for diff operations.
#[derive(Debug, Clone, Default)]
pub struct AccessControl {
    /// User roles.
    user_roles: HashMap<String, Role>,
}

impl AccessControl {
    /// Creates a new access control list.
    pub fn new() -> Self {
        Self {
            user_roles: HashMap::new(),
        }
    }

    /// Assigns a role to a user.
    pub fn assign_role(&mut self, user: &str, role: Role) {
        self.user_roles.insert(user.to_string(), role);
    }

    /// Checks if a user has a specific permission.
    pub fn check_permission(&self, user: &str, permission: Permission) -> bool {
        self.user_roles
            .get(user)
            .map(|role| role.has_permission(permission))
            .unwrap_or(false)
    }

    /// Gets a user's role.
    pub fn get_role(&self, user: &str) -> Option<&Role> {
        self.user_roles.get(user)
    }
}

/// Audit trail entry for a diff operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// When the operation occurred.
    pub timestamp: DateTime<Utc>,
    /// Who performed the operation.
    pub user: String,
    /// The operation performed.
    pub operation: DiffOperation,
    /// The statute ID affected.
    pub statute_id: String,
    /// Optional additional details.
    pub details: Option<String>,
}

/// Types of diff operations that can be audited.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffOperation {
    /// A diff was created.
    Create,
    /// A diff was viewed.
    View,
    /// A diff was archived.
    Archive,
    /// A diff was deleted.
    Delete,
    /// A retention policy was modified.
    ModifyRetention,
    /// Access control was changed.
    AccessChange,
}

/// Audit trail for diff operations.
#[derive(Debug, Clone, Default)]
pub struct AuditTrail {
    /// Audit entries.
    entries: Vec<AuditEntry>,
}

impl AuditTrail {
    /// Creates a new audit trail.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::enterprise::AuditTrail;
    ///
    /// let trail = AuditTrail::new();
    /// assert_eq!(trail.entry_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Logs an operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::enterprise::{AuditTrail, DiffOperation};
    ///
    /// let mut trail = AuditTrail::new();
    /// trail.log("admin", DiffOperation::Create, "statute-123", None);
    /// assert_eq!(trail.entry_count(), 1);
    /// ```
    pub fn log(
        &mut self,
        user: &str,
        operation: DiffOperation,
        statute_id: &str,
        details: Option<String>,
    ) {
        self.entries.push(AuditEntry {
            timestamp: Utc::now(),
            user: user.to_string(),
            operation,
            statute_id: statute_id.to_string(),
            details,
        });
    }

    /// Gets all entries for a user.
    pub fn get_user_entries(&self, user: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.user == user).collect()
    }

    /// Gets all entries for a statute.
    pub fn get_statute_entries(&self, statute_id: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id == statute_id)
            .collect()
    }

    /// Gets entries within a date range.
    pub fn get_entries_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Gets the total number of audit entries.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

/// Analytics dashboard data for diffs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Total number of diffs.
    pub total_diffs: usize,
    /// Diffs by severity.
    pub by_severity: HashMap<String, usize>,
    /// Diffs by statute.
    pub by_statute: HashMap<String, usize>,
    /// Recent activity.
    pub recent_activity: Vec<DashboardActivity>,
    /// Top contributors.
    pub top_contributors: Vec<(String, usize)>,
}

/// Activity item for the dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardActivity {
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub operation: String,
    pub statute_id: String,
}

/// Generates dashboard analytics data.
///
/// # Examples
///
/// ```
/// use legalis_diff::enterprise::{DiffArchive, AuditTrail, generate_dashboard_data};
///
/// let archive = DiffArchive::new();
/// let trail = AuditTrail::new();
/// let data = generate_dashboard_data(&archive, &trail);
///
/// assert_eq!(data.total_diffs, 0);
/// ```
pub fn generate_dashboard_data(archive: &DiffArchive, trail: &AuditTrail) -> DashboardData {
    let total_diffs = archive.count();

    // Count diffs by severity
    let mut by_severity = HashMap::new();
    for diffs in archive.diffs.values() {
        for archived in diffs {
            let severity = format!("{:?}", archived.diff.impact.severity);
            *by_severity.entry(severity).or_insert(0) += 1;
        }
    }

    // Count diffs by statute
    let mut by_statute = HashMap::new();
    for (statute_id, diffs) in &archive.diffs {
        by_statute.insert(statute_id.clone(), diffs.len());
    }

    // Get recent activity (last 10 entries)
    let recent_activity: Vec<DashboardActivity> = trail
        .entries
        .iter()
        .rev()
        .take(10)
        .map(|e| DashboardActivity {
            timestamp: e.timestamp,
            user: e.user.clone(),
            operation: format!("{:?}", e.operation),
            statute_id: e.statute_id.clone(),
        })
        .collect();

    // Calculate top contributors
    let mut contributor_counts: HashMap<String, usize> = HashMap::new();
    for entry in &trail.entries {
        *contributor_counts.entry(entry.user.clone()).or_insert(0) += 1;
    }
    let mut top_contributors: Vec<(String, usize)> = contributor_counts.into_iter().collect();
    top_contributors.sort_by(|a, b| b.1.cmp(&a.1));
    top_contributors.truncate(5);

    DashboardData {
        total_diffs,
        by_severity,
        by_statute,
        recent_activity,
        top_contributors,
    }
}

/// Search query for finding diffs.
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    /// Search by statute ID.
    pub statute_id: Option<String>,
    /// Search by tags.
    pub tags: Vec<String>,
    /// Search by date range.
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Search by archived user.
    pub archived_by: Option<String>,
    /// Minimum severity level.
    pub min_severity: Option<crate::Severity>,
}

impl SearchQuery {
    /// Creates a new empty search query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Searches the archive with this query.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::enterprise::{DiffArchive, SearchQuery};
    ///
    /// let archive = DiffArchive::new();
    /// let query = SearchQuery::new().with_statute_id("law-123");
    /// let results = query.search(&archive);
    /// ```
    pub fn search<'a>(&self, archive: &'a DiffArchive) -> Vec<&'a ArchivedDiff> {
        let mut results: Vec<&ArchivedDiff> = archive.diffs.values().flatten().collect();

        // Filter by statute ID
        if let Some(ref id) = self.statute_id {
            results.retain(|d| &d.diff.statute_id == id);
        }

        // Filter by tags
        if !self.tags.is_empty() {
            results.retain(|d| self.tags.iter().any(|t| d.tags.contains(t)));
        }

        // Filter by date range
        if let Some((start, end)) = self.date_range {
            results.retain(|d| d.archived_at >= start && d.archived_at <= end);
        }

        // Filter by archived user
        if let Some(ref user) = self.archived_by {
            results.retain(|d| &d.archived_by == user);
        }

        // Filter by minimum severity
        if let Some(min_sev) = self.min_severity {
            results.retain(|d| d.diff.impact.severity >= min_sev);
        }

        results
    }

    /// Sets the statute ID filter.
    pub fn with_statute_id(mut self, statute_id: &str) -> Self {
        self.statute_id = Some(statute_id.to_string());
        self
    }

    /// Adds a tag filter.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Sets the date range filter.
    pub fn with_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.date_range = Some((start, end));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_statute(title: &str) -> Statute {
        Statute::new("test", title, Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_retention_policy_permanent() {
        let policy = RetentionPolicy::Permanent;
        let archived_at = Utc::now() - Duration::days(3650); // 10 years ago
        assert!(policy.should_retain(archived_at, Utc::now()));
    }

    #[test]
    fn test_retention_policy_years() {
        let policy = RetentionPolicy::Years(5);
        let archived_at = Utc::now() - Duration::days(365 * 3); // 3 years ago
        assert!(policy.should_retain(archived_at, Utc::now()));

        let old_archived = Utc::now() - Duration::days(365 * 6); // 6 years ago
        assert!(!policy.should_retain(old_archived, Utc::now()));
    }

    #[test]
    fn test_archive_and_retrieve() {
        let mut archive = DiffArchive::new();
        let old = test_statute("Old");
        let new = test_statute("New");
        let diff_result = diff(&old, &new).unwrap();

        archive.archive(
            diff_result,
            "admin",
            RetentionPolicy::Years(7),
            vec!["test".to_string()],
        );

        assert_eq!(archive.count(), 1);
        let diffs = archive.get_diffs("test");
        assert_eq!(diffs.len(), 1);
    }

    #[test]
    fn test_search_by_tag() {
        let mut archive = DiffArchive::new();
        let diff1 = diff(&test_statute("Old1"), &test_statute("New1")).unwrap();
        let diff2 = diff(&test_statute("Old2"), &test_statute("New2")).unwrap();

        archive.archive(
            diff1,
            "admin",
            RetentionPolicy::Permanent,
            vec!["important".to_string()],
        );
        archive.archive(
            diff2,
            "admin",
            RetentionPolicy::Permanent,
            vec!["other".to_string()],
        );

        let results = archive.search_by_tag("important");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_purge_expired() {
        let mut archive = DiffArchive::new();
        let diff1 = diff(&test_statute("Old1"), &test_statute("New1")).unwrap();

        archive.archive(diff1, "admin", RetentionPolicy::Years(1), vec![]);

        // Simulate time passing
        let future = Utc::now() + Duration::days(365 * 2);
        let purged = archive.purge_expired(future);

        assert_eq!(purged, 1);
        assert_eq!(archive.count(), 0);
    }

    #[test]
    fn test_role_permissions() {
        let admin = Role::Admin;
        assert!(admin.has_permission(Permission::ViewDiff));
        assert!(admin.has_permission(Permission::DeleteDiff));

        let editor = Role::Editor;
        assert!(editor.has_permission(Permission::ViewDiff));
        assert!(editor.has_permission(Permission::CreateDiff));
        assert!(!editor.has_permission(Permission::DeleteDiff));

        let viewer = Role::Viewer;
        assert!(viewer.has_permission(Permission::ViewDiff));
        assert!(!viewer.has_permission(Permission::CreateDiff));
    }

    #[test]
    fn test_access_control() {
        let mut acl = AccessControl::new();
        acl.assign_role("alice", Role::Admin);
        acl.assign_role("bob", Role::Viewer);

        assert!(acl.check_permission("alice", Permission::DeleteDiff));
        assert!(!acl.check_permission("bob", Permission::DeleteDiff));
        assert!(acl.check_permission("bob", Permission::ViewDiff));
    }

    #[test]
    fn test_audit_trail() {
        let mut trail = AuditTrail::new();
        trail.log("admin", DiffOperation::Create, "statute-1", None);
        trail.log("admin", DiffOperation::View, "statute-2", None);
        trail.log("user", DiffOperation::View, "statute-1", None);

        assert_eq!(trail.entry_count(), 3);

        let admin_entries = trail.get_user_entries("admin");
        assert_eq!(admin_entries.len(), 2);

        let statute1_entries = trail.get_statute_entries("statute-1");
        assert_eq!(statute1_entries.len(), 2);
    }

    #[test]
    fn test_dashboard_generation() {
        let archive = DiffArchive::new();
        let trail = AuditTrail::new();

        let data = generate_dashboard_data(&archive, &trail);
        assert_eq!(data.total_diffs, 0);
        assert!(data.recent_activity.is_empty());
    }

    #[test]
    fn test_search_query() {
        let mut archive = DiffArchive::new();
        let diff1 = diff(&test_statute("Old"), &test_statute("New")).unwrap();

        archive.archive(
            diff1,
            "admin",
            RetentionPolicy::Permanent,
            vec!["important".to_string()],
        );

        let query = SearchQuery::new()
            .with_statute_id("test")
            .with_tag("important");

        let results = query.search(&archive);
        assert_eq!(results.len(), 1);
    }
}
