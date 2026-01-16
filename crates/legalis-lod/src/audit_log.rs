//! Audit logging for knowledge graph access and operations.
//!
//! This module provides comprehensive audit logging:
//! - Access logging (who accessed what, when)
//! - Modification tracking (what changed, by whom)
//! - Query logging
//! - Compliance audit trails

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of audited action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditAction {
    /// Read operation
    Read,
    /// Write/create operation
    Write,
    /// Update operation
    Update,
    /// Delete operation
    Delete,
    /// SPARQL query execution
    Query,
    /// User login
    Login,
    /// User logout
    Logout,
    /// Permission change
    PermissionChange,
    /// Configuration change
    ConfigChange,
    /// Export operation
    Export,
    /// Import operation
    Import,
}

/// Audit event representing a logged action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event ID
    pub id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// User who performed the action
    pub user_id: String,
    /// Action type
    pub action: AuditAction,
    /// Resource URI affected
    pub resource_uri: Option<String>,
    /// Graph URI (if applicable)
    pub graph_uri: Option<String>,
    /// Success or failure
    pub success: bool,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Additional details
    pub details: HashMap<String, String>,
}

impl AuditEvent {
    /// Creates a new audit event.
    pub fn new(user_id: impl Into<String>, action: AuditAction) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user_id: user_id.into(),
            action,
            resource_uri: None,
            graph_uri: None,
            success: true,
            ip_address: None,
            user_agent: None,
            details: HashMap::new(),
        }
    }

    /// Sets the resource URI.
    pub fn with_resource(mut self, resource_uri: impl Into<String>) -> Self {
        self.resource_uri = Some(resource_uri.into());
        self
    }

    /// Sets the graph URI.
    pub fn with_graph(mut self, graph_uri: impl Into<String>) -> Self {
        self.graph_uri = Some(graph_uri.into());
        self
    }

    /// Sets success status.
    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Sets IP address.
    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    /// Adds a detail entry.
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

/// Filter for querying audit logs.
#[derive(Debug, Clone, Default)]
pub struct AuditFilter {
    /// Filter by user ID
    pub user_id: Option<String>,
    /// Filter by action type
    pub action: Option<AuditAction>,
    /// Filter by resource URI pattern
    pub resource_pattern: Option<String>,
    /// Start time
    pub start_time: Option<DateTime<Utc>>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Filter by success status
    pub success: Option<bool>,
}

impl AuditFilter {
    /// Creates a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters by user.
    pub fn for_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Filters by action.
    pub fn for_action(mut self, action: AuditAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Filters by time range.
    pub fn between(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    /// Checks if an event matches this filter.
    pub fn matches(&self, event: &AuditEvent) -> bool {
        if let Some(ref user_id) = self.user_id {
            if &event.user_id != user_id {
                return false;
            }
        }

        if let Some(action) = self.action {
            if event.action != action {
                return false;
            }
        }

        if let Some(ref start) = self.start_time {
            if event.timestamp < *start {
                return false;
            }
        }

        if let Some(ref end) = self.end_time {
            if event.timestamp > *end {
                return false;
            }
        }

        if let Some(success) = self.success {
            if event.success != success {
                return false;
            }
        }

        true
    }
}

/// Audit logger for tracking knowledge graph operations.
pub struct AuditLogger {
    /// All audit events
    events: Vec<AuditEvent>,
    /// Maximum number of events to keep in memory
    max_events: usize,
}

impl AuditLogger {
    /// Creates a new audit logger.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_events: 10_000,
        }
    }

    /// Sets the maximum number of events to keep.
    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    /// Logs an audit event.
    pub fn log(&mut self, event: AuditEvent) {
        self.events.push(event);

        // Trim old events if over limit
        if self.events.len() > self.max_events {
            let remove_count = self.events.len() - self.max_events;
            self.events.drain(0..remove_count);
        }
    }

    /// Queries events with a filter.
    pub fn query(&self, filter: &AuditFilter) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| filter.matches(e)).collect()
    }

    /// Gets all events.
    pub fn all_events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Gets events by user.
    pub fn events_by_user(&self, user_id: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.user_id == user_id)
            .collect()
    }

    /// Gets events by action type.
    pub fn events_by_action(&self, action: AuditAction) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.action == action).collect()
    }

    /// Gets failed operations.
    pub fn failed_operations(&self) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| !e.success).collect()
    }

    /// Generates a summary report.
    pub fn summary(&self) -> AuditSummary {
        let mut summary = AuditSummary {
            total_events: self.events.len(),
            ..Default::default()
        };

        for event in &self.events {
            *summary.actions_by_type.entry(event.action).or_insert(0) += 1;

            if event.success {
                summary.successful_operations += 1;
            } else {
                summary.failed_operations += 1;
            }

            *summary
                .events_by_user
                .entry(event.user_id.clone())
                .or_insert(0) += 1;
        }

        summary
    }

    /// Exports audit log to JSON.
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.events).map_err(|e| format!("JSON export failed: {}", e))
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary statistics for audit log.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditSummary {
    /// Total number of events
    pub total_events: usize,
    /// Successful operations
    pub successful_operations: usize,
    /// Failed operations
    pub failed_operations: usize,
    /// Events by action type
    pub actions_by_type: HashMap<AuditAction, usize>,
    /// Events by user
    pub events_by_user: HashMap<String, usize>,
}

