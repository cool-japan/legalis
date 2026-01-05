//! Presence awareness for tracking user activity.
//!
//! This module provides real-time tracking of which users are viewing or editing
//! which resources, enabling collaborative features and preventing edit conflicts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Type of presence activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenceActivity {
    /// User is viewing a resource
    Viewing,
    /// User is editing a resource
    Editing,
}

/// Information about a user's presence on a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceInfo {
    /// User ID
    pub user_id: String,
    /// Username for display
    pub username: String,
    /// Type of activity
    pub activity: PresenceActivity,
    /// Timestamp when the presence was last updated
    pub last_seen: DateTime<Utc>,
    /// Additional metadata (e.g., cursor position, selected text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Presence manager for tracking user presence across resources.
#[derive(Clone)]
pub struct PresenceManager {
    /// Map of resource_id -> list of present users
    presences: Arc<RwLock<HashMap<String, Vec<PresenceInfo>>>>,
    /// Timeout duration for presence expiry (in seconds)
    timeout_seconds: u64,
}

impl PresenceManager {
    /// Creates a new presence manager.
    ///
    /// # Arguments
    /// * `timeout_seconds` - How long before a presence entry is considered stale (default: 30 seconds)
    pub fn new(timeout_seconds: u64) -> Self {
        Self {
            presences: Arc::new(RwLock::new(HashMap::new())),
            timeout_seconds,
        }
    }

    /// Updates a user's presence on a resource.
    pub async fn update_presence(
        &self,
        resource_id: String,
        user_id: String,
        username: String,
        activity: PresenceActivity,
        metadata: Option<serde_json::Value>,
    ) {
        let mut presences = self.presences.write().await;
        let resource_presences = presences.entry(resource_id).or_insert_with(Vec::new);

        // Remove existing presence for this user
        resource_presences.retain(|p| p.user_id != user_id);

        // Add new presence
        resource_presences.push(PresenceInfo {
            user_id,
            username,
            activity,
            last_seen: Utc::now(),
            metadata,
        });

        // Clean up stale entries
        self.cleanup_stale_presences(resource_presences);
    }

    /// Removes a user's presence from a resource.
    pub async fn remove_presence(&self, resource_id: &str, user_id: &str) {
        let mut presences = self.presences.write().await;
        if let Some(resource_presences) = presences.get_mut(resource_id) {
            resource_presences.retain(|p| p.user_id != user_id);

            // Remove the resource entry if no one is present
            if resource_presences.is_empty() {
                presences.remove(resource_id);
            }
        }
    }

    /// Gets all users present on a resource.
    pub async fn get_presence(&self, resource_id: &str) -> Vec<PresenceInfo> {
        let mut presences = self.presences.write().await;
        if let Some(resource_presences) = presences.get_mut(resource_id) {
            self.cleanup_stale_presences(resource_presences);
            resource_presences.clone()
        } else {
            Vec::new()
        }
    }

    /// Gets all users currently editing a resource.
    pub async fn get_editors(&self, resource_id: &str) -> Vec<PresenceInfo> {
        let presences = self.get_presence(resource_id).await;
        presences
            .into_iter()
            .filter(|p| p.activity == PresenceActivity::Editing)
            .collect()
    }

    /// Gets all users currently viewing a resource.
    pub async fn get_viewers(&self, resource_id: &str) -> Vec<PresenceInfo> {
        let presences = self.get_presence(resource_id).await;
        presences
            .into_iter()
            .filter(|p| p.activity == PresenceActivity::Viewing)
            .collect()
    }

    /// Cleans up all stale presences across all resources.
    pub async fn cleanup_all(&self) {
        let mut presences = self.presences.write().await;
        for resource_presences in presences.values_mut() {
            self.cleanup_stale_presences(resource_presences);
        }

        // Remove empty resource entries
        presences.retain(|_, v| !v.is_empty());
    }

