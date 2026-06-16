//! LDAP / Active-Directory support: directory model, filters and group resolution.
//!
//! This module provides a pure-Rust model of an LDAP/AD directory:
//! [`DistinguishedName`] parsing, [`DirectoryEntry`] objects with multi-valued
//! attributes, an [RFC 4515](https://www.rfc-editor.org/rfc/rfc4515) search
//! [`LdapFilter`] parser/matcher, password [`DirectoryService::bind`]
//! authentication, scoped [`DirectoryService::search`], and **transitive group
//! resolution** (nested `memberOf`/`member`, as used by Active Directory).
//!
//! The [`DirectoryService`] trait abstracts the directory. [`InMemoryDirectory`]
//! implements it entirely in memory; a production binding (a networked LDAP
//! client) can implement the same trait without changing callers. That
//! networked binding is intentionally deferred.
//!
//! # Example
//!
//! ```
//! use legalis_diff::governance::directory::{
//!     DirectoryEntry, DirectoryService, InMemoryDirectory, LdapFilter, SearchScope,
//! };
//!
//! let mut dir = InMemoryDirectory::new();
//! dir.add_entry(
//!     DirectoryEntry::new("uid=alice,ou=people,dc=example,dc=com")
//!         .with_object_class("inetOrgPerson")
//!         .with_attr("uid", "alice")
//!         .with_attr("cn", "Alice Counsel")
//!         .with_attr("mail", "alice@example.com"),
//! );
//! dir.set_password("uid=alice,ou=people,dc=example,dc=com", "s3cret");
//!
//! assert!(dir.bind("uid=alice,ou=people,dc=example,dc=com", "s3cret").is_ok());
//! let filter = LdapFilter::parse("(uid=alice)").unwrap();
//! let hits = dir.search("dc=example,dc=com", SearchScope::Subtree, &filter).unwrap();
//! assert_eq!(hits.len(), 1);
//! ```

use crate::governance::{Principal, glob_match_ci};
use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

/// A parsed distinguished name (a sequence of relative DN components).
///
/// Parsing is the common, comma-separated form (e.g.
/// `uid=alice,ou=people,dc=example,dc=com`). Escaped commas inside values are
/// not interpreted — a documented simplification of the full RFC 4514 grammar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistinguishedName {
    /// Relative components as `(attribute, value)` pairs, most-specific first.
    pub components: Vec<(String, String)>,
}

impl DistinguishedName {
    /// Parses a DN string.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::DirectoryError`] if a component is missing an `=`.
    pub fn parse(dn: &str) -> DiffResult<Self> {
        let mut components = Vec::new();
        for raw in dn.split(',') {
            let part = raw.trim();
            if part.is_empty() {
                continue;
            }
            let (attr, value) = part.split_once('=').ok_or_else(|| {
                DiffError::DirectoryError(format!("invalid DN component '{part}'"))
            })?;
            components.push((attr.trim().to_lowercase(), value.trim().to_string()));
        }
        if components.is_empty() {
            return Err(DiffError::DirectoryError(
                "empty distinguished name".to_string(),
            ));
        }
        Ok(Self { components })
    }

    /// The normalized canonical form (attributes lowercased), used as a map key.
    pub fn normalized(&self) -> String {
        self.components
            .iter()
            .map(|(a, v)| format!("{a}={}", v.to_lowercase()))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// The most-specific relative DN component (e.g. `uid=alice`).
    pub fn rdn(&self) -> Option<String> {
        self.components.first().map(|(a, v)| format!("{a}={v}"))
    }

    /// The parent DN (everything but the first component), if any.
    pub fn parent(&self) -> Option<DistinguishedName> {
        if self.components.len() <= 1 {
            None
        } else {
            Some(DistinguishedName {
                components: self.components[1..].to_vec(),
            })
        }
    }

    /// Returns `true` if `self` is `other` or a descendant of `other`.
    pub fn is_within(&self, other: &DistinguishedName) -> bool {
        if self.components.len() < other.components.len() {
            return false;
        }
        let offset = self.components.len() - other.components.len();
        self.components[offset..]
            .iter()
            .zip(&other.components)
            .all(|((a1, v1), (a2, v2))| a1 == a2 && v1.eq_ignore_ascii_case(v2))
    }
}

impl std::fmt::Display for DistinguishedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self
            .components
            .iter()
            .map(|(a, v)| format!("{a}={v}"))
            .collect::<Vec<_>>()
            .join(",");
        f.write_str(&joined)
    }
}

