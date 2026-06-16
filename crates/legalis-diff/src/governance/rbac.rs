//! Advanced role-based access control (RBAC) with hierarchy and ABAC conditions.
//!
//! This goes well beyond the flat `Role` enums in [`crate::enterprise`] and
//! [`crate::security`]:
//!
//! - **Hierarchical roles**: a [`Role`] can inherit from parent roles
//!   (transitively, cycle-safe).
//! - **Resource-pattern permissions**: a [`Permission`] grants an action over a
//!   glob [`ResourcePattern`] (e.g. `statute:tax-*`).
//! - **ABAC conditions**: a permission can carry a [`Condition`] evaluated
//!   against the request's subject attributes, group memberships and
//!   environment.
//! - **Deny-override**: an explicit [`Effect::Deny`] always beats an allow, and
//!   the default decision is deny.
//! - **Group → role mapping**: groups carried by an SSO/LDAP [`Principal`] are
//!   mapped to roles, so directory membership drives authorization.
//!
//! # Example
//!
//! ```
//! use legalis_diff::governance::rbac::{RbacEngine, Role, RequestContext};
//!
//! let mut rbac = RbacEngine::new();
//! rbac.add_role(Role::new("reader").allow("diff:read", "statute:*"));
//! rbac.add_role(
//!     Role::new("editor")
//!         .with_parent("reader") // inherits diff:read
//!         .allow("diff:write", "statute:tax-*"),
//! );
//! rbac.assign_role("alice", "editor");
//!
//! let ctx = RequestContext::new("alice");
//! assert!(rbac.is_allowed(&ctx, "diff:read", "statute:any"));    // inherited
//! assert!(rbac.is_allowed(&ctx, "diff:write", "statute:tax-99"));
//! assert!(!rbac.is_allowed(&ctx, "diff:write", "statute:labour"));
//! ```

use crate::governance::{Principal, glob_match};
use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Whether a permission grants or denies access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    /// Grant access (subject to conditions and deny-override).
    Allow,
    /// Explicitly deny access (always wins).
    Deny,
}

/// A glob pattern over resource identifiers (`*` matches any sequence).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePattern(String);

impl ResourcePattern {
    /// Creates a pattern from a string.
    pub fn new(pattern: impl Into<String>) -> Self {
        Self(pattern.into())
    }

    /// Returns `true` if `resource` matches this pattern.
    pub fn matches(&self, resource: &str) -> bool {
        glob_match(&self.0, resource)
    }

    /// The underlying pattern string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ResourcePattern {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for ResourcePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// An attribute-based condition gating a permission (ABAC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    /// The subject attribute `key` equals `value`.
    AttributeEquals { key: String, value: String },
    /// The subject attribute `key` is present.
    AttributePresent(String),
    /// The subject belongs to `group`.
    InGroup(String),
    /// The environment attribute `key` equals `value`.
    EnvironmentEquals { key: String, value: String },
    /// All sub-conditions hold.
    All(Vec<Condition>),
    /// At least one sub-condition holds.
    Any(Vec<Condition>),
    /// The sub-condition does not hold.
    Not(Box<Condition>),
}

impl Condition {
    /// Evaluates the condition against a request context.
    pub fn evaluate(&self, ctx: &RequestContext) -> bool {
        match self {
            Self::AttributeEquals { key, value } => {
                ctx.attributes.get(key).map(String::as_str) == Some(value.as_str())
            }
            Self::AttributePresent(key) => ctx.attributes.contains_key(key),
            Self::InGroup(group) => ctx.groups.iter().any(|g| g == group),
            Self::EnvironmentEquals { key, value } => {
                ctx.environment.get(key).map(String::as_str) == Some(value.as_str())
            }
            Self::All(subs) => subs.iter().all(|c| c.evaluate(ctx)),
            Self::Any(subs) => subs.iter().any(|c| c.evaluate(ctx)),
            Self::Not(inner) => !inner.evaluate(ctx),
        }
    }
}

