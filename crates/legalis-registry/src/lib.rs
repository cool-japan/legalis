//! Legalis-Registry: Statute registry and version management for Legalis-RS.
//!
//! This crate provides a central registry for managing statute collections:
//! - Version control for statutes
//! - Hierarchical statute organization
//! - Cross-reference management
//! - Amendment tracking

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

/// Errors during registry operations.
#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Statute not found: {0}")]
    StatuteNotFound(String),

    #[error("Version not found: {statute_id} v{version}")]
    VersionNotFound { statute_id: String, version: u32 },

    #[error("Duplicate statute ID: {0}")]
    DuplicateId(String),

    #[error("Circular reference detected: {0}")]
    CircularReference(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Result type for registry operations.
pub type RegistryResult<T> = Result<T, RegistryError>;

/// A versioned statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteEntry {
    /// Unique registry ID
    pub registry_id: Uuid,
    /// The statute data
    pub statute: Statute,
    /// Version number
    pub version: u32,
    /// Status
    pub status: StatuteStatus,
    /// Effective date
    pub effective_date: Option<DateTime<Utc>>,
    /// Expiry date
    pub expiry_date: Option<DateTime<Utc>>,
    /// Parent statute (for amendments)
    pub amends: Option<String>,
    /// Statutes this one supersedes
    pub supersedes: Vec<String>,
    /// References to other statutes
    pub references: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Metadata
    pub metadata: HashMap<String, String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

impl StatuteEntry {
    /// Creates a new statute entry.
    pub fn new(statute: Statute, jurisdiction: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            registry_id: Uuid::new_v4(),
            statute,
            version: 1,
            status: StatuteStatus::Draft,
            effective_date: None,
            expiry_date: None,
            amends: None,
            supersedes: Vec::new(),
            references: Vec::new(),
            tags: Vec::new(),
            jurisdiction: jurisdiction.into(),
            metadata: HashMap::new(),
            created_at: now,
            modified_at: now,
        }
    }

    /// Sets the effective date.
    pub fn with_effective_date(mut self, date: DateTime<Utc>) -> Self {
        self.effective_date = Some(date);
        self
    }

    /// Sets the status.
    pub fn with_status(mut self, status: StatuteStatus) -> Self {
        self.status = status;
        self
    }

    /// Adds a reference.
    pub fn with_reference(mut self, statute_id: impl Into<String>) -> Self {
        self.references.push(statute_id.into());
        self
    }

    /// Adds a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Returns whether this statute is currently active.
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.status == StatuteStatus::Active
            && self.effective_date.map_or(true, |d| d <= now)
            && self.expiry_date.map_or(true, |d| d > now)
    }
}

/// Status of a statute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatuteStatus {
    /// Being drafted
    Draft,
    /// Under review
    UnderReview,
    /// Approved but not yet effective
    Approved,
    /// Currently in force
    Active,
    /// No longer in force
    Repealed,
    /// Replaced by another statute
    Superseded,
}

/// The central statute registry.
#[derive(Debug, Default)]
pub struct StatuteRegistry {
    /// Statutes by ID (latest version)
    statutes: IndexMap<String, StatuteEntry>,
    /// Version history: statute_id -> version -> entry
    versions: HashMap<String, HashMap<u32, StatuteEntry>>,
    /// Index by tag
    tag_index: HashMap<String, HashSet<String>>,
    /// Index by jurisdiction
    jurisdiction_index: HashMap<String, HashSet<String>>,
}

