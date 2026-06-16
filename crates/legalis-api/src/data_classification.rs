//! Data classification: sensitivity levels, tagging, and field-level handling.
//!
//! Provides a classification taxonomy (Public → Restricted), per-field/resource
//! classification tags, and a registry that maps logical field paths to their
//! classification. Higher levels imply stricter handling (encryption-at-rest,
//! redaction in logs, export restrictions). The module can redact a JSON
//! response by masking fields at or above a configured sensitivity threshold —
//! useful for emitting lower-sensitivity views of a resource.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Data sensitivity classification levels, ordered from least to most sensitive.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum Classification {
    /// Freely shareable, no restrictions.
    #[default]
    Public,
    /// Internal use; not for external distribution.
    Internal,
    /// Confidential business data.
    Confidential,
    /// Personally identifiable information.
    Pii,
    /// Highly restricted (legal hold, special category data).
    Restricted,
}

impl Classification {
    /// Returns the canonical string label.
    pub fn label(&self) -> &'static str {
        match self {
            Classification::Public => "public",
            Classification::Internal => "internal",
            Classification::Confidential => "confidential",
            Classification::Pii => "pii",
            Classification::Restricted => "restricted",
        }
    }

    /// Whether data at this level must be encrypted at rest.
    pub fn requires_encryption_at_rest(&self) -> bool {
        *self >= Classification::Confidential
    }

    /// Whether data at this level must be redacted from application logs.
    pub fn requires_log_redaction(&self) -> bool {
        *self >= Classification::Confidential
    }

    /// Whether data at this level may be included in unrestricted exports.
    pub fn is_exportable_unrestricted(&self) -> bool {
        *self <= Classification::Internal
    }

    /// Parses a classification from a label (case-insensitive).
    pub fn from_label(label: &str) -> Option<Self> {
        match label.to_ascii_lowercase().as_str() {
            "public" => Some(Classification::Public),
            "internal" => Some(Classification::Internal),
            "confidential" => Some(Classification::Confidential),
            "pii" => Some(Classification::Pii),
            "restricted" => Some(Classification::Restricted),
            _ => None,
        }
    }
}

/// A classification tag attached to a logical field path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationTag {
    /// Dotted field path the tag applies to (e.g. `"author.email"`).
    pub field_path: String,
    /// The assigned classification level.
    pub classification: Classification,
    /// Optional human-readable note (e.g. regulation reference).
    pub note: Option<String>,
}

impl ClassificationTag {
    /// Creates a tag.
    pub fn new(field_path: impl Into<String>, classification: Classification) -> Self {
        Self {
            field_path: field_path.into(),
            classification,
            note: None,
        }
    }

    /// Adds a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// A registry mapping field paths to classifications.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClassificationRegistry {
    tags: BTreeMap<String, Classification>,
}

