//! Statute comparison and diff utilities.
//!
//! This module provides tools for comparing statutes and legal documents,
//! identifying differences, and generating diff reports.

use crate::ast::{LegalDocument, StatuteNode};
use std::collections::{HashMap, HashSet};

/// Represents differences between two legal documents or statutes.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentDiff {
    /// Statutes added in the new document
    pub added_statutes: Vec<String>,
    /// Statutes removed from the old document
    pub removed_statutes: Vec<String>,
    /// Statutes that exist in both but have changes
    pub modified_statutes: Vec<StatuteDiff>,
    /// Statutes that are identical in both documents
    pub unchanged_statutes: Vec<String>,
}

/// Represents differences between two versions of a statute.
#[derive(Debug, Clone, PartialEq)]
pub struct StatuteDiff {
    /// Statute ID
    pub id: String,
    /// Changes to the statute
    pub changes: Vec<Change>,
}

/// A single change in a statute.
#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    /// Title changed
    TitleChanged { old: String, new: String },
    /// Condition added
    ConditionAdded { condition: String },
    /// Condition removed
    ConditionRemoved { condition: String },
    /// Condition modified
    ConditionModified { old: String, new: String },
    /// Effect added
    EffectAdded { effect: String },
    /// Effect removed
    EffectRemoved { effect: String },
    /// Effect modified
    EffectModified { old: String, new: String },
    /// Discretion changed
    DiscretionChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Exception added
    ExceptionAdded { description: String },
    /// Exception removed
    ExceptionRemoved { description: String },
    /// Amendment added
    AmendmentAdded { description: String },
    /// Amendment removed
    AmendmentRemoved { description: String },
    /// Requires clause changed
    RequiresChanged { old: Vec<String>, new: Vec<String> },
    /// Supersedes clause changed
    SupersedesChanged { old: Vec<String>, new: Vec<String> },
}

impl DocumentDiff {
    /// Computes the diff between two legal documents.
    pub fn compute(old_doc: &LegalDocument, new_doc: &LegalDocument) -> Self {
        let old_ids: HashSet<&String> = old_doc.statutes.iter().map(|s| &s.id).collect();
        let new_ids: HashSet<&String> = new_doc.statutes.iter().map(|s| &s.id).collect();

        let mut added_statutes: Vec<String> =
            new_ids.difference(&old_ids).map(|s| (*s).clone()).collect();
        added_statutes.sort();

        let mut removed_statutes: Vec<String> =
            old_ids.difference(&new_ids).map(|s| (*s).clone()).collect();
        removed_statutes.sort();

        let common_ids: Vec<_> = old_ids.intersection(&new_ids).collect();

        let old_map: HashMap<&String, &StatuteNode> =
            old_doc.statutes.iter().map(|s| (&s.id, s)).collect();
        let new_map: HashMap<&String, &StatuteNode> =
            new_doc.statutes.iter().map(|s| (&s.id, s)).collect();

        let mut modified_statutes = Vec::new();
        let mut unchanged_statutes = Vec::new();

        for id in common_ids {
            let old_statute = old_map[id];
            let new_statute = new_map[id];

            let changes = StatuteDiff::compute_changes(old_statute, new_statute);
            if changes.is_empty() {
                unchanged_statutes.push((*id).clone());
            } else {
                modified_statutes.push(StatuteDiff {
                    id: (*id).clone(),
                    changes,
                });
            }
        }

        unchanged_statutes.sort();
        modified_statutes.sort_by(|a, b| a.id.cmp(&b.id));

        Self {
            added_statutes,
            removed_statutes,
            modified_statutes,
            unchanged_statutes,
        }
    }

    /// Returns true if there are any differences.
    pub fn has_changes(&self) -> bool {
        !self.added_statutes.is_empty()
            || !self.removed_statutes.is_empty()
            || !self.modified_statutes.is_empty()
    }

