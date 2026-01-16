//! SDK Auto-Update Notifications
//!
//! This module provides SDK version tracking and update notifications:
//! - SDK version registry
//! - Update notifications
//! - Breaking change alerts
//! - Deprecation warnings

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

/// Error types for SDK notification operations
#[derive(Debug, Error)]
pub enum SdkNotificationError {
    #[error("SDK notification error: {0}")]
    Error(String),

    #[error("SDK not found: {0}")]
    SdkNotFound(String),

    #[error("Version not found: {0}")]
    VersionNotFound(String),
}

/// Result type for SDK notification operations
pub type SdkResult<T> = Result<T, SdkNotificationError>;

/// SDK language/platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SdkPlatform {
    TypeScript,
    Python,
    Rust,
    Go,
    Java,
    Ruby,
    PHP,
    CSharp,
}

impl std::fmt::Display for SdkPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdkPlatform::TypeScript => write!(f, "TypeScript"),
            SdkPlatform::Python => write!(f, "Python"),
            SdkPlatform::Rust => write!(f, "Rust"),
            SdkPlatform::Go => write!(f, "Go"),
            SdkPlatform::Java => write!(f, "Java"),
            SdkPlatform::Ruby => write!(f, "Ruby"),
            SdkPlatform::PHP => write!(f, "PHP"),
            SdkPlatform::CSharp => write!(f, "C#"),
        }
    }
}

/// Semantic version
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemanticVersion {
    /// Create a new semantic version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse from string (e.g., "1.2.3")
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;

        Some(Self::new(major, minor, patch))
    }

    /// Check if this version has breaking changes compared to other
    pub fn has_breaking_changes(&self, other: &Self) -> bool {
        self.major > other.major
    }

    /// Check if this version has new features compared to other
    pub fn has_new_features(&self, other: &Self) -> bool {
        self.major == other.major && self.minor > other.minor
    }

    /// Check if this version is a patch update compared to other
    pub fn is_patch_update(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch > other.patch
    }
}

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Update severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateSeverity {
    /// Critical security update
    Critical,

    /// Important update with breaking changes
    Major,

    /// Minor update with new features
    Minor,

    /// Patch update with bug fixes
    Patch,
}

/// SDK version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkVersion {
    /// Platform
    pub platform: SdkPlatform,

    /// Version
    pub version: SemanticVersion,

    /// Release date
    pub release_date: DateTime<Utc>,

    /// Changelog
    pub changelog: String,

    /// Breaking changes
    pub breaking_changes: Vec<String>,

    /// Deprecations
    pub deprecations: Vec<String>,

    /// Download URL
    pub download_url: String,

    /// Documentation URL
    pub documentation_url: String,
}

impl SdkVersion {
    /// Create a new SDK version
    pub fn new(
        platform: SdkPlatform,
        version: SemanticVersion,
        changelog: String,
        download_url: String,
    ) -> Self {
        Self {
            platform,
            version,
            release_date: Utc::now(),
            changelog,
            breaking_changes: Vec::new(),
            deprecations: Vec::new(),
            download_url,
            documentation_url: String::new(),
        }
    }

    /// Add breaking change
    pub fn with_breaking_change(mut self, change: String) -> Self {
        self.breaking_changes.push(change);
        self
    }

    /// Add deprecation
    pub fn with_deprecation(mut self, deprecation: String) -> Self {
        self.deprecations.push(deprecation);
        self
    }

    /// Set documentation URL
    pub fn with_documentation_url(mut self, url: String) -> Self {
        self.documentation_url = url;
        self
    }

    /// Get update severity compared to another version
    pub fn update_severity(&self, current: &SemanticVersion) -> UpdateSeverity {
        if self.version.has_breaking_changes(current) {
            if !self.breaking_changes.is_empty() {
                UpdateSeverity::Critical
            } else {
                UpdateSeverity::Major
            }
        } else if self.version.has_new_features(current) {
            UpdateSeverity::Minor
        } else {
            UpdateSeverity::Patch
        }
    }
}

