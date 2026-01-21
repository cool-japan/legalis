//! Git-style diff interface for statute changes.
//!
//! This module provides a familiar git-like interface for viewing
//! statute diffs, including unified diff format and patch generation.

use crate::{Change, ChangeType, StatuteDiff};

#[cfg(test)]
use crate::ChangeTarget;

/// Git-style unified diff formatter.
pub struct UnifiedDiffFormatter {
    /// Number of context lines before and after changes.
    pub context_lines: usize,
    /// Show color codes in output.
    pub color: bool,
}

impl UnifiedDiffFormatter {
    /// Create a new unified diff formatter with default settings.
    pub fn new() -> Self {
        Self {
            context_lines: 3,
            color: false,
        }
    }

    /// Set the number of context lines.
    pub fn with_context_lines(mut self, lines: usize) -> Self {
        self.context_lines = lines;
        self
    }

    /// Enable or disable color output.
    pub fn with_color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    /// Format a diff as a unified diff (similar to git diff).
    pub fn format(&self, diff: &StatuteDiff) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "diff --statute a/{} b/{}\n",
            diff.statute_id, diff.statute_id
        ));

        if let Some(ref version_info) = diff.version_info
            && let (Some(old_ver), Some(new_ver)) =
                (version_info.old_version, version_info.new_version)
        {
            output.push_str(&format!("index {}..{}\n", old_ver, new_ver));
        }

        output.push_str(&format!("--- a/{}\n", diff.statute_id));
        output.push_str(&format!("+++ b/{}\n", diff.statute_id));

        // Changes
        if !diff.changes.is_empty() {
            output.push_str("@@ Changes @@\n");

            for change in &diff.changes {
                output.push_str(&self.format_change(change));
            }
        }

        // Impact summary
        if !diff.impact.notes.is_empty() {
            output.push('\n');
            for note in &diff.impact.notes {
                output.push_str(&self.colorize(&format!("! {}\n", note), AnsiColor::Yellow));
            }
        }

        output
    }

    fn format_change(&self, change: &Change) -> String {
        let mut output = String::new();

        match change.change_type {
            ChangeType::Added => {
                let line = format!("+{}: {}\n", change.target, change.description);
                output.push_str(&self.colorize(&line, AnsiColor::Green));
                if let Some(ref new_val) = change.new_value {
                    for line in new_val.lines() {
                        output
                            .push_str(&self.colorize(&format!("+  {}\n", line), AnsiColor::Green));
                    }
                }
            }
            ChangeType::Removed => {
                let line = format!("-{}: {}\n", change.target, change.description);
                output.push_str(&self.colorize(&line, AnsiColor::Red));
                if let Some(ref old_val) = change.old_value {
                    for line in old_val.lines() {
                        output.push_str(&self.colorize(&format!("-  {}\n", line), AnsiColor::Red));
                    }
                }
            }
            ChangeType::Modified => {
                output.push_str(&format!(" {}: {}\n", change.target, change.description));
                if let Some(ref old_val) = change.old_value {
                    for line in old_val.lines() {
                        output.push_str(&self.colorize(&format!("-  {}\n", line), AnsiColor::Red));
                    }
                }
                if let Some(ref new_val) = change.new_value {
                    for line in new_val.lines() {
                        output
                            .push_str(&self.colorize(&format!("+  {}\n", line), AnsiColor::Green));
                    }
                }
            }
            ChangeType::Reordered => {
                output.push_str(&self.colorize(
                    &format!("~{}: {}\n", change.target, change.description),
                    AnsiColor::Cyan,
                ));
            }
        }

        output
    }

    fn colorize(&self, text: &str, color: AnsiColor) -> String {
        if self.color {
            format!("{}{}{}", color.code(), text, AnsiColor::Reset.code())
        } else {
            text.to_string()
        }
    }
}

impl Default for UnifiedDiffFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
enum AnsiColor {
    Red,
    Green,
    Yellow,
    Cyan,
    Reset,
}

impl AnsiColor {
    fn code(&self) -> &'static str {
        match self {
            AnsiColor::Red => "\x1b[31m",
            AnsiColor::Green => "\x1b[32m",
            AnsiColor::Yellow => "\x1b[33m",
            AnsiColor::Cyan => "\x1b[36m",
            AnsiColor::Reset => "\x1b[0m",
        }
    }
}

/// Compact diff formatter (similar to git diff --stat).
pub struct CompactDiffFormatter;

impl CompactDiffFormatter {
    /// Create a new compact diff formatter.
    pub fn new() -> Self {
        Self
    }

