//! Output formatters for statute diffs.
//!
//! This module provides different output formats for presenting
//! statute diffs: JSON, HTML, and Markdown.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, formats::{DiffFormatter, MarkdownFormatter, JsonFormatter}};
//!
//! let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut new = old.clone();
//! new.title = "New Title".to_string();
//!
//! let diff_result = diff(&old, &new).unwrap();
//!
//! // Format as Markdown
//! let md_formatter = MarkdownFormatter::new();
//! let markdown = md_formatter.format(&diff_result);
//! assert!(markdown.contains("# Statute Diff"));
//!
//! // Format as JSON
//! let json_formatter = JsonFormatter::new();
//! let json = json_formatter.format(&diff_result);
//! assert!(json.contains("statute_id"));
//! ```

use crate::{ChangeType, StatuteDiff};

/// Trait for formatting statute diffs.
pub trait DiffFormatter {
    /// Format a diff into a string.
    fn format(&self, diff: &StatuteDiff) -> String;
}

/// JSON formatter (uses serde_json).
pub struct JsonFormatter {
    /// Pretty-print the JSON output.
    pub pretty: bool,
}

impl JsonFormatter {
    /// Create a new JSON formatter.
    pub fn new() -> Self {
        Self { pretty: true }
    }

    /// Set whether to pretty-print.
    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFormatter for JsonFormatter {
    fn format(&self, diff: &StatuteDiff) -> String {
        if self.pretty {
            serde_json::to_string_pretty(diff).unwrap_or_else(|e| format!("JSON error: {}", e))
        } else {
            serde_json::to_string(diff).unwrap_or_else(|e| format!("JSON error: {}", e))
        }
    }
}

/// Markdown formatter.
pub struct MarkdownFormatter {
    /// Include detailed change information.
    pub detailed: bool,
}

impl MarkdownFormatter {
    /// Create a new Markdown formatter.
    pub fn new() -> Self {
        Self { detailed: true }
    }

    /// Set whether to include detailed information.
    pub fn with_detailed(mut self, detailed: bool) -> Self {
        self.detailed = detailed;
        self
    }
}

impl Default for MarkdownFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFormatter for MarkdownFormatter {
    fn format(&self, diff: &StatuteDiff) -> String {
        let mut md = String::new();

        // Header
        md.push_str(&format!("# Statute Diff: {}\n\n", diff.statute_id));

        // Version info
        if let Some(ref version_info) = diff.version_info {
            md.push_str("## Version Information\n\n");
            if let Some(old) = version_info.old_version {
                md.push_str(&format!("- **Old Version**: {}\n", old));
            }
            if let Some(new) = version_info.new_version {
                md.push_str(&format!("- **New Version**: {}\n", new));
            }
            md.push('\n');
        }

        // Impact summary
        md.push_str("## Impact Summary\n\n");
        md.push_str(&format!("- **Severity**: {:?}\n", diff.impact.severity));
        md.push_str(&format!("- **Total Changes**: {}\n", diff.changes.len()));
        md.push_str(&format!(
            "- **Affects Eligibility**: {}\n",
            if diff.impact.affects_eligibility {
                "Yes"
            } else {
                "No"
            }
        ));
        md.push_str(&format!(
            "- **Affects Outcome**: {}\n",
            if diff.impact.affects_outcome {
                "Yes"
            } else {
                "No"
            }
        ));
        md.push_str(&format!(
            "- **Discretion Changed**: {}\n\n",
            if diff.impact.discretion_changed {
                "Yes"
            } else {
                "No"
            }
        ));

        // Changes
        if !diff.changes.is_empty() {
            md.push_str("## Changes\n\n");
            for (idx, change) in diff.changes.iter().enumerate() {
                md.push_str(&format!("### {}. ", idx + 1));
                md.push_str(format_change_type(change.change_type));
                md.push_str(&format!(": {}\n\n", change.target));
                md.push_str(&format!("**Description**: {}\n\n", change.description));

                if self.detailed {
                    if let Some(ref old) = change.old_value {
                        md.push_str(&format!("**Old Value**:\n```\n{}\n```\n\n", old));
                    }
                    if let Some(ref new) = change.new_value {
                        md.push_str(&format!("**New Value**:\n```\n{}\n```\n\n", new));
                    }
                }
            }
        }

        // Impact notes
        if !diff.impact.notes.is_empty() {
            md.push_str("## Impact Notes\n\n");
            for note in &diff.impact.notes {
                md.push_str(&format!("- {}\n", note));
            }
            md.push('\n');
        }

        md
    }
}

/// HTML formatter.
pub struct HtmlFormatter {
    /// Include CSS styling.
    pub include_style: bool,
}

impl HtmlFormatter {
    /// Create a new HTML formatter.
    pub fn new() -> Self {
        Self {
            include_style: true,
        }
    }

