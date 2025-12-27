//! AST transformation pipeline for composable statute transformations.
//!
//! This module provides a framework for applying transformations to legal
//! documents in a composable, type-safe manner.

use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use crate::{DslError, DslResult};
use std::collections::HashMap;

/// A transformation that can be applied to a legal document.
pub trait DocumentTransform: Send + Sync {
    /// Applies the transformation to a document.
    fn transform(&self, doc: &LegalDocument) -> DslResult<LegalDocument>;

    /// Returns a description of what this transformation does.
    fn description(&self) -> &str;

    /// Returns true if this transformation is reversible.
    fn is_reversible(&self) -> bool {
        false
    }

    /// Reverses the transformation if supported.
    /// Returns an error if the transformation is not reversible.
    fn reverse(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        let _ = doc;
        Err(DslError::parse_error(&format!(
            "Transformation '{}' is not reversible",
            self.description()
        )))
    }

    /// Validates that the transformation can be safely applied.
    fn validate(&self, doc: &LegalDocument) -> DslResult<()> {
        let _ = doc;
        Ok(())
    }
}

/// A transformation that can be applied to a statute.
pub trait StatuteTransform: Send + Sync {
    /// Applies the transformation to a statute.
    fn transform(&self, statute: &StatuteNode) -> DslResult<StatuteNode>;

    /// Returns a description of what this transformation does.
    fn description(&self) -> &str;
}

/// A transformation that can be applied to a condition.
pub trait ConditionTransform: Send + Sync {
    /// Applies the transformation to a condition.
    fn transform(&self, condition: &ConditionNode) -> DslResult<ConditionNode>;

    /// Returns a description of what this transformation does.
    fn description(&self) -> &str;
}

/// A composable transformation pipeline with undo support.
#[derive(Default)]
pub struct TransformPipeline {
    transforms: Vec<Box<dyn DocumentTransform>>,
}

impl TransformPipeline {
    /// Creates a new empty pipeline.
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }

    /// Adds a transformation to the pipeline.
    #[allow(clippy::should_implement_trait)]
    pub fn add<T: DocumentTransform + 'static>(mut self, transform: T) -> Self {
        self.transforms.push(Box::new(transform));
        self
    }

    /// Applies all transformations in sequence.
    pub fn apply(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        let mut result = doc.clone();
        for transform in &self.transforms {
            result = transform.transform(&result)?;
        }
        Ok(result)
    }

    /// Validates all transformations before applying.
    pub fn validate(&self, doc: &LegalDocument) -> DslResult<()> {
        for transform in &self.transforms {
            transform.validate(doc)?;
        }
        Ok(())
    }

    /// Applies all transformations with validation.
    pub fn apply_validated(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        self.validate(doc)?;
        self.apply(doc)
    }

    /// Returns descriptions of all transformations in the pipeline.
    pub fn describe(&self) -> Vec<String> {
        self.transforms
            .iter()
            .map(|t| t.description().to_string())
            .collect()
    }

    /// Checks if all transformations in the pipeline are reversible.
    pub fn is_reversible(&self) -> bool {
        self.transforms.iter().all(|t| t.is_reversible())
    }
}

/// Transformation history with undo/redo support.
pub struct TransformHistory {
    history: Vec<LegalDocument>,
    current_index: usize,
}

impl TransformHistory {
    /// Creates a new history with the initial document.
    pub fn new(initial: LegalDocument) -> Self {
        Self {
            history: vec![initial],
            current_index: 0,
        }
    }

    /// Applies a transformation and adds to history.
    pub fn apply(&mut self, transform: &dyn DocumentTransform) -> DslResult<&LegalDocument> {
        let current = &self.history[self.current_index];
        let transformed = transform.transform(current)?;

        // Truncate future history if we're not at the end
        self.history.truncate(self.current_index + 1);

        // Add new state
        self.history.push(transformed);
        self.current_index += 1;

        Ok(&self.history[self.current_index])
    }

    /// Undoes the last transformation if possible.
    pub fn undo(&mut self) -> Option<&LegalDocument> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    /// Redoes the last undone transformation if possible.
    pub fn redo(&mut self) -> Option<&LegalDocument> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    /// Returns the current document.
    pub fn current(&self) -> &LegalDocument {
        &self.history[self.current_index]
    }

    /// Returns whether undo is available.
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    /// Returns whether redo is available.
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    /// Returns the number of states in the history.
    pub fn len(&self) -> usize {
        self.history.len()
    }

