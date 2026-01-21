//! Consistency checking across related statutes.
//!
//! This module provides analysis to detect logical inconsistencies, conflicts,
//! and contradictions between related statutes.

use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::collections::{HashMap, HashSet};

/// Types of consistency issues that can be detected
#[derive(Debug, Clone, PartialEq)]
pub enum ConsistencyIssue {
    /// Two statutes have contradictory conditions
    ContradictoryConditions {
        statute1: String,
        statute2: String,
        field: String,
        description: String,
    },
    /// A statute requires another that has been superseded
    RequiresSuperseded {
        statute: String,
        required: String,
        superseded_by: String,
    },
    /// Circular supersession chain detected
    CircularSupersession { chain: Vec<String> },
    /// Conflicting default values
    ConflictingDefaults {
        statute1: String,
        statute2: String,
        field: String,
        value1: String,
        value2: String,
    },
    /// Jurisdiction mismatch in dependencies
    JurisdictionMismatch {
        statute: String,
        jurisdiction: String,
        dependent: String,
        dependent_jurisdiction: String,
    },
    /// Effect type conflict (grant vs revoke)
    EffectConflict {
        statute1: String,
        statute2: String,
        effect_type1: String,
        effect_type2: String,
        description: String,
    },
    /// Temporal inconsistency (expiry before effective)
    TemporalInconsistency {
        statute: String,
        effective_date: String,
        expiry_date: String,
    },
}

/// Consistency checker for legal documents
pub struct ConsistencyChecker {
    issues: Vec<ConsistencyIssue>,
    statute_map: HashMap<String, StatuteNode>,
}

