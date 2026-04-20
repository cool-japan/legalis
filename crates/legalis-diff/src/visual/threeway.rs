//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ChangeType, StatuteDiff};

/// Generates a three-way diff visualization for merge scenarios.
///
/// This displays differences between a base version and two modified versions
/// (typically "ours" and "theirs"), useful for resolving conflicts.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, visual::generate_three_way_diff};
///
/// let base = Statute::new("law", "Base Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut ours = base.clone();
/// ours.title = "Our Title".to_string();
/// let mut theirs = base.clone();
/// theirs.title = "Their Title".to_string();
///
/// let diff_ours = diff(&base, &ours).unwrap();
/// let diff_theirs = diff(&base, &theirs).unwrap();
/// let html = generate_three_way_diff(&diff_ours, &diff_theirs);
///
/// assert!(html.contains("Three-Way Diff"));
/// ```
pub fn generate_three_way_diff(diff_ours: &StatuteDiff, diff_theirs: &StatuteDiff) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!(
        "<title>Three-Way Diff: {}</title>\n",
        diff_ours.statute_id
    ));
    html.push_str(
        r#"
<style>
    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        margin: 0;
        padding: 20px;
        background: #f6f8fa;
    }
    .container {
        max-width: 1600px;
        margin: 0 auto;
        background: white;
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
    .header {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 25px;
        text-align: center;
    }
    .header h1 {
        margin: 0;
        font-size: 28px;
    }
    .header .subtitle {
        margin-top: 8px;
        opacity: 0.9;
        font-size: 14px;
    }
    .three-way-view {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        gap: 1px;
        background: #e1e4e8;
    }
    .column {
        background: white;
        padding: 20px;
    }
    .column-header {
        font-size: 18px;
        font-weight: bold;
        padding: 15px;
        text-align: center;
        border-bottom: 3px solid;
        margin-bottom: 20px;
    }
    .column-header.base {
        background: #f6f8fa;
        border-color: #8b949e;
        color: #24292e;
    }
    .column-header.ours {
        background: #e6ffed;
        border-color: #28a745;
        color: #155724;
    }
    .column-header.theirs {
        background: #fff5b1;
        border-color: #ffc107;
        color: #856404;
    }
    .change-item {
        padding: 12px;
        margin-bottom: 12px;
        border-radius: 6px;
        border-left: 4px solid;
        background: #f6f8fa;
        font-size: 14px;
    }
    .change-item.added {
        border-left-color: #28a745;
        background: #d4edda;
    }
    .change-item.removed {
        border-left-color: #dc3545;
        background: #f8d7da;
    }
    .change-item.modified {
        border-left-color: #ffc107;
        background: #fff3cd;
    }
    .change-item.conflict {
        border-left-color: #d6336c;
        background: #ffeef0;
    }
    .change-type-badge {
        display: inline-block;
        padding: 3px 8px;
        border-radius: 3px;
        font-size: 11px;
        font-weight: bold;
        text-transform: uppercase;
        margin-bottom: 5px;
    }
    .change-type-badge.added {
        background: #28a745;
        color: white;
    }
    .change-type-badge.removed {
        background: #dc3545;
        color: white;
    }
    .change-type-badge.modified {
        background: #ffc107;
        color: #24292e;
    }
    .change-type-badge.conflict {
        background: #d6336c;
        color: white;
    }
    .change-target {
        font-weight: bold;
        margin: 5px 0;
    }
    .change-value {
        font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
        font-size: 12px;
        padding: 8px;
        background: white;
        border-radius: 4px;
        margin-top: 5px;
        word-break: break-all;
    }
    .conflict-indicator {
        background: #ffeef0;
        border: 2px solid #d6336c;
        border-radius: 6px;
        padding: 15px;
        margin: 10px 0;
        text-align: center;
        font-weight: bold;
        color: #d6336c;
    }
    .stats-bar {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        gap: 1px;
        background: #e1e4e8;
        padding: 0;
    }
    .stat-column {
        background: white;
        padding: 20px;
        text-align: center;
    }
    .stat-value {
        font-size: 32px;
        font-weight: bold;
        color: #0366d6;
    }
    .stat-label {
        font-size: 14px;
        color: #586069;
        margin-top: 5px;
    }
    .legend {
        padding: 20px;
        background: #f6f8fa;
        display: flex;
        gap: 20px;
        justify-content: center;
        flex-wrap: wrap;
    }
    .legend-item {
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .legend-color {
        width: 20px;
        height: 20px;
        border-radius: 4px;
    }
</style>
"#,
    );
    html.push_str("</head>\n<body>\n");
    html.push_str("<div class=\"container\">\n");
    html.push_str("<div class=\"header\">\n");
    html.push_str("<h1>Three-Way Diff Viewer</h1>\n");
    html.push_str(
        &format!(
            "<div class=\"subtitle\">Statute: {} | Comparing BASE with YOUR changes and THEIR changes</div>\n",
            diff_ours.statute_id
        ),
    );
    html.push_str("</div>\n");
    html.push_str("<div class=\"legend\">\n");
    html.push_str(
        "<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #28a745;\"></div><span>Added</span></div>\n",
    );
    html.push_str(
        "<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #dc3545;\"></div><span>Removed</span></div>\n",
    );
    html.push_str(
        "<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #ffc107;\"></div><span>Modified</span></div>\n",
    );
    html.push_str(
        "<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #d6336c;\"></div><span>Conflict</span></div>\n",
    );
    html.push_str("</div>\n");
    html.push_str("<div class=\"stats-bar\">\n");
    html.push_str("<div class=\"stat-column\">\n");
    html.push_str("<div class=\"stat-value\">BASE</div>\n");
    html.push_str("<div class=\"stat-label\">Original Version</div>\n");
    html.push_str("</div>\n");
    html.push_str("<div class=\"stat-column\">\n");
    html.push_str(&format!(
        "<div class=\"stat-value\">{}</div>\n",
        diff_ours.changes.len()
    ));
    html.push_str("<div class=\"stat-label\">Your Changes</div>\n");
    html.push_str("</div>\n");
    html.push_str("<div class=\"stat-column\">\n");
    html.push_str(&format!(
        "<div class=\"stat-value\">{}</div>\n",
        diff_theirs.changes.len()
    ));
    html.push_str("<div class=\"stat-label\">Their Changes</div>\n");
    html.push_str("</div>\n");
    html.push_str("</div>\n");
    let conflicts = detect_conflicts(diff_ours, diff_theirs);
    if !conflicts.is_empty() {
        html.push_str("<div class=\"conflict-indicator\">\n");
        html.push_str(&format!(
            "⚠️ {} conflict(s) detected! Review carefully before merging.\n",
            conflicts.len()
        ));
        html.push_str("</div>\n");
    }
    html.push_str("<div class=\"three-way-view\">\n");
    html.push_str("<div class=\"column\">\n");
    html.push_str("<div class=\"column-header base\">BASE</div>\n");
    let mut all_targets = std::collections::HashSet::new();
    for change in &diff_ours.changes {
        all_targets.insert(format!("{}", change.target));
    }
    for change in &diff_theirs.changes {
        all_targets.insert(format!("{}", change.target));
    }
    for target in &all_targets {
        html.push_str("<div class=\"change-item\">\n");
        html.push_str(&format!("<div class=\"change-target\">{}</div>\n", target));
        html.push_str("</div>\n");
    }
    html.push_str("</div>\n");
    html.push_str("<div class=\"column\">\n");
    html.push_str("<div class=\"column-header ours\">YOURS</div>\n");
    for change in &diff_ours.changes {
        let is_conflict = conflicts.contains(&format!("{}", change.target));
        let class = if is_conflict {
            "conflict"
        } else {
            match change.change_type {
                ChangeType::Added => "added",
                ChangeType::Removed => "removed",
                ChangeType::Modified => "modified",
                _ => "",
            }
        };
        let badge_class = if is_conflict { "conflict" } else { class };
        html.push_str(&format!("<div class=\"change-item {}\">\n", class));
        html.push_str(&format!(
            "<span class=\"change-type-badge {}\">{}</span>\n",
            badge_class,
            if is_conflict {
                "CONFLICT"
            } else {
                match change.change_type {
                    ChangeType::Added => "ADDED",
                    ChangeType::Removed => "REMOVED",
                    ChangeType::Modified => "MODIFIED",
                    _ => "CHANGED",
                }
            }
        ));
        html.push_str(&format!(
            "<div class=\"change-target\">{}</div>\n",
            change.target
        ));
        html.push_str(&format!("<div>{}</div>\n", change.description));
        if let Some(new_val) = &change.new_value {
            html.push_str(&format!("<div class=\"change-value\">{}</div>\n", new_val));
        }
        html.push_str("</div>\n");
    }
    html.push_str("</div>\n");
    html.push_str("<div class=\"column\">\n");
    html.push_str("<div class=\"column-header theirs\">THEIRS</div>\n");
    for change in &diff_theirs.changes {
        let is_conflict = conflicts.contains(&format!("{}", change.target));
        let class = if is_conflict {
            "conflict"
        } else {
            match change.change_type {
                ChangeType::Added => "added",
                ChangeType::Removed => "removed",
                ChangeType::Modified => "modified",
                _ => "",
            }
        };
        let badge_class = if is_conflict { "conflict" } else { class };
        html.push_str(&format!("<div class=\"change-item {}\">\n", class));
        html.push_str(&format!(
            "<span class=\"change-type-badge {}\">{}</span>\n",
            badge_class,
            if is_conflict {
                "CONFLICT"
            } else {
                match change.change_type {
                    ChangeType::Added => "ADDED",
                    ChangeType::Removed => "REMOVED",
                    ChangeType::Modified => "MODIFIED",
                    _ => "CHANGED",
                }
            }
        ));
        html.push_str(&format!(
            "<div class=\"change-target\">{}</div>\n",
            change.target
        ));
        html.push_str(&format!("<div>{}</div>\n", change.description));
        if let Some(new_val) = &change.new_value {
            html.push_str(&format!("<div class=\"change-value\">{}</div>\n", new_val));
        }
        html.push_str("</div>\n");
    }
    html.push_str("</div>\n");
    html.push_str("</div>\n");
    html.push_str("</div>\n");
    html.push_str("</body>\n</html>");
    html
}
/// Detects conflicts between two diffs.
///
/// A conflict occurs when both diffs modify the same target in different ways.
fn detect_conflicts(diff_ours: &StatuteDiff, diff_theirs: &StatuteDiff) -> Vec<String> {
    let mut conflicts = Vec::new();
    for change_ours in &diff_ours.changes {
        for change_theirs in &diff_theirs.changes {
            if change_ours.target == change_theirs.target
                && change_ours.new_value != change_theirs.new_value
            {
                conflicts.push(format!("{}", change_ours.target));
            }
        }
    }
    conflicts
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Change, ChangeTarget, ImpactAssessment, Severity, VersionInfo};
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
                    target: ChangeTarget::Precondition { index: 0 },
                    description: "Added precondition".to_string(),
                    old_value: None,
                    new_value: Some("Age >= 18".to_string()),
                },
                Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::Title,
                    description: "Title modified".to_string(),
                    old_value: Some("Old".to_string()),
                    new_value: Some("New".to_string()),
                },
                Change {
                    change_type: ChangeType::Removed,
                    target: ChangeTarget::Precondition { index: 1 },
                    description: "Removed precondition".to_string(),
                    old_value: Some("Income <= 5000000".to_string()),
                    new_value: None,
                },
            ],
            impact: ImpactAssessment {
                severity: Severity::Major,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec!["Significant eligibility changes".to_string()],
            },
        }
    }
    #[test]
    fn test_generate_three_way_diff() {
        let diff_ours = test_diff();
        let diff_theirs = test_diff();
        let html = generate_three_way_diff(&diff_ours, &diff_theirs);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Three-Way Diff"));
        assert!(html.contains("test-statute"));
        assert!(html.contains("BASE"));
        assert!(html.contains("YOURS"));
        assert!(html.contains("THEIRS"));
    }
    #[test]
    fn test_detect_conflicts() {
        let diff_ours = test_diff();
        let mut diff_theirs = test_diff();
        if let Some(change) = diff_theirs.changes.first_mut() {
            change.new_value = Some("Different value".to_string());
        }
        let conflicts = detect_conflicts(&diff_ours, &diff_theirs);
        assert!(!conflicts.is_empty());
    }
    #[test]
    fn test_detect_no_conflicts() {
        let diff_ours = test_diff();
        let diff_theirs = test_diff();
        let conflicts = detect_conflicts(&diff_ours, &diff_theirs);
        assert!(conflicts.is_empty());
    }
}
