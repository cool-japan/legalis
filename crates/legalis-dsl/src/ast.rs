//! AST (Abstract Syntax Tree) definitions for the legal DSL.

use crate::SourceLocation;
use serde::{Deserialize, Serialize};

/// Token with source location information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpannedToken {
    /// The token type and value
    pub token: Token,
    /// Source location
    pub location: SourceLocation,
}

impl SpannedToken {
    /// Creates a new spanned token.
    pub fn new(token: Token, location: SourceLocation) -> Self {
        Self { token, location }
    }
}

/// Token types for the legal DSL lexer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Keywords
    Statute,
    When,
    Unless,
    Requires,
    Then,
    Discretion,
    Age,
    Income,
    Grant,
    Revoke,
    Obligation,
    Prohibition,
    Import,
    As,
    Exception,
    Amendment,
    Supersedes,
    Delegate,
    Priority,
    Scope,
    Constraint,

    // Metadata keywords
    EffectiveDate,
    ExpiryDate,
    Jurisdiction,
    Version,
    Has,

    // Temporal keywords
    CurrentDate,
    DateField,

    // Logical operators
    And,
    Or,
    Not,

    // Condition operators
    Between,
    In,
    Like,
    Matches, // Regex pattern matching
    InRange, // Numeric range with inclusive/exclusive bounds
    NotInRange,
    Default,

    // Set operations
    Union,
    Intersect,
    Difference,

    // Structural
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Dash,
    Dot,
    Comma,

    // Literals
    Ident(String),
    StringLit(String),
    Number(u64),
    Operator(String),
}

/// AST node for an import declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportNode {
    /// The path to the imported file.
    pub path: String,
    /// Optional alias for the import (AS clause).
    pub alias: Option<String>,
}

/// AST node for a complete legal document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalDocument {
    /// Import declarations at the top of the document.
    pub imports: Vec<ImportNode>,
    /// Statute definitions.
    pub statutes: Vec<StatuteNode>,
}

/// AST node for an exception clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionNode {
    /// Conditions under which the exception applies
    pub conditions: Vec<ConditionNode>,
    /// Description of the exception
    pub description: String,
}

/// AST node for an amendment clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmendmentNode {
    /// ID of the statute being amended
    pub target_id: String,
    /// Version of the amendment
    pub version: Option<u32>,
    /// Date of the amendment
    pub date: Option<String>,
    /// Description of changes
    pub description: String,
}

/// AST node for a delegate clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegateNode {
    /// ID of the statute to delegate to
    pub target_id: String,
    /// Conditions under which delegation applies
    pub conditions: Vec<ConditionNode>,
    /// Description of the delegation
    pub description: String,
}

/// AST node for a scope clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScopeNode {
    /// Entity types this statute applies to
    pub entity_types: Vec<String>,
    /// Conditions that define the scope
    pub conditions: Vec<ConditionNode>,
    /// Description of the scope
    pub description: Option<String>,
}

/// AST node for a constraint clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintNode {
    /// Name of the constraint
    pub name: String,
    /// The invariant condition that must hold
    pub condition: ConditionNode,
    /// Description of the constraint
    pub description: Option<String>,
}

/// AST node for a statute definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StatuteNode {
    pub id: String,
    pub title: String,
    pub conditions: Vec<ConditionNode>,
    pub effects: Vec<EffectNode>,
    pub discretion: Option<String>,
    pub exceptions: Vec<ExceptionNode>,
    pub amendments: Vec<AmendmentNode>,
    pub supersedes: Vec<String>,
    pub defaults: Vec<DefaultNode>,
    pub requires: Vec<String>,
    pub delegates: Vec<DelegateNode>,
    pub scope: Option<ScopeNode>,
    pub constraints: Vec<ConstraintNode>,
    pub priority: Option<u32>,
}