    /// Format a diff as a compact summary.
    pub fn format(&self, diff: &StatuteDiff) -> String {
        let mut output = String::new();

        output.push_str(&format!("{}\n", diff.statute_id));

        let additions = diff
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Added))
            .count();
        let deletions = diff
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Removed))
            .count();
        let modifications = diff
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Modified))
            .count();

        output.push_str(&format!(
            " {} changes: {} additions(+), {} deletions(-), {} modifications(~)\n",
            diff.changes.len(),
            additions,
            deletions,
            modifications
        ));

        // Show severity
        output.push_str(&format!(" Severity: {:?}\n", diff.impact.severity));

        // Show impact flags
        let mut impacts = Vec::new();
        if diff.impact.affects_eligibility {
            impacts.push("eligibility");
        }
        if diff.impact.affects_outcome {
            impacts.push("outcome");
        }
        if diff.impact.discretion_changed {
            impacts.push("discretion");
        }

        if !impacts.is_empty() {
            output.push_str(&format!(" Impacts: {}\n", impacts.join(", ")));
        }

        output
    }
}

impl Default for CompactDiffFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Short-stat formatter (similar to git diff --shortstat).
pub struct ShortStatFormatter;

impl ShortStatFormatter {
    /// Create a new short-stat formatter.
    pub fn new() -> Self {
        Self
    }

    /// Format a diff as a single-line stat.
    pub fn format(&self, diff: &StatuteDiff) -> String {
        let additions = diff
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Added))
            .count();
        let deletions = diff
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Removed))
            .count();
        let modifications = diff
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Modified))
            .count();

        format!(
            "{} changes, {} insertions(+), {} deletions(-), {} modifications(~)",
            diff.changes.len(),
            additions,
            deletions,
            modifications
        )
    }
}

impl Default for ShortStatFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Name-only formatter (similar to git diff --name-only).
pub struct NameOnlyFormatter;

impl NameOnlyFormatter {
    /// Create a new name-only formatter.
    pub fn new() -> Self {
        Self
    }

    /// Format a diff showing only the statute ID.
    pub fn format(&self, diff: &StatuteDiff) -> String {
        diff.statute_id.clone()
    }
}

impl Default for NameOnlyFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImpactAssessment, Severity, VersionInfo};

    fn test_diff() -> StatuteDiff {
        StatuteDiff {
            statute_id: "test-statute".to_string(),
            version_info: Some(VersionInfo {
                old_version: Some(1),
                new_version: Some(2),
            }),
            changes: vec![
                Change {
                    change_type: ChangeType::Added,
                    target: ChangeTarget::Precondition { index: 1 },
                    description: "Added new precondition".to_string(),
                    old_value: None,
                    new_value: Some("Age >= 18".to_string()),
                },
                Change {
                    change_type: ChangeType::Removed,
                    target: ChangeTarget::Precondition { index: 0 },
                    description: "Removed old precondition".to_string(),
                    old_value: Some("Income <= 5000000".to_string()),
                    new_value: None,
                },
                Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::Title,
                    description: "Title modified".to_string(),
                    old_value: Some("Old Title".to_string()),
                    new_value: Some("New Title".to_string()),
                },
            ],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec!["Eligibility criteria changed".to_string()],
            },
        }
    }

    #[test]
    fn test_unified_diff_formatter() {
        let diff = test_diff();
        let formatter = UnifiedDiffFormatter::new();
        let output = formatter.format(&diff);

        assert!(output.contains("diff --statute"));
        assert!(output.contains("test-statute"));
        assert!(output.contains("index 1..2"));
        assert!(output.contains("+Precondition #2"));
        assert!(output.contains("-Precondition #1"));
    }

    #[test]
    fn test_unified_diff_with_color() {
        let diff = test_diff();
        let formatter = UnifiedDiffFormatter::new().with_color(true);
        let output = formatter.format(&diff);

        // Check for ANSI color codes
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn test_compact_diff_formatter() {
        let diff = test_diff();
        let formatter = CompactDiffFormatter::new();
        let output = formatter.format(&diff);

        assert!(output.contains("test-statute"));
        assert!(output.contains("3 changes"));
        assert!(output.contains("1 additions"));
        assert!(output.contains("1 deletions"));
        assert!(output.contains("1 modifications"));
        assert!(output.contains("Severity: Moderate"));
    }

    #[test]
    fn test_shortstat_formatter() {
        let diff = test_diff();
        let formatter = ShortStatFormatter::new();
        let output = formatter.format(&diff);

        assert!(output.contains("3 changes"));
        assert!(output.contains("1 insertions"));
        assert!(output.contains("1 deletions"));
        assert!(output.contains("1 modifications"));
    }

    #[test]
    fn test_name_only_formatter() {
        let diff = test_diff();
        let formatter = NameOnlyFormatter::new();
        let output = formatter.format(&diff);

        assert_eq!(output, "test-statute");
    }
}
