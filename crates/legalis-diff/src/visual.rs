//! Visual diff reports with charts and graphical representations.
//!
//! This module generates visual representations of statute diffs,
//! including SVG charts, impact graphs, and change timelines.

use crate::{ChangeType, Severity, StatuteDiff};
use std::collections::HashMap;

/// Generates an SVG bar chart showing change distribution.
pub fn generate_change_distribution_chart(diff: &StatuteDiff) -> String {
    let mut counts: HashMap<ChangeType, usize> = HashMap::new();

    for change in &diff.changes {
        *counts.entry(change.change_type).or_insert(0) += 1;
    }

    let width = 600;
    let height = 400;
    let margin = 50;
    let bar_width = 80;
    let max_count = counts.values().max().copied().unwrap_or(1);

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );

    // Title
    svg.push_str(&format!(
        r#"<text x="{}" y="30" font-size="18" font-weight="bold" text-anchor="middle">Change Distribution</text>"#,
        width / 2
    ));

    // Bars
    let change_types = [
        (ChangeType::Added, "Added", "#28a745"),
        (ChangeType::Removed, "Removed", "#dc3545"),
        (ChangeType::Modified, "Modified", "#ffc107"),
        (ChangeType::Reordered, "Reordered", "#17a2b8"),
    ];

    for (i, (change_type, label, color)) in change_types.iter().enumerate() {
        let count = counts.get(change_type).copied().unwrap_or(0);
        let bar_height = if max_count > 0 {
            (count as f64 / max_count as f64) * (height - 2 * margin) as f64
        } else {
            0.0
        };
        let x = margin + i * (bar_width + 20);
        let y = height - margin - bar_height as usize;

        // Bar
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
            x, y, bar_width, bar_height, color
        ));

        // Count label
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="14" font-weight="bold" text-anchor="middle">{}</text>"#,
            x + bar_width / 2,
            y - 5,
            count
        ));

        // X-axis label
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="12" text-anchor="middle">{}</text>"#,
            x + bar_width / 2,
            height - margin + 20,
            label
        ));
    }

    // Y-axis
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2" />"#,
        margin,
        margin,
        margin,
        height - margin
    ));

    // X-axis
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2" />"#,
        margin,
        height - margin,
        width - margin,
        height - margin
    ));

    svg.push_str("</svg>");
    svg
}

/// Generates an SVG severity gauge showing impact level.
pub fn generate_severity_gauge(diff: &StatuteDiff) -> String {
    let width = 300;
    let height = 200;
    let center_x = width / 2;
    let center_y = height - 30;
    let radius = 100;

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );

    // Title
    svg.push_str(&format!(
        r#"<text x="{}" y="20" font-size="16" font-weight="bold" text-anchor="middle">Severity Level</text>"#,
        center_x
    ));

    // Severity arc backgrounds
    let severities = [
        (Severity::None, "#e1e4e8", 0.0),
        (Severity::Minor, "#dbedff", 0.2),
        (Severity::Moderate, "#fff5b1", 0.4),
        (Severity::Major, "#ffeef0", 0.6),
        (Severity::Breaking, "#f8d7da", 0.8),
    ];

    for (_sev, color, start_ratio) in &severities {
        let start_angle = -180.0 + start_ratio * 180.0;
        let end_angle = start_angle + 36.0;

        let arc = create_arc(center_x, center_y, radius, start_angle, end_angle);
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}" stroke="white" stroke-width="2" />"#,
            arc, color
        ));
    }

    // Needle pointing to current severity
    let severity_angle: f64 = match diff.impact.severity {
        Severity::None => -180.0,
        Severity::Minor => -144.0,
        Severity::Moderate => -108.0,
        Severity::Major => -72.0,
        Severity::Breaking => -36.0,
    };

    let needle_end_x = center_x as f64 + (radius as f64 * 0.8) * severity_angle.to_radians().cos();
    let needle_end_y = center_y as f64 + (radius as f64 * 0.8) * severity_angle.to_radians().sin();

    svg.push_str(&format!(
        r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#b60205" stroke-width="4" stroke-linecap="round" />"##,
        center_x, center_y, needle_end_x, needle_end_y
    ));

    svg.push_str(&format!(
        r##"<circle cx="{}" cy="{}" r="8" fill="#b60205" />"##,
        center_x, center_y
    ));

    // Label
    svg.push_str(&format!(
        r#"<text x="{}" y="{}" font-size="14" font-weight="bold" text-anchor="middle">{:?}</text>"#,
        center_x,
        height - 5,
        diff.impact.severity
    ));

    svg.push_str("</svg>");
    svg
}

