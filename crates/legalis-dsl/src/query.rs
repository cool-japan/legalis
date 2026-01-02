//! Query API for filtering and searching legal documents.
//!
//! This module provides a fluent query interface for searching and filtering
//! statutes based on various criteria such as jurisdiction, version, dates,
//! conditions, and more.

use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use chrono::NaiveDate;
use std::collections::HashSet;

/// A query builder for filtering statutes.
#[derive(Debug, Default, Clone)]
pub struct StatuteQuery {
    /// Filter by jurisdiction
    jurisdiction: Option<String>,
    /// Filter by minimum version
    min_version: Option<u32>,
    /// Filter by maximum version
    max_version: Option<u32>,
    /// Filter by effective date range
    effective_after: Option<NaiveDate>,
    effective_before: Option<NaiveDate>,
    /// Filter by IDs
    ids: Option<HashSet<String>>,
    /// Filter by title pattern (case-insensitive substring match)
    title_contains: Option<String>,
    /// Filter statutes that require specific statute IDs
    requires_any: Option<HashSet<String>>,
    requires_all: Option<HashSet<String>>,
    /// Filter statutes that supersede specific IDs
    supersedes: Option<HashSet<String>>,
    /// Filter by presence of discretion
    has_discretion: Option<bool>,
    /// Filter by presence of exceptions
    has_exceptions: Option<bool>,
    /// Filter by presence of amendments
    has_amendments: Option<bool>,
    /// Filter by minimum number of conditions
    min_conditions: Option<usize>,
}

impl StatuteQuery {
    /// Creates a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters by jurisdiction (exact match, case-insensitive).
    pub fn jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }

    /// Filters by minimum version (inclusive).
    pub fn min_version(mut self, version: u32) -> Self {
        self.min_version = Some(version);
        self
    }

    /// Filters by maximum version (inclusive).
    pub fn max_version(mut self, version: u32) -> Self {
        self.max_version = Some(version);
        self
    }

    /// Filters by exact version.
    pub fn version(mut self, version: u32) -> Self {
        self.min_version = Some(version);
        self.max_version = Some(version);
        self
    }

    /// Filters statutes effective after the given date (inclusive).
    pub fn effective_after(mut self, date: NaiveDate) -> Self {
        self.effective_after = Some(date);
        self
    }

    /// Filters statutes effective before the given date (inclusive).
    pub fn effective_before(mut self, date: NaiveDate) -> Self {
        self.effective_before = Some(date);
        self
    }

    /// Filters by statute IDs (any of the provided IDs).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.ids = Some(ids.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Filters by title containing the given substring (case-insensitive).
    pub fn title_contains(mut self, pattern: impl Into<String>) -> Self {
        self.title_contains = Some(pattern.into());
        self
    }

    /// Filters statutes that require any of the given statute IDs.
    pub fn requires_any<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.requires_any = Some(ids.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Filters statutes that require all of the given statute IDs.
    pub fn requires_all<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.requires_all = Some(ids.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Filters statutes that supersede any of the given IDs.
    pub fn supersedes<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.supersedes = Some(ids.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Filters statutes based on whether they have discretion logic.
    pub fn has_discretion(mut self, has: bool) -> Self {
        self.has_discretion = Some(has);
        self
    }

    /// Filters statutes based on whether they have exceptions.
    pub fn has_exceptions(mut self, has: bool) -> Self {
        self.has_exceptions = Some(has);
        self
    }

    /// Filters statutes based on whether they have amendments.
    pub fn has_amendments(mut self, has: bool) -> Self {
        self.has_amendments = Some(has);
        self
    }

    /// Filters statutes by minimum number of conditions.
    pub fn min_conditions(mut self, min: usize) -> Self {
        self.min_conditions = Some(min);
        self
    }

    /// Executes the query against a legal document.
    pub fn execute<'a>(&self, doc: &'a LegalDocument) -> Vec<&'a StatuteNode> {
        doc.statutes
            .iter()
            .filter(|statute| self.matches(statute))
            .collect()
    }

    /// Executes the query and returns cloned results.
    pub fn execute_cloned(&self, doc: &LegalDocument) -> Vec<StatuteNode> {
        doc.statutes
            .iter()
            .filter(|statute| self.matches(statute))
            .cloned()
            .collect()
    }

    /// Checks if a statute matches this query.
    fn matches(&self, statute: &StatuteNode) -> bool {
        // Check IDs
        if let Some(ref ids) = self.ids {
            if !ids.contains(&statute.id) {
                return false;
            }
        }

        // Check title
        if let Some(ref pattern) = self.title_contains {
            if !statute
                .title
                .to_lowercase()
                .contains(&pattern.to_lowercase())
            {
                return false;
            }
        }

        // Check requires_any
        if let Some(ref required) = self.requires_any {
            if !statute.requires.iter().any(|r| required.contains(r)) {
                return false;
            }
        }

        // Check requires_all
        if let Some(ref required) = self.requires_all {
            if !required.iter().all(|r| statute.requires.contains(r)) {
                return false;
            }
        }

        // Check supersedes
        if let Some(ref superseded) = self.supersedes {
            if !statute.supersedes.iter().any(|s| superseded.contains(s)) {
                return false;
            }
        }

        // Check discretion
        if let Some(has) = self.has_discretion {
            if has != statute.discretion.is_some() {
                return false;
            }
        }

        // Check exceptions
        if let Some(has) = self.has_exceptions {
            if has == statute.exceptions.is_empty() {
                return false;
            }
        }

        // Check amendments
        if let Some(has) = self.has_amendments {
            if has == statute.amendments.is_empty() {
                return false;
            }
        }

        // Check minimum conditions
        if let Some(min) = self.min_conditions {
            if statute.conditions.len() < min {
                return false;
            }
        }

        true
    }

    /// Counts the number of statutes matching this query.
    pub fn count(&self, doc: &LegalDocument) -> usize {
        doc.statutes
            .iter()
            .filter(|statute| self.matches(statute))
            .count()
    }

    /// Checks if any statute matches this query.
    pub fn exists(&self, doc: &LegalDocument) -> bool {
        doc.statutes.iter().any(|statute| self.matches(statute))
    }
}

