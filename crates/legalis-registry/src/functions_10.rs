//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Real-Time Collaboration (v0.2.8)
///
/// This module provides real-time collaboration features for multi-user statute editing:
/// - WebSocket-based live updates
/// - Collaborative editing locks
/// - Real-time conflict notifications
/// - Presence indicators
/// - Change stream subscriptions
pub mod realtime {
    use super::*;
    use std::time::Duration;
    /// WebSocket message type for live updates.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum WebSocketMessage {
        /// Statute was created
        StatuteCreated {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was updated
        StatuteUpdated {
            statute_id: String,
            version: u32,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was deleted
        StatuteDeleted {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was locked for editing
        StatuteLocked {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute lock was released
        StatuteUnlocked {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// User presence update
        PresenceUpdate {
            user_id: String,
            status: PresenceStatus,
            timestamp: DateTime<Utc>,
        },
        /// Conflict notification
        ConflictDetected {
            statute_id: String,
            conflict_type: String,
            description: String,
            timestamp: DateTime<Utc>,
        },
        /// Heartbeat/ping message
        Ping,
        /// Heartbeat/pong response
        Pong,
    }
    impl WebSocketMessage {
        /// Get the statute ID associated with this message, if any.
        pub fn statute_id(&self) -> Option<&str> {
            match self {
                WebSocketMessage::StatuteCreated { statute_id, .. }
                | WebSocketMessage::StatuteUpdated { statute_id, .. }
                | WebSocketMessage::StatuteDeleted { statute_id, .. }
                | WebSocketMessage::StatuteLocked { statute_id, .. }
                | WebSocketMessage::StatuteUnlocked { statute_id, .. }
                | WebSocketMessage::ConflictDetected { statute_id, .. } => Some(statute_id),
                _ => None,
            }
        }
        /// Get the actor associated with this message, if any.
        pub fn actor(&self) -> Option<&str> {
            match self {
                WebSocketMessage::StatuteCreated { actor, .. }
                | WebSocketMessage::StatuteUpdated { actor, .. }
                | WebSocketMessage::StatuteDeleted { actor, .. }
                | WebSocketMessage::StatuteLocked { actor, .. }
                | WebSocketMessage::StatuteUnlocked { actor, .. } => Some(actor),
                _ => None,
            }
        }
        /// Get the timestamp of this message.
        pub fn timestamp(&self) -> Option<DateTime<Utc>> {
            match self {
                WebSocketMessage::StatuteCreated { timestamp, .. }
                | WebSocketMessage::StatuteUpdated { timestamp, .. }
                | WebSocketMessage::StatuteDeleted { timestamp, .. }
                | WebSocketMessage::StatuteLocked { timestamp, .. }
                | WebSocketMessage::StatuteUnlocked { timestamp, .. }
                | WebSocketMessage::PresenceUpdate { timestamp, .. }
                | WebSocketMessage::ConflictDetected { timestamp, .. } => Some(*timestamp),
                _ => None,
            }
        }
    }
    /// User presence status.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum PresenceStatus {
        /// User is online and active
        Online,
        /// User is idle
        Idle,
        /// User is away
        Away,
        /// User is offline
        Offline,
    }
    /// User presence information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserPresence {
        /// User ID
        pub user_id: String,
        /// Current status
        pub status: PresenceStatus,
        /// Currently viewing/editing statute ID (if any)
        pub current_statute: Option<String>,
        /// Last activity timestamp
        pub last_activity: DateTime<Utc>,
        /// User display name
        pub display_name: Option<String>,
    }
    impl UserPresence {
        /// Create a new user presence entry.
        pub fn new(user_id: String) -> Self {
            Self {
                user_id,
                status: PresenceStatus::Online,
                current_statute: None,
                last_activity: Utc::now(),
                display_name: None,
            }
        }
        /// Set the display name.
        pub fn with_display_name(mut self, name: String) -> Self {
            self.display_name = Some(name);
            self
        }
        /// Set the current statute being viewed/edited.
        pub fn with_current_statute(mut self, statute_id: String) -> Self {
            self.current_statute = Some(statute_id);
            self
        }
        /// Update the user's status.
        pub fn set_status(&mut self, status: PresenceStatus) {
            self.status = status;
            self.last_activity = Utc::now();
        }
        /// Update the currently viewed/edited statute.
        pub fn set_current_statute(&mut self, statute_id: Option<String>) {
            self.current_statute = statute_id;
            self.last_activity = Utc::now();
        }
        /// Update last activity timestamp.
        pub fn touch(&mut self) {
            self.last_activity = Utc::now();
        }
        /// Check if user is inactive for a given duration.
        pub fn is_inactive(&self, threshold: Duration) -> bool {
            let elapsed = Utc::now()
                .signed_duration_since(self.last_activity)
                .to_std()
                .unwrap_or(Duration::ZERO);
            elapsed > threshold
        }
    }
    /// Presence tracker for managing user presence.
    pub struct PresenceTracker {
        presences: Arc<Mutex<HashMap<String, UserPresence>>>,
        inactive_threshold: Duration,
    }
    impl PresenceTracker {
        /// Create a new presence tracker.
        pub fn new() -> Self {
            Self {
                presences: Arc::new(Mutex::new(HashMap::new())),
                inactive_threshold: Duration::from_secs(300),
            }
        }
        /// Set the inactive threshold duration.
        pub fn with_inactive_threshold(mut self, threshold: Duration) -> Self {
            self.inactive_threshold = threshold;
            self
        }
        /// Register or update a user's presence.
        pub fn update_presence(&self, presence: UserPresence) {
            let mut presences = self.presences.lock().expect("presences mutex poisoned");
            presences.insert(presence.user_id.clone(), presence);
        }
        /// Get a user's presence.
        pub fn get_presence(&self, user_id: &str) -> Option<UserPresence> {
            let presences = self.presences.lock().expect("presences mutex poisoned");
            presences.get(user_id).cloned()
        }
        /// Get all online users.
        pub fn get_online_users(&self) -> Vec<UserPresence> {
            let presences = self.presences.lock().expect("presences mutex poisoned");
            presences
                .values()
                .filter(|p| p.status == PresenceStatus::Online)
                .cloned()
                .collect()
        }
        /// Get users viewing/editing a specific statute.
        pub fn get_users_for_statute(&self, statute_id: &str) -> Vec<UserPresence> {
            let presences = self.presences.lock().expect("presences mutex poisoned");
            presences
                .values()
                .filter(|p| {
                    p.current_statute
                        .as_ref()
                        .is_some_and(|sid| sid == statute_id)
                })
                .cloned()
                .collect()
        }
        /// Remove inactive users and mark them as offline.
        pub fn cleanup_inactive(&self) -> Vec<String> {
            let mut presences = self.presences.lock().expect("presences mutex poisoned");
            let mut inactive_users = Vec::new();
            for (user_id, presence) in presences.iter_mut() {
                if presence.is_inactive(self.inactive_threshold)
                    && presence.status != PresenceStatus::Offline
                {
                    presence.status = PresenceStatus::Offline;
                    inactive_users.push(user_id.clone());
                }
            }
            inactive_users
        }
        /// Remove a user's presence.
        pub fn remove_user(&self, user_id: &str) {
            let mut presences = self.presences.lock().expect("presences mutex poisoned");
            presences.remove(user_id);
        }
        /// Count online users.
        pub fn online_count(&self) -> usize {
            let presences = self.presences.lock().expect("presences mutex poisoned");
            presences
                .values()
                .filter(|p| p.status == PresenceStatus::Online)
                .count()
        }
    }
    impl Default for PresenceTracker {
        fn default() -> Self {
            Self::new()
        }
    }
    /// Editing lock for collaborative editing.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EditingLock {
        /// Statute ID being locked
        pub statute_id: String,
        /// User holding the lock
        pub holder: String,
        /// When the lock was acquired
        pub acquired_at: DateTime<Utc>,
        /// When the lock expires
        pub expires_at: DateTime<Utc>,
        /// Lock type
        pub lock_type: LockType,
    }
    /// Type of editing lock.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum LockType {
        /// Exclusive write lock
        Exclusive,
        /// Shared read lock
        Shared,
    }
    impl EditingLock {
        /// Create a new exclusive lock.
        pub fn exclusive(statute_id: String, holder: String, duration: Duration) -> Self {
            let acquired_at = Utc::now();
            let expires_at = acquired_at
                + chrono::Duration::from_std(duration)
                    .expect("duration is within chrono representable range");
            Self {
                statute_id,
                holder,
                acquired_at,
                expires_at,
                lock_type: LockType::Exclusive,
            }
        }
        /// Create a new shared lock.
        pub fn shared(statute_id: String, holder: String, duration: Duration) -> Self {
            let acquired_at = Utc::now();
            let expires_at = acquired_at
                + chrono::Duration::from_std(duration)
                    .expect("duration is within chrono representable range");
            Self {
                statute_id,
                holder,
                acquired_at,
                expires_at,
                lock_type: LockType::Shared,
            }
        }
        /// Check if the lock has expired.
        pub fn is_expired(&self) -> bool {
            Utc::now() > self.expires_at
        }
        /// Extend the lock by a duration.
        pub fn extend(&mut self, duration: Duration) {
            self.expires_at += chrono::Duration::from_std(duration)
                .expect("duration is within chrono representable range");
        }
        /// Get remaining time until expiration.
        pub fn time_remaining(&self) -> Option<Duration> {
            if self.is_expired() {
                None
            } else {
                self.expires_at
                    .signed_duration_since(Utc::now())
                    .to_std()
                    .ok()
            }
        }
    }
    /// Lock manager for collaborative editing locks.
    pub struct LockManager {
        locks: Arc<Mutex<HashMap<String, Vec<EditingLock>>>>,
        default_duration: Duration,
    }
    impl LockManager {
        /// Create a new lock manager.
        pub fn new() -> Self {
            Self {
                locks: Arc::new(Mutex::new(HashMap::new())),
                default_duration: Duration::from_secs(300),
            }
        }
        /// Set the default lock duration.
        pub fn with_default_duration(mut self, duration: Duration) -> Self {
            self.default_duration = duration;
            self
        }
        /// Acquire an exclusive lock on a statute.
        pub fn acquire_exclusive(
            &self,
            statute_id: String,
            holder: String,
        ) -> Result<EditingLock, String> {
            let mut locks = self.locks.lock().expect("locks mutex poisoned");
            if let Some(existing_locks) = locks.get(&statute_id) {
                for lock in existing_locks {
                    if !lock.is_expired() {
                        return Err(format!(
                            "Statute {} is locked by {}",
                            statute_id, lock.holder
                        ));
                    }
                }
            }
            let lock = EditingLock::exclusive(statute_id.clone(), holder, self.default_duration);
            locks.insert(statute_id, vec![lock.clone()]);
            Ok(lock)
        }
        /// Acquire a shared lock on a statute.
        pub fn acquire_shared(
            &self,
            statute_id: String,
            holder: String,
        ) -> Result<EditingLock, String> {
            let mut locks = self.locks.lock().expect("locks mutex poisoned");
            if let Some(existing_locks) = locks.get(&statute_id) {
                for lock in existing_locks {
                    if !lock.is_expired() && lock.lock_type == LockType::Exclusive {
                        return Err(format!(
                            "Statute {} has exclusive lock by {}",
                            statute_id, lock.holder
                        ));
                    }
                }
            }
            let lock = EditingLock::shared(statute_id.clone(), holder, self.default_duration);
            locks.entry(statute_id).or_default().push(lock.clone());
            Ok(lock)
        }
        /// Release a lock.
        pub fn release(&self, statute_id: &str, holder: &str) -> bool {
            let mut locks = self.locks.lock().expect("locks mutex poisoned");
            if let Some(statute_locks) = locks.get_mut(statute_id) {
                let initial_len = statute_locks.len();
                statute_locks.retain(|lock| lock.holder != holder);
                let new_len = statute_locks.len();
                if statute_locks.is_empty() {
                    locks.remove(statute_id);
                }
                return new_len < initial_len;
            }
            false
        }
        /// Get all locks for a statute.
        pub fn get_locks(&self, statute_id: &str) -> Vec<EditingLock> {
            let locks = self.locks.lock().expect("locks mutex poisoned");
            locks
                .get(statute_id)
                .map(|v| v.iter().filter(|l| !l.is_expired()).cloned().collect())
                .unwrap_or_default()
        }
        /// Check if a statute is locked.
        pub fn is_locked(&self, statute_id: &str) -> bool {
            let locks = self.locks.lock().expect("locks mutex poisoned");
            locks
                .get(statute_id)
                .is_some_and(|v| v.iter().any(|lock| !lock.is_expired()))
        }
        /// Extend a lock.
        pub fn extend_lock(&self, statute_id: &str, holder: &str, duration: Duration) -> bool {
            let mut locks = self.locks.lock().expect("locks mutex poisoned");
            if let Some(statute_locks) = locks.get_mut(statute_id) {
                for lock in statute_locks.iter_mut() {
                    if lock.holder == holder && !lock.is_expired() {
                        lock.extend(duration);
                        return true;
                    }
                }
            }
            false
        }
        /// Clean up expired locks.
        pub fn cleanup_expired(&self) -> usize {
            let mut locks = self.locks.lock().expect("locks mutex poisoned");
            let mut removed_count = 0;
            locks.retain(|_, statute_locks| {
                let initial_len = statute_locks.len();
                statute_locks.retain(|lock| !lock.is_expired());
                removed_count += initial_len - statute_locks.len();
                !statute_locks.is_empty()
            });
            removed_count
        }
        /// Get the holder of an exclusive lock, if any.
        pub fn get_exclusive_holder(&self, statute_id: &str) -> Option<String> {
            let locks = self.locks.lock().expect("locks mutex poisoned");
            locks.get(statute_id).and_then(|v| {
                v.iter()
                    .find(|lock| !lock.is_expired() && lock.lock_type == LockType::Exclusive)
                    .map(|lock| lock.holder.clone())
            })
        }
    }
    impl Default for LockManager {
        fn default() -> Self {
            Self::new()
        }
    }
    /// Real-time conflict notification.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConflictNotification {
        /// Unique notification ID
        pub notification_id: Uuid,
        /// Statute ID where conflict occurred
        pub statute_id: String,
        /// Type of conflict
        pub conflict_type: ConflictType,
        /// Users involved in the conflict
        pub users: Vec<String>,
        /// Description of the conflict
        pub description: String,
        /// When the conflict was detected
        pub detected_at: DateTime<Utc>,
        /// Conflict resolution status
        pub status: ConflictStatus,
    }
    /// Type of conflict.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConflictType {
        /// Concurrent edits to the same statute
        ConcurrentEdit,
        /// Version mismatch
        VersionMismatch,
        /// Lock acquisition conflict
        LockConflict,
        /// Merge conflict
        MergeConflict,
        /// Other conflict type
        Other(String),
    }
    /// Status of conflict resolution.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConflictStatus {
        /// Conflict detected but not yet resolved
        Pending,
        /// Conflict is being resolved
        Resolving,
        /// Conflict has been resolved
        Resolved,
        /// Conflict resolution abandoned
        Abandoned,
    }
    impl ConflictNotification {
        /// Create a new conflict notification.
        pub fn new(
            statute_id: String,
            conflict_type: ConflictType,
            users: Vec<String>,
            description: String,
        ) -> Self {
            Self {
                notification_id: Uuid::new_v4(),
                statute_id,
                conflict_type,
                users,
                description,
                detected_at: Utc::now(),
                status: ConflictStatus::Pending,
            }
        }
        /// Mark the conflict as being resolved.
        pub fn mark_resolving(&mut self) {
            self.status = ConflictStatus::Resolving;
        }
        /// Mark the conflict as resolved.
        pub fn mark_resolved(&mut self) {
            self.status = ConflictStatus::Resolved;
        }
        /// Mark the conflict as abandoned.
        pub fn mark_abandoned(&mut self) {
            self.status = ConflictStatus::Abandoned;
        }
        /// Check if the conflict is resolved.
        pub fn is_resolved(&self) -> bool {
            self.status == ConflictStatus::Resolved
        }
    }
    /// Conflict notification manager.
    pub struct ConflictManager {
        notifications: Arc<Mutex<HashMap<Uuid, ConflictNotification>>>,
        statute_conflicts: Arc<Mutex<HashMap<String, Vec<Uuid>>>>,
    }
    impl ConflictManager {
        /// Create a new conflict manager.
        pub fn new() -> Self {
            Self {
                notifications: Arc::new(Mutex::new(HashMap::new())),
                statute_conflicts: Arc::new(Mutex::new(HashMap::new())),
            }
        }
        /// Register a new conflict.
        pub fn register_conflict(&self, notification: ConflictNotification) -> Uuid {
            let notification_id = notification.notification_id;
            let statute_id = notification.statute_id.clone();
            let mut notifications = self
                .notifications
                .lock()
                .expect("notifications mutex poisoned");
            notifications.insert(notification_id, notification);
            let mut statute_conflicts = self
                .statute_conflicts
                .lock()
                .expect("statute_conflicts mutex poisoned");
            statute_conflicts
                .entry(statute_id)
                .or_default()
                .push(notification_id);
            notification_id
        }
        /// Get a conflict by ID.
        pub fn get_conflict(&self, notification_id: Uuid) -> Option<ConflictNotification> {
            let notifications = self
                .notifications
                .lock()
                .expect("notifications mutex poisoned");
            notifications.get(&notification_id).cloned()
        }
        /// Get all conflicts for a statute.
        pub fn get_conflicts_for_statute(&self, statute_id: &str) -> Vec<ConflictNotification> {
            let statute_conflicts = self
                .statute_conflicts
                .lock()
                .expect("statute_conflicts mutex poisoned");
            let notifications = self
                .notifications
                .lock()
                .expect("notifications mutex poisoned");
            statute_conflicts
                .get(statute_id)
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| notifications.get(id).cloned())
                        .collect()
                })
                .unwrap_or_default()
        }
        /// Get all pending conflicts.
        pub fn get_pending_conflicts(&self) -> Vec<ConflictNotification> {
            let notifications = self
                .notifications
                .lock()
                .expect("notifications mutex poisoned");
            notifications
                .values()
                .filter(|n| n.status == ConflictStatus::Pending)
                .cloned()
                .collect()
        }
        /// Update conflict status.
        pub fn update_status(&self, notification_id: Uuid, status: ConflictStatus) -> bool {
            let mut notifications = self
                .notifications
                .lock()
                .expect("notifications mutex poisoned");
            if let Some(notification) = notifications.get_mut(&notification_id) {
                notification.status = status;
                return true;
            }
            false
        }
        /// Remove resolved conflicts older than a threshold.
        pub fn cleanup_resolved(&self, older_than: Duration) -> usize {
            let threshold = Utc::now()
                - chrono::Duration::from_std(older_than)
                    .expect("older_than duration is within chrono representable range");
            let mut notifications = self
                .notifications
                .lock()
                .expect("notifications mutex poisoned");
            let mut statute_conflicts = self
                .statute_conflicts
                .lock()
                .expect("statute_conflicts mutex poisoned");
            let to_remove: Vec<Uuid> = notifications
                .iter()
                .filter(|(_, n)| n.status == ConflictStatus::Resolved && n.detected_at < threshold)
                .map(|(id, _)| *id)
                .collect();
            let removed_count = to_remove.len();
            for id in to_remove {
                if let Some(notification) = notifications.remove(&id)
                    && let Some(conflicts) = statute_conflicts.get_mut(&notification.statute_id)
                {
                    conflicts.retain(|cid| *cid != id);
                }
            }
            statute_conflicts.retain(|_, conflicts| !conflicts.is_empty());
            removed_count
        }
    }
    impl Default for ConflictManager {
        fn default() -> Self {
            Self::new()
        }
    }
    /// Change stream subscription for real-time updates.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChangeSubscription {
        /// Subscription ID
        pub subscription_id: Uuid,
        /// User or client ID
        pub subscriber: String,
        /// Filter for statute IDs (if empty, subscribe to all)
        pub statute_filters: Vec<String>,
        /// Filter for event types (if empty, subscribe to all)
        pub event_filters: Vec<String>,
        /// When the subscription was created
        pub created_at: DateTime<Utc>,
        /// Last activity timestamp
        pub last_activity: DateTime<Utc>,
    }
    impl ChangeSubscription {
        /// Create a new subscription.
        pub fn new(subscriber: String) -> Self {
            Self {
                subscription_id: Uuid::new_v4(),
                subscriber,
                statute_filters: Vec::new(),
                event_filters: Vec::new(),
                created_at: Utc::now(),
                last_activity: Utc::now(),
            }
        }
        /// Add a statute ID filter.
        pub fn with_statute_filter(mut self, statute_id: String) -> Self {
            self.statute_filters.push(statute_id);
            self
        }
        /// Add an event type filter.
        pub fn with_event_filter(mut self, event_type: String) -> Self {
            self.event_filters.push(event_type);
            self
        }
        /// Check if this subscription matches a statute ID.
        pub fn matches_statute(&self, statute_id: &str) -> bool {
            self.statute_filters.is_empty()
                || self.statute_filters.contains(&statute_id.to_string())
        }
        /// Check if this subscription matches an event type.
        pub fn matches_event(&self, event_type: &str) -> bool {
            self.event_filters.is_empty() || self.event_filters.contains(&event_type.to_string())
        }
        /// Update last activity timestamp.
        pub fn touch(&mut self) {
            self.last_activity = Utc::now();
        }
    }
    /// Subscription manager for change streams.
    pub struct SubscriptionManager {
        subscriptions: Arc<Mutex<HashMap<Uuid, ChangeSubscription>>>,
        subscriber_subscriptions: Arc<Mutex<HashMap<String, Vec<Uuid>>>>,
    }
    impl SubscriptionManager {
        /// Create a new subscription manager.
        pub fn new() -> Self {
            Self {
                subscriptions: Arc::new(Mutex::new(HashMap::new())),
                subscriber_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            }
        }
        /// Create a new subscription.
        pub fn subscribe(&self, subscription: ChangeSubscription) -> Uuid {
            let subscription_id = subscription.subscription_id;
            let subscriber = subscription.subscriber.clone();
            let mut subscriptions = self
                .subscriptions
                .lock()
                .expect("subscriptions mutex poisoned");
            subscriptions.insert(subscription_id, subscription);
            let mut subscriber_subscriptions = self
                .subscriber_subscriptions
                .lock()
                .expect("subscriber_subscriptions mutex poisoned");
            subscriber_subscriptions
                .entry(subscriber)
                .or_default()
                .push(subscription_id);
            subscription_id
        }
        /// Unsubscribe by subscription ID.
        pub fn unsubscribe(&self, subscription_id: Uuid) -> bool {
            let mut subscriptions = self
                .subscriptions
                .lock()
                .expect("subscriptions mutex poisoned");
            if let Some(subscription) = subscriptions.remove(&subscription_id) {
                let mut subscriber_subscriptions = self
                    .subscriber_subscriptions
                    .lock()
                    .expect("subscriber_subscriptions mutex poisoned");
                if let Some(subs) = subscriber_subscriptions.get_mut(&subscription.subscriber) {
                    subs.retain(|id| *id != subscription_id);
                    if subs.is_empty() {
                        subscriber_subscriptions.remove(&subscription.subscriber);
                    }
                }
                return true;
            }
            false
        }
        /// Get all subscriptions for a subscriber.
        pub fn get_subscriber_subscriptions(&self, subscriber: &str) -> Vec<ChangeSubscription> {
            let subscriber_subscriptions = self
                .subscriber_subscriptions
                .lock()
                .expect("subscriber_subscriptions mutex poisoned");
            let subscriptions = self
                .subscriptions
                .lock()
                .expect("subscriptions mutex poisoned");
            subscriber_subscriptions
                .get(subscriber)
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| subscriptions.get(id).cloned())
                        .collect()
                })
                .unwrap_or_default()
        }
        /// Get all subscriptions that match a statute ID and event type.
        pub fn find_matching_subscriptions(
            &self,
            statute_id: &str,
            event_type: &str,
        ) -> Vec<ChangeSubscription> {
            let subscriptions = self
                .subscriptions
                .lock()
                .expect("subscriptions mutex poisoned");
            subscriptions
                .values()
                .filter(|sub| sub.matches_statute(statute_id) && sub.matches_event(event_type))
                .cloned()
                .collect()
        }
        /// Get total subscription count.
        pub fn subscription_count(&self) -> usize {
            let subscriptions = self
                .subscriptions
                .lock()
                .expect("subscriptions mutex poisoned");
            subscriptions.len()
        }
        /// Get total subscriber count.
        pub fn subscriber_count(&self) -> usize {
            let subscriber_subscriptions = self
                .subscriber_subscriptions
                .lock()
                .expect("subscriber_subscriptions mutex poisoned");
            subscriber_subscriptions.len()
        }
        /// Unsubscribe all subscriptions for a subscriber.
        pub fn unsubscribe_all(&self, subscriber: &str) -> usize {
            let mut subscriber_subscriptions = self
                .subscriber_subscriptions
                .lock()
                .expect("subscriber_subscriptions mutex poisoned");
            if let Some(sub_ids) = subscriber_subscriptions.remove(subscriber) {
                let mut subscriptions = self
                    .subscriptions
                    .lock()
                    .expect("subscriptions mutex poisoned");
                for id in &sub_ids {
                    subscriptions.remove(id);
                }
                return sub_ids.len();
            }
            0
        }
    }
    impl Default for SubscriptionManager {
        fn default() -> Self {
            Self::new()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_websocket_message_statute_id() {
            let msg = WebSocketMessage::StatuteCreated {
                statute_id: "test-1".to_string(),
                actor: "user1".to_string(),
                timestamp: Utc::now(),
            };
            assert_eq!(msg.statute_id(), Some("test-1"));
            let ping = WebSocketMessage::Ping;
            assert_eq!(ping.statute_id(), None);
        }
        #[test]
        fn test_websocket_message_actor() {
            let msg = WebSocketMessage::StatuteUpdated {
                statute_id: "test-1".to_string(),
                version: 2,
                actor: "user2".to_string(),
                timestamp: Utc::now(),
            };
            assert_eq!(msg.actor(), Some("user2"));
        }
        #[test]
        fn test_websocket_message_timestamp() {
            let now = Utc::now();
            let msg = WebSocketMessage::StatuteDeleted {
                statute_id: "test-1".to_string(),
                actor: "user1".to_string(),
                timestamp: now,
            };
            assert_eq!(msg.timestamp(), Some(now));
        }
        #[test]
        fn test_user_presence_creation() {
            let presence = UserPresence::new("user1".to_string())
                .with_display_name("Alice".to_string())
                .with_current_statute("statute-1".to_string());
            assert_eq!(presence.user_id, "user1");
            assert_eq!(presence.display_name, Some("Alice".to_string()));
            assert_eq!(presence.current_statute, Some("statute-1".to_string()));
            assert_eq!(presence.status, PresenceStatus::Online);
        }
        #[test]
        fn test_user_presence_set_status() {
            let mut presence = UserPresence::new("user1".to_string());
            presence.set_status(PresenceStatus::Away);
            assert_eq!(presence.status, PresenceStatus::Away);
        }
        #[test]
        fn test_user_presence_is_inactive() {
            let mut presence = UserPresence::new("user1".to_string());
            assert!(!presence.is_inactive(Duration::from_secs(60)));
            presence.last_activity = Utc::now() - chrono::Duration::seconds(120);
            assert!(presence.is_inactive(Duration::from_secs(60)));
        }
        #[test]
        fn test_presence_tracker_update_and_get() {
            let tracker = PresenceTracker::new();
            let presence = UserPresence::new("user1".to_string());
            tracker.update_presence(presence.clone());
            let retrieved = tracker.get_presence("user1");
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap().user_id, "user1");
        }
        #[test]
        fn test_presence_tracker_online_users() {
            let tracker = PresenceTracker::new();
            let presence1 = UserPresence::new("user1".to_string());
            let mut presence2 = UserPresence::new("user2".to_string());
            presence2.status = PresenceStatus::Offline;
            tracker.update_presence(presence1);
            tracker.update_presence(presence2);
            let online_users = tracker.get_online_users();
            assert_eq!(online_users.len(), 1);
            assert_eq!(online_users[0].user_id, "user1");
        }
        #[test]
        fn test_presence_tracker_users_for_statute() {
            let tracker = PresenceTracker::new();
            let presence1 = UserPresence::new("user1".to_string())
                .with_current_statute("statute-1".to_string());
            let presence2 = UserPresence::new("user2".to_string())
                .with_current_statute("statute-2".to_string());
            tracker.update_presence(presence1);
            tracker.update_presence(presence2);
            let users = tracker.get_users_for_statute("statute-1");
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].user_id, "user1");
        }
        #[test]
        fn test_presence_tracker_cleanup_inactive() {
            let tracker = PresenceTracker::new().with_inactive_threshold(Duration::from_secs(60));
            let mut presence = UserPresence::new("user1".to_string());
            presence.last_activity = Utc::now() - chrono::Duration::seconds(120);
            tracker.update_presence(presence);
            let inactive_users = tracker.cleanup_inactive();
            assert_eq!(inactive_users.len(), 1);
            assert_eq!(inactive_users[0], "user1");
            let presence = tracker.get_presence("user1");
            assert_eq!(presence.unwrap().status, PresenceStatus::Offline);
        }
        #[test]
        fn test_editing_lock_exclusive() {
            let lock = EditingLock::exclusive(
                "statute-1".to_string(),
                "user1".to_string(),
                Duration::from_secs(300),
            );
            assert_eq!(lock.statute_id, "statute-1");
            assert_eq!(lock.holder, "user1");
            assert_eq!(lock.lock_type, LockType::Exclusive);
            assert!(!lock.is_expired());
        }
        #[test]
        fn test_editing_lock_shared() {
            let lock = EditingLock::shared(
                "statute-1".to_string(),
                "user1".to_string(),
                Duration::from_secs(300),
            );
            assert_eq!(lock.lock_type, LockType::Shared);
        }
        #[test]
        fn test_editing_lock_extend() {
            let mut lock = EditingLock::exclusive(
                "statute-1".to_string(),
                "user1".to_string(),
                Duration::from_secs(300),
            );
            let initial_expiry = lock.expires_at;
            lock.extend(Duration::from_secs(60));
            assert!(lock.expires_at > initial_expiry);
        }
        #[test]
        fn test_lock_manager_acquire_exclusive() {
            let manager = LockManager::new();
            let result = manager.acquire_exclusive("statute-1".to_string(), "user1".to_string());
            assert!(result.is_ok());
            let result2 = manager.acquire_exclusive("statute-1".to_string(), "user2".to_string());
            assert!(result2.is_err());
        }
        #[test]
        fn test_lock_manager_acquire_shared() {
            let manager = LockManager::new();
            let result1 = manager.acquire_shared("statute-1".to_string(), "user1".to_string());
            assert!(result1.is_ok());
            let result2 = manager.acquire_shared("statute-1".to_string(), "user2".to_string());
            assert!(result2.is_ok());
        }
        #[test]
        fn test_lock_manager_exclusive_blocks_shared() {
            let manager = LockManager::new();
            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();
            let result = manager.acquire_shared("statute-1".to_string(), "user2".to_string());
            assert!(result.is_err());
        }
        #[test]
        fn test_lock_manager_release() {
            let manager = LockManager::new();
            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();
            let released = manager.release("statute-1", "user1");
            assert!(released);
            let result = manager.acquire_exclusive("statute-1".to_string(), "user2".to_string());
            assert!(result.is_ok());
        }
        #[test]
        fn test_lock_manager_is_locked() {
            let manager = LockManager::new();
            assert!(!manager.is_locked("statute-1"));
            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();
            assert!(manager.is_locked("statute-1"));
        }
        #[test]
        fn test_lock_manager_extend_lock() {
            let manager = LockManager::new();
            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();
            let extended = manager.extend_lock("statute-1", "user1", Duration::from_secs(60));
            assert!(extended);
        }
        #[test]
        fn test_conflict_notification_creation() {
            let notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string(), "user2".to_string()],
                "Concurrent edits detected".to_string(),
            );
            assert_eq!(notification.statute_id, "statute-1");
            assert_eq!(notification.conflict_type, ConflictType::ConcurrentEdit);
            assert_eq!(notification.users.len(), 2);
            assert_eq!(notification.status, ConflictStatus::Pending);
        }
        #[test]
        fn test_conflict_notification_status_changes() {
            let mut notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test".to_string(),
            );
            notification.mark_resolving();
            assert_eq!(notification.status, ConflictStatus::Resolving);
            notification.mark_resolved();
            assert_eq!(notification.status, ConflictStatus::Resolved);
            assert!(notification.is_resolved());
        }
        #[test]
        fn test_conflict_manager_register() {
            let manager = ConflictManager::new();
            let notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test".to_string(),
            );
            let id = manager.register_conflict(notification);
            let retrieved = manager.get_conflict(id);
            assert!(retrieved.is_some());
        }
        #[test]
        fn test_conflict_manager_get_for_statute() {
            let manager = ConflictManager::new();
            let notification1 = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test 1".to_string(),
            );
            let notification2 = ConflictNotification::new(
                "statute-2".to_string(),
                ConflictType::LockConflict,
                vec!["user2".to_string()],
                "Test 2".to_string(),
            );
            manager.register_conflict(notification1);
            manager.register_conflict(notification2);
            let conflicts = manager.get_conflicts_for_statute("statute-1");
            assert_eq!(conflicts.len(), 1);
            assert_eq!(conflicts[0].statute_id, "statute-1");
        }
        #[test]
        fn test_conflict_manager_pending_conflicts() {
            let manager = ConflictManager::new();
            let notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test".to_string(),
            );
            let id = manager.register_conflict(notification);
            let pending = manager.get_pending_conflicts();
            assert_eq!(pending.len(), 1);
            manager.update_status(id, ConflictStatus::Resolved);
            let pending = manager.get_pending_conflicts();
            assert_eq!(pending.len(), 0);
        }
        #[test]
        fn test_change_subscription_creation() {
            let subscription = ChangeSubscription::new("user1".to_string())
                .with_statute_filter("statute-1".to_string())
                .with_event_filter("StatuteUpdated".to_string());
            assert_eq!(subscription.subscriber, "user1");
            assert_eq!(subscription.statute_filters.len(), 1);
            assert_eq!(subscription.event_filters.len(), 1);
        }
        #[test]
        fn test_change_subscription_matches() {
            let subscription = ChangeSubscription::new("user1".to_string())
                .with_statute_filter("statute-1".to_string())
                .with_event_filter("StatuteUpdated".to_string());
            assert!(subscription.matches_statute("statute-1"));
            assert!(!subscription.matches_statute("statute-2"));
            assert!(subscription.matches_event("StatuteUpdated"));
            assert!(!subscription.matches_event("StatuteDeleted"));
        }
        #[test]
        fn test_change_subscription_match_all() {
            let subscription = ChangeSubscription::new("user1".to_string());
            assert!(subscription.matches_statute("any-statute"));
            assert!(subscription.matches_event("any-event"));
        }
        #[test]
        fn test_subscription_manager_subscribe() {
            let manager = SubscriptionManager::new();
            let subscription = ChangeSubscription::new("user1".to_string());
            let _id = manager.subscribe(subscription);
            assert_eq!(manager.subscription_count(), 1);
            assert_eq!(manager.subscriber_count(), 1);
        }
        #[test]
        fn test_subscription_manager_unsubscribe() {
            let manager = SubscriptionManager::new();
            let subscription = ChangeSubscription::new("user1".to_string());
            let id = manager.subscribe(subscription);
            let unsubscribed = manager.unsubscribe(id);
            assert!(unsubscribed);
            assert_eq!(manager.subscription_count(), 0);
        }
        #[test]
        fn test_subscription_manager_find_matching() {
            let manager = SubscriptionManager::new();
            let subscription = ChangeSubscription::new("user1".to_string())
                .with_statute_filter("statute-1".to_string())
                .with_event_filter("StatuteUpdated".to_string());
            manager.subscribe(subscription);
            let matches = manager.find_matching_subscriptions("statute-1", "StatuteUpdated");
            assert_eq!(matches.len(), 1);
            let no_matches = manager.find_matching_subscriptions("statute-2", "StatuteUpdated");
            assert_eq!(no_matches.len(), 0);
        }
        #[test]
        fn test_subscription_manager_unsubscribe_all() {
            let manager = SubscriptionManager::new();
            let sub1 = ChangeSubscription::new("user1".to_string());
            let sub2 = ChangeSubscription::new("user1".to_string());
            manager.subscribe(sub1);
            manager.subscribe(sub2);
            assert_eq!(manager.subscription_count(), 2);
            let removed = manager.unsubscribe_all("user1");
            assert_eq!(removed, 2);
            assert_eq!(manager.subscription_count(), 0);
        }
    }
}