/// Update notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotification {
    /// Notification ID
    pub id: String,

    /// Platform
    pub platform: SdkPlatform,

    /// Current version
    pub current_version: SemanticVersion,

    /// Latest version
    pub latest_version: SemanticVersion,

    /// Update severity
    pub severity: UpdateSeverity,

    /// Update message
    pub message: String,

    /// Update URL
    pub update_url: String,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Acknowledged by user
    pub acknowledged: bool,
}

impl UpdateNotification {
    /// Create a new update notification
    pub fn new(
        platform: SdkPlatform,
        current_version: SemanticVersion,
        latest_version: SemanticVersion,
        severity: UpdateSeverity,
        update_url: String,
    ) -> Self {
        let message = match severity {
            UpdateSeverity::Critical => {
                format!(
                    "Critical update available for {} SDK: {} -> {}. Please update immediately!",
                    platform, current_version, latest_version
                )
            }
            UpdateSeverity::Major => {
                format!(
                    "Major update available for {} SDK: {} -> {}. This update contains breaking changes.",
                    platform, current_version, latest_version
                )
            }
            UpdateSeverity::Minor => {
                format!(
                    "New features available for {} SDK: {} -> {}",
                    platform, current_version, latest_version
                )
            }
            UpdateSeverity::Patch => {
                format!(
                    "Bug fixes available for {} SDK: {} -> {}",
                    platform, current_version, latest_version
                )
            }
        };

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            platform,
            current_version,
            latest_version,
            severity,
            message,
            update_url,
            created_at: Utc::now(),
            acknowledged: false,
        }
    }

    /// Acknowledge this notification
    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }
}

/// SDK registry for managing SDK versions
pub struct SdkRegistry {
    versions: Arc<RwLock<HashMap<SdkPlatform, Vec<SdkVersion>>>>,
    notifications: Arc<RwLock<Vec<UpdateNotification>>>,
}

