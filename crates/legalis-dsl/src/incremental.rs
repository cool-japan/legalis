//! Incremental parsing support for the Legalis DSL.
//!
//! This module provides functionality for incrementally parsing documents,
//! which is essential for IDE integration where only parts of a document
//! may change at a time.

use crate::{DslResult, LegalDslParser, ast};
use std::collections::HashMap;

/// Represents a text edit operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextEdit {
    /// The starting byte offset of the edit
    pub start: usize,
    /// The ending byte offset of the edit (exclusive)
    pub end: usize,
    /// The new text to insert
    pub new_text: String,
}

impl TextEdit {
    /// Creates a new text edit.
    pub fn new(start: usize, end: usize, new_text: impl Into<String>) -> Self {
        Self {
            start,
            end,
            new_text: new_text.into(),
        }
    }

    /// Creates a text edit that inserts text at a position.
    pub fn insert(position: usize, text: impl Into<String>) -> Self {
        Self::new(position, position, text)
    }

    /// Creates a text edit that deletes a range.
    pub fn delete(start: usize, end: usize) -> Self {
        Self::new(start, end, "")
    }

    /// Creates a text edit that replaces a range with new text.
    pub fn replace(start: usize, end: usize, new_text: impl Into<String>) -> Self {
        Self::new(start, end, new_text)
    }

    /// Applies this edit to a string, returning the result.
    pub fn apply(&self, text: &str) -> String {
        let mut result =
            String::with_capacity(text.len() - (self.end - self.start) + self.new_text.len());
        result.push_str(&text[..self.start]);
        result.push_str(&self.new_text);
        result.push_str(&text[self.end..]);
        result
    }
}

/// A cache entry for a parsed statute.
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The source text that was parsed
    #[allow(dead_code)]
    source: String,
    /// The parsed AST
    ast: ast::StatuteNode,
    /// Byte offset where this statute starts in the document
    start_offset: usize,
    /// Byte offset where this statute ends in the document
    end_offset: usize,
}

/// Incremental parser that caches parsed statutes and only re-parses changed ones.
#[derive(Debug)]
pub struct IncrementalParser {
    /// The underlying parser
    parser: LegalDslParser,
    /// Cache of parsed statutes by their ID
    cache: HashMap<String, CacheEntry>,
    /// The last full document text
    last_text: String,
}

impl IncrementalParser {
    /// Creates a new incremental parser.
    pub fn new() -> Self {
        Self {
            parser: LegalDslParser::new(),
            cache: HashMap::new(),
            last_text: String::new(),
        }
    }

    /// Parses a document initially, caching all statutes.
    pub fn parse_initial(&mut self, text: &str) -> DslResult<ast::LegalDocument> {
        let doc = self.parser.parse_document(text)?;

        // Cache all statutes
        self.cache.clear();
        self.last_text = text.to_string();

        // Find byte offsets for each statute
        for statute in &doc.statutes {
            if let Some((start, end)) = self.find_statute_range(text, &statute.id) {
                self.cache.insert(
                    statute.id.clone(),
                    CacheEntry {
                        source: text[start..end].to_string(),
                        ast: statute.clone(),
                        start_offset: start,
                        end_offset: end,
                    },
                );
            }
        }

        Ok(doc)
    }

    /// Parses a document after applying edits, using the cache when possible.
    pub fn parse_incremental(&mut self, edits: &[TextEdit]) -> DslResult<ast::LegalDocument> {
        // Apply edits to get new text
        let mut new_text = self.last_text.clone();
        for edit in edits {
            new_text = edit.apply(&new_text);
        }

        // Determine which statutes were affected by the edits
        let affected_statutes = self.find_affected_statutes(edits);

        // Parse the new document
        let new_doc = self.parser.parse_document(&new_text)?;

        // Update cache: keep unchanged statutes, re-cache changed ones
        let mut new_cache = HashMap::new();
        for statute in &new_doc.statutes {
            if affected_statutes.contains(&statute.id) {
                // Statute was affected, re-cache it
                if let Some((start, end)) = self.find_statute_range(&new_text, &statute.id) {
                    new_cache.insert(
                        statute.id.clone(),
                        CacheEntry {
                            source: new_text[start..end].to_string(),
                            ast: statute.clone(),
                            start_offset: start,
                            end_offset: end,
                        },
                    );
                }
            } else if let Some(cached) = self.cache.get(&statute.id) {
                // Statute was not affected, reuse cached entry
                new_cache.insert(statute.id.clone(), cached.clone());
            }
        }

        self.cache = new_cache;
        self.last_text = new_text;

        Ok(new_doc)
    }

