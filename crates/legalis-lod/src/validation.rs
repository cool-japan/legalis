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
    /// Ontology inconsistency (e.g., property used with wrong domain/range)
    OntologyInconsistency { subject: String, issue: String },
    /// Cyclic reference detected
    CyclicReference { subject: String, path: Vec<String> },
    /// Conflicting property values
    ConflictingValues {
        subject: String,
        property: String,
        values: Vec<String>,
    },
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
                ValidationIssue::OntologyInconsistency { .. } => "OntologyInconsistency",
                ValidationIssue::CyclicReference { .. } => "CyclicReference",
                ValidationIssue::ConflictingValues { .. } => "ConflictingValues",
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
    /// Whether to check for ontology consistency
    check_ontology_consistency: bool,
    /// Property domain constraints (property -> expected subject type)
    property_domains: HashMap<String, Vec<String>>,
    /// Property range constraints (property -> expected object type)
    property_ranges: HashMap<String, Vec<String>>,
    /// Functional properties (can have only one value)
    functional_properties: HashSet<String>,
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
        let mut property_domains = HashMap::new();
        let mut property_ranges = HashMap::new();
        let mut functional_properties = HashSet::new();

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

        // Property domain constraints
        property_domains.insert(
            "legalis:hasEffect".to_string(),
            vec!["legalis:Statute".to_string()],
        );
        property_domains.insert(
            "legalis:hasPrecondition".to_string(),
            vec!["legalis:Statute".to_string()],
        );
        property_domains.insert(
            "legalis:effectType".to_string(),
            vec!["legalis:Effect".to_string()],
        );

        // Property range constraints
        property_ranges.insert(
            "legalis:hasEffect".to_string(),
            vec!["legalis:Effect".to_string()],
        );
        property_ranges.insert(
            "legalis:hasPrecondition".to_string(),
            vec![
                "legalis:Condition".to_string(),
                "legalis:AgeCondition".to_string(),
                "legalis:IncomeCondition".to_string(),
                "legalis:AndCondition".to_string(),
                "legalis:OrCondition".to_string(),
                "legalis:NotCondition".to_string(),
            ],
        );

        // Functional properties (can have only one value)
        functional_properties.insert("eli:title".to_string());
        functional_properties.insert("dcterms:identifier".to_string());
        functional_properties.insert("legalis:effectType".to_string());

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
            check_ontology_consistency: true,
            property_domains,
            property_ranges,
            functional_properties,
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

    /// Sets whether to check for ontology consistency.
    pub fn with_ontology_consistency_check(mut self, check: bool) -> Self {
        self.check_ontology_consistency = check;
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

    /// Adds a property domain constraint.
    pub fn add_property_domain(&mut self, property: impl Into<String>, domain: impl Into<String>) {
        self.property_domains
            .entry(property.into())
            .or_default()
            .push(domain.into());
    }

    /// Adds a property range constraint.
    pub fn add_property_range(&mut self, property: impl Into<String>, range: impl Into<String>) {
        self.property_ranges
            .entry(property.into())
            .or_default()
            .push(range.into());
    }

    /// Marks a property as functional (can have only one value).
    pub fn add_functional_property(&mut self, property: impl Into<String>) {
        self.functional_properties.insert(property.into());
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
            if triple.predicate == "rdf:type"
                && let RdfValue::Uri(ref type_uri) = triple.object
            {
                subject_types
                    .entry(triple.subject.clone())
                    .or_default()
                    .push(type_uri.clone());
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
            if let RdfValue::Literal(ref s, _) = triple.object
                && s.is_empty()
            {
                issues.push(ValidationIssue::EmptyLiteral {
                    subject: triple.subject.clone(),
                    predicate: triple.predicate.clone(),
                });
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
            if let RdfValue::Uri(ref uri) = triple.object
                && !is_valid_uri(uri)
                && !uri.contains(':')
            {
                issues.push(ValidationIssue::InvalidUri {
                    uri: uri.clone(),
                    reason: "Invalid URI format or missing namespace".to_string(),
                });
            }
        }

        // Ontology consistency checks
        if self.check_ontology_consistency {
            // Collect property values for functional property checks
            let mut property_values: HashMap<(String, String), Vec<String>> = HashMap::new();

            for triple in triples {
                // Check property domain constraints
                if let Some(expected_domains) = self.property_domains.get(&triple.predicate)
                    && let Some(types) = subject_types.get(&triple.subject)
                {
                    let has_valid_domain = types.iter().any(|t| expected_domains.contains(t));
                    if !has_valid_domain {
                        issues.push(ValidationIssue::OntologyInconsistency {
                                subject: triple.subject.clone(),
                                issue: format!(
                                    "Property '{}' used with subject of type {:?}, expected one of {:?}",
                                    triple.predicate, types, expected_domains
                                ),
                            });
                    }
                }

                // Check property range constraints
                if let Some(expected_ranges) = self.property_ranges.get(&triple.predicate)
                    && let RdfValue::Uri(ref object_uri) = triple.object
                {
                    // Find the object's types
                    if let Some(object_types) = subject_types.get(object_uri) {
                        let has_valid_range =
                            object_types.iter().any(|t| expected_ranges.contains(t));
                        if !has_valid_range {
                            issues.push(ValidationIssue::OntologyInconsistency {
                                subject: triple.subject.clone(),
                                issue: format!(
                                    "Property '{}' has object of type {:?}, expected one of {:?}",
                                    triple.predicate, object_types, expected_ranges
                                ),
                            });
                        }
                    }
                }

                // Collect values for functional property check
                if self.functional_properties.contains(&triple.predicate) {
                    let key = (triple.subject.clone(), triple.predicate.clone());
                    property_values
                        .entry(key)
                        .or_default()
                        .push(format!("{:?}", triple.object));
                }
            }

            // Check functional properties (should have at most one value)
            for ((subject, property), values) in property_values {
                if values.len() > 1 {
                    issues.push(ValidationIssue::ConflictingValues {
                        subject,
                        property,
                        values,
                    });
                }
            }

            // Check for cyclic references in condition hierarchies
            for triple in triples {
                if (triple.predicate == "legalis:leftOperand"
                    || triple.predicate == "legalis:rightOperand")
                    && let RdfValue::Uri(ref target) = triple.object
                {
                    let mut visited = HashSet::new();
                    let mut path = vec![triple.subject.clone()];
                    if Self::has_cycle(triples, &triple.subject, target, &mut visited, &mut path) {
                        issues.push(ValidationIssue::CyclicReference {
                            subject: triple.subject.clone(),
                            path,
                        });
                    }
                }
            }
        }

        ValidationReport {
            issues,
            triple_count: triples.len(),
            subject_count: subjects.len(),
        }
    }

    /// Checks for cyclic references in the RDF graph.
    fn has_cycle(
        triples: &[Triple],
        start: &str,
        current: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if current == start && !path.is_empty() {
            return true;
        }

        if visited.contains(current) {
            return false;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        for triple in triples {
            if triple.subject == current
                && (triple.predicate == "legalis:leftOperand"
                    || triple.predicate == "legalis:rightOperand")
                && let RdfValue::Uri(ref next) = triple.object
                && Self::has_cycle(triples, start, next, visited, path)
            {
                return true;
            }
        }

        path.pop();
        false
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

    #[test]
    fn test_property_domain_validation() {
        let validator = RdfValidator::new();

        // Create triples with wrong domain (Effect using a Statute-only property)
        let triples = vec![
            Triple {
                subject: "https://example.org/effect/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Effect".to_string()),
            },
            Triple {
                subject: "https://example.org/effect/1".to_string(),
                predicate: "legalis:hasPrecondition".to_string(),
                object: RdfValue::Uri("https://example.org/cond/1".to_string()),
            },
        ];

        let report = validator.validate(&triples);
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::OntologyInconsistency { .. }))
        );
    }

    #[test]
    fn test_property_range_validation() {
        let validator = RdfValidator::new();

        // Create triples with wrong range (hasEffect pointing to a Condition instead of Effect)
        let triples = vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "legalis:hasEffect".to_string(),
                object: RdfValue::Uri("https://example.org/obj/1".to_string()),
            },
            Triple {
                subject: "https://example.org/obj/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Condition".to_string()),
            },
        ];

        let report = validator.validate(&triples);
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::OntologyInconsistency { .. }))
        );
    }

    #[test]
    fn test_functional_property_validation() {
        let validator = RdfValidator::new();

        // Create triples with multiple values for a functional property
        let triples = vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("First Title"),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("Second Title"),
            },
        ];

        let report = validator.validate(&triples);
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::ConflictingValues { .. }))
        );
    }

    #[test]
    fn test_disable_ontology_consistency_check() {
        let validator = RdfValidator::new().with_ontology_consistency_check(false);

        // Create triples with wrong domain, but consistency check is disabled
        let triples = vec![
            Triple {
                subject: "https://example.org/effect/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Effect".to_string()),
            },
            Triple {
                subject: "https://example.org/effect/1".to_string(),
                predicate: "legalis:hasPrecondition".to_string(),
                object: RdfValue::Uri("https://example.org/cond/1".to_string()),
            },
        ];

        let report = validator.validate(&triples);
        // Should not have ontology inconsistency issues since check is disabled
        assert!(
            !report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::OntologyInconsistency { .. }))
        );
    }

    #[test]
    fn test_add_custom_property_constraints() {
        let mut validator = RdfValidator::new();

        // Add custom domain constraint
        validator.add_property_domain("custom:myProperty", "custom:MyType");

        // Add custom functional property
        validator.add_functional_property("custom:uniqueId");

        // Test with triples that violate the custom constraints
        let triples = vec![
            Triple {
                subject: "https://example.org/obj/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("custom:WrongType".to_string()),
            },
            Triple {
                subject: "https://example.org/obj/1".to_string(),
                predicate: "custom:myProperty".to_string(),
                object: RdfValue::string("value"),
            },
        ];

        let report = validator.validate(&triples);
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::OntologyInconsistency { .. }))
        );
    }

    #[test]
    fn test_valid_ontology() {
        let validator = RdfValidator::new();

        // Create valid triples with correct domain and range
        let triples = vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("Test"),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "dcterms:identifier".to_string(),
                object: RdfValue::string("test-1"),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "legalis:hasEffect".to_string(),
                object: RdfValue::Uri("https://example.org/effect/1".to_string()),
            },
            Triple {
                subject: "https://example.org/effect/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Effect".to_string()),
            },
        ];

        let report = validator.validate(&triples);
        // Should have no ontology inconsistency issues
        assert!(
            !report
                .issues
                .iter()
                .any(|i| matches!(i, ValidationIssue::OntologyInconsistency { .. }))
        );
    }
}