impl SdkRegistry {
    /// Create a new SDK registry
    pub fn new() -> Self {
        Self {
            versions: Arc::new(RwLock::new(HashMap::new())),
            notifications: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a new SDK version
    pub fn register_version(&self, version: SdkVersion) -> SdkResult<()> {
        let mut versions = self.versions.write().map_err(|e| {
            SdkNotificationError::Error(format!("Failed to acquire write lock: {}", e))
        })?;

        let platform_versions = versions.entry(version.platform).or_insert_with(Vec::new);
        platform_versions.push(version);
        platform_versions.sort_by(|a, b| b.version.cmp(&a.version));

        Ok(())
    }

    /// Get latest version for a platform
    pub fn get_latest_version(&self, platform: SdkPlatform) -> SdkResult<SdkVersion> {
        let versions = self.versions.read().map_err(|e| {
            SdkNotificationError::Error(format!("Failed to acquire read lock: {}", e))
        })?;

        versions
            .get(&platform)
            .and_then(|v| v.first())
            .cloned()
            .ok_or_else(|| SdkNotificationError::SdkNotFound(platform.to_string()))
    }

    /// Get all versions for a platform
    pub fn get_all_versions(&self, platform: SdkPlatform) -> Vec<SdkVersion> {
        self.versions
            .read()
            .unwrap()
            .get(&platform)
            .cloned()
            .unwrap_or_default()
    }

    /// Check for updates
    pub fn check_for_updates(
        &self,
        platform: SdkPlatform,
        current_version: SemanticVersion,
    ) -> SdkResult<Option<UpdateNotification>> {
        let latest = self.get_latest_version(platform)?;

        if latest.version > current_version {
            let severity = latest.update_severity(&current_version);
            let notification = UpdateNotification::new(
                platform,
                current_version,
                latest.version.clone(),
                severity,
                latest.download_url.clone(),
            );

            // Store notification
            self.notifications
                .write()
                .unwrap()
                .push(notification.clone());

            Ok(Some(notification))
        } else {
            Ok(None)
        }
    }

    /// Get all notifications
    pub fn get_notifications(&self) -> Vec<UpdateNotification> {
        self.notifications.read().unwrap().clone()
    }

    /// Get unacknowledged notifications
    pub fn get_unacknowledged_notifications(&self) -> Vec<UpdateNotification> {
        self.notifications
            .read()
            .unwrap()
            .iter()
            .filter(|n| !n.acknowledged)
            .cloned()
            .collect()
    }

    /// Acknowledge a notification
    pub fn acknowledge_notification(&self, id: &str) -> SdkResult<()> {
        let mut notifications = self.notifications.write().map_err(|e| {
            SdkNotificationError::Error(format!("Failed to acquire write lock: {}", e))
        })?;

        if let Some(notification) = notifications.iter_mut().find(|n| n.id == id) {
            notification.acknowledge();
            Ok(())
        } else {
            Err(SdkNotificationError::Error(
                "Notification not found".to_string(),
            ))
        }
    }

    /// Clear all notifications
    pub fn clear_notifications(&self) -> SdkResult<()> {
        let mut notifications = self.notifications.write().map_err(|e| {
            SdkNotificationError::Error(format!("Failed to acquire write lock: {}", e))
        })?;

        notifications.clear();
        Ok(())
    }
}

impl Default for SdkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_version() {
        let v1 = SemanticVersion::new(1, 0, 0);
        let v2 = SemanticVersion::new(2, 0, 0);
        let v1_1 = SemanticVersion::new(1, 1, 0);
        let v1_0_1 = SemanticVersion::new(1, 0, 1);

        assert!(v2 > v1);
        assert!(v2.has_breaking_changes(&v1));
        assert!(v1_1.has_new_features(&v1));
        assert!(v1_0_1.is_patch_update(&v1));
    }

    #[test]
    fn test_version_parsing() {
        let v = SemanticVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_sdk_registry() {
        let registry = SdkRegistry::new();

        let version = SdkVersion::new(
            SdkPlatform::TypeScript,
            SemanticVersion::new(1, 0, 0),
            "Initial release".to_string(),
            "https://example.com/sdk".to_string(),
        );

        registry.register_version(version).unwrap();

        let latest = registry
            .get_latest_version(SdkPlatform::TypeScript)
            .unwrap();
        assert_eq!(latest.version, SemanticVersion::new(1, 0, 0));
    }

    #[test]
    fn test_update_check() {
        let registry = SdkRegistry::new();

        // Register v1.0.0
        let v1 = SdkVersion::new(
            SdkPlatform::Python,
            SemanticVersion::new(1, 0, 0),
            "Initial release".to_string(),
            "https://example.com/v1".to_string(),
        );
        registry.register_version(v1).unwrap();

        // Register v2.0.0
        let v2 = SdkVersion::new(
            SdkPlatform::Python,
            SemanticVersion::new(2, 0, 0),
            "Major update".to_string(),
            "https://example.com/v2".to_string(),
        )
        .with_breaking_change("API changed".to_string());
        registry.register_version(v2).unwrap();

        // Check for updates from v1.0.0
        let current = SemanticVersion::new(1, 0, 0);
        let notification = registry
            .check_for_updates(SdkPlatform::Python, current)
            .unwrap();

        assert!(notification.is_some());
        let notification = notification.unwrap();
        assert_eq!(notification.severity, UpdateSeverity::Critical);
    }

    #[test]
    fn test_notification_acknowledgement() {
        let registry = SdkRegistry::new();

        let v1 = SdkVersion::new(
            SdkPlatform::Rust,
            SemanticVersion::new(1, 0, 0),
            "Release".to_string(),
            "https://example.com".to_string(),
        );
        registry.register_version(v1).unwrap();

        let v2 = SdkVersion::new(
            SdkPlatform::Rust,
            SemanticVersion::new(1, 1, 0),
            "New features".to_string(),
            "https://example.com".to_string(),
        );
        registry.register_version(v2).unwrap();

        let notification = registry
            .check_for_updates(SdkPlatform::Rust, SemanticVersion::new(1, 0, 0))
            .unwrap()
            .unwrap();

        let unack = registry.get_unacknowledged_notifications();
        assert_eq!(unack.len(), 1);

        registry.acknowledge_notification(&notification.id).unwrap();

        let unack = registry.get_unacknowledged_notifications();
        assert_eq!(unack.len(), 0);
    }
}