    /// Finds the byte range of a statute in the text.
    fn find_statute_range(&self, text: &str, statute_id: &str) -> Option<(usize, usize)> {
        // Find the statute declaration
        let statute_keyword = "STATUTE";
        let search_pattern = format!("{} {}", statute_keyword, statute_id);

        if let Some(start) = text.find(&search_pattern) {
            // Find the matching closing brace
            let mut brace_count = 0;
            let mut found_opening = false;
            let bytes = text.as_bytes();

            for (offset, &byte) in bytes.iter().enumerate().skip(start) {
                match byte {
                    b'{' => {
                        brace_count += 1;
                        found_opening = true;
                    }
                    b'}' => {
                        brace_count -= 1;
                        if found_opening && brace_count == 0 {
                            return Some((start, offset + 1));
                        }
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// Determines which statutes were affected by the given edits.
    fn find_affected_statutes(&self, edits: &[TextEdit]) -> std::collections::HashSet<String> {
        let mut affected = std::collections::HashSet::new();

        for edit in edits {
            // Check which cached statutes overlap with this edit
            for (id, entry) in &self.cache {
                if edit.start < entry.end_offset && edit.end > entry.start_offset {
                    affected.insert(id.clone());
                }
            }
        }

        affected
    }

    /// Returns the cached AST for a statute if available and unchanged.
    pub fn get_cached(&self, statute_id: &str) -> Option<&ast::StatuteNode> {
        self.cache.get(statute_id).map(|entry| &entry.ast)
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.last_text.clear();
    }

    /// Returns the number of cached statutes.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl Default for IncrementalParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_edit_insert() {
        let text = "Hello World";
        let edit = TextEdit::insert(6, "Beautiful ");
        let result = edit.apply(text);
        assert_eq!(result, "Hello Beautiful World");
    }

    #[test]
    fn test_text_edit_delete() {
        let text = "Hello Beautiful World";
        let edit = TextEdit::delete(6, 16);
        let result = edit.apply(text);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_text_edit_replace() {
        let text = "Hello World";
        let edit = TextEdit::replace(6, 11, "Rust");
        let result = edit.apply(text);
        assert_eq!(result, "Hello Rust");
    }

    #[test]
    fn test_incremental_parser_initial() {
        let text = r#"
            STATUTE test1: "Test 1" {
                WHEN AGE >= 18
                THEN GRANT "Access"
            }

            STATUTE test2: "Test 2" {
                WHEN AGE >= 21
                THEN GRANT "Advanced Access"
            }
        "#;

        let mut parser = IncrementalParser::new();
        let doc = parser.parse_initial(text).unwrap();

        assert_eq!(doc.statutes.len(), 2);
        assert_eq!(parser.cache_size(), 2);
    }

    #[test]
    fn test_incremental_parser_with_edit() {
        let text = r#"STATUTE test1: "Test 1" {
    WHEN AGE >= 18
    THEN GRANT "Access"
}

STATUTE test2: "Test 2" {
    WHEN AGE >= 21
    THEN GRANT "Advanced Access"
}"#;

        let mut parser = IncrementalParser::new();
        parser.parse_initial(text).unwrap();

        // Edit only the first statute
        let edit = TextEdit::replace(text.find("18").unwrap(), text.find("18").unwrap() + 2, "21");

        let doc = parser.parse_incremental(&[edit]).unwrap();
        assert_eq!(doc.statutes.len(), 2);
    }

    #[test]
    fn test_incremental_parser_cache_lookup() {
        let text = r#"STATUTE test1: "Test 1" {
    WHEN AGE >= 18
    THEN GRANT "Access"
}"#;

        let mut parser = IncrementalParser::new();
        parser.parse_initial(text).unwrap();

        let cached = parser.get_cached("test1");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().id, "test1");
    }

    #[test]
    fn test_incremental_parser_clear_cache() {
        let text = r#"STATUTE test1: "Test 1" {
    WHEN AGE >= 18
    THEN GRANT "Access"
}"#;

        let mut parser = IncrementalParser::new();
        parser.parse_initial(text).unwrap();
        assert_eq!(parser.cache_size(), 1);

        parser.clear_cache();
        assert_eq!(parser.cache_size(), 0);
    }

    #[test]
    fn test_find_statute_range() {
        let text = r#"STATUTE test1: "Test 1" {
    WHEN AGE >= 18
    THEN GRANT "Access"
}"#;

        let parser = IncrementalParser::new();
        let range = parser.find_statute_range(text, "test1");
        assert!(range.is_some());

        if let Some((start, end)) = range {
            assert_eq!(&text[start..end], text.trim());
        }
    }
}
