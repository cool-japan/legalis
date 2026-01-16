//! Deadline management for regulatory compliance.
//!
//! Tracks regulatory deadlines, sends reminders, and monitors
//! compliance submission timeliness.

use crate::AuditResult;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Deadline manager for regulatory compliance
pub struct DeadlineManager {
    deadlines: HashMap<Uuid, Deadline>,
    reminders: ReminderConfig,
    notifications: Vec<DeadlineNotification>,
}

impl DeadlineManager {
    /// Create a new deadline manager
    pub fn new() -> Self {
        Self {
            deadlines: HashMap::new(),
            reminders: ReminderConfig::default(),
            notifications: Vec::new(),
        }
    }

    /// Create with custom reminder configuration
    pub fn with_reminders(reminders: ReminderConfig) -> Self {
        Self {
            deadlines: HashMap::new(),
            reminders,
            notifications: Vec::new(),
        }
    }

    /// Add a deadline
    pub fn add_deadline(&mut self, deadline: Deadline) -> Uuid {
        let id = deadline.id;
        self.deadlines.insert(id, deadline);
        id
    }

    /// Get a deadline by ID
    pub fn get_deadline(&self, id: Uuid) -> Option<&Deadline> {
        self.deadlines.get(&id)
    }

    /// Get all deadlines
    pub fn all_deadlines(&self) -> Vec<&Deadline> {
        self.deadlines.values().collect()
    }

    /// Get upcoming deadlines
    pub fn get_upcoming_deadlines(&self, days: i64) -> Vec<&Deadline> {
        let cutoff = Utc::now() + Duration::days(days);
        self.deadlines
            .values()
            .filter(|d| {
                d.due_date <= cutoff
                    && d.status != DeadlineStatus::Completed
                    && d.status != DeadlineStatus::Cancelled
            })
            .collect()
    }

    /// Get overdue deadlines
    pub fn get_overdue_deadlines(&self) -> Vec<&Deadline> {
        let now = Utc::now();
        self.deadlines
            .values()
            .filter(|d| {
                d.due_date < now
                    && d.status != DeadlineStatus::Completed
                    && d.status != DeadlineStatus::Cancelled
            })
            .collect()
    }

    /// Mark deadline as completed
    pub fn complete_deadline(&mut self, id: Uuid) -> AuditResult<()> {
        let deadline = self
            .deadlines
            .get_mut(&id)
            .ok_or_else(|| crate::AuditError::RecordNotFound(id))?;

        deadline.status = DeadlineStatus::Completed;
        deadline.completed_at = Some(Utc::now());

        Ok(())
    }

    /// Cancel a deadline
    pub fn cancel_deadline(&mut self, id: Uuid) -> AuditResult<()> {
        let deadline = self
            .deadlines
            .get_mut(&id)
            .ok_or_else(|| crate::AuditError::RecordNotFound(id))?;

        deadline.status = DeadlineStatus::Cancelled;
        Ok(())
    }

    /// Check and generate reminder notifications
    pub fn check_reminders(&mut self) -> Vec<DeadlineNotification> {
        let now = Utc::now();
        let mut new_notifications = Vec::new();

        for deadline in self.deadlines.values() {
            if deadline.status == DeadlineStatus::Completed
                || deadline.status == DeadlineStatus::Cancelled
            {
                continue;
            }

            let days_until = (deadline.due_date - now).num_days();

            // Check if reminders should be sent
            for days_before in &self.reminders.remind_before_days {
                if days_until == *days_before as i64 {
                    let notification = DeadlineNotification {
                        id: Uuid::new_v4(),
                        deadline_id: deadline.id,
                        regulation: deadline.regulation.clone(),
                        message: format!(
                            "Reminder: {} deadline in {} days",
                            deadline.title, days_before
                        ),
                        severity: if *days_before <= 3 {
                            NotificationSeverity::Urgent
                        } else if *days_before <= 7 {
                            NotificationSeverity::Warning
                        } else {
                            NotificationSeverity::Info
                        },
                        created_at: now,
                        sent: false,
                    };
                    new_notifications.push(notification.clone());
                    self.notifications.push(notification);
                }
            }

            // Check for overdue
            if days_until < 0 && self.reminders.notify_overdue {
                let notification = DeadlineNotification {
                    id: Uuid::new_v4(),
                    deadline_id: deadline.id,
                    regulation: deadline.regulation.clone(),
                    message: format!(
                        "OVERDUE: {} deadline passed {} days ago",
                        deadline.title, -days_until
                    ),
                    severity: NotificationSeverity::Critical,
                    created_at: now,
                    sent: false,
                };
                new_notifications.push(notification.clone());
                self.notifications.push(notification);
            }
        }

        new_notifications
    }

