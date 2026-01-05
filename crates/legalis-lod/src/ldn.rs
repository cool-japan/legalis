//! Linked Data Notifications (LDN) support.
//!
//! This module implements the W3C Linked Data Notifications specification
//! for sending and receiving notifications about RDF dataset changes.

use crate::{RdfValue, Triple};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Notification types as per Activity Streams 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Resource was created
    Create,
    /// Resource was updated
    Update,
    /// Resource was deleted
    Delete,
    /// Resource was announced
    Announce,
    /// Custom notification type
    Custom(String),
}

impl NotificationType {
    /// Converts to Activity Streams URI.
    pub fn to_as_uri(&self) -> String {
        match self {
            NotificationType::Create => "https://www.w3.org/ns/activitystreams#Create".to_string(),
            NotificationType::Update => "https://www.w3.org/ns/activitystreams#Update".to_string(),
            NotificationType::Delete => "https://www.w3.org/ns/activitystreams#Delete".to_string(),
            NotificationType::Announce => {
                "https://www.w3.org/ns/activitystreams#Announce".to_string()
            }
            NotificationType::Custom(uri) => uri.clone(),
        }
    }
}

/// Linked Data Notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification identifier
    pub id: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Actor (who triggered the notification)
    pub actor: Option<String>,
    /// Object (what the notification is about)
    pub object: String,
    /// Target (where the notification is sent)
    pub target: Option<String>,
    /// Published timestamp
    pub published: DateTime<Utc>,
    /// Summary/description
    pub summary: Option<String>,
    /// Additional context
    pub context: Option<String>,
}

impl Notification {
    /// Creates a new notification.
    pub fn new(
        id: impl Into<String>,
        notification_type: NotificationType,
        object: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            notification_type,
            actor: None,
            object: object.into(),
            target: None,
            published: Utc::now(),
            summary: None,
            context: None,
        }
    }

    /// Sets the actor.
    pub fn with_actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }

    /// Sets the target.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Sets the summary.
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Sets the context.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Converts the notification to RDF triples (JSON-LD compatible).
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: self.id.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("https://www.w3.org/ns/activitystreams#Activity".to_string()),
        });

        // Specific type
        triples.push(Triple {
            subject: self.id.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(self.notification_type.to_as_uri()),
        });

        // Actor
        if let Some(ref actor) = self.actor {
            triples.push(Triple {
                subject: self.id.clone(),
                predicate: "https://www.w3.org/ns/activitystreams#actor".to_string(),
                object: RdfValue::Uri(actor.clone()),
            });
        }

        // Object
        triples.push(Triple {
            subject: self.id.clone(),
            predicate: "https://www.w3.org/ns/activitystreams#object".to_string(),
            object: RdfValue::Uri(self.object.clone()),
        });

        // Target
        if let Some(ref target) = self.target {
            triples.push(Triple {
                subject: self.id.clone(),
                predicate: "https://www.w3.org/ns/activitystreams#target".to_string(),
                object: RdfValue::Uri(target.clone()),
            });
        }

        // Published
        triples.push(Triple {
            subject: self.id.clone(),
            predicate: "https://www.w3.org/ns/activitystreams#published".to_string(),
            object: RdfValue::datetime(self.published),
        });

        // Summary
        if let Some(ref summary) = self.summary {
            triples.push(Triple {
                subject: self.id.clone(),
                predicate: "https://www.w3.org/ns/activitystreams#summary".to_string(),
                object: RdfValue::string(summary),
            });
        }

        // Context
        if let Some(ref context) = self.context {
            triples.push(Triple {
                subject: self.id.clone(),
                predicate: "https://www.w3.org/ns/activitystreams#context".to_string(),
                object: RdfValue::Uri(context.clone()),
            });
        }

        triples
    }

    /// Serializes to JSON-LD format.
    pub fn to_json_ld(&self) -> serde_json::Value {
        let mut obj = serde_json::json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "id": self.id,
            "type": match &self.notification_type {
                NotificationType::Create => "Create",
                NotificationType::Update => "Update",
                NotificationType::Delete => "Delete",
                NotificationType::Announce => "Announce",
                NotificationType::Custom(t) => t.as_str(),
            },
            "object": self.object,
            "published": self.published.to_rfc3339(),
        });

        if let Some(ref actor) = self.actor {
            obj["actor"] = serde_json::json!(actor);
        }

        if let Some(ref target) = self.target {
            obj["target"] = serde_json::json!(target);
        }

        if let Some(ref summary) = self.summary {
            obj["summary"] = serde_json::json!(summary);
        }

        if let Some(ref context) = self.context {
            obj["context"] = serde_json::json!(context);
        }

        obj
    }
}

/// Notification inbox for receiving notifications.
#[derive(Debug)]
pub struct NotificationInbox {
    /// Inbox URI
    pub uri: String,
    /// Received notifications
    notifications: Vec<Notification>,
}

impl NotificationInbox {
    /// Creates a new notification inbox.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            notifications: Vec::new(),
        }
    }

    /// Receives a notification.
    pub fn receive(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    /// Returns all notifications.
    pub fn notifications(&self) -> &[Notification] {
        &self.notifications
    }

    /// Returns notifications of a specific type.
    pub fn notifications_by_type(
        &self,
        notification_type: &NotificationType,
    ) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| &n.notification_type == notification_type)
            .collect()
    }

    /// Returns notifications about a specific object.
    pub fn notifications_for_object(&self, object: &str) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| n.object == object)
            .collect()
    }

    /// Clears all notifications.
    pub fn clear(&mut self) {
        self.notifications.clear();
    }
}