    /// Returns whether the history is empty.
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
}

/// Removes duplicate statutes with the same ID (keeps first occurrence).
pub struct DeduplicateStatutes;

impl DocumentTransform for DeduplicateStatutes {
    fn transform(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        let mut seen_ids = HashMap::new();
        let mut deduplicated = Vec::new();

        for statute in &doc.statutes {
            if !seen_ids.contains_key(&statute.id) {
                seen_ids.insert(statute.id.clone(), true);
                deduplicated.push(statute.clone());
            }
        }

        Ok(LegalDocument {
            imports: doc.imports.clone(),
            statutes: deduplicated,
        })
    }

    fn description(&self) -> &str {
        "Remove duplicate statutes with the same ID"
    }
}

/// Simplifies nested NOT conditions (double negation elimination).
pub struct SimplifyConditions;

impl ConditionTransform for SimplifyConditions {
    fn transform(&self, condition: &ConditionNode) -> DslResult<ConditionNode> {
        match condition {
            // Double negation: NOT NOT x => x
            ConditionNode::Not(inner) => {
                if let ConditionNode::Not(inner_inner) = inner.as_ref() {
                    ConditionTransform::transform(self, inner_inner)
                } else {
                    Ok(ConditionNode::Not(Box::new(ConditionTransform::transform(
                        self, inner,
                    )?)))
                }
            }
            // Recursively simplify AND
            ConditionNode::And(left, right) => Ok(ConditionNode::And(
                Box::new(ConditionTransform::transform(self, left)?),
                Box::new(ConditionTransform::transform(self, right)?),
            )),
            // Recursively simplify OR
            ConditionNode::Or(left, right) => Ok(ConditionNode::Or(
                Box::new(ConditionTransform::transform(self, left)?),
                Box::new(ConditionTransform::transform(self, right)?),
            )),
            // Other conditions pass through unchanged
            _ => Ok(condition.clone()),
        }
    }

    fn description(&self) -> &str {
        "Simplify conditions (eliminate double negations)"
    }
}

impl DocumentTransform for SimplifyConditions {
    fn transform(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        let mut simplified_statutes = Vec::new();

        for statute in &doc.statutes {
            let mut simplified_conditions = Vec::new();
            for condition in &statute.conditions {
                simplified_conditions.push(ConditionTransform::transform(self, condition)?);
            }

            let mut simplified_statute = statute.clone();
            simplified_statute.conditions = simplified_conditions;
            simplified_statutes.push(simplified_statute);
        }

        Ok(LegalDocument {
            imports: doc.imports.clone(),
            statutes: simplified_statutes,
        })
    }

    fn description(&self) -> &str {
        ConditionTransform::description(self)
    }
}

/// Removes statutes that have no effects (dead code elimination).
pub struct RemoveEmptyStatutes;

impl DocumentTransform for RemoveEmptyStatutes {
    fn transform(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        let filtered: Vec<_> = doc
            .statutes
            .iter()
            .filter(|s| !s.effects.is_empty() || s.discretion.is_some())
            .cloned()
            .collect();

        Ok(LegalDocument {
            imports: doc.imports.clone(),
            statutes: filtered,
        })
    }

    fn description(&self) -> &str {
        "Remove statutes with no effects"
    }
}

/// Sorts statutes by their dependencies (topological sort).
pub struct SortByDependencies;

impl DocumentTransform for SortByDependencies {
    fn transform(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        // Build dependency graph
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        for statute in &doc.statutes {
            graph.insert(statute.id.clone(), statute.requires.clone());
            in_degree.insert(statute.id.clone(), 0);
        }

        // Calculate in-degrees
        for statute in &doc.statutes {
            for req in &statute.requires {
                *in_degree.entry(req.clone()).or_insert(0) += 1;
            }
        }

        // Topological sort using Kahn's algorithm
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut sorted_ids = Vec::new();

        while let Some(id) = queue.pop() {
            sorted_ids.push(id.clone());

            if let Some(dependencies) = graph.get(&id) {
                for dep in dependencies {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dep.clone());
                        }
                    }
                }
            }
        }

        // Check for cycles
        if sorted_ids.len() != doc.statutes.len() {
            return Err(DslError::parse_error(
                "Cannot sort statutes: circular dependencies detected",
            ));
        }

        // Reorder statutes according to sorted IDs
        let statute_map: HashMap<_, _> = doc
            .statutes
            .iter()
            .map(|s| (s.id.clone(), s.clone()))
            .collect();

        let sorted_statutes: Vec<_> = sorted_ids
            .iter()
            .filter_map(|id| statute_map.get(id).cloned())
            .collect();

        Ok(LegalDocument {
            imports: doc.imports.clone(),
            statutes: sorted_statutes,
        })
    }

    fn description(&self) -> &str {
        "Sort statutes by dependencies (topological order)"
    }
}

