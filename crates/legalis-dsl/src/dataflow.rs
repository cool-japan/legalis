//! Data flow analysis for statute dependencies.
//!
//! This module provides analysis of how legal effects and conditions
//! flow through the statute dependency graph.

use crate::ast::{ConditionNode, LegalDocument};
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents the data flow state at a particular statute.
#[derive(Debug, Clone, Default)]
pub struct DataFlowState {
    /// Fields that are read by this statute's conditions
    pub reads: HashSet<String>,
    /// Fields that are written/modified by this statute's effects
    pub writes: HashSet<String>,
    /// Statutes that this one depends on
    pub dependencies: HashSet<String>,
    /// Statutes that depend on this one
    pub dependents: HashSet<String>,
}

/// Collects field names that are read by a condition.
fn collect_reads(condition: &ConditionNode, reads: &mut HashSet<String>) {
    match condition {
        ConditionNode::Comparison { field, .. }
        | ConditionNode::Between { field, .. }
        | ConditionNode::In { field, .. }
        | ConditionNode::Like { field, .. }
        | ConditionNode::Matches { field, .. }
        | ConditionNode::InRange { field, .. }
        | ConditionNode::NotInRange { field, .. } => {
            reads.insert(field.clone());
        }
        ConditionNode::HasAttribute { key } => {
            reads.insert(key.clone());
        }
        ConditionNode::TemporalComparison { field, .. } => {
            reads.insert(format!("{:?}", field));
        }
        ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
            collect_reads(left, reads);
            collect_reads(right, reads);
        }
        ConditionNode::Not(inner) => {
            collect_reads(inner, reads);
        }
    }
}

/// Data flow analyzer for legal documents.
pub struct DataFlowAnalyzer {
    /// Mapping from statute ID to its data flow state
    states: HashMap<String, DataFlowState>,
}

impl DataFlowAnalyzer {
    /// Creates a new data flow analyzer.
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Analyzes a legal document and computes data flow for all statutes.
    pub fn analyze(&mut self, doc: &LegalDocument) {
        // First pass: collect reads and writes for each statute
        for statute in &doc.statutes {
            let mut state = DataFlowState::default();

            // Collect fields read by conditions
            for condition in &statute.conditions {
                collect_reads(condition, &mut state.reads);
            }

            // Collect fields read by exception conditions
            for exception in &statute.exceptions {
                for condition in &exception.conditions {
                    collect_reads(condition, &mut state.reads);
                }
            }

            // Collect fields read by delegate conditions
            for delegate in &statute.delegates {
                for condition in &delegate.conditions {
                    collect_reads(condition, &mut state.reads);
                }
            }

            // Collect fields read by scope conditions
            if let Some(scope) = &statute.scope {
                for condition in &scope.conditions {
                    collect_reads(condition, &mut state.reads);
                }
            }

            // Collect fields read by constraint conditions
            for constraint in &statute.constraints {
                collect_reads(&constraint.condition, &mut state.reads);
            }

            // Collect fields written by effects (simplified - just effect types)
            for effect in &statute.effects {
                state.writes.insert(effect.effect_type.clone());
            }

            // Collect direct dependencies
            for req in &statute.requires {
                state.dependencies.insert(req.clone());
            }

            // Collect delegation dependencies
            for delegate in &statute.delegates {
                state.dependencies.insert(delegate.target_id.clone());
            }

            self.states.insert(statute.id.clone(), state);
        }

        // Second pass: compute transitive dependencies and dependents
        self.compute_transitive_dependencies(doc);
    }

    /// Computes transitive dependencies using a worklist algorithm.
    fn compute_transitive_dependencies(&mut self, doc: &LegalDocument) {
        // Build reverse dependency graph (dependents)
        let mut reverse_deps: HashMap<String, HashSet<String>> = HashMap::new();

        for statute in &doc.statutes {
            if let Some(state) = self.states.get(&statute.id) {
                for dep in &state.dependencies {
                    reverse_deps
                        .entry(dep.clone())
                        .or_default()
                        .insert(statute.id.clone());
                }
            }
        }

        // Update dependents in the states
        for (statute_id, dependents) in reverse_deps {
            if let Some(state) = self.states.get_mut(&statute_id) {
                state.dependents = dependents;
            }
        }
    }

    /// Gets the data flow state for a statute.
    pub fn get_state(&self, statute_id: &str) -> Option<&DataFlowState> {
        self.states.get(statute_id)
    }

