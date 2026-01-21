//! Metadata extraction utilities for legal documents.
//!
//! This module provides tools for extracting structured metadata from statute documents:
//! - Jurisdiction hierarchies
//! - Temporal version histories
//! - Entity relationships
//! - Amendment audit trails

#[cfg(test)]
use crate::ast::AmendmentNode;
use crate::ast::{LegalDocument, StatuteNode};
use chrono::NaiveDate;
use std::collections::HashMap;

/// Extracted jurisdiction hierarchy information.
#[derive(Debug, Clone, PartialEq)]
pub struct JurisdictionHierarchy {
    /// Map of jurisdiction to its parent jurisdiction
    pub hierarchy: HashMap<String, Option<String>>,
    /// All statutes grouped by jurisdiction
    pub statutes_by_jurisdiction: HashMap<String, Vec<String>>,
}

impl JurisdictionHierarchy {
    /// Creates a new jurisdiction hierarchy from a document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut hierarchy = HashMap::new();
        let mut statutes_by_jurisdiction: HashMap<String, Vec<String>> = HashMap::new();

        for statute in &doc.statutes {
            // Extract jurisdiction from statute metadata
            if let Some(jurisdiction) = extract_jurisdiction_from_id(&statute.id) {
                // Add to hierarchy if not already present
                hierarchy.entry(jurisdiction.clone()).or_insert(None);

                // Group statute by jurisdiction
                statutes_by_jurisdiction
                    .entry(jurisdiction)
                    .or_default()
                    .push(statute.id.clone());

                // Try to infer parent jurisdiction from ID structure
                // E.g., "US-CA-SF" -> parent is "US-CA"
                if let Some(parent) = infer_parent_jurisdiction(&statute.id) {
                    hierarchy.insert(statute.id.clone(), Some(parent));
                }
            }
        }

