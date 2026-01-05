//! Machine-readable diff formats for external integrations.
//!
//! This module provides support for various machine-readable diff formats:
//! - JSON Patch (RFC 6902)
//! - JSON Merge Patch (RFC 7386)
//! - OpenAPI diff specification
//! - GraphQL schema diff
//! - Protocol Buffers serialization

use crate::{ChangeTarget, ChangeType, DiffError, DiffResult, StatuteDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// JSON Patch operation (RFC 6902).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonPatchOp {
    /// The operation type (add, remove, replace, move, copy, test).
    pub op: String,
    /// The target path.
    pub path: String,
    /// The value (for add, replace, test operations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// The source path (for move and copy operations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

/// JSON Patch document (array of operations).
pub type JsonPatch = Vec<JsonPatchOp>;

/// Converts a StatuteDiff to JSON Patch format (RFC 6902).
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, machine_readable::to_json_patch};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let patch = to_json_patch(&diff_result);
///
/// assert!(!patch.is_empty());
/// ```
pub fn to_json_patch(diff: &StatuteDiff) -> JsonPatch {
    let mut patch = Vec::new();

    for change in &diff.changes {
        match change.change_type {
            ChangeType::Added => {
                if let Some(ref value) = change.new_value {
                    patch.push(JsonPatchOp {
                        op: "add".to_string(),
                        path: format_path(&change.target),
                        value: Some(Value::String(value.clone())),
                        from: None,
                    });
                }
            }
            ChangeType::Removed => {
                patch.push(JsonPatchOp {
                    op: "remove".to_string(),
                    path: format_path(&change.target),
                    value: None,
                    from: None,
                });
            }
            ChangeType::Modified => {
                if let Some(ref value) = change.new_value {
                    patch.push(JsonPatchOp {
                        op: "replace".to_string(),
                        path: format_path(&change.target),
                        value: Some(Value::String(value.clone())),
                        from: None,
                    });
                }
            }
            ChangeType::Reordered => {
                // JSON Patch doesn't have a direct "reorder" operation
                // We represent it as a move operation
                if let (Some(old_val), Some(new_val)) = (&change.old_value, &change.new_value) {
                    patch.push(JsonPatchOp {
                        op: "move".to_string(),
                        path: new_val.clone(),
                        value: None,
                        from: Some(old_val.clone()),
                    });
                }
            }
        }
    }

    patch
}

/// Formats a ChangeTarget as a JSON Pointer path.
fn format_path(target: &ChangeTarget) -> String {
    match target {
        ChangeTarget::Title => "/title".to_string(),
        ChangeTarget::Precondition { index } => format!("/preconditions/{}", index),
        ChangeTarget::Effect => "/effect".to_string(),
        ChangeTarget::DiscretionLogic => "/discretion_logic".to_string(),
        ChangeTarget::Metadata { key } => format!("/metadata/{}", key),
    }
}

/// JSON Merge Patch (RFC 7386) representation.
pub type JsonMergePatch = HashMap<String, Value>;

/// Converts a StatuteDiff to JSON Merge Patch format (RFC 7386).
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, machine_readable::to_json_merge_patch};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let merge_patch = to_json_merge_patch(&diff_result);
///
/// assert!(merge_patch.contains_key("title"));
/// ```
pub fn to_json_merge_patch(diff: &StatuteDiff) -> JsonMergePatch {
    let mut patch = HashMap::new();

    for change in &diff.changes {
        match &change.target {
            ChangeTarget::Title => {
                if let Some(ref value) = change.new_value {
                    patch.insert("title".to_string(), Value::String(value.clone()));
                } else {
                    patch.insert("title".to_string(), Value::Null);
                }
            }
            ChangeTarget::Effect => {
                if let Some(ref value) = change.new_value {
                    patch.insert("effect".to_string(), Value::String(value.clone()));
                } else {
                    patch.insert("effect".to_string(), Value::Null);
                }
            }
            ChangeTarget::DiscretionLogic => {
                if let Some(ref value) = change.new_value {
                    patch.insert("discretion_logic".to_string(), Value::String(value.clone()));
                } else {
                    patch.insert("discretion_logic".to_string(), Value::Null);
                }
            }
            ChangeTarget::Precondition { index } => {
                if let Some(ref value) = change.new_value {
                    patch.insert(
                        format!("precondition_{}", index),
                        Value::String(value.clone()),
                    );
                }
            }
            ChangeTarget::Metadata { key } => {
                if let Some(ref value) = change.new_value {
                    patch.insert(format!("metadata_{}", key), Value::String(value.clone()));
                }
            }
        }
    }

    patch
}

