//! Audit log export capabilities.
//!
//! Serializes [`crate::audit::AuditEntry`] records into portable export formats
//! suitable for compliance archival and ingestion into external SIEM / log
//! systems: pretty JSON, newline-delimited JSON (NDJSON), and RFC 4180 CSV.
//!
//! The CSV writer performs correct field quoting/escaping and flattens the
//! structured `details` JSON into a compact string column so the export remains
//! a flat table.

use crate::audit::{AuditEntry, AuditResult};
use serde::{Deserialize, Serialize};

/// Supported audit export formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    /// Pretty-printed JSON array.
    Json,
    /// Newline-delimited JSON (one record per line).
    Ndjson,
    /// RFC 4180 comma-separated values with a header row.
    Csv,
}

impl ExportFormat {
    /// Returns the MIME content type for the format.
    pub fn content_type(&self) -> &'static str {
        match self {
            ExportFormat::Json => "application/json",
            ExportFormat::Ndjson => "application/x-ndjson",
            ExportFormat::Csv => "text/csv",
        }
    }

    /// Returns a suggested file extension.
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json => "json",
            ExportFormat::Ndjson => "ndjson",
            ExportFormat::Csv => "csv",
        }
    }

    /// Parses a format from a textual name (case-insensitive).
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "json" => Some(ExportFormat::Json),
            "ndjson" | "jsonl" => Some(ExportFormat::Ndjson),
            "csv" => Some(ExportFormat::Csv),
            _ => None,
        }
    }
}

/// Errors produced during export.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    /// Serialization failed.
    #[error("serialization failed: {0}")]
    Serialization(String),
}

/// Renders an [`AuditResult`] as a stable lowercase token.
fn result_token(result: &AuditResult) -> &'static str {
    match result {
        AuditResult::Success => "success",
        AuditResult::Failure => "failure",
        AuditResult::Partial => "partial",
    }
}

/// Quotes a CSV field per RFC 4180: wraps in double quotes if it contains a
/// comma, quote, CR, or LF, doubling any embedded quotes.
fn csv_quote(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        let escaped = field.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        field.to_string()
    }
}

/// The CSV header row used by [`export_csv`].
pub const CSV_HEADER: &str = "id,timestamp,event_type,user_id,username,resource_id,resource_type,action,result,error_message,details";

/// Exports audit entries to a CSV string with a header row.
pub fn export_csv(entries: &[AuditEntry]) -> String {
    let mut out = String::new();
    out.push_str(CSV_HEADER);
    out.push('\n');
    for entry in entries {
        let event_type = serde_json::to_value(&entry.event_type)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        let details = entry.details.to_string();
        let row = [
            csv_quote(&entry.id),
            csv_quote(&entry.timestamp.to_rfc3339()),
            csv_quote(&event_type),
            csv_quote(&entry.user_id),
            csv_quote(&entry.username),
            csv_quote(entry.resource_id.as_deref().unwrap_or("")),
            csv_quote(entry.resource_type.as_deref().unwrap_or("")),
            csv_quote(&entry.action),
            csv_quote(result_token(&entry.result)),
            csv_quote(entry.error_message.as_deref().unwrap_or("")),
            csv_quote(&details),
        ]
        .join(",");
        out.push_str(&row);
        out.push('\n');
    }
    out
}

/// Exports audit entries as a pretty-printed JSON array.
pub fn export_json(entries: &[AuditEntry]) -> Result<String, ExportError> {
    serde_json::to_string_pretty(entries).map_err(|e| ExportError::Serialization(e.to_string()))
}

/// Exports audit entries as newline-delimited JSON.
pub fn export_ndjson(entries: &[AuditEntry]) -> Result<String, ExportError> {
    let mut out = String::new();
    for entry in entries {
        let line =
            serde_json::to_string(entry).map_err(|e| ExportError::Serialization(e.to_string()))?;
        out.push_str(&line);
        out.push('\n');
    }
    Ok(out)
}

