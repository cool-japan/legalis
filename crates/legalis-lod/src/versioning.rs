//! Dataset versioning and change tracking.
//!
//! This module provides utilities for managing different versions of RDF datasets,
//! tracking changes, and generating version metadata.

use crate::{RdfValue, Triple};
use chrono::{DateTime, Utc};

/// Version of an RDF dataset.
#[derive(Debug, Clone)]
pub struct DatasetVersion {
    /// Version identifier
    pub version: String,
    /// Version label (e.g., "v1.0.0")
    pub label: String,
    /// Description of this version
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Creator/agent
    pub creator: Option<String>,
    /// Previous version (if any)
    pub previous_version: Option<String>,
    /// Change summary
    pub changes: Vec<Change>,
    /// Version URI
    pub uri: String,
}

/// Type of change in a version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    /// Addition of new data
    Addition,
    /// Modification of existing data
    Modification,
    /// Deletion of data
    Deletion,
    /// Correction of error
    Correction,
    /// Enhancement/improvement
    Enhancement,
}

/// Individual change in a version.
#[derive(Debug, Clone)]
pub struct Change {
    /// Type of change
    pub change_type: ChangeType,
    /// Description of the change
    pub description: String,
    /// Affected subjects
    pub affected_subjects: Vec<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Version history manager.
#[derive(Debug)]
pub struct VersionHistory {
    /// Dataset URI
    pub dataset_uri: String,
    /// All versions (ordered chronologically)
    pub versions: Vec<DatasetVersion>,
    /// Current version
    pub current_version: Option<String>,
}

impl DatasetVersion {
    /// Creates a new dataset version.
    pub fn new(
        version: impl Into<String>,
        label: impl Into<String>,
        uri: impl Into<String>,
    ) -> Self {
        Self {
            version: version.into(),
            label: label.into(),
            description: None,
            created_at: Utc::now(),
            creator: None,
            previous_version: None,
            changes: Vec::new(),
            uri: uri.into(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the creator.
    pub fn with_creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    /// Sets the previous version.
    pub fn with_previous_version(mut self, previous: impl Into<String>) -> Self {
        self.previous_version = Some(previous.into());
        self
    }

    /// Adds a change to this version.
    pub fn add_change(&mut self, change: Change) {
        self.changes.push(change);
    }

    /// Converts the version to RDF triples.
    #[allow(clippy::vec_init_then_push)]
    pub fn to_triples(&self, dataset_uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Version type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("dcat:Dataset".to_string()),
        });

        // Version info
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcterms:isVersionOf".to_string(),
            object: RdfValue::Uri(dataset_uri.to_string()),
        });

        triples.push(Triple {
            subject: dataset_uri.to_string(),
            predicate: "dcterms:hasVersion".to_string(),
            object: RdfValue::Uri(self.uri.clone()),
        });

        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "owl:versionInfo".to_string(),
            object: RdfValue::string(&self.version),
        });

        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&self.label),
        });

        // Description
        if let Some(ref desc) = self.description {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:description".to_string(),
                object: RdfValue::string(desc),
            });
        }

        // Created
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcterms:created".to_string(),
            object: RdfValue::datetime(self.created_at),
        });

        // Creator
        if let Some(ref creator) = self.creator {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:creator".to_string(),
                object: RdfValue::Uri(creator.clone()),
            });
        }

        // Previous version
        if let Some(ref prev) = self.previous_version {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "prov:wasRevisionOf".to_string(),
                object: RdfValue::Uri(prev.clone()),
            });
        }

        // Changes
        for (i, change) in self.changes.iter().enumerate() {
            let change_uri = format!("{}#change-{}", self.uri, i);

            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "legalis:hasChange".to_string(),
                object: RdfValue::Uri(change_uri.clone()),
            });

            triples.extend(change.to_triples(&change_uri));
        }

        triples
    }

    /// Returns statistics about this version.
    pub fn stats(&self) -> VersionStats {
        let mut additions = 0;
        let mut modifications = 0;
        let mut deletions = 0;
        let mut corrections = 0;
        let mut enhancements = 0;

        for change in &self.changes {
            match change.change_type {
                ChangeType::Addition => additions += 1,
                ChangeType::Modification => modifications += 1,
                ChangeType::Deletion => deletions += 1,
                ChangeType::Correction => corrections += 1,
                ChangeType::Enhancement => enhancements += 1,
            }
        }

        VersionStats {
            total_changes: self.changes.len(),
            additions,
            modifications,
            deletions,
            corrections,
            enhancements,
        }
    }
}

