//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

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
    svg.push_str(
        &format!(
            r#"<text x="{}" y="30" font-size="18" font-weight="bold" text-anchor="middle">Change Distribution</text>"#,
            width / 2
        ),
    );
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
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
            x, y, bar_width, bar_height, color
        ));
        svg.push_str(
            &format!(
                r#"<text x="{}" y="{}" font-size="14" font-weight="bold" text-anchor="middle">{}</text>"#,
                x + bar_width / 2, y - 5, count
            ),
        );
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="12" text-anchor="middle">{}</text>"#,
            x + bar_width / 2,
            height - margin + 20,
            label
        ));
    }
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2" />"#,
        margin,
        margin,
        margin,
        height - margin
    ));
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
    svg.push_str(
        &format!(
            r#"<text x="{}" y="20" font-size="16" font-weight="bold" text-anchor="middle">Severity Level</text>"#,
            center_x
        ),
    );
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
    let severity_angle: f64 = match diff.impact.severity {
        Severity::None => -180.0,
        Severity::Minor => -144.0,
        Severity::Moderate => -108.0,
        Severity::Major => -72.0,
        Severity::Breaking => -36.0,
    };
    let needle_end_x = center_x as f64 + (radius as f64 * 0.8) * severity_angle.to_radians().cos();
    let needle_end_y = center_y as f64 + (radius as f64 * 0.8) * severity_angle.to_radians().sin();
    svg.push_str(
        &format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#b60205" stroke-width="4" stroke-linecap="round" />"##,
            center_x, center_y, needle_end_x, needle_end_y
        ),
    );
    svg.push_str(&format!(
        r##"<circle cx="{}" cy="{}" r="8" fill="#b60205" />"##,
        center_x, center_y
    ));
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
    svg.push_str(
        &format!(
            r#"<text x="{}" y="30" font-size="18" font-weight="bold" text-anchor="middle">Impact Matrix</text>"#,
            width / 2
        ),
    );
    let impacts = [
        ("Eligibility", diff.impact.affects_eligibility, 70),
        ("Outcome", diff.impact.affects_outcome, 120),
        ("Discretion", diff.impact.discretion_changed, 170),
    ];
    for (label, affected, y) in &impacts {
        let color = if *affected { "#28a745" } else { "#e1e4e8" };
        let icon = if *affected { "✓" } else { "✗" };
        svg.push_str(
            &format!(
                r##"<rect x="{}" y="{}" width="250" height="35" fill="{}" stroke="#24292e" stroke-width="2" rx="5" />"##,
                margin, y, color
            ),
        );
        svg.push_str(
            &format!(
                r##"<text x="{}" y="{}" font-size="16" font-weight="bold" fill="#24292e">{}: {}</text>"##,
                margin + 10, y + 23, label, icon
            ),
        );
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
    html.push_str(&format!(
        "<h1>Visual Diff Report: {}</h1>\n",
        diff.statute_id
    ));
    html.push_str("<div class=\"summary\">\n");
    html.push_str("<h2>Summary</h2>\n");
    html.push_str(
        &format!(
            "<div class=\"stat\"><span class=\"stat-label\">Total Changes:</span> <span class=\"stat-value\">{}</span></div>\n",
            diff.changes.len()
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"stat\"><span class=\"stat-label\">Severity:</span> <span class=\"stat-value\">{:?}</span></div>\n",
            diff.impact.severity
        ),
    );
    html.push_str("</div>\n");
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
pub(super) fn create_arc(
    cx: usize,
    cy: usize,
    radius: usize,
    start_angle: f64,
    end_angle: f64,
) -> String {
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
/// Generates an interactive HTML diff viewer with expandable sections.
///
/// This viewer includes:
/// - Collapsible change sections
/// - Interactive navigation
/// - Tooltips for detailed information
/// - Search and filter functionality
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, visual::generate_interactive_diff_viewer};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let html = generate_interactive_diff_viewer(&diff_result);
///
/// assert!(html.contains("<!DOCTYPE html>"));
/// assert!(html.contains("Interactive Diff Viewer"));
/// ```
pub fn generate_interactive_diff_viewer(diff: &StatuteDiff) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!(
        "<title>Interactive Diff: {}</title>\n",
        diff.statute_id
    ));
    html.push_str(
        r#"
<style>
    * {
        box-sizing: border-box;
    }
    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
        margin: 0;
        padding: 0;
        background: #f6f8fa;
        color: #24292e;
    }
    .header {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 30px;
        box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    }
    .header h1 {
        margin: 0 0 10px 0;
        font-size: 32px;
    }
    .header .subtitle {
        opacity: 0.9;
        font-size: 16px;
    }
    .container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 30px;
    }
    .controls {
        background: white;
        padding: 20px;
        border-radius: 8px;
        margin-bottom: 20px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        display: flex;
        gap: 15px;
        align-items: center;
        flex-wrap: wrap;
    }
    .search-box {
        flex: 1;
        min-width: 200px;
        padding: 10px 15px;
        border: 2px solid #e1e4e8;
        border-radius: 6px;
        font-size: 14px;
    }
    .filter-btn {
        padding: 10px 20px;
        border: 2px solid #e1e4e8;
        background: white;
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        transition: all 0.2s;
    }
    .filter-btn:hover {
        background: #f6f8fa;
        border-color: #0366d6;
    }
    .filter-btn.active {
        background: #0366d6;
        color: white;
        border-color: #0366d6;
    }
    .stats-bar {
        background: white;
        padding: 20px;
        border-radius: 8px;
        margin-bottom: 20px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 20px;
    }
    .stat {
        text-align: center;
    }
    .stat-value {
        font-size: 36px;
        font-weight: bold;
        color: #0366d6;
    }
    .stat-label {
        font-size: 14px;
        color: #586069;
        margin-top: 5px;
    }
    .change-card {
        background: white;
        padding: 20px;
        border-radius: 8px;
        margin-bottom: 15px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        border-left: 4px solid #e1e4e8;
        transition: all 0.3s ease;
    }
    .change-card:hover {
        box-shadow: 0 4px 8px rgba(0,0,0,0.15);
        transform: translateY(-2px);
    }
    .change-card.added {
        border-left-color: #28a745;
    }
    .change-card.removed {
        border-left-color: #dc3545;
    }
    .change-card.modified {
        border-left-color: #ffc107;
    }
    .change-card.reordered {
        border-left-color: #17a2b8;
    }
    .change-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        cursor: pointer;
        user-select: none;
    }
    .change-type {
        display: inline-block;
        padding: 4px 12px;
        border-radius: 4px;
        font-size: 12px;
        font-weight: bold;
        text-transform: uppercase;
    }
    .change-type.added {
        background: #d4edda;
        color: #155724;
    }
    .change-type.removed {
        background: #f8d7da;
        color: #721c24;
    }
    .change-type.modified {
        background: #fff3cd;
        color: #856404;
    }
    .change-type.reordered {
        background: #d1ecf1;
        color: #0c5460;
    }
    .change-title {
        font-weight: bold;
        font-size: 16px;
        margin: 10px 0;
    }
    .change-details {
        max-height: 0;
        overflow: hidden;
        transition: max-height 0.3s ease;
    }
    .change-details.expanded {
        max-height: 1000px;
    }
    .change-content {
        padding-top: 15px;
        border-top: 1px solid #e1e4e8;
        margin-top: 15px;
    }
    .value-diff {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 15px;
        margin-top: 10px;
    }
    .old-value, .new-value {
        padding: 10px;
        border-radius: 4px;
        font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
        font-size: 13px;
        white-space: pre-wrap;
        word-break: break-all;
    }
    .old-value {
        background: #ffebe9;
        border-left: 3px solid #dc3545;
    }
    .new-value {
        background: #e6ffed;
        border-left: 3px solid #28a745;
    }
    .label {
        font-weight: bold;
        font-size: 12px;
        margin-bottom: 5px;
        color: #586069;
    }
    .expand-icon {
        transition: transform 0.3s ease;
        font-size: 20px;
    }
    .expand-icon.expanded {
        transform: rotate(180deg);
    }
    .tooltip {
        position: relative;
        display: inline-block;
        border-bottom: 1px dotted #0366d6;
        cursor: help;
    }
    .tooltip .tooltiptext {
        visibility: hidden;
        background-color: #24292e;
        color: #fff;
        text-align: center;
        border-radius: 6px;
        padding: 8px 12px;
        position: absolute;
        z-index: 1;
        bottom: 125%;
        left: 50%;
        transform: translateX(-50%);
        opacity: 0;
        transition: opacity 0.3s;
        font-size: 12px;
        white-space: nowrap;
    }
    .tooltip:hover .tooltiptext {
        visibility: visible;
        opacity: 1;
    }
    .impact-badge {
        display: inline-block;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 11px;
        font-weight: bold;
        margin-left: 5px;
    }
    .impact-badge.eligibility {
        background: #fff3cd;
        color: #856404;
    }
    .impact-badge.outcome {
        background: #f8d7da;
        color: #721c24;
    }
    .impact-badge.discretion {
        background: #d1ecf1;
        color: #0c5460;
    }
    .no-results {
        text-align: center;
        padding: 40px;
        color: #586069;
        font-size: 16px;
    }