/// Exports audit entries in the requested format, returning the body and the
/// content type.
pub fn export(
    entries: &[AuditEntry],
    format: ExportFormat,
) -> Result<(String, &'static str), ExportError> {
    let body = match format {
        ExportFormat::Json => export_json(entries)?,
        ExportFormat::Ndjson => export_ndjson(entries)?,
        ExportFormat::Csv => export_csv(entries),
    };
    Ok((body, format.content_type()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::AuditEventType;
    use chrono::Utc;

    fn sample_entry(action: &str, details: serde_json::Value) -> AuditEntry {
        AuditEntry {
            id: "id-1".to_string(),
            timestamp: Utc::now(),
            event_type: AuditEventType::StatuteCreated,
            user_id: "user-1".to_string(),
            username: "alice".to_string(),
            resource_id: Some("statute-9".to_string()),
            resource_type: Some("statute".to_string()),
            action: action.to_string(),
            details,
            ip_address: None,
            user_agent: None,
            result: AuditResult::Success,
            error_message: None,
        }
    }

    #[test]
    fn test_format_metadata() {
        assert_eq!(ExportFormat::Json.content_type(), "application/json");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
        assert_eq!(ExportFormat::from_name("JSONL"), Some(ExportFormat::Ndjson));
        assert_eq!(ExportFormat::from_name("xml"), None);
    }

    #[test]
    fn test_csv_quote() {
        assert_eq!(csv_quote("plain"), "plain");
        assert_eq!(csv_quote("a,b"), "\"a,b\"");
        assert_eq!(csv_quote("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(csv_quote("line\nbreak"), "\"line\nbreak\"");
    }

    #[test]
    fn test_export_csv_header_and_rows() {
        let entries = vec![
            sample_entry("create", serde_json::json!({"k": "v"})),
            sample_entry("update", serde_json::json!({"a": 1})),
        ];
        let csv = export_csv(&entries);
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], CSV_HEADER);
        assert_eq!(lines.len(), 3); // header + 2 rows
        assert!(lines[1].contains("statute_created"));
        assert!(lines[1].contains("alice"));
    }

    #[test]
    fn test_export_csv_quotes_details_with_commas() {
        let entry = sample_entry("create", serde_json::json!({"a": 1, "b": 2}));
        let csv = export_csv(&[entry]);
        // The details JSON contains a comma so it must be quoted.
        assert!(csv.contains("\"{\"\"a\"\":1,\"\"b\"\":2}\""));
    }

    #[test]
    fn test_export_json_roundtrip() {
        let entries = vec![sample_entry("create", serde_json::json!({"k": "v"}))];
        let json = export_json(&entries).expect("json");
        let parsed: Vec<AuditEntry> = serde_json::from_str(&json).expect("parse");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].action, "create");
    }

    #[test]
    fn test_export_ndjson() {
        let entries = vec![
            sample_entry("create", serde_json::json!({})),
            sample_entry("delete", serde_json::json!({})),
        ];
        let ndjson = export_ndjson(&entries).expect("ndjson");
        let lines: Vec<&str> = ndjson.lines().collect();
        assert_eq!(lines.len(), 2);
        for line in lines {
            let _: AuditEntry = serde_json::from_str(line).expect("each line parses");
        }
    }

    #[test]
    fn test_export_dispatch() {
        let entries = vec![sample_entry("create", serde_json::json!({}))];
        let (body, ct) = export(&entries, ExportFormat::Csv).expect("export");
        assert_eq!(ct, "text/csv");
        assert!(body.starts_with(CSV_HEADER));

        let (body, ct) = export(&entries, ExportFormat::Ndjson).expect("export");
        assert_eq!(ct, "application/x-ndjson");
        assert_eq!(body.lines().count(), 1);
    }

    #[test]
    fn test_export_empty() {
        let csv = export_csv(&[]);
        assert_eq!(csv.trim(), CSV_HEADER);
        let json = export_json(&[]).expect("json");
        assert_eq!(json.trim(), "[]");
        let ndjson = export_ndjson(&[]).expect("ndjson");
        assert!(ndjson.is_empty());
    }
}