/// Statistics about a version.
#[derive(Debug, Clone, Default)]
pub struct VersionStats {
    /// Total number of changes
    pub total_changes: usize,
    /// Number of additions
    pub additions: usize,
    /// Number of modifications
    pub modifications: usize,
    /// Number of deletions
    pub deletions: usize,
    /// Number of corrections
    pub corrections: usize,
    /// Number of enhancements
    pub enhancements: usize,
}

impl Change {
    /// Creates a new change.
    pub fn new(change_type: ChangeType, description: impl Into<String>) -> Self {
        Self {
            change_type,
            description: description.into(),
            affected_subjects: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Adds an affected subject.
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.affected_subjects.push(subject.into());
        self
    }

    /// Converts the change to RDF triples.
    #[allow(clippy::vec_init_then_push)]
    pub fn to_triples(&self, change_uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();

        triples.push(Triple {
            subject: change_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Change".to_string()),
        });

        triples.push(Triple {
            subject: change_uri.to_string(),
            predicate: "legalis:changeType".to_string(),
            object: RdfValue::string(self.change_type_str()),
        });

        triples.push(Triple {
            subject: change_uri.to_string(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&self.description),
        });

        triples.push(Triple {
            subject: change_uri.to_string(),
            predicate: "dcterms:created".to_string(),
            object: RdfValue::datetime(self.timestamp),
        });

        for subject in &self.affected_subjects {
            triples.push(Triple {
                subject: change_uri.to_string(),
                predicate: "legalis:affectsSubject".to_string(),
                object: RdfValue::Uri(subject.clone()),
            });
        }

        triples
    }

    fn change_type_str(&self) -> &str {
        match self.change_type {
            ChangeType::Addition => "addition",
            ChangeType::Modification => "modification",
            ChangeType::Deletion => "deletion",
            ChangeType::Correction => "correction",
            ChangeType::Enhancement => "enhancement",
        }
    }
}

impl VersionHistory {
    /// Creates a new version history.
    pub fn new(dataset_uri: impl Into<String>) -> Self {
        Self {
            dataset_uri: dataset_uri.into(),
            versions: Vec::new(),
            current_version: None,
        }
    }

    /// Adds a version to the history.
    pub fn add_version(&mut self, version: DatasetVersion) {
        self.current_version = Some(version.version.clone());
        self.versions.push(version);
    }

    /// Gets a specific version by identifier.
    pub fn get_version(&self, version: &str) -> Option<&DatasetVersion> {
        self.versions.iter().find(|v| v.version == version)
    }

    /// Gets the current version.
    pub fn get_current_version(&self) -> Option<&DatasetVersion> {
        self.current_version
            .as_ref()
            .and_then(|v| self.get_version(v))
    }

    /// Returns all versions in chronological order.
    pub fn all_versions(&self) -> &[DatasetVersion] {
        &self.versions
    }

    /// Compares two versions and generates a change summary.
    pub fn diff(&self, version1: &str, version2: &str) -> Option<Vec<Change>> {
        let _v1 = self.get_version(version1)?;
        let v2 = self.get_version(version2)?;

        // Return changes from version2 (assuming it's newer)
        Some(v2.changes.clone())
    }

    /// Converts the version history to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Dataset type
        triples.push(Triple {
            subject: self.dataset_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("dcat:Dataset".to_string()),
        });

        // Current version
        if let Some(ref current) = self.current_version
            && let Some(version) = self.get_version(current)
        {
            triples.push(Triple {
                subject: self.dataset_uri.clone(),
                predicate: "dcterms:hasVersion".to_string(),
                object: RdfValue::Uri(version.uri.clone()),
            });
        }

