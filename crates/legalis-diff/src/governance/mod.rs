//! Enterprise governance: identity, access control, compliance and audit (v0.5.6).
//!
//! This module groups the *enterprise* concerns that wrap statute-diff
//! operations rather than computing them. It is intentionally distinct from the
//! existing [`crate::enterprise`] (diff archiving / basic role checks),
//! [`crate::security`] (signing / encryption) and [`crate::compliance`]
//! (regulatory *change* impact) modules — here the subject is the *system's*
//! identity, authorization and compliance posture. Everything is pure Rust and
//! self-contained:
//!
//! - [`sso`] — single sign-on: SAML 2.0 / OpenID-Connect token modelling and
//!   HMAC-SHA256 token validation behind a pluggable [`sso::SsoProvider`].
//! - [`directory`] — LDAP / Active-Directory directory model: distinguished
//!   names, RFC 4515 filters, bind/search and transitive group resolution
//!   behind a pluggable [`directory::DirectoryService`].
//! - [`rbac`] — advanced role-based access control: hierarchical roles with
//!   inheritance, resource-pattern permissions, ABAC-style conditions and
//!   deny-override resolution.
//! - [`compliance_report`] — control catalogues and report generation for
//!   SOC 2, GDPR and HIPAA.
//! - [`audit_log`] — tamper-evident, hash-chained enterprise audit logs with
//!   retention policies and legal holds.
//!
//! # Deferred external bindings
//!
//! Talking to a *live* identity provider over HTTP (SAML redirect/POST bindings,
//! OIDC discovery + JWKS, RSA signature verification) or to a *live* LDAP/AD
//! server over the network requires services this offline workspace does not
//! have. Those bindings are abstracted behind the [`sso::SsoProvider`] and
//! [`directory::DirectoryService`] traits; pure-Rust in-memory backends
//! ([`sso::InMemoryIdentityProvider`], [`directory::InMemoryDirectory`])
//! implement the full validation/bind/search workflow locally, so a networked
//! backend can be added later without touching callers.
//!
//! # Example
//!
//! ```
//! use legalis_diff::governance::{Principal, rbac::{RbacEngine, Role, Permission, RequestContext}};
//!
//! // A subject authenticated via SSO/LDAP, carrying group memberships.
//! let principal = Principal::new("alice")
//!     .with_display_name("Alice Counsel")
//!     .with_group("legal-editors");
//!
//! // Map the group to a role that may write tax statutes.
//! let mut rbac = RbacEngine::new();
//! rbac.add_role(Role::new("editor").allow("diff:write", "statute:tax-*"));
//! rbac.assign_group_role("legal-editors", "editor");
//!
//! let ctx = RequestContext::from_principal(&principal);
//! assert!(rbac.is_allowed(&ctx, "diff:write", "statute:tax-2026"));
//! assert!(!rbac.is_allowed(&ctx, "diff:write", "statute:labour-2026"));
//! ```

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub mod audit_log;
pub mod compliance_report;
pub mod directory;
pub mod rbac;
pub mod sso;

pub use audit_log::{
    AuditEvent, AuditOutcome, AuditQuery, AuditSeverity, EnterpriseAuditEntry, EnterpriseAuditLog,
    IntegrityReport, PurgeSummary, RetentionPolicy,
};
pub use compliance_report::{
    ComplianceAssessment, ComplianceFramework, ComplianceReport, Control, ControlAssessment,
    ControlStatus, Finding, SecurityPosture, generate_report,
};
pub use directory::{
    DirectoryEntry, DirectoryService, DistinguishedName, InMemoryDirectory, LdapFilter, SearchScope,
};
pub use rbac::{Condition, Effect, Permission, RbacEngine, RequestContext, ResourcePattern, Role};
pub use sso::{
    InMemoryIdentityProvider, OidcIdToken, SamlAssertion, SsoProtocol, SsoProvider, SsoSession,
    SsoToken,
};

/// Computes a lowercase hex SHA-256 digest over a single byte slice.
pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Computes a lowercase hex SHA-256 digest over several byte slices.
///
/// Each part is length-prefixed before hashing so that, for example,
/// `["ab", "c"]` and `["a", "bc"]` produce different digests (domain separation
/// against trivial concatenation collisions).
pub(crate) fn sha256_parts(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    hex::encode(hasher.finalize())
}