/// OpenAPI diff specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiDiff {
    /// OpenAPI version.
    pub openapi: String,
    /// Information about the diff.
    pub info: OpenApiInfo,
    /// The changes grouped by type.
    pub changes: OpenApiChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiChanges {
    pub added: Vec<OpenApiChange>,
    pub removed: Vec<OpenApiChange>,
    pub modified: Vec<OpenApiChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiChange {
    pub path: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
}

/// Converts a StatuteDiff to OpenAPI diff specification.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, machine_readable::to_openapi_diff};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let openapi = to_openapi_diff(&diff_result, "1.0.0");
///
/// assert_eq!(openapi.openapi, "3.0.0");
/// ```
pub fn to_openapi_diff(diff: &StatuteDiff, version: &str) -> OpenApiDiff {
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    for change in &diff.changes {
        let api_change = OpenApiChange {
            path: format_path(&change.target),
            description: change.description.clone(),
            old_value: change.old_value.clone(),
            new_value: change.new_value.clone(),
        };

        match change.change_type {
            ChangeType::Added => added.push(api_change),
            ChangeType::Removed => removed.push(api_change),
            ChangeType::Modified | ChangeType::Reordered => modified.push(api_change),
        }
    }

    OpenApiDiff {
        openapi: "3.0.0".to_string(),
        info: OpenApiInfo {
            title: format!("Statute Diff: {}", diff.statute_id),
            version: version.to_string(),
            description: format!(
                "Diff showing changes to statute '{}' with {} total changes",
                diff.statute_id,
                diff.changes.len()
            ),
        },
        changes: OpenApiChanges {
            added,
            removed,
            modified,
        },
    }
}

/// GraphQL schema diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlSchemaDiff {
    /// The schema version.
    pub schema_version: String,
    /// Breaking changes.
    pub breaking_changes: Vec<GraphQlChange>,
    /// Non-breaking changes.
    pub non_breaking_changes: Vec<GraphQlChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlChange {
    /// The type of change (field, type, directive, etc.).
    pub change_kind: String,
    /// The path to the change.
    pub path: String,
    /// Description of the change.
    pub description: String,
    /// Criticality level.
    pub criticality: String,
}

/// Converts a StatuteDiff to GraphQL schema diff format.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, machine_readable::to_graphql_schema_diff};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "Revoke");
///
/// let diff_result = diff(&old, &new).unwrap();
/// let graphql_diff = to_graphql_schema_diff(&diff_result);
///
/// assert!(!graphql_diff.breaking_changes.is_empty() || !graphql_diff.non_breaking_changes.is_empty());
/// ```
pub fn to_graphql_schema_diff(diff: &StatuteDiff) -> GraphQlSchemaDiff {
    let mut breaking_changes = Vec::new();
    let mut non_breaking_changes = Vec::new();

    for change in &diff.changes {
        let criticality = match change.change_type {
            ChangeType::Removed => "breaking",
            ChangeType::Modified => {
                if matches!(change.target, ChangeTarget::Effect) {
                    "breaking"
                } else {
                    "non-breaking"
                }
            }
            ChangeType::Added => "non-breaking",
            ChangeType::Reordered => "non-breaking",
        };

        let gql_change = GraphQlChange {
            change_kind: match &change.target {
                ChangeTarget::Title => "field".to_string(),
                ChangeTarget::Precondition { .. } => "field".to_string(),
                ChangeTarget::Effect => "type".to_string(),
                ChangeTarget::DiscretionLogic => "directive".to_string(),
                ChangeTarget::Metadata { .. } => "field".to_string(),
            },
            path: format!("Statute.{}", format_path(&change.target)),
            description: change.description.clone(),
            criticality: criticality.to_string(),
        };

        if criticality == "breaking" {
            breaking_changes.push(gql_change);
        } else {
            non_breaking_changes.push(gql_change);
        }
    }

    GraphQlSchemaDiff {
        schema_version: "1.0.0".to_string(),
        breaking_changes,
        non_breaking_changes,
    }
}

/// Protocol Buffers message for diff serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufDiff {
    /// Statute ID.
    pub statute_id: String,
    /// Version information.
    pub version_info: Option<ProtobufVersionInfo>,
    /// List of changes.
    pub changes: Vec<ProtobufChange>,
    /// Impact assessment.
    pub impact: ProtobufImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufVersionInfo {
    pub old_version: u32,
    pub new_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufChange {
    pub change_type: i32, // Enum: 0=Added, 1=Removed, 2=Modified, 3=Reordered
    pub target_type: i32, // Enum: 0=Title, 1=Precondition, 2=Effect, etc.
    pub target_index: i32,
    pub description: String,
    pub old_value: String,
    pub new_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufImpact {
    pub severity: i32, // Enum: 0=None, 1=Minor, 2=Moderate, 3=Major, 4=Breaking
    pub affects_eligibility: bool,
    pub affects_outcome: bool,
    pub discretion_changed: bool,
    pub notes: Vec<String>,
}

/// Converts a StatuteDiff to Protocol Buffers format.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, machine_readable::to_protobuf};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let proto = to_protobuf(&diff_result);
///
/// assert_eq!(proto.statute_id, "law");
/// ```
pub fn to_protobuf(diff: &StatuteDiff) -> ProtobufDiff {
    let version_info = diff.version_info.as_ref().map(|v| ProtobufVersionInfo {
        old_version: v.old_version.unwrap_or(0),
        new_version: v.new_version.unwrap_or(0),
    });

    let changes = diff
        .changes
        .iter()
        .map(|c| {
            let change_type = match c.change_type {
                ChangeType::Added => 0,
                ChangeType::Removed => 1,
                ChangeType::Modified => 2,
                ChangeType::Reordered => 3,
            };

            let (target_type, target_index) = match &c.target {
                ChangeTarget::Title => (0, 0),
                ChangeTarget::Precondition { index } => (1, *index as i32),
                ChangeTarget::Effect => (2, 0),
                ChangeTarget::DiscretionLogic => (3, 0),
                ChangeTarget::Metadata { .. } => (4, 0),
            };

            ProtobufChange {
                change_type,
                target_type,
                target_index,
                description: c.description.clone(),
                old_value: c.old_value.clone().unwrap_or_default(),
                new_value: c.new_value.clone().unwrap_or_default(),
            }
        })
        .collect();

    let severity = match diff.impact.severity {
        crate::Severity::None => 0,
        crate::Severity::Minor => 1,
        crate::Severity::Moderate => 2,
        crate::Severity::Major => 3,
        crate::Severity::Breaking => 4,
    };

    ProtobufDiff {
        statute_id: diff.statute_id.clone(),
        version_info,
        changes,
        impact: ProtobufImpact {
            severity,
            affects_eligibility: diff.impact.affects_eligibility,
            affects_outcome: diff.impact.affects_outcome,
            discretion_changed: diff.impact.discretion_changed,
            notes: diff.impact.notes.clone(),
        },
    }
}

/// Serializes a StatuteDiff to JSON Patch format as a JSON string.
pub fn serialize_json_patch(diff: &StatuteDiff) -> DiffResult<String> {
    let patch = to_json_patch(diff);
    serde_json::to_string_pretty(&patch)
        .map_err(|e| DiffError::SerializationError(format!("JSON Patch: {}", e)))
}

/// Serializes a StatuteDiff to JSON Merge Patch format as a JSON string.
pub fn serialize_json_merge_patch(diff: &StatuteDiff) -> DiffResult<String> {
    let patch = to_json_merge_patch(diff);
    serde_json::to_string_pretty(&patch)
        .map_err(|e| DiffError::SerializationError(format!("JSON Merge Patch: {}", e)))
}

/// Serializes a StatuteDiff to OpenAPI format as a JSON string.
pub fn serialize_openapi(diff: &StatuteDiff, version: &str) -> DiffResult<String> {
    let openapi = to_openapi_diff(diff, version);
    serde_json::to_string_pretty(&openapi)
        .map_err(|e| DiffError::SerializationError(format!("OpenAPI: {}", e)))
}

/// Serializes a StatuteDiff to GraphQL schema diff format as a JSON string.
pub fn serialize_graphql(diff: &StatuteDiff) -> DiffResult<String> {
    let graphql = to_graphql_schema_diff(diff);
    serde_json::to_string_pretty(&graphql)
        .map_err(|e| DiffError::SerializationError(format!("GraphQL: {}", e)))
}

/// Serializes a StatuteDiff to Protocol Buffers format as a JSON string.
pub fn serialize_protobuf(diff: &StatuteDiff) -> DiffResult<String> {
    let proto = to_protobuf(diff);
    serde_json::to_string_pretty(&proto)
        .map_err(|e| DiffError::SerializationError(format!("Protobuf: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn test_statute() -> Statute {
        Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_json_patch_title_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let patch = to_json_patch(&diff_result);

        assert_eq!(patch.len(), 1);
        assert_eq!(patch[0].op, "replace");
        assert_eq!(patch[0].path, "/title");
    }

    #[test]
    fn test_json_merge_patch() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let merge_patch = to_json_merge_patch(&diff_result);

        assert!(merge_patch.contains_key("title"));
    }

    #[test]
    fn test_openapi_diff() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let openapi = to_openapi_diff(&diff_result, "1.0.0");

        assert_eq!(openapi.openapi, "3.0.0");
        assert_eq!(openapi.info.version, "1.0.0");
        assert_eq!(openapi.changes.modified.len(), 1);
    }

    #[test]
    fn test_graphql_schema_diff() {
        let old = test_statute();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");

        let diff_result = diff(&old, &new).unwrap();
        let graphql = to_graphql_schema_diff(&diff_result);

        assert!(!graphql.breaking_changes.is_empty());
    }

    #[test]
    fn test_protobuf_conversion() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let proto = to_protobuf(&diff_result);

        assert_eq!(proto.statute_id, "test");
        assert_eq!(proto.changes.len(), 1);
    }

    #[test]
    fn test_serialize_json_patch() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let json = serialize_json_patch(&diff_result).unwrap();

        assert!(json.contains("replace"));
        assert!(json.contains("/title"));
    }
}