/// A directory entry with multi-valued attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectoryEntry {
    /// The entry's distinguished name (canonical form preserved as given).
    pub dn: String,
    /// `objectClass` values.
    pub object_classes: Vec<String>,
    /// Attribute name -> values (multi-valued, as in LDAP).
    pub attributes: BTreeMap<String, Vec<String>>,
}

impl DirectoryEntry {
    /// Creates an entry with the given DN and no attributes.
    pub fn new(dn: impl Into<String>) -> Self {
        Self {
            dn: dn.into(),
            object_classes: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Adds an `objectClass`.
    #[must_use]
    pub fn with_object_class(mut self, class: impl Into<String>) -> Self {
        self.object_classes.push(class.into());
        self
    }

    /// Appends a single attribute value.
    #[must_use]
    pub fn with_attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes
            .entry(name.into())
            .or_default()
            .push(value.into());
        self
    }

    /// Returns the first value of an attribute, if present (case-insensitive name).
    pub fn attr_first(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .and_then(|(_, v)| v.first())
            .map(String::as_str)
    }

    /// Returns all values of an attribute (case-insensitive name).
    pub fn attr_values(&self, name: &str) -> Vec<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// Returns `true` if the entry has the given `objectClass` (case-insensitive).
    pub fn has_object_class(&self, class: &str) -> bool {
        self.object_classes
            .iter()
            .any(|c| c.eq_ignore_ascii_case(class))
    }

    /// Builds a [`Principal`] from this entry, using the directly-listed
    /// `memberOf` values as groups (use [`InMemoryDirectory::principal_for`] for
    /// transitive groups).
    pub fn to_principal(&self) -> Principal {
        let subject = self
            .attr_first("uid")
            .or_else(|| self.attr_first("sAMAccountName"))
            .or_else(|| self.attr_first("cn"))
            .unwrap_or(&self.dn)
            .to_string();
        let display_name = self
            .attr_first("displayName")
            .or_else(|| self.attr_first("cn"))
            .unwrap_or(&subject)
            .to_string();
        let email = self
            .attr_first("mail")
            .or_else(|| self.attr_first("userPrincipalName"))
            .map(str::to_string);
        let groups = self
            .attr_values("memberOf")
            .into_iter()
            .map(str::to_string)
            .collect();
        Principal {
            subject,
            display_name,
            email,
            groups,
            attributes: BTreeMap::new(),
        }
    }
}

/// The scope of a directory search relative to the base DN.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchScope {
    /// Only the base entry itself.
    Base,
    /// Only the immediate children of the base.
    OneLevel,
    /// The base entry and its entire subtree.
    Subtree,
}

/// An RFC 4515 search filter (a useful subset).
///
/// Supports presence (`(a=*)`), equality (`(a=v)`), substring wildcards
/// (`(a=*v*)`) and the `&`/`|`/`!` boolean operators. Attribute matching is
/// case-insensitive, mirroring common LDAP `caseIgnoreMatch` rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LdapFilter {
    /// `(attr=*)` — attribute is present.
    Present(String),
    /// `(attr=value)` — exact (case-insensitive) match.
    Equality(String, String),
    /// `(attr=*foo*)` — substring/wildcard match.
    Substring(String, String),
    /// `(&(...)(...))` — all sub-filters must match.
    And(Vec<LdapFilter>),
    /// `(|(...)(...))` — at least one sub-filter must match.
    Or(Vec<LdapFilter>),
    /// `(!(...))` — sub-filter must not match.
    Not(Box<LdapFilter>),
}

impl LdapFilter {
    /// Parses a filter string such as `(&(objectClass=person)(uid=a*))`.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::DirectoryError`] if the filter is malformed.
    pub fn parse(input: &str) -> DiffResult<Self> {
        let chars: Vec<char> = input.chars().collect();
        let mut pos = 0usize;
        let filter = parse_filter(&chars, &mut pos)?;
        // Allow trailing whitespace only.
        while pos < chars.len() {
            if !chars[pos].is_whitespace() {
                return Err(DiffError::DirectoryError(
                    "trailing characters after filter".to_string(),
                ));
            }
            pos += 1;
        }
        Ok(filter)
    }

