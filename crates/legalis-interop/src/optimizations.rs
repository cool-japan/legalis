//! Performance optimizations for format conversions.
//!
//! This module provides optimized parsers and utilities for improving
//! conversion performance across different legal DSL formats.

use std::collections::HashMap;
use std::sync::Arc;

/// String interner for reducing memory usage by deduplicating common strings.
///
/// Legal documents often contain repeated identifiers, keywords, and phrases.
/// The interner stores each unique string once and returns references to avoid
/// duplicating memory.
pub struct StringInterner {
    strings: HashMap<String, Arc<str>>,
}

impl StringInterner {
    /// Creates a new string interner.
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }

    /// Interns a string, returning a reference to the shared storage.
    ///
    /// If the string already exists in the interner, returns the existing reference.
    /// Otherwise, stores the string and returns a new reference.
    pub fn intern(&mut self, s: &str) -> Arc<str> {
        if let Some(existing) = self.strings.get(s) {
            Arc::clone(existing)
        } else {
            let arc: Arc<str> = Arc::from(s);
            self.strings.insert(s.to_string(), Arc::clone(&arc));
            arc
        }
    }

    /// Returns the number of unique strings stored.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns true if the interner is empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Clears all interned strings.
    pub fn clear(&mut self) {
        self.strings.clear();
    }

    /// Returns the estimated memory usage in bytes.
    pub fn memory_usage(&self) -> usize {
        self.strings
            .iter()
            .map(|(k, v)| k.len() + v.len() + std::mem::size_of::<Arc<str>>())
            .sum()
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-compiled regex patterns for common parsing operations.
///
/// Compiling regex patterns is expensive. This struct pre-compiles
/// commonly used patterns to avoid repeated compilation overhead.
#[allow(dead_code)]
pub struct RegexCache {
    // Catala patterns
    scope_declaration: regex_lite::Regex,
    definition: regex_lite::Regex,

    // L4 patterns
    rule_pattern: regex_lite::Regex,
    deontic_pattern: regex_lite::Regex,

    // Stipula patterns
    agreement_pattern: regex_lite::Regex,

    // Common patterns
    age_condition: regex_lite::Regex,
    comparison: regex_lite::Regex,
    identifier: regex_lite::Regex,
}

impl RegexCache {
    /// Creates a new regex cache with pre-compiled patterns.
    pub fn new() -> Self {
        Self {
            // Catala patterns
            scope_declaration: regex_lite::Regex::new(r"declaration\s+scope\s+(\w+)").unwrap(),
            definition: regex_lite::Regex::new(r"definition\s+(\w+(?:\.\w+)*)\s+equals").unwrap(),

            // L4 patterns
            rule_pattern: regex_lite::Regex::new(r"RULE\s+(\w+)").unwrap(),
            deontic_pattern: regex_lite::Regex::new(r"\b(MUST|MAY|SHANT|SHALL|SHOULD)\b").unwrap(),

            // Stipula patterns
            agreement_pattern: regex_lite::Regex::new(r"agreement\s+(\w+)\s*\(").unwrap(),

            // Common patterns
            age_condition: regex_lite::Regex::new(r"age\s*(>=|<=|>|<|==|!=)\s*(\d+)").unwrap(),
            comparison: regex_lite::Regex::new(
                r"(\w+(?:\.\w+)*)\s*(>=|<=|>|<|==|!=)\s*(\d+|true|false|\w+)",
            )
            .unwrap(),
            identifier: regex_lite::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap(),
        }
    }

    /// Finds Catala scope declarations in text.
    pub fn find_catala_scopes<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.scope_declaration
            .find_iter(text)
            .filter_map(|m| m.as_str().split_whitespace().last())
            .collect()
    }

    /// Finds L4 rule names in text.
    pub fn find_l4_rules<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.rule_pattern
            .find_iter(text)
            .filter_map(|m| m.as_str().split_whitespace().nth(1))
            .collect()
    }

    /// Finds Stipula agreement names in text.
    pub fn find_stipula_agreements<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.agreement_pattern
            .find_iter(text)
            .filter_map(|m| {
                let text = m.as_str();
                text.split_whitespace().nth(1)?.split('(').next()
            })
            .collect()
    }

    /// Extracts age conditions from text.
    pub fn extract_age_conditions(&self, text: &str) -> Vec<(String, u32)> {
        self.age_condition
            .captures_iter(text)
            .filter_map(|cap| {
                let op = cap.get(1)?.as_str().to_string();
                let value = cap.get(2)?.as_str().parse::<u32>().ok()?;
                Some((op, value))
            })
            .collect()
    }

    /// Checks if a string is a valid identifier.
    pub fn is_valid_identifier(&self, s: &str) -> bool {
        self.identifier.is_match(s)
    }

    /// Finds deontic modalities in L4 text.
    pub fn find_deontic_modalities<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.deontic_pattern
            .find_iter(text)
            .map(|m| m.as_str())
            .collect()
    }
}