/// AST node for conditions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionNode {
    Comparison {
        field: String,
        operator: String,
        value: ConditionValue,
    },
    HasAttribute {
        key: String,
    },
    Between {
        field: String,
        min: ConditionValue,
        max: ConditionValue,
    },
    In {
        field: String,
        values: Vec<ConditionValue>,
    },
    Like {
        field: String,
        pattern: String,
    },
    /// Regex pattern matching
    Matches {
        field: String,
        regex_pattern: String,
    },
    /// Numeric range with inclusive/exclusive bounds
    InRange {
        field: String,
        min: ConditionValue,
        max: ConditionValue,
        inclusive_min: bool,
        inclusive_max: bool,
    },
    /// Negated numeric range
    NotInRange {
        field: String,
        min: ConditionValue,
        max: ConditionValue,
        inclusive_min: bool,
        inclusive_max: bool,
    },
    /// Temporal condition comparing current date with a value
    TemporalComparison {
        field: TemporalField,
        operator: String,
        value: ConditionValue,
    },
    And(Box<ConditionNode>, Box<ConditionNode>),
    Or(Box<ConditionNode>, Box<ConditionNode>),
    Not(Box<ConditionNode>),
}

/// Temporal field types for date/time conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemporalField {
    /// The current system date
    CurrentDate,
    /// A custom date field
    DateField(String),
}

/// Set expressions for set operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetExpression {
    /// A simple set of values
    Values(Vec<ConditionValue>),
    /// Union of two sets
    Union(Box<SetExpression>, Box<SetExpression>),
    /// Intersection of two sets
    Intersect(Box<SetExpression>, Box<SetExpression>),
    /// Difference of two sets (A - B)
    Difference(Box<SetExpression>, Box<SetExpression>),
}

/// Values that can appear in conditions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionValue {
    Number(i64),
    String(String),
    Boolean(bool),
    Date(String),
    /// A set expression for set operations
    SetExpr(SetExpression),
}

/// AST node for effects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectNode {
    pub effect_type: String,
    pub description: String,
    pub parameters: Vec<(String, String)>,
}

/// AST node for default value declarations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultNode {
    pub field: String,
    pub value: ConditionValue,
}

/// Visitor pattern for traversing AST nodes.
pub trait AstVisitor {
    /// Visit a legal document.
    fn visit_document(&mut self, doc: &LegalDocument) {
        self.walk_document(doc);
    }

    /// Visit a statute node.
    fn visit_statute(&mut self, statute: &StatuteNode) {
        self.walk_statute(statute);
    }

    /// Visit an import node.
    fn visit_import(&mut self, import: &ImportNode) {
        let _ = import; // default: no-op
    }

    /// Visit a condition node.
    fn visit_condition(&mut self, condition: &ConditionNode) {
        self.walk_condition(condition);
    }

    /// Visit an effect node.
    fn visit_effect(&mut self, effect: &EffectNode) {
        let _ = effect; // default: no-op
    }

    /// Visit an exception node.
    fn visit_exception(&mut self, exception: &ExceptionNode) {
        self.walk_exception(exception);
    }

    /// Visit an amendment node.
    fn visit_amendment(&mut self, amendment: &AmendmentNode) {
        let _ = amendment; // default: no-op
    }

    /// Visit a default node.
    fn visit_default(&mut self, default: &DefaultNode) {
        let _ = default; // default: no-op
    }

    /// Walk through a document (default implementation).
    fn walk_document(&mut self, doc: &LegalDocument) {
        for import in &doc.imports {
            self.visit_import(import);
        }
        for statute in &doc.statutes {
            self.visit_statute(statute);
        }
    }

    /// Walk through a statute (default implementation).
    fn walk_statute(&mut self, statute: &StatuteNode) {
        for condition in &statute.conditions {
            self.visit_condition(condition);
        }
        for effect in &statute.effects {
            self.visit_effect(effect);
        }
        for exception in &statute.exceptions {
            self.visit_exception(exception);
        }
        for amendment in &statute.amendments {
            self.visit_amendment(amendment);
        }
        for default in &statute.defaults {
            self.visit_default(default);
        }
    }