/// Generates an SVG impact matrix showing what's affected.
pub fn generate_impact_matrix(diff: &StatuteDiff) -> String {
    let width = 400;
    let height = 250;
    let margin = 50;

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );

    // Title
    svg.push_str(&format!(
        r#"<text x="{}" y="30" font-size="18" font-weight="bold" text-anchor="middle">Impact Matrix</text>"#,
        width / 2
    ));

    let impacts = [
        ("Eligibility", diff.impact.affects_eligibility, 70),
        ("Outcome", diff.impact.affects_outcome, 120),
        ("Discretion", diff.impact.discretion_changed, 170),
    ];

    for (label, affected, y) in &impacts {
        let color = if *affected { "#28a745" } else { "#e1e4e8" };
        let icon = if *affected { "✓" } else { "✗" };

        // Box
        svg.push_str(&format!(
            r##"<rect x="{}" y="{}" width="250" height="35" fill="{}" stroke="#24292e" stroke-width="2" rx="5" />"##,
            margin, y, color
        ));

        // Label
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" font-size="16" font-weight="bold" fill="#24292e">{}: {}</text>"##,
            margin + 10,
            y + 23,
            label,
            icon
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Generates a complete visual report combining multiple charts.
pub fn generate_visual_report(diff: &StatuteDiff) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str(&format!(
        "<title>Visual Diff Report: {}</title>\n",
        diff.statute_id
    ));
    html.push_str(
        r#"
<style>
    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        padding: 20px;
        background: #f6f8fa;
    }
    .container {
        max-width: 1200px;
        margin: 0 auto;
        background: white;
        padding: 30px;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
    h1 {
        color: #24292e;
        border-bottom: 2px solid #e1e4e8;
        padding-bottom: 16px;
    }
    .chart-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 30px;
        margin: 30px 0;
    }
    .chart {
        background: #fff;
        padding: 20px;
        border: 1px solid #e1e4e8;
        border-radius: 6px;
    }
    .summary {
        background: #f6f8fa;
        padding: 20px;
        border-radius: 6px;
        margin: 20px 0;
    }
    .stat {
        display: inline-block;
        margin: 10px 20px 10px 0;
    }
    .stat-label {
        font-weight: 600;
        color: #586069;
    }
    .stat-value {
        font-size: 24px;
        font-weight: bold;
        color: #24292e;
    }
</style>
"#,
    );
    html.push_str("</head>\n<body>\n");
    html.push_str("<div class=\"container\">\n");

    // Header
    html.push_str(&format!(
        "<h1>Visual Diff Report: {}</h1>\n",
        diff.statute_id
    ));

    // Summary stats
    html.push_str("<div class=\"summary\">\n");
    html.push_str("<h2>Summary</h2>\n");
    html.push_str(&format!(
        "<div class=\"stat\"><span class=\"stat-label\">Total Changes:</span> <span class=\"stat-value\">{}</span></div>\n",
        diff.changes.len()
    ));
    html.push_str(&format!(
        "<div class=\"stat\"><span class=\"stat-label\">Severity:</span> <span class=\"stat-value\">{:?}</span></div>\n",
        diff.impact.severity
    ));
    html.push_str("</div>\n");

    // Charts
    html.push_str("<div class=\"chart-grid\">\n");

    html.push_str("<div class=\"chart\">\n");
    html.push_str(&generate_change_distribution_chart(diff));
    html.push_str("</div>\n");

    html.push_str("<div class=\"chart\">\n");
    html.push_str(&generate_severity_gauge(diff));
    html.push_str("</div>\n");

    html.push_str("<div class=\"chart\">\n");
    html.push_str(&generate_impact_matrix(diff));
    html.push_str("</div>\n");

    html.push_str("</div>\n");

    // Impact notes
    if !diff.impact.notes.is_empty() {
        html.push_str("<div class=\"summary\">\n");
        html.push_str("<h2>Impact Notes</h2>\n<ul>\n");
        for note in &diff.impact.notes {
            html.push_str(&format!("<li>{}</li>\n", note));
        }
        html.push_str("</ul>\n</div>\n");
    }

    html.push_str("</div>\n</body>\n</html>");
    html
}

/// Creates an SVG arc path.
fn create_arc(cx: usize, cy: usize, radius: usize, start_angle: f64, end_angle: f64) -> String {
    let start_x = cx as f64 + radius as f64 * start_angle.to_radians().cos();
    let start_y = cy as f64 + radius as f64 * start_angle.to_radians().sin();
    let end_x = cx as f64 + radius as f64 * end_angle.to_radians().cos();
    let end_y = cy as f64 + radius as f64 * end_angle.to_radians().sin();

    let large_arc = if end_angle - start_angle > 180.0 {
        1
    } else {
        0
    };

    format!(
        "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
        cx, cy, start_x, start_y, radius, radius, large_arc, end_x, end_y
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Change, ChangeTarget, ImpactAssessment, VersionInfo};

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
    fn test_generate_change_distribution_chart() {
        let diff = test_diff();
        let svg = generate_change_distribution_chart(&diff);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Change Distribution"));
        assert!(svg.contains("Added"));
        assert!(svg.contains("Modified"));
        assert!(svg.contains("Removed"));
    }

    #[test]
    fn test_generate_severity_gauge() {
        let diff = test_diff();
        let svg = generate_severity_gauge(&diff);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Severity Level"));
        assert!(svg.contains("Major"));
    }

    #[test]
    fn test_generate_impact_matrix() {
        let diff = test_diff();
        let svg = generate_impact_matrix(&diff);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Impact Matrix"));
        assert!(svg.contains("Eligibility"));
        assert!(svg.contains("Outcome"));
        assert!(svg.contains("Discretion"));
    }

    #[test]
    fn test_generate_visual_report() {
        let diff = test_diff();
        let html = generate_visual_report(&diff);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Visual Diff Report"));
        assert!(html.contains("test-statute"));
        assert!(html.contains("Summary"));
        assert!(html.contains("Total Changes"));
    }

    #[test]
    fn test_create_arc() {
        let arc = create_arc(100, 100, 50, 0.0, 90.0);
        assert!(arc.contains("M 100 100"));
        assert!(arc.contains("A 50 50"));
    }
}