/// Normalize statute IDs (lowercase, replace spaces with hyphens).
pub struct NormalizeIds;

impl DocumentTransform for NormalizeIds {
    fn transform(&self, doc: &LegalDocument) -> DslResult<LegalDocument> {
        let mut normalized = Vec::new();
        let mut id_mapping: HashMap<String, String> = HashMap::new();

        // First pass: create ID mapping
        for statute in &doc.statutes {
            let normalized_id = statute.id.to_lowercase().replace([' ', '_'], "-");
            id_mapping.insert(statute.id.clone(), normalized_id);
        }

        // Second pass: apply mapping
        for statute in &doc.statutes {
            let mut normalized_statute = statute.clone();
            normalized_statute.id = id_mapping
                .get(&statute.id)
                .cloned()
                .unwrap_or_else(|| statute.id.clone());

            // Update requires references
            normalized_statute.requires = statute
                .requires
                .iter()
                .map(|req| id_mapping.get(req).cloned().unwrap_or_else(|| req.clone()))
                .collect();

            // Update supersedes references
            normalized_statute.supersedes = statute
                .supersedes
                .iter()
                .map(|sup| id_mapping.get(sup).cloned().unwrap_or_else(|| sup.clone()))
                .collect();

            normalized.push(normalized_statute);
        }

        Ok(LegalDocument {
            imports: doc.imports.clone(),
            statutes: normalized,
        })
    }

    fn description(&self) -> &str {
        "Normalize statute IDs to lowercase with hyphens"
    }
}

/// Preset transformation recipes for common patterns.
pub mod presets {
    use super::*;

    /// Creates a pipeline that cleans up a document.
    /// Deduplicates statutes, removes empty ones, and normalizes IDs.
    pub fn cleanup_pipeline() -> TransformPipeline {
        TransformPipeline::new()
            .add(DeduplicateStatutes)
            .add(RemoveEmptyStatutes)
            .add(NormalizeIds)
    }

    /// Creates a pipeline that optimizes a document.
    /// Simplifies conditions and removes dead code.
    pub fn optimization_pipeline() -> TransformPipeline {
        TransformPipeline::new()
            .add(SimplifyConditions)
            .add(RemoveEmptyStatutes)
    }

    /// Creates a pipeline that normalizes a document.
    /// Normalizes IDs and sorts by dependencies.
    pub fn normalization_pipeline() -> TransformPipeline {
        TransformPipeline::new()
            .add(NormalizeIds)
            .add(SortByDependencies)
    }

    /// Creates a full processing pipeline.
    /// Combines cleanup, optimization, and normalization.
    pub fn full_pipeline() -> TransformPipeline {
        TransformPipeline::new()
            .add(DeduplicateStatutes)
            .add(SimplifyConditions)
            .add(RemoveEmptyStatutes)
            .add(NormalizeIds)
            .add(SortByDependencies)
    }

    /// Creates a minimal pipeline for quick fixes.
    pub fn quick_fix_pipeline() -> TransformPipeline {
        TransformPipeline::new()
            .add(DeduplicateStatutes)
            .add(SimplifyConditions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::EffectNode;

    fn sample_statute(id: &str, requires: Vec<String>) -> StatuteNode {
        StatuteNode {
            id: id.to_string(),
            title: format!("Statute {}", id),
            conditions: vec![],
            effects: vec![EffectNode {
                effect_type: "grant".to_string(),
                description: "test".to_string(),
                parameters: vec![],
            }],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires,
        }
    }

    #[test]
    fn test_deduplicate_statutes() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("A", vec![]),
                sample_statute("B", vec![]),
                sample_statute("A", vec![]), // duplicate
            ],
        };

        let transform = DeduplicateStatutes;
        let result = transform.transform(&doc).unwrap();

