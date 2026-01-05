//! Ontology metrics and quality assessment.
//!
//! This module provides comprehensive metrics for evaluating ontology quality:
//! - Structural metrics (size, depth, breadth)
//! - Richness metrics (property usage, axiom diversity)
//! - Cohesion and coupling metrics
//! - Documentation quality metrics

use crate::{RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Structural metrics about an ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralMetrics {
    /// Total number of triples
    pub triple_count: usize,
    /// Number of classes
    pub class_count: usize,
    /// Number of properties (object + datatype)
    pub property_count: usize,
    /// Number of object properties
    pub object_property_count: usize,
    /// Number of datatype properties
    pub datatype_property_count: usize,
    /// Number of individuals
    pub individual_count: usize,
    /// Maximum depth of class hierarchy
    pub max_hierarchy_depth: usize,
    /// Average number of children per class
    pub avg_children_per_class: f64,
    /// Number of root classes (no superclass)
    pub root_class_count: usize,
    /// Number of leaf classes (no subclasses)
    pub leaf_class_count: usize,
}

impl Default for StructuralMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl StructuralMetrics {
    /// Creates empty structural metrics.
    pub fn new() -> Self {
        Self {
            triple_count: 0,
            class_count: 0,
            property_count: 0,
            object_property_count: 0,
            datatype_property_count: 0,
            individual_count: 0,
            max_hierarchy_depth: 0,
            avg_children_per_class: 0.0,
            root_class_count: 0,
            leaf_class_count: 0,
        }
    }
}

/// Richness metrics measuring ontology expressiveness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichnessMetrics {
    /// Ratio of properties to classes
    pub property_class_ratio: f64,
    /// Ratio of individuals to classes
    pub individual_class_ratio: f64,
    /// Average number of properties per class
    pub avg_properties_per_class: f64,
    /// Percentage of classes with documentation
    pub documented_class_percentage: f64,
    /// Percentage of properties with documentation
    pub documented_property_percentage: f64,
    /// Number of different axiom types used
    pub axiom_type_diversity: usize,
    /// Average number of restrictions per class
    pub avg_restrictions_per_class: f64,
}

impl Default for RichnessMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RichnessMetrics {
    /// Creates empty richness metrics.
    pub fn new() -> Self {
        Self {
            property_class_ratio: 0.0,
            individual_class_ratio: 0.0,
            avg_properties_per_class: 0.0,
            documented_class_percentage: 0.0,
            documented_property_percentage: 0.0,
            axiom_type_diversity: 0,
            avg_restrictions_per_class: 0.0,
        }
    }
}

/// Cohesion metrics measuring how well-connected the ontology is.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohesionMetrics {
    /// Number of connected components
    pub connected_components: usize,
    /// Size of largest connected component
    pub largest_component_size: usize,
    /// Average path length between classes
    pub avg_path_length: f64,
    /// Graph density
    pub density: f64,
}

impl Default for CohesionMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CohesionMetrics {
    /// Creates empty cohesion metrics.
    pub fn new() -> Self {
        Self {
            connected_components: 0,
            largest_component_size: 0,
            avg_path_length: 0.0,
            density: 0.0,
        }
    }
}

