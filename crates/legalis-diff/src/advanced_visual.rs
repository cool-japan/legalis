//! Advanced visualization features for statute diffs.
//!
//! This module provides advanced interactive visualizations including:
//! - Web-based diff explorers with search and filtering
//! - Dependency graphs showing change relationships
//! - Heatmaps for change frequency analysis
//! - Temporal visualizations of amendments over time
//! - Customizable dashboards
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, advanced_visual::DashboardConfig};
//!
//! let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
//! let new = old.clone();
//! let diff_result = diff(&old, &new).unwrap();
//!
//! let config = DashboardConfig::default();
//! let dashboard = legalis_diff::advanced_visual::generate_dashboard(&[diff_result], &config);
//! ```

use crate::{ChangeTarget, ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for dashboard generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Title for the dashboard
    pub title: String,
    /// Whether to include change frequency heatmap
    pub include_heatmap: bool,
    /// Whether to include dependency graph
    pub include_dependency_graph: bool,
    /// Whether to include temporal timeline
    pub include_timeline: bool,
    /// Color scheme
    pub color_scheme: ColorScheme,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            title: "Statute Diff Dashboard".to_string(),
            include_heatmap: true,
            include_dependency_graph: true,
            include_timeline: true,
            color_scheme: ColorScheme::Default,
        }
    }
}

/// Color schemes for visualizations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorScheme {
    Default,
    HighContrast,
    Monochrome,
    Colorblind,
}

impl ColorScheme {
    fn get_colors(&self) -> HashMap<&'static str, &'static str> {
        match self {
            ColorScheme::Default => {
                let mut colors = HashMap::new();
                colors.insert("added", "#28a745");
                colors.insert("removed", "#dc3545");
                colors.insert("modified", "#ffc107");
                colors.insert("reordered", "#17a2b8");
                colors.insert("background", "#ffffff");
                colors.insert("text", "#212529");
                colors
            }
            ColorScheme::HighContrast => {
                let mut colors = HashMap::new();
                colors.insert("added", "#00ff00");
                colors.insert("removed", "#ff0000");
                colors.insert("modified", "#ffff00");
                colors.insert("reordered", "#00ffff");
                colors.insert("background", "#000000");
                colors.insert("text", "#ffffff");
                colors
            }
            ColorScheme::Monochrome => {
                let mut colors = HashMap::new();
                colors.insert("added", "#404040");
                colors.insert("removed", "#808080");
                colors.insert("modified", "#606060");
                colors.insert("reordered", "#505050");
                colors.insert("background", "#ffffff");
                colors.insert("text", "#000000");
                colors
            }
            ColorScheme::Colorblind => {
                let mut colors = HashMap::new();
                colors.insert("added", "#0173b2");
                colors.insert("removed", "#de8f05");
                colors.insert("modified", "#029e73");
                colors.insert("reordered", "#cc78bc");
                colors.insert("background", "#ffffff");
                colors.insert("text", "#000000");
                colors
            }
        }
    }
}