</style>
"#,
    );
    html.push_str(
        r#"
<script>
document.addEventListener('DOMContentLoaded', function() {
    // Toggle change details
    const headers = document.querySelectorAll('.change-header');
    headers.forEach(header => {
        header.addEventListener('click', function() {
            const details = this.nextElementSibling;
            const icon = this.querySelector('.expand-icon');
            details.classList.toggle('expanded');
            icon.classList.toggle('expanded');
        });
    });

    // Search functionality
    const searchBox = document.getElementById('searchBox');
    searchBox.addEventListener('input', filterChanges);

    // Filter buttons
    const filterBtns = document.querySelectorAll('.filter-btn');
    filterBtns.forEach(btn => {
        btn.addEventListener('click', function() {
            this.classList.toggle('active');
            filterChanges();
        });
    });

    function filterChanges() {
        const searchTerm = searchBox.value.toLowerCase();
        const activeFilters = Array.from(document.querySelectorAll('.filter-btn.active'))
            .map(btn => btn.dataset.type);

        const cards = document.querySelectorAll('.change-card');
        let visibleCount = 0;

        cards.forEach(card => {
            const text = card.textContent.toLowerCase();
            const type = card.dataset.type;

            const matchesSearch = searchTerm === '' || text.includes(searchTerm);
            const matchesFilter = activeFilters.length === 0 || activeFilters.includes(type);

            if (matchesSearch && matchesFilter) {
                card.style.display = 'block';
                visibleCount++;
            } else {
                card.style.display = 'none';
            }
        });

        // Show/hide no results message
        const noResults = document.getElementById('noResults');
        if (visibleCount === 0) {
            noResults.style.display = 'block';
        } else {
            noResults.style.display = 'none';
        }
    }

    // Expand all
    document.getElementById('expandAll').addEventListener('click', function() {
        document.querySelectorAll('.change-details').forEach(d => d.classList.add('expanded'));
        document.querySelectorAll('.expand-icon').forEach(i => i.classList.add('expanded'));
    });

    // Collapse all
    document.getElementById('collapseAll').addEventListener('click', function() {
        document.querySelectorAll('.change-details').forEach(d => d.classList.remove('expanded'));
        document.querySelectorAll('.expand-icon').forEach(i => i.classList.remove('expanded'));
    });
});
</script>
"#,
    );
    html.push_str("</head>\n<body>\n");
    html.push_str("<div class=\"header\">\n");
    html.push_str("<h1>Interactive Diff Viewer</h1>\n");
    html.push_str(&format!(
        "<div class=\"subtitle\">Statute: {} | Total Changes: {} | Severity: {:?}</div>\n",
        diff.statute_id,
        diff.changes.len(),
        diff.impact.severity
    ));
    html.push_str("</div>\n");
    html.push_str("<div class=\"container\">\n");
    let added_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Added)
        .count();
    let removed_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Removed)
        .count();
    let modified_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Modified)
        .count();
    let reordered_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Reordered)
        .count();
    html.push_str("<div class=\"stats-bar\">\n");
    html.push_str(
        &format!(
            "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Added</div></div>\n",
            added_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Removed</div></div>\n",
            removed_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Modified</div></div>\n",
            modified_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Reordered</div></div>\n",
            reordered_count
        ),
    );
    html.push_str("</div>\n");
    html.push_str("<div class=\"controls\">\n");
    html.push_str(
        "<input type=\"text\" id=\"searchBox\" class=\"search-box\" placeholder=\"Search changes...\">\n",
    );
    html.push_str("<button class=\"filter-btn\" data-type=\"Added\">Added</button>\n");
    html.push_str("<button class=\"filter-btn\" data-type=\"Removed\">Removed</button>\n");
    html.push_str("<button class=\"filter-btn\" data-type=\"Modified\">Modified</button>\n");
    html.push_str("<button class=\"filter-btn\" data-type=\"Reordered\">Reordered</button>\n");
    html.push_str("<button id=\"expandAll\" class=\"filter-btn\">Expand All</button>\n");
    html.push_str("<button id=\"collapseAll\" class=\"filter-btn\">Collapse All</button>\n");
    html.push_str("</div>\n");
    for (i, change) in diff.changes.iter().enumerate() {
        let change_type_str = format!("{:?}", change.change_type).to_lowercase();
        let change_type_display = format!("{:?}", change.change_type);
        html.push_str(&format!(
            "<div class=\"change-card {}\" data-type=\"{:?}\" id=\"change-{}\">\n",
            change_type_str, change.change_type, i
        ));
        html.push_str("<div class=\"change-header\">\n");
        html.push_str("<div>\n");
        html.push_str(&format!(
            "<span class=\"change-type {}\">{}</span>\n",
            change_type_str, change_type_display
        ));
        if matches!(change.target, crate::ChangeTarget::Precondition { .. })
            && diff.impact.affects_eligibility
        {
            html.push_str("<span class=\"impact-badge eligibility\">Affects Eligibility</span>\n");
        }
        if matches!(change.target, crate::ChangeTarget::Effect) && diff.impact.affects_outcome {
            html.push_str("<span class=\"impact-badge outcome\">Affects Outcome</span>\n");
        }
        if matches!(change.target, crate::ChangeTarget::DiscretionLogic)
            && diff.impact.discretion_changed
        {
            html.push_str("<span class=\"impact-badge discretion\">Discretion Changed</span>\n");
        }
        html.push_str(&format!(
            "<div class=\"change-title\">{}</div>\n",
            change.target
        ));
        html.push_str("</div>\n");
        html.push_str("<div class=\"expand-icon\">▼</div>\n");
        html.push_str("</div>\n");
        html.push_str("<div class=\"change-details\">\n");
        html.push_str("<div class=\"change-content\">\n");
        html.push_str(&format!("<p>{}</p>\n", change.description));
        if change.old_value.is_some() || change.new_value.is_some() {
            html.push_str("<div class=\"value-diff\">\n");
            if let Some(old) = &change.old_value {
                html.push_str("<div>\n");
                html.push_str("<div class=\"label\">OLD VALUE</div>\n");
                html.push_str(&format!("<div class=\"old-value\">{}</div>\n", old));
                html.push_str("</div>\n");
            }
            if let Some(new) = &change.new_value {
                html.push_str("<div>\n");
                html.push_str("<div class=\"label\">NEW VALUE</div>\n");
                html.push_str(&format!("<div class=\"new-value\">{}</div>\n", new));
                html.push_str("</div>\n");
            }
            html.push_str("</div>\n");
        }
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");
    }
    html.push_str("<div id=\"noResults\" class=\"no-results\" style=\"display: none;\">\n");
    html.push_str("No changes match your search criteria.\n");
    html.push_str("</div>\n");
    html.push_str("</div>\n");
    html.push_str("</body>\n</html>");
    html
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

    #[test]
    fn test_generate_interactive_diff_viewer() {
        let diff = test_diff();
        let html = generate_interactive_diff_viewer(&diff);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Interactive Diff Viewer"));
        assert!(html.contains("test-statute"));
        assert!(html.contains("searchBox"));
        assert!(html.contains("expandAll"));
        assert!(html.contains("collapseAll"));
    }
}
