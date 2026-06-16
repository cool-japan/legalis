//! Role-based access control for collaborative visualizations.
//!
//! [`AccessControlList`] maps user ids to a [`Role`]; each role grants a fixed
//! set of [`Capability`]s. [`AccessControlList::can`] answers authorization
//! questions ("may this user edit?") and is consulted by
//! [`EditSession`](crate::EditSession) before applying edit operations.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data_exchange::escape_xml;
use crate::types_7::CollaborativeUser;
use crate::{VizError, VizResult};

/// A discrete action a user may be permitted to perform.
///
/// Ordered from least to most privileged, so `>=` comparisons are meaningful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Capability {
    /// View the visualization.
    View,
    /// Post comments.
    Comment,
    /// Edit the document (add/remove/modify nodes and edges).
    Edit,
    /// Manage content settings, presets and versions.
    Manage,
    /// Administer other users' permissions.
    AdministerPermissions,
}

impl Capability {
    /// A stable lower-case slug.
    pub fn slug(&self) -> &'static str {
        match self {
            Capability::View => "view",
            Capability::Comment => "comment",
            Capability::Edit => "edit",
            Capability::Manage => "manage",
            Capability::AdministerPermissions => "administer-permissions",
        }
    }
}

/// A named role bundling a set of [`Capability`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// Full control, including permission administration.
    Owner,
    /// May view, comment and edit.
    Editor,
    /// May view and comment.
    Commenter,
    /// May view only.
    Viewer,
}

impl Role {
    /// A human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Role::Owner => "Owner",
            Role::Editor => "Editor",
            Role::Commenter => "Commenter",
            Role::Viewer => "Viewer",
        }
    }

    /// The capabilities granted by this role, from least to most privileged.
    pub fn capabilities(&self) -> Vec<Capability> {
        match self {
            Role::Owner => vec![
                Capability::View,
                Capability::Comment,
                Capability::Edit,
                Capability::Manage,
                Capability::AdministerPermissions,
            ],
            Role::Editor => vec![Capability::View, Capability::Comment, Capability::Edit],
            Role::Commenter => vec![Capability::View, Capability::Comment],
            Role::Viewer => vec![Capability::View],
        }
    }

    /// Whether this role grants the given capability.
    pub fn allows(&self, capability: Capability) -> bool {
        match self {
            Role::Owner => true,
            Role::Editor => matches!(
                capability,
                Capability::View | Capability::Comment | Capability::Edit
            ),
            Role::Commenter => matches!(capability, Capability::View | Capability::Comment),
            Role::Viewer => capability == Capability::View,
        }
    }
}

/// Maps user ids to roles and answers authorization queries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessControlList {
    entries: BTreeMap<String, Role>,
}

impl AccessControlList {
    /// Creates an empty access control list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Grants (or updates) a role for a user id.
    pub fn grant(&mut self, user_id: &str, role: Role) {
        self.entries.insert(user_id.to_string(), role);
    }

    /// Grants a role for a [`CollaborativeUser`] (convenience over `grant`).
    pub fn grant_user(&mut self, user: &CollaborativeUser, role: Role) {
        self.grant(&user.user_id, role);
    }

    /// Revokes a user's role; returns whether one was removed.
    pub fn revoke(&mut self, user_id: &str) -> bool {
        self.entries.remove(user_id).is_some()
    }

    /// The role assigned to a user, if any.
    pub fn role_of(&self, user_id: &str) -> Option<Role> {
        self.entries.get(user_id).copied()
    }

    /// Whether a user has a capability (false if the user is unknown).
    pub fn can(&self, user_id: &str, capability: Capability) -> bool {
        self.role_of(user_id)
            .map(|role| role.allows(capability))
            .unwrap_or(false)
    }

    /// Requires a capability, returning an error naming the missing permission.
    pub fn require(&self, user_id: &str, capability: Capability) -> VizResult<()> {
        if self.can(user_id, capability) {
            Ok(())
        } else {
            Err(VizError::InvalidStructure(format!(
                "user '{}' lacks capability '{}'",
                user_id,
                capability.slug()
            )))
        }
    }

