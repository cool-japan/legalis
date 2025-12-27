//! Statistics and metrics for legal document analysis.
//!
//! This module provides utilities to analyze and collect metrics about
//! legal documents, statutes, and their relationships.

use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use std::collections::{HashMap, HashSet};

/// Statistics about a legal document.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DocumentStatistics {
    /// Total number of statutes
    pub statute_count: usize,
    /// Total number of imports
    pub import_count: usize,
    /// Total number of conditions across all statutes
    pub total_conditions: usize,
    /// Total number of effects across all statutes
    pub total_effects: usize,
    /// Total number of exceptions
    pub total_exceptions: usize,
    /// Total number of amendments
    pub total_amendments: usize,
    /// Maximum condition depth (nesting level)
    pub max_condition_depth: usize,
    /// Average conditions per statute
    pub avg_conditions_per_statute: f64,
    /// Statutes with dependencies (REQUIRES)
    pub statutes_with_dependencies: usize,
    /// Statutes that supersede others
    pub statutes_with_supersedes: usize,
    /// Effect type distribution
    pub effect_types: HashMap<String, usize>,
    /// Condition type distribution
    pub condition_types: HashMap<String, usize>,
}

impl DocumentStatistics {
    /// Computes statistics for a legal document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut stats = Self {
            statute_count: doc.statutes.len(),
            import_count: doc.imports.len(),
            ..Default::default()
        };

        for statute in &doc.statutes {
            // Count conditions
            stats.total_conditions += statute.conditions.len();

            // Count effects
            stats.total_effects += statute.effects.len();

            // Count exceptions
            stats.total_exceptions += statute.exceptions.len();

            // Count amendments
            stats.total_amendments += statute.amendments.len();

            // Check for dependencies
            if !statute.requires.is_empty() {
                stats.statutes_with_dependencies += 1;
            }

            // Check for supersedes
            if !statute.supersedes.is_empty() {
                stats.statutes_with_supersedes += 1;
            }

            // Analyze conditions
            for condition in &statute.conditions {
                let depth = condition_depth(condition);
                if depth > stats.max_condition_depth {
                    stats.max_condition_depth = depth;
                }

                // Count condition types
                count_condition_types(condition, &mut stats.condition_types);
            }

            // Count effect types
            for effect in &statute.effects {
                *stats
                    .effect_types
                    .entry(effect.effect_type.clone())
                    .or_insert(0) += 1;
            }
        }

        // Calculate averages
        if stats.statute_count > 0 {
            stats.avg_conditions_per_statute =
                stats.total_conditions as f64 / stats.statute_count as f64;
        }

        stats
    }

    /// Returns a formatted report string.
    pub fn report(&self) -> String {
        let mut output = String::new();

        output.push_str("=== Legal Document Statistics ===\n\n");

        output.push_str(&format!("Statutes: {}\n", self.statute_count));
        output.push_str(&format!("Imports: {}\n", self.import_count));
        output.push_str(&format!("Total Conditions: {}\n", self.total_conditions));
        output.push_str(&format!("Total Effects: {}\n", self.total_effects));
        output.push_str(&format!("Total Exceptions: {}\n", self.total_exceptions));
        output.push_str(&format!("Total Amendments: {}\n", self.total_amendments));
        output.push_str(&format!(
            "Max Condition Depth: {}\n",
            self.max_condition_depth
        ));
        output.push_str(&format!(
            "Avg Conditions per Statute: {:.2}\n",
            self.avg_conditions_per_statute
        ));
        output.push_str(&format!(
            "Statutes with Dependencies: {}\n",
            self.statutes_with_dependencies
        ));
        output.push_str(&format!(
            "Statutes with Supersedes: {}\n",
            self.statutes_with_supersedes
        ));

        if !self.effect_types.is_empty() {
            output.push_str("\n--- Effect Type Distribution ---\n");
            let mut effect_vec: Vec<_> = self.effect_types.iter().collect();
            effect_vec.sort_by(|a, b| b.1.cmp(a.1));
            for (effect_type, count) in effect_vec {
                output.push_str(&format!("  {}: {}\n", effect_type, count));
            }
        }

        if !self.condition_types.is_empty() {
            output.push_str("\n--- Condition Type Distribution ---\n");
            let mut cond_vec: Vec<_> = self.condition_types.iter().collect();
            cond_vec.sort_by(|a, b| b.1.cmp(a.1));
            for (cond_type, count) in cond_vec {
                output.push_str(&format!("  {}: {}\n", cond_type, count));
            }
        }

        output
    }
}

