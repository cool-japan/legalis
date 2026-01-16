//! Team collaboration features for Legalis CLI.
//!
//! This module provides team workspace management, shared command history,
//! collaborative sessions, notifications, and role-based access control.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Team workspace structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Workspace ID
    pub id: String,
    /// Workspace name
    pub name: String,
    /// Workspace description
    pub description: Option<String>,
    /// Workspace owner
    pub owner: String,
    /// Team members with roles
    pub members: HashMap<String, Role>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Workspace settings
    pub settings: WorkspaceSettings,
}

/// Workspace settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    /// Enable command history sharing
    pub share_history: bool,
    /// Enable notifications
    pub enable_notifications: bool,
    /// Maximum session participants
    pub max_session_participants: usize,
    /// Session timeout in minutes
    pub session_timeout: u64,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            share_history: true,
            enable_notifications: true,
            max_session_participants: 10,
            session_timeout: 60,
        }
    }
}

/// User role in a workspace.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    /// Owner (full access, can delete workspace)
    Owner,
    /// Admin (can manage members and access)
    Admin,
    /// Write (can modify and execute commands)
    Write,
    /// Read (can view only)
    Read,
}

impl Role {
    /// Check if role has permission to perform an action.
    pub fn can_manage_members(&self) -> bool {
        matches!(self, Role::Owner | Role::Admin)
    }

    /// Check if role can write.
    pub fn can_write(&self) -> bool {
        matches!(self, Role::Owner | Role::Admin | Role::Write)
    }

    /// Check if role can delete workspace.
    pub fn can_delete_workspace(&self) -> bool {
        matches!(self, Role::Owner)
    }
}

/// Command history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Entry ID
    pub id: String,
    /// Workspace ID
    pub workspace_id: String,
    /// User who executed the command
    pub user: String,
    /// Command that was executed
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Command output (truncated)
    pub output: Option<String>,
}

/// Collaborative session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: String,
    /// Session name
    pub name: String,
    /// Workspace ID
    pub workspace_id: String,
    /// Session description
    pub description: Option<String>,
    /// Session owner
    pub owner: String,
    /// Active participants
    pub participants: Vec<Participant>,
    /// Maximum participants allowed
    pub max_participants: usize,
    /// Session status
    pub status: SessionStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
}

/// Session participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// User name
    pub user: String,
    /// Join timestamp
    pub joined_at: DateTime<Utc>,
    /// Read-only mode
    pub readonly: bool,
}

/// Session status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    /// Session is active
    Active,
    /// Session is paused
    Paused,
    /// Session has ended
    Ended,
}

/// Notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Workspace ID
    pub workspace_id: String,
    /// Sender
    pub sender: String,
    /// Recipients
    pub recipients: Vec<String>,
    /// Message
    pub message: String,
    /// Priority
    pub priority: Priority,
    /// Read status by user
    pub read_by: HashMap<String, DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Notification priority.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
}

/// Team manager for workspace operations.
pub struct TeamManager {
    /// Base directory for team data
    base_dir: PathBuf,
}

impl TeamManager {
    /// Create a new team manager.
    pub fn new() -> Result<Self> {
        let base_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("legalis")
            .join("team");

        fs::create_dir_all(&base_dir)?;

        Ok(Self { base_dir })
    }

    /// Get workspace directory.
    fn workspace_dir(&self, workspace_id: &str) -> PathBuf {
        self.base_dir.join("workspaces").join(workspace_id)
    }

    /// Get workspace config path.
    fn workspace_config_path(&self, workspace_id: &str) -> PathBuf {
        self.workspace_dir(workspace_id).join("workspace.toml")
    }

    /// Get history directory.
    fn history_dir(&self, workspace_id: &str) -> PathBuf {
        self.workspace_dir(workspace_id).join("history")
    }

    /// Get sessions directory.
    fn sessions_dir(&self, workspace_id: &str) -> PathBuf {
        self.workspace_dir(workspace_id).join("sessions")
    }

    /// Get notifications directory.
    fn notifications_dir(&self, workspace_id: &str) -> PathBuf {
        self.workspace_dir(workspace_id).join("notifications")
    }

    /// Create a new workspace.
    pub fn create_workspace(
        &self,
        name: &str,
        description: Option<String>,
        owner: &str,
        members: Vec<String>,
    ) -> Result<Workspace> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let mut member_map = HashMap::new();
        member_map.insert(owner.to_string(), Role::Owner);
        for member in members {
            member_map.insert(member, Role::Write);
        }

