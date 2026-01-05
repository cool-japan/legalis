//! Completion suggestions for Legalis DSL
//!
//! This module provides intelligent completion suggestions for statute authoring,
//! helping users write legal DSL code more efficiently.

use crate::ast::*;
use std::collections::HashMap;

/// Completion item representing a suggestion
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionItem {
    /// The label shown to the user
    pub label: String,
    /// The text to insert
    pub insert_text: String,
    /// Description/documentation
    pub description: Option<String>,
    /// Category of the completion
    pub category: CompletionCategory,
    /// Priority/relevance score (higher = more relevant)
    pub score: f64,
}

/// Category of completion suggestion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionCategory {
    /// Keyword (STATUTE, WHEN, THEN, etc.)
    Keyword,
    /// Statute ID suggestion
    StatuteId,
    /// Condition field
    Field,
    /// Comparison operator
    Operator,
    /// Effect type
    EffectType,
    /// Metadata clause
    Metadata,
    /// Module system keyword
    Module,
    /// Snippet (multi-line template)
    Snippet,
}

/// Context information for completion
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionContext {
    /// At the document level
    Document,
    /// Inside a statute definition
    Statute,
    /// In a condition clause
    Condition,
    /// In an effect clause
    Effect,
    /// In metadata
    Metadata,
    /// After a field name
    AfterField,
    /// In import/export statement
    Module,
}

/// Completion provider that suggests items based on context
pub struct CompletionProvider {
    /// Existing statutes for contextual suggestions
    existing_statutes: Vec<StatuteNode>,
    /// Common field names seen in the codebase
    common_fields: HashMap<String, usize>,
}

impl CompletionProvider {
    /// Create a new completion provider
    pub fn new() -> Self {
        Self {
            existing_statutes: Vec::new(),
            common_fields: HashMap::new(),
        }
    }

