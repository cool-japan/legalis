//! Audit logging for tracking all mutations and important operations.
//!
//! This module provides comprehensive audit logging for security and compliance:
//! - Tracks all create, update, delete operations
//! - Records user identity, timestamp, and operation details
//! - Supports querying audit logs with filtering
//! - Provides retention policies and log rotation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Audit event types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// Statute was created
    StatuteCreated,
    /// Statute was updated
    StatuteUpdated,
    /// Statute was deleted
    StatuteDeleted,
    /// Batch of statutes created
    BatchStatutesCreated,
    /// Batch of statutes deleted
    BatchStatutesDeleted,
    /// Statute version created
    StatuteVersionCreated,
    /// Verification performed
    VerificationExecuted,
    /// Simulation executed
    SimulationExecuted,
    /// Simulation saved
    SimulationSaved,
    /// Simulation deleted
    SimulationDeleted,
    /// Permission granted
    PermissionGranted,
    /// Permission revoked
    PermissionRevoked,
    /// API key created
    ApiKeyCreated,
    /// API key rotated
    ApiKeyRotated,
    /// API key revoked
    ApiKeyRevoked,
    /// User logged in
    UserLogin,
    /// User logged out
    UserLogout,
    /// Configuration changed
    ConfigurationChanged,
}

/// Audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique audit entry ID
    pub id: String,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Type of event
    pub event_type: AuditEventType,
    /// User who performed the action
    pub user_id: String,
    /// Username for display
    pub username: String,
    /// Resource ID affected (e.g., statute ID)
    pub resource_id: Option<String>,
    /// Resource type (e.g., "statute", "simulation")
    pub resource_type: Option<String>,
    /// Action performed
    pub action: String,
    /// Additional details (JSON)
    pub details: serde_json::Value,
    /// IP address of the requester
    pub ip_address: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Result of the operation (success, failure)
    pub result: AuditResult,
    /// Error message if operation failed
    pub error_message: Option<String>,
}

/// Result of an audited operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuditResult {
    Success,
    Failure,
    Partial,
}

/// Audit log store.
pub struct AuditLog {
    /// In-memory storage of audit entries (in production, use persistent storage)
    entries: Arc<RwLock<Vec<AuditEntry>>>,
    /// Maximum number of entries to keep in memory
    max_entries: usize,
}

impl AuditLog {
    /// Create a new audit log with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }

