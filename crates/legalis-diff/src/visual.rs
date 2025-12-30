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

    // CSS Styles
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

    // JavaScript
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

    // Header
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

    // Stats bar
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
    html.push_str(&format!(
        "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Added</div></div>\n",
        added_count
    ));
    html.push_str(&format!(
        "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Removed</div></div>\n",
        removed_count
    ));
    html.push_str(&format!(
        "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Modified</div></div>\n",
        modified_count
    ));
    html.push_str(&format!(
        "<div class=\"stat\"><div class=\"stat-value\">{}</div><div class=\"stat-label\">Reordered</div></div>\n",
        reordered_count
    ));
    html.push_str("</div>\n");

    // Controls
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

    // Change cards
    for (i, change) in diff.changes.iter().enumerate() {
        let change_type_str = format!("{:?}", change.change_type).to_lowercase();
        let change_type_display = format!("{:?}", change.change_type);

        html.push_str(&format!(
            "<div class=\"change-card {}\" data-type=\"{:?}\" id=\"change-{}\">\n",
            change_type_str, change.change_type, i
        ));

        // Header
        html.push_str("<div class=\"change-header\">\n");
        html.push_str("<div>\n");
        html.push_str(&format!(
            "<span class=\"change-type {}\">{}</span>\n",
            change_type_str, change_type_display
        ));

        // Impact badges
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

        // Details
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

    // No results message
    html.push_str("<div id=\"noResults\" class=\"no-results\" style=\"display: none;\">\n");
    html.push_str("No changes match your search criteria.\n");
    html.push_str("</div>\n");

    html.push_str("</div>\n");
    html.push_str("</body>\n</html>");
    html
}

/// Generates syntax-highlighted diff output in HTML format.
///
/// This function produces a side-by-side view with syntax highlighting
/// similar to GitHub's diff view.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, visual::generate_syntax_highlighted_diff};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
/// let mut new = old.clone();
/// new.preconditions[0] = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 60,
/// };
///
/// let diff_result = diff(&old, &new).unwrap();
/// let html = generate_syntax_highlighted_diff(&diff_result);
///
/// assert!(html.contains("Syntax Highlighted Diff"));
/// ```
pub fn generate_syntax_highlighted_diff(diff: &StatuteDiff) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str(&format!(
        "<title>Syntax Highlighted Diff: {}</title>\n",
        diff.statute_id
    ));

    html.push_str(
        r#"
<style>
    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        margin: 0;
        padding: 20px;
        background: #0d1117;
        color: #c9d1d9;
    }
    .container {
        max-width: 1400px;
        margin: 0 auto;
        background: #161b22;
        border-radius: 6px;
        overflow: hidden;
    }
    .header {
        background: #21262d;
        padding: 16px 20px;
        border-bottom: 1px solid #30363d;
    }
    .header h1 {
        margin: 0;
        font-size: 20px;
        color: #f0f6fc;
    }
    .diff-view {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1px;
        background: #30363d;
    }
    .diff-side {
        background: #0d1117;
        padding: 20px;
        font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
        font-size: 13px;
        line-height: 1.6;
        overflow-x: auto;
    }
    .diff-side.old {
        border-right: 1px solid #30363d;
    }
    .side-header {
        font-weight: bold;
        color: #8b949e;
        margin-bottom: 15px;
        padding-bottom: 10px;
        border-bottom: 2px solid #30363d;
    }
    .line {
        display: flex;
        padding: 2px 0;
    }
    .line-number {
        width: 40px;
        color: #6e7681;
        text-align: right;
        padding-right: 10px;
        user-select: none;
    }
    .line-content {
        flex: 1;
        white-space: pre-wrap;
        word-break: break-all;
    }
    .line.added {
        background: rgba(46, 160, 67, 0.15);
    }
    .line.added .line-content {
        color: #3fb950;
    }
    .line.removed {
        background: rgba(248, 81, 73, 0.15);
    }
    .line.removed .line-content {
        color: #f85149;
    }
    .line.modified {
        background: rgba(187, 128, 9, 0.15);
    }
    .line.modified .line-content {
        color: #d29922;
    }
    .keyword {
        color: #ff7b72;
    }
    .string {
        color: #a5d6ff;
    }
    .number {
        color: #79c0ff;
    }
    .operator {
        color: #ff7b72;
    }
    .field {
        color: #ffa657;
    }
    .change-marker {
        display: inline-block;
        width: 20px;
        text-align: center;
        font-weight: bold;
    }
    .change-marker.add {
        color: #3fb950;
    }
    .change-marker.remove {
        color: #f85149;
    }
    .change-marker.modify {
        color: #d29922;
    }
    .stats {
        padding: 15px 20px;
        background: #21262d;
        border-top: 1px solid #30363d;
        display: flex;
        gap: 20px;
    }
    .stat {
        display: flex;
        align-items: center;
        gap: 5px;
    }
    .stat-label {
        color: #8b949e;
    }
    .stat-value {
        font-weight: bold;
    }
    .stat-value.add {
        color: #3fb950;
    }
    .stat-value.remove {
        color: #f85149;
    }
    .stat-value.modify {
        color: #d29922;
    }