    /// Mark notification as sent
    pub fn mark_notification_sent(&mut self, notification_id: Uuid) -> AuditResult<()> {
        let notification = self
            .notifications
            .iter_mut()
            .find(|n| n.id == notification_id)
            .ok_or_else(|| crate::AuditError::RecordNotFound(notification_id))?;

        notification.sent = true;
        Ok(())
    }

    /// Get pending notifications
    pub fn get_pending_notifications(&self) -> Vec<&DeadlineNotification> {
        self.notifications.iter().filter(|n| !n.sent).collect()
    }

    /// Get all notifications
    pub fn all_notifications(&self) -> &[DeadlineNotification] {
        &self.notifications
    }

    /// Get deadlines by regulation
    pub fn get_deadlines_by_regulation(&self, regulation: &str) -> Vec<&Deadline> {
        self.deadlines
            .values()
            .filter(|d| d.regulation == regulation)
            .collect()
    }

    /// Get deadline statistics
    pub fn get_statistics(&self) -> DeadlineStatistics {
        let total = self.deadlines.len();
        let completed = self
            .deadlines
            .values()
            .filter(|d| d.status == DeadlineStatus::Completed)
            .count();
        let pending = self
            .deadlines
            .values()
            .filter(|d| d.status == DeadlineStatus::Pending)
            .count();
        let overdue = self.get_overdue_deadlines().len();

        let on_time_rate = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        DeadlineStatistics {
            total_deadlines: total,
            completed,
            pending,
            overdue,
            on_time_rate,
        }
    }
}

impl Default for DeadlineManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Regulatory deadline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    pub id: Uuid,
    pub title: String,
    pub regulation: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub status: DeadlineStatus,
    pub priority: DeadlinePriority,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Deadline {
    /// Create a new deadline
    pub fn new(
        title: String,
        regulation: String,
        description: String,
        due_date: DateTime<Utc>,
        priority: DeadlinePriority,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            regulation,
            description,
            due_date,
            status: DeadlineStatus::Pending,
            priority,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Check if deadline is overdue
    pub fn is_overdue(&self) -> bool {
        Utc::now() > self.due_date
            && self.status != DeadlineStatus::Completed
            && self.status != DeadlineStatus::Cancelled
    }

    /// Get days until deadline
    pub fn days_until(&self) -> i64 {
        (self.due_date - Utc::now()).num_days()
    }
}

/// Deadline status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlineStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

/// Deadline priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlinePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Reminder configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderConfig {
    /// Days before deadline to send reminders
    pub remind_before_days: Vec<usize>,
    /// Whether to notify when overdue
    pub notify_overdue: bool,
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            remind_before_days: vec![30, 14, 7, 3, 1],
            notify_overdue: true,
        }
    }
}

/// Deadline notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineNotification {
    pub id: Uuid,
    pub deadline_id: Uuid,
    pub regulation: String,
    pub message: String,
    pub severity: NotificationSeverity,
    pub created_at: DateTime<Utc>,
    pub sent: bool,
}

/// Notification severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationSeverity {
    Critical,
    Urgent,
    Warning,
    Info,
}