/// Searches for conditions matching specific criteria.
#[derive(Debug, Default)]
pub struct ConditionSearch {
    /// Field name to search for
    field: Option<String>,
    /// Minimum value (for numeric conditions)
    min_value: Option<i64>,
    /// Maximum value (for numeric conditions)
    max_value: Option<i64>,
}

impl ConditionSearch {
    /// Creates a new condition search.
    pub fn new() -> Self {
        Self::default()
    }

    /// Searches for conditions involving a specific field.
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Searches for numeric conditions with values >= min.
    pub fn min_value(mut self, min: i64) -> Self {
        self.min_value = Some(min);
        self
    }

    /// Searches for numeric conditions with values <= max.
    pub fn max_value(mut self, max: i64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Finds all matching conditions in a statute.
    pub fn find_in_statute<'a>(&self, statute: &'a StatuteNode) -> Vec<&'a ConditionNode> {
        let mut results = Vec::new();
        for condition in &statute.conditions {
            self.find_in_condition(condition, &mut results);
        }
        results
    }

    /// Recursively searches for matching conditions.
    fn find_in_condition<'a>(
        &self,
        condition: &'a ConditionNode,
        results: &mut Vec<&'a ConditionNode>,
    ) {
        if self.matches_condition(condition) {
            results.push(condition);
        }

        // Recursively search nested conditions
        match condition {
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.find_in_condition(left, results);
                self.find_in_condition(right, results);
            }
            ConditionNode::Not(inner) => {
                self.find_in_condition(inner, results);
            }
            _ => {}
        }
    }

    /// Checks if a condition matches the search criteria.
    fn matches_condition(&self, condition: &ConditionNode) -> bool {
        // Check field name
        if let Some(ref field) = self.field {
            let condition_field = match condition {
                ConditionNode::Comparison { field: f, .. }
                | ConditionNode::Between { field: f, .. }
                | ConditionNode::In { field: f, .. }
                | ConditionNode::Like { field: f, .. }
                | ConditionNode::Matches { field: f, .. }
                | ConditionNode::InRange { field: f, .. }
                | ConditionNode::NotInRange { field: f, .. } => Some(f),
                ConditionNode::HasAttribute { key } => Some(key),
                _ => None,
            };

            if condition_field != Some(field) {
                return false;
            }
        }

        // Check value ranges
        if let Some(min) = self.min_value {
            let has_min = match condition {
                ConditionNode::Comparison {
                    value: ConditionValue::Number(n),
                    ..
                } => *n >= min,
                ConditionNode::Between {
                    min: ConditionValue::Number(n),
                    ..
                } => *n >= min,
                _ => false,
            };

            if !has_min {
                return false;
            }
        }

        if let Some(max) = self.max_value {
            let has_max = match condition {
                ConditionNode::Comparison {
                    value: ConditionValue::Number(n),
                    ..
                } => *n <= max,
                ConditionNode::Between {
                    max: ConditionValue::Number(n),
                    ..
                } => *n <= max,
                _ => false,
            };

            if !has_max {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> LegalDocument {
        LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute1".to_string(),
                    title: "Voting Rights Act".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![ConditionNode::Comparison {
                        field: "age".to_string(),
                        operator: ">=".to_string(),
                        value: ConditionValue::Number(18),
                    }],
                    effects: vec![],
                    discretion: Some("Review required".to_string()),
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec!["citizenship".to_string()],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
                StatuteNode {
                    id: "statute2".to_string(),
                    title: "Tax Benefits".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![ConditionNode::Between {
                        field: "income".to_string(),
                        min: ConditionValue::Number(20000),
                        max: ConditionValue::Number(50000),
                    }],
                    effects: vec![],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec!["old-tax-law".to_string()],
                    defaults: vec![],
                    requires: vec![],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
                StatuteNode {
                    id: "statute3".to_string(),
                    title: "Employment Rights".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![
                        ConditionNode::Comparison {
                            field: "age".to_string(),
                            operator: ">=".to_string(),
                            value: ConditionValue::Number(16),
                        },
                        ConditionNode::HasAttribute {
                            key: "work_permit".to_string(),
                        },
                    ],
                    effects: vec![],
                    discretion: Some("Manager approval".to_string()),
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec!["citizenship".to_string(), "background-check".to_string()],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
            ],
        }
    }

    #[test]
    fn test_query_by_title() {
        let doc = create_test_document();
        let results = StatuteQuery::new().title_contains("voting").execute(&doc);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "statute1");
    }

    #[test]
    fn test_query_by_discretion() {
        let doc = create_test_document();
        let results = StatuteQuery::new().has_discretion(true).execute(&doc);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|s| s.id == "statute1"));
        assert!(results.iter().any(|s| s.id == "statute3"));
    }

    #[test]
    fn test_query_by_requires_any() {
        let doc = create_test_document();
        let results = StatuteQuery::new()
            .requires_any(vec!["citizenship"])
            .execute(&doc);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_by_requires_all() {
        let doc = create_test_document();
        let results = StatuteQuery::new()
            .requires_all(vec!["citizenship", "background-check"])
            .execute(&doc);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "statute3");
    }

    #[test]
    fn test_query_by_supersedes() {
        let doc = create_test_document();
        let results = StatuteQuery::new()
            .supersedes(vec!["old-tax-law"])
            .execute(&doc);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "statute2");
    }

    #[test]
    fn test_query_count() {
        let doc = create_test_document();
        let count = StatuteQuery::new().has_discretion(true).count(&doc);

        assert_eq!(count, 2);
    }

    #[test]
    fn test_query_exists() {
        let doc = create_test_document();
        assert!(StatuteQuery::new().title_contains("voting").exists(&doc));
        assert!(
            !StatuteQuery::new()
                .title_contains("nonexistent")
                .exists(&doc)
        );
    }

    #[test]
    fn test_condition_search() {
        let doc = create_test_document();
        let statute = &doc.statutes[0];

        let search = ConditionSearch::new().field("age").min_value(18);
        let results = search.find_in_statute(statute);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_query_min_conditions() {
        let doc = create_test_document();
        let results = StatuteQuery::new().min_conditions(2).execute(&doc);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "statute3");
    }
}
