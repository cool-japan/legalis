//! RDF validation utilities.
//!
//! This module provides utilities to validate RDF exports for common issues:
//! - Syntax errors
//! - Missing required properties
//! - Invalid URIs
//! - Orphaned nodes

use crate::{RdfValue, Triple};
use std::collections::{HashMap, HashSet};

/// Validation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationIssue {
    /// Missing required property for a subject
    MissingRequiredProperty { subject: String, property: String },
    /// Invalid URI format
    InvalidUri { uri: String, reason: String },
    /// Orphaned blank node (not referenced)
    OrphanedBlankNode { node_id: String },
    /// Missing type declaration
    MissingType { subject: String },
    /// Empty literal value
    EmptyLiteral { subject: String, predicate: String },
}

/// Validation report.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// List of validation issues
    pub issues: Vec<ValidationIssue>,
    /// Number of triples validated
    pub triple_count: usize,
    /// Number of unique subjects
    pub subject_count: usize,
}

impl ValidationReport {
    /// Returns true if there are no validation issues.
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns the number of issues.
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Groups issues by type.
    pub fn issues_by_type(&self) -> HashMap<&'static str, Vec<&ValidationIssue>> {
        let mut grouped: HashMap<&'static str, Vec<&ValidationIssue>> = HashMap::new();

        for issue in &self.issues {
            let issue_type = match issue {
                ValidationIssue::MissingRequiredProperty { .. } => "MissingRequiredProperty",
                ValidationIssue::InvalidUri { .. } => "InvalidUri",
                ValidationIssue::OrphanedBlankNode { .. } => "OrphanedBlankNode",
                ValidationIssue::MissingType { .. } => "MissingType",
                ValidationIssue::EmptyLiteral { .. } => "EmptyLiteral",
            };
            grouped.entry(issue_type).or_default().push(issue);
        }

        grouped
    }
}

/// RDF validator.
#[derive(Debug)]
pub struct RdfValidator {
    /// Required properties for specific types
    required_properties: HashMap<String, Vec<String>>,
    /// Whether to check for orphaned blank nodes
    check_orphaned_nodes: bool,
    /// Whether to require type declarations
    require_types: bool,
}

impl Default for RdfValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl RdfValidator {
    /// Creates a new validator with default settings.
    pub fn new() -> Self {
        let mut required_properties = HashMap::new();

        // ELI LegalResource requirements
        required_properties.insert(
            "eli:LegalResource".to_string(),
            vec!["eli:title".to_string()],
        );

        // Legalis Statute requirements
        required_properties.insert(
            "legalis:Statute".to_string(),
            vec!["eli:title".to_string(), "dcterms:identifier".to_string()],
        );

        // SKOS Concept requirements
        required_properties.insert(
            "skos:Concept".to_string(),
            vec!["skos:prefLabel".to_string()],
        );

        // SKOS ConceptScheme requirements
        required_properties.insert(
            "skos:ConceptScheme".to_string(),
            vec!["skos:prefLabel".to_string()],
        );

        Self {
            required_properties,
            check_orphaned_nodes: true,
            require_types: true,
        }
    }

    /// Sets whether to check for orphaned blank nodes.
    pub fn with_orphaned_node_check(mut self, check: bool) -> Self {
        self.check_orphaned_nodes = check;
        self
    }

    /// Sets whether to require type declarations.
    pub fn with_type_requirement(mut self, require: bool) -> Self {
        self.require_types = require;
        self
    }

    /// Adds a required property for a specific type.
    pub fn add_required_property(
        &mut self,
        rdf_type: impl Into<String>,
        property: impl Into<String>,
    ) {
        self.required_properties
            .entry(rdf_type.into())
            .or_default()
            .push(property.into());
    }