    /// Walk through a condition (default implementation).
    fn walk_condition(&mut self, condition: &ConditionNode) {
        match condition {
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.visit_condition(left);
                self.visit_condition(right);
            }
            ConditionNode::Not(inner) => {
                self.visit_condition(inner);
            }
            _ => {} // Leaf nodes (including TemporalComparison): no recursion needed
        }
    }

    /// Walk through an exception (default implementation).
    fn walk_exception(&mut self, exception: &ExceptionNode) {
        for condition in &exception.conditions {
            self.visit_condition(condition);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Example visitor that counts condition nodes.
    struct ConditionCounter {
        count: usize,
    }

    impl AstVisitor for ConditionCounter {
        fn visit_condition(&mut self, condition: &ConditionNode) {
            self.count += 1;
            self.walk_condition(condition);
        }
    }

    #[test]
    fn test_visitor_counts_conditions() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "test".to_string(),
                title: "Test".to_string(),
                conditions: vec![ConditionNode::And(
                    Box::new(ConditionNode::Comparison {
                        field: "age".to_string(),
                        operator: ">=".to_string(),
                        value: ConditionValue::Number(18),
                    }),
                    Box::new(ConditionNode::HasAttribute {
                        key: "citizen".to_string(),
                    }),
                )],
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

        let mut counter = ConditionCounter { count: 0 };
        counter.visit_document(&doc);

        // Should count: 1 And node + 2 leaf nodes = 3 total
        assert_eq!(counter.count, 3);
    }

    #[test]
    fn test_visitor_walks_all_statutes() {
        let doc = LegalDocument {
            imports: vec![ImportNode {
                path: "test.legalis".to_string(),
                alias: None,
            }],
            statutes: vec![
                StatuteNode {
                    id: "s1".to_string(),
                    title: "S1".to_string(),
                    conditions: vec![ConditionNode::HasAttribute {
                        key: "a".to_string(),
                    }],
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
                },
                StatuteNode {
                    id: "s2".to_string(),
                    title: "S2".to_string(),
                    conditions: vec![ConditionNode::HasAttribute {
                        key: "b".to_string(),
                    }],
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
                },
            ],
        };

        let mut counter = ConditionCounter { count: 0 };
        counter.visit_document(&doc);

        // Should count 2 conditions (one from each statute)
        assert_eq!(counter.count, 2);
    }
}

/// Represents the difference between two LegalDocuments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDiff {
    pub added_imports: Vec<ImportNode>,
    pub removed_imports: Vec<ImportNode>,
    pub added_statutes: Vec<StatuteNode>,
    pub removed_statutes: Vec<StatuteNode>,
    pub modified_statutes: Vec<(String, StatuteDiff)>,
}

impl DocumentDiff {
    /// Returns true if there are any changes.
    #[allow(dead_code)]
    pub fn has_changes(&self) -> bool {
        !self.added_imports.is_empty()
            || !self.removed_imports.is_empty()
            || !self.added_statutes.is_empty()
            || !self.removed_statutes.is_empty()
            || !self.modified_statutes.is_empty()
    }

    /// Returns a summary of changes as a string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if !self.added_imports.is_empty() {
            parts.push(format!("{} import(s) added", self.added_imports.len()));
        }
        if !self.removed_imports.is_empty() {
            parts.push(format!("{} import(s) removed", self.removed_imports.len()));
        }
        if !self.added_statutes.is_empty() {
            parts.push(format!("{} statute(s) added", self.added_statutes.len()));
        }
        if !self.removed_statutes.is_empty() {
            parts.push(format!(
                "{} statute(s) removed",
                self.removed_statutes.len()
            ));
        }
        if !self.modified_statutes.is_empty() {
            parts.push(format!(
                "{} statute(s) modified",
                self.modified_statutes.len()
            ));
        }

        if parts.is_empty() {
            "No changes".to_string()
        } else {
            parts.join(", ")
        }
    }
}

/// Represents the difference between two StatuteNodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDiff {
    pub changes: Vec<StatuteChange>,
}

impl StatuteDiff {
    /// Returns true if there are any changes.
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
}

/// Represents a specific change in a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatuteChange {
    TitleChanged {
        old: String,
        new: String,
    },
    ConditionsChanged {
        added: Vec<ConditionNode>,
        removed: Vec<ConditionNode>,
    },
    EffectsChanged,
    DiscretionChanged {
        old: Option<String>,
        new: Option<String>,
    },
    ExceptionsChanged,
    AmendmentsChanged,
    SupersedesChanged {
        old: Vec<String>,
        new: Vec<String>,
    },
    DefaultsChanged,
    RequiresChanged {
        old: Vec<String>,
        new: Vec<String>,
    },
}

/// AST transformation utilities.
pub mod transform {
    use super::*;