</style>
"#,
    );

    html.push_str("</head>\n<body>\n");
    html.push_str("<div class=\"container\">\n");

    // Header
    html.push_str("<div class=\"header\">\n");
    html.push_str(&format!(
        "<h1>Diff for {} (Severity: {:?})</h1>\n",
        diff.statute_id, diff.impact.severity
    ));
    html.push_str("</div>\n");

    // Stats
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

    html.push_str("<div class=\"stats\">\n");
    html.push_str(&format!(
        "<div class=\"stat\"><span class=\"stat-label\">Added:</span> <span class=\"stat-value add\">+{}</span></div>\n",
        added_count
    ));
    html.push_str(&format!(
        "<div class=\"stat\"><span class=\"stat-label\">Removed:</span> <span class=\"stat-value remove\">-{}</span></div>\n",
        removed_count
    ));
    html.push_str(&format!(
        "<div class=\"stat\"><span class=\"stat-label\">Modified:</span> <span class=\"stat-value modify\">~{}</span></div>\n",
        modified_count
    ));
    html.push_str("</div>\n");

    // Diff view
    html.push_str("<div class=\"diff-view\">\n");

    // Old side
    html.push_str("<div class=\"diff-side old\">\n");
    html.push_str("<div class=\"side-header\">BEFORE</div>\n");

    for (i, change) in diff.changes.iter().enumerate() {
        if let Some(old_val) = &change.old_value {
            let class = match change.change_type {
                ChangeType::Removed => "removed",
                ChangeType::Modified => "modified",
                _ => "",
            };
            let marker = match change.change_type {
                ChangeType::Removed => "−",
                ChangeType::Modified => "~",
                _ => " ",
            };
            let marker_class = match change.change_type {
                ChangeType::Removed => "remove",
                ChangeType::Modified => "modify",
                _ => "",
            };

            html.push_str(&format!("<div class=\"line {}\">\n", class));
            html.push_str(&format!("<span class=\"line-number\">{}</span>\n", i + 1));
            html.push_str(&format!(
                "<span class=\"change-marker {}\">{}</span>\n",
                marker_class, marker
            ));
            html.push_str(&format!(
                "<span class=\"line-content\">{}</span>\n",
                syntax_highlight(old_val)
            ));
            html.push_str("</div>\n");
        }
    }

    html.push_str("</div>\n");

    // New side
    html.push_str("<div class=\"diff-side new\">\n");
    html.push_str("<div class=\"side-header\">AFTER</div>\n");

    for (i, change) in diff.changes.iter().enumerate() {
        if let Some(new_val) = &change.new_value {
            let class = match change.change_type {
                ChangeType::Added => "added",
                ChangeType::Modified => "modified",
                _ => "",
            };
            let marker = match change.change_type {
                ChangeType::Added => "+",
                ChangeType::Modified => "~",
                _ => " ",
            };
            let marker_class = match change.change_type {
                ChangeType::Added => "add",
                ChangeType::Modified => "modify",
                _ => "",
            };

            html.push_str(&format!("<div class=\"line {}\">\n", class));
            html.push_str(&format!("<span class=\"line-number\">{}</span>\n", i + 1));
            html.push_str(&format!(
                "<span class=\"change-marker {}\">{}</span>\n",
                marker_class, marker
            ));
            html.push_str(&format!(
                "<span class=\"line-content\">{}</span>\n",
                syntax_highlight(new_val)
            ));
            html.push_str("</div>\n");
        }
    }

    html.push_str("</div>\n");
    html.push_str("</div>\n");

    html.push_str("</div>\n");
    html.push_str("</body>\n</html>");
    html
}

