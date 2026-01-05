//! Field selection support for REST API.
//!
//! Allows clients to specify which fields they want in the response,
//! similar to GraphQL field selection.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Query parameter for field selection.
#[derive(Debug, Clone, Deserialize)]
pub struct FieldsQuery {
    /// Comma-separated list of fields to include
    pub fields: Option<String>,
}

impl FieldsQuery {
    /// Parses the fields query into a set of field names.
    pub fn parse(&self) -> Option<HashSet<String>> {
        self.fields.as_ref().map(|f| {
            f.split(',')
                .map(|s| s.trim().to_string())
                .collect::<HashSet<_>>()
        })
    }

    /// Checks if a field should be included.
    pub fn should_include(&self, field: &str) -> bool {
        match self.parse() {
            None => true, // Include all fields if no selection specified
            Some(fields) => fields.contains(field),
        }
    }
}

/// Trait for types that support field selection.
pub trait FieldSelectable {
    /// Applies field selection to produce a filtered JSON value.
    fn apply_fields(&self, fields: &Option<HashSet<String>>) -> serde_json::Value;
}

/// Macro to help implement field selection for structs.
#[macro_export]
macro_rules! impl_field_selectable {
    ($type:ty, { $($field:ident),* $(,)? }) => {
        impl $crate::field_selection::FieldSelectable for $type {
            fn apply_fields(&self, fields: &Option<HashSet<String>>) -> serde_json::Value {
                let mut map = serde_json::Map::new();

                $(
                    if fields.as_ref().map_or(true, |f| f.contains(stringify!($field))) {
                        map.insert(
                            stringify!($field).to_string(),
                            serde_json::to_value(&self.$field).unwrap_or(serde_json::Value::Null),
                        );
                    }
                )*

                serde_json::Value::Object(map)
            }
        }
    };
}

/// Helper to apply field selection to a serializable value.
pub fn apply_field_selection<T: Serialize>(
    value: &T,
    fields: &Option<HashSet<String>>,
) -> serde_json::Value {
    let full_value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);

    match fields {
        None => full_value,
        Some(field_set) => filter_json_fields(&full_value, field_set),
    }
}

/// Filters a JSON value to include only specified fields.
fn filter_json_fields(value: &serde_json::Value, fields: &HashSet<String>) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut filtered = serde_json::Map::new();
            for (key, val) in map {
                if fields.contains(key) {
                    filtered.insert(key.clone(), val.clone());
                }
            }
            serde_json::Value::Object(filtered)
        }
        serde_json::Value::Array(arr) => {
            let filtered: Vec<_> = arr
                .iter()
                .map(|item| filter_json_fields(item, fields))
                .collect();
            serde_json::Value::Array(filtered)
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct TestStruct {
        id: String,
        name: String,
        age: u32,
        email: String,
    }

    #[test]
    fn test_fields_query_parse() {
        let query = FieldsQuery {
            fields: Some("id,name,age".to_string()),
        };
        let fields = query.parse().unwrap();
        assert!(fields.contains("id"));
        assert!(fields.contains("name"));
        assert!(fields.contains("age"));
        assert!(!fields.contains("email"));
    }

    #[test]
    fn test_fields_query_should_include() {
        let query = FieldsQuery {
            fields: Some("id,name".to_string()),
        };
        assert!(query.should_include("id"));
        assert!(query.should_include("name"));
        assert!(!query.should_include("age"));
        assert!(!query.should_include("email"));
    }

    #[test]
    fn test_fields_query_no_selection() {
        let query = FieldsQuery { fields: None };
        assert!(query.should_include("id"));
        assert!(query.should_include("name"));
        assert!(query.should_include("age"));
        assert!(query.should_include("email"));
    }

    #[test]
    fn test_apply_field_selection() {
        let test_data = TestStruct {
            id: "123".to_string(),
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
        };

        let mut fields = HashSet::new();
        fields.insert("id".to_string());
        fields.insert("name".to_string());

        let result = apply_field_selection(&test_data, &Some(fields));

        assert!(result.get("id").is_some());
        assert!(result.get("name").is_some());
        assert!(result.get("age").is_none());
        assert!(result.get("email").is_none());
    }

    #[test]
    fn test_apply_field_selection_no_filter() {
        let test_data = TestStruct {
            id: "123".to_string(),
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
        };

        let result = apply_field_selection(&test_data, &None);

        assert!(result.get("id").is_some());
        assert!(result.get("name").is_some());
        assert!(result.get("age").is_some());
        assert!(result.get("email").is_some());
    }

    #[test]
    fn test_filter_json_fields_array() {
        let data = serde_json::json!([
            {"id": "1", "name": "Alice", "age": 30},
            {"id": "2", "name": "Bob", "age": 25}
        ]);

        let mut fields = HashSet::new();
        fields.insert("id".to_string());
        fields.insert("name".to_string());

        let result = filter_json_fields(&data, &fields);

        if let serde_json::Value::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            for item in arr {
                assert!(item.get("id").is_some());
                assert!(item.get("name").is_some());
                assert!(item.get("age").is_none());
            }
        } else {
            panic!("Expected array");
        }
    }
}
