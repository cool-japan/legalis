//! Role-Based Access Control (RBAC) for RDF data.
//!
//! This module provides comprehensive access control for RDF knowledge graphs:
//! - Role definition and management
//! - Permission-based access to graphs, resources, and operations
//! - User and group management
//! - Access control policy evaluation

use crate::Triple;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Permission types for RDF operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read triples
    Read,
    /// Write/create triples
    Write,
    /// Delete triples
    Delete,
    /// Execute SPARQL queries
    Query,
    /// Update via SPARQL UPDATE
    Update,
    /// Manage graph structure
    ManageGraph,
    /// Administer access control
    Admin,
}

impl Permission {
    /// Returns all available permissions.
    pub fn all() -> Vec<Permission> {
        vec![
            Permission::Read,
            Permission::Write,
            Permission::Delete,
            Permission::Query,
            Permission::Update,
            Permission::ManageGraph,
            Permission::Admin,
        ]
    }
}

/// A role with a set of permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role ID
    pub id: String,
    /// Role name
    pub name: String,
    /// Description
    pub description: String,
    /// Permissions granted by this role
    pub permissions: HashSet<Permission>,
    /// Is this a system role (cannot be deleted)
    pub system_role: bool,
}

impl Role {
    /// Creates a new role.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            permissions: HashSet::new(),
            system_role: false,
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Adds a permission to the role.
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }

    /// Adds multiple permissions.
    pub fn with_permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.permissions.extend(permissions);
        self
    }

    /// Marks as a system role.
    pub fn as_system_role(mut self) -> Self {
        self.system_role = true;
        self
    }

    /// Checks if the role has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    /// Creates a read-only role.
    pub fn read_only() -> Self {
        Self::new("reader", "Reader")
            .with_description("Read-only access to RDF data")
            .with_permissions(vec![Permission::Read, Permission::Query])
            .as_system_role()
    }

    /// Creates an editor role.
    pub fn editor() -> Self {
        Self::new("editor", "Editor")
            .with_description("Can read and modify RDF data")
            .with_permissions(vec![
                Permission::Read,
                Permission::Write,
                Permission::Query,
                Permission::Update,
            ])
            .as_system_role()
    }

    /// Creates an admin role.
    pub fn admin() -> Self {
        Self::new("admin", "Administrator")
            .with_description("Full access to RDF data and system")
            .with_permissions(Permission::all())
            .as_system_role()
    }
}

/// A user in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Roles assigned to the user
    pub roles: HashSet<String>,
    /// Groups the user belongs to
    pub groups: HashSet<String>,
    /// User is active
    pub active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Creates a new user.
    pub fn new(
        id: impl Into<String>,
        username: impl Into<String>,
        email: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            email: email.into(),
            roles: HashSet::new(),
            groups: HashSet::new(),
            active: true,
            created_at: Utc::now(),
        }
    }

    /// Adds a role to the user.
    pub fn add_role(&mut self, role_id: impl Into<String>) {
        self.roles.insert(role_id.into());
    }

    /// Removes a role from the user.
    pub fn remove_role(&mut self, role_id: &str) {
        self.roles.remove(role_id);
    }

    /// Adds the user to a group.
    pub fn add_to_group(&mut self, group_id: impl Into<String>) {
        self.groups.insert(group_id.into());
    }
}

/// A group of users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// Description
    pub description: String,
    /// Roles assigned to the group
    pub roles: HashSet<String>,
}

impl Group {
    /// Creates a new group.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            roles: HashSet::new(),
        }
    }

    /// Adds a role to the group.
    pub fn add_role(&mut self, role_id: impl Into<String>) {
        self.roles.insert(role_id.into());
    }
}

/// Resource-level access control policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy ID
    pub id: String,
    /// Resource URI pattern (supports wildcards)
    pub resource_pattern: String,
    /// Graph URI (optional, applies to specific graph)
    pub graph_uri: Option<String>,
    /// Permissions granted
    pub permissions: HashSet<Permission>,
    /// Roles this policy applies to
    pub roles: HashSet<String>,
    /// Users this policy applies to (overrides roles)
    pub users: HashSet<String>,
    /// Priority (higher priority policies take precedence)
    pub priority: i32,
}

impl AccessPolicy {
    /// Creates a new access policy.
    pub fn new(id: impl Into<String>, resource_pattern: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            resource_pattern: resource_pattern.into(),
            graph_uri: None,
            permissions: HashSet::new(),
            roles: HashSet::new(),
            users: HashSet::new(),
            priority: 0,
        }
    }

    /// Sets the graph URI.
    pub fn for_graph(mut self, graph_uri: impl Into<String>) -> Self {
        self.graph_uri = Some(graph_uri.into());
        self
    }

    /// Adds a permission.
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }

    /// Adds a role.
    pub fn for_role(mut self, role_id: impl Into<String>) -> Self {
        self.roles.insert(role_id.into());
        self
    }

    /// Adds a user.
    pub fn for_user(mut self, user_id: impl Into<String>) -> Self {
        self.users.insert(user_id.into());
        self
    }

    /// Sets priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Checks if a resource matches this policy's pattern.
    pub fn matches_resource(&self, resource_uri: &str) -> bool {
        if self.resource_pattern.contains('*') {
            // Simple wildcard matching
            let pattern = self.resource_pattern.replace('*', "");
            resource_uri.contains(&pattern)
        } else {
            resource_uri == self.resource_pattern
        }
    }
}

