//! Advanced export formats for diff results.
//!
//! This module provides support for:
//! - Word track-changes format
//! - PDF with highlighted changes
//! - LaTeX redline format
//! - Unified diff format (patch files)
//! - Structured changelog (CHANGELOG.md)

use crate::{Change, ChangeType, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Generates a unified diff format (like Git patch files).
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, export::generate_unified_diff};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let unified = generate_unified_diff(&diff_result);
///
/// assert!(unified.contains("---"));
/// assert!(unified.contains("+++"));
/// ```
pub fn generate_unified_diff(diff: &StatuteDiff) -> String {
    let mut output = String::new();

    output.push_str(&format!("--- a/{}\n", diff.statute_id));
    output.push_str(&format!("+++ b/{}\n", diff.statute_id));
    output.push_str(&format!("@@ Changes: {} @@\n", diff.changes.len()));

    for change in &diff.changes {
        match change.change_type {
            ChangeType::Added => {
                if let Some(new_val) = &change.new_value {
                    output.push_str(&format!("+{}: {}\n", change.target, new_val));
                }
            }
            ChangeType::Removed => {
                if let Some(old_val) = &change.old_value {
                    output.push_str(&format!("-{}: {}\n", change.target, old_val));
                }
            }
            ChangeType::Modified => {
                if let Some(old_val) = &change.old_value {
                    output.push_str(&format!("-{}: {}\n", change.target, old_val));
                }
                if let Some(new_val) = &change.new_value {
                    output.push_str(&format!("+{}: {}\n", change.target, new_val));
                }
            }
            ChangeType::Reordered => {
                output.push_str(&format!(" {}: reordered\n", change.target));
            }
        }
    }

    output
}

/// Generates LaTeX redline format for changes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, export::generate_latex_redline};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let latex = generate_latex_redline(&diff_result);
///
/// assert!(latex.contains("\\documentclass"));
/// assert!(latex.contains("\\textcolor"));
/// ```
pub fn generate_latex_redline(diff: &StatuteDiff) -> String {
    let mut output = String::new();

    // LaTeX preamble
    output.push_str("\\documentclass{article}\n");
    output.push_str("\\usepackage{xcolor}\n");
    output.push_str("\\usepackage{soul}\n");
    output.push_str("\\begin{document}\n\n");

    output.push_str(&format!(
        "\\section{{Changes to Statute: {}}}\n\n",
        diff.statute_id
    ));

    for change in &diff.changes {
        output.push_str("\\subsection*{");
        output.push_str(&format!("{}", change.target));
        output.push_str("}\n\n");

        match change.change_type {
            ChangeType::Added => {
                if let Some(new_val) = &change.new_value {
                    output.push_str(&format!(
                        "\\textcolor{{green}}{{Added: {}}}",
                        latex_escape(new_val)
                    ));
                }
            }
            ChangeType::Removed => {
                if let Some(old_val) = &change.old_value {
                    output.push_str(&format!(
                        "\\textcolor{{red}}{{\\st{{{}}}}}",
                        latex_escape(old_val)
                    ));
                }
            }
            ChangeType::Modified => {
                if let Some(old_val) = &change.old_value {
                    output.push_str(&format!(
                        "\\textcolor{{red}}{{\\st{{{}}}}}",
                        latex_escape(old_val)
                    ));
                }
                output.push_str(" $\\rightarrow$ ");
                if let Some(new_val) = &change.new_value {
                    output.push_str(&format!(
                        "\\textcolor{{green}}{{{}}}",
                        latex_escape(new_val)
                    ));
                }
            }
            ChangeType::Reordered => {
                output.push_str(&format!(
                    "\\textcolor{{blue}}{{Reordered: {}}}",
                    change.description
                ));
            }
        }

        output.push_str("\n\n");
    }

    output.push_str("\\end{document}\n");

    output
}

/// Escapes special LaTeX characters.
fn latex_escape(s: &str) -> String {
    s.replace('\\', "\\textbackslash{}")
        .replace('&', "\\&")
        .replace('%', "\\%")
        .replace('$', "\\$")
        .replace('#', "\\#")
        .replace('_', "\\_")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('~', "\\textasciitilde{}")
        .replace('^', "\\textasciicircum{}")
}