        let workspace = Workspace {
            id: id.clone(),
            name: name.to_string(),
            description,
            owner: owner.to_string(),
            members: member_map,
            created_at: now,
            updated_at: now,
            settings: WorkspaceSettings::default(),
        };

        // Create workspace directory
        let workspace_dir = self.workspace_dir(&id);
        fs::create_dir_all(&workspace_dir)?;

        // Save workspace config
        self.save_workspace(&workspace)?;

        // Create subdirectories
        fs::create_dir_all(self.history_dir(&id))?;
        fs::create_dir_all(self.sessions_dir(&id))?;
        fs::create_dir_all(self.notifications_dir(&id))?;

        Ok(workspace)
    }

    /// Save workspace configuration.
    pub fn save_workspace(&self, workspace: &Workspace) -> Result<()> {
        let config_path = self.workspace_config_path(&workspace.id);
        let toml_str = toml::to_string_pretty(workspace)?;
        fs::write(config_path, toml_str)?;
        Ok(())
    }

    /// Load workspace configuration.
    pub fn load_workspace(&self, workspace_id: &str) -> Result<Workspace> {
        let config_path = self.workspace_config_path(workspace_id);
        let toml_str = fs::read_to_string(&config_path).with_context(|| {
            format!("Failed to read workspace config: {}", config_path.display())
        })?;
        let workspace: Workspace = toml::from_str(&toml_str)?;
        Ok(workspace)
    }

    /// List all workspaces.
    pub fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        let workspaces_dir = self.base_dir.join("workspaces");
        if !workspaces_dir.exists() {
            return Ok(Vec::new());
        }

        let mut workspaces = Vec::new();
        for entry in fs::read_dir(workspaces_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let workspace_id = entry.file_name().to_string_lossy().to_string();
                if let Ok(workspace) = self.load_workspace(&workspace_id) {
                    workspaces.push(workspace);
                }
            }
        }

        Ok(workspaces)
    }

    /// Add command to history.
    pub fn add_history_entry(
        &self,
        workspace_id: &str,
        user: &str,
        command: &str,
        args: Vec<String>,
        exit_code: Option<i32>,
        output: Option<String>,
    ) -> Result<HistoryEntry> {
        let id = uuid::Uuid::new_v4().to_string();
        let entry = HistoryEntry {
            id: id.clone(),
            workspace_id: workspace_id.to_string(),
            user: user.to_string(),
            command: command.to_string(),
            args,
            executed_at: Utc::now(),
            exit_code,
            output,
        };

        let history_file = self.history_dir(workspace_id).join(format!("{}.json", id));
        let json_str = serde_json::to_string_pretty(&entry)?;
        fs::write(history_file, json_str)?;

        Ok(entry)
    }

    /// Get command history.
    pub fn get_history(
        &self,
        workspace_id: &str,
        limit: usize,
        user_filter: Option<&str>,
    ) -> Result<Vec<HistoryEntry>> {
        let history_dir = self.history_dir(workspace_id);
        if !history_dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for entry in fs::read_dir(history_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let json_str = fs::read_to_string(entry.path())?;
                if let Ok(history_entry) = serde_json::from_str::<HistoryEntry>(&json_str) {
                    if let Some(user) = user_filter {
                        if history_entry.user == user {
                            entries.push(history_entry);
                        }
                    } else {
                        entries.push(history_entry);
                    }
                }
            }
        }

        // Sort by timestamp (newest first)
        entries.sort_by(|a, b| b.executed_at.cmp(&a.executed_at));

        // Limit results
        entries.truncate(limit);

        Ok(entries)
    }

    /// Create a new session.
    pub fn create_session(
        &self,
        workspace_id: &str,
        name: &str,
        description: Option<String>,
        owner: &str,
        max_participants: usize,
    ) -> Result<Session> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let session = Session {
            id: id.clone(),
            name: name.to_string(),
            workspace_id: workspace_id.to_string(),
            description,
            owner: owner.to_string(),
            participants: vec![Participant {
                user: owner.to_string(),
                joined_at: now,
                readonly: false,
            }],
            max_participants,
            status: SessionStatus::Active,
            created_at: now,
            last_activity: now,
        };

        // Save session
        let session_file = self.sessions_dir(workspace_id).join(format!("{}.json", id));
        let json_str = serde_json::to_string_pretty(&session)?;
        fs::write(session_file, json_str)?;

        Ok(session)
    }

    /// Load session.
    pub fn load_session(&self, workspace_id: &str, session_id: &str) -> Result<Session> {
        let session_file = self
            .sessions_dir(workspace_id)
            .join(format!("{}.json", session_id));
        let json_str = fs::read_to_string(&session_file)?;
        let session: Session = serde_json::from_str(&json_str)?;
        Ok(session)
    }

    /// Save session.
    pub fn save_session(&self, session: &Session) -> Result<()> {
        let session_file = self
            .sessions_dir(&session.workspace_id)
            .join(format!("{}.json", session.id));
        let json_str = serde_json::to_string_pretty(session)?;
        fs::write(session_file, json_str)?;
        Ok(())
    }

    /// List sessions.
    pub fn list_sessions(&self, workspace_id: &str, include_ended: bool) -> Result<Vec<Session>> {
        let sessions_dir = self.sessions_dir(workspace_id);
        if !sessions_dir.exists() {
            return Ok(Vec::new());
        }

        let mut sessions = Vec::new();
        for entry in fs::read_dir(sessions_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let json_str = fs::read_to_string(entry.path())?;
                if let Ok(session) = serde_json::from_str::<Session>(&json_str) {
                    if include_ended || session.status != SessionStatus::Ended {
                        sessions.push(session);
                    }
                }
            }
        }

        // Sort by last activity (newest first)
        sessions.sort_by(|a, b| b.last_activity.cmp(&a.last_activity));

        Ok(sessions)
    }

    /// Create a notification.
    pub fn create_notification(
        &self,
        workspace_id: &str,
        sender: &str,
        recipients: Vec<String>,
        message: &str,
        priority: Priority,
    ) -> Result<Notification> {
        let id = uuid::Uuid::new_v4().to_string();

        let notification = Notification {
            id: id.clone(),
            workspace_id: workspace_id.to_string(),
            sender: sender.to_string(),
            recipients,
            message: message.to_string(),
            priority,
            read_by: HashMap::new(),
            created_at: Utc::now(),
        };

        // Save notification
        let notification_file = self
            .notifications_dir(workspace_id)
            .join(format!("{}.json", id));
        let json_str = serde_json::to_string_pretty(&notification)?;
        fs::write(notification_file, json_str)?;

        Ok(notification)
    }

    /// Get notifications for a workspace.
    pub fn get_notifications(
        &self,
        workspace_id: &str,
        unread_only: bool,
        user: &str,
        limit: usize,
    ) -> Result<Vec<Notification>> {
        let notifications_dir = self.notifications_dir(workspace_id);
        if !notifications_dir.exists() {
            return Ok(Vec::new());
        }

        let mut notifications = Vec::new();
        for entry in fs::read_dir(notifications_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let json_str = fs::read_to_string(entry.path())?;
                if let Ok(notification) = serde_json::from_str::<Notification>(&json_str) {
                    // Check if user is a recipient
                    if !notification.recipients.contains(&user.to_string()) {
                        continue;
                    }

                    // Filter by read status
                    if unread_only && notification.read_by.contains_key(user) {
                        continue;
                    }

                    notifications.push(notification);
                }
            }
        }

        // Sort by timestamp (newest first)
        notifications.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Limit results
        notifications.truncate(limit);

        Ok(notifications)
    }

    /// Mark notification as read.
    pub fn mark_notification_read(
        &self,
        workspace_id: &str,
        notification_id: &str,
        user: &str,
    ) -> Result<()> {
        let notification_file = self
            .notifications_dir(workspace_id)
            .join(format!("{}.json", notification_id));

        let json_str = fs::read_to_string(&notification_file)?;
        let mut notification: Notification = serde_json::from_str(&json_str)?;

        notification.read_by.insert(user.to_string(), Utc::now());

        let json_str = serde_json::to_string_pretty(&notification)?;
        fs::write(notification_file, json_str)?;

        Ok(())
    }

    /// Update user role in workspace.
    pub fn update_user_role(&self, workspace_id: &str, user: &str, role: Role) -> Result<()> {
        let mut workspace = self.load_workspace(workspace_id)?;
        workspace.members.insert(user.to_string(), role);
        workspace.updated_at = Utc::now();
        self.save_workspace(&workspace)?;
        Ok(())
    }

    /// Remove user from workspace.
    pub fn remove_user(&self, workspace_id: &str, user: &str) -> Result<()> {
        let mut workspace = self.load_workspace(workspace_id)?;
        workspace.members.remove(user);
        workspace.updated_at = Utc::now();
        self.save_workspace(&workspace)?;
        Ok(())
    }
}

impl Default for TeamManager {
    fn default() -> Self {
        Self::new().expect("Failed to create team manager")
    }
}