/// Generates an interactive web-based diff explorer.
///
/// Creates a full HTML page with JavaScript for interactive exploration,
/// including search, filtering, and navigation features.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, advanced_visual::generate_web_explorer};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diffs = vec![diff(&old, &new).unwrap()];
///
/// let html = generate_web_explorer(&diffs);
/// assert!(html.contains("<html"));
/// ```
pub fn generate_web_explorer(diffs: &[StatuteDiff]) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Interactive Diff Explorer</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif; background: #f5f5f5; }
        .container { max-width: 1400px; margin: 0 auto; padding: 20px; }
        .header { background: #2c3e50; color: white; padding: 20px; margin-bottom: 20px; border-radius: 8px; }
        .header h1 { margin-bottom: 10px; }
        .search-bar { margin: 20px 0; display: flex; gap: 10px; }
        .search-bar input { flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px; }
        .search-bar select { padding: 10px; border: 1px solid #ddd; border-radius: 4px; }
        .filters { display: flex; gap: 10px; margin-bottom: 20px; flex-wrap: wrap; }
        .filter-btn { padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; background: #e0e0e0; }
        .filter-btn.active { background: #3498db; color: white; }
        .diff-list { background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .diff-item { border-bottom: 1px solid #eee; padding: 20px; cursor: pointer; transition: background 0.2s; }
        .diff-item:hover { background: #f9f9f9; }
        .diff-item:last-child { border-bottom: none; }
        .diff-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }
        .statute-id { font-weight: bold; font-size: 16px; color: #2c3e50; }
        .severity-badge { padding: 4px 12px; border-radius: 12px; font-size: 12px; font-weight: bold; }
        .severity-none { background: #95a5a6; color: white; }
        .severity-minor { background: #3498db; color: white; }
        .severity-moderate { background: #f39c12; color: white; }
        .severity-major { background: #e67e22; color: white; }
        .severity-breaking { background: #e74c3c; color: white; }
        .changes-summary { font-size: 14px; color: #7f8c8d; margin-bottom: 8px; }
        .change-details { display: none; margin-top: 15px; padding-top: 15px; border-top: 1px solid #eee; }
        .change-details.expanded { display: block; }
        .change-item { padding: 8px; margin: 4px 0; border-left: 3px solid #3498db; background: #ecf0f1; }
        .change-type { display: inline-block; padding: 2px 8px; border-radius: 3px; font-size: 11px; font-weight: bold; margin-right: 8px; }
        .type-added { background: #28a745; color: white; }
        .type-removed { background: #dc3545; color: white; }
        .type-modified { background: #ffc107; color: #000; }
        .type-reordered { background: #17a2b8; color: white; }
        .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-bottom: 20px; }
        .stat-card { background: white; padding: 15px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .stat-value { font-size: 32px; font-weight: bold; color: #2c3e50; }
        .stat-label { font-size: 14px; color: #7f8c8d; margin-top: 5px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Interactive Diff Explorer</h1>
            <p>Explore and analyze statute changes</p>
        </div>

        <div class="stats" id="stats"></div>

        <div class="search-bar">
            <input type="text" id="searchInput" placeholder="Search statute ID, changes, or descriptions...">
            <select id="sortSelect">
                <option value="id">Sort by ID</option>
                <option value="severity">Sort by Severity</option>
                <option value="changes">Sort by Change Count</option>
            </select>
        </div>

        <div class="filters" id="filters"></div>

        <div class="diff-list" id="diffList"></div>
    </div>

    <script>
        const diffs = "#,
    );

    // Serialize diffs to JSON
    html.push_str(&serde_json::to_string(diffs).unwrap_or_else(|_| "[]".to_string()));
    html.push_str(
        r#";

        let currentFilter = 'all';
        let searchTerm = '';
        let sortBy = 'id';

        function renderStats() {
            const stats = {
                totalDiffs: diffs.length,
                totalChanges: diffs.reduce((sum, d) => sum + d.changes.length, 0),
                highSeverity: diffs.filter(d => d.impact.severity === 'Major' || d.impact.severity === 'Breaking').length,
                avgChanges: Math.round(diffs.reduce((sum, d) => sum + d.changes.length, 0) / diffs.length)
            };

            document.getElementById('stats').innerHTML = `
                <div class="stat-card">
                    <div class="stat-value">${stats.totalDiffs}</div>
                    <div class="stat-label">Total Diffs</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${stats.totalChanges}</div>
                    <div class="stat-label">Total Changes</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${stats.highSeverity}</div>
                    <div class="stat-label">High Severity</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${stats.avgChanges}</div>
                    <div class="stat-label">Avg Changes/Diff</div>
                </div>
            `;
        }

        function renderFilters() {
            const severities = ['All', 'None', 'Minor', 'Moderate', 'Major', 'Breaking'];
            document.getElementById('filters').innerHTML = severities.map(s =>
                `<button class="filter-btn ${s.toLowerCase() === currentFilter ? 'active' : ''}"
                         onclick="setFilter('${s.toLowerCase()}')">${s}</button>`
            ).join('');
        }

        function setFilter(filter) {
            currentFilter = filter;
            renderFilters();
            renderDiffs();
        }

        function renderDiffs() {
            let filtered = diffs.filter(d => {
                if (currentFilter !== 'all' && d.impact.severity.toLowerCase() !== currentFilter) return false;
                if (searchTerm && !JSON.stringify(d).toLowerCase().includes(searchTerm.toLowerCase())) return false;
                return true;
            });

            if (sortBy === 'severity') {
                const severityOrder = { 'Breaking': 4, 'Major': 3, 'Moderate': 2, 'Minor': 1, 'None': 0 };
                filtered.sort((a, b) => severityOrder[b.impact.severity] - severityOrder[a.impact.severity]);
            } else if (sortBy === 'changes') {
                filtered.sort((a, b) => b.changes.length - a.changes.length);
            } else {
                filtered.sort((a, b) => a.statute_id.localeCompare(b.statute_id));
            }

            document.getElementById('diffList').innerHTML = filtered.map((d, i) => `
                <div class="diff-item" onclick="toggleDetails(${i})">
                    <div class="diff-header">
                        <span class="statute-id">${d.statute_id}</span>
                        <span class="severity-badge severity-${d.impact.severity.toLowerCase()}">${d.impact.severity}</span>
                    </div>
                    <div class="changes-summary">
                        ${d.changes.length} change${d.changes.length !== 1 ? 's' : ''}
                        ${d.impact.affects_eligibility ? ' • Affects Eligibility' : ''}
                        ${d.impact.affects_outcome ? ' • Affects Outcome' : ''}
                    </div>
                    <div class="change-details" id="details-${i}">
                        ${d.changes.map(c => `
                            <div class="change-item">
                                <span class="change-type type-${c.change_type.toLowerCase()}">${c.change_type}</span>
                                ${c.description}
                            </div>
                        `).join('')}
                    </div>
                </div>
            `).join('');
        }

        function toggleDetails(index) {
            const details = document.getElementById(`details-${index}`);
            details.classList.toggle('expanded');
        }

        document.getElementById('searchInput').addEventListener('input', (e) => {
            searchTerm = e.target.value;
            renderDiffs();
        });

        document.getElementById('sortSelect').addEventListener('change', (e) => {
            sortBy = e.target.value;
            renderDiffs();
        });

        renderStats();
        renderFilters();
        renderDiffs();
    </script>
</body>
</html>"#,
    );

    html
}

/// Generates a dependency graph showing relationships between changes.
///
/// Creates a DOT format graph that can be rendered with Graphviz.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, advanced_visual::generate_dependency_graph};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diff_result = diff(&old, &new).unwrap();
///
/// let dot = generate_dependency_graph(&diff_result);
/// assert!(dot.contains("digraph"));
/// ```
pub fn generate_dependency_graph(diff: &StatuteDiff) -> String {
    let mut dot = String::from("digraph DependencyGraph {\n");
    dot.push_str("    rankdir=LR;\n");
    dot.push_str("    node [shape=box, style=rounded];\n\n");

    // Add statute node
    dot.push_str(&format!(
        "    statute [label=\"{}\", shape=ellipse, fillcolor=lightblue, style=filled];\n",
        diff.statute_id
    ));

    // Add change nodes
    for (i, change) in diff.changes.iter().enumerate() {
        let color = match change.change_type {
            ChangeType::Added => "lightgreen",
            ChangeType::Removed => "lightcoral",
            ChangeType::Modified => "lightyellow",
            ChangeType::Reordered => "lightcyan",
        };

        let label = format!("{:?}\\n{}", change.change_type, change.target);
        dot.push_str(&format!(
            "    change{} [label=\"{}\", fillcolor={}, style=filled];\n",
            i, label, color
        ));
        dot.push_str(&format!("    statute -> change{};\n", i));
    }

    // Add impact node if significant
    if diff.impact.severity >= Severity::Moderate {
        dot.push_str(&format!(
            "    impact [label=\"Impact: {:?}\", shape=diamond, fillcolor=orange, style=filled];\n",
            diff.impact.severity
        ));
        for i in 0..diff.changes.len() {
            dot.push_str(&format!("    change{} -> impact;\n", i));
        }
    }

    dot.push_str("}\n");
    dot
}

/// Generates a heatmap showing change frequency across different targets.
///
/// Returns an SVG heatmap visualization.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, advanced_visual::generate_change_heatmap};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
/// let diffs = vec![diff(&old, &new).unwrap()];
///
/// let heatmap = generate_change_heatmap(&diffs);
/// assert!(heatmap.contains("<svg"));
/// ```
pub fn generate_change_heatmap(diffs: &[StatuteDiff]) -> String {
    // Count changes by target type and change type
    let mut heat_data: HashMap<(String, ChangeType), usize> = HashMap::new();

    for diff in diffs {
        for change in &diff.changes {
            let target = match &change.target {
                ChangeTarget::Title => "Title".to_string(),
                ChangeTarget::Precondition { .. } => "Precondition".to_string(),
                ChangeTarget::Effect => "Effect".to_string(),
                ChangeTarget::DiscretionLogic => "Discretion".to_string(),
                ChangeTarget::Metadata { .. } => "Metadata".to_string(),
            };
            *heat_data.entry((target, change.change_type)).or_insert(0) += 1;
        }
    }

    let width = 600;
    let height = 400;
    let margin = 80;

    let targets = ["Title", "Precondition", "Effect", "Discretion", "Metadata"];
    let change_types = [
        ChangeType::Added,
        ChangeType::Removed,
        ChangeType::Modified,
        ChangeType::Reordered,
    ];

    let cell_width = (width - 2 * margin) / change_types.len();
    let cell_height = (height - 2 * margin) / targets.len();

    let max_count = heat_data.values().max().copied().unwrap_or(1);

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );

    // Title
    svg.push_str(&format!(
        r#"<text x="{}" y="30" font-size="18" font-weight="bold" text-anchor="middle">Change Frequency Heatmap</text>"#,
        width / 2
    ));

    // Column headers (change types)
    for (i, ct) in change_types.iter().enumerate() {
        let x = margin + i * cell_width + cell_width / 2;
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="12" text-anchor="middle" transform="rotate(-45 {} {})">{:?}</text>"#,
            x, margin - 10, x, margin - 10, ct
        ));
    }

    // Row headers (targets) and cells
    for (row, target) in targets.iter().enumerate() {
        let y = margin + row * cell_height;

        // Row header
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="12" text-anchor="end" dominant-baseline="middle">{}</text>"#,
            margin - 10,
            y + cell_height / 2,
            target
        ));

        // Cells
        for (col, ct) in change_types.iter().enumerate() {
            let x = margin + col * cell_width;
            let count = heat_data
                .get(&(target.to_string(), *ct))
                .copied()
                .unwrap_or(0);

            let intensity = if max_count > 0 {
                (count as f64 / max_count as f64 * 255.0) as u8
            } else {
                0
            };

            let color = format!("rgb({}, {}, 255)", 255 - intensity, 255 - intensity);

            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="white" stroke-width="1" />"#,
                x, y, cell_width, cell_height, color
            ));

            if count > 0 {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-size="14" font-weight="bold" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
                    x + cell_width / 2,
                    y + cell_height / 2,
                    count
                ));
            }
        }
    }

    svg.push_str("</svg>");
    svg
}

/// Generates a temporal visualization showing amendments over time.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, advanced_visual::generate_temporal_visualization};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diffs = vec![("2024-01-01", diff(&old, &new).unwrap())];
///
/// let svg = generate_temporal_visualization(&diffs);
/// assert!(svg.contains("<svg"));
/// ```
pub fn generate_temporal_visualization(diffs: &[(&str, StatuteDiff)]) -> String {
    let width = 1000;
    let height = 400;
    let margin = 60;

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );

    svg.push_str(&format!(
        r#"<text x="{}" y="30" font-size="18" font-weight="bold" text-anchor="middle">Temporal Amendment Visualization</text>"#,
        width / 2
    ));

    // Timeline
    svg.push_str(&format!(
        r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#333" stroke-width="2" />"##,
        margin,
        height / 2,
        width - margin,
        height / 2
    ));

    let timeline_width = width - 2 * margin;
    let step = if diffs.len() > 1 {
        timeline_width / (diffs.len() - 1)
    } else {
        timeline_width
    };

    for (i, (date, diff)) in diffs.iter().enumerate() {
        let x = margin + i * step;
        let y = height / 2;

        // Marker
        let severity_color = match diff.impact.severity {
            Severity::None => "#95a5a6",
            Severity::Minor => "#3498db",
            Severity::Moderate => "#f39c12",
            Severity::Major => "#e67e22",
            Severity::Breaking => "#e74c3c",
        };

        let radius = 5 + diff.changes.len().min(10) * 2;

        svg.push_str(&format!(
            r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="white" stroke-width="2" />"#,
            x, y, radius, severity_color
        ));

        // Date label
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="10" text-anchor="middle">{}</text>"#,
            x,
            y + radius + 15,
            date
        ));

        // Change count
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-size="12" font-weight="bold" text-anchor="middle" fill="white">{}</text>"#,
            x,
            y,
            diff.changes.len()
        ));
    }

    // Legend
    let legend_y = height - 40;
    svg.push_str(&format!(
        r#"<text x="{}" y="{}" font-size="12">Circle size: Number of changes</text>"#,
        margin, legend_y
    ));

    svg.push_str("</svg>");
    svg
}

/// Generates a customizable dashboard with multiple visualizations.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, advanced_visual::{generate_dashboard, DashboardConfig}};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diffs = vec![diff(&old, &new).unwrap()];
///
/// let config = DashboardConfig::default();
/// let dashboard = generate_dashboard(&diffs, &config);
/// assert!(dashboard.contains("<html"));
/// ```
pub fn generate_dashboard(diffs: &[StatuteDiff], config: &DashboardConfig) -> String {
    let colors = config.color_scheme.get_colors();

    let mut html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                background: {}; color: {}; padding: 20px; }}
        .dashboard {{ max-width: 1400px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 30px; }}
        .header h1 {{ font-size: 36px; margin-bottom: 10px; }}
        .grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(500px, 1fr)); gap: 20px; }}
        .panel {{ background: white; border-radius: 8px; padding: 20px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
        .panel h2 {{ margin-bottom: 15px; color: #2c3e50; }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(4, 1fr); gap: 15px; margin-bottom: 20px; }}
        .stat-box {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white;
                     padding: 20px; border-radius: 8px; text-align: center; }}
        .stat-number {{ font-size: 32px; font-weight: bold; }}
        .stat-label {{ font-size: 14px; opacity: 0.9; margin-top: 5px; }}
    </style>
</head>
<body>
    <div class="dashboard">
        <div class="header">
            <h1>{}</h1>
            <p>Comprehensive statute diff analysis</p>
        </div>

        <div class="stats-grid">
            <div class="stat-box">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total Diffs</div>
            </div>
            <div class="stat-box">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total Changes</div>
            </div>
            <div class="stat-box">
                <div class="stat-number">{}</div>
                <div class="stat-label">High Severity</div>
            </div>
            <div class="stat-box">
                <div class="stat-number">{:.1}</div>
                <div class="stat-label">Avg Changes</div>
            </div>
        </div>

        <div class="grid">"#,
        config.title,
        colors["background"],
        colors["text"],
        config.title,
        diffs.len(),
        diffs.iter().map(|d| d.changes.len()).sum::<usize>(),
        diffs
            .iter()
            .filter(|d| d.impact.severity >= Severity::Major)
            .count(),
        diffs.iter().map(|d| d.changes.len()).sum::<usize>() as f64 / diffs.len() as f64
    );

    if config.include_heatmap {
        html.push_str(
            r#"
            <div class="panel">
                <h2>Change Frequency Heatmap</h2>
                "#,
        );
        html.push_str(&generate_change_heatmap(diffs));
        html.push_str("</div>");
    }

    if config.include_dependency_graph && !diffs.is_empty() {
        html.push_str(
            r#"
            <div class="panel">
                <h2>Dependency Graph (DOT format)</h2>
                <pre style="background: #f5f5f5; padding: 15px; border-radius: 4px; overflow-x: auto;">"#,
        );
        html.push_str(&generate_dependency_graph(&diffs[0]));
        html.push_str("</pre></div>");
    }

    html.push_str(
        r#"
        </div>
    </div>
</body>
</html>"#,
    );

    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn test_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_dashboard_config_default() {
        let config = DashboardConfig::default();
        assert_eq!(config.title, "Statute Diff Dashboard");
        assert!(config.include_heatmap);
        assert!(config.include_dependency_graph);
        assert!(config.include_timeline);
    }

    #[test]
    fn test_color_scheme() {
        let default_colors = ColorScheme::Default.get_colors();
        assert_eq!(default_colors["added"], "#28a745");

        let hc_colors = ColorScheme::HighContrast.get_colors();
        assert_eq!(hc_colors["background"], "#000000");
    }

    #[test]
    fn test_web_explorer() {
        let old = test_statute();
        let new = test_statute();
        let diffs = vec![diff(&old, &new).unwrap()];

        let html = generate_web_explorer(&diffs);
        assert!(html.contains("<html"));
        assert!(html.contains("Interactive Diff Explorer"));
        assert!(html.contains("searchInput"));
    }

    #[test]
    fn test_dependency_graph() {
        let old = test_statute();
        let mut new = test_statute();
        new.title = "Modified Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let dot = generate_dependency_graph(&diff_result);

        assert!(dot.contains("digraph"));
        assert!(dot.contains("test-statute"));
    }

    #[test]
    fn test_change_heatmap() {
        let old = test_statute();
        let mut new = test_statute();
        new.title = "Modified".to_string();

        let diffs = vec![diff(&old, &new).unwrap()];
        let heatmap = generate_change_heatmap(&diffs);

        assert!(heatmap.contains("<svg"));
        assert!(heatmap.contains("Heatmap"));
    }

    #[test]
    fn test_temporal_visualization() {
        let statute = test_statute();
        let diffs = vec![("2024-01-01", diff(&statute, &statute).unwrap())];

        let svg = generate_temporal_visualization(&diffs);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("2024-01-01"));
    }

    #[test]
    fn test_dashboard() {
        let old = test_statute();
        let new = test_statute();
        let diffs = vec![diff(&old, &new).unwrap()];

        let config = DashboardConfig::default();
        let dashboard = generate_dashboard(&diffs, &config);

        assert!(dashboard.contains("<html"));
        assert!(dashboard.contains("Statute Diff Dashboard"));
        assert!(dashboard.contains("Total Diffs"));
    }

    #[test]
    fn test_dashboard_custom_config() {
        let old = test_statute();
        let new = test_statute();
        let diffs = vec![diff(&old, &new).unwrap()];

        let config = DashboardConfig {
            title: "Custom Dashboard".to_string(),
            include_heatmap: false,
            include_dependency_graph: true,
            include_timeline: false,
            color_scheme: ColorScheme::Monochrome,
        };

        let dashboard = generate_dashboard(&diffs, &config);
        assert!(dashboard.contains("Custom Dashboard"));
    }
}