/// Wildcard glob match supporting `*` (matches any sequence, including empty),
/// case-sensitive. Used for RBAC action/resource patterns.
///
/// This is an iterative two-pointer matcher with back-tracking, so it never
/// recurses and runs in O(pattern × text) worst case without allocation beyond
/// the character buffers.
pub(crate) fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0usize, 0usize);
    let mut star: Option<usize> = None;
    let mut mark = 0usize;
    while ti < t.len() {
        if pi < p.len() && p[pi] == t[ti] {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some(pi);
            mark = ti;
            pi += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            mark += 1;
            ti = mark;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
}

/// Case-insensitive variant of [`glob_match`] (used for LDAP attribute matching,
/// whose equality matching rules are case-insensitive by default).
pub(crate) fn glob_match_ci(pattern: &str, text: &str) -> bool {
    glob_match(&pattern.to_lowercase(), &text.to_lowercase())
}

/// An authenticated identity, produced by [`sso`] or [`directory`] and consumed
/// by [`rbac`] and [`audit_log`].
///
/// A principal is the canonical, protocol-independent representation of "who is
/// acting": a stable [`Principal::subject`] identifier, a human-readable
/// display name, an optional email, the groups the subject belongs to (used by
/// [`rbac`] to derive roles) and arbitrary attributes (used by ABAC
/// [`rbac::Condition`]s).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Principal {
    /// Stable unique subject identifier (e.g. SAML `NameID`, OIDC `sub`, LDAP `uid`).
    pub subject: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Optional email address.
    pub email: Option<String>,
    /// Group memberships (roles are derived from these by [`rbac`]).
    pub groups: Vec<String>,
    /// Arbitrary attributes for ABAC decisions, kept ordered for determinism.
    pub attributes: BTreeMap<String, String>,
}

impl Principal {
    /// Creates a principal with the given subject; the display name defaults to
    /// the subject and can be overridden with [`Principal::with_display_name`].
    pub fn new(subject: impl Into<String>) -> Self {
        let subject = subject.into();
        Self {
            display_name: subject.clone(),
            subject,
            email: None,
            groups: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the display name.
    #[must_use]
    pub fn with_display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = name.into();
        self
    }

    /// Sets the email address.
    #[must_use]
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Adds a group membership (deduplicated).
    #[must_use]
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        let group = group.into();
        if !self.groups.contains(&group) {
            self.groups.push(group);
        }
        self
    }

    /// Adds an attribute.
    #[must_use]
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Returns `true` if the principal belongs to `group`.
    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.iter().any(|g| g == group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hex_deterministic_and_hex() {
        let a = sha256_hex(b"legalis");
        let b = sha256_hex(b"legalis");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sha256_parts_domain_separation() {
        assert_ne!(sha256_parts(&[b"ab", b"c"]), sha256_parts(&[b"a", b"bc"]));
    }

    #[test]
    fn test_glob_match_basic() {
        assert!(glob_match("diff:*", "diff:read"));
        assert!(glob_match("*", "anything"));
        assert!(glob_match("statute:tax-*", "statute:tax-2026"));
        assert!(!glob_match("statute:tax-*", "statute:labour-2026"));
        assert!(glob_match("a*c", "abbbc"));
        assert!(!glob_match("a*c", "abbbd"));
        assert!(glob_match("exact", "exact"));
        assert!(!glob_match("exact", "exactly"));
    }

    #[test]
    fn test_glob_match_ci() {
        assert!(glob_match_ci("UID", "uid"));
        assert!(glob_match_ci("Person*", "personFull"));
        assert!(!glob_match_ci("admin", "administrator"));
    }

    #[test]
    fn test_principal_builder() {
        let p = Principal::new("alice")
            .with_display_name("Alice Q.")
            .with_email("alice@example.com")
            .with_group("editors")
            .with_group("editors") // dedup
            .with_attribute("department", "legal");
        assert_eq!(p.subject, "alice");
        assert_eq!(p.display_name, "Alice Q.");
        assert_eq!(p.email.as_deref(), Some("alice@example.com"));
        assert_eq!(p.groups, vec!["editors".to_string()]);
        assert!(p.is_in_group("editors"));
        assert!(!p.is_in_group("admins"));
        assert_eq!(
            p.attributes.get("department").map(String::as_str),
            Some("legal")
        );
    }
}