/// Generates a structured changelog in Markdown format.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, export::generate_changelog};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let changelog = generate_changelog(&diff_result, "1.1.0");
///
/// assert!(changelog.contains("# Changelog"));
/// assert!(changelog.contains("1.1.0"));
/// ```
pub fn generate_changelog(diff: &StatuteDiff, version: &str) -> String {
    let mut output = String::new();

    output.push_str("# Changelog\n\n");
    output.push_str(&format!(
        "## [{}] - {}\n\n",
        version,
        chrono::Utc::now().format("%Y-%m-%d")
    ));
    output.push_str(&format!("### Statute: {}\n\n", diff.statute_id));

    // Group changes by type
    let added: Vec<&Change> = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Added)
        .collect();
    let removed: Vec<&Change> = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Removed)
        .collect();
    let modified: Vec<&Change> = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Modified)
        .collect();

    if !added.is_empty() {
        output.push_str("#### Added\n\n");
        for change in added {
            output.push_str(&format!("- {}: {}\n", change.target, change.description));
        }
        output.push('\n');
    }

    if !modified.is_empty() {
        output.push_str("#### Changed\n\n");
        for change in modified {
            output.push_str(&format!("- {}: {}\n", change.target, change.description));
        }
        output.push('\n');
    }

    if !removed.is_empty() {
        output.push_str("#### Removed\n\n");
        for change in removed {
            output.push_str(&format!("- {}: {}\n", change.target, change.description));
        }
        output.push('\n');
    }

    // Add impact assessment
    if diff.impact.severity > crate::Severity::None {
        output.push_str("#### Impact\n\n");
        output.push_str(&format!("- Severity: {:?}\n", diff.impact.severity));
        if diff.impact.affects_eligibility {
            output.push_str("- Affects eligibility criteria\n");
        }
        if diff.impact.affects_outcome {
            output.push_str("- Affects outcome\n");
        }
        if diff.impact.discretion_changed {
            output.push_str("- Discretion requirements changed\n");
        }
        output.push('\n');
    }

    output
}

/// Generates Word-compatible track changes XML format.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, export::generate_word_track_changes};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let word_xml = generate_word_track_changes(&diff_result, "John Doe");
///
/// assert!(word_xml.contains("<?xml"));
/// assert!(word_xml.contains("John Doe"));
/// ```
pub fn generate_word_track_changes(diff: &StatuteDiff, author: &str) -> String {
    let mut output = String::new();

    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    output.push_str(
        "<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\n",
    );
    output.push_str("  <w:body>\n");

    output.push_str(&format!(
        "    <w:p><w:r><w:t>Changes to Statute: {}</w:t></w:r></w:p>\n",
        xml_escape(&diff.statute_id)
    ));

    for change in &diff.changes {
        match change.change_type {
            ChangeType::Added => {
                if let Some(new_val) = &change.new_value {
                    output.push_str("    <w:p>\n");
                    output.push_str("      <w:ins w:author=\"");
                    output.push_str(author);
                    output.push_str("\">\n");
                    output.push_str("        <w:r><w:t>");
                    output.push_str(&xml_escape(&format!("{}: {}", change.target, new_val)));
                    output.push_str("</w:t></w:r>\n");
                    output.push_str("      </w:ins>\n");
                    output.push_str("    </w:p>\n");
                }
            }
            ChangeType::Removed => {
                if let Some(old_val) = &change.old_value {
                    output.push_str("    <w:p>\n");
                    output.push_str("      <w:del w:author=\"");
                    output.push_str(author);
                    output.push_str("\">\n");
                    output.push_str("        <w:r><w:delText>");
                    output.push_str(&xml_escape(&format!("{}: {}", change.target, old_val)));
                    output.push_str("</w:delText></w:r>\n");
                    output.push_str("      </w:del>\n");
                    output.push_str("    </w:p>\n");
                }
            }
            ChangeType::Modified => {
                output.push_str("    <w:p>\n");
                if let Some(old_val) = &change.old_value {
                    output.push_str("      <w:del w:author=\"");
                    output.push_str(author);
                    output.push_str("\">\n");
                    output.push_str("        <w:r><w:delText>");
                    output.push_str(&xml_escape(&format!("{}: {}", change.target, old_val)));
                    output.push_str("</w:delText></w:r>\n");
                    output.push_str("      </w:del>\n");
                }
                if let Some(new_val) = &change.new_value {
                    output.push_str("      <w:ins w:author=\"");
                    output.push_str(author);
                    output.push_str("\">\n");
                    output.push_str("        <w:r><w:t>");
                    output.push_str(&xml_escape(&format!("{}: {}", change.target, new_val)));
                    output.push_str("</w:t></w:r>\n");
                    output.push_str("      </w:ins>\n");
                }
                output.push_str("    </w:p>\n");
            }
            ChangeType::Reordered => {
                output.push_str("    <w:p>\n");
                output.push_str("      <w:r><w:t>");
                output.push_str(&xml_escape(&format!(
                    "{}: Reordered - {}",
                    change.target, change.description
                )));
                output.push_str("</w:t></w:r>\n");
                output.push_str("    </w:p>\n");
            }
        }
    }

    output.push_str("  </w:body>\n");
    output.push_str("</w:document>\n");

    output
}