    /// Finds all statutes that read a particular field.
    pub fn find_readers(&self, field: &str) -> Vec<String> {
        self.states
            .iter()
            .filter(|(_, state)| state.reads.contains(field))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Finds all statutes that write a particular field/effect.
    pub fn find_writers(&self, field: &str) -> Vec<String> {
        self.states
            .iter()
            .filter(|(_, state)| state.writes.contains(field))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Computes the transitive closure of dependencies for a statute.
    pub fn transitive_dependencies(&self, statute_id: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(statute_id.to_string());

        while let Some(id) = queue.pop_front() {
            if let Some(state) = self.states.get(&id) {
                for dep in &state.dependencies {
                    if result.insert(dep.clone()) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        result
    }

    /// Computes the transitive closure of dependents for a statute.
    pub fn transitive_dependents(&self, statute_id: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(statute_id.to_string());

        while let Some(id) = queue.pop_front() {
            if let Some(state) = self.states.get(&id) {
                for dependent in &state.dependents {
                    if result.insert(dependent.clone()) {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }

        result
    }

    /// Detects potential data flow issues.
    pub fn detect_issues(&self) -> Vec<DataFlowIssue> {
        let mut issues = Vec::new();

        // Check for read-after-write hazards
        for (id, state) in &self.states {
            // Check if any dependency writes a field that we read
            for dep in &state.dependencies {
                if let Some(dep_state) = self.states.get(dep) {
                    let common: HashSet<_> = state
                        .reads
                        .intersection(&dep_state.writes)
                        .cloned()
                        .collect();
                    if !common.is_empty() {
                        issues.push(DataFlowIssue::ReadAfterWrite {
                            statute: id.clone(),
                            dependency: dep.clone(),
                            fields: common,
                        });
                    }
                }
            }
        }

        issues
    }
}

impl Default for DataFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a data flow issue detected during analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum DataFlowIssue {
    /// A statute reads fields that a dependency writes
    ReadAfterWrite {
        statute: String,
        dependency: String,
        fields: HashSet<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{EffectNode, LegalDocument, StatuteNode};

    #[test]
    fn test_simple_dataflow() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "base".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Base".to_string(),
                    conditions: vec![ConditionNode::Comparison {
                        field: "age".to_string(),
                        operator: ">=".to_string(),
                        value: crate::ast::ConditionValue::Number(18),
                    }],
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Rights".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "derived".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Derived".to_string(),
                    conditions: vec![],
                    effects: vec![],
                    requires: vec!["base".to_string()],
                    ..Default::default()
                },
            ],
        };

        let mut analyzer = DataFlowAnalyzer::new();
        analyzer.analyze(&doc);

        // Check base statute
        let base_state = analyzer.get_state("base").unwrap();
        assert!(base_state.reads.contains("age"));
        assert!(base_state.writes.contains("grant"));
        assert!(base_state.dependencies.is_empty());
        assert_eq!(base_state.dependents.len(), 1);
        assert!(base_state.dependents.contains("derived"));

        // Check derived statute
        let derived_state = analyzer.get_state("derived").unwrap();
        assert_eq!(derived_state.dependencies.len(), 1);
        assert!(derived_state.dependencies.contains("base"));
    }

    #[test]
    fn test_transitive_dependencies() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "a".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "A".to_string(),
                    ..Default::default()
                },
                StatuteNode {
                    id: "b".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "B".to_string(),
                    requires: vec!["a".to_string()],
                    ..Default::default()
                },
                StatuteNode {
                    id: "c".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "C".to_string(),
                    requires: vec!["b".to_string()],
                    ..Default::default()
                },
            ],
        };

        let mut analyzer = DataFlowAnalyzer::new();
        analyzer.analyze(&doc);

        // C transitively depends on both B and A
        let deps = analyzer.transitive_dependencies("c");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains("a"));
        assert!(deps.contains("b"));

        // A is transitively depended on by both B and C
        let dependents = analyzer.transitive_dependents("a");
        assert_eq!(dependents.len(), 2);
        assert!(dependents.contains("b"));
        assert!(dependents.contains("c"));
    }

    #[test]
    fn test_find_readers_and_writers() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "reader".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Reader".to_string(),
                    conditions: vec![ConditionNode::Comparison {
                        field: "income".to_string(),
                        operator: ">".to_string(),
                        value: crate::ast::ConditionValue::Number(50000),
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "writer".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Writer".to_string(),
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Benefit".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
            ],
        };

        let mut analyzer = DataFlowAnalyzer::new();
        analyzer.analyze(&doc);

        let readers = analyzer.find_readers("income");
        assert_eq!(readers.len(), 1);
        assert!(readers.contains(&"reader".to_string()));

        let writers = analyzer.find_writers("grant");
        assert_eq!(writers.len(), 1);
        assert!(writers.contains(&"writer".to_string()));
    }
}