impl ClassificationRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers (or overwrites) a classification for a field path.
    pub fn classify(&mut self, field_path: impl Into<String>, level: Classification) -> &mut Self {
        self.tags.insert(field_path.into(), level);
        self
    }

    /// Registers a tag.
    pub fn add_tag(&mut self, tag: ClassificationTag) -> &mut Self {
        self.tags.insert(tag.field_path, tag.classification);
        self
    }

    /// Returns the classification for an exact field path, if present.
    pub fn get(&self, field_path: &str) -> Option<Classification> {
        self.tags.get(field_path).copied()
    }

    /// Returns the effective classification of a field path, considering both an
    /// exact match and any ancestor prefix match (the most sensitive wins).
    ///
    /// For example, if `author` is `Pii`, then `author.email` inherits `Pii`
    /// even without its own tag.
    pub fn effective(&self, field_path: &str) -> Classification {
        let mut max = Classification::Public;
        for (path, level) in &self.tags {
            let matches = field_path == path
                || field_path.starts_with(&format!("{path}."))
                || path.starts_with(&format!("{field_path}."));
            // Only ancestor/exact matches propagate downward; descendant tags
            // bubble up to the most sensitive observed for the subtree.
            let ancestor_or_exact =
                field_path == path || field_path.starts_with(&format!("{path}."));
            if matches && ancestor_or_exact && *level > max {
                max = *level;
            }
        }
        max
    }

    /// Returns the highest classification across all registered fields.
    pub fn max_classification(&self) -> Classification {
        self.tags
            .values()
            .copied()
            .max()
            .unwrap_or(Classification::Public)
    }

    /// Number of registered tags.
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Redacts a JSON value, masking any field at or above `threshold`.
    ///
    /// Masked scalar fields become the string `"[REDACTED]"`. Nested objects are
    /// traversed using dotted paths so that, e.g., `author.email` is matched.
    pub fn redact(
        &self,
        value: &serde_json::Value,
        threshold: Classification,
    ) -> serde_json::Value {
        self.redact_path(value, "", threshold)
    }

    fn redact_path(
        &self,
        value: &serde_json::Value,
        prefix: &str,
        threshold: Classification,
    ) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let mut out = serde_json::Map::new();
                for (key, child) in map {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    if self.effective(&path) >= threshold {
                        out.insert(
                            key.clone(),
                            serde_json::Value::String("[REDACTED]".to_string()),
                        );
                    } else {
                        out.insert(key.clone(), self.redact_path(child, &path, threshold));
                    }
                }
                serde_json::Value::Object(out)
            }
            serde_json::Value::Array(arr) => serde_json::Value::Array(
                arr.iter()
                    .map(|item| self.redact_path(item, prefix, threshold))
                    .collect(),
            ),
            other => other.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_ordering() {
        assert!(Classification::Public < Classification::Internal);
        assert!(Classification::Confidential < Classification::Pii);
        assert!(Classification::Pii < Classification::Restricted);
    }

    #[test]
    fn test_handling_rules() {
        assert!(!Classification::Public.requires_encryption_at_rest());
        assert!(Classification::Confidential.requires_encryption_at_rest());
        assert!(Classification::Pii.requires_log_redaction());
        assert!(Classification::Internal.is_exportable_unrestricted());
        assert!(!Classification::Pii.is_exportable_unrestricted());
    }

    #[test]
    fn test_label_roundtrip() {
        for level in [
            Classification::Public,
            Classification::Internal,
            Classification::Confidential,
            Classification::Pii,
            Classification::Restricted,
        ] {
            assert_eq!(Classification::from_label(level.label()), Some(level));
        }
        assert_eq!(Classification::from_label("bogus"), None);
    }

    #[test]
    fn test_registry_exact_get() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("email", Classification::Pii);
        assert_eq!(reg.get("email"), Some(Classification::Pii));
        assert_eq!(reg.get("name"), None);
    }

    #[test]
    fn test_effective_inherits_from_ancestor() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("author", Classification::Pii);
        // Child inherits ancestor classification.
        assert_eq!(reg.effective("author.email"), Classification::Pii);
        // Exact match.
        assert_eq!(reg.effective("author"), Classification::Pii);
        // Unrelated path defaults to Public.
        assert_eq!(reg.effective("title"), Classification::Public);
    }

    #[test]
    fn test_effective_most_sensitive_wins() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("a", Classification::Internal);
        reg.classify("a.b", Classification::Restricted);
        assert_eq!(reg.effective("a.b"), Classification::Restricted);
    }

    #[test]
    fn test_max_classification() {
        let mut reg = ClassificationRegistry::new();
        assert_eq!(reg.max_classification(), Classification::Public);
        reg.classify("x", Classification::Internal);
        reg.classify("y", Classification::Pii);
        assert_eq!(reg.max_classification(), Classification::Pii);
    }

    #[test]
    fn test_add_tag_with_note() {
        let mut reg = ClassificationRegistry::new();
        let tag = ClassificationTag::new("ssn", Classification::Restricted)
            .with_note("GDPR special category");
        assert_eq!(tag.note.as_deref(), Some("GDPR special category"));
        reg.add_tag(tag);
        assert_eq!(reg.get("ssn"), Some(Classification::Restricted));
    }

    #[test]
    fn test_redact_scalar_field() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("email", Classification::Pii);
        let value = json!({"name": "Alice", "email": "alice@x.com"});
        let out = reg.redact(&value, Classification::Pii);
        assert_eq!(out, json!({"name": "Alice", "email": "[REDACTED]"}));
    }

    #[test]
    fn test_redact_nested() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("author.email", Classification::Pii);
        let value = json!({
            "title": "T",
            "author": {"name": "Bob", "email": "bob@x.com"}
        });
        let out = reg.redact(&value, Classification::Pii);
        assert_eq!(
            out,
            json!({"title": "T", "author": {"name": "Bob", "email": "[REDACTED]"}})
        );
    }

    #[test]
    fn test_redact_subtree_via_ancestor_tag() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("author", Classification::Pii);
        let value = json!({"author": {"name": "Bob", "email": "bob@x.com"}});
        let out = reg.redact(&value, Classification::Pii);
        // The entire `author` object is redacted because the object's own path
        // is classified Pii.
        assert_eq!(out, json!({"author": "[REDACTED]"}));
    }

    #[test]
    fn test_redact_threshold_respected() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("email", Classification::Internal);
        let value = json!({"email": "x@y.com"});
        // Threshold is Pii; Internal is below it so nothing is redacted.
        let out = reg.redact(&value, Classification::Pii);
        assert_eq!(out, json!({"email": "x@y.com"}));
    }

    #[test]
    fn test_redact_array() {
        let mut reg = ClassificationRegistry::new();
        reg.classify("email", Classification::Pii);
        let value = json!([{"email": "a@x"}, {"email": "b@x"}]);
        let out = reg.redact(&value, Classification::Pii);
        assert_eq!(
            out,
            json!([{"email": "[REDACTED]"}, {"email": "[REDACTED]"}])
        );
    }
}