/// Notification sender for publishing notifications.
#[derive(Debug)]
pub struct NotificationSender {
    /// Default actor for notifications
    default_actor: Option<String>,
}

impl Default for NotificationSender {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationSender {
    /// Creates a new notification sender.
    pub fn new() -> Self {
        Self {
            default_actor: None,
        }
    }

    /// Sets the default actor.
    pub fn with_default_actor(mut self, actor: impl Into<String>) -> Self {
        self.default_actor = Some(actor.into());
        self
    }

    /// Creates a notification for a resource creation.
    pub fn notify_create(&self, id: impl Into<String>, object: impl Into<String>) -> Notification {
        let mut notification = Notification::new(id, NotificationType::Create, object);
        if let Some(ref actor) = self.default_actor {
            notification = notification.with_actor(actor);
        }
        notification
    }

    /// Creates a notification for a resource update.
    pub fn notify_update(&self, id: impl Into<String>, object: impl Into<String>) -> Notification {
        let mut notification = Notification::new(id, NotificationType::Update, object);
        if let Some(ref actor) = self.default_actor {
            notification = notification.with_actor(actor);
        }
        notification
    }

    /// Creates a notification for a resource deletion.
    pub fn notify_delete(&self, id: impl Into<String>, object: impl Into<String>) -> Notification {
        let mut notification = Notification::new(id, NotificationType::Delete, object);
        if let Some(ref actor) = self.default_actor {
            notification = notification.with_actor(actor);
        }
        notification
    }

    /// Creates a notification for an announcement.
    pub fn notify_announce(
        &self,
        id: impl Into<String>,
        object: impl Into<String>,
    ) -> Notification {
        let mut notification = Notification::new(id, NotificationType::Announce, object);
        if let Some(ref actor) = self.default_actor {
            notification = notification.with_actor(actor);
        }
        notification
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_notification() {
        let notification = Notification::new(
            "https://example.org/notification/1",
            NotificationType::Create,
            "https://example.org/statute/123",
        )
        .with_actor("https://example.org/agent/admin")
        .with_summary("New statute created");

        assert_eq!(notification.id, "https://example.org/notification/1");
        assert_eq!(notification.notification_type, NotificationType::Create);
        assert_eq!(notification.object, "https://example.org/statute/123");
        assert!(notification.actor.is_some());
        assert!(notification.summary.is_some());
    }

    #[test]
    fn test_notification_to_triples() {
        let notification = Notification::new(
            "https://example.org/notification/1",
            NotificationType::Update,
            "https://example.org/statute/123",
        );

        let triples = notification.to_triples();
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
    }

    #[test]
    fn test_notification_to_json_ld() {
        let notification = Notification::new(
            "https://example.org/notification/1",
            NotificationType::Create,
            "https://example.org/statute/123",
        );

        let json = notification.to_json_ld();
        assert_eq!(json["id"], "https://example.org/notification/1");
        assert_eq!(json["type"], "Create");
        assert_eq!(json["object"], "https://example.org/statute/123");
    }

    #[test]
    fn test_notification_inbox() {
        let mut inbox = NotificationInbox::new("https://example.org/inbox");

        let notification = Notification::new(
            "https://example.org/notification/1",
            NotificationType::Create,
            "https://example.org/statute/123",
        );

        inbox.receive(notification);
        assert_eq!(inbox.notifications().len(), 1);
    }

    #[test]
    fn test_inbox_filter_by_type() {
        let mut inbox = NotificationInbox::new("https://example.org/inbox");

        inbox.receive(Notification::new("n1", NotificationType::Create, "obj1"));
        inbox.receive(Notification::new("n2", NotificationType::Update, "obj2"));
        inbox.receive(Notification::new("n3", NotificationType::Create, "obj3"));

        let creates = inbox.notifications_by_type(&NotificationType::Create);
        assert_eq!(creates.len(), 2);
    }

    #[test]
    fn test_inbox_filter_by_object() {
        let mut inbox = NotificationInbox::new("https://example.org/inbox");

        inbox.receive(Notification::new("n1", NotificationType::Create, "obj1"));
        inbox.receive(Notification::new("n2", NotificationType::Update, "obj1"));
        inbox.receive(Notification::new("n3", NotificationType::Create, "obj2"));

        let obj1_notifications = inbox.notifications_for_object("obj1");
        assert_eq!(obj1_notifications.len(), 2);
    }

    #[test]
    fn test_notification_sender() {
        let sender =
            NotificationSender::new().with_default_actor("https://example.org/agent/system");

        let notification = sender.notify_create("n1", "obj1");
        assert_eq!(notification.notification_type, NotificationType::Create);
        assert_eq!(
            notification.actor,
            Some("https://example.org/agent/system".to_string())
        );
    }

    #[test]
    fn test_notification_types() {
        let sender = NotificationSender::new();
        let create = sender.notify_create("n1", "obj1");
        let update = sender.notify_update("n2", "obj2");
        let delete = sender.notify_delete("n3", "obj3");
        let announce = sender.notify_announce("n4", "obj4");

        assert_eq!(create.notification_type, NotificationType::Create);
        assert_eq!(update.notification_type, NotificationType::Update);
        assert_eq!(delete.notification_type, NotificationType::Delete);
        assert_eq!(announce.notification_type, NotificationType::Announce);
    }

    #[test]
    fn test_inbox_clear() {
        let mut inbox = NotificationInbox::new("https://example.org/inbox");

        inbox.receive(Notification::new("n1", NotificationType::Create, "obj1"));
        inbox.receive(Notification::new("n2", NotificationType::Update, "obj2"));

        assert_eq!(inbox.notifications().len(), 2);

        inbox.clear();
        assert_eq!(inbox.notifications().len(), 0);
    }
}