/// Computes the maximum depth of nested conditions.
fn condition_depth(condition: &ConditionNode) -> usize {
    match condition {
        ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
            1 + condition_depth(left).max(condition_depth(right))
        }
        ConditionNode::Not(inner) => 1 + condition_depth(inner),
        _ => 1,
    }
}

/// Counts condition types recursively.
fn count_condition_types(condition: &ConditionNode, counts: &mut HashMap<String, usize>) {
    let cond_type = match condition {
        ConditionNode::Comparison { .. } => "Comparison",
        ConditionNode::HasAttribute { .. } => "HasAttribute",
        ConditionNode::Between { .. } => "Between",
        ConditionNode::In { .. } => "In",
        ConditionNode::Like { .. } => "Like",
        ConditionNode::Matches { .. } => "Matches",
        ConditionNode::InRange { .. } => "InRange",
        ConditionNode::NotInRange { .. } => "NotInRange",
        ConditionNode::TemporalComparison { .. } => "TemporalComparison",
        ConditionNode::And(left, right) => {
            count_condition_types(left, counts);
            count_condition_types(right, counts);
            "And"
        }
        ConditionNode::Or(left, right) => {
            count_condition_types(left, counts);
            count_condition_types(right, counts);
            "Or"
        }
        ConditionNode::Not(inner) => {
            count_condition_types(inner, counts);
            "Not"
        }
    };

    *counts.entry(cond_type.to_string()).or_insert(0) += 1;
}

/// Analyzes statute dependencies and relationships.
#[derive(Debug, Clone, Default)]
pub struct DependencyAnalysis {
    /// Map of statute ID to statutes it requires
    pub dependencies: HashMap<String, Vec<String>>,
    /// Map of statute ID to statutes it supersedes
    pub supersedes: HashMap<String, Vec<String>>,
    /// Statutes that have no dependencies
    pub independent_statutes: Vec<String>,
    /// Statutes that are not required by any other statute
    pub leaf_statutes: Vec<String>,
}

impl DependencyAnalysis {
    /// Analyzes dependencies in a legal document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut analysis = Self::default();
        let mut all_statute_ids: HashSet<String> = HashSet::new();
        let mut required_by_others: HashSet<String> = HashSet::new();

        // First pass: collect all statute IDs and build dependency maps
        for statute in &doc.statutes {
            all_statute_ids.insert(statute.id.clone());

            if !statute.requires.is_empty() {
                analysis
                    .dependencies
                    .insert(statute.id.clone(), statute.requires.clone());
                for req in &statute.requires {
                    required_by_others.insert(req.clone());
                }
            } else {
                analysis.independent_statutes.push(statute.id.clone());
            }

            if !statute.supersedes.is_empty() {
                analysis
                    .supersedes
                    .insert(statute.id.clone(), statute.supersedes.clone());
            }
        }

        // Find leaf statutes (not required by any other)
        for id in &all_statute_ids {
            if !required_by_others.contains(id) {
                analysis.leaf_statutes.push(id.clone());
            }
        }

        analysis.leaf_statutes.sort();
        analysis.independent_statutes.sort();

        analysis
    }

    /// Returns a formatted report string.
    pub fn report(&self) -> String {
        let mut output = String::new();

        output.push_str("=== Dependency Analysis ===\n\n");

        output.push_str(&format!(
            "Independent Statutes (no dependencies): {}\n",
            self.independent_statutes.len()
        ));
        for id in &self.independent_statutes {
            output.push_str(&format!("  - {}\n", id));
        }

        output.push_str(&format!(
            "\nLeaf Statutes (not required by others): {}\n",
            self.leaf_statutes.len()
        ));
        for id in &self.leaf_statutes {
            output.push_str(&format!("  - {}\n", id));
        }

        if !self.dependencies.is_empty() {
            output.push_str("\n--- Dependency Graph ---\n");
            let mut dep_vec: Vec<_> = self.dependencies.iter().collect();
            dep_vec.sort_by_key(|a| a.0);
            for (statute_id, requires) in dep_vec {
                output.push_str(&format!("{} requires:\n", statute_id));
                for req in requires {
                    output.push_str(&format!("  -> {}\n", req));
                }
            }
        }

        if !self.supersedes.is_empty() {
            output.push_str("\n--- Supersedes Relationships ---\n");
            let mut super_vec: Vec<_> = self.supersedes.iter().collect();
            super_vec.sort_by_key(|a| a.0);
            for (statute_id, supersedes) in super_vec {
                output.push_str(&format!("{} supersedes:\n", statute_id));
                for sup in supersedes {
                    output.push_str(&format!("  -> {}\n", sup));
                }
            }
        }

        output
    }
}