/// Escapes special XML characters.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// PDF export configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    /// Include syntax highlighting
    pub syntax_highlighting: bool,
    /// Highlight color for additions (RGB)
    pub addition_color: (u8, u8, u8),
    /// Highlight color for deletions (RGB)
    pub deletion_color: (u8, u8, u8),
    /// Highlight color for modifications (RGB)
    pub modification_color: (u8, u8, u8),
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            syntax_highlighting: true,
            addition_color: (144, 238, 144),     // Light green
            deletion_color: (255, 182, 193),     // Light red
            modification_color: (255, 255, 153), // Light yellow
        }
    }
}

/// Generates PDF metadata and content structure for diff.
///
/// Note: This generates a simplified text representation that can be
/// converted to PDF using external tools like wkhtmltopdf or pandoc.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, export::{generate_pdf_content, PdfConfig}};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let config = PdfConfig::default();
/// let pdf_html = generate_pdf_content(&diff_result, &config);
///
/// assert!(pdf_html.contains("<html>"));
/// assert!(pdf_html.contains("background-color"));
/// ```
pub fn generate_pdf_content(diff: &StatuteDiff, config: &PdfConfig) -> String {
    let mut output = String::new();

    // HTML that can be converted to PDF
    output.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    output.push_str("<meta charset=\"UTF-8\">\n");
    output.push_str(&format!("<title>Diff: {}</title>\n", diff.statute_id));
    output.push_str("<style>\n");
    output.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
    output.push_str("h1 { color: #333; }\n");
    output.push_str(".change { margin: 10px 0; padding: 10px; border-radius: 5px; }\n");
    output.push_str(&format!(
        ".added {{ background-color: rgb({}, {}, {}); }}\n",
        config.addition_color.0, config.addition_color.1, config.addition_color.2
    ));
    output.push_str(&format!(
        ".removed {{ background-color: rgb({}, {}, {}); text-decoration: line-through; }}\n",
        config.deletion_color.0, config.deletion_color.1, config.deletion_color.2
    ));
    output.push_str(&format!(
        ".modified {{ background-color: rgb({}, {}, {}); }}\n",
        config.modification_color.0, config.modification_color.1, config.modification_color.2
    ));
    output.push_str(".impact { margin-top: 20px; padding: 15px; background-color: #f0f0f0; border-left: 4px solid #333; }\n");
    output.push_str("</style>\n</head>\n<body>\n");

    output.push_str(&format!(
        "<h1>Changes to Statute: {}</h1>\n",
        diff.statute_id
    ));

    if let Some(version_info) = &diff.version_info {
        output.push_str("<p><strong>Version: </strong>");
        if let Some(old_ver) = version_info.old_version {
            output.push_str(&format!("{}", old_ver));
        } else {
            output.push_str("N/A");
        }
        output.push_str(" → ");
        if let Some(new_ver) = version_info.new_version {
            output.push_str(&format!("{}", new_ver));
        } else {
            output.push_str("N/A");
        }
        output.push_str("</p>\n");
    }

    output.push_str("<h2>Changes</h2>\n");

    for change in &diff.changes {
        let class = match change.change_type {
            ChangeType::Added => "added",
            ChangeType::Removed => "removed",
            ChangeType::Modified => "modified",
            ChangeType::Reordered => "modified",
        };

        output.push_str(&format!("<div class=\"change {}\">\n", class));
        output.push_str(&format!("<strong>{}</strong>: ", change.target));
        output.push_str(&format!("{}<br>\n", change.description));

        if let Some(old_val) = &change.old_value {
            output.push_str(&format!("<span class=\"removed\">Old: {}</span>", old_val));
        }
        if change.old_value.is_some() && change.new_value.is_some() {
            output.push_str(" → ");
        }
        if let Some(new_val) = &change.new_value {
            output.push_str(&format!("<span class=\"added\">New: {}</span>", new_val));
        }

        output.push_str("</div>\n");
    }

    // Impact assessment
    output.push_str("<div class=\"impact\">\n");
    output.push_str("<h2>Impact Assessment</h2>\n");
    output.push_str(&format!(
        "<p><strong>Severity:</strong> {:?}</p>\n",
        diff.impact.severity
    ));
    output.push_str(&format!(
        "<p><strong>Affects Eligibility:</strong> {}</p>\n",
        diff.impact.affects_eligibility
    ));
    output.push_str(&format!(
        "<p><strong>Affects Outcome:</strong> {}</p>\n",
        diff.impact.affects_outcome
    ));
    output.push_str(&format!(
        "<p><strong>Discretion Changed:</strong> {}</p>\n",
        diff.impact.discretion_changed
    ));

    if !diff.impact.notes.is_empty() {
        output.push_str("<h3>Notes:</h3>\n<ul>\n");
        for note in &diff.impact.notes {
            output.push_str(&format!("<li>{}</li>\n", note));
        }
        output.push_str("</ul>\n");
    }

    output.push_str("</div>\n");

    output.push_str("</body>\n</html>\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_diff() -> StatuteDiff {
        let old = Statute::new(
            "test-law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let new = Statute::new(
            "test-law",
            "New Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        diff(&old, &new).unwrap()
    }

    #[test]
    fn test_unified_diff() {
        let diff = test_diff();
        let unified = generate_unified_diff(&diff);

        assert!(unified.contains("---"));
        assert!(unified.contains("+++"));
        assert!(unified.contains("test-law"));
    }

    #[test]
    fn test_latex_redline() {
        let diff = test_diff();
        let latex = generate_latex_redline(&diff);

        assert!(latex.contains("\\documentclass"));
        assert!(latex.contains("\\textcolor"));
        assert!(latex.contains("test-law"));
    }

    #[test]
    fn test_changelog() {
        let diff = test_diff();
        let changelog = generate_changelog(&diff, "1.1.0");

        assert!(changelog.contains("# Changelog"));
        assert!(changelog.contains("1.1.0"));
        assert!(changelog.contains("test-law"));
    }

    #[test]
    fn test_word_track_changes() {
        let diff = test_diff();
        let word_xml = generate_word_track_changes(&diff, "John Doe");

        assert!(word_xml.contains("<?xml"));
        assert!(word_xml.contains("John Doe"));
        assert!(word_xml.contains("test-law"));
    }

    #[test]
    fn test_pdf_content() {
        let diff = test_diff();
        let config = PdfConfig::default();
        let pdf_html = generate_pdf_content(&diff, &config);

        assert!(pdf_html.contains("<html>"));
        assert!(pdf_html.contains("test-law"));
        assert!(pdf_html.contains("background-color"));
    }

    #[test]
    fn test_latex_escape() {
        let escaped = latex_escape("Test & $100 #tag");
        assert!(escaped.contains("\\&"));
        assert!(escaped.contains("\\$"));
        assert!(escaped.contains("\\#"));
    }

    #[test]
    fn test_xml_escape() {
        let escaped = xml_escape("Test <tag> & \"quote\"");
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&amp;"));
        assert!(escaped.contains("&quot;"));
    }
}