/// RBAC manager for the knowledge graph.
pub struct RbacManager {
    /// All roles
    roles: HashMap<String, Role>,
    /// All users
    users: HashMap<String, User>,
    /// All groups
    groups: HashMap<String, Group>,
    /// Access policies
    policies: Vec<AccessPolicy>,
}

impl RbacManager {
    /// Creates a new RBAC manager with default roles.
    pub fn new() -> Self {
        let mut roles = HashMap::new();

        let reader = Role::read_only();
        let editor = Role::editor();
        let admin = Role::admin();

        roles.insert(reader.id.clone(), reader);
        roles.insert(editor.id.clone(), editor);
        roles.insert(admin.id.clone(), admin);

        Self {
            roles,
            users: HashMap::new(),
            groups: HashMap::new(),
            policies: Vec::new(),
        }
    }

    /// Adds a role.
    pub fn add_role(&mut self, role: Role) -> Result<(), String> {
        if self.roles.contains_key(&role.id) {
            return Err(format!("Role {} already exists", role.id));
        }
        self.roles.insert(role.id.clone(), role);
        Ok(())
    }

    /// Removes a role.
    pub fn remove_role(&mut self, role_id: &str) -> Result<(), String> {
        if let Some(role) = self.roles.get(role_id) {
            if role.system_role {
                return Err("Cannot remove system role".to_string());
            }
        }

        self.roles.remove(role_id);
        Ok(())
    }