    /// Generates a human-readable diff report.
    pub fn report(&self) -> String {
        let mut lines = Vec::new();

        lines.push("Document Diff Report".to_string());
        lines.push("===================".to_string());
        lines.push(String::new());

        if !self.added_statutes.is_empty() {
            lines.push("Added Statutes:".to_string());
            for id in &self.added_statutes {
                lines.push(format!("  + {}", id));
            }
            lines.push(String::new());
        }

        if !self.removed_statutes.is_empty() {
            lines.push("Removed Statutes:".to_string());
            for id in &self.removed_statutes {
                lines.push(format!("  - {}", id));
            }
            lines.push(String::new());
        }

        if !self.modified_statutes.is_empty() {
            lines.push("Modified Statutes:".to_string());
            for statute_diff in &self.modified_statutes {
                lines.push(format!("  ~ {}", statute_diff.id));
                for change in &statute_diff.changes {
                    lines.push(format!("    {}", change.description()));
                }
            }
            lines.push(String::new());
        }

        if !self.unchanged_statutes.is_empty() {
            lines.push(format!(
                "Unchanged: {} statute(s)",
                self.unchanged_statutes.len()
            ));
        }

        lines.join("\n")
    }
}

impl StatuteDiff {
    /// Computes changes between two versions of the same statute.
    fn compute_changes(old: &StatuteNode, new: &StatuteNode) -> Vec<Change> {
        let mut changes = Vec::new();

        // Check title
        if old.title != new.title {
            changes.push(Change::TitleChanged {
                old: old.title.clone(),
                new: new.title.clone(),
            });
        }

        // Check conditions
        let old_conds: Vec<String> = old.conditions.iter().map(|c| format!("{:?}", c)).collect();
        let new_conds: Vec<String> = new.conditions.iter().map(|c| format!("{:?}", c)).collect();

        for cond in &new_conds {
            if !old_conds.contains(cond) {
                changes.push(Change::ConditionAdded {
                    condition: cond.clone(),
                });
            }
        }

        for cond in &old_conds {
            if !new_conds.contains(cond) {
                changes.push(Change::ConditionRemoved {
                    condition: cond.clone(),
                });
            }
        }

        // Check effects
        let old_effects: Vec<String> = old
            .effects
            .iter()
            .map(|e| format!("{} {}", e.effect_type, e.description))
            .collect();
        let new_effects: Vec<String> = new
            .effects
            .iter()
            .map(|e| format!("{} {}", e.effect_type, e.description))
            .collect();

        for effect in &new_effects {
            if !old_effects.contains(effect) {
                changes.push(Change::EffectAdded {
                    effect: effect.clone(),
                });
            }
        }

        for effect in &old_effects {
            if !new_effects.contains(effect) {
                changes.push(Change::EffectRemoved {
                    effect: effect.clone(),
                });
            }
        }

        // Check discretion
        if old.discretion != new.discretion {
            changes.push(Change::DiscretionChanged {
                old: old.discretion.clone(),
                new: new.discretion.clone(),
            });
        }

        // Check requires
        if old.requires != new.requires {
            changes.push(Change::RequiresChanged {
                old: old.requires.clone(),
                new: new.requires.clone(),
            });
        }

        // Check supersedes
        if old.supersedes != new.supersedes {
            changes.push(Change::SupersedesChanged {
                old: old.supersedes.clone(),
                new: new.supersedes.clone(),
            });
        }

        changes
    }
}

