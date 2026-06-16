//! Partial responses (sparse fieldsets) with nested field path support.
//!
//! Extends the flat field selection in [`crate::field_selection`] with support
//! for nested field paths using dotted notation (`"a.b.c"`) and brace expansion
//! (`"author{name,email}"`). This lets clients request deeply nested subsets of
//! a JSON response, trimming payload size for bandwidth-constrained clients.
//!
//! Example selections:
//! - `"id,title"` — top-level fields only.
//! - `"id,meta.total"` — `id` plus the nested `meta.total`.
//! - `"id,author{name,email}"` — `id` plus a projection of the `author` object.

use serde::Serialize;
use std::collections::BTreeMap;

/// A parsed selection tree of field paths.
///
/// An empty tree (no children) at a node means "include the whole subtree at
/// this node". A node with children means "include only these child paths".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FieldTree {
    children: BTreeMap<String, FieldTree>,
}

impl FieldTree {
    /// Creates an empty field tree (selects everything).
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether this node selects everything below it (a leaf).
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the child sub-tree for a field name, if present.
    pub fn child(&self, name: &str) -> Option<&FieldTree> {
        self.children.get(name)
    }

    /// Number of direct children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Inserts a dotted path into the tree.
    fn insert_path(&mut self, path: &[&str]) {
        if path.is_empty() {
            return;
        }
        let head = path[0];
        let node = self.children.entry(head.to_string()).or_default();
        if path.len() > 1 {
            node.insert_path(&path[1..]);
        }
    }

    /// Parses a selection spec into a field tree.
    ///
    /// Grammar (informal): a comma-separated list of field paths, where each
    /// path is `name` or `name.sub` or `name{a,b.c,d}`. Brace groups expand to
    /// dotted paths rooted at the group's prefix. Whitespace around names is
    /// trimmed.
    pub fn parse(spec: &str) -> Result<FieldTree, PartialResponseError> {
        let mut tree = FieldTree::new();
        let paths = expand_spec(spec)?;
        for path in paths {
            let segments: Vec<&str> = path.split('.').map(|s| s.trim()).collect();
            if segments.iter().any(|s| s.is_empty()) {
                return Err(PartialResponseError::InvalidPath(path.clone()));
            }
            tree.insert_path(&segments);
        }
        Ok(tree)
    }
}

/// Expands a selection spec (with possible brace groups) into a flat list of
/// dotted field paths.
fn expand_spec(spec: &str) -> Result<Vec<String>, PartialResponseError> {
    let mut paths = Vec::new();
    let mut depth = 0i32;

    // We split on top-level commas (depth 0). Brace groups are recursed into.
    let mut buf = String::new();
    for ch in spec.chars() {
        match ch {
            '{' => {
                depth += 1;
                buf.push(ch);
            }
            '}' => {
                depth -= 1;
                if depth < 0 {
                    return Err(PartialResponseError::UnbalancedBraces);
                }
                buf.push(ch);
            }
            ',' if depth == 0 => {
                expand_segment(buf.trim(), &mut paths)?;
                buf.clear();
            }
            _ => buf.push(ch),
        }
    }
    if depth != 0 {
        return Err(PartialResponseError::UnbalancedBraces);
    }
    if !buf.trim().is_empty() {
        expand_segment(buf.trim(), &mut paths)?;
    }
    Ok(paths)
}

/// Expands a single top-level segment which may contain a brace group.
fn expand_segment(segment: &str, out: &mut Vec<String>) -> Result<(), PartialResponseError> {
    if segment.is_empty() {
        return Ok(());
    }
    if let Some(brace_pos) = segment.find('{') {
        if !segment.ends_with('}') {
            return Err(PartialResponseError::UnbalancedBraces);
        }
        let prefix = segment[..brace_pos].trim();
        if prefix.is_empty() {
            return Err(PartialResponseError::InvalidPath(segment.to_string()));
        }
        let inner = &segment[brace_pos + 1..segment.len() - 1];
        let inner_paths = expand_spec(inner)?;
        if inner_paths.is_empty() {
            // `prefix{}` selects the whole prefix subtree.
            out.push(prefix.to_string());
        } else {
            for sub in inner_paths {
                out.push(format!("{prefix}.{sub}"));
            }
        }
    } else {
        out.push(segment.to_string());
    }
    Ok(())
}

/// Errors produced while parsing a partial-response selection.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PartialResponseError {
    /// Braces in the selection spec were not balanced.
    #[error("unbalanced braces in field selection")]
    UnbalancedBraces,
    /// A field path was empty or otherwise malformed.
    #[error("invalid field path: {0}")]
    InvalidPath(String),
}

/// Projects a JSON value according to a field tree.
///
/// - For objects, only keys present in the tree are kept; a child key whose
///   subtree is a leaf keeps the entire value, otherwise the value is projected
///   recursively.
/// - For arrays, the tree is applied element-wise.
/// - Scalars are returned unchanged.
pub fn project(value: &serde_json::Value, tree: &FieldTree) -> serde_json::Value {
    if tree.is_leaf() {
        return value.clone();
    }
    match value {
        serde_json::Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (key, child_tree) in &tree.children {
                if let Some(child_value) = map.get(key) {
                    out.insert(key.clone(), project(child_value, child_tree));
                }
            }
            serde_json::Value::Object(out)
        }
        serde_json::Value::Array(arr) => {
            let projected: Vec<serde_json::Value> =
                arr.iter().map(|item| project(item, tree)).collect();
            serde_json::Value::Array(projected)
        }
        // Selection asks for sub-fields of a scalar: nothing matches.
        other => other.clone(),
    }
}