impl ConsistencyChecker {
    /// Creates a new consistency checker
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            statute_map: HashMap::new(),
        }
    }

    /// Analyzes a legal document for consistency issues
    pub fn check(&mut self, doc: &LegalDocument) -> &[ConsistencyIssue] {
        self.issues.clear();
        self.statute_map.clear();

        // Build statute map
        for statute in &doc.statutes {
            self.statute_map.insert(statute.id.clone(), statute.clone());
        }

        // Run all consistency checks
        self.check_supersession_consistency(doc);
        self.check_requirement_consistency(doc);
        self.check_condition_consistency(doc);
        self.check_default_consistency(doc);
        self.check_effect_consistency(doc);

        &self.issues
    }

    /// Checks for supersession-related issues
    fn check_supersession_consistency(&mut self, doc: &LegalDocument) {
        // Check for circular supersession
        for statute in &doc.statutes {
            let mut visited = HashSet::new();
            let mut chain = Vec::new();

            if self.has_circular_supersession(&statute.id, &mut visited, &mut chain) {
                self.issues.push(ConsistencyIssue::CircularSupersession {
                    chain: chain.clone(),
                });
            }
        }

        // Check if required statutes are superseded
        for statute in &doc.statutes {
            for required in &statute.requires {
                if self.statute_map.contains_key(required) {
                    // Check if any statute supersedes this required one
                    for other in &doc.statutes {
                        if other.supersedes.contains(required) {
                            self.issues.push(ConsistencyIssue::RequiresSuperseded {
                                statute: statute.id.clone(),
                                required: required.clone(),
                                superseded_by: other.id.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    /// Checks for circular supersession chains
    fn has_circular_supersession(
        &self,
        statute_id: &str,
        visited: &mut HashSet<String>,
        chain: &mut Vec<String>,
    ) -> bool {
        if chain.contains(&statute_id.to_string()) {
            return true;
        }

        if visited.contains(statute_id) {
            return false;
        }

        visited.insert(statute_id.to_string());
        chain.push(statute_id.to_string());

        if let Some(statute) = self.statute_map.get(statute_id) {
            for superseded in &statute.supersedes {
                if self.has_circular_supersession(superseded, visited, chain) {
                    return true;
                }
            }
        }

        chain.pop();
        false
    }

    /// Checks requirement consistency
    fn check_requirement_consistency(&mut self, _doc: &LegalDocument) {
        // Check for jurisdiction mismatches
        for (id, statute) in &self.statute_map {
            for required in &statute.requires {
                if let Some(required_statute) = self.statute_map.get(required) {
                    // Extract jurisdiction from conditions (simplified)
                    let statute_jurisdiction = self.extract_jurisdiction(statute);
                    let required_jurisdiction = self.extract_jurisdiction(required_statute);

                    if let (Some(j1), Some(j2)) = (statute_jurisdiction, required_jurisdiction)
                        && j1 != j2
                    {
                        self.issues.push(ConsistencyIssue::JurisdictionMismatch {
                            statute: id.clone(),
                            jurisdiction: j1,
                            dependent: required.clone(),
                            dependent_jurisdiction: j2,
                        });
                    }
                }
            }
        }
    }

    /// Extracts jurisdiction from statute conditions (simplified)
    fn extract_jurisdiction(&self, _statute: &StatuteNode) -> Option<String> {
        // This is a simplified version - in a real implementation,
        // we'd parse jurisdiction metadata or specific conditions
        None
    }

    /// Checks for contradictory conditions between related statutes
    fn check_condition_consistency(&mut self, _doc: &LegalDocument) {
        // Collect all statute pairs to avoid borrowing issues
        let statute_pairs: Vec<_> = self.statute_map.keys().cloned().collect();

        for i in 0..statute_pairs.len() {
            for j in (i + 1)..statute_pairs.len() {
                let id1 = &statute_pairs[i];
                let id2 = &statute_pairs[j];

                // Clone the statutes to avoid borrow checker issues
                if let (Some(s1), Some(s2)) = (
                    self.statute_map.get(id1).cloned(),
                    self.statute_map.get(id2).cloned(),
                ) {
                    // Check if they're related (one requires the other)
                    if s1.requires.contains(id2) || s2.requires.contains(id1) {
                        self.check_condition_contradiction(&s1, &s2);
                    }
                }
            }
        }
    }

    /// Checks two statutes for contradictory conditions
    fn check_condition_contradiction(&mut self, s1: &StatuteNode, s2: &StatuteNode) {
        // Extract field comparisons from both statutes
        let fields1 = extract_field_conditions(&s1.conditions);
        let fields2 = extract_field_conditions(&s2.conditions);

        // Check for contradictions
        for (field, (op1, val1)) in &fields1 {
            if let Some((op2, val2)) = fields2.get(field)
                && self.are_contradictory(op1, val1, op2, val2)
            {
                self.issues.push(ConsistencyIssue::ContradictoryConditions {
                    statute1: s1.id.clone(),
                    statute2: s2.id.clone(),
                    field: field.clone(),
                    description: format!(
                        "{} requires {} {} but {} requires {} {}",
                        s1.id,
                        field,
                        self.format_condition(op1, val1),
                        s2.id,
                        field,
                        self.format_condition(op2, val2)
                    ),
                });
            }
        }
    }

    /// Checks if two conditions are contradictory
    fn are_contradictory(
        &self,
        op1: &str,
        val1: &ConditionValue,
        op2: &str,
        val2: &ConditionValue,
    ) -> bool {
        // Simple contradiction detection
        match (op1, op2) {
            ("==", "!=") | ("!=", "==") => val1 == val2,
            ("<", ">") | (">", "<") => {
                // x < 10 and x > 20 is contradictory
                if let (ConditionValue::Number(n1), ConditionValue::Number(n2)) = (val1, val2) {
                    (op1 == "<" && n1 < n2) || (op1 == ">" && n1 > n2)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Formats a condition for display
    fn format_condition(&self, op: &str, val: &ConditionValue) -> String {
        match val {
            ConditionValue::Number(n) => format!("{} {}", op, n),
            ConditionValue::String(s) => format!("{} \"{}\"", op, s),
            ConditionValue::Boolean(b) => format!("{} {}", op, b),
            ConditionValue::Date(d) => format!("{} {}", op, d),
            ConditionValue::SetExpr(_) => format!("{} <set>", op),
        }
    }

    /// Checks for conflicting default values
    fn check_default_consistency(&mut self, _doc: &LegalDocument) {
        // Build a map of field -> (statute_id, value)
        let mut defaults: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for (id, statute) in &self.statute_map {
            for default in &statute.defaults {
                let value_str = format!("{:?}", default.value);
                defaults
                    .entry(default.field.clone())
                    .or_default()
                    .push((id.clone(), value_str));
            }
        }

        // Check for conflicts
        for (field, values) in defaults {
            if values.len() > 1 {
                // Check if all values are the same
                let first_value = &values[0].1;
                for (statute_id, value) in &values[1..] {
                    if value != first_value {
                        self.issues.push(ConsistencyIssue::ConflictingDefaults {
                            statute1: values[0].0.clone(),
                            statute2: statute_id.clone(),
                            field: field.clone(),
                            value1: first_value.clone(),
                            value2: value.clone(),
                        });
                    }
                }
            }
        }
    }

    /// Checks for effect consistency (grant vs revoke conflicts)
    fn check_effect_consistency(&mut self, _doc: &LegalDocument) {
        // Build a map of effect descriptions to (statute_id, effect_type)
        let mut effects: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for (id, statute) in &self.statute_map {
            for effect in &statute.effects {
                effects
                    .entry(effect.description.clone())
                    .or_default()
                    .push((id.clone(), effect.effect_type.clone()));
            }
        }

        // Check for grant/revoke conflicts
        for (description, statute_effects) in effects {
            for i in 0..statute_effects.len() {
                for j in (i + 1)..statute_effects.len() {
                    let (id1, type1) = &statute_effects[i];
                    let (id2, type2) = &statute_effects[j];

                    // Check for opposing effects
                    if (type1 == "grant" && type2 == "revoke")
                        || (type1 == "revoke" && type2 == "grant")
                    {
                        self.issues.push(ConsistencyIssue::EffectConflict {
                            statute1: id1.clone(),
                            statute2: id2.clone(),
                            effect_type1: type1.clone(),
                            effect_type2: type2.clone(),
                            description: description.clone(),
                        });
                    }
                }
            }
        }
    }

    /// Gets all detected issues
    pub fn issues(&self) -> &[ConsistencyIssue] {
        &self.issues
    }

    /// Returns true if any issues were found
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

impl Default for ConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracts field conditions from a list of conditions (standalone helper function)
fn extract_field_conditions(
    conditions: &[ConditionNode],
) -> HashMap<String, (String, ConditionValue)> {
    let mut result = HashMap::new();

    for condition in conditions {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                result.insert(field.clone(), (operator.clone(), value.clone()));
            }
            ConditionNode::And(left, right) => {
                let left_fields = extract_field_conditions(&[*left.clone()]);
                let right_fields = extract_field_conditions(&[*right.clone()]);
                result.extend(left_fields);
                result.extend(right_fields);
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{EffectNode, LegalDocument};

    #[test]
    fn test_no_issues_simple_doc() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "statute1".to_string(),
                visibility: crate::module_system::Visibility::Private,
                title: "Test Statute".to_string(),
                conditions: vec![],
                effects: vec![],
                discretion: None,
                exceptions: vec![],
                amendments: vec![],
                supersedes: vec![],
                defaults: vec![],
                requires: vec![],
                delegates: vec![],
                scope: None,
                constraints: vec![],
                priority: None,
            }],
        };

        let mut checker = ConsistencyChecker::new();
        let issues = checker.check(&doc);

        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_circular_supersession() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "a".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "A".to_string(),
                    supersedes: vec!["b".to_string()],
                    ..Default::default()
                },
                StatuteNode {
                    id: "b".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "B".to_string(),
                    supersedes: vec!["a".to_string()],
                    ..Default::default()
                },
            ],
        };

        let mut checker = ConsistencyChecker::new();
        let issues = checker.check(&doc);

        assert!(
            issues
                .iter()
                .any(|i| matches!(i, ConsistencyIssue::CircularSupersession { .. }))
        );
    }

    #[test]
    fn test_requires_superseded() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "new".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "New".to_string(),
                    requires: vec!["old".to_string()],
                    ..Default::default()
                },
                StatuteNode {
                    id: "old".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Old".to_string(),
                    ..Default::default()
                },
                StatuteNode {
                    id: "replacement".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Replacement".to_string(),
                    supersedes: vec!["old".to_string()],
                    ..Default::default()
                },
            ],
        };

        let mut checker = ConsistencyChecker::new();
        let issues = checker.check(&doc);

        assert!(
            issues
                .iter()
                .any(|i| matches!(i, ConsistencyIssue::RequiresSuperseded { .. }))
        );
    }

    #[test]
    fn test_effect_conflict() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute1".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 1".to_string(),
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "voting rights".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "statute2".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 2".to_string(),
                    effects: vec![EffectNode {
                        effect_type: "revoke".to_string(),
                        description: "voting rights".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
            ],
        };

        let mut checker = ConsistencyChecker::new();
        let issues = checker.check(&doc);

        assert!(
            issues
                .iter()
                .any(|i| matches!(i, ConsistencyIssue::EffectConflict { .. }))
        );
    }

    #[test]
    fn test_conflicting_defaults() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute1".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 1".to_string(),
                    defaults: vec![crate::ast::DefaultNode {
                        field: "status".to_string(),
                        value: ConditionValue::String("active".to_string()),
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "statute2".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 2".to_string(),
                    defaults: vec![crate::ast::DefaultNode {
                        field: "status".to_string(),
                        value: ConditionValue::String("pending".to_string()),
                    }],
                    ..Default::default()
                },
            ],
        };

        let mut checker = ConsistencyChecker::new();
        let issues = checker.check(&doc);

        assert!(
            issues
                .iter()
                .any(|i| matches!(i, ConsistencyIssue::ConflictingDefaults { .. }))
        );
    }
}
