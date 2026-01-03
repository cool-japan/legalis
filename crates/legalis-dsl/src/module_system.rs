//! Module system support for Legalis DSL (v0.1.4).
//!
//! This module provides namespace, import, export, and visibility features
//! for organizing legal documents into modular structures.

use serde::{Deserialize, Serialize};

/// Import kind for different import styles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ImportKind {
    /// Simple import: IMPORT "path"
    #[default]
    Simple,
    /// Wildcard import: IMPORT path.*
    Wildcard,
    /// Selective import: IMPORT { name1, name2 } FROM path
    Selective(Vec<String>),
}

/// Visibility modifier for statutes and other declarations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum Visibility {
    /// Public (exported, can be imported by other modules)
    Public,
    /// Private (internal to the module, not exported)
    #[default]
    Private,
}

/// AST node for a namespace declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamespaceNode {
    /// The namespace path (e.g., "tax.income.2024")
    pub path: String,
}

/// AST node for an export/re-export declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportNode {
    /// What to export: statute ID or wildcard
    pub items: Vec<String>,
    /// Optional re-export source (for re-exporting from another module)
    pub from: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_default() {
        assert_eq!(Visibility::default(), Visibility::Private);
    }

    #[test]
    fn test_import_kind_default() {
        assert_eq!(ImportKind::default(), ImportKind::Simple);
    }

    #[test]
    fn test_namespace_creation() {
        let ns = NamespaceNode {
            path: "tax.income.2024".to_string(),
        };
        assert_eq!(ns.path, "tax.income.2024");
    }

    #[test]
    fn test_export_node_creation() {
        let export = ExportNode {
            items: vec!["statute1".to_string(), "statute2".to_string()],
            from: Some("other_module".to_string()),
        };
        assert_eq!(export.items.len(), 2);
        assert_eq!(export.from, Some("other_module".to_string()));
    }

    #[test]
    fn test_selective_import() {
        let import = ImportKind::Selective(vec!["credit".to_string(), "deduction".to_string()]);
        match import {
            ImportKind::Selective(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], "credit");
                assert_eq!(items[1], "deduction");
            }
            _ => panic!("Expected Selective import"),
        }
    }
}