    /// Create a new audit log with specified capacity.
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            max_entries,
        }
    }

    /// Log an audit event.
    #[allow(clippy::too_many_arguments)]
    pub async fn log(
        &self,
        event_type: AuditEventType,
        user_id: String,
        username: String,
        action: String,
        resource_id: Option<String>,
        resource_type: Option<String>,
        details: serde_json::Value,
        result: AuditResult,
        error_message: Option<String>,
    ) {
        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: event_type.clone(),
            user_id: user_id.clone(),
            username: username.clone(),
            resource_id: resource_id.clone(),
            resource_type: resource_type.clone(),
            action: action.clone(),
            details,
            ip_address: None, // Would be extracted from request context
            user_agent: None, // Would be extracted from request headers
            result: result.clone(),
            error_message: error_message.clone(),
        };

        info!(
            event = ?event_type,
            user = %username,
            action = %action,
            result = ?result,
            resource = ?resource_id,
            "Audit event logged"
        );

        let mut entries = self.entries.write().await;

        // Add new entry
        entries.push(entry);

        // Enforce capacity limit (simple FIFO rotation)
        while entries.len() > self.max_entries {
            entries.remove(0);
        }
    }

    /// Log a successful operation.
    #[allow(clippy::too_many_arguments)]
    pub async fn log_success(
        &self,
        event_type: AuditEventType,
        user_id: String,
        username: String,
        action: String,
        resource_id: Option<String>,
        resource_type: Option<String>,
        details: serde_json::Value,
    ) {
        self.log(
            event_type,
            user_id,
            username,
            action,
            resource_id,
            resource_type,
            details,
            AuditResult::Success,
            None,
        )
        .await;
    }

    /// Log a failed operation.
    #[allow(clippy::too_many_arguments)]
    pub async fn log_failure(
        &self,
        event_type: AuditEventType,
        user_id: String,
        username: String,
        action: String,
        resource_id: Option<String>,
        resource_type: Option<String>,
        details: serde_json::Value,
        error: String,
    ) {
        self.log(
            event_type,
            user_id,
            username,
            action,
            resource_id,
            resource_type,
            details,
            AuditResult::Failure,
            Some(error),
        )
        .await;
    }

    /// Query audit logs with filtering.
    pub async fn query(&self, filter: AuditQueryFilter) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;

        let mut results: Vec<AuditEntry> = entries
            .iter()
            .filter(|entry| {
                // Filter by user
                if let Some(ref user_id) = filter.user_id {
                    if &entry.user_id != user_id {
                        return false;
                    }
                }

                // Filter by event type
                if let Some(ref event_type) = filter.event_type {
                    if &entry.event_type != event_type {
                        return false;
                    }
                }

                // Filter by resource type
                if let Some(ref resource_type) = filter.resource_type {
                    if entry.resource_type.as_ref() != Some(resource_type) {
                        return false;
                    }
                }

                // Filter by resource ID
                if let Some(ref resource_id) = filter.resource_id {
                    if entry.resource_id.as_ref() != Some(resource_id) {
                        return false;
                    }
                }

                // Filter by result
                if let Some(ref result) = filter.result {
                    if &entry.result != result {
                        return false;
                    }
                }

                // Filter by time range
                if let Some(start) = filter.start_time {
                    if entry.timestamp < start {
                        return false;
                    }
                }

                if let Some(end) = filter.end_time {
                    if entry.timestamp > end {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // Sort by timestamp (newest first)
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit and offset
        let offset = filter.offset.unwrap_or(0);
        let limit = filter.limit.unwrap_or(100).min(1000);

        results.into_iter().skip(offset).take(limit).collect()
    }

    /// Get total count of audit entries.
    pub async fn count(&self) -> usize {
        self.entries.read().await.len()
    }

    /// Get count matching a filter.
    pub async fn count_filtered(&self, filter: AuditQueryFilter) -> usize {
        let entries = self.entries.read().await;

        entries
            .iter()
            .filter(|entry| {
                if let Some(ref user_id) = filter.user_id {
                    if &entry.user_id != user_id {
                        return false;
                    }
                }

                if let Some(ref event_type) = filter.event_type {
                    if &entry.event_type != event_type {
                        return false;
                    }
                }

                if let Some(ref resource_type) = filter.resource_type {
                    if entry.resource_type.as_ref() != Some(resource_type) {
                        return false;
                    }
                }

                if let Some(ref resource_id) = filter.resource_id {
                    if entry.resource_id.as_ref() != Some(resource_id) {
                        return false;
                    }
                }

                if let Some(ref result) = filter.result {
                    if &entry.result != result {
                        return false;
                    }
                }

                if let Some(start) = filter.start_time {
                    if entry.timestamp < start {
                        return false;
                    }
                }

                if let Some(end) = filter.end_time {
                    if entry.timestamp > end {
                        return false;
                    }
                }

                true
            })
            .count()
    }

    /// Clear all audit logs (use with caution).
    #[allow(dead_code)]
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Query filter for audit logs.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuditQueryFilter {
    /// Filter by user ID
    pub user_id: Option<String>,
    /// Filter by event type
    pub event_type: Option<AuditEventType>,
    /// Filter by resource type
    pub resource_type: Option<String>,
    /// Filter by resource ID
    pub resource_id: Option<String>,
    /// Filter by result
    pub result: Option<AuditResult>,
    /// Start time for range filter
    pub start_time: Option<DateTime<Utc>>,
    /// End time for range filter
    pub end_time: Option<DateTime<Utc>>,
    /// Pagination offset
    pub offset: Option<usize>,
    /// Pagination limit
    pub limit: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_log_basic() {
        let audit = AuditLog::new();

        audit
            .log_success(
                AuditEventType::StatuteCreated,
                "user-123".to_string(),
                "alice".to_string(),
                "create_statute".to_string(),
                Some("statute-1".to_string()),
                Some("statute".to_string()),
                serde_json::json!({"title": "Test Statute"}),
            )
            .await;

        let count = audit.count().await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_audit_log_query_by_user() {
        let audit = AuditLog::new();

        audit
            .log_success(
                AuditEventType::StatuteCreated,
                "user-1".to_string(),
                "alice".to_string(),
                "create".to_string(),
                None,
                None,
                serde_json::json!({}),
            )
            .await;

        audit
            .log_success(
                AuditEventType::StatuteDeleted,
                "user-2".to_string(),
                "bob".to_string(),
                "delete".to_string(),
                None,
                None,
                serde_json::json!({}),
            )
            .await;

        let filter = AuditQueryFilter {
            user_id: Some("user-1".to_string()),
            ..Default::default()
        };

        let results = audit.query(filter).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].user_id, "user-1");
    }

    #[tokio::test]
    async fn test_audit_log_query_by_event_type() {
        let audit = AuditLog::new();

        audit
            .log_success(
                AuditEventType::StatuteCreated,
                "user-1".to_string(),
                "alice".to_string(),
                "create".to_string(),
                None,
                None,
                serde_json::json!({}),
            )
            .await;

        audit
            .log_success(
                AuditEventType::StatuteDeleted,
                "user-1".to_string(),
                "alice".to_string(),
                "delete".to_string(),
                None,
                None,
                serde_json::json!({}),
            )
            .await;

        let filter = AuditQueryFilter {
            event_type: Some(AuditEventType::StatuteCreated),
            ..Default::default()
        };

        let results = audit.query(filter).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].event_type, AuditEventType::StatuteCreated);
    }

    #[tokio::test]
    async fn test_audit_log_capacity_limit() {
        let audit = AuditLog::with_capacity(5);

        for i in 0..10 {
            audit
                .log_success(
                    AuditEventType::StatuteCreated,
                    format!("user-{}", i),
                    format!("user{}", i),
                    "create".to_string(),
                    None,
                    None,
                    serde_json::json!({}),
                )
                .await;
        }

        let count = audit.count().await;
        assert_eq!(count, 5); // Should only keep last 5 entries
    }

    #[tokio::test]
    async fn test_audit_log_failure() {
        let audit = AuditLog::new();

        audit
            .log_failure(
                AuditEventType::StatuteCreated,
                "user-1".to_string(),
                "alice".to_string(),
                "create".to_string(),
                None,
                None,
                serde_json::json!({}),
                "Duplicate ID".to_string(),
            )
            .await;

        let filter = AuditQueryFilter {
            result: Some(AuditResult::Failure),
            ..Default::default()
        };

        let results = audit.query(filter).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].result, AuditResult::Failure);
        assert_eq!(results[0].error_message, Some("Duplicate ID".to_string()));
    }
}