    /// Adds a user.
    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.id) {
            return Err(format!("User {} already exists", user.id));
        }
        self.users.insert(user.id.clone(), user);
        Ok(())
    }

    /// Gets a user.
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    /// Gets a mutable user.
    pub fn get_user_mut(&mut self, user_id: &str) -> Option<&mut User> {
        self.users.get_mut(user_id)
    }

    /// Adds a group.
    pub fn add_group(&mut self, group: Group) -> Result<(), String> {
        if self.groups.contains_key(&group.id) {
            return Err(format!("Group {} already exists", group.id));
        }
        self.groups.insert(group.id.clone(), group);
        Ok(())
    }

    /// Adds an access policy.
    pub fn add_policy(&mut self, policy: AccessPolicy) {
        self.policies.push(policy);
        self.policies.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Checks if a user has a specific permission for a resource.
    pub fn check_permission(
        &self,
        user_id: &str,
        resource_uri: &str,
        permission: Permission,
        graph_uri: Option<&str>,
    ) -> bool {
        let user = match self.users.get(user_id) {
            Some(u) if u.active => u,
            _ => return false,
        };

        // Collect all roles for the user (direct + from groups)
        let mut all_roles = user.roles.clone();
        for group_id in &user.groups {
            if let Some(group) = self.groups.get(group_id) {
                all_roles.extend(group.roles.clone());
            }
        }

        // Check policies (higher priority first)
        for policy in &self.policies {
            // Check if policy applies to this graph
            if let Some(ref policy_graph) = policy.graph_uri {
                if let Some(requested_graph) = graph_uri {
                    if policy_graph != requested_graph {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // Check if resource matches
            if !policy.matches_resource(resource_uri) {
                continue;
            }

            // Check if user or role matches
            let applies_to_user = policy.users.contains(user_id);
            let applies_to_role = policy.roles.iter().any(|r| all_roles.contains(r));

            if applies_to_user || applies_to_role {
                // Check if permission is granted
                if policy.permissions.contains(&permission) {
                    return true;
                }
            }
        }

        // Check role permissions (fallback)
        for role_id in &all_roles {
            if let Some(role) = self.roles.get(role_id) {
                if role.has_permission(permission) {
                    return true;
                }
            }
        }

        false
    }

    /// Checks if a user can read a triple.
    pub fn can_read_triple(&self, user_id: &str, triple: &Triple, graph_uri: Option<&str>) -> bool {
        self.check_permission(user_id, &triple.subject, Permission::Read, graph_uri)
    }

    /// Filters triples based on read permissions.
    pub fn filter_readable_triples(
        &self,
        user_id: &str,
        triples: &[Triple],
        graph_uri: Option<&str>,
    ) -> Vec<Triple> {
        triples
            .iter()
            .filter(|t| self.can_read_triple(user_id, t, graph_uri))
            .cloned()
            .collect()
    }
}

impl Default for RbacManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;

    #[test]
    fn test_role_creation() {
        let role = Role::new("custom", "Custom Role")
            .with_description("A custom role")
            .with_permission(Permission::Read)
            .with_permission(Permission::Query);

        assert_eq!(role.id, "custom");
        assert_eq!(role.permissions.len(), 2);
        assert!(role.has_permission(Permission::Read));
        assert!(!role.has_permission(Permission::Write));
    }

    #[test]
    fn test_system_roles() {
        let reader = Role::read_only();
        assert!(reader.system_role);
        assert!(reader.has_permission(Permission::Read));
        assert!(!reader.has_permission(Permission::Write));

        let editor = Role::editor();
        assert!(editor.has_permission(Permission::Write));

        let admin = Role::admin();
        assert_eq!(admin.permissions.len(), Permission::all().len());
    }

    #[test]
    fn test_user_creation() {
        let mut user = User::new("user1", "testuser", "test@example.com");
        assert_eq!(user.id, "user1");
        assert!(user.active);

        user.add_role("reader");
        assert!(user.roles.contains("reader"));

        user.remove_role("reader");
        assert!(!user.roles.contains("reader"));
    }

    #[test]
    fn test_group_creation() {
        let mut group = Group::new("group1", "Test Group");
        group.add_role("editor");

        assert_eq!(group.id, "group1");
        assert!(group.roles.contains("editor"));
    }

    #[test]
    fn test_access_policy() {
        let policy = AccessPolicy::new("policy1", "http://example.org/*")
            .with_permission(Permission::Read)
            .for_role("reader")
            .with_priority(10);

        assert_eq!(policy.priority, 10);
        assert!(policy.matches_resource("http://example.org/resource1"));
        assert!(!policy.matches_resource("http://other.org/resource1"));
    }

    #[test]
    fn test_rbac_manager() {
        let mut manager = RbacManager::new();

        // Add user
        let mut user = User::new("user1", "testuser", "test@example.com");
        user.add_role("reader");
        manager.add_user(user).unwrap();

        assert!(manager.get_user("user1").is_some());

        // Try to add duplicate
        let user2 = User::new("user1", "duplicate", "dup@example.com");
        assert!(manager.add_user(user2).is_err());
    }

    #[test]
    fn test_permission_check() {
        let mut manager = RbacManager::new();

        let mut user = User::new("user1", "testuser", "test@example.com");
        user.add_role("reader");
        manager.add_user(user).unwrap();

        // Reader role has read permission
        assert!(manager.check_permission(
            "user1",
            "http://example.org/resource",
            Permission::Read,
            None
        ));

        // Reader role doesn't have write permission
        assert!(!manager.check_permission(
            "user1",
            "http://example.org/resource",
            Permission::Write,
            None
        ));
    }

    #[test]
    fn test_policy_based_access() {
        let mut manager = RbacManager::new();

        let mut user = User::new("user1", "testuser", "test@example.com");
        user.add_role("reader");
        manager.add_user(user).unwrap();

        // Add policy that grants write access to specific resource
        let policy = AccessPolicy::new("policy1", "http://example.org/special")
            .with_permission(Permission::Write)
            .for_user("user1");

        manager.add_policy(policy);

        // User should have write access to the special resource
        assert!(manager.check_permission(
            "user1",
            "http://example.org/special",
            Permission::Write,
            None
        ));

        // But not to other resources
        assert!(!manager.check_permission(
            "user1",
            "http://example.org/other",
            Permission::Write,
            None
        ));
    }

    #[test]
    fn test_group_permissions() {
        let mut manager = RbacManager::new();

        // Create group with editor role
        let mut group = Group::new("editors", "Editors Group");
        group.add_role("editor");
        manager.add_group(group).unwrap();

        // Create user and add to group
        let mut user = User::new("user1", "testuser", "test@example.com");
        user.add_to_group("editors");
        manager.add_user(user).unwrap();

        // User should have editor permissions through group
        assert!(manager.check_permission(
            "user1",
            "http://example.org/resource",
            Permission::Write,
            None
        ));
    }

    #[test]
    fn test_filter_readable_triples() {
        let mut manager = RbacManager::new();

        let mut user = User::new("user1", "testuser", "test@example.com");
        user.add_role("reader");
        manager.add_user(user).unwrap();

        let triples = vec![
            Triple {
                subject: "http://example.org/public".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("owl:Class".to_string()),
            },
            Triple {
                subject: "http://example.org/private".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("owl:Class".to_string()),
            },
        ];

        // All triples should be readable by default with reader role
        let readable = manager.filter_readable_triples("user1", &triples, None);
        assert_eq!(readable.len(), 2);
    }

    #[test]
    fn test_system_role_protection() {
        let mut manager = RbacManager::new();

        // Try to remove system role
        assert!(manager.remove_role("admin").is_err());
        assert!(manager.remove_role("reader").is_err());
    }

    #[test]
    fn test_inactive_user() {
        let mut manager = RbacManager::new();

        let mut user = User::new("user1", "testuser", "test@example.com");
        user.add_role("admin");
        user.active = false;
        manager.add_user(user).unwrap();

        // Inactive user should not have any permissions
        assert!(!manager.check_permission(
            "user1",
            "http://example.org/resource",
            Permission::Read,
            None
        ));
    }
}