impl AuditSummary {
    /// Gets the success rate.
    pub fn success_rate(&self) -> f64 {
        if self.total_events == 0 {
            0.0
        } else {
            self.successful_operations as f64 / self.total_events as f64
        }
    }

    /// Gets the most active user.
    pub fn most_active_user(&self) -> Option<(&String, &usize)> {
        self.events_by_user.iter().max_by_key(|(_, count)| *count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new("user1", AuditAction::Read)
            .with_resource("http://example.org/resource1")
            .with_graph("http://example.org/graph1")
            .with_success(true)
            .with_ip("192.168.1.1")
            .with_detail("query", "SELECT * WHERE { ?s ?p ?o }");

        assert_eq!(event.user_id, "user1");
        assert_eq!(event.action, AuditAction::Read);
        assert!(event.success);
        assert_eq!(
            event.details.get("query").map(|s| s.as_str()),
            Some("SELECT * WHERE { ?s ?p ?o }")
        );
    }

    #[test]
    fn test_audit_logger() {
        let mut logger = AuditLogger::new();

        let event1 = AuditEvent::new("user1", AuditAction::Read);
        let event2 = AuditEvent::new("user2", AuditAction::Write);

        logger.log(event1);
        logger.log(event2);

        assert_eq!(logger.all_events().len(), 2);
    }

    #[test]
    fn test_audit_filter() {
        let mut logger = AuditLogger::new();

        logger.log(AuditEvent::new("user1", AuditAction::Read));
        logger.log(AuditEvent::new("user1", AuditAction::Write));
        logger.log(AuditEvent::new("user2", AuditAction::Read));

        let filter = AuditFilter::new().for_user("user1");
        let results = logger.query(&filter);

        assert_eq!(results.len(), 2);

        let filter2 = AuditFilter::new().for_action(AuditAction::Read);
        let results2 = logger.query(&filter2);

        assert_eq!(results2.len(), 2);
    }

    #[test]
    fn test_events_by_user() {
        let mut logger = AuditLogger::new();

        logger.log(AuditEvent::new("user1", AuditAction::Read));
        logger.log(AuditEvent::new("user1", AuditAction::Write));
        logger.log(AuditEvent::new("user2", AuditAction::Read));

        let user1_events = logger.events_by_user("user1");
        assert_eq!(user1_events.len(), 2);
    }

    #[test]
    fn test_failed_operations() {
        let mut logger = AuditLogger::new();

        logger.log(AuditEvent::new("user1", AuditAction::Read).with_success(true));
        logger.log(AuditEvent::new("user1", AuditAction::Write).with_success(false));
        logger.log(AuditEvent::new("user2", AuditAction::Delete).with_success(false));

        let failed = logger.failed_operations();
        assert_eq!(failed.len(), 2);
    }

    #[test]
    fn test_audit_summary() {
        let mut logger = AuditLogger::new();

        logger.log(AuditEvent::new("user1", AuditAction::Read));
        logger.log(AuditEvent::new("user1", AuditAction::Write));
        logger.log(AuditEvent::new("user2", AuditAction::Read).with_success(false));

        let summary = logger.summary();

        assert_eq!(summary.total_events, 3);
        assert_eq!(summary.successful_operations, 2);
        assert_eq!(summary.failed_operations, 1);
        assert_eq!(*summary.actions_by_type.get(&AuditAction::Read).unwrap(), 2);
        assert_eq!(
            *summary.actions_by_type.get(&AuditAction::Write).unwrap(),
            1
        );
    }

    #[test]
    fn test_success_rate() {
        let mut logger = AuditLogger::new();

        logger.log(AuditEvent::new("user1", AuditAction::Read));
        logger.log(AuditEvent::new("user1", AuditAction::Write));
        logger.log(AuditEvent::new("user2", AuditAction::Read).with_success(false));

        let summary = logger.summary();
        let rate = summary.success_rate();

        assert!((rate - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_max_events_limit() {
        let mut logger = AuditLogger::new().with_max_events(5);

        for i in 0..10 {
            logger.log(AuditEvent::new(format!("user{}", i), AuditAction::Read));
        }

        assert_eq!(logger.all_events().len(), 5);
    }

    #[test]
    fn test_export_json() {
        let mut logger = AuditLogger::new();
        logger.log(AuditEvent::new("user1", AuditAction::Read));

        let json = logger.export_json().unwrap();
        assert!(json.contains("user1"));
        assert!(json.contains("Read"));
    }
}