impl Default for RegexCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Whitespace normalization utilities.
pub struct WhitespaceNormalizer;

impl WhitespaceNormalizer {
    /// Normalizes whitespace in legal text while preserving structure.
    ///
    /// - Converts all whitespace to single spaces
    /// - Removes leading/trailing whitespace
    /// - Preserves paragraph breaks (double newlines)
    pub fn normalize(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut prev_was_space = false;
        let mut newline_count = 0;

        for ch in text.chars() {
            match ch {
                '\n' | '\r' => {
                    newline_count += 1;
                    if newline_count >= 2 {
                        if !result.is_empty() && !result.ends_with("\n\n") {
                            // Remove trailing space before adding double newline
                            if result.ends_with(' ') {
                                result.pop();
                            }
                            result.push_str("\n\n");
                        }
                        prev_was_space = true;
                    } else if !prev_was_space {
                        result.push(' ');
                        prev_was_space = true;
                    }
                }
                ' ' | '\t' => {
                    if !prev_was_space {
                        result.push(' ');
                        prev_was_space = true;
                    }
                    newline_count = 0;
                }
                _ => {
                    result.push(ch);
                    prev_was_space = false;
                    newline_count = 0;
                }
            }
        }

        result.trim().to_string()
    }

    /// Removes all whitespace from text.
    pub fn strip(text: &str) -> String {
        text.chars().filter(|c| !c.is_whitespace()).collect()
    }

    /// Normalizes indentation to use consistent spacing.
    pub fn normalize_indentation(text: &str, spaces_per_indent: usize) -> String {
        let mut result = String::with_capacity(text.len());

        for line in text.lines() {
            let trimmed = line.trim_start();
            let indent_level = (line.len() - trimmed.len()) / 2; // Assume 2-space indent
            let new_indent = " ".repeat(indent_level * spaces_per_indent);
            result.push_str(&new_indent);
            result.push_str(trimmed);
            result.push('\n');
        }

        result
    }
}

/// Optimized identifier normalization.
pub struct IdentifierNormalizer;

impl IdentifierNormalizer {
    /// Normalizes an identifier to a standard format (lowercase, snake_case).
    pub fn normalize(identifier: &str) -> String {
        identifier
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>()
            .to_lowercase()
    }