        assert_eq!(result.statutes.len(), 2);
        assert_eq!(result.statutes[0].id, "A");
        assert_eq!(result.statutes[1].id, "B");
    }

    #[test]
    fn test_simplify_double_negation() {
        let condition = ConditionNode::Not(Box::new(ConditionNode::Not(Box::new(
            ConditionNode::HasAttribute {
                key: "test".to_string(),
            },
        ))));

        let transform = SimplifyConditions;
        let result = ConditionTransform::transform(&transform, &condition).unwrap();

        assert!(matches!(result, ConditionNode::HasAttribute { .. }));
    }

    #[test]
    fn test_remove_empty_statutes() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("A", vec![]),
                StatuteNode {
                    id: "empty".to_string(),
                    title: "Empty".to_string(),
                    conditions: vec![],
                    effects: vec![], // no effects
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                },
            ],
        };

        let transform = RemoveEmptyStatutes;
        let result = transform.transform(&doc).unwrap();

        assert_eq!(result.statutes.len(), 1);
        assert_eq!(result.statutes[0].id, "A");
    }

    #[test]
    fn test_normalize_ids() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("My_Statute", vec!["Other_Statute".to_string()]),
                sample_statute("Other_Statute", vec![]),
            ],
        };

        let transform = NormalizeIds;
        let result = transform.transform(&doc).unwrap();

        assert_eq!(result.statutes[0].id, "my-statute");
        assert_eq!(result.statutes[1].id, "other-statute");
        assert_eq!(result.statutes[0].requires[0], "other-statute");
    }

    #[test]
    fn test_transformation_pipeline() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("A", vec![]),
                sample_statute("A", vec![]), // duplicate
                StatuteNode {
                    id: "empty".to_string(),
                    title: "Empty".to_string(),
                    conditions: vec![],
                    effects: vec![],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                },
            ],
        };

        let pipeline = TransformPipeline::new()
            .add(DeduplicateStatutes)
            .add(RemoveEmptyStatutes);

        let result = pipeline.apply(&doc).unwrap();

        assert_eq!(result.statutes.len(), 1);
        assert_eq!(result.statutes[0].id, "A");

        let descriptions = pipeline.describe();
        assert_eq!(descriptions.len(), 2);
    }

    #[test]
    fn test_transform_history() {
        let initial = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("A", vec![]),
                sample_statute("A", vec![]), // duplicate
            ],
        };

        let mut history = TransformHistory::new(initial);

        assert_eq!(history.len(), 1);
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        // Apply transformation
        let dedup = DeduplicateStatutes;
        history.apply(&dedup).unwrap();

        assert_eq!(history.len(), 2);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.current().statutes.len(), 1);

        // Undo
        let undone = history.undo();
        assert!(undone.is_some());
        assert_eq!(history.current().statutes.len(), 2);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Redo
        let redone = history.redo();
        assert!(redone.is_some());
        assert_eq!(history.current().statutes.len(), 1);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_transform_validation() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![sample_statute("A", vec![])],
        };

        let pipeline = TransformPipeline::new()
            .add(DeduplicateStatutes)
            .add(SimplifyConditions);

        // Validation should pass
        assert!(pipeline.validate(&doc).is_ok());

        // Apply with validation
        let result = pipeline.apply_validated(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preset_cleanup_pipeline() {
        use super::presets::*;

        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("A", vec![]),
                sample_statute("A", vec![]), // duplicate
                StatuteNode {
                    id: "EMPTY_ONE".to_string(),
                    title: "Empty".to_string(),
                    conditions: vec![],
                    effects: vec![],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                },
            ],
        };

        let pipeline = cleanup_pipeline();
        let result = pipeline.apply(&doc).unwrap();

        assert_eq!(result.statutes.len(), 1);
        assert_eq!(result.statutes[0].id, "a"); // normalized
    }

    #[test]
    fn test_preset_full_pipeline() {
        use super::presets::*;

        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                sample_statute("My_Statute", vec![]),
                sample_statute("My_Statute", vec![]), // duplicate
            ],
        };

        let pipeline = full_pipeline();
        let result = pipeline.apply(&doc).unwrap();

        assert_eq!(result.statutes.len(), 1);
        assert_eq!(result.statutes[0].id, "my-statute");
    }

    #[test]
    fn test_reversible_check() {
        let pipeline = TransformPipeline::new()
            .add(DeduplicateStatutes)
            .add(SimplifyConditions);

        // Current transforms are not reversible
        assert!(!pipeline.is_reversible());
    }
}