    /// Removes stale presence entries (older than timeout).
    fn cleanup_stale_presences(&self, presences: &mut Vec<PresenceInfo>) {
        let now = Utc::now();
        let timeout = chrono::Duration::seconds(self.timeout_seconds as i64);
        presences.retain(|p| now - p.last_seen < timeout);
    }

    /// Gets a summary of presence across all resources.
    pub async fn get_summary(&self) -> HashMap<String, usize> {
        let presences = self.presences.read().await;
        presences
            .iter()
            .map(|(resource_id, users)| (resource_id.clone(), users.len()))
            .collect()
    }
}

impl Default for PresenceManager {
    fn default() -> Self {
        Self::new(30) // 30 seconds default timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_presence() {
        let manager = PresenceManager::new(60);

        manager
            .update_presence(
                "statute-1".to_string(),
                "user-1".to_string(),
                "Alice".to_string(),
                PresenceActivity::Viewing,
                None,
            )
            .await;

        let presences = manager.get_presence("statute-1").await;
        assert_eq!(presences.len(), 1);
        assert_eq!(presences[0].user_id, "user-1");
        assert_eq!(presences[0].username, "Alice");
        assert_eq!(presences[0].activity, PresenceActivity::Viewing);
    }

    #[tokio::test]
    async fn test_multiple_users() {
        let manager = PresenceManager::new(60);

        manager
            .update_presence(
                "statute-1".to_string(),
                "user-1".to_string(),
                "Alice".to_string(),
                PresenceActivity::Viewing,
                None,
            )
            .await;

        manager
            .update_presence(
                "statute-1".to_string(),
                "user-2".to_string(),
                "Bob".to_string(),
                PresenceActivity::Editing,
                None,
            )
            .await;

        let presences = manager.get_presence("statute-1").await;
        assert_eq!(presences.len(), 2);

        let editors = manager.get_editors("statute-1").await;
        assert_eq!(editors.len(), 1);
        assert_eq!(editors[0].user_id, "user-2");

        let viewers = manager.get_viewers("statute-1").await;
        assert_eq!(viewers.len(), 1);
        assert_eq!(viewers[0].user_id, "user-1");
    }

    #[tokio::test]
    async fn test_remove_presence() {
        let manager = PresenceManager::new(60);

        manager
            .update_presence(
                "statute-1".to_string(),
                "user-1".to_string(),
                "Alice".to_string(),
                PresenceActivity::Viewing,
                None,
            )
            .await;

        manager.remove_presence("statute-1", "user-1").await;

        let presences = manager.get_presence("statute-1").await;
        assert_eq!(presences.len(), 0);
    }

    #[tokio::test]
    async fn test_presence_update() {
        let manager = PresenceManager::new(60);

        // User starts viewing
        manager
            .update_presence(
                "statute-1".to_string(),
                "user-1".to_string(),
                "Alice".to_string(),
                PresenceActivity::Viewing,
                None,
            )
            .await;

        // User starts editing
        manager
            .update_presence(
                "statute-1".to_string(),
                "user-1".to_string(),
                "Alice".to_string(),
                PresenceActivity::Editing,
                None,
            )
            .await;

        let presences = manager.get_presence("statute-1").await;
        assert_eq!(presences.len(), 1);
        assert_eq!(presences[0].activity, PresenceActivity::Editing);
    }

    #[tokio::test]
    async fn test_summary() {
        let manager = PresenceManager::new(60);

        manager
            .update_presence(
                "statute-1".to_string(),
                "user-1".to_string(),
                "Alice".to_string(),
                PresenceActivity::Viewing,
                None,
            )
            .await;

        manager
            .update_presence(
                "statute-2".to_string(),
                "user-2".to_string(),
                "Bob".to_string(),
                PresenceActivity::Editing,
                None,
            )
            .await;

        let summary = manager.get_summary().await;
        assert_eq!(summary.len(), 2);
        assert_eq!(summary.get("statute-1"), Some(&1));
        assert_eq!(summary.get("statute-2"), Some(&1));
    }
}