/// Apply basic syntax highlighting to text.
fn syntax_highlight(text: &str) -> String {
    let mut result = text.to_string();

    // Highlight keywords
    for keyword in &[
        "Grant",
        "Revoke",
        "Obligation",
        "Prohibit",
        "Age",
        "Income",
        "Residence",
    ] {
        result = result.replace(
            keyword,
            &format!("<span class=\"keyword\">{}</span>", keyword),
        );
    }

    // Highlight operators (longest first to avoid partial matches)
    for op in &[">=", "<=", "==", "!=", ">", "<"] {
        result = result.replace(op, &format!("<span class=\"operator\">{}</span>", op));
    }

    // Simple number highlighting without regex
    // Collect words that are numbers first to avoid borrow issues
    let number_words: Vec<String> = result
        .split_whitespace()
        .filter(|word| word.chars().all(|c| c.is_ascii_digit()))
        .map(|s| s.to_string())
        .collect();

    for word in number_words {
        result = result.replace(&word, &format!("<span class=\"number\">{}</span>", word));
    }

    result
}

/// Generates an animated diff presentation for showcasing changes.
///
/// This creates a presentation-style view with smooth animations
/// that reveal changes one by one.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, visual::generate_animated_diff_presentation};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let html = generate_animated_diff_presentation(&diff_result);
///
/// assert!(html.contains("Diff Presentation"));
/// ```
pub fn generate_animated_diff_presentation(diff: &StatuteDiff) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!(
        "<title>Animated Diff: {}</title>\n",
        diff.statute_id
    ));

    html.push_str(
        r#"
<style>
    * {
        box-sizing: border-box;
    }
    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        margin: 0;
        padding: 0;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .presentation {
        width: 90%;
        max-width: 1000px;
        background: white;
        border-radius: 12px;
        box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        overflow: hidden;
    }
    .slide {
        padding: 60px;
        min-height: 600px;
        display: none;
        opacity: 0;
        animation: fadeIn 0.8s ease forwards;
    }
    .slide.active {
        display: block;
    }
    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
    .slide-header {
        font-size: 48px;
        font-weight: bold;
        margin-bottom: 30px;
        color: #24292e;
    }
    .slide-content {
        font-size: 24px;
        line-height: 1.6;
        color: #586069;
    }
    .change-animation {
        display: inline-block;
        padding: 10px 20px;
        border-radius: 6px;
        margin: 10px 0;
        animation: slideIn 0.5s ease forwards;
    }
    @keyframes slideIn {
        from {
            opacity: 0;
            transform: translateX(-30px);
        }
        to {
            opacity: 1;
            transform: translateX(0);
        }
    }
    .change-animation.added {
        background: #d4edda;
        color: #155724;
    }
    .change-animation.removed {
        background: #f8d7da;
        color: #721c24;
    }
    .change-animation.modified {
        background: #fff3cd;
        color: #856404;
    }
    .controls {
        display: flex;
        justify-content: space-between;
        padding: 20px 60px;
        background: #f6f8fa;
        border-top: 1px solid #e1e4e8;
    }
    .btn {
        padding: 12px 30px;
        background: #0366d6;
        color: white;
        border: none;
        border-radius: 6px;
        font-size: 16px;
        cursor: pointer;
        transition: all 0.2s;
    }
    .btn:hover {
        background: #0256c7;
        transform: translateY(-2px);
    }
    .btn:disabled {
        background: #e1e4e8;
        color: #959da5;
        cursor: not-allowed;
        transform: none;
    }
    .slide-indicator {
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .dot {
        width: 10px;
        height: 10px;
        border-radius: 50%;
        background: #e1e4e8;
        transition: all 0.3s;
    }
    .dot.active {
        background: #0366d6;
        transform: scale(1.3);
    }
    .severity-badge {
        display: inline-block;
        padding: 8px 16px;
        border-radius: 6px;
        font-size: 18px;
        font-weight: bold;
        margin: 20px 0;
    }
    .severity-badge.major {
        background: #f8d7da;
        color: #721c24;
    }
    .severity-badge.moderate {
        background: #fff3cd;
        color: #856404;
    }
    .severity-badge.minor {
        background: #d1ecf1;
        color: #0c5460;
    }
    .severity-badge.breaking {
        background: #d6336c;
        color: white;
    }
    .value-box {
        padding: 20px;
        margin: 15px 0;
        border-radius: 6px;
        font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
        font-size: 18px;
    }
    .value-box.old {
        background: #ffebe9;
        border-left: 4px solid #dc3545;
    }
    .value-box.new {
        background: #e6ffed;
        border-left: 4px solid #28a745;
    }
</style>
"#,
    );

    html.push_str(
        r#"
<script>
let currentSlide = 0;
const totalSlides = document.querySelectorAll('.slide').length;

function showSlide(n) {
    const slides = document.querySelectorAll('.slide');
    const dots = document.querySelectorAll('.dot');

    if (n >= slides.length) currentSlide = slides.length - 1;
    if (n < 0) currentSlide = 0;
    else currentSlide = n;

    slides.forEach(slide => slide.classList.remove('active'));
    dots.forEach(dot => dot.classList.remove('active'));

    slides[currentSlide].classList.add('active');
    dots[currentSlide].classList.add('active');

    document.getElementById('prevBtn').disabled = currentSlide === 0;
    document.getElementById('nextBtn').disabled = currentSlide === slides.length - 1;
}

function nextSlide() {
    showSlide(currentSlide + 1);
}

function prevSlide() {
    showSlide(currentSlide - 1);
}

document.addEventListener('DOMContentLoaded', function() {
    showSlide(0);

    document.addEventListener('keydown', function(e) {
        if (e.key === 'ArrowRight') nextSlide();
        if (e.key === 'ArrowLeft') prevSlide();
    });
});
</script>
"#,
    );

    html.push_str("</head>\n<body>\n");
    html.push_str("<div class=\"presentation\">\n");

    // Title slide
    html.push_str("<div class=\"slide active\">\n");
    html.push_str("<div class=\"slide-header\">Diff Presentation</div>\n");
    html.push_str(&format!(
        "<div class=\"slide-content\">Statute: <strong>{}</strong></div>\n",
        diff.statute_id
    ));
    html.push_str(&format!(
        "<div class=\"slide-content\">Total Changes: <strong>{}</strong></div>\n",
        diff.changes.len()
    ));
    html.push_str(&format!(
        "<div class=\"severity-badge {}\">{:?} Severity</div>\n",
        format!("{:?}", diff.impact.severity).to_lowercase(),
        diff.impact.severity
    ));
    html.push_str("</div>\n");

    // Change slides
    for (i, change) in diff.changes.iter().enumerate() {
        html.push_str("<div class=\"slide\">\n");
        html.push_str(&format!(
            "<div class=\"slide-header\">Change #{}</div>\n",
            i + 1
        ));

        html.push_str(&format!(
            "<div class=\"change-animation {}\">Type: {:?}</div><br>\n",
            format!("{:?}", change.change_type).to_lowercase(),
            change.change_type
        ));

        html.push_str(&format!(
            "<div class=\"slide-content\"><strong>Target:</strong> {}</div>\n",
            change.target
        ));

        html.push_str(&format!(
            "<div class=\"slide-content\"><strong>Description:</strong> {}</div>\n",
            change.description
        ));

        if let Some(old) = &change.old_value {
            html.push_str(&format!(
                "<div class=\"value-box old\">Old: {}</div>\n",
                old
            ));
        }

        if let Some(new) = &change.new_value {
            html.push_str(&format!(
                "<div class=\"value-box new\">New: {}</div>\n",
                new
            ));
        }

        html.push_str("</div>\n");
    }

    // Summary slide
    html.push_str("<div class=\"slide\">\n");
    html.push_str("<div class=\"slide-header\">Summary</div>\n");
    if diff.impact.affects_eligibility {
        html.push_str("<div class=\"slide-content\">✓ Affects Eligibility</div>\n");
    }
    if diff.impact.affects_outcome {
        html.push_str("<div class=\"slide-content\">✓ Affects Outcome</div>\n");
    }
    if diff.impact.discretion_changed {
        html.push_str("<div class=\"slide-content\">✓ Discretion Changed</div>\n");
    }
    for note in &diff.impact.notes {
        html.push_str(&format!("<div class=\"slide-content\">• {}</div>\n", note));
    }
    html.push_str("</div>\n");

    // Controls
    html.push_str("<div class=\"controls\">\n");
    html.push_str(
        "<button id=\"prevBtn\" class=\"btn\" onclick=\"prevSlide()\">Previous</button>\n",
    );
    html.push_str("<div class=\"slide-indicator\">\n");

    for i in 0..=diff.changes.len() + 1 {
        let active = if i == 0 { " active" } else { "" };
        html.push_str(&format!("<div class=\"dot{}\"></div>\n", active));
    }

    html.push_str("</div>\n");
    html.push_str("<button id=\"nextBtn\" class=\"btn\" onclick=\"nextSlide()\">Next</button>\n");
    html.push_str("</div>\n");

    html.push_str("</div>\n");
    html.push_str("</body>\n</html>");
    html
}

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

    // Header
    html.push_str("<div class=\"header\">\n");
    html.push_str("<h1>Three-Way Diff Viewer</h1>\n");
    html.push_str(&format!(
        "<div class=\"subtitle\">Statute: {} | Comparing BASE with YOUR changes and THEIR changes</div>\n",
        diff_ours.statute_id
    ));
    html.push_str("</div>\n");

    // Legend
    html.push_str("<div class=\"legend\">\n");
    html.push_str("<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #28a745;\"></div><span>Added</span></div>\n");
    html.push_str("<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #dc3545;\"></div><span>Removed</span></div>\n");
    html.push_str("<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #ffc107;\"></div><span>Modified</span></div>\n");
    html.push_str("<div class=\"legend-item\"><div class=\"legend-color\" style=\"background: #d6336c;\"></div><span>Conflict</span></div>\n");
    html.push_str("</div>\n");

    // Stats
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

    // Detect conflicts
    let conflicts = detect_conflicts(diff_ours, diff_theirs);

    if !conflicts.is_empty() {
        html.push_str("<div class=\"conflict-indicator\">\n");
        html.push_str(&format!(
            "⚠️ {} conflict(s) detected! Review carefully before merging.\n",
            conflicts.len()
        ));
        html.push_str("</div>\n");
    }

    // Three-way view
    html.push_str("<div class=\"three-way-view\">\n");

    // Base column (showing what's being changed from)
    html.push_str("<div class=\"column\">\n");
    html.push_str("<div class=\"column-header base\">BASE</div>\n");

    // Collect all unique targets from both diffs
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

    // Ours column
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

    // Theirs column
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

    #[test]
    fn test_generate_syntax_highlighted_diff() {
        let diff = test_diff();
        let html = generate_syntax_highlighted_diff(&diff);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Syntax Highlighted Diff"));
        assert!(html.contains("test-statute"));
        assert!(html.contains("BEFORE"));
        assert!(html.contains("AFTER"));
    }

    #[test]
    fn test_syntax_highlight() {
        let text = "Grant benefit Age 18";
        let highlighted = syntax_highlight(text);

        assert!(highlighted.contains("keyword"));
        assert!(highlighted.contains("number"));
    }

    #[test]
    fn test_generate_animated_diff_presentation() {
        let diff = test_diff();
        let html = generate_animated_diff_presentation(&diff);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Diff Presentation"));
        assert!(html.contains("test-statute"));
        assert!(html.contains("slide"));
        assert!(html.contains("prevBtn"));
        assert!(html.contains("nextBtn"));
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

        // Modify one change in diff_theirs to create a conflict
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