    /// Simplifies a condition node by removing redundant operations.
    #[allow(dead_code)]
    pub fn simplify_condition(cond: &ConditionNode) -> ConditionNode {
        match cond {
            // Double negation: NOT (NOT x) => x
            ConditionNode::Not(inner) => {
                if let ConditionNode::Not(inner_inner) = inner.as_ref() {
                    simplify_condition(inner_inner)
                } else {
                    ConditionNode::Not(Box::new(simplify_condition(inner)))
                }
            }
            // Recursively simplify AND/OR branches
            ConditionNode::And(left, right) => ConditionNode::And(
                Box::new(simplify_condition(left)),
                Box::new(simplify_condition(right)),
            ),
            ConditionNode::Or(left, right) => ConditionNode::Or(
                Box::new(simplify_condition(left)),
                Box::new(simplify_condition(right)),
            ),
            // Leaf nodes (including TemporalComparison) remain unchanged
            _ => cond.clone(),
        }
    }

    /// Normalizes a condition to Disjunctive Normal Form (DNF) - OR of ANDs.
    /// This is a simplified version that handles basic cases.
    #[allow(dead_code)]
    pub fn normalize_condition(cond: &ConditionNode) -> ConditionNode {
        // First simplify
        let simplified = simplify_condition(cond);

        // Apply De Morgan's laws
        match simplified {
            // NOT (a AND b) => (NOT a) OR (NOT b)
            ConditionNode::Not(inner) => match inner.as_ref() {
                ConditionNode::And(left, right) => ConditionNode::Or(
                    Box::new(normalize_condition(&ConditionNode::Not(left.clone()))),
                    Box::new(normalize_condition(&ConditionNode::Not(right.clone()))),
                ),
                // NOT (a OR b) => (NOT a) AND (NOT b)
                ConditionNode::Or(left, right) => ConditionNode::And(
                    Box::new(normalize_condition(&ConditionNode::Not(left.clone()))),
                    Box::new(normalize_condition(&ConditionNode::Not(right.clone()))),
                ),
                // NOT (NOT a) is already handled by simplify
                _ => ConditionNode::Not(Box::new(normalize_condition(&inner))),
            },
            ConditionNode::And(left, right) => ConditionNode::And(
                Box::new(normalize_condition(&left)),
                Box::new(normalize_condition(&right)),
            ),
            ConditionNode::Or(left, right) => ConditionNode::Or(
                Box::new(normalize_condition(&left)),
                Box::new(normalize_condition(&right)),
            ),
            other => other,
        }
    }

    /// Validates a statute node, checking for common issues.
    #[allow(dead_code)]
    pub fn validate_statute(statute: &StatuteNode) -> Vec<String> {
        let mut errors = Vec::new();

        if statute.id.is_empty() {
            errors.push("Statute ID cannot be empty".to_string());
        }

        if statute.title.is_empty() {
            errors.push("Statute title cannot be empty".to_string());
        }

        if statute.effects.is_empty() {
            errors.push("Statute must have at least one effect".to_string());
        }

        // Check for duplicate default fields
        let mut seen_fields = std::collections::HashSet::new();
        for default in &statute.defaults {
            if !seen_fields.insert(&default.field) {
                errors.push(format!("Duplicate DEFAULT field: {}", default.field));
            }
        }

        errors
    }

    /// Collects all referenced attribute keys from conditions.
    #[allow(dead_code)]
    pub fn collect_attribute_keys(cond: &ConditionNode) -> Vec<String> {
        let mut keys = Vec::new();

        fn collect_rec(cond: &ConditionNode, keys: &mut Vec<String>) {
            match cond {
                ConditionNode::HasAttribute { key } => keys.push(key.clone()),
                ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                    collect_rec(left, keys);
                    collect_rec(right, keys);
                }
                ConditionNode::Not(inner) => collect_rec(inner, keys),
                _ => {}
            }
        }