    /// Create a completion provider from an existing document
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut provider = Self::new();
        provider.index_document(doc);
        provider
    }

    /// Index a document to learn from existing patterns
    pub fn index_document(&mut self, doc: &LegalDocument) {
        self.existing_statutes = doc.statutes.clone();

        // Extract common field names
        for statute in &doc.statutes {
            for condition in &statute.conditions {
                self.extract_fields_from_condition(condition);
            }
        }
    }

    fn extract_fields_from_condition(&mut self, condition: &ConditionNode) {
        match condition {
            ConditionNode::Comparison { field, .. }
            | ConditionNode::Between { field, .. }
            | ConditionNode::In { field, .. }
            | ConditionNode::Like { field, .. }
            | ConditionNode::Matches { field, .. }
            | ConditionNode::InRange { field, .. }
            | ConditionNode::NotInRange { field, .. } => {
                *self.common_fields.entry(field.clone()).or_insert(0) += 1;
            }
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.extract_fields_from_condition(left);
                self.extract_fields_from_condition(right);
            }
            ConditionNode::Not(inner) => {
                self.extract_fields_from_condition(inner);
            }
            _ => {}
        }
    }

    /// Get completion suggestions for a given context
    pub fn complete(&self, context: CompletionContext, prefix: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        match context {
            CompletionContext::Document => {
                items.extend(self.document_level_completions(prefix));
            }
            CompletionContext::Statute => {
                items.extend(self.statute_level_completions(prefix));
            }
            CompletionContext::Condition => {
                items.extend(self.condition_completions(prefix));
            }
            CompletionContext::Effect => {
                items.extend(self.effect_completions(prefix));
            }
            CompletionContext::Metadata => {
                items.extend(self.metadata_completions(prefix));
            }
            CompletionContext::AfterField => {
                items.extend(self.operator_completions(prefix));
            }
            CompletionContext::Module => {
                items.extend(self.module_completions(prefix));
            }
        }

        // Filter by prefix and sort by score
        items.retain(|item| {
            item.label
                .to_lowercase()
                .starts_with(&prefix.to_lowercase())
        });
        items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        items
    }

    fn document_level_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "STATUTE".to_string(),
                insert_text: "STATUTE ${1:id}: \"${2:title}\" {\n    $0\n}".to_string(),
                description: Some("Define a new statute".to_string()),
                category: CompletionCategory::Snippet,
                score: 10.0,
            },
            CompletionItem {
                label: "IMPORT".to_string(),
                insert_text: "IMPORT \"${1:path}\"".to_string(),
                description: Some("Import from another module".to_string()),
                category: CompletionCategory::Keyword,
                score: 8.0,
            },
            CompletionItem {
                label: "NAMESPACE".to_string(),
                insert_text: "NAMESPACE ${1:path}".to_string(),
                description: Some("Declare a namespace".to_string()),
                category: CompletionCategory::Keyword,
                score: 7.0,
            },
            CompletionItem {
                label: "EXPORT".to_string(),
                insert_text: "EXPORT ${1:items}".to_string(),
                description: Some("Export public items".to_string()),
                category: CompletionCategory::Keyword,
                score: 6.0,
            },
        ]
    }

    fn statute_level_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "WHEN".to_string(),
                insert_text: "WHEN ${1:condition}".to_string(),
                description: Some("Add a condition".to_string()),
                category: CompletionCategory::Keyword,
                score: 10.0,
            },
            CompletionItem {
                label: "THEN".to_string(),
                insert_text: "THEN ${1:GRANT|REVOKE|OBLIGATION|PROHIBITION} \"${2:description}\""
                    .to_string(),
                description: Some("Add an effect".to_string()),
                category: CompletionCategory::Keyword,
                score: 9.0,
            },
            CompletionItem {
                label: "DEFAULT".to_string(),
                insert_text: "DEFAULT ${1:field} = ${2:value}".to_string(),
                description: Some("Set a default value".to_string()),
                category: CompletionCategory::Keyword,
                score: 7.0,
            },
            CompletionItem {
                label: "EXCEPTION".to_string(),
                insert_text: "EXCEPTION WHEN ${1:condition} \"${2:description}\"".to_string(),
                description: Some("Add an exception clause".to_string()),
                category: CompletionCategory::Keyword,
                score: 7.0,
            },
            CompletionItem {
                label: "REQUIRES".to_string(),
                insert_text: "REQUIRES ${1:statute_id}".to_string(),
                description: Some("Declare a dependency on another statute".to_string()),
                category: CompletionCategory::Keyword,
                score: 6.0,
            },
            CompletionItem {
                label: "SUPERSEDES".to_string(),
                insert_text: "SUPERSEDES ${1:statute_id}".to_string(),
                description: Some("Supersede an older statute".to_string()),
                category: CompletionCategory::Keyword,
                score: 6.0,
            },
            CompletionItem {
                label: "DISCRETION".to_string(),
                insert_text: "DISCRETION \"${1:guidance}\"".to_string(),
                description: Some("Add discretionary guidance".to_string()),
                category: CompletionCategory::Keyword,
                score: 5.0,
            },
            CompletionItem {
                label: "PRIORITY".to_string(),
                insert_text: "PRIORITY ${1:level}".to_string(),
                description: Some("Set priority level".to_string()),
                category: CompletionCategory::Keyword,
                score: 5.0,
            },
            CompletionItem {
                label: "SCOPE".to_string(),
                insert_text: "SCOPE ${1:entity_types}".to_string(),
                description: Some("Define applicable scope".to_string()),
                category: CompletionCategory::Keyword,
                score: 5.0,
            },
        ]
    }

    fn condition_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        let mut items = vec![
            CompletionItem {
                label: "AND".to_string(),
                insert_text: "AND ".to_string(),
                description: Some("Logical AND operator".to_string()),
                category: CompletionCategory::Operator,
                score: 10.0,
            },
            CompletionItem {
                label: "OR".to_string(),
                insert_text: "OR ".to_string(),
                description: Some("Logical OR operator".to_string()),
                category: CompletionCategory::Operator,
                score: 9.0,
            },
            CompletionItem {
                label: "NOT".to_string(),
                insert_text: "NOT ".to_string(),
                description: Some("Logical NOT operator".to_string()),
                category: CompletionCategory::Operator,
                score: 8.0,
            },
            CompletionItem {
                label: "HAS".to_string(),
                insert_text: "HAS ${1:attribute}".to_string(),
                description: Some("Check if an attribute exists".to_string()),
                category: CompletionCategory::Keyword,
                score: 8.0,
            },
        ];

        // Add common field suggestions
        let mut fields: Vec<_> = self.common_fields.iter().collect();
        fields.sort_by(|a, b| b.1.cmp(a.1)); // Sort by frequency

        for (field, count) in fields.iter().take(10) {
            items.push(CompletionItem {
                label: (*field).clone(),
                insert_text: format!("{} ", field),
                description: Some(format!("Field (used {} times)", count)),
                category: CompletionCategory::Field,
                score: 7.0 + (**count as f64 * 0.1),
            });
        }

        // Add standard fields
        for field in &["age", "income", "date", "status", "amount", "type"] {
            if !self.common_fields.contains_key(*field) {
                items.push(CompletionItem {
                    label: (*field).to_string(),
                    insert_text: format!("{} ", field),
                    description: Some("Common field".to_string()),
                    category: CompletionCategory::Field,
                    score: 5.0,
                });
            }
        }

        items
    }

    fn operator_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "=".to_string(),
                insert_text: "= ".to_string(),
                description: Some("Equals".to_string()),
                category: CompletionCategory::Operator,
                score: 10.0,
            },
            CompletionItem {
                label: ">".to_string(),
                insert_text: "> ".to_string(),
                description: Some("Greater than".to_string()),
                category: CompletionCategory::Operator,
                score: 9.0,
            },
            CompletionItem {
                label: "<".to_string(),
                insert_text: "< ".to_string(),
                description: Some("Less than".to_string()),
                category: CompletionCategory::Operator,
                score: 9.0,
            },
            CompletionItem {
                label: ">=".to_string(),
                insert_text: ">= ".to_string(),
                description: Some("Greater than or equal".to_string()),
                category: CompletionCategory::Operator,
                score: 8.0,
            },
            CompletionItem {
                label: "<=".to_string(),
                insert_text: "<= ".to_string(),
                description: Some("Less than or equal".to_string()),
                category: CompletionCategory::Operator,
                score: 8.0,
            },
            CompletionItem {
                label: "BETWEEN".to_string(),
                insert_text: "BETWEEN ${1:min} AND ${2:max}".to_string(),
                description: Some("Range check".to_string()),
                category: CompletionCategory::Keyword,
                score: 7.0,
            },
            CompletionItem {
                label: "IN".to_string(),
                insert_text: "IN (${1:values})".to_string(),
                description: Some("Set membership".to_string()),
                category: CompletionCategory::Keyword,
                score: 7.0,
            },
            CompletionItem {
                label: "LIKE".to_string(),
                insert_text: "LIKE \"${1:pattern}\"".to_string(),
                description: Some("Pattern matching".to_string()),
                category: CompletionCategory::Keyword,
                score: 6.0,
            },
            CompletionItem {
                label: "MATCHES".to_string(),
                insert_text: "MATCHES \"${1:regex}\"".to_string(),
                description: Some("Regex matching".to_string()),
                category: CompletionCategory::Keyword,
                score: 6.0,
            },
        ]
    }

    fn effect_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "GRANT".to_string(),
                insert_text: "GRANT \"${1:description}\"".to_string(),
                description: Some("Grant a right or benefit".to_string()),
                category: CompletionCategory::EffectType,
                score: 10.0,
            },
            CompletionItem {
                label: "REVOKE".to_string(),
                insert_text: "REVOKE \"${1:description}\"".to_string(),
                description: Some("Revoke a right or benefit".to_string()),
                category: CompletionCategory::EffectType,
                score: 9.0,
            },
            CompletionItem {
                label: "OBLIGATION".to_string(),
                insert_text: "OBLIGATION \"${1:description}\"".to_string(),
                description: Some("Impose an obligation".to_string()),
                category: CompletionCategory::EffectType,
                score: 9.0,
            },
            CompletionItem {
                label: "PROHIBITION".to_string(),
                insert_text: "PROHIBITION \"${1:description}\"".to_string(),
                description: Some("Impose a prohibition".to_string()),
                category: CompletionCategory::EffectType,
                score: 9.0,
            },
        ]
    }

    fn metadata_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "JURISDICTION".to_string(),
                insert_text: "JURISDICTION \"${1:code}\"".to_string(),
                description: Some("Set jurisdiction".to_string()),
                category: CompletionCategory::Metadata,
                score: 10.0,
            },
            CompletionItem {
                label: "VERSION".to_string(),
                insert_text: "VERSION ${1:number}".to_string(),
                description: Some("Set version number".to_string()),
                category: CompletionCategory::Metadata,
                score: 9.0,
            },
            CompletionItem {
                label: "EFFECTIVE_DATE".to_string(),
                insert_text: "EFFECTIVE_DATE ${1:YYYY-MM-DD}".to_string(),
                description: Some("Set effective date".to_string()),
                category: CompletionCategory::Metadata,
                score: 9.0,
            },
            CompletionItem {
                label: "EXPIRY_DATE".to_string(),
                insert_text: "EXPIRY_DATE ${1:YYYY-MM-DD}".to_string(),
                description: Some("Set expiry date".to_string()),
                category: CompletionCategory::Metadata,
                score: 8.0,
            },
        ]
    }

    fn module_completions(&self, _prefix: &str) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "IMPORT".to_string(),
                insert_text: "IMPORT \"${1:path}\"".to_string(),
                description: Some("Import from another module".to_string()),
                category: CompletionCategory::Module,
                score: 10.0,
            },
            CompletionItem {
                label: "FROM".to_string(),
                insert_text: "FROM \"${1:path}\"".to_string(),
                description: Some("Import from path".to_string()),
                category: CompletionCategory::Module,
                score: 9.0,
            },
            CompletionItem {
                label: "EXPORT".to_string(),
                insert_text: "EXPORT ${1:items}".to_string(),
                description: Some("Export items".to_string()),
                category: CompletionCategory::Module,
                score: 9.0,
            },
            CompletionItem {
                label: "PUBLIC".to_string(),
                insert_text: "PUBLIC ".to_string(),
                description: Some("Mark as public".to_string()),
                category: CompletionCategory::Module,
                score: 8.0,
            },
            CompletionItem {
                label: "PRIVATE".to_string(),
                insert_text: "PRIVATE ".to_string(),
                description: Some("Mark as private".to_string()),
                category: CompletionCategory::Module,
                score: 8.0,
            },
        ]
    }

    /// Suggest statute IDs based on existing patterns
    pub fn suggest_statute_ids(&self, prefix: &str, max_suggestions: usize) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // Analyze existing statute ID patterns
        let mut patterns: HashMap<String, usize> = HashMap::new();
        for statute in &self.existing_statutes {
            if let Some(prefix_part) = statute.id.split('-').next() {
                *patterns.entry(prefix_part.to_string()).or_insert(0) += 1;
            }
        }

        // Suggest IDs based on common patterns
        for (pattern, count) in patterns.iter() {
            if pattern.to_lowercase().starts_with(&prefix.to_lowercase()) {
                items.push(CompletionItem {
                    label: format!("{}-XXX", pattern),
                    insert_text: format!("{}-", pattern),
                    description: Some(format!("Common pattern (used {} times)", count)),
                    category: CompletionCategory::StatuteId,
                    score: 10.0 + (*count as f64 * 0.5),
                });
            }
        }

        items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        items.truncate(max_suggestions);
        items
    }
}

impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_level_completions() {
        let provider = CompletionProvider::new();
        let items = provider.complete(CompletionContext::Document, "");

        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "STATUTE"));
        assert!(items.iter().any(|i| i.label == "IMPORT"));
    }

    #[test]
    fn test_statute_level_completions() {
        let provider = CompletionProvider::new();
        let items = provider.complete(CompletionContext::Statute, "");

        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "WHEN"));
        assert!(items.iter().any(|i| i.label == "THEN"));
    }

    #[test]
    fn test_condition_completions() {
        let provider = CompletionProvider::new();
        let items = provider.complete(CompletionContext::Condition, "");

        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "AND"));
        assert!(items.iter().any(|i| i.label == "OR"));
    }

    #[test]
    fn test_effect_completions() {
        let provider = CompletionProvider::new();
        let items = provider.complete(CompletionContext::Effect, "");

        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "GRANT"));
        assert!(items.iter().any(|i| i.label == "REVOKE"));
    }

    #[test]
    fn test_prefix_filtering() {
        let provider = CompletionProvider::new();
        let items = provider.complete(CompletionContext::Effect, "GR");

        assert!(
            items
                .iter()
                .all(|i| i.label.to_lowercase().starts_with("gr"))
        );
        assert!(items.iter().any(|i| i.label == "GRANT"));
    }

    #[test]
    fn test_completion_from_document() {
        let doc = LegalDocument {
            namespace: None,
            imports: vec![],
            exports: vec![],
            statutes: vec![StatuteNode {
                id: "test-001".to_string(),
                title: "Test".to_string(),
                visibility: crate::module_system::Visibility::Public,
                conditions: vec![ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                }],
                effects: vec![],
                defaults: vec![],
                exceptions: vec![],
                discretion: None,
                amendments: vec![],
                requires: vec![],
                supersedes: vec![],
                delegates: vec![],
                priority: None,
                scope: None,
                constraints: vec![],
            }],
        };

        let provider = CompletionProvider::from_document(&doc);
        let items = provider.complete(CompletionContext::Condition, "a");

        assert!(items.iter().any(|i| i.label == "age"));
    }
}