impl StatuteRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new statute.
    pub fn register(&mut self, entry: StatuteEntry) -> RegistryResult<Uuid> {
        let statute_id = entry.statute.id.clone();

        if self.statutes.contains_key(&statute_id) {
            return Err(RegistryError::DuplicateId(statute_id));
        }

        let registry_id = entry.registry_id;

        // Update indexes
        for tag in &entry.tags {
            self.tag_index
                .entry(tag.clone())
                .or_default()
                .insert(statute_id.clone());
        }

        self.jurisdiction_index
            .entry(entry.jurisdiction.clone())
            .or_default()
            .insert(statute_id.clone());

        // Store version
        self.versions
            .entry(statute_id.clone())
            .or_default()
            .insert(entry.version, entry.clone());

        // Store as current
        self.statutes.insert(statute_id, entry);

        Ok(registry_id)
    }

    /// Updates a statute (creates new version).
    pub fn update(&mut self, statute_id: &str, statute: Statute) -> RegistryResult<u32> {
        let existing = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let new_version = existing.version + 1;
        let mut new_entry = StatuteEntry::new(statute, &existing.jurisdiction);
        new_entry.version = new_version;
        new_entry.tags = existing.tags.clone();
        new_entry.references = existing.references.clone();
        new_entry.modified_at = Utc::now();

        // Store version
        self.versions
            .entry(statute_id.to_string())
            .or_default()
            .insert(new_version, new_entry.clone());

        // Update current
        self.statutes.insert(statute_id.to_string(), new_entry);

        Ok(new_version)
    }

    /// Gets a statute by ID (latest version).
    pub fn get(&self, statute_id: &str) -> Option<&StatuteEntry> {
        self.statutes.get(statute_id)
    }

    /// Gets a specific version of a statute.
    pub fn get_version(&self, statute_id: &str, version: u32) -> RegistryResult<&StatuteEntry> {
        self.versions
            .get(statute_id)
            .and_then(|versions| versions.get(&version))
            .ok_or_else(|| RegistryError::VersionNotFound {
                statute_id: statute_id.to_string(),
                version,
            })
    }

    /// Lists all versions of a statute.
    pub fn list_versions(&self, statute_id: &str) -> Vec<u32> {
        self.versions
            .get(statute_id)
            .map(|v| {
                let mut versions: Vec<u32> = v.keys().copied().collect();
                versions.sort();
                versions
            })
            .unwrap_or_default()
    }

    /// Lists all statutes.
    pub fn list(&self) -> Vec<&StatuteEntry> {
        self.statutes.values().collect()
    }

    /// Lists active statutes.
    pub fn list_active(&self) -> Vec<&StatuteEntry> {
        self.statutes.values().filter(|e| e.is_active()).collect()
    }

    /// Queries statutes by tag.
    pub fn query_by_tag(&self, tag: &str) -> Vec<&StatuteEntry> {
        self.tag_index
            .get(tag)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.statutes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Queries statutes by jurisdiction.
    pub fn query_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&StatuteEntry> {
        self.jurisdiction_index
            .get(jurisdiction)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.statutes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Sets the status of a statute.
    pub fn set_status(&mut self, statute_id: &str, status: StatuteStatus) -> RegistryResult<()> {
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        entry.status = status;
        entry.modified_at = Utc::now();

        Ok(())
    }

    /// Returns the total count of statutes.
    pub fn count(&self) -> usize {
        self.statutes.len()
    }

    /// Returns all tags.
    pub fn all_tags(&self) -> Vec<&String> {
        self.tag_index.keys().collect()
    }

    /// Returns all jurisdictions.
    pub fn all_jurisdictions(&self) -> Vec<&String> {
        self.jurisdiction_index.keys().collect()
    }

    /// Finds statutes that reference a given statute.
    pub fn find_referencing(&self, statute_id: &str) -> Vec<&StatuteEntry> {
        self.statutes
            .values()
            .filter(|e| e.references.contains(&statute_id.to_string()))
            .collect()
    }

    /// Gets the dependency graph for a statute.
    pub fn get_dependencies(&self, statute_id: &str) -> HashSet<String> {
        let mut deps = HashSet::new();
        self.collect_dependencies(statute_id, &mut deps, &mut HashSet::new());
        deps
    }

    fn collect_dependencies(
        &self,
        statute_id: &str,
        deps: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(statute_id) {
            return;
        }
        visited.insert(statute_id.to_string());

        if let Some(entry) = self.statutes.get(statute_id) {
            for reference in &entry.references {
                deps.insert(reference.clone());
                self.collect_dependencies(reference, deps, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn test_statute(id: &str) -> Statute {
        Statute::new(id, format!("Test {}", id), Effect::new(EffectType::Grant, "Test"))
    }

    #[test]
    fn test_register_statute() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("test-1"), "JP")
            .with_tag("civil")
            .with_status(StatuteStatus::Active);

        let id = registry.register(entry).unwrap();
        assert!(!id.is_nil());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_version_management() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("test-1"), "JP");
        registry.register(entry).unwrap();

        let new_version = registry.update("test-1", test_statute("test-1")).unwrap();
        assert_eq!(new_version, 2);

        let versions = registry.list_versions("test-1");
        assert_eq!(versions, vec![1, 2]);
    }

    #[test]
    fn test_query_by_tag() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("civil-1"), "JP").with_tag("civil"))
            .unwrap();

        registry
            .register(StatuteEntry::new(test_statute("criminal-1"), "JP").with_tag("criminal"))
            .unwrap();

        let civil = registry.query_by_tag("civil");
        assert_eq!(civil.len(), 1);
        assert_eq!(civil[0].statute.id, "civil-1");
    }

    #[test]
    fn test_is_active() {
        let mut entry = StatuteEntry::new(test_statute("test"), "JP");
        entry.status = StatuteStatus::Active;
        assert!(entry.is_active());

        entry.status = StatuteStatus::Draft;
        assert!(!entry.is_active());
    }
}