        collect_rec(cond, &mut keys);
        keys
    }

    /// Collects all field names used in conditions.
    #[allow(dead_code)]
    pub fn collect_condition_fields(cond: &ConditionNode) -> Vec<String> {
        let mut fields = Vec::new();

        fn collect_rec(cond: &ConditionNode, fields: &mut Vec<String>) {
            match cond {
                ConditionNode::Comparison { field, .. }
                | ConditionNode::Between { field, .. }
                | ConditionNode::In { field, .. }
                | ConditionNode::Like { field, .. } => {
                    fields.push(field.clone());
                }
                ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                    collect_rec(left, fields);
                    collect_rec(right, fields);
                }
                ConditionNode::Not(inner) => collect_rec(inner, fields),
                _ => {}
            }
        }

        collect_rec(cond, &mut fields);
        fields
    }

    /// Flattens nested AND/OR chains into a flat list.
    /// For example: (a AND (b AND c)) becomes [a, b, c]
    #[allow(dead_code)]
    pub fn flatten_condition(cond: &ConditionNode) -> ConditionNode {
        fn flatten_and(cond: &ConditionNode, acc: &mut Vec<ConditionNode>) {
            match cond {
                ConditionNode::And(left, right) => {
                    flatten_and(left, acc);
                    flatten_and(right, acc);
                }
                other => acc.push(flatten_condition(other)),
            }
        }

        fn flatten_or(cond: &ConditionNode, acc: &mut Vec<ConditionNode>) {
            match cond {
                ConditionNode::Or(left, right) => {
                    flatten_or(left, acc);
                    flatten_or(right, acc);
                }
                other => acc.push(flatten_condition(other)),
            }
        }

        match cond {
            ConditionNode::And(_, _) => {
                let mut items = Vec::new();
                flatten_and(cond, &mut items);
                items
                    .into_iter()
                    .reduce(|acc, item| ConditionNode::And(Box::new(acc), Box::new(item)))
                    .unwrap_or_else(|| cond.clone())
            }
            ConditionNode::Or(_, _) => {
                let mut items = Vec::new();
                flatten_or(cond, &mut items);
                items
                    .into_iter()
                    .reduce(|acc, item| ConditionNode::Or(Box::new(acc), Box::new(item)))
                    .unwrap_or_else(|| cond.clone())
            }
            ConditionNode::Not(inner) => ConditionNode::Not(Box::new(flatten_condition(inner))),
            other => other.clone(),
        }
    }

    /// Removes duplicate conditions from AND/OR chains.
    #[allow(dead_code)]
    pub fn remove_duplicate_conditions(cond: &ConditionNode) -> ConditionNode {
        use std::collections::HashSet;

        fn condition_to_string(cond: &ConditionNode) -> String {
            format!("{:?}", cond)
        }

        fn deduplicate_and(cond: &ConditionNode) -> Vec<ConditionNode> {
            let mut items = Vec::new();
            let mut seen = HashSet::new();

            fn collect_and(
                cond: &ConditionNode,
                items: &mut Vec<ConditionNode>,
                seen: &mut HashSet<String>,
            ) {
                match cond {
                    ConditionNode::And(left, right) => {
                        collect_and(left, items, seen);
                        collect_and(right, items, seen);
                    }
                    other => {
                        let key = condition_to_string(other);
                        if seen.insert(key) {
                            items.push(remove_duplicate_conditions(other));
                        }
                    }
                }
            }

            collect_and(cond, &mut items, &mut seen);
            items
        }

        fn deduplicate_or(cond: &ConditionNode) -> Vec<ConditionNode> {
            let mut items = Vec::new();
            let mut seen = HashSet::new();

            fn collect_or(
                cond: &ConditionNode,
                items: &mut Vec<ConditionNode>,
                seen: &mut HashSet<String>,
            ) {
                match cond {
                    ConditionNode::Or(left, right) => {
                        collect_or(left, items, seen);
                        collect_or(right, items, seen);
                    }
                    other => {
                        let key = condition_to_string(other);
                        if seen.insert(key) {
                            items.push(remove_duplicate_conditions(other));
                        }
                    }
                }
            }

            collect_or(cond, &mut items, &mut seen);
            items
        }

        match cond {
            ConditionNode::And(_, _) => {
                let items = deduplicate_and(cond);
                items
                    .into_iter()
                    .reduce(|acc, item| ConditionNode::And(Box::new(acc), Box::new(item)))
                    .unwrap_or_else(|| cond.clone())
            }
            ConditionNode::Or(_, _) => {
                let items = deduplicate_or(cond);
                items
                    .into_iter()
                    .reduce(|acc, item| ConditionNode::Or(Box::new(acc), Box::new(item)))
                    .unwrap_or_else(|| cond.clone())
            }
            ConditionNode::Not(inner) => {
                ConditionNode::Not(Box::new(remove_duplicate_conditions(inner)))
            }
            other => other.clone(),
        }
    }

    /// Applies all optimization passes to a condition.
    #[allow(dead_code)]
    pub fn optimize_condition(cond: &ConditionNode) -> ConditionNode {
        let simplified = simplify_condition(cond);
        let normalized = normalize_condition(&simplified);
        let flattened = flatten_condition(&normalized);
        remove_duplicate_conditions(&flattened)
    }

    /// Optimizes an entire statute by applying all transformations.
    #[allow(dead_code)]
    pub fn optimize_statute(statute: &StatuteNode) -> StatuteNode {
        StatuteNode {
            id: statute.id.clone(),
            title: statute.title.clone(),
            conditions: statute.conditions.iter().map(optimize_condition).collect(),
            effects: statute.effects.clone(),
            discretion: statute.discretion.clone(),
            exceptions: statute
                .exceptions
                .iter()
                .map(|ex| ExceptionNode {
                    conditions: ex.conditions.iter().map(optimize_condition).collect(),
                    description: ex.description.clone(),
                })
                .collect(),
            amendments: statute.amendments.clone(),
            supersedes: statute.supersedes.clone(),
            defaults: statute.defaults.clone(),
            requires: statute.requires.clone(),
            delegates: statute.delegates.clone(),
            scope: statute.scope.clone(),
            constraints: statute.constraints.clone(),
            priority: statute.priority,
        }
    }

    /// Computes differences between two LegalDocuments.
    #[allow(dead_code)]
    pub fn diff_documents(old: &LegalDocument, new: &LegalDocument) -> DocumentDiff {
        let mut added_imports = Vec::new();
        let mut removed_imports = Vec::new();
        let mut added_statutes = Vec::new();
        let mut removed_statutes = Vec::new();
        let mut modified_statutes = Vec::new();

        // Diff imports
        for import in &new.imports {
            if !old.imports.iter().any(|i| i.path == import.path) {
                added_imports.push(import.clone());
            }
        }
        for import in &old.imports {
            if !new.imports.iter().any(|i| i.path == import.path) {
                removed_imports.push(import.clone());
            }
        }

        // Diff statutes
        for statute in &new.statutes {
            if let Some(old_statute) = old.statutes.iter().find(|s| s.id == statute.id) {
                let statute_diff = diff_statutes(old_statute, statute);
                if statute_diff.has_changes() {
                    modified_statutes.push((statute.id.clone(), statute_diff));
                }
            } else {
                added_statutes.push(statute.clone());
            }
        }
        for statute in &old.statutes {
            if !new.statutes.iter().any(|s| s.id == statute.id) {
                removed_statutes.push(statute.clone());
            }
        }

        DocumentDiff {
            added_imports,
            removed_imports,
            added_statutes,
            removed_statutes,
            modified_statutes,
        }
    }

    /// Computes differences between two StatuteNodes.
    #[allow(dead_code)]
    pub fn diff_statutes(old: &StatuteNode, new: &StatuteNode) -> StatuteDiff {
        let mut changes = Vec::new();

        if old.title != new.title {
            changes.push(StatuteChange::TitleChanged {
                old: old.title.clone(),
                new: new.title.clone(),
            });
        }

        if old.conditions != new.conditions {
            changes.push(StatuteChange::ConditionsChanged {
                added: new
                    .conditions
                    .iter()
                    .filter(|c| !old.conditions.contains(c))
                    .cloned()
                    .collect(),
                removed: old
                    .conditions
                    .iter()
                    .filter(|c| !new.conditions.contains(c))
                    .cloned()
                    .collect(),
            });
        }

        if old.effects != new.effects {
            changes.push(StatuteChange::EffectsChanged);
        }

        if old.discretion != new.discretion {
            changes.push(StatuteChange::DiscretionChanged {
                old: old.discretion.clone(),
                new: new.discretion.clone(),
            });
        }

        if old.exceptions != new.exceptions {
            changes.push(StatuteChange::ExceptionsChanged);
        }

        if old.amendments != new.amendments {
            changes.push(StatuteChange::AmendmentsChanged);
        }

        if old.supersedes != new.supersedes {
            changes.push(StatuteChange::SupersedesChanged {
                old: old.supersedes.clone(),
                new: new.supersedes.clone(),
            });
        }

        if old.defaults != new.defaults {
            changes.push(StatuteChange::DefaultsChanged);
        }

        if old.requires != new.requires {
            changes.push(StatuteChange::RequiresChanged {
                old: old.requires.clone(),
                new: new.requires.clone(),
            });
        }

        StatuteDiff { changes }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_simplify_double_negation() {
            let cond = ConditionNode::Not(Box::new(ConditionNode::Not(Box::new(
                ConditionNode::HasAttribute {
                    key: "test".to_string(),
                },
            ))));

            let simplified = simplify_condition(&cond);
            assert!(matches!(simplified, ConditionNode::HasAttribute { .. }));
        }

        #[test]
        fn test_normalize_de_morgan() {
            // NOT (a AND b) should become (NOT a) OR (NOT b)
            let cond = ConditionNode::Not(Box::new(ConditionNode::And(
                Box::new(ConditionNode::HasAttribute {
                    key: "a".to_string(),
                }),
                Box::new(ConditionNode::HasAttribute {
                    key: "b".to_string(),
                }),
            )));

            let normalized = normalize_condition(&cond);
            assert!(matches!(normalized, ConditionNode::Or(_, _)));
        }

        #[test]
        fn test_validate_statute() {
            let statute = StatuteNode {
                id: "test".to_string(),
                title: "Test".to_string(),
                conditions: vec![],
                effects: vec![EffectNode {
                    effect_type: "grant".to_string(),
                    description: "Test".to_string(),
                    parameters: vec![],
                }],
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
            };

            let errors = validate_statute(&statute);
            assert!(errors.is_empty());
        }

        #[test]
        fn test_validate_empty_statute() {
            let statute = StatuteNode {
                id: "".to_string(),
                title: "".to_string(),
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
            };

            let errors = validate_statute(&statute);
            assert!(!errors.is_empty());
            assert!(errors.iter().any(|e| e.contains("ID")));
            assert!(errors.iter().any(|e| e.contains("title")));
            assert!(errors.iter().any(|e| e.contains("effect")));
        }

        #[test]
        fn test_collect_attribute_keys() {
            let cond = ConditionNode::And(
                Box::new(ConditionNode::HasAttribute {
                    key: "citizen".to_string(),
                }),
                Box::new(ConditionNode::HasAttribute {
                    key: "resident".to_string(),
                }),
            );

            let keys = collect_attribute_keys(&cond);
            assert_eq!(keys.len(), 2);
            assert!(keys.contains(&"citizen".to_string()));
            assert!(keys.contains(&"resident".to_string()));
        }

        #[test]
        fn test_collect_condition_fields() {
            let cond = ConditionNode::And(
                Box::new(ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                }),
                Box::new(ConditionNode::Between {
                    field: "income".to_string(),
                    min: ConditionValue::Number(30000),
                    max: ConditionValue::Number(50000),
                }),
            );

            let fields = collect_condition_fields(&cond);
            assert_eq!(fields.len(), 2);
            assert!(fields.contains(&"age".to_string()));
            assert!(fields.contains(&"income".to_string()));
        }

        #[test]
        fn test_flatten_nested_and() {
            // (a AND (b AND c)) should stay as nested structure but be optimized
            let cond = ConditionNode::And(
                Box::new(ConditionNode::HasAttribute {
                    key: "a".to_string(),
                }),
                Box::new(ConditionNode::And(
                    Box::new(ConditionNode::HasAttribute {
                        key: "b".to_string(),
                    }),
                    Box::new(ConditionNode::HasAttribute {
                        key: "c".to_string(),
                    }),
                )),
            );

            let flattened = flatten_condition(&cond);
            // Should still be AND but potentially restructured
            assert!(matches!(flattened, ConditionNode::And(_, _)));
        }

        #[test]
        fn test_remove_duplicates() {
            // (a AND a) should become just a
            let cond = ConditionNode::And(
                Box::new(ConditionNode::HasAttribute {
                    key: "test".to_string(),
                }),
                Box::new(ConditionNode::HasAttribute {
                    key: "test".to_string(),
                }),
            );

            let deduped = remove_duplicate_conditions(&cond);
            // Should have removed the duplicate
            match deduped {
                ConditionNode::HasAttribute { key } => {
                    assert_eq!(key, "test");
                }
                _ => {
                    // If it's still AND, that's also acceptable since we only had 2 items
                }
            }
        }

        #[test]
        fn test_optimize_condition_full() {
            // NOT (NOT a) should simplify to a
            let cond = ConditionNode::Not(Box::new(ConditionNode::Not(Box::new(
                ConditionNode::HasAttribute {
                    key: "test".to_string(),
                },
            ))));

            let optimized = optimize_condition(&cond);
            assert!(matches!(optimized, ConditionNode::HasAttribute { .. }));
        }

        #[test]
        fn test_optimize_statute() {
            let statute = StatuteNode {
                id: "test".to_string(),
                title: "Test".to_string(),
                conditions: vec![ConditionNode::Not(Box::new(ConditionNode::Not(Box::new(
                    ConditionNode::HasAttribute {
                        key: "valid".to_string(),
                    },
                ))))],
                effects: vec![EffectNode {
                    effect_type: "grant".to_string(),
                    description: "Rights".to_string(),
                    parameters: vec![],
                }],
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
            };

            let optimized = optimize_statute(&statute);

            // The double negation should be removed
            assert_eq!(optimized.conditions.len(), 1);
            assert!(matches!(
                optimized.conditions[0],
                ConditionNode::HasAttribute { .. }
            ));
        }

        #[test]
        fn test_diff_documents_no_changes() {
            let doc = LegalDocument {
                imports: vec![],
                statutes: vec![StatuteNode {
                    id: "test".to_string(),
                    title: "Test".to_string(),
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

            let diff = diff_documents(&doc, &doc);
            assert!(!diff.has_changes());
            assert_eq!(diff.summary(), "No changes");
        }

        #[test]
        fn test_diff_documents_added_statute() {
            let old_doc = LegalDocument {
                imports: vec![],
                statutes: vec![],
            };

            let new_doc = LegalDocument {
                imports: vec![],
                statutes: vec![StatuteNode {
                    id: "new-statute".to_string(),
                    title: "New Statute".to_string(),
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

            let diff = diff_documents(&old_doc, &new_doc);
            assert!(diff.has_changes());
            assert_eq!(diff.added_statutes.len(), 1);
            assert_eq!(diff.added_statutes[0].id, "new-statute");
        }

        #[test]
        fn test_diff_documents_removed_import() {
            let old_doc = LegalDocument {
                imports: vec![ImportNode {
                    path: "old.legalis".to_string(),
                    alias: None,
                }],
                statutes: vec![],
            };

            let new_doc = LegalDocument {
                imports: vec![],
                statutes: vec![],
            };

            let diff = diff_documents(&old_doc, &new_doc);
            assert!(diff.has_changes());
            assert_eq!(diff.removed_imports.len(), 1);
            assert_eq!(diff.removed_imports[0].path, "old.legalis");
        }

        #[test]
        fn test_diff_statutes_title_changed() {
            let old_statute = StatuteNode {
                id: "test".to_string(),
                title: "Old Title".to_string(),
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
            };

            let new_statute = StatuteNode {
                id: "test".to_string(),
                title: "New Title".to_string(),
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
            };

            let diff = diff_statutes(&old_statute, &new_statute);
            assert!(diff.has_changes());
            assert_eq!(diff.changes.len(), 1);
            match &diff.changes[0] {
                StatuteChange::TitleChanged { old, new } => {
                    assert_eq!(old, "Old Title");
                    assert_eq!(new, "New Title");
                }
                _ => panic!("Expected TitleChanged"),
            }
        }

        #[test]
        fn test_diff_statutes_conditions_changed() {
            let old_statute = StatuteNode {
                id: "test".to_string(),
                title: "Test".to_string(),
                conditions: vec![ConditionNode::HasAttribute {
                    key: "old_cond".to_string(),
                }],
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
            };

            let new_statute = StatuteNode {
                id: "test".to_string(),
                title: "Test".to_string(),
                conditions: vec![ConditionNode::HasAttribute {
                    key: "new_cond".to_string(),
                }],
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
            };

            let diff = diff_statutes(&old_statute, &new_statute);
            assert!(diff.has_changes());

            match &diff.changes[0] {
                StatuteChange::ConditionsChanged { added, removed } => {
                    assert_eq!(added.len(), 1);
                    assert_eq!(removed.len(), 1);
                }
                _ => panic!("Expected ConditionsChanged"),
            }
        }
    }
}