        Self {
            hierarchy,
            statutes_by_jurisdiction,
        }
    }

    /// Gets all child jurisdictions of a given jurisdiction.
    pub fn get_children(&self, jurisdiction: &str) -> Vec<&String> {
        self.hierarchy
            .iter()
            .filter_map(|(j, parent)| {
                if parent.as_deref() == Some(jurisdiction) {
                    Some(j)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets the parent jurisdiction of a given jurisdiction.
    pub fn get_parent(&self, jurisdiction: &str) -> Option<&String> {
        self.hierarchy.get(jurisdiction)?.as_ref()
    }

    /// Gets all statutes in a specific jurisdiction.
    pub fn get_statutes(&self, jurisdiction: &str) -> Vec<&String> {
        self.statutes_by_jurisdiction
            .get(jurisdiction)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
}

/// Temporal version history for a statute.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionHistory {
    /// Statute ID
    pub statute_id: String,
    /// Ordered list of versions (oldest to newest)
    pub versions: Vec<VersionEntry>,
}

/// A single version entry in the history.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionEntry {
    /// Version number
    pub version: u32,
    /// Effective date (if specified)
    pub effective_date: Option<NaiveDate>,
    /// Expiry date (if specified)
    pub expiry_date: Option<NaiveDate>,
    /// Amendment that created this version
    pub amendment: Option<String>,
}

impl VersionHistory {
    /// Creates a version history for a statute.
    pub fn from_statute(statute: &StatuteNode) -> Self {
        let mut versions = Vec::new();

        // Extract versions from amendments
        for amendment in &statute.amendments {
            let version = amendment.version.unwrap_or(1);
            let effective_date = amendment
                .date
                .as_ref()
                .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

            versions.push(VersionEntry {
                version,
                effective_date,
                expiry_date: None,
                amendment: Some(amendment.description.clone()),
            });
        }

        // Sort by version number
        versions.sort_by_key(|v| v.version);

        Self {
            statute_id: statute.id.clone(),
            versions,
        }
    }

    /// Gets the current version (highest version number).
    pub fn current_version(&self) -> Option<&VersionEntry> {
        self.versions.last()
    }

    /// Gets a specific version.
    pub fn get_version(&self, version_number: u32) -> Option<&VersionEntry> {
        self.versions.iter().find(|v| v.version == version_number)
    }

    /// Checks if a version was active on a specific date.
    pub fn is_active_on(&self, version_number: u32, date: &NaiveDate) -> bool {
        if let Some(version) = self.get_version(version_number) {
            if let Some(effective) = &version.effective_date
                && date < effective
            {
                return false;
            }
            if let Some(expiry) = &version.expiry_date
                && date > expiry
            {
                return false;
            }
            true
        } else {
            false
        }
    }
}

/// Extracted entity relationships from statutes.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityRelationships {
    /// Dependencies (statute -> required statutes)
    pub dependencies: HashMap<String, Vec<String>>,
    /// Supersessions (statute -> superseded statutes)
    pub supersessions: HashMap<String, Vec<String>>,
    /// References (statute -> referenced statutes in amendments)
    pub references: HashMap<String, Vec<String>>,
}

impl EntityRelationships {
    /// Extracts entity relationships from a document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut dependencies = HashMap::new();
        let mut supersessions = HashMap::new();
        let mut references = HashMap::new();

        for statute in &doc.statutes {
            // Extract dependencies (REQUIRES clauses)
            if !statute.requires.is_empty() {
                dependencies.insert(statute.id.clone(), statute.requires.clone());
            }

            // Extract supersessions (SUPERSEDES clauses)
            if !statute.supersedes.is_empty() {
                supersessions.insert(statute.id.clone(), statute.supersedes.clone());
            }

            // Extract amendment references
            let amendment_refs: Vec<String> = statute
                .amendments
                .iter()
                .map(|a| a.target_id.clone())
                .collect();
            if !amendment_refs.is_empty() {
                references.insert(statute.id.clone(), amendment_refs);
            }
        }

        Self {
            dependencies,
            supersessions,
            references,
        }
    }

    /// Gets all statutes that depend on a given statute.
    pub fn get_dependents(&self, statute_id: &str) -> Vec<&String> {
        self.dependencies
            .iter()
            .filter_map(|(id, deps)| {
                if deps.contains(&statute_id.to_string()) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets all statutes that are superseded by a given statute.
    pub fn get_superseded_by(&self, statute_id: &str) -> Option<&Vec<String>> {
        self.supersessions.get(statute_id)
    }

    /// Gets all statutes referenced by amendments in a given statute.
    pub fn get_referenced(&self, statute_id: &str) -> Option<&Vec<String>> {
        self.references.get(statute_id)
    }
}

/// Audit trail constructed from amendments.
#[derive(Debug, Clone, PartialEq)]
pub struct AmendmentAuditTrail {
    /// Map of statute ID to its amendment history
    pub history: HashMap<String, Vec<AuditEntry>>,
}

/// A single audit entry representing an amendment.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    /// Target statute that was amended
    pub target_id: String,
    /// Version number (if specified)
    pub version: Option<u32>,
    /// Effective date (if specified)
    pub date: Option<String>,
    /// Description of the amendment
    pub description: String,
    /// Source statute that made the amendment
    pub source_statute_id: String,
}

impl AmendmentAuditTrail {
    /// Creates an audit trail from a document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut history: HashMap<String, Vec<AuditEntry>> = HashMap::new();

        for statute in &doc.statutes {
            for amendment in &statute.amendments {
                let entry = AuditEntry {
                    target_id: amendment.target_id.clone(),
                    version: amendment.version,
                    date: amendment.date.clone(),
                    description: amendment.description.clone(),
                    source_statute_id: statute.id.clone(),
                };

                history
                    .entry(amendment.target_id.clone())
                    .or_default()
                    .push(entry);
            }
        }

        // Sort each statute's history by date (if available)
        for entries in history.values_mut() {
            entries.sort_by(|a, b| match (&a.date, &b.date) {
                (Some(d1), Some(d2)) => d1.cmp(d2),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });
        }

        Self { history }
    }

    /// Gets the amendment history for a specific statute.
    pub fn get_history(&self, statute_id: &str) -> Vec<&AuditEntry> {
        self.history
            .get(statute_id)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all statutes that have been amended.
    pub fn get_amended_statutes(&self) -> Vec<&String> {
        self.history.keys().collect()
    }
}

/// Extracts jurisdiction from a statute ID.
/// Assumes format like "US-CA-statute-1" or "EU-FR-law-2024".
fn extract_jurisdiction_from_id(statute_id: &str) -> Option<String> {
    let parts: Vec<&str> = statute_id.split('-').collect();
    if parts.len() >= 2 {
        Some(format!("{}-{}", parts[0], parts[1]))
    } else {
        None
    }
}

/// Infers parent jurisdiction from statute ID structure.
/// E.g., "US-CA-SF-law-1" (5 parts) -> parent is "US-CA"
/// Only infers a parent if there are at least 5 parts (suggesting nested jurisdictions)
fn infer_parent_jurisdiction(statute_id: &str) -> Option<String> {
    let parts: Vec<&str> = statute_id.split('-').collect();
    // Need at least 5 parts: country-state-city-type-id (e.g., US-CA-SF-law-1)
    if parts.len() >= 5 {
        Some(format!("{}-{}", parts[0], parts[1]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_statute(
        id: &str,
        requires: Vec<String>,
        supersedes: Vec<String>,
    ) -> StatuteNode {
        StatuteNode {
            id: id.to_string(),
            visibility: crate::module_system::Visibility::Private,
            title: format!("Test Statute {}", id),
            conditions: Vec::new(),
            effects: Vec::new(),
            discretion: None,
            exceptions: Vec::new(),
            amendments: Vec::new(),
            supersedes,
            defaults: Vec::new(),
            requires,
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        }
    }

    #[test]
    fn test_jurisdiction_extraction() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: Vec::new(),
            statutes: vec![
                create_test_statute("US-CA-law-1", vec![], vec![]),
                create_test_statute("US-CA-law-2", vec![], vec![]),
                create_test_statute("US-NY-law-1", vec![], vec![]),
            ],
        };

        let hierarchy = JurisdictionHierarchy::from_document(&doc);

        assert!(hierarchy.statutes_by_jurisdiction.contains_key("US-CA"));
        assert!(hierarchy.statutes_by_jurisdiction.contains_key("US-NY"));

        let ca_statutes = hierarchy.get_statutes("US-CA");
        assert_eq!(ca_statutes.len(), 2);
    }

    #[test]
    fn test_entity_relationships() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: Vec::new(),
            statutes: vec![
                create_test_statute("stat1", vec!["stat2".to_string()], vec![]),
                create_test_statute("stat2", vec![], vec!["stat3".to_string()]),
                create_test_statute("stat3", vec![], vec![]),
            ],
        };

        let relationships = EntityRelationships::from_document(&doc);

        assert_eq!(relationships.dependencies.get("stat1").unwrap().len(), 1);
        assert_eq!(relationships.supersessions.get("stat2").unwrap().len(), 1);

        let dependents = relationships.get_dependents("stat2");
        assert_eq!(dependents.len(), 1);
        assert_eq!(dependents[0], "stat1");
    }

    #[test]
    fn test_version_history() {
        let mut statute = create_test_statute("test", vec![], vec![]);
        statute.amendments = vec![
            AmendmentNode {
                target_id: "old-law".to_string(),
                version: Some(1),
                date: Some("2020-01-01".to_string()),
                description: "First amendment".to_string(),
            },
            AmendmentNode {
                target_id: "old-law".to_string(),
                version: Some(2),
                date: Some("2021-01-01".to_string()),
                description: "Second amendment".to_string(),
            },
        ];

        let history = VersionHistory::from_statute(&statute);

        assert_eq!(history.versions.len(), 2);
        assert_eq!(history.current_version().unwrap().version, 2);
        assert!(history.get_version(1).is_some());
    }

    #[test]
    fn test_amendment_audit_trail() {
        let mut statute1 = create_test_statute("stat1", vec![], vec![]);
        statute1.amendments = vec![AmendmentNode {
            target_id: "old-law".to_string(),
            version: Some(2),
            date: Some("2023-01-01".to_string()),
            description: "Updated rules".to_string(),
        }];

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: Vec::new(),
            statutes: vec![statute1],
        };

        let audit_trail = AmendmentAuditTrail::from_document(&doc);

        let history = audit_trail.get_history("old-law");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].source_statute_id, "stat1");
        assert_eq!(history[0].description, "Updated rules");
    }

    #[test]
    fn test_extract_jurisdiction_from_id() {
        assert_eq!(
            extract_jurisdiction_from_id("US-CA-law-1"),
            Some("US-CA".to_string())
        );
        assert_eq!(
            extract_jurisdiction_from_id("EU-FR-statute-2024"),
            Some("EU-FR".to_string())
        );
        assert_eq!(extract_jurisdiction_from_id("simple"), None);
    }

    #[test]
    fn test_infer_parent_jurisdiction() {
        assert_eq!(
            infer_parent_jurisdiction("US-CA-SF-law-1"),
            Some("US-CA".to_string())
        );
        assert_eq!(infer_parent_jurisdiction("US-CA-law-1"), None);
    }
}
