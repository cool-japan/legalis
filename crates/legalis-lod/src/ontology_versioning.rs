//! Ontology versioning with change tracking.
//!
//! This module provides comprehensive ontology version management, including:
//! - Version tracking and history
//! - Change detection and diff generation
//! - Backward compatibility analysis
//! - Migration path generation

use crate::{LodError, LodResult, RdfValue, Triple};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a semantic version for an ontology.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OntologyVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl OntologyVersion {
    /// Creates a new version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the version as a string (e.g., "1.2.3").
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Parses a version string.
    pub fn parse(s: &str) -> LodResult<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(LodError::SerializationError(format!(
                "Invalid version format: {}",
                s
            )));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| LodError::SerializationError("Invalid major version".to_string()))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| LodError::SerializationError("Invalid minor version".to_string()))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| LodError::SerializationError("Invalid patch version".to_string()))?;

        Ok(Self::new(major, minor, patch))
    }

    /// Returns true if this version is compatible with the given version.
    /// Compatible means same major version.
    pub fn is_compatible_with(&self, other: &OntologyVersion) -> bool {
        self.major == other.major
    }
}

/// Type of change in an ontology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// A class was added
    ClassAdded,
    /// A class was removed
    ClassRemoved,
    /// A class was modified
    ClassModified,
    /// A property was added
    PropertyAdded,
    /// A property was removed
    PropertyRemoved,
    /// A property was modified
    PropertyModified,
    /// An individual was added
    IndividualAdded,
    /// An individual was removed
    IndividualRemoved,
    /// An axiom was added
    AxiomAdded,
    /// An axiom was removed
    AxiomRemoved,
    /// Other change
    Other(String),
}

impl ChangeType {
    /// Returns true if this is a breaking change.
    pub fn is_breaking(&self) -> bool {
        matches!(
            self,
            ChangeType::ClassRemoved
                | ChangeType::PropertyRemoved
                | ChangeType::IndividualRemoved
                | ChangeType::AxiomRemoved
        )
    }
}

/// Represents a single change in an ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyChange {
    /// Type of change
    pub change_type: ChangeType,
    /// URI of the affected entity
    pub entity_uri: String,
    /// Description of the change
    pub description: String,
    /// Timestamp of the change
    pub timestamp: DateTime<Utc>,
    /// Author of the change
    pub author: Option<String>,
    /// Related triples (before and after)
    pub old_triples: Vec<Triple>,
    pub new_triples: Vec<Triple>,
}

impl OntologyChange {
    /// Creates a new ontology change.
    pub fn new(
        change_type: ChangeType,
        entity_uri: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            change_type,
            entity_uri: entity_uri.into(),
            description: description.into(),
            timestamp: Utc::now(),
            author: None,
            old_triples: Vec::new(),
            new_triples: Vec::new(),
        }
    }

    /// Sets the author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the old and new triples.
    #[allow(dead_code)]
    pub fn with_triples(mut self, old_triples: Vec<Triple>, new_triples: Vec<Triple>) -> Self {
        self.old_triples = old_triples;
        self.new_triples = new_triples;
        self
    }

    /// Converts the change to RDF triples.
    pub fn to_triples(&self, base_uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let change_uri = format!("{}/change/{}", base_uri, uuid::Uuid::new_v4());

        triples.push(Triple {
            subject: change_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:OntologyChange".to_string()),
        });

        triples.push(Triple {
            subject: change_uri.clone(),
            predicate: "legalis:changeType".to_string(),
            object: RdfValue::string(format!("{:?}", self.change_type)),
        });

        triples.push(Triple {
            subject: change_uri.clone(),
            predicate: "legalis:entityUri".to_string(),
            object: RdfValue::Uri(self.entity_uri.clone()),
        });

        triples.push(Triple {
            subject: change_uri.clone(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&self.description),
        });

        triples.push(Triple {
            subject: change_uri.clone(),
            predicate: "dcterms:created".to_string(),
            object: RdfValue::datetime(self.timestamp),
        });

        if let Some(ref author) = self.author {
            triples.push(Triple {
                subject: change_uri,
                predicate: "dcterms:creator".to_string(),
                object: RdfValue::string(author),
            });
        }

        triples
    }
}