impl Change {
    /// Returns a human-readable description of the change.
    pub fn description(&self) -> String {
        match self {
            Change::TitleChanged { old, new } => {
                format!("Title: '{}' → '{}'", old, new)
            }
            Change::ConditionAdded { condition } => {
                format!("+ Condition: {}", condition)
            }
            Change::ConditionRemoved { condition } => {
                format!("- Condition: {}", condition)
            }
            Change::ConditionModified { old, new } => {
                format!("~ Condition: {} → {}", old, new)
            }
            Change::EffectAdded { effect } => {
                format!("+ Effect: {}", effect)
            }
            Change::EffectRemoved { effect } => {
                format!("- Effect: {}", effect)
            }
            Change::EffectModified { old, new } => {
                format!("~ Effect: {} → {}", old, new)
            }
            Change::DiscretionChanged { old, new } => {
                format!(
                    "Discretion: {} → {}",
                    old.as_deref().unwrap_or("none"),
                    new.as_deref().unwrap_or("none")
                )
            }
            Change::ExceptionAdded { description } => {
                format!("+ Exception: {}", description)
            }
            Change::ExceptionRemoved { description } => {
                format!("- Exception: {}", description)
            }
            Change::AmendmentAdded { description } => {
                format!("+ Amendment: {}", description)
            }
            Change::AmendmentRemoved { description } => {
                format!("- Amendment: {}", description)
            }
            Change::RequiresChanged { old, new } => {
                format!("Requires: {:?} → {:?}", old, new)
            }
            Change::SupersedesChanged { old, new } => {
                format!("Supersedes: {:?} → {:?}", old, new)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_statute(id: &str, title: &str) -> StatuteNode {
        StatuteNode {
            id: id.to_string(),
            title: title.to_string(),
            conditions: Vec::new(),
            effects: Vec::new(),
            discretion: None,
            exceptions: Vec::new(),
            amendments: Vec::new(),
            supersedes: Vec::new(),
            defaults: Vec::new(),
            requires: Vec::new(),
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        }
    }

    #[test]
    fn test_no_changes() {
        let statute1 = create_test_statute("test-1", "Test Statute");
        let statute2 = create_test_statute("test-1", "Test Statute");

        let changes = StatuteDiff::compute_changes(&statute1, &statute2);
        assert!(changes.is_empty());
    }

    #[test]
    fn test_title_change() {
        let statute1 = create_test_statute("test-1", "Old Title");
        let statute2 = create_test_statute("test-1", "New Title");

        let changes = StatuteDiff::compute_changes(&statute1, &statute2);
        assert_eq!(changes.len(), 1);

        match &changes[0] {
            Change::TitleChanged { old, new } => {
                assert_eq!(old, "Old Title");
                assert_eq!(new, "New Title");
            }
            _ => panic!("Expected TitleChanged"),
        }
    }

    #[test]
    fn test_document_diff_added_statutes() {
        let doc1 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "Statute 1")],
        };

        let doc2 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![
                create_test_statute("stat1", "Statute 1"),
                create_test_statute("stat2", "Statute 2"),
            ],
        };

        let diff = DocumentDiff::compute(&doc1, &doc2);
        assert_eq!(diff.added_statutes, vec!["stat2"]);
        assert!(diff.removed_statutes.is_empty());
        assert_eq!(diff.unchanged_statutes, vec!["stat1"]);
    }

    #[test]
    fn test_document_diff_removed_statutes() {
        let doc1 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![
                create_test_statute("stat1", "Statute 1"),
                create_test_statute("stat2", "Statute 2"),
            ],
        };

        let doc2 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "Statute 1")],
        };

        let diff = DocumentDiff::compute(&doc1, &doc2);
        assert!(diff.added_statutes.is_empty());
        assert_eq!(diff.removed_statutes, vec!["stat2"]);
        assert_eq!(diff.unchanged_statutes, vec!["stat1"]);
    }

    #[test]
    fn test_document_diff_modified_statutes() {
        let doc1 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "Old Title")],
        };

        let doc2 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "New Title")],
        };

        let diff = DocumentDiff::compute(&doc1, &doc2);
        assert!(diff.added_statutes.is_empty());
        assert!(diff.removed_statutes.is_empty());
        assert!(diff.unchanged_statutes.is_empty());
        assert_eq!(diff.modified_statutes.len(), 1);
        assert_eq!(diff.modified_statutes[0].id, "stat1");
    }

    #[test]
    fn test_has_changes() {
        let doc1 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "Title")],
        };

        let doc2 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "Title")],
        };

        let diff = DocumentDiff::compute(&doc1, &doc2);
        assert!(!diff.has_changes());

        let doc3 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "New Title")],
        };

        let diff2 = DocumentDiff::compute(&doc1, &doc3);
        assert!(diff2.has_changes());
    }

    #[test]
    fn test_diff_report() {
        let doc1 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![create_test_statute("stat1", "Old")],
        };

        let doc2 = LegalDocument {
            imports: Vec::new(),
            statutes: vec![
                create_test_statute("stat1", "New"),
                create_test_statute("stat2", "Added"),
            ],
        };

        let diff = DocumentDiff::compute(&doc1, &doc2);
        let report = diff.report();

        assert!(report.contains("Added Statutes:"));
        assert!(report.contains("+ stat2"));
        assert!(report.contains("Modified Statutes:"));
        assert!(report.contains("~ stat1"));
    }
}