        // All versions
        for version in &self.versions {
            triples.extend(version.to_triples(&self.dataset_uri));
        }

        triples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_version() {
        let version = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1")
            .with_description("Initial release")
            .with_creator("https://example.org/creator");

        assert_eq!(version.version, "1.0.0");
        assert_eq!(version.label, "Version 1.0.0");
        assert!(version.description.is_some());
        assert!(version.creator.is_some());
    }

    #[test]
    fn test_add_change() {
        let mut version = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");

        let change =
            Change::new(ChangeType::Addition, "Added new statute").with_subject("statute:123");

        version.add_change(change);

        assert_eq!(version.changes.len(), 1);
        assert_eq!(version.changes[0].change_type, ChangeType::Addition);
    }

    #[test]
    fn test_version_stats() {
        let mut version = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");

        version.add_change(Change::new(ChangeType::Addition, "Change 1"));
        version.add_change(Change::new(ChangeType::Modification, "Change 2"));
        version.add_change(Change::new(ChangeType::Deletion, "Change 3"));

        let stats = version.stats();

        assert_eq!(stats.total_changes, 3);
        assert_eq!(stats.additions, 1);
        assert_eq!(stats.modifications, 1);
        assert_eq!(stats.deletions, 1);
    }

    #[test]
    fn test_version_history() {
        let mut history = VersionHistory::new("https://example.org/dataset");

        let v1 = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");
        let v2 = DatasetVersion::new("2.0.0", "Version 2.0.0", "https://example.org/v2")
            .with_previous_version("https://example.org/v1");

        history.add_version(v1);
        history.add_version(v2);

        assert_eq!(history.versions.len(), 2);
        assert_eq!(history.current_version, Some("2.0.0".to_string()));
    }

    #[test]
    fn test_get_version() {
        let mut history = VersionHistory::new("https://example.org/dataset");

        let v1 = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");
        history.add_version(v1);

        let retrieved = history.get_version("1.0.0");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().version, "1.0.0");
    }

    #[test]
    fn test_get_current_version() {
        let mut history = VersionHistory::new("https://example.org/dataset");

        let v1 = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");
        let v2 = DatasetVersion::new("2.0.0", "Version 2.0.0", "https://example.org/v2");

        history.add_version(v1);
        history.add_version(v2);

        let current = history.get_current_version();
        assert!(current.is_some());
        assert_eq!(current.unwrap().version, "2.0.0");
    }

    #[test]
    fn test_version_to_triples() {
        let version = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");
        let triples = version.to_triples("https://example.org/dataset");

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
        assert!(triples.iter().any(|t| t.predicate == "owl:versionInfo"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:isVersionOf"));
    }

    #[test]
    fn test_change_to_triples() {
        let change =
            Change::new(ChangeType::Addition, "Added new data").with_subject("statute:123");

        let triples = change.to_triples("https://example.org/change/1");

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "legalis:changeType"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:affectsSubject")
        );
    }

    #[test]
    fn test_version_history_to_triples() {
        let mut history = VersionHistory::new("https://example.org/dataset");

        let v1 = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");
        history.add_version(v1);

        let triples = history.to_triples();

        assert!(!triples.is_empty());
        assert!(
            triples
                .iter()
                .any(|t| t.subject == "https://example.org/dataset")
        );
    }

    #[test]
    fn test_diff() {
        let mut history = VersionHistory::new("https://example.org/dataset");

        let mut v1 = DatasetVersion::new("1.0.0", "Version 1.0.0", "https://example.org/v1");
        v1.add_change(Change::new(ChangeType::Addition, "Initial data"));

        let mut v2 = DatasetVersion::new("2.0.0", "Version 2.0.0", "https://example.org/v2");
        v2.add_change(Change::new(ChangeType::Modification, "Updated data"));

        history.add_version(v1);
        history.add_version(v2);

        let diff = history.diff("1.0.0", "2.0.0");
        assert!(diff.is_some());

        let changes = diff.unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].change_type, ChangeType::Modification);
    }
}