/// Complexity metrics for a statute.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexityMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Number of conditions
    pub condition_count: usize,
    /// Maximum condition nesting depth
    pub max_depth: usize,
    /// Number of effects
    pub effect_count: usize,
    /// Number of exceptions
    pub exception_count: usize,
    /// Number of amendments
    pub amendment_count: usize,
    /// Number of dependencies (REQUIRES)
    pub dependency_count: usize,
    /// Overall complexity score (heuristic)
    pub complexity_score: f64,
}

impl ComplexityMetrics {
    /// Computes complexity metrics for a statute.
    pub fn from_statute(statute: &StatuteNode) -> Self {
        let condition_count = statute.conditions.len();
        let max_depth = statute
            .conditions
            .iter()
            .map(condition_depth)
            .max()
            .unwrap_or(0);
        let effect_count = statute.effects.len();
        let exception_count = statute.exceptions.len();
        let amendment_count = statute.amendments.len();
        let dependency_count = statute.requires.len();

        // Heuristic complexity score
        let complexity_score = condition_count as f64 * 1.0
            + max_depth as f64 * 2.0
            + effect_count as f64 * 0.5
            + exception_count as f64 * 1.5
            + amendment_count as f64 * 1.0
            + dependency_count as f64 * 0.5;

        Self {
            statute_id: statute.id.clone(),
            condition_count,
            max_depth,
            effect_count,
            exception_count,
            amendment_count,
            dependency_count,
            complexity_score,
        }
    }
}

/// Computes complexity metrics for all statutes in a document.
pub fn analyze_complexity(doc: &LegalDocument) -> Vec<ComplexityMetrics> {
    let mut metrics: Vec<_> = doc
        .statutes
        .iter()
        .map(ComplexityMetrics::from_statute)
        .collect();

    // Sort by complexity score (descending)
    metrics.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());

    metrics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ConditionValue, EffectNode};

    fn sample_statute() -> StatuteNode {
        StatuteNode {
            id: "test-statute".to_string(),
            title: "Test Statute".to_string(),
            conditions: vec![
                ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                },
                ConditionNode::And(
                    Box::new(ConditionNode::HasAttribute {
                        key: "citizen".to_string(),
                    }),
                    Box::new(ConditionNode::Not(Box::new(ConditionNode::HasAttribute {
                        key: "convicted".to_string(),
                    }))),
                ),
            ],
            effects: vec![EffectNode {
                effect_type: "grant".to_string(),
                description: "Voting rights".to_string(),
                parameters: vec![],
            }],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec!["citizenship-law".to_string()],
        }
    }

    #[test]
    fn test_condition_depth() {
        let simple = ConditionNode::HasAttribute {
            key: "test".to_string(),
        };
        assert_eq!(condition_depth(&simple), 1);

        let nested = ConditionNode::And(
            Box::new(ConditionNode::HasAttribute {
                key: "a".to_string(),
            }),
            Box::new(ConditionNode::Not(Box::new(ConditionNode::HasAttribute {
                key: "b".to_string(),
            }))),
        );
        assert_eq!(condition_depth(&nested), 3);
    }

    #[test]
    fn test_complexity_metrics() {
        let statute = sample_statute();
        let metrics = ComplexityMetrics::from_statute(&statute);

        assert_eq!(metrics.statute_id, "test-statute");
        assert_eq!(metrics.condition_count, 2);
        assert_eq!(metrics.max_depth, 3);
        assert_eq!(metrics.effect_count, 1);
        assert_eq!(metrics.dependency_count, 1);
        assert!(metrics.complexity_score > 0.0);
    }

    #[test]
    fn test_document_statistics() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let stats = DocumentStatistics::from_document(&doc);

        assert_eq!(stats.statute_count, 1);
        assert_eq!(stats.import_count, 0);
        assert_eq!(stats.total_conditions, 2);
        assert_eq!(stats.total_effects, 1);
        assert_eq!(stats.statutes_with_dependencies, 1);
    }

    #[test]
    fn test_dependency_analysis() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let analysis = DependencyAnalysis::from_document(&doc);

        assert_eq!(analysis.dependencies.len(), 1);
        assert_eq!(analysis.independent_statutes.len(), 0);
    }
}