/// A single permission: an action over a resource pattern, with an effect and an
/// optional condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission {
    /// The action (glob pattern, e.g. `diff:read` or `diff:*`).
    pub action: String,
    /// The resource pattern the action applies to.
    pub resource: ResourcePattern,
    /// Whether this allows or denies.
    pub effect: Effect,
    /// Optional ABAC condition that must hold for the permission to apply.
    pub condition: Option<Condition>,
}

impl Permission {
    /// Creates an allow permission.
    pub fn allow(action: impl Into<String>, resource: impl Into<ResourcePattern>) -> Self {
        Self {
            action: action.into(),
            resource: resource.into(),
            effect: Effect::Allow,
            condition: None,
        }
    }

    /// Creates a deny permission.
    pub fn deny(action: impl Into<String>, resource: impl Into<ResourcePattern>) -> Self {
        Self {
            action: action.into(),
            resource: resource.into(),
            effect: Effect::Deny,
            condition: None,
        }
    }

    /// Attaches a condition to the permission.
    #[must_use]
    pub fn when(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Returns `true` if this permission applies to the given request.
    pub fn applies(&self, action: &str, resource: &str, ctx: &RequestContext) -> bool {
        glob_match(&self.action, action)
            && self.resource.matches(resource)
            && self
                .condition
                .as_ref()
                .map(|c| c.evaluate(ctx))
                .unwrap_or(true)
    }
}

/// A named role with optional parents and a set of permissions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    /// Unique role name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Parent role names this role inherits from.
    pub parents: Vec<String>,
    /// Permissions granted directly by this role.
    pub permissions: Vec<Permission>,
}

impl Role {
    /// Creates an empty role.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            parents: Vec::new(),
            permissions: Vec::new(),
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Adds a parent role to inherit from.
    #[must_use]
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parents.push(parent.into());
        self
    }

    /// Adds an allow permission (builder form).
    #[must_use]
    pub fn allow(
        mut self,
        action: impl Into<String>,
        resource: impl Into<ResourcePattern>,
    ) -> Self {
        self.permissions.push(Permission::allow(action, resource));
        self
    }

    /// Adds a deny permission (builder form).
    #[must_use]
    pub fn deny(mut self, action: impl Into<String>, resource: impl Into<ResourcePattern>) -> Self {
        self.permissions.push(Permission::deny(action, resource));
        self
    }

    /// Adds a conditional allow permission (builder form).
    #[must_use]
    pub fn allow_when(
        mut self,
        action: impl Into<String>,
        resource: impl Into<ResourcePattern>,
        condition: Condition,
    ) -> Self {
        self.permissions
            .push(Permission::allow(action, resource).when(condition));
        self
    }

    /// Adds an arbitrary permission (builder form).
    #[must_use]
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.push(permission);
        self
    }
}

/// The context of an access request: who is acting and under what attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestContext {
    /// The acting subject identifier.
    pub subject: String,
    /// The subject's group memberships (drive group→role mapping).
    pub groups: Vec<String>,
    /// Subject attributes for ABAC conditions.
    pub attributes: BTreeMap<String, String>,
    /// Environment attributes for ABAC conditions (e.g. time, IP).
    pub environment: BTreeMap<String, String>,
}

impl RequestContext {
    /// Creates a context for the given subject.
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            groups: Vec::new(),
            attributes: BTreeMap::new(),
            environment: BTreeMap::new(),
        }
    }

    /// Builds a context from an authenticated [`Principal`].
    pub fn from_principal(principal: &Principal) -> Self {
        Self {
            subject: principal.subject.clone(),
            groups: principal.groups.clone(),
            attributes: principal.attributes.clone(),
            environment: BTreeMap::new(),
        }
    }

    /// Adds a group membership.
    #[must_use]
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    /// Adds a subject attribute.
    #[must_use]
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Adds an environment attribute.
    #[must_use]
    pub fn with_environment(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }
}

/// The RBAC engine: a registry of roles plus subject/group assignments.
#[derive(Debug, Clone, Default)]
pub struct RbacEngine {
    roles: HashMap<String, Role>,
    subject_roles: HashMap<String, BTreeSet<String>>,
    group_roles: HashMap<String, BTreeSet<String>>,
}