/// Deadline statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineStatistics {
    pub total_deadlines: usize,
    pub completed: usize,
    pub pending: usize,
    pub overdue: usize,
    pub on_time_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadline_manager_creation() {
        let manager = DeadlineManager::new();
        assert!(manager.deadlines.is_empty());
        assert!(manager.notifications.is_empty());
    }

    #[test]
    fn test_add_deadline() {
        let mut manager = DeadlineManager::new();
        let deadline = Deadline::new(
            "GDPR Report".to_string(),
            "GDPR".to_string(),
            "Annual GDPR compliance report".to_string(),
            Utc::now() + Duration::days(30),
            DeadlinePriority::High,
        );

        let id = manager.add_deadline(deadline);
        assert!(manager.get_deadline(id).is_some());
    }

    #[test]
    fn test_get_upcoming_deadlines() {
        let mut manager = DeadlineManager::new();

        // Deadline in 10 days
        manager.add_deadline(Deadline::new(
            "Test 1".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(10),
            DeadlinePriority::Medium,
        ));

        // Deadline in 40 days
        manager.add_deadline(Deadline::new(
            "Test 2".to_string(),
            "SOX".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(40),
            DeadlinePriority::Low,
        ));

        let upcoming = manager.get_upcoming_deadlines(30);
        assert_eq!(upcoming.len(), 1);
        assert_eq!(upcoming[0].title, "Test 1");
    }

    #[test]
    fn test_get_overdue_deadlines() {
        let mut manager = DeadlineManager::new();

        // Past deadline
        manager.add_deadline(Deadline::new(
            "Overdue Test".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() - Duration::days(5),
            DeadlinePriority::High,
        ));

        // Future deadline
        manager.add_deadline(Deadline::new(
            "Future Test".to_string(),
            "SOX".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(5),
            DeadlinePriority::Low,
        ));

        let overdue = manager.get_overdue_deadlines();
        assert_eq!(overdue.len(), 1);
        assert_eq!(overdue[0].title, "Overdue Test");
    }

    #[test]
    fn test_complete_deadline() {
        let mut manager = DeadlineManager::new();
        let deadline = Deadline::new(
            "Test".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(10),
            DeadlinePriority::Medium,
        );

        let id = manager.add_deadline(deadline);
        manager.complete_deadline(id).unwrap();

        let deadline = manager.get_deadline(id).unwrap();
        assert_eq!(deadline.status, DeadlineStatus::Completed);
        assert!(deadline.completed_at.is_some());
    }

    #[test]
    fn test_cancel_deadline() {
        let mut manager = DeadlineManager::new();
        let deadline = Deadline::new(
            "Test".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(10),
            DeadlinePriority::Low,
        );

        let id = manager.add_deadline(deadline);
        manager.cancel_deadline(id).unwrap();

        let deadline = manager.get_deadline(id).unwrap();
        assert_eq!(deadline.status, DeadlineStatus::Cancelled);
    }

    #[test]
    fn test_deadline_is_overdue() {
        let past_deadline = Deadline::new(
            "Test".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() - Duration::days(1),
            DeadlinePriority::High,
        );

        assert!(past_deadline.is_overdue());

        let future_deadline = Deadline::new(
            "Test".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(1),
            DeadlinePriority::Low,
        );

        assert!(!future_deadline.is_overdue());
    }

    #[test]
    fn test_deadline_days_until() {
        let deadline = Deadline::new(
            "Test".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(10),
            DeadlinePriority::Medium,
        );

        let days = deadline.days_until();
        assert!((9..=10).contains(&days));
    }

    #[test]
    fn test_get_deadlines_by_regulation() {
        let mut manager = DeadlineManager::new();

        manager.add_deadline(Deadline::new(
            "GDPR 1".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(10),
            DeadlinePriority::High,
        ));

        manager.add_deadline(Deadline::new(
            "GDPR 2".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(20),
            DeadlinePriority::Medium,
        ));

        manager.add_deadline(Deadline::new(
            "SOX 1".to_string(),
            "SOX".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(15),
            DeadlinePriority::Low,
        ));

        let gdpr_deadlines = manager.get_deadlines_by_regulation("GDPR");
        assert_eq!(gdpr_deadlines.len(), 2);
    }

    #[test]
    fn test_get_statistics() {
        let mut manager = DeadlineManager::new();

        let deadline1 = Deadline::new(
            "Test 1".to_string(),
            "GDPR".to_string(),
            "Description".to_string(),
            Utc::now() + Duration::days(10),
            DeadlinePriority::High,
        );
        let id1 = manager.add_deadline(deadline1);

        manager.add_deadline(Deadline::new(
            "Test 2".to_string(),
            "SOX".to_string(),
            "Description".to_string(),
            Utc::now() - Duration::days(5),
            DeadlinePriority::Medium,
        ));

        manager.complete_deadline(id1).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_deadlines, 2);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.pending, 1); // The overdue deadline is still Pending status
        assert_eq!(stats.overdue, 1);
    }

    #[test]
    fn test_custom_reminder_config() {
        let config = ReminderConfig {
            remind_before_days: vec![7, 3, 1],
            notify_overdue: true,
        };

        let manager = DeadlineManager::with_reminders(config);
        assert_eq!(manager.reminders.remind_before_days.len(), 3);
    }
}