    /// Converts CamelCase to snake_case.
    pub fn camel_to_snake(identifier: &str) -> String {
        let mut result = String::with_capacity(identifier.len() + 5);

        for (i, ch) in identifier.chars().enumerate() {
            if ch.is_uppercase() {
                // Add underscore before uppercase if not at start
                if i > 0 {
                    result.push('_');
                }
                result.push(ch.to_ascii_lowercase());
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Converts snake_case to CamelCase.
    pub fn snake_to_camel(identifier: &str) -> String {
        let mut result = String::with_capacity(identifier.len());
        let mut capitalize_next = true;

        for ch in identifier.chars() {
            if ch == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(ch);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();

        let s1 = interner.intern("age");
        let s2 = interner.intern("age");
        let s3 = interner.intern("income");

        // Same string should return same Arc
        assert!(Arc::ptr_eq(&s1, &s2));
        assert!(!Arc::ptr_eq(&s1, &s3));

        assert_eq!(interner.len(), 2);
        assert!(!interner.is_empty());
    }

    #[test]
    fn test_string_interner_memory_usage() {
        let mut interner = StringInterner::new();

        interner.intern("test");
        interner.intern("example");

        assert!(interner.memory_usage() > 0);
    }

    #[test]
    fn test_regex_cache_catala() {
        let cache = RegexCache::new();

        let text = r#"
declaration scope VotingRights:
  context input content Input
declaration scope TaxBenefit:
  context output content Output
"#;

        let scopes = cache.find_catala_scopes(text);
        assert_eq!(scopes, vec!["VotingRights", "TaxBenefit"]);
    }

    #[test]
    fn test_regex_cache_l4() {
        let cache = RegexCache::new();

        let text = r#"
RULE VotingAge WHEN age >= 18 THEN Person MAY vote
RULE DrivingAge WHEN age >= 16 THEN Person MAY drive
"#;

        let rules = cache.find_l4_rules(text);
        assert_eq!(rules, vec!["VotingAge", "DrivingAge"]);
    }

    #[test]
    fn test_regex_cache_stipula() {
        let cache = RegexCache::new();

        let text = r#"
agreement RentalContract(Landlord, Tenant) { }
agreement ServiceAgreement(Provider, Client) { }
"#;

        let agreements = cache.find_stipula_agreements(text);
        assert_eq!(agreements, vec!["RentalContract", "ServiceAgreement"]);
    }

    #[test]
    fn test_regex_cache_age_conditions() {
        let cache = RegexCache::new();

        let text = "age >= 18 and age <= 65";
        let conditions = cache.extract_age_conditions(text);

        assert_eq!(conditions.len(), 2);
        assert_eq!(conditions[0], (">=".to_string(), 18));
        assert_eq!(conditions[1], ("<=".to_string(), 65));
    }

    #[test]
    fn test_regex_cache_valid_identifier() {
        let cache = RegexCache::new();

        assert!(cache.is_valid_identifier("valid_id"));
        assert!(cache.is_valid_identifier("_private"));
        assert!(cache.is_valid_identifier("CamelCase"));
        assert!(!cache.is_valid_identifier("123invalid"));
        assert!(!cache.is_valid_identifier("invalid-id"));
    }

    #[test]
    fn test_regex_cache_deontic() {
        let cache = RegexCache::new();

        let text = "Person MUST pay taxes and MAY vote but SHANT evade";
        let modalities = cache.find_deontic_modalities(text);

        assert_eq!(modalities, vec!["MUST", "MAY", "SHANT"]);
    }

    #[test]
    fn test_whitespace_normalizer() {
        let text = "  hello   world  \n\n  foo   bar  ";
        let normalized = WhitespaceNormalizer::normalize(text);

        assert_eq!(normalized, "hello world\n\nfoo bar");
    }

    #[test]
    fn test_whitespace_strip() {
        let text = "  hello   world  ";
        let stripped = WhitespaceNormalizer::strip(text);

        assert_eq!(stripped, "helloworld");
    }

    #[test]
    fn test_whitespace_normalize_indentation() {
        let text = "  line1\n    line2\n  line3";
        let normalized = WhitespaceNormalizer::normalize_indentation(text, 4);

        assert!(normalized.contains("    line1"));
        assert!(normalized.contains("        line2"));
    }

    #[test]
    fn test_identifier_normalize() {
        assert_eq!(IdentifierNormalizer::normalize("Test-ID_123"), "testid_123");
        assert_eq!(IdentifierNormalizer::normalize("CamelCase"), "camelcase");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(
            IdentifierNormalizer::camel_to_snake("VotingRights"),
            "voting_rights"
        );
        assert_eq!(
            IdentifierNormalizer::camel_to_snake("XMLParser"),
            "x_m_l_parser"
        );
    }

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(
            IdentifierNormalizer::snake_to_camel("voting_rights"),
            "VotingRights"
        );
        assert_eq!(
            IdentifierNormalizer::snake_to_camel("tax_benefit"),
            "TaxBenefit"
        );
    }

    #[test]
    fn test_string_interner_clear() {
        let mut interner = StringInterner::new();
        interner.intern("test");
        assert_eq!(interner.len(), 1);

        interner.clear();
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }

    #[test]
    fn test_string_interner_default() {
        let interner = StringInterner::default();
        assert!(interner.is_empty());
    }

    #[test]
    fn test_regex_cache_default() {
        let cache = RegexCache::default();
        assert!(cache.is_valid_identifier("test"));
    }
}