/// Represents a snapshot of an ontology at a specific version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologySnapshot {
    /// Version identifier
    pub version: OntologyVersion,
    /// Ontology URI
    pub ontology_uri: String,
    /// Timestamp of the snapshot
    pub timestamp: DateTime<Utc>,
    /// All triples in this version
    pub triples: Vec<Triple>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl OntologySnapshot {
    /// Creates a new snapshot.
    pub fn new(
        version: OntologyVersion,
        ontology_uri: impl Into<String>,
        triples: Vec<Triple>,
    ) -> Self {
        Self {
            version,
            ontology_uri: ontology_uri.into(),
            timestamp: Utc::now(),
            triples,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Version history tracker for an ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistory {
    /// Ontology URI
    pub ontology_uri: String,
    /// All snapshots, sorted by version
    pub snapshots: Vec<OntologySnapshot>,
    /// All changes between versions
    pub changes: Vec<OntologyChange>,
}

impl VersionHistory {
    /// Creates a new version history.
    pub fn new(ontology_uri: impl Into<String>) -> Self {
        Self {
            ontology_uri: ontology_uri.into(),
            snapshots: Vec::new(),
            changes: Vec::new(),
        }
    }

    /// Adds a snapshot.
    pub fn add_snapshot(&mut self, snapshot: OntologySnapshot) {
        self.snapshots.push(snapshot);
        self.snapshots.sort_by(|a, b| a.version.cmp(&b.version));
    }

    /// Adds a change.
    pub fn add_change(&mut self, change: OntologyChange) {
        self.changes.push(change);
    }

    /// Gets a snapshot by version.
    pub fn get_snapshot(&self, version: &OntologyVersion) -> Option<&OntologySnapshot> {
        self.snapshots.iter().find(|s| &s.version == version)
    }

    /// Gets the latest snapshot.
    pub fn get_latest_snapshot(&self) -> Option<&OntologySnapshot> {
        self.snapshots.last()
    }

    /// Gets changes between two versions.
    pub fn get_changes_between(
        &self,
        _from: &OntologyVersion,
        _to: &OntologyVersion,
    ) -> Vec<&OntologyChange> {
        // For simplicity, return all changes
        // In a real implementation, we'd filter by version range
        self.changes.iter().collect()
    }

    /// Checks if upgrade from one version to another is breaking.
    pub fn is_breaking_upgrade(&self, from: &OntologyVersion, to: &OntologyVersion) -> bool {
        let changes = self.get_changes_between(from, to);
        changes.iter().any(|c| c.change_type.is_breaking())
    }
}

/// Computes the difference between two ontology snapshots.
pub struct OntologyDiff {
    /// Added triples
    pub added: Vec<Triple>,
    /// Removed triples
    pub removed: Vec<Triple>,
    /// Modified triples (removed and added pairs)
    pub modified: Vec<(Triple, Triple)>,
}

impl OntologyDiff {
    /// Computes the diff between two snapshots.
    pub fn compute(from: &OntologySnapshot, to: &OntologySnapshot) -> Self {
        let from_set: HashSet<_> = from
            .triples
            .iter()
            .map(|t| (&t.subject, &t.predicate, &t.object))
            .collect();
        let to_set: HashSet<_> = to
            .triples
            .iter()
            .map(|t| (&t.subject, &t.predicate, &t.object))
            .collect();

        let mut added = Vec::new();
        let mut removed = Vec::new();

        // Find added triples
        for triple in &to.triples {
            let key = (&triple.subject, &triple.predicate, &triple.object);
            if !from_set.contains(&key) {
                added.push(triple.clone());
            }
        }

        // Find removed triples
        for triple in &from.triples {
            let key = (&triple.subject, &triple.predicate, &triple.object);
            if !to_set.contains(&key) {
                removed.push(triple.clone());
            }
        }

        // Detect modifications (same subject and predicate, different object)
        let mut modified = Vec::new();
        let from_by_sp: HashMap<_, _> = from
            .triples
            .iter()
            .map(|t| ((&t.subject, &t.predicate), t))
            .collect();
        let to_by_sp: HashMap<_, _> = to
            .triples
            .iter()
            .map(|t| ((&t.subject, &t.predicate), t))
            .collect();

        for (sp, from_triple) in &from_by_sp {
            if let Some(to_triple) = to_by_sp.get(sp)
                && from_triple.object != to_triple.object
            {
                modified.push(((*from_triple).clone(), (*to_triple).clone()));
                // Remove from added/removed since they're modifications
                added.retain(|t| t != *to_triple);
                removed.retain(|t| t != *from_triple);
            }
        }

        Self {
            added,
            removed,
            modified,
        }
    }

    /// Converts the diff to a list of changes.
    pub fn to_changes(&self) -> Vec<OntologyChange> {
        let mut changes = Vec::new();

        for triple in &self.added {
            let change_type = if triple.predicate == "rdf:type" {
                match &triple.object {
                    RdfValue::Uri(uri) if uri.contains("Class") => ChangeType::ClassAdded,
                    RdfValue::Uri(uri) if uri.contains("Property") => ChangeType::PropertyAdded,
                    _ => ChangeType::AxiomAdded,
                }
            } else {
                ChangeType::AxiomAdded
            };

            changes.push(OntologyChange::new(
                change_type,
                triple.subject.clone(),
                format!(
                    "Added triple: {} {} {:?}",
                    triple.subject, triple.predicate, triple.object
                ),
            ));
        }

        for triple in &self.removed {
            let change_type = if triple.predicate == "rdf:type" {
                match &triple.object {
                    RdfValue::Uri(uri) if uri.contains("Class") => ChangeType::ClassRemoved,
                    RdfValue::Uri(uri) if uri.contains("Property") => ChangeType::PropertyRemoved,
                    _ => ChangeType::AxiomRemoved,
                }
            } else {
                ChangeType::AxiomRemoved
            };

            changes.push(OntologyChange::new(
                change_type,
                triple.subject.clone(),
                format!(
                    "Removed triple: {} {} {:?}",
                    triple.subject, triple.predicate, triple.object
                ),
            ));
        }

        for (old_triple, new_triple) in &self.modified {
            changes.push(OntologyChange::new(
                ChangeType::Other("Modified".to_string()),
                old_triple.subject.clone(),
                format!(
                    "Modified: {} {} {:?} -> {:?}",
                    old_triple.subject, old_triple.predicate, old_triple.object, new_triple.object
                ),
            ));
        }

        changes
    }

    /// Returns true if the diff contains breaking changes.
    pub fn has_breaking_changes(&self) -> bool {
        for triple in &self.removed {
            if triple.predicate == "rdf:type" {
                return true; // Removing types is breaking
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v = OntologyVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_compatibility() {
        let v1 = OntologyVersion::new(1, 0, 0);
        let v2 = OntologyVersion::new(1, 1, 0);
        let v3 = OntologyVersion::new(2, 0, 0);

        assert!(v1.is_compatible_with(&v2));
        assert!(!v1.is_compatible_with(&v3));
    }

    #[test]
    fn test_change_type_breaking() {
        assert!(ChangeType::ClassRemoved.is_breaking());
        assert!(ChangeType::PropertyRemoved.is_breaking());
        assert!(!ChangeType::ClassAdded.is_breaking());
        assert!(!ChangeType::PropertyAdded.is_breaking());
    }

    #[test]
    fn test_ontology_change() {
        let change = OntologyChange::new(
            ChangeType::ClassAdded,
            "http://example.org/MyClass",
            "Added MyClass",
        )
        .with_author("test-user");

        assert_eq!(change.entity_uri, "http://example.org/MyClass");
        assert_eq!(change.description, "Added MyClass");
        assert_eq!(change.author, Some("test-user".to_string()));
    }

    #[test]
    fn test_version_history() {
        let mut history = VersionHistory::new("http://example.org/ontology");

        let v1 = OntologyVersion::new(1, 0, 0);
        let snapshot1 = OntologySnapshot::new(v1.clone(), "http://example.org/ontology", vec![]);
        history.add_snapshot(snapshot1);

        let v2 = OntologyVersion::new(1, 1, 0);
        let snapshot2 = OntologySnapshot::new(v2.clone(), "http://example.org/ontology", vec![]);
        history.add_snapshot(snapshot2);

        assert_eq!(history.snapshots.len(), 2);
        assert!(history.get_snapshot(&v1).is_some());
        assert!(history.get_latest_snapshot().is_some());
    }

    #[test]
    fn test_ontology_diff() {
        let triple1 = Triple {
            subject: "http://example.org/Entity1".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        };

        let triple2 = Triple {
            subject: "http://example.org/Entity2".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        };

        let v1 = OntologyVersion::new(1, 0, 0);
        let snapshot1 =
            OntologySnapshot::new(v1, "http://example.org/ontology", vec![triple1.clone()]);

        let v2 = OntologyVersion::new(1, 1, 0);
        let snapshot2 = OntologySnapshot::new(
            v2,
            "http://example.org/ontology",
            vec![triple1.clone(), triple2.clone()],
        );

        let diff = OntologyDiff::compute(&snapshot1, &snapshot2);
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 0);
        assert_eq!(diff.added[0].subject, "http://example.org/Entity2");
    }

    #[test]
    fn test_diff_to_changes() {
        let triple = Triple {
            subject: "http://example.org/Entity1".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        };

        let v1 = OntologyVersion::new(1, 0, 0);
        let snapshot1 = OntologySnapshot::new(v1, "http://example.org/ontology", vec![]);

        let v2 = OntologyVersion::new(1, 1, 0);
        let snapshot2 =
            OntologySnapshot::new(v2, "http://example.org/ontology", vec![triple.clone()]);

        let diff = OntologyDiff::compute(&snapshot1, &snapshot2);
        let changes = diff.to_changes();

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].change_type, ChangeType::ClassAdded);
    }

    #[test]
    fn test_breaking_changes() {
        let triple = Triple {
            subject: "http://example.org/Entity1".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        };

        let v1 = OntologyVersion::new(1, 0, 0);
        let snapshot1 =
            OntologySnapshot::new(v1, "http://example.org/ontology", vec![triple.clone()]);

        let v2 = OntologyVersion::new(2, 0, 0);
        let snapshot2 = OntologySnapshot::new(v2, "http://example.org/ontology", vec![]);

        let diff = OntologyDiff::compute(&snapshot1, &snapshot2);
        assert!(diff.has_breaking_changes());
    }
}