    /// Set whether to include CSS styling.
    pub fn with_style(mut self, include_style: bool) -> Self {
        self.include_style = include_style;
        self
    }

    fn get_style(&self) -> &str {
        r#"
<style>
    .statute-diff {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
        max-width: 900px;
        margin: 20px auto;
        padding: 20px;
    }
    .header {
        border-bottom: 2px solid #e1e4e8;
        padding-bottom: 16px;
        margin-bottom: 16px;
    }
    .statute-id {
        font-size: 24px;
        font-weight: 600;
        color: #24292e;
    }
    .impact-summary {
        background: #f6f8fa;
        border-radius: 6px;
        padding: 16px;
        margin: 16px 0;
    }
    .severity {
        display: inline-block;
        padding: 4px 12px;
        border-radius: 12px;
        font-size: 14px;
        font-weight: 600;
    }
    .severity-none { background: #e1e4e8; color: #586069; }
    .severity-minor { background: #dbedff; color: #0366d6; }
    .severity-moderate { background: #fff5b1; color: #735c0f; }
    .severity-major { background: #ffeef0; color: #d73a49; }
    .severity-breaking { background: #f8d7da; color: #b60205; }
    .change {
        border: 1px solid #e1e4e8;
        border-radius: 6px;
        padding: 16px;
        margin: 12px 0;
    }
    .change-type {
        display: inline-block;
        padding: 2px 8px;
        border-radius: 4px;
        font-size: 12px;
        font-weight: 600;
        margin-right: 8px;
    }
    .change-added { background: #d4edda; color: #155724; }
    .change-removed { background: #f8d7da; color: #721c24; }
    .change-modified { background: #fff3cd; color: #856404; }
    .change-reordered { background: #cce5ff; color: #004085; }
    .change-target {
        font-weight: 600;
        color: #24292e;
    }
    .change-description {
        margin: 8px 0;
        color: #586069;
    }
    .code-block {
        background: #f6f8fa;
        border: 1px solid #e1e4e8;
        border-radius: 3px;
        padding: 12px;
        margin: 8px 0;
        font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
        font-size: 12px;
        overflow-x: auto;
    }
    .notes {
        background: #fff5b1;
        border-left: 4px solid #ffd33d;
        padding: 12px;
        margin: 16px 0;
    }
    .notes ul {
        margin: 8px 0;
        padding-left: 20px;
    }
</style>
"#
    }
}

impl Default for HtmlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFormatter for HtmlFormatter {
    fn format(&self, diff: &StatuteDiff) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!(
            "<title>Statute Diff: {}</title>\n",
            diff.statute_id
        ));

        if self.include_style {
            html.push_str(self.get_style());
        }

        html.push_str("</head>\n<body>\n");
        html.push_str("<div class=\"statute-diff\">\n");

        // Header
        html.push_str("<div class=\"header\">\n");
        html.push_str(&format!(
            "<h1 class=\"statute-id\">Statute Diff: {}</h1>\n",
            diff.statute_id
        ));
        if let Some(ref version_info) = diff.version_info {
            html.push_str("<div class=\"version-info\">\n");
            if let Some(old) = version_info.old_version {
                html.push_str(&format!("<span>Old Version: {}</span> ", old));
            }
            if let Some(new) = version_info.new_version {
                html.push_str(&format!("<span>New Version: {}</span>", new));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</div>\n");

        // Impact summary
        html.push_str("<div class=\"impact-summary\">\n");
        html.push_str("<h2>Impact Summary</h2>\n");
        let severity_class = format!(
            "severity-{}",
            format!("{:?}", diff.impact.severity).to_lowercase()
        );
        html.push_str(&format!(
            "<p><strong>Severity:</strong> <span class=\"severity {}\">{:?}</span></p>\n",
            severity_class, diff.impact.severity
        ));
        html.push_str(&format!(
            "<p><strong>Total Changes:</strong> {}</p>\n",
            diff.changes.len()
        ));
        html.push_str(&format!(
            "<p><strong>Affects Eligibility:</strong> {}</p>\n",
            if diff.impact.affects_eligibility {
                "Yes"
            } else {
                "No"
            }
        ));
        html.push_str(&format!(
            "<p><strong>Affects Outcome:</strong> {}</p>\n",
            if diff.impact.affects_outcome {
                "Yes"
            } else {
                "No"
            }
        ));
        html.push_str(&format!(
            "<p><strong>Discretion Changed:</strong> {}</p>\n",
            if diff.impact.discretion_changed {
                "Yes"
            } else {
                "No"
            }
        ));
        html.push_str("</div>\n");

        // Changes
        if !diff.changes.is_empty() {
            html.push_str("<h2>Changes</h2>\n");
            for change in &diff.changes {
                let change_class = format!(
                    "change-{}",
                    format!("{:?}", change.change_type).to_lowercase()
                );
                html.push_str("<div class=\"change\">\n");
                html.push_str(&format!(
                    "<span class=\"change-type {}\">{:?}</span>\n",
                    change_class, change.change_type
                ));
                html.push_str(&format!(
                    "<span class=\"change-target\">{}</span>\n",
                    change.target
                ));
                html.push_str(&format!(
                    "<div class=\"change-description\">{}</div>\n",
                    change.description
                ));

                if let Some(ref old) = change.old_value {
                    html.push_str(&format!(
                        "<div><strong>Old Value:</strong></div>\n<div class=\"code-block\">{}</div>\n",
                        html_escape(old)
                    ));
                }
                if let Some(ref new) = change.new_value {
                    html.push_str(&format!(
                        "<div><strong>New Value:</strong></div>\n<div class=\"code-block\">{}</div>\n",
                        html_escape(new)
                    ));
                }
                html.push_str("</div>\n");
            }
        }

        // Impact notes
        if !diff.impact.notes.is_empty() {
            html.push_str("<div class=\"notes\">\n");
            html.push_str("<h3>Impact Notes</h3>\n<ul>\n");
            for note in &diff.impact.notes {
                html.push_str(&format!("<li>{}</li>\n", note));
            }
            html.push_str("</ul>\n</div>\n");
        }

        html.push_str("</div>\n</body>\n</html>");
        html
    }
}

/// Side-by-side comparison formatter.
pub struct SideBySideFormatter {
    /// Format to use (markdown or html).
    pub format: SideBySideFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum SideBySideFormat {
    Markdown,
    Html,
}

impl SideBySideFormatter {
    /// Create a new side-by-side formatter.
    pub fn new(format: SideBySideFormat) -> Self {
        Self { format }
    }
}

impl Default for SideBySideFormatter {
    fn default() -> Self {
        Self {
            format: SideBySideFormat::Markdown,
        }
    }
}

impl DiffFormatter for SideBySideFormatter {
    fn format(&self, diff: &StatuteDiff) -> String {
        match self.format {
            SideBySideFormat::Markdown => format_side_by_side_markdown(diff),
            SideBySideFormat::Html => format_side_by_side_html(diff),
        }
    }
}

fn format_side_by_side_markdown(diff: &StatuteDiff) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Side-by-Side Diff: {}\n\n", diff.statute_id));
    md.push_str(&format!("**Severity**: {:?}\n\n", diff.impact.severity));

    if !diff.changes.is_empty() {
        md.push_str("| Change | Old Value | New Value |\n");
        md.push_str("|--------|-----------|----------|\n");

        for change in &diff.changes {
            let change_desc = format!("{:?} {}", change.change_type, change.target);
            let old_val = change
                .old_value
                .as_ref()
                .map(|s| truncate_for_table(s))
                .unwrap_or_else(|| "(none)".to_string());
            let new_val = change
                .new_value
                .as_ref()
                .map(|s| truncate_for_table(s))
                .unwrap_or_else(|| "(none)".to_string());

            md.push_str(&format!(
                "| {} | {} | {} |\n",
                change_desc, old_val, new_val
            ));
        }
        md.push('\n');
    }

    md
}

fn format_side_by_side_html(diff: &StatuteDiff) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str(&format!(
        "<title>Side-by-Side Diff: {}</title>\n",
        diff.statute_id
    ));
    html.push_str(
        r#"
<style>
    body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; padding: 20px; }
    h1 { color: #24292e; }
    table { border-collapse: collapse; width: 100%; margin: 20px 0; }
    th, td { border: 1px solid #e1e4e8; padding: 12px; text-align: left; }
    th { background: #f6f8fa; font-weight: 600; }
    .old-value { background: #ffeef0; }
    .new-value { background: #e6ffed; }
</style>
"#,
    );
    html.push_str("</head>\n<body>\n");

    html.push_str(&format!(
        "<h1>Side-by-Side Diff: {}</h1>\n",
        diff.statute_id
    ));
    html.push_str(&format!(
        "<p><strong>Severity:</strong> {:?}</p>\n",
        diff.impact.severity
    ));

    if !diff.changes.is_empty() {
        html.push_str("<table>\n<thead>\n<tr>\n");
        html.push_str("<th>Change</th><th>Old Value</th><th>New Value</th>\n");
        html.push_str("</tr>\n</thead>\n<tbody>\n");

        for change in &diff.changes {
            let change_desc = format!("{:?} {}", change.change_type, change.target);
            let old_val = change
                .old_value
                .as_ref()
                .map(|s| html_escape(s))
                .unwrap_or_else(|| "(none)".to_string());
            let new_val = change
                .new_value
                .as_ref()
                .map(|s| html_escape(s))
                .unwrap_or_else(|| "(none)".to_string());

            html.push_str("<tr>\n");
            html.push_str(&format!("<td>{}</td>\n", change_desc));
            html.push_str(&format!("<td class=\"old-value\">{}</td>\n", old_val));
            html.push_str(&format!("<td class=\"new-value\">{}</td>\n", new_val));
            html.push_str("</tr>\n");
        }

        html.push_str("</tbody>\n</table>\n");
    }

    html.push_str("</body>\n</html>");
    html
}

fn format_change_type(change_type: ChangeType) -> &'static str {
    match change_type {
        ChangeType::Added => "Added",
        ChangeType::Removed => "Removed",
        ChangeType::Modified => "Modified",
        ChangeType::Reordered => "Reordered",
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn truncate_for_table(s: &str) -> String {
    const MAX_LEN: usize = 50;
    if s.len() > MAX_LEN {
        format!("{}...", &s[..MAX_LEN])
    } else {
        s.replace('\n', " ").replace('|', "\\|")
    }
}

/// CSV formatter for diff results.
///
/// Formats statute diffs as CSV (Comma-Separated Values) for use in
/// spreadsheet applications or data analysis tools.
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, formats::{DiffFormatter, CsvFormatter}};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let formatter = CsvFormatter::new();
/// let csv = formatter.format(&diff_result);
///
/// assert!(csv.contains("statute_id,change_type,target"));
/// ```
pub struct CsvFormatter {
    /// Include header row.
    pub include_header: bool,
}

impl CsvFormatter {
    /// Create a new CSV formatter.
    pub fn new() -> Self {
        Self {
            include_header: true,
        }
    }

    /// Set whether to include header row.
    pub fn with_header(mut self, include_header: bool) -> Self {
        self.include_header = include_header;
        self
    }
}

impl Default for CsvFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFormatter for CsvFormatter {
    fn format(&self, diff: &StatuteDiff) -> String {
        let mut csv = String::new();

        // Header
        if self.include_header {
            csv.push_str("statute_id,change_type,target,description,old_value,new_value,severity,affects_eligibility,affects_outcome,discretion_changed\n");
        }

        // Rows
        for change in &diff.changes {
            csv.push_str(&csv_escape(&diff.statute_id));
            csv.push(',');
            csv.push_str(&format!("{:?}", change.change_type));
            csv.push(',');
            csv.push_str(&csv_escape(&format!("{}", change.target)));
            csv.push(',');
            csv.push_str(&csv_escape(&change.description));
            csv.push(',');
            csv.push_str(&csv_escape(change.old_value.as_deref().unwrap_or("")));
            csv.push(',');
            csv.push_str(&csv_escape(change.new_value.as_deref().unwrap_or("")));
            csv.push(',');
            csv.push_str(&format!("{:?}", diff.impact.severity));
            csv.push(',');
            csv.push_str(if diff.impact.affects_eligibility {
                "true"
            } else {
                "false"
            });
            csv.push(',');
            csv.push_str(if diff.impact.affects_outcome {
                "true"
            } else {
                "false"
            });
            csv.push(',');
            csv.push_str(if diff.impact.discretion_changed {
                "true"
            } else {
                "false"
            });
            csv.push('\n');
        }

        csv
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Formats multiple diffs as a single CSV file.
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, formats::format_batch_csv};
///
/// let old1 = Statute::new("law1", "Title 1", Effect::new(EffectType::Grant, "Benefit"));
/// let new1 = Statute::new("law1", "New Title 1", Effect::new(EffectType::Grant, "Benefit"));
///
/// let old2 = Statute::new("law2", "Title 2", Effect::new(EffectType::Grant, "Benefit"));
/// let new2 = Statute::new("law2", "New Title 2", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diffs = vec![
///     diff(&old1, &new1).unwrap(),
///     diff(&old2, &new2).unwrap(),
/// ];
///
/// let csv = format_batch_csv(&diffs);
/// assert!(csv.contains("law1"));
/// assert!(csv.contains("law2"));
/// ```
pub fn format_batch_csv(diffs: &[StatuteDiff]) -> String {
    let mut csv = String::new();

    // Header (only once)
    csv.push_str("statute_id,change_type,target,description,old_value,new_value,severity,affects_eligibility,affects_outcome,discretion_changed\n");

    // Format each diff without headers
    let formatter = CsvFormatter::new().with_header(false);
    for diff in diffs {
        csv.push_str(&formatter.format(diff));
    }

    csv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Change, ChangeTarget, ChangeType, ImpactAssessment, Severity, StatuteDiff};

    fn test_diff() -> StatuteDiff {
        StatuteDiff {
            statute_id: "test-statute".to_string(),
            version_info: None,
            changes: vec![
                Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::Title,
                    description: "Title changed".to_string(),
                    old_value: Some("Old Title".to_string()),
                    new_value: Some("New Title".to_string()),
                },
                Change {
                    change_type: ChangeType::Added,
                    target: ChangeTarget::Precondition { index: 0 },
                    description: "Added new precondition".to_string(),
                    old_value: None,
                    new_value: Some("Age >= 18".to_string()),
                },
            ],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec!["Test note".to_string()],
            },
        }
    }

    #[test]
    fn test_json_formatter() {
        let diff = test_diff();
        let formatter = JsonFormatter::new();
        let output = formatter.format(&diff);
        assert!(output.contains("test-statute"));
        assert!(output.contains("Moderate"));
    }

    #[test]
    fn test_markdown_formatter() {
        let diff = test_diff();
        let formatter = MarkdownFormatter::new();
        let output = formatter.format(&diff);
        assert!(output.contains("# Statute Diff"));
        assert!(output.contains("test-statute"));
        assert!(output.contains("## Impact Summary"));
    }

    #[test]
    fn test_html_formatter() {
        let diff = test_diff();
        let formatter = HtmlFormatter::new();
        let output = formatter.format(&diff);
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("test-statute"));
        assert!(output.contains("class=\"severity"));
    }

    #[test]
    fn test_side_by_side_markdown() {
        let diff = test_diff();
        let formatter = SideBySideFormatter::new(SideBySideFormat::Markdown);
        let output = formatter.format(&diff);
        assert!(output.contains("Side-by-Side Diff"));
        assert!(output.contains("|"));
    }

    #[test]
    fn test_side_by_side_html() {
        let diff = test_diff();
        let formatter = SideBySideFormatter::new(SideBySideFormat::Html);
        let output = formatter.format(&diff);
        assert!(output.contains("<table>"));
        assert!(output.contains("Side-by-Side Diff"));
    }

    #[test]
    fn test_csv_formatter() {
        let diff = test_diff();
        let formatter = CsvFormatter::new();
        let output = formatter.format(&diff);
        assert!(output.contains("statute_id,change_type,target"));
        assert!(output.contains("test-statute"));
        assert!(output.contains("Modified"));
        assert!(output.contains("Added"));
    }

    #[test]
    fn test_csv_formatter_no_header() {
        let diff = test_diff();
        let formatter = CsvFormatter::new().with_header(false);
        let output = formatter.format(&diff);
        assert!(!output.contains("statute_id,change_type,target"));
        assert!(output.contains("test-statute"));
    }

    #[test]
    fn test_format_batch_csv() {
        let diff1 = test_diff();
        let mut diff2 = test_diff();
        diff2.statute_id = "test-statute-2".to_string();

        let csv = format_batch_csv(&[diff1, diff2]);
        assert!(csv.contains("test-statute"));
        assert!(csv.contains("test-statute-2"));
        // Header should only appear once
        let header_count = csv.matches("statute_id,change_type,target").count();
        assert_eq!(header_count, 1);
    }

    #[test]
    fn test_csv_escape() {
        assert_eq!(csv_escape("simple"), "simple");
        assert_eq!(csv_escape("has,comma"), "\"has,comma\"");
        assert_eq!(csv_escape("has\"quote"), "\"has\"\"quote\"");
        assert_eq!(csv_escape("has\nnewline"), "\"has\nnewline\"");
    }
}

/// Export format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    JsonPretty,
    Markdown,
    Html,
    Csv,
}

/// Result of a batch export operation.
#[derive(Debug, Clone)]
pub struct BatchExportResult {
    pub format: ExportFormat,
    pub outputs: Vec<String>,
}

/// Exports multiple diffs to a specific format in parallel.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, formats::{parallel_export, ExportFormat}};
///
/// let pairs: Vec<_> = (0..10)
///     .map(|i| {
///         let id = format!("law{}", i);
///         (
///             Statute::new(&id, "Old Title", Effect::new(EffectType::Grant, "Benefit")),
///             Statute::new(&id, "New Title", Effect::new(EffectType::Grant, "Benefit")),
///         )
///     })
///     .collect();
///
/// let diffs: Vec<_> = pairs
///     .iter()
///     .map(|(old, new)| diff(old, new).unwrap())
///     .collect();
///
/// let result = parallel_export(&diffs, ExportFormat::Json);
/// assert_eq!(result.outputs.len(), 10);
/// ```
pub fn parallel_export(diffs: &[StatuteDiff], format: ExportFormat) -> BatchExportResult {
    use rayon::prelude::*;

    let outputs: Vec<String> = diffs
        .par_iter()
        .map(|diff| match format {
            ExportFormat::Json => JsonFormatter::new().with_pretty(false).format(diff),
            ExportFormat::JsonPretty => JsonFormatter::new().with_pretty(true).format(diff),
            ExportFormat::Markdown => MarkdownFormatter::new().format(diff),
            ExportFormat::Html => HtmlFormatter::new().format(diff),
            ExportFormat::Csv => CsvFormatter::new().format(diff),
        })
        .collect();

    BatchExportResult { format, outputs }
}

/// Exports multiple diffs to all formats in parallel.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, formats::parallel_export_all};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let results = parallel_export_all(&[diff_result]);
///
/// // Should have results for all formats
/// assert_eq!(results.len(), 5);
/// ```
pub fn parallel_export_all(diffs: &[StatuteDiff]) -> Vec<BatchExportResult> {
    use rayon::prelude::*;

    vec![
        ExportFormat::Json,
        ExportFormat::JsonPretty,
        ExportFormat::Markdown,
        ExportFormat::Html,
        ExportFormat::Csv,
    ]
    .par_iter()
    .map(|&format| parallel_export(diffs, format))
    .collect()
}

/// Exports a single diff to all formats.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, formats::export_to_all_formats};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let exports = export_to_all_formats(&diff_result);
///
/// assert!(exports.contains_key(&"json"));
/// assert!(exports.contains_key(&"markdown"));
/// assert!(exports.contains_key(&"html"));
/// assert!(exports.contains_key(&"csv"));
/// ```
pub fn export_to_all_formats(
    diff: &StatuteDiff,
) -> std::collections::HashMap<&'static str, String> {
    let mut exports = std::collections::HashMap::new();

    exports.insert("json", JsonFormatter::new().with_pretty(false).format(diff));
    exports.insert(
        "json_pretty",
        JsonFormatter::new().with_pretty(true).format(diff),
    );
    exports.insert("markdown", MarkdownFormatter::new().format(diff));
    exports.insert("html", HtmlFormatter::new().format(diff));
    exports.insert("csv", CsvFormatter::new().format(diff));

    exports
}

#[cfg(test)]
mod export_tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test benefit"))
    }

    #[test]
    fn test_parallel_export_json() {
        let pairs: Vec<_> = (0..20)
            .map(|i| {
                let id = format!("law{}", i);
                (
                    create_test_statute(&id, "Old Title"),
                    create_test_statute(&id, "New Title"),
                )
            })
            .collect();

        let diffs: Vec<_> = pairs
            .iter()
            .map(|(old, new)| diff(old, new).unwrap())
            .collect();

        let result = parallel_export(&diffs, ExportFormat::Json);
        assert_eq!(result.outputs.len(), 20);
        assert_eq!(result.format, ExportFormat::Json);

        for output in &result.outputs {
            assert!(output.contains("statute_id"));
        }
    }

    #[test]
    fn test_parallel_export_markdown() {
        let pairs: Vec<_> = (0..15)
            .map(|i| {
                let id = format!("law{}", i);
                (
                    create_test_statute(&id, "Old Title"),
                    create_test_statute(&id, "New Title"),
                )
            })
            .collect();

        let diffs: Vec<_> = pairs
            .iter()
            .map(|(old, new)| diff(old, new).unwrap())
            .collect();

        let result = parallel_export(&diffs, ExportFormat::Markdown);
        assert_eq!(result.outputs.len(), 15);

        for output in &result.outputs {
            assert!(output.contains("# Statute Diff"));
        }
    }

    #[test]
    fn test_parallel_export_all() {
        let old = create_test_statute("law", "Old Title");
        let new = create_test_statute("law", "New Title");
        let diff_result = diff(&old, &new).unwrap();

        let results = parallel_export_all(&[diff_result]);
        assert_eq!(results.len(), 5);

        for result in &results {
            assert_eq!(result.outputs.len(), 1);
        }
    }

    #[test]
    fn test_export_to_all_formats() {
        let old = create_test_statute("law", "Old Title");
        let new = create_test_statute("law", "New Title");
        let diff_result = diff(&old, &new).unwrap();

        let exports = export_to_all_formats(&diff_result);

        assert_eq!(exports.len(), 5);
        assert!(exports.contains_key("json"));
        assert!(exports.contains_key("json_pretty"));
        assert!(exports.contains_key("markdown"));
        assert!(exports.contains_key("html"));
        assert!(exports.contains_key("csv"));

        assert!(exports["json"].contains("statute_id"));
        assert!(exports["markdown"].contains("# Statute Diff"));
        assert!(exports["html"].contains("<html>"));
        assert!(exports["csv"].contains("statute_id,change_type"));
    }

    #[test]
    fn test_parallel_export_large_batch() {
        let pairs: Vec<_> = (0..100)
            .map(|i| {
                let id = format!("statute-{}", i);
                let old = create_test_statute(&id, "Old Title").with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 65,
                });
                let mut new = old.clone();
                new.title = format!("New Title {}", i);
                (old, new)
            })
            .collect();

        let diffs: Vec<_> = pairs
            .iter()
            .map(|(old, new)| diff(old, new).unwrap())
            .collect();

        let result = parallel_export(&diffs, ExportFormat::JsonPretty);
        assert_eq!(result.outputs.len(), 100);
    }
}