    /// Returns `true` if `entry` satisfies this filter.
    pub fn matches(&self, entry: &DirectoryEntry) -> bool {
        match self {
            Self::Present(attr) => {
                if attr.eq_ignore_ascii_case("objectClass") {
                    !entry.object_classes.is_empty()
                } else {
                    !entry.attr_values(attr).is_empty()
                }
            }
            Self::Equality(attr, value) => self
                .attr_pool(entry, attr)
                .iter()
                .any(|v| v.eq_ignore_ascii_case(value)),
            Self::Substring(attr, pattern) => self
                .attr_pool(entry, attr)
                .iter()
                .any(|v| glob_match_ci(pattern, v)),
            Self::And(subs) => subs.iter().all(|f| f.matches(entry)),
            Self::Or(subs) => subs.iter().any(|f| f.matches(entry)),
            Self::Not(inner) => !inner.matches(entry),
        }
    }

    fn attr_pool(&self, entry: &DirectoryEntry, attr: &str) -> Vec<String> {
        if attr.eq_ignore_ascii_case("objectClass") {
            entry.object_classes.clone()
        } else {
            entry
                .attr_values(attr)
                .into_iter()
                .map(str::to_string)
                .collect()
        }
    }
}

fn skip_ws(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

fn parse_filter(chars: &[char], pos: &mut usize) -> DiffResult<LdapFilter> {
    skip_ws(chars, pos);
    if *pos >= chars.len() || chars[*pos] != '(' {
        return Err(DiffError::DirectoryError("expected '('".to_string()));
    }
    *pos += 1; // consume '('
    skip_ws(chars, pos);
    if *pos >= chars.len() {
        return Err(DiffError::DirectoryError(
            "unexpected end of filter".to_string(),
        ));
    }
    let filter = match chars[*pos] {
        '&' | '|' => {
            let op = chars[*pos];
            *pos += 1;
            let mut subs = Vec::new();
            loop {
                skip_ws(chars, pos);
                if *pos < chars.len() && chars[*pos] == '(' {
                    subs.push(parse_filter(chars, pos)?);
                } else {
                    break;
                }
            }
            if subs.is_empty() {
                return Err(DiffError::DirectoryError(
                    "boolean filter needs at least one operand".to_string(),
                ));
            }
            if op == '&' {
                LdapFilter::And(subs)
            } else {
                LdapFilter::Or(subs)
            }
        }
        '!' => {
            *pos += 1;
            let inner = parse_filter(chars, pos)?;
            LdapFilter::Not(Box::new(inner))
        }
        _ => parse_item(chars, pos)?,
    };
    skip_ws(chars, pos);
    if *pos >= chars.len() || chars[*pos] != ')' {
        return Err(DiffError::DirectoryError("expected ')'".to_string()));
    }
    *pos += 1; // consume ')'
    Ok(filter)
}

fn parse_item(chars: &[char], pos: &mut usize) -> DiffResult<LdapFilter> {
    let mut attr = String::new();
    while *pos < chars.len() && chars[*pos] != '=' && chars[*pos] != ')' {
        attr.push(chars[*pos]);
        *pos += 1;
    }
    let attr = attr.trim().to_string();
    if attr.is_empty() {
        return Err(DiffError::DirectoryError(
            "empty attribute in filter".to_string(),
        ));
    }
    if *pos >= chars.len() || chars[*pos] != '=' {
        return Err(DiffError::DirectoryError(format!(
            "expected '=' after attribute '{attr}'"
        )));
    }
    *pos += 1; // consume '='
    let mut value = String::new();
    while *pos < chars.len() && chars[*pos] != ')' {
        value.push(chars[*pos]);
        *pos += 1;
    }
    let value = value.trim().to_string();
    if value == "*" {
        Ok(LdapFilter::Present(attr))
    } else if value.contains('*') {
        Ok(LdapFilter::Substring(attr, value))
    } else {
        Ok(LdapFilter::Equality(attr, value))
    }
}

/// Abstraction over a directory backend (bind + search + lookup).
///
/// Implemented in-memory by [`InMemoryDirectory`]; a networked LDAP client can
/// implement the same trait without changing callers.
pub trait DirectoryService {
    /// Authenticates a DN with its password.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::AuthenticationFailed`] if the DN is unknown or the
    /// password is wrong.
    fn bind(&self, dn: &str, password: &str) -> DiffResult<()>;

    /// Returns the entry at `dn`, if present.
    fn lookup(&self, dn: &str) -> Option<DirectoryEntry>;

    /// Searches the directory.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::DirectoryError`] if `base_dn` is malformed.
    fn search(
        &self,
        base_dn: &str,
        scope: SearchScope,
        filter: &LdapFilter,
    ) -> DiffResult<Vec<DirectoryEntry>>;
}

/// A pure-Rust in-memory directory.
#[derive(Debug, Clone, Default)]
pub struct InMemoryDirectory {
    entries: HashMap<String, DirectoryEntry>,
    credentials: HashMap<String, String>,
}

impl InMemoryDirectory {
    /// Creates an empty directory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds (or replaces) an entry, keyed by its normalized DN.
    pub fn add_entry(&mut self, entry: DirectoryEntry) {
        let key = DistinguishedName::parse(&entry.dn)
            .map(|d| d.normalized())
            .unwrap_or_else(|_| entry.dn.to_lowercase());
        self.entries.insert(key, entry);
    }

    /// Sets a bind password for a DN.
    pub fn set_password(&mut self, dn: &str, password: impl Into<String>) {
        let key = DistinguishedName::parse(dn)
            .map(|d| d.normalized())
            .unwrap_or_else(|_| dn.to_lowercase());
        self.credentials.insert(key, password.into());
    }

    /// The number of entries in the directory.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the directory has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn key_for(dn: &str) -> String {
        DistinguishedName::parse(dn)
            .map(|d| d.normalized())
            .unwrap_or_else(|_| dn.to_lowercase())
    }

    /// Resolves the transitive set of group DNs the entry belongs to, following
    /// nested `memberOf` (and reverse `member`) links — as Active Directory does.
    pub fn effective_groups(&self, dn: &str) -> Vec<String> {
        let mut result: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(Self::key_for(dn));
        let mut visited: HashSet<String> = HashSet::new();

        // Pre-index reverse membership: group -> members via `member` attribute.
        while let Some(current_key) = queue.pop_front() {
            if !visited.insert(current_key.clone()) {
                continue;
            }
            // Forward links: this entry's memberOf values.
            if let Some(entry) = self.entries.get(&current_key) {
                for group_dn in entry.attr_values("memberOf") {
                    result.insert(group_dn.to_string());
                    queue.push_back(Self::key_for(group_dn));
                }
            }
            // Reverse links: groups whose `member` lists this DN.
            for entry in self.entries.values() {
                if !entry.has_object_class("groupOfNames")
                    && !entry.has_object_class("group")
                    && entry.attr_values("member").is_empty()
                {
                    continue;
                }
                let is_member = entry
                    .attr_values("member")
                    .iter()
                    .any(|m| Self::key_for(m) == current_key);
                if is_member && result.insert(entry.dn.clone()) {
                    queue.push_back(Self::key_for(&entry.dn));
                }
            }
        }
        let mut groups: Vec<String> = result.into_iter().collect();
        groups.sort();
        groups
    }

    /// Returns `true` if `dn` is (transitively) a member of `group_dn`.
    pub fn is_member_of(&self, dn: &str, group_dn: &str) -> bool {
        let target = Self::key_for(group_dn);
        self.effective_groups(dn)
            .iter()
            .any(|g| Self::key_for(g) == target)
    }

    /// Builds a [`Principal`] for a DN, populating groups with the transitive
    /// [`effective_groups`](Self::effective_groups) closure.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::DirectoryError`] if the DN is not found.
    pub fn principal_for(&self, dn: &str) -> DiffResult<Principal> {
        let entry = self
            .entries
            .get(&Self::key_for(dn))
            .ok_or_else(|| DiffError::DirectoryError(format!("entry not found: {dn}")))?;
        let mut principal = entry.to_principal();
        principal.groups = self.effective_groups(dn);
        Ok(principal)
    }
}