    /// The user ids that have a given capability, sorted.
    pub fn users_with(&self, capability: Capability) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, role)| role.allows(capability))
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// The number of users with an assigned role.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no users have roles assigned.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Renders the access list as an HTML table (user, role, capabilities).
    pub fn to_html(&self) -> String {
        let mut html = String::from("<table class=\"access-control\">\n");
        html.push_str(
            "  <thead><tr><th>User</th><th>Role</th><th>Capabilities</th></tr></thead>\n",
        );
        html.push_str("  <tbody>\n");
        for (user_id, role) in &self.entries {
            let caps = role
                .capabilities()
                .iter()
                .map(|c| c.slug())
                .collect::<Vec<_>>()
                .join(", ");
            html.push_str(&format!(
                "    <tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                escape_xml(user_id),
                role.label(),
                escape_xml(&caps)
            ));
        }
        html.push_str("  </tbody>\n</table>\n");
        html
    }

    /// Serializes the access list to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("access control to JSON: {}", e)))
    }

    /// Parses an access list from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("access control from JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_capabilities_are_hierarchical() {
        assert!(Role::Owner.allows(Capability::AdministerPermissions));
        assert!(Role::Editor.allows(Capability::Edit));
        assert!(!Role::Editor.allows(Capability::Manage));
        assert!(Role::Commenter.allows(Capability::Comment));
        assert!(!Role::Commenter.allows(Capability::Edit));
        assert!(Role::Viewer.allows(Capability::View));
        assert!(!Role::Viewer.allows(Capability::Comment));
    }

    #[test]
    fn capability_ordering_reflects_privilege() {
        assert!(Capability::View < Capability::Edit);
        assert!(Capability::Edit < Capability::AdministerPermissions);
    }

    #[test]
    fn acl_grant_revoke_and_can() {
        let mut acl = AccessControlList::new();
        assert!(acl.is_empty());
        acl.grant("alice", Role::Owner);
        acl.grant("bob", Role::Viewer);
        assert!(acl.can("alice", Capability::Edit));
        assert!(!acl.can("bob", Capability::Edit));
        // Unknown user has nothing.
        assert!(!acl.can("carol", Capability::View));
        assert_eq!(acl.role_of("bob"), Some(Role::Viewer));
        assert!(acl.revoke("bob"));
        assert!(!acl.revoke("bob"));
        assert_eq!(acl.len(), 1);
    }

    #[test]
    fn grant_user_uses_user_id() {
        let mut acl = AccessControlList::new();
        let user = CollaborativeUser::new("u-1", "Alice", "#ff0000");
        acl.grant_user(&user, Role::Editor);
        assert!(acl.can("u-1", Capability::Edit));
    }

    #[test]
    fn require_reports_missing_capability() {
        let mut acl = AccessControlList::new();
        acl.grant("bob", Role::Viewer);
        let err = acl.require("bob", Capability::Edit).unwrap_err();
        match err {
            VizError::InvalidStructure(msg) => assert!(msg.contains("edit")),
            other => panic!("unexpected error: {:?}", other),
        }
        assert!(acl.require("bob", Capability::View).is_ok());
    }

    #[test]
    fn users_with_capability_is_sorted() {
        let mut acl = AccessControlList::new();
        acl.grant("zoe", Role::Editor);
        acl.grant("amy", Role::Owner);
        acl.grant("bob", Role::Viewer);
        assert_eq!(acl.users_with(Capability::Edit), vec!["amy", "zoe"]);
        assert_eq!(acl.users_with(Capability::View), vec!["amy", "bob", "zoe"]);
    }

    #[test]
    fn acl_html_and_json_round_trip() {
        let mut acl = AccessControlList::new();
        acl.grant("a<b>", Role::Editor);
        let html = acl.to_html();
        assert!(html.contains("a&lt;b&gt;"));
        assert!(html.contains("Editor"));
        let json = acl.to_json().expect("to_json");
        let restored = AccessControlList::from_json(&json).expect("from_json");
        assert_eq!(acl, restored);
    }
}