    /// Validates a list of triples.
    pub fn validate(&self, triples: &[Triple]) -> ValidationReport {
        let mut issues = Vec::new();
        let mut subjects = HashSet::new();
        let mut subject_types: HashMap<String, Vec<String>> = HashMap::new();
        let mut subject_properties: HashMap<String, HashSet<String>> = HashMap::new();
        let mut blank_nodes_defined = HashSet::new();
        let mut blank_nodes_referenced = HashSet::new();

        // First pass: collect information
        for triple in triples {
            subjects.insert(triple.subject.clone());

            // Track types
            if triple.predicate == "rdf:type" {
                if let RdfValue::Uri(ref type_uri) = triple.object {
                    subject_types
                        .entry(triple.subject.clone())
                        .or_default()
                        .push(type_uri.clone());
                }
            }

            // Track properties
            subject_properties
                .entry(triple.subject.clone())
                .or_default()
                .insert(triple.predicate.clone());

            // Track blank nodes
            if let RdfValue::BlankNode(id) = &triple.object {
                blank_nodes_referenced.insert(id.clone());
            }

            if triple.subject.starts_with("_:") {
                blank_nodes_defined.insert(triple.subject.clone());
            }

            // Check for empty literals
            if let RdfValue::Literal(ref s, _) = triple.object {
                if s.is_empty() {
                    issues.push(ValidationIssue::EmptyLiteral {
                        subject: triple.subject.clone(),
                        predicate: triple.predicate.clone(),
                    });
                }
            }
        }

        // Check for missing types
        if self.require_types {
            for subject in &subjects {
                if !subject_types.contains_key(subject) && !subject.starts_with("_:") {
                    issues.push(ValidationIssue::MissingType {
                        subject: subject.clone(),
                    });
                }
            }
        }

        // Check required properties
        for (subject, types) in &subject_types {
            for rdf_type in types {
                if let Some(required) = self.required_properties.get(rdf_type) {
                    let properties = subject_properties.get(subject).cloned().unwrap_or_default();
                    for req_prop in required {
                        if !properties.contains(req_prop) {
                            issues.push(ValidationIssue::MissingRequiredProperty {
                                subject: subject.clone(),
                                property: req_prop.clone(),
                            });
                        }
                    }
                }
            }
        }

        // Check for orphaned blank nodes
        if self.check_orphaned_nodes {
            for node_id in &blank_nodes_defined {
                if !blank_nodes_referenced.contains(node_id) {
                    issues.push(ValidationIssue::OrphanedBlankNode {
                        node_id: node_id.clone(),
                    });
                }
            }
        }

        // Validate URIs
        for triple in triples {
            // Check subject URI
            if !triple.subject.starts_with("_:") && !is_valid_uri(&triple.subject) {
                issues.push(ValidationIssue::InvalidUri {
                    uri: triple.subject.clone(),
                    reason: "Invalid URI format".to_string(),
                });
            }

            // Check object URI
            if let RdfValue::Uri(ref uri) = triple.object {
                if !is_valid_uri(uri) && !uri.contains(':') {
                    issues.push(ValidationIssue::InvalidUri {
                        uri: uri.clone(),
                        reason: "Invalid URI format or missing namespace".to_string(),
                    });
                }
            }
        }

        ValidationReport {
            issues,
            triple_count: triples.len(),
            subject_count: subjects.len(),
        }
    }

    /// Validates triples and returns a summary string.
    pub fn validate_summary(&self, triples: &[Triple]) -> String {
        let report = self.validate(triples);

        if report.is_valid() {
            format!(
                "✓ Validation passed\n  {} triples, {} subjects\n  No issues found",
                report.triple_count, report.subject_count
            )
        } else {
            let mut summary = format!(
                "✗ Validation failed\n  {} triples, {} subjects\n  {} issues found:\n",
                report.triple_count,
                report.subject_count,
                report.issue_count()
            );

            let grouped = report.issues_by_type();
            for (issue_type, issues) in grouped {
                summary.push_str(&format!("  - {}: {} issues\n", issue_type, issues.len()));
            }

            summary
        }
    }
}

/// Checks if a string is a valid URI.
fn is_valid_uri(uri: &str) -> bool {
    // Basic URI validation
    if uri.starts_with("http://") || uri.starts_with("https://") {
        return true;
    }

    // Check for prefixed URI (namespace:local)
    if uri.contains(':') && !uri.starts_with("http") {
        let parts: Vec<&str> = uri.splitn(2, ':').collect();
        return parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("Test Statute"),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "dcterms:identifier".to_string(),
                object: RdfValue::string("statute-1"),
            },
        ]
    }

    #[test]
    fn test_valid_triples() {
        let validator = RdfValidator::new();
        let triples = create_sample_triples();
        let report = validator.validate(&triples);

        assert!(report.is_valid());
        assert_eq!(report.triple_count, 3);
        assert_eq!(report.subject_count, 1);
    }

    #[test]
    fn test_missing_required_property() {
        let validator = RdfValidator::new();
        let triples = vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            // Missing eli:title and dcterms:identifier
        ];

        let report = validator.validate(&triples);
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|i| matches!(
            i,
            ValidationIssue::MissingRequiredProperty { property, .. } if property == "eli:title"
        )));
    }

    #[test]
    fn test_missing_type() {
        let validator = RdfValidator::new();
        let triples = vec![Triple {
            subject: "https://example.org/statute/1".to_string(),
            predicate: "eli:title".to_string(),
            object: RdfValue::string("Test"),
        }];

        let report = validator.validate(&triples);
        assert!(!report.is_valid());
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::MissingType { .. }))
        );
    }

    #[test]
    fn test_empty_literal() {
        let validator = RdfValidator::new();
        let triples = vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string(""),
            },
        ];

        let report = validator.validate(&triples);
        assert!(!report.is_valid());
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::EmptyLiteral { .. }))
        );
    }

    #[test]
    fn test_validation_summary() {
        let validator = RdfValidator::new();
        let triples = create_sample_triples();
        let summary = validator.validate_summary(&triples);

        assert!(summary.contains("✓ Validation passed"));
        assert!(summary.contains("3 triples"));
    }

    #[test]
    fn test_is_valid_uri() {
        assert!(is_valid_uri("http://example.org/test"));
        assert!(is_valid_uri("https://example.org/test"));
        assert!(is_valid_uri("rdf:type"));
        assert!(is_valid_uri("legalis:Statute"));
        assert!(!is_valid_uri("not a uri"));
        assert!(!is_valid_uri(""));
    }

    #[test]
    fn test_issues_by_type() {
        let validator = RdfValidator::new();
        let triples = vec![Triple {
            subject: "https://example.org/statute/1".to_string(),
            predicate: "eli:title".to_string(),
            object: RdfValue::string(""),
        }];

        let report = validator.validate(&triples);
        let grouped = report.issues_by_type();

        assert!(grouped.contains_key("EmptyLiteral"));
        assert!(grouped.contains_key("MissingType"));
    }
}