/// Complete quality assessment report for an ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    /// Structural metrics
    pub structural: StructuralMetrics,
    /// Richness metrics
    pub richness: RichnessMetrics,
    /// Cohesion metrics
    pub cohesion: CohesionMetrics,
    /// Overall quality score (0-100)
    pub overall_score: f64,
    /// Quality issues found
    pub issues: Vec<QualityIssue>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl QualityAssessment {
    /// Creates a new quality assessment.
    pub fn new() -> Self {
        Self {
            structural: StructuralMetrics::new(),
            richness: RichnessMetrics::new(),
            cohesion: CohesionMetrics::new(),
            overall_score: 0.0,
            issues: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    /// Generates a summary report.
    pub fn summary(&self) -> String {
        format!(
            "Ontology Quality Assessment\n\
             Overall Score: {:.1}/100\n\
             \n\
             Structural Metrics:\n\
             - Classes: {}\n\
             - Properties: {}\n\
             - Individuals: {}\n\
             - Max Hierarchy Depth: {}\n\
             \n\
             Richness Metrics:\n\
             - Property/Class Ratio: {:.2}\n\
             - Individual/Class Ratio: {:.2}\n\
             - Documented Classes: {:.1}%\n\
             \n\
             Issues Found: {}\n\
             Recommendations: {}",
            self.overall_score,
            self.structural.class_count,
            self.structural.property_count,
            self.structural.individual_count,
            self.structural.max_hierarchy_depth,
            self.richness.property_class_ratio,
            self.richness.individual_class_ratio,
            self.richness.documented_class_percentage,
            self.issues.len(),
            self.recommendations.len()
        )
    }
}

impl Default for QualityAssessment {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of quality issues.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityIssueType {
    /// Missing documentation
    MissingDocumentation,
    /// Orphan class (no connections)
    OrphanClass,
    /// Deep hierarchy (too many levels)
    DeepHierarchy,
    /// Wide class (too many children)
    WideClass,
    /// Empty class (no instances)
    EmptyClass,
    /// Unused property
    UnusedProperty,
    /// Circular dependency
    CircularDependency,
    /// Other issue
    Other,
}

/// Represents a quality issue in an ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Type of issue
    pub issue_type: QualityIssueType,
    /// Severity (1-10, 10 being most severe)
    pub severity: u8,
    /// Entity URI that has the issue
    pub entity_uri: String,
    /// Description of the issue
    pub description: String,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

impl QualityIssue {
    /// Creates a new quality issue.
    pub fn new(
        issue_type: QualityIssueType,
        severity: u8,
        entity_uri: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            issue_type,
            severity: severity.min(10),
            entity_uri: entity_uri.into(),
            description: description.into(),
            suggested_fix: None,
        }
    }

    /// Sets the suggested fix.
    #[allow(dead_code)]
    pub fn with_suggested_fix(mut self, fix: impl Into<String>) -> Self {
        self.suggested_fix = Some(fix.into());
        self
    }
}

/// Analyzer for computing ontology metrics and quality assessment.
pub struct OntologyAnalyzer {
    /// RDF triples to analyze
    triples: Vec<Triple>,
}

impl OntologyAnalyzer {
    /// Creates a new analyzer.
    pub fn new(triples: Vec<Triple>) -> Self {
        Self { triples }
    }

    /// Performs complete quality assessment.
    pub fn assess_quality(&self) -> QualityAssessment {
        let mut assessment = QualityAssessment::new();

        // Compute structural metrics
        assessment.structural = self.compute_structural_metrics();

        // Compute richness metrics
        assessment.richness = self.compute_richness_metrics(&assessment.structural);

        // Compute cohesion metrics
        assessment.cohesion = self.compute_cohesion_metrics();

        // Find quality issues
        assessment.issues = self.find_quality_issues(&assessment.structural, &assessment.richness);

        // Generate recommendations
        assessment.recommendations = self.generate_recommendations(&assessment);

        // Compute overall score
        assessment.overall_score = self.compute_overall_score(&assessment);

        assessment
    }

    /// Computes structural metrics.
    fn compute_structural_metrics(&self) -> StructuralMetrics {
        let mut metrics = StructuralMetrics::new();
        metrics.triple_count = self.triples.len();

        let mut classes = HashSet::new();
        let mut object_properties = HashSet::new();
        let mut datatype_properties = HashSet::new();
        let mut individuals = HashSet::new();
        let mut subclass_relations: HashMap<String, Vec<String>> = HashMap::new();

        for triple in &self.triples {
            // Count classes
            if triple.predicate == "rdf:type" {
                if let RdfValue::Uri(ref type_uri) = triple.object {
                    if type_uri.contains("Class") {
                        classes.insert(triple.subject.clone());
                    } else if type_uri.contains("ObjectProperty") {
                        object_properties.insert(triple.subject.clone());
                    } else if type_uri.contains("DatatypeProperty") {
                        datatype_properties.insert(triple.subject.clone());
                    } else {
                        individuals.insert(triple.subject.clone());
                    }
                }
            }

            // Track subclass relations
            if triple.predicate == "rdfs:subClassOf" {
                if let RdfValue::Uri(ref superclass) = triple.object {
                    subclass_relations
                        .entry(superclass.clone())
                        .or_default()
                        .push(triple.subject.clone());
                }
            }
        }

        metrics.class_count = classes.len();
        metrics.object_property_count = object_properties.len();
        metrics.datatype_property_count = datatype_properties.len();
        metrics.property_count = metrics.object_property_count + metrics.datatype_property_count;
        metrics.individual_count = individuals.len();

        // Compute hierarchy metrics
        let (max_depth, root_count, leaf_count) =
            self.compute_hierarchy_metrics(&classes, &subclass_relations);
        metrics.max_hierarchy_depth = max_depth;
        metrics.root_class_count = root_count;
        metrics.leaf_class_count = leaf_count;

        // Average children per class
        if !subclass_relations.is_empty() {
            let total_children: usize = subclass_relations.values().map(|v| v.len()).sum();
            metrics.avg_children_per_class =
                total_children as f64 / subclass_relations.len() as f64;
        }

        metrics
    }

    /// Computes hierarchy depth metrics.
    fn compute_hierarchy_metrics(
        &self,
        classes: &HashSet<String>,
        subclass_relations: &HashMap<String, Vec<String>>,
    ) -> (usize, usize, usize) {
        // Find root classes (not a subclass of anything)
        let all_subclasses: HashSet<_> =
            subclass_relations.values().flat_map(|v| v.iter()).collect();

        let root_classes: Vec<_> = classes
            .iter()
            .filter(|c| !all_subclasses.contains(c))
            .collect();

        // Count leaf classes (no subclasses)
        let leaf_count = classes
            .iter()
            .filter(|c| !subclass_relations.contains_key(*c))
            .count();

        // Compute max depth using BFS
        let max_depth = root_classes
            .iter()
            .map(|root| self.compute_depth(root, subclass_relations, 0))
            .max()
            .unwrap_or(0);

        (max_depth, root_classes.len(), leaf_count)
    }

    /// Recursively computes depth from a class.
    #[allow(clippy::only_used_in_recursion)]
    fn compute_depth(
        &self,
        class: &str,
        subclass_relations: &HashMap<String, Vec<String>>,
        current_depth: usize,
    ) -> usize {
        if let Some(children) = subclass_relations.get(class) {
            children
                .iter()
                .map(|child| self.compute_depth(child, subclass_relations, current_depth + 1))
                .max()
                .unwrap_or(current_depth)
        } else {
            current_depth
        }
    }

    /// Computes richness metrics.
    fn compute_richness_metrics(&self, structural: &StructuralMetrics) -> RichnessMetrics {
        let mut metrics = RichnessMetrics::new();

        if structural.class_count > 0 {
            metrics.property_class_ratio =
                structural.property_count as f64 / structural.class_count as f64;
            metrics.individual_class_ratio =
                structural.individual_count as f64 / structural.class_count as f64;
        }

        // Count documented entities
        let mut classes_with_labels = HashSet::new();
        let mut properties_with_labels = HashSet::new();

        for triple in &self.triples {
            if triple.predicate == "rdfs:label" || triple.predicate == "rdfs:comment" {
                // Check if subject is a class or property
                if self.is_class(&triple.subject) {
                    classes_with_labels.insert(&triple.subject);
                } else if self.is_property(&triple.subject) {
                    properties_with_labels.insert(&triple.subject);
                }
            }
        }

        let documented_classes = classes_with_labels.len();
        let documented_properties = properties_with_labels.len();

        if structural.class_count > 0 {
            metrics.documented_class_percentage =
                (documented_classes as f64 / structural.class_count as f64) * 100.0;
        }

        if structural.property_count > 0 {
            metrics.documented_property_percentage =
                (documented_properties as f64 / structural.property_count as f64) * 100.0;
        }

        // Count axiom type diversity
        let axiom_types: HashSet<_> = self.triples.iter().map(|t| &t.predicate).collect();
        metrics.axiom_type_diversity = axiom_types.len();

        metrics
    }

    /// Computes cohesion metrics.
    fn compute_cohesion_metrics(&self) -> CohesionMetrics {
        let mut metrics = CohesionMetrics::new();

        // Build adjacency graph
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        for triple in &self.triples {
            if let RdfValue::Uri(ref target) = triple.object {
                graph
                    .entry(triple.subject.clone())
                    .or_default()
                    .insert(target.clone());
            }
        }

        // Compute connected components
        let components = self.find_connected_components(&graph);
        metrics.connected_components = components.len();

        if let Some(largest) = components.iter().max_by_key(|c| c.len()) {
            metrics.largest_component_size = largest.len();
        }

        // Compute density
        let node_count = graph.len();
        if node_count > 1 {
            let edge_count: usize = graph.values().map(|neighbors| neighbors.len()).sum();
            let max_edges = node_count * (node_count - 1);
            metrics.density = edge_count as f64 / max_edges as f64;
        }

        metrics
    }

    /// Finds connected components in a graph.
    fn find_connected_components(
        &self,
        graph: &HashMap<String, HashSet<String>>,
    ) -> Vec<HashSet<String>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                let component = self.explore_component(node, graph, &mut visited);
                components.push(component);
            }
        }

        components
    }

    /// Explores a connected component using DFS.
    fn explore_component(
        &self,
        start: &str,
        graph: &HashMap<String, HashSet<String>>,
        visited: &mut HashSet<String>,
    ) -> HashSet<String> {
        let mut component = HashSet::new();
        let mut stack = vec![start.to_string()];

        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }

            visited.insert(node.clone());
            component.insert(node.clone());

            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        component
    }

    /// Finds quality issues.
    fn find_quality_issues(
        &self,
        structural: &StructuralMetrics,
        richness: &RichnessMetrics,
    ) -> Vec<QualityIssue> {
        let mut issues = Vec::new();

        // Check for deep hierarchy
        if structural.max_hierarchy_depth > 7 {
            issues.push(QualityIssue::new(
                QualityIssueType::DeepHierarchy,
                7,
                "ontology",
                format!(
                    "Hierarchy depth ({}) exceeds recommended maximum (7)",
                    structural.max_hierarchy_depth
                ),
            ));
        }

        // Check for poor documentation
        if richness.documented_class_percentage < 50.0 {
            issues.push(QualityIssue::new(
                QualityIssueType::MissingDocumentation,
                8,
                "classes",
                format!(
                    "Only {:.1}% of classes are documented",
                    richness.documented_class_percentage
                ),
            ));
        }

        issues
    }

    /// Generates recommendations.
    fn generate_recommendations(&self, assessment: &QualityAssessment) -> Vec<String> {
        let mut recommendations = Vec::new();

        if assessment.richness.documented_class_percentage < 80.0 {
            recommendations
                .push("Add rdfs:label and rdfs:comment to undocumented classes".to_string());
        }

        if assessment.structural.max_hierarchy_depth > 7 {
            recommendations.push("Consider flattening the class hierarchy".to_string());
        }

        if assessment.richness.property_class_ratio < 1.0 {
            recommendations.push(
                "Consider adding more properties to increase ontology expressiveness".to_string(),
            );
        }

        recommendations
    }

    /// Computes overall quality score.
    fn compute_overall_score(&self, assessment: &QualityAssessment) -> f64 {
        let mut score = 100.0;

        // Deduct for issues
        for issue in &assessment.issues {
            score -= issue.severity as f64;
        }

        // Deduct for poor documentation
        let doc_score = (assessment.richness.documented_class_percentage
            + assessment.richness.documented_property_percentage)
            / 2.0;
        score = score.min(doc_score + 20.0);

        score.max(0.0)
    }

    /// Checks if a URI represents a class.
    fn is_class(&self, uri: &str) -> bool {
        self.triples.iter().any(|t| {
            t.subject == uri
                && t.predicate == "rdf:type"
                && matches!(&t.object, RdfValue::Uri(u) if u.contains("Class"))
        })
    }

    /// Checks if a URI represents a property.
    fn is_property(&self, uri: &str) -> bool {
        self.triples.iter().any(|t| {
            t.subject == uri
                && t.predicate == "rdf:type"
                && matches!(&t.object, RdfValue::Uri(u) if u.contains("Property"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "http://example.org/Person".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("owl:Class".to_string()),
            },
            Triple {
                subject: "http://example.org/Company".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("owl:Class".to_string()),
            },
            Triple {
                subject: "http://example.org/hasName".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("owl:DatatypeProperty".to_string()),
            },
        ]
    }

    #[test]
    fn test_structural_metrics() {
        let analyzer = OntologyAnalyzer::new(sample_triples());
        let metrics = analyzer.compute_structural_metrics();

        assert_eq!(metrics.triple_count, 3);
        assert_eq!(metrics.class_count, 2);
        assert_eq!(metrics.datatype_property_count, 1);
    }

    #[test]
    fn test_quality_assessment() {
        let analyzer = OntologyAnalyzer::new(sample_triples());
        let assessment = analyzer.assess_quality();

        assert_eq!(assessment.structural.class_count, 2);
        assert!(assessment.overall_score >= 0.0 && assessment.overall_score <= 100.0);
    }

    #[test]
    fn test_quality_issue() {
        let issue = QualityIssue::new(
            QualityIssueType::MissingDocumentation,
            5,
            "http://example.org/MyClass",
            "Class lacks documentation",
        );

        assert_eq!(issue.severity, 5);
        assert_eq!(issue.entity_uri, "http://example.org/MyClass");
    }

    #[test]
    fn test_assessment_summary() {
        let analyzer = OntologyAnalyzer::new(sample_triples());
        let assessment = analyzer.assess_quality();
        let summary = assessment.summary();

        assert!(summary.contains("Overall Score"));
        assert!(summary.contains("Classes"));
    }
}