/// Applies a partial-response selection spec to a serializable value.
///
/// Returns the original value unchanged when `spec` is `None` or empty.
pub fn apply_partial<T: Serialize>(
    value: &T,
    spec: Option<&str>,
) -> Result<serde_json::Value, PartialResponseError> {
    let full = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    match spec {
        None => Ok(full),
        Some(s) if s.trim().is_empty() => Ok(full),
        Some(s) => {
            let tree = FieldTree::parse(s)?;
            if tree.is_leaf() {
                Ok(full)
            } else {
                Ok(project(&full, &tree))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_flat() {
        let tree = FieldTree::parse("id,title,name").expect("parse");
        assert_eq!(tree.child_count(), 3);
        assert!(tree.child("id").is_some());
        assert!(tree.child("title").is_some());
        assert!(tree.child("nope").is_none());
    }

    #[test]
    fn test_parse_dotted() {
        let tree = FieldTree::parse("id,meta.total,meta.page").expect("parse");
        let meta = tree.child("meta").expect("meta");
        assert!(meta.child("total").is_some());
        assert!(meta.child("page").is_some());
        assert_eq!(meta.child_count(), 2);
    }

    #[test]
    fn test_parse_brace_group() {
        let tree = FieldTree::parse("id,author{name,email}").expect("parse");
        let author = tree.child("author").expect("author");
        assert!(author.child("name").is_some());
        assert!(author.child("email").is_some());
        assert_eq!(author.child_count(), 2);
    }

    #[test]
    fn test_parse_nested_brace() {
        let tree = FieldTree::parse("a{b{c,d}}").expect("parse");
        let b = tree.child("a").expect("a").child("b").expect("b");
        assert!(b.child("c").is_some());
        assert!(b.child("d").is_some());
    }

    #[test]
    fn test_parse_unbalanced() {
        assert_eq!(
            FieldTree::parse("a{b,c"),
            Err(PartialResponseError::UnbalancedBraces)
        );
        assert_eq!(
            FieldTree::parse("a}b"),
            Err(PartialResponseError::UnbalancedBraces)
        );
    }

    #[test]
    fn test_parse_invalid_path() {
        assert!(FieldTree::parse("a..b").is_err());
    }

    #[test]
    fn test_project_flat() {
        let value = json!({"id": "1", "title": "T", "secret": "s"});
        let tree = FieldTree::parse("id,title").expect("parse");
        let out = project(&value, &tree);
        assert_eq!(out, json!({"id": "1", "title": "T"}));
    }

    #[test]
    fn test_project_nested() {
        let value = json!({
            "id": "1",
            "meta": {"total": 10, "page": 2, "internal": "x"},
            "extra": "drop"
        });
        let tree = FieldTree::parse("id,meta.total").expect("parse");
        let out = project(&value, &tree);
        assert_eq!(out, json!({"id": "1", "meta": {"total": 10}}));
    }

    #[test]
    fn test_project_array() {
        let value = json!([
            {"id": "1", "name": "a", "x": 1},
            {"id": "2", "name": "b", "x": 2}
        ]);
        let tree = FieldTree::parse("id,name").expect("parse");
        let out = project(&value, &tree);
        assert_eq!(
            out,
            json!([{"id": "1", "name": "a"}, {"id": "2", "name": "b"}])
        );
    }

    #[test]
    fn test_project_array_of_nested() {
        let value = json!({
            "items": [
                {"id": "1", "author": {"name": "a", "email": "a@x"}},
                {"id": "2", "author": {"name": "b", "email": "b@x"}}
            ]
        });
        let tree = FieldTree::parse("items{id,author{name}}").expect("parse");
        let out = project(&value, &tree);
        assert_eq!(
            out,
            json!({"items": [
                {"id": "1", "author": {"name": "a"}},
                {"id": "2", "author": {"name": "b"}}
            ]})
        );
    }

    #[derive(Serialize)]
    struct Sample {
        id: String,
        title: String,
        hidden: u32,
    }

    #[test]
    fn test_apply_partial_some() {
        let s = Sample {
            id: "1".to_string(),
            title: "T".to_string(),
            hidden: 99,
        };
        let out = apply_partial(&s, Some("id,title")).expect("apply");
        assert_eq!(out, json!({"id": "1", "title": "T"}));
    }

    #[test]
    fn test_apply_partial_none_returns_full() {
        let s = Sample {
            id: "1".to_string(),
            title: "T".to_string(),
            hidden: 99,
        };
        let out = apply_partial(&s, None).expect("apply");
        assert_eq!(out, json!({"id": "1", "title": "T", "hidden": 99}));
    }

    #[test]
    fn test_apply_partial_empty_returns_full() {
        let s = Sample {
            id: "1".to_string(),
            title: "T".to_string(),
            hidden: 99,
        };
        let out = apply_partial(&s, Some("   ")).expect("apply");
        assert_eq!(out, json!({"id": "1", "title": "T", "hidden": 99}));
    }

    #[test]
    fn test_project_leaf_keeps_subtree() {
        let value = json!({"meta": {"a": 1, "b": 2}});
        // Selecting "meta" (leaf) keeps the whole object.
        let tree = FieldTree::parse("meta").expect("parse");
        let out = project(&value, &tree);
        assert_eq!(out, json!({"meta": {"a": 1, "b": 2}}));
    }
}