impl RbacEngine {
    /// Creates an empty engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds (or replaces) a role.
    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role.name.clone(), role);
    }

    /// Returns a role by name.
    pub fn get_role(&self, name: &str) -> Option<&Role> {
        self.roles.get(name)
    }

    /// The number of registered roles.
    pub fn role_count(&self) -> usize {
        self.roles.len()
    }

    /// Assigns a role directly to a subject.
    pub fn assign_role(&mut self, subject: impl Into<String>, role: impl Into<String>) {
        self.subject_roles
            .entry(subject.into())
            .or_default()
            .insert(role.into());
    }

    /// Maps a group to a role (so group members gain the role).
    pub fn assign_group_role(&mut self, group: impl Into<String>, role: impl Into<String>) {
        self.group_roles
            .entry(group.into())
            .or_default()
            .insert(role.into());
    }

    /// Computes the transitive set of role names that apply to a request,
    /// expanding direct assignments, group mappings and parent inheritance.
    pub fn effective_roles(&self, ctx: &RequestContext) -> BTreeSet<String> {
        let mut seeds: BTreeSet<String> = BTreeSet::new();
        if let Some(direct) = self.subject_roles.get(&ctx.subject) {
            seeds.extend(direct.iter().cloned());
        }
        for group in &ctx.groups {
            if let Some(mapped) = self.group_roles.get(group) {
                seeds.extend(mapped.iter().cloned());
            }
        }
        let mut resolved: BTreeSet<String> = BTreeSet::new();
        for seed in seeds {
            self.expand_role(&seed, &mut resolved);
        }
        resolved
    }

    fn expand_role(&self, name: &str, resolved: &mut BTreeSet<String>) {
        if !resolved.insert(name.to_string()) {
            return; // already visited (cycle-safe)
        }
        if let Some(role) = self.roles.get(name) {
            for parent in &role.parents {
                self.expand_role(parent, resolved);
            }
        }
    }

    /// Returns every effective permission for the request (for introspection).
    pub fn effective_permissions(&self, ctx: &RequestContext) -> Vec<Permission> {
        let mut perms = Vec::new();
        for role_name in self.effective_roles(ctx) {
            if let Some(role) = self.roles.get(&role_name) {
                perms.extend(role.permissions.iter().cloned());
            }
        }
        perms
    }

    /// Decides whether the request is allowed.
    ///
    /// Resolution: gather every effective permission; if any matching [`Effect::Deny`]
    /// applies the request is denied (deny-override); otherwise it is allowed iff
    /// at least one matching [`Effect::Allow`] applies; the default is deny.
    pub fn is_allowed(&self, ctx: &RequestContext, action: &str, resource: &str) -> bool {
        let mut allow = false;
        for role_name in self.effective_roles(ctx) {
            let Some(role) = self.roles.get(&role_name) else {
                continue;
            };
            for perm in &role.permissions {
                if perm.applies(action, resource, ctx) {
                    match perm.effect {
                        Effect::Deny => return false,
                        Effect::Allow => allow = true,
                    }
                }
            }
        }
        allow
    }

    /// Enforces an access decision, returning an error if denied.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::AccessDenied`] if the request is not allowed.
    pub fn authorize(&self, ctx: &RequestContext, action: &str, resource: &str) -> DiffResult<()> {
        if self.is_allowed(ctx, action, resource) {
            Ok(())
        } else {
            Err(DiffError::AccessDenied(format!(
                "subject '{}' may not '{action}' on '{resource}'",
                ctx.subject
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_pattern_matching() {
        let p = ResourcePattern::new("statute:tax-*");
        assert!(p.matches("statute:tax-2026"));
        assert!(!p.matches("statute:labour"));
        assert_eq!(p.as_str(), "statute:tax-*");
        assert_eq!(ResourcePattern::from("*").to_string(), "*");
    }

    #[test]
    fn test_role_inheritance() {
        let mut rbac = RbacEngine::new();
        rbac.add_role(Role::new("reader").allow("diff:read", "statute:*"));
        rbac.add_role(
            Role::new("editor")
                .with_parent("reader")
                .allow("diff:write", "statute:*"),
        );
        rbac.assign_role("alice", "editor");
        let ctx = RequestContext::new("alice");
        assert!(rbac.is_allowed(&ctx, "diff:read", "statute:x")); // inherited
        assert!(rbac.is_allowed(&ctx, "diff:write", "statute:x"));
        assert!(!rbac.is_allowed(&ctx, "diff:delete", "statute:x")); // default deny
    }

    #[test]
    fn test_resource_scope_restriction() {
        let mut rbac = RbacEngine::new();
        rbac.add_role(Role::new("tax-editor").allow("diff:write", "statute:tax-*"));
        rbac.assign_role("bob", "tax-editor");
        let ctx = RequestContext::new("bob");
        assert!(rbac.is_allowed(&ctx, "diff:write", "statute:tax-2026"));
        assert!(!rbac.is_allowed(&ctx, "diff:write", "statute:labour-2026"));
    }

    #[test]
    fn test_deny_override() {
        let mut rbac = RbacEngine::new();
        rbac.add_role(
            Role::new("contractor")
                .allow("diff:*", "statute:*")
                .deny("diff:delete", "statute:*"),
        );
        rbac.assign_role("carol", "contractor");
        let ctx = RequestContext::new("carol");
        assert!(rbac.is_allowed(&ctx, "diff:write", "statute:x"));
        assert!(!rbac.is_allowed(&ctx, "diff:delete", "statute:x")); // explicit deny wins
    }

    #[test]
    fn test_group_to_role_mapping() {
        let mut rbac = RbacEngine::new();
        rbac.add_role(Role::new("editor").allow("diff:write", "statute:*"));
        rbac.assign_group_role("legal-editors", "editor");
        let principal = Principal::new("dave").with_group("legal-editors");
        let ctx = RequestContext::from_principal(&principal);
        assert!(rbac.is_allowed(&ctx, "diff:write", "statute:x"));

        let outsider = RequestContext::new("dave"); // no groups
        assert!(!rbac.is_allowed(&outsider, "diff:write", "statute:x"));
    }

    #[test]
    fn test_abac_condition() {
        let mut rbac = RbacEngine::new();
        rbac.add_role(Role::new("dept-editor").allow_when(
            "diff:write",
            "statute:*",
            Condition::AttributeEquals {
                key: "department".to_string(),
                value: "legal".to_string(),
            },
        ));
        rbac.assign_role("erin", "dept-editor");

        let legal = RequestContext::new("erin").with_attribute("department", "legal");
        assert!(rbac.is_allowed(&legal, "diff:write", "statute:x"));

        let finance = RequestContext::new("erin").with_attribute("department", "finance");
        assert!(!rbac.is_allowed(&finance, "diff:write", "statute:x"));
    }

    #[test]
    fn test_composite_condition() {
        let cond = Condition::All(vec![
            Condition::InGroup("reviewers".to_string()),
            Condition::Not(Box::new(Condition::EnvironmentEquals {
                key: "network".to_string(),
                value: "public".to_string(),
            })),
        ]);
        let ok = RequestContext::new("f")
            .with_group("reviewers")
            .with_environment("network", "internal");
        let bad = RequestContext::new("f")
            .with_group("reviewers")
            .with_environment("network", "public");
        assert!(cond.evaluate(&ok));
        assert!(!cond.evaluate(&bad));
    }

    #[test]
    fn test_authorize_error_and_cycle_safety() {
        let mut rbac = RbacEngine::new();
        // Mutually-recursive roles must not loop forever.
        rbac.add_role(Role::new("a").with_parent("b").allow("read", "x"));
        rbac.add_role(Role::new("b").with_parent("a").allow("write", "x"));
        rbac.assign_role("z", "a");
        let ctx = RequestContext::new("z");
        assert_eq!(rbac.effective_roles(&ctx).len(), 2);
        assert!(rbac.authorize(&ctx, "read", "x").is_ok());
        assert!(rbac.authorize(&ctx, "write", "x").is_ok());
        assert!(matches!(
            rbac.authorize(&ctx, "delete", "x"),
            Err(DiffError::AccessDenied(_))
        ));
        assert_eq!(rbac.role_count(), 2);
        assert!(!rbac.effective_permissions(&ctx).is_empty());
        assert!(rbac.get_role("a").is_some());
    }
}
