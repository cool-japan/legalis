use super::*;

/// Notification type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Approval request submitted
    ApprovalRequested,
    /// Approval granted
    ApprovalGranted,
    /// Approval rejected
    ApprovalRejected,
    /// Task assigned
    TaskAssigned,
    /// Task completed
    TaskCompleted,
    /// SLA warning
    SlaWarning,
    /// SLA breach
    SlaBreach,
    /// Statute updated
    StatuteUpdated,
    /// Custom notification
    Custom(String),
}

/// Notification priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NotificationPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Notification channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notification
    Email,
    /// SMS notification
    Sms,
    /// In-app notification
    InApp,
    /// Webhook notification
    Webhook { url: String },
    /// Custom channel
    Custom(String),
}

/// A notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub notification_id: Uuid,
    /// Recipient user ID
    pub recipient: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Priority
    pub priority: NotificationPriority,
    /// Title
    pub title: String,
    /// Message
    pub message: String,
    /// Related entity ID (e.g., request ID, statute ID)
    pub related_entity_id: Option<String>,
    /// Channels to send through
    pub channels: Vec<NotificationChannel>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Sent timestamp
    pub sent_at: Option<DateTime<Utc>>,
    /// Read timestamp
    pub read_at: Option<DateTime<Utc>>,
}

impl Notification {
    /// Creates a new notification.
    pub fn new(
        recipient: impl Into<String>,
        notification_type: NotificationType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            notification_id: Uuid::new_v4(),
            recipient: recipient.into(),
            notification_type,
            priority: NotificationPriority::Normal,
            title: title.into(),
            message: message.into(),
            related_entity_id: None,
            channels: vec![NotificationChannel::InApp],
            created_at: Utc::now(),
            sent_at: None,
            read_at: None,
        }
    }

    /// Sets priority.
    pub fn with_priority(mut self, priority: NotificationPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Sets related entity ID.
    pub fn with_related_entity(mut self, entity_id: impl Into<String>) -> Self {
        self.related_entity_id = Some(entity_id.into());
        self
    }

    /// Adds a channel.
    pub fn with_channel(mut self, channel: NotificationChannel) -> Self {
        self.channels.push(channel);
        self
    }

    /// Marks as sent.
    pub fn mark_sent(&mut self) {
        self.sent_at = Some(Utc::now());
    }

    /// Marks as read.
    pub fn mark_read(&mut self) {
        self.read_at = Some(Utc::now());
    }

    /// Checks if sent.
    pub fn is_sent(&self) -> bool {
        self.sent_at.is_some()
    }

    /// Checks if read.
    pub fn is_read(&self) -> bool {
        self.read_at.is_some()
    }
}

/// Notification manager.
#[derive(Debug)]
pub struct NotificationManager {
    notifications: Vec<Notification>,
    max_notifications: usize,
}

impl NotificationManager {
    /// Creates a new notification manager.
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            max_notifications: 10000,
        }
    }

    /// Sends a notification.
    pub fn send(&mut self, mut notification: Notification) {
        notification.mark_sent();
        self.notifications.push(notification);

        // Rotate if needed
        if self.notifications.len() > self.max_notifications {
            self.notifications
                .drain(0..self.notifications.len() - self.max_notifications);
        }
    }

    /// Gets unread notifications for a user.
    pub fn unread_for_user(&self, user_id: &str) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| n.recipient == user_id && !n.is_read())
            .collect()
    }

    /// Marks a notification as read.
    pub fn mark_as_read(&mut self, notification_id: Uuid) -> bool {
        if let Some(notification) = self
            .notifications
            .iter_mut()
            .find(|n| n.notification_id == notification_id)
        {
            notification.mark_read();
            true
        } else {
            false
        }
    }

    /// Gets all notifications for a user.
    pub fn for_user(&self, user_id: &str) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| n.recipient == user_id)
            .collect()
    }

    /// Gets notifications by priority.
    pub fn by_priority(&self, min_priority: NotificationPriority) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| n.priority >= min_priority)
            .collect()
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}