impl DirectoryService for InMemoryDirectory {
    fn bind(&self, dn: &str, password: &str) -> DiffResult<()> {
        let key = Self::key_for(dn);
        if !self.entries.contains_key(&key) {
            return Err(DiffError::AuthenticationFailed(format!(
                "bind DN not found: {dn}"
            )));
        }
        match self.credentials.get(&key) {
            Some(stored) if stored == password => Ok(()),
            _ => Err(DiffError::AuthenticationFailed(
                "invalid credentials".to_string(),
            )),
        }
    }

    fn lookup(&self, dn: &str) -> Option<DirectoryEntry> {
        self.entries.get(&Self::key_for(dn)).cloned()
    }

    fn search(
        &self,
        base_dn: &str,
        scope: SearchScope,
        filter: &LdapFilter,
    ) -> DiffResult<Vec<DirectoryEntry>> {
        let base = DistinguishedName::parse(base_dn)?;
        let mut hits: Vec<DirectoryEntry> = Vec::new();
        for entry in self.entries.values() {
            let entry_dn = match DistinguishedName::parse(&entry.dn) {
                Ok(d) => d,
                Err(_) => continue,
            };
            let in_scope = match scope {
                SearchScope::Base => entry_dn.normalized() == base.normalized(),
                SearchScope::OneLevel => entry_dn
                    .parent()
                    .map(|p| p.normalized() == base.normalized())
                    .unwrap_or(false),
                SearchScope::Subtree => entry_dn.is_within(&base),
            };
            if in_scope && filter.matches(entry) {
                hits.push(entry.clone());
            }
        }
        hits.sort_by(|a, b| a.dn.cmp(&b.dn));
        Ok(hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn directory() -> InMemoryDirectory {
        let mut dir = InMemoryDirectory::new();
        dir.add_entry(
            DirectoryEntry::new("cn=editors,ou=groups,dc=example,dc=com")
                .with_object_class("groupOfNames")
                .with_attr("cn", "editors")
                .with_attr("member", "uid=alice,ou=people,dc=example,dc=com"),
        );
        dir.add_entry(
            DirectoryEntry::new("cn=staff,ou=groups,dc=example,dc=com")
                .with_object_class("groupOfNames")
                .with_attr("cn", "staff")
                .with_attr("member", "cn=editors,ou=groups,dc=example,dc=com"),
        );
        dir.add_entry(
            DirectoryEntry::new("uid=alice,ou=people,dc=example,dc=com")
                .with_object_class("inetOrgPerson")
                .with_attr("uid", "alice")
                .with_attr("cn", "Alice Counsel")
                .with_attr("mail", "alice@example.com"),
        );
        dir.add_entry(
            DirectoryEntry::new("uid=bob,ou=people,dc=example,dc=com")
                .with_object_class("inetOrgPerson")
                .with_attr("uid", "bob")
                .with_attr("cn", "Bob Clerk"),
        );
        dir.set_password("uid=alice,ou=people,dc=example,dc=com", "s3cret");
        dir
    }

    #[test]
    fn test_dn_parse_and_parent() {
        let dn = DistinguishedName::parse("uid=alice,ou=people,dc=example,dc=com").unwrap();
        assert_eq!(dn.components.len(), 4);
        assert_eq!(dn.rdn().as_deref(), Some("uid=alice"));
        let parent = dn.parent().unwrap();
        assert_eq!(parent.rdn().as_deref(), Some("ou=people"));
        assert!(DistinguishedName::parse("bogus").is_err());
    }

    #[test]
    fn test_dn_is_within() {
        let child = DistinguishedName::parse("uid=alice,ou=people,dc=example,dc=com").unwrap();
        let base = DistinguishedName::parse("dc=example,dc=com").unwrap();
        let other = DistinguishedName::parse("dc=other,dc=com").unwrap();
        assert!(child.is_within(&base));
        assert!(child.is_within(&child));
        assert!(!child.is_within(&other));
    }

    #[test]
    fn test_filter_parse_and_match() {
        let dir = directory();
        let alice = dir.lookup("uid=alice,ou=people,dc=example,dc=com").unwrap();

        let eq = LdapFilter::parse("(uid=alice)").unwrap();
        assert!(eq.matches(&alice));

        let present = LdapFilter::parse("(mail=*)").unwrap();
        assert!(present.matches(&alice));
        let bob = dir.lookup("uid=bob,ou=people,dc=example,dc=com").unwrap();
        assert!(!present.matches(&bob));

        let substr = LdapFilter::parse("(cn=Alice*)").unwrap();
        assert!(substr.matches(&alice));

        let composite =
            LdapFilter::parse("(&(objectClass=inetOrgPerson)(|(uid=alice)(uid=carol)))").unwrap();
        assert!(composite.matches(&alice));

        let negation = LdapFilter::parse("(!(uid=bob))").unwrap();
        assert!(negation.matches(&alice));
        assert!(!negation.matches(&bob));
    }

    #[test]
    fn test_filter_parse_errors() {
        assert!(LdapFilter::parse("uid=alice").is_err()); // missing parens
        assert!(LdapFilter::parse("(uid)").is_err()); // missing '='
        assert!(LdapFilter::parse("(&)").is_err()); // empty boolean
        assert!(LdapFilter::parse("(=value)").is_err()); // empty attribute
    }

    #[test]
    fn test_bind_success_and_failure() {
        let dir = directory();
        assert!(
            dir.bind("uid=alice,ou=people,dc=example,dc=com", "s3cret")
                .is_ok()
        );
        assert!(matches!(
            dir.bind("uid=alice,ou=people,dc=example,dc=com", "wrong"),
            Err(DiffError::AuthenticationFailed(_))
        ));
        assert!(matches!(
            dir.bind("uid=ghost,ou=people,dc=example,dc=com", "x"),
            Err(DiffError::AuthenticationFailed(_))
        ));
    }

    #[test]
    fn test_search_scopes() {
        let dir = directory();
        let all = LdapFilter::parse("(objectClass=*)").unwrap();

        let subtree = dir
            .search("dc=example,dc=com", SearchScope::Subtree, &all)
            .unwrap();
        assert_eq!(subtree.len(), 4);

        let one_level = dir
            .search("ou=people,dc=example,dc=com", SearchScope::OneLevel, &all)
            .unwrap();
        assert_eq!(one_level.len(), 2); // alice + bob

        let base = dir
            .search(
                "uid=alice,ou=people,dc=example,dc=com",
                SearchScope::Base,
                &all,
            )
            .unwrap();
        assert_eq!(base.len(), 1);

        assert!(dir.search("bogus", SearchScope::Base, &all).is_err());
    }

    #[test]
    fn test_transitive_group_resolution() {
        let dir = directory();
        // alice -> editors (direct member) -> staff (editors is a member of staff)
        let groups = dir.effective_groups("uid=alice,ou=people,dc=example,dc=com");
        assert!(groups.iter().any(|g| g.starts_with("cn=editors")));
        assert!(groups.iter().any(|g| g.starts_with("cn=staff")));
        assert!(dir.is_member_of(
            "uid=alice,ou=people,dc=example,dc=com",
            "cn=staff,ou=groups,dc=example,dc=com"
        ));
        assert!(!dir.is_member_of(
            "uid=bob,ou=people,dc=example,dc=com",
            "cn=staff,ou=groups,dc=example,dc=com"
        ));
    }

    #[test]
    fn test_principal_for_includes_transitive_groups() {
        let dir = directory();
        let principal = dir
            .principal_for("uid=alice,ou=people,dc=example,dc=com")
            .unwrap();
        assert_eq!(principal.subject, "alice");
        assert_eq!(principal.display_name, "Alice Counsel");
        assert_eq!(principal.email.as_deref(), Some("alice@example.com"));
        assert!(principal.groups.iter().any(|g| g.starts_with("cn=staff")));
        assert!(dir.principal_for("uid=ghost,dc=example,dc=com").is_err());
    }

    #[test]
    fn test_entry_accessors_and_len() {
        let dir = directory();
        assert_eq!(dir.len(), 4);
        assert!(!dir.is_empty());
        let alice = dir.lookup("uid=alice,ou=people,dc=example,dc=com").unwrap();
        assert!(alice.has_object_class("inetOrgPerson"));
        assert!(!alice.has_object_class("computer"));
        assert_eq!(alice.attr_first("uid"), Some("alice"));
    }
}
