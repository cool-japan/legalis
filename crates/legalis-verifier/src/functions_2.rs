//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::functions::html_escape;
use super::functions_3::{check_ctl_star_path, check_ctl_star_path_universal};
use super::types::{CrossReferenceErrorType, SequenceConstraint};
use super::types_3::{
    CrossReferenceError, CtlFormula, DeadlineViolation, SequenceVerificationResult,
    SequenceViolation, SimilarityScore, TerminologyInconsistency,
};
use super::types_4::{
    AmbiguousTerm, CtlStarFormula, CtlStarPathFormula, Deadline, LtlFormula, Severity,
    TemporalState, TransitionSystem,
};
use super::types_5::{
    DeadlineVerificationResult, IdeDiagnostic, QuickFix, VerificationError, VerificationResult,
};

/// Generates an interactive HTML report with filtering, search, and sorting capabilities.
///
/// This creates a feature-rich HTML report with:
/// - Severity filtering
/// - Search functionality
/// - Expandable/collapsible sections
/// - Statistics dashboard
/// - Dark mode toggle
pub fn generate_interactive_html_report(result: &VerificationResult, title: &str) -> String {
    let severity_counts = result.severity_counts();
    let critical_count = severity_counts.get(&Severity::Critical).unwrap_or(&0);
    let error_count = severity_counts.get(&Severity::Error).unwrap_or(&0);
    let warning_count = severity_counts.get(&Severity::Warning).unwrap_or(&0);
    let info_count = severity_counts.get(&Severity::Info).unwrap_or(&0);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        :root {{
            --bg-primary: #ffffff;
            --bg-secondary: #f5f5f5;
            --text-primary: #333;
            --text-secondary: #666;
            --border-color: #ddd;
            --critical-bg: #fee;
            --critical-border: #dc3545;
            --error-bg: #f8d7da;
            --error-border: #dc3545;
            --warning-bg: #fff3cd;
            --warning-border: #ffc107;
            --info-bg: #d1ecf1;
            --info-border: #17a2b8;
            --success-bg: #d4edda;
            --success-border: #28a745;
        }}

        body.dark-mode {{
            --bg-primary: #1e1e1e;
            --bg-secondary: #2d2d2d;
            --text-primary: #e0e0e0;
            --text-secondary: #aaa;
            --border-color: #444;
            --critical-bg: #4a1f1f;
            --error-bg: #3a1f1f;
            --warning-bg: #3a3220;
            --info-bg: #1f2f3a;
            --success-bg: #1f3a1f;
        }}

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: var(--bg-secondary);
            color: var(--text-primary);
            line-height: 1.6;
            transition: background 0.3s, color 0.3s;
        }}

        .container {{
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }}

        header {{
            background: var(--bg-primary);
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-bottom: 20px;
        }}

        h1 {{
            color: var(--text-primary);
            margin-bottom: 10px;
        }}

        .controls {{
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 15px;
        }}

        .search-box {{
            flex: 1;
            min-width: 200px;
        }}

        .search-box input {{
            width: 100%;
            padding: 10px;
            border: 1px solid var(--border-color);
            border-radius: 4px;
            background: var(--bg-primary);
            color: var(--text-primary);
            font-size: 14px;
        }}

        .filter-buttons {{
            display: flex;
            gap: 5px;
            flex-wrap: wrap;
        }}

        .filter-btn, .theme-toggle {{
            padding: 10px 15px;
            border: 1px solid var(--border-color);
            border-radius: 4px;
            background: var(--bg-primary);
            color: var(--text-primary);
            cursor: pointer;
            font-size: 14px;
            transition: all 0.2s;
        }}

        .filter-btn:hover, .theme-toggle:hover {{
            opacity: 0.8;
        }}

        .filter-btn.active {{
            background: #4CAF50;
            color: white;
            border-color: #4CAF50;
        }}

        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 20px;
        }}

        .stat-card {{
            background: var(--bg-primary);
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            border-left: 4px solid;
        }}

        .stat-card.critical {{ border-color: var(--critical-border); }}
        .stat-card.error {{ border-color: var(--error-border); }}
        .stat-card.warning {{ border-color: var(--warning-border); }}
        .stat-card.info {{ border-color: var(--info-border); }}
        .stat-card.success {{ border-color: var(--success-border); }}

        .stat-value {{
            font-size: 2em;
            font-weight: bold;
            margin-bottom: 5px;
        }}

        .stat-label {{
            color: var(--text-secondary);
            font-size: 0.9em;
        }}

        .section {{
            background: var(--bg-primary);
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-bottom: 20px;
        }}

        .section-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            cursor: pointer;
            padding: 10px 0;
            border-bottom: 2px solid var(--border-color);
            margin-bottom: 15px;
        }}

        .section-header h2 {{
            color: var(--text-primary);
        }}

        .toggle-icon {{
            font-size: 1.2em;
            transition: transform 0.3s;
        }}

        .toggle-icon.collapsed {{
            transform: rotate(-90deg);
        }}

        .item {{
            padding: 15px;
            margin: 10px 0;
            border-radius: 4px;
            border-left: 4px solid;
            transition: all 0.2s;
        }}

        .item:hover {{
            transform: translateX(5px);
        }}

        .item.critical {{
            background: var(--critical-bg);
            border-color: var(--critical-border);
        }}

        .item.error {{
            background: var(--error-bg);
            border-color: var(--error-border);
        }}

        .item.warning {{
            background: var(--warning-bg);
            border-color: var(--warning-border);
        }}

        .item.info {{
            background: var(--info-bg);
            border-color: var(--info-border);
        }}

        .item.hidden {{
            display: none;
        }}

        .severity-badge {{
            display: inline-block;
            padding: 4px 8px;
            border-radius: 3px;
            font-size: 0.8em;
            font-weight: bold;
            margin-right: 10px;
        }}

        .severity-badge.critical {{
            background: var(--critical-border);
            color: white;
        }}

        .severity-badge.error {{
            background: var(--error-border);
            color: white;
        }}

        .severity-badge.warning {{
            background: var(--warning-border);
            color: #333;
        }}

        .severity-badge.info {{
            background: var(--info-border);
            color: white;
        }}

        .empty {{
            color: var(--text-secondary);
            font-style: italic;
            text-align: center;
            padding: 20px;
        }}

        .timestamp {{
            text-align: center;
            color: var(--text-secondary);
            font-size: 0.9em;
            margin-top: 20px;
            padding: 15px;
            background: var(--bg-primary);
            border-radius: 8px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>{title}</h1>
            <div class="controls">
                <div class="search-box">
                    <input type="text" id="searchInput" placeholder="Search errors, warnings, suggestions...">
                </div>
                <div class="filter-buttons">
                    <button class="filter-btn active" data-filter="all">All</button>
                    <button class="filter-btn" data-filter="critical">Critical</button>
                    <button class="filter-btn" data-filter="error">Errors</button>
                    <button class="filter-btn" data-filter="warning">Warnings</button>
                    <button class="filter-btn" data-filter="info">Info</button>
                    <button class="theme-toggle" id="themeToggle">🌙 Dark Mode</button>
                </div>
            </div>
        </header>

        <div class="stats">
            <div class="stat-card success">
                <div class="stat-value">{status}</div>
                <div class="stat-label">Status</div>
            </div>
            <div class="stat-card critical">
                <div class="stat-value">{critical_count}</div>
                <div class="stat-label">Critical</div>
            </div>
            <div class="stat-card error">
                <div class="stat-value">{error_count}</div>
                <div class="stat-label">Errors</div>
            </div>
            <div class="stat-card warning">
                <div class="stat-value">{warning_count}</div>
                <div class="stat-label">Warnings</div>
            </div>
            <div class="stat-card info">
                <div class="stat-value">{info_count}</div>
                <div class="stat-label">Info</div>
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('errors')">
                <h2>Errors ({error_total})</h2>
                <span class="toggle-icon" id="errors-toggle">▼</span>
            </div>
            <div id="errors-content">
                {errors_html}
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('warnings')">
                <h2>Warnings ({warnings_total})</h2>
                <span class="toggle-icon" id="warnings-toggle">▼</span>
            </div>
            <div id="warnings-content">
                {warnings_html}
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('suggestions')">
                <h2>Suggestions ({suggestions_total})</h2>
                <span class="toggle-icon" id="suggestions-toggle">▼</span>
            </div>
            <div id="suggestions-content">
                {suggestions_html}
            </div>
        </div>

        <div class="timestamp">
            Generated: {timestamp}
        </div>
    </div>

    <script>
        // Dark mode toggle
        const themeToggle = document.getElementById('themeToggle');
        const body = document.body;

        themeToggle.addEventListener('click', () => {{
            body.classList.toggle('dark-mode');
            themeToggle.textContent = body.classList.contains('dark-mode') ? '☀️ Light Mode' : '🌙 Dark Mode';
        }});

        // Search functionality
        const searchInput = document.getElementById('searchInput');
        searchInput.addEventListener('input', (e) => {{
            const searchTerm = e.target.value.toLowerCase();
            const items = document.querySelectorAll('.item');

            items.forEach(item => {{
                const text = item.textContent.toLowerCase();
                if (text.includes(searchTerm)) {{
                    item.style.display = 'block';
                }} else {{
                    item.style.display = 'none';
                }}
            }});
        }});

        // Filter functionality
        const filterButtons = document.querySelectorAll('.filter-btn');
        filterButtons.forEach(button => {{
            button.addEventListener('click', () => {{
                // Update active state
                filterButtons.forEach(btn => btn.classList.remove('active'));
                button.classList.add('active');

                const filter = button.dataset.filter;
                const items = document.querySelectorAll('.item');

                items.forEach(item => {{
                    if (filter === 'all' || item.classList.contains(filter)) {{
                        item.style.display = 'block';
                    }} else {{
                        item.style.display = 'none';
                    }}
                }});
            }});
        }});

        // Section toggle
        function toggleSection(sectionId) {{
            const content = document.getElementById(sectionId + '-content');
            const toggle = document.getElementById(sectionId + '-toggle');

            if (content.style.display === 'none') {{
                content.style.display = 'block';
                toggle.classList.remove('collapsed');
            }} else {{
                content.style.display = 'none';
                toggle.classList.add('collapsed');
            }}
        }}
    </script>
</body>
</html>"#,
        title = html_escape(title),
        status = if result.passed {
            "✓ PASS"
        } else {
            "✗ FAIL"
        },
        critical_count = critical_count,
        error_count = error_count,
        warning_count = warning_count,
        info_count = info_count,
        error_total = result.errors.len(),
        warnings_total = result.warnings.len(),
        suggestions_total = result.suggestions.len(),
        errors_html = if result.errors.is_empty() {
            "<p class=\"empty\">No errors found</p>".to_string()
        } else {
            result.errors
        .iter().map(| e | { let severity = e.severity(); let severity_str = format!("{}",
        severity) .to_lowercase();
        format!("<div class=\"item {}\" data-severity=\"{}\"><span class=\"severity-badge {}\">{}</span>{}</div>",
        severity_str, severity_str, severity_str, severity, html_escape(& e.to_string()))
        }).collect::< Vec < _ >> ().join("\n")
        },
        warnings_html = if result.warnings.is_empty() {
            "<p class=\"empty\">No warnings found</p>".to_string()
        } else {
            result
                .warnings
                .iter()
                .map(|w| {
                    format!(
                        "<div class=\"item warning\" data-severity=\"warning\">{}</div>",
                        html_escape(w)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        suggestions_html = if result.suggestions.is_empty() {
            "<p class=\"empty\">No suggestions</p>".to_string()
        } else {
            result
                .suggestions
                .iter()
                .map(|s| {
                    format!(
                        "<div class=\"item info\" data-severity=\"info\">{}</div>",
                        html_escape(s)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )
}
/// Generates a PDF report for verification results (requires 'pdf' feature).
///
/// Creates a professional PDF document with verification results,
/// including errors, warnings, and suggestions with proper formatting.
#[cfg(feature = "pdf")]
pub fn generate_pdf_report(
    result: &VerificationResult,
    title: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use fop_render::{PdfBuiltinFont, SimpleDocumentBuilder};

    // A helper that wraps text and writes each line onto the builder,
    // managing page breaks automatically.
    struct PdfWriter {
        builder: SimpleDocumentBuilder,
        y_position: f32,
        line_height: f32,
        left_margin: f32,
        page_top: f32,
        page_bottom: f32,
    }

    impl PdfWriter {
        fn new(title: &str) -> Self {
            Self {
                builder: SimpleDocumentBuilder::new(title),
                y_position: 270.0,
                line_height: 6.0,
                left_margin: 20.0,
                page_top: 270.0,
                page_bottom: 30.0,
            }
        }

        fn add_text(&mut self, text: &str, size: f32, font: PdfBuiltinFont) {
            self.add_text_at(text, size, self.left_margin, font);
        }

        fn add_text_at(&mut self, text: &str, size: f32, x: f32, font: PdfBuiltinFont) {
            if self.y_position < self.page_bottom {
                self.new_page();
            }
            self.builder.text(text, size, x, self.y_position, font);
        }

        fn advance_line(&mut self) {
            self.y_position -= self.line_height;
        }

        fn advance_lines(&mut self, n: f32) {
            self.y_position -= self.line_height * n;
        }

        fn new_page(&mut self) {
            self.builder.new_page();
            self.y_position = self.page_top;
        }

        fn save(self) -> Vec<u8> {
            self.builder.save()
        }
    }

    let mut writer = PdfWriter::new(title);

    writer.add_text(title, 18.0, PdfBuiltinFont::HelveticaBold);
    writer.advance_lines(2.0);

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    writer.add_text(
        &format!("Generated: {}", timestamp),
        10.0,
        PdfBuiltinFont::Helvetica,
    );
    writer.advance_lines(2.0);

    let status_text = if result.passed {
        "Verification Passed"
    } else {
        "Verification Failed"
    };
    writer.add_text(status_text, 14.0, PdfBuiltinFont::HelveticaBold);
    writer.advance_lines(2.0);

    writer.add_text("Errors:", 12.0, PdfBuiltinFont::HelveticaBold);
    writer.advance_line();
    if result.errors.is_empty() {
        writer.add_text("  No errors found", 10.0, PdfBuiltinFont::Helvetica);
        writer.advance_line();
    } else {
        for (idx, error) in result.errors.iter().enumerate() {
            let error_text = format!("  {}. {}", idx + 1, error);
            for line in wrap_text(&error_text, 80) {
                writer.add_text(&line, 10.0, PdfBuiltinFont::Helvetica);
                writer.advance_line();
            }
        }
    }

    writer.advance_line();
    writer.add_text("Warnings:", 12.0, PdfBuiltinFont::HelveticaBold);
    writer.advance_line();
    if result.warnings.is_empty() {
        writer.add_text("  No warnings found", 10.0, PdfBuiltinFont::Helvetica);
        writer.advance_line();
    } else {
        for (idx, warning) in result.warnings.iter().enumerate() {
            let warning_text = format!("  {}. {}", idx + 1, warning);
            for line in wrap_text(&warning_text, 80) {
                writer.add_text(&line, 10.0, PdfBuiltinFont::Helvetica);
                writer.advance_line();
            }
        }
    }

    writer.advance_line();
    writer.add_text("Suggestions:", 12.0, PdfBuiltinFont::HelveticaBold);
    writer.advance_line();
    if result.suggestions.is_empty() {
        writer.add_text("  No suggestions", 10.0, PdfBuiltinFont::Helvetica);
    } else {
        for (idx, suggestion) in result.suggestions.iter().enumerate() {
            let suggestion_text = format!("  {}. {}", idx + 1, suggestion);
            for line in wrap_text(&suggestion_text, 80) {
                writer.add_text(&line, 10.0, PdfBuiltinFont::Helvetica);
                writer.advance_line();
            }
        }
    }

    Ok(writer.save())
}

/// Helper function to wrap text to a specified width.
#[cfg(feature = "pdf")]
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width && !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}
/// Calculates semantic similarity between two statutes.
///
/// The similarity is based on:
/// - Title similarity (Levenshtein distance)
/// - Condition overlap
/// - Effect type similarity
/// - Discretion similarity
pub fn semantic_similarity(statute1: &Statute, statute2: &Statute) -> SimilarityScore {
    let mut similarity = 0.0f64;
    let mut weight_sum = 0.0f64;
    let title_weight = 0.2;
    let title_sim = string_similarity(&statute1.title, &statute2.title);
    similarity += title_sim * title_weight;
    weight_sum += title_weight;
    let effect_weight = 0.3;
    let effect_sim = if statute1.effect.effect_type == statute2.effect.effect_type {
        1.0
    } else {
        0.0
    };
    similarity += effect_sim * effect_weight;
    weight_sum += effect_weight;
    let condition_weight = 0.4;
    let condition_sim =
        condition_overlap_similarity(&statute1.preconditions, &statute2.preconditions);
    similarity += condition_sim * condition_weight;
    weight_sum += condition_weight;
    let discretion_weight = 0.1;
    let discretion_sim = match (&statute1.discretion_logic, &statute2.discretion_logic) {
        (Some(_), Some(_)) => 1.0,
        (None, None) => 1.0,
        _ => 0.0,
    };
    similarity += discretion_sim * discretion_weight;
    weight_sum += discretion_weight;
    SimilarityScore::new(similarity / weight_sum)
}
/// Calculates string similarity using Levenshtein distance.
pub(crate) fn string_similarity(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }
    let distance = levenshtein_distance(s1, s2);
    let max_len = s1.len().max(s2.len());
    1.0 - (distance as f64 / max_len as f64)
}
/// Calculates Levenshtein distance between two strings.
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    let mut matrix = vec![vec![0usize; len2 + 1]; len1 + 1];
    #[allow(clippy::needless_range_loop)]
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[len1][len2]
}
/// Calculates overlap similarity between two sets of conditions.
fn condition_overlap_similarity(
    conditions1: &[legalis_core::Condition],
    conditions2: &[legalis_core::Condition],
) -> f64 {
    if conditions1.is_empty() && conditions2.is_empty() {
        return 1.0;
    }
    if conditions1.is_empty() || conditions2.is_empty() {
        return 0.0;
    }
    let mut matching_pairs = 0;
    let total_comparisons = conditions1.len() * conditions2.len();
    for c1 in conditions1 {
        for c2 in conditions2 {
            if conditions_are_similar(c1, c2) {
                matching_pairs += 1;
            }
        }
    }
    matching_pairs as f64 / total_comparisons as f64
}
/// Checks if two conditions are similar.
fn conditions_are_similar(c1: &legalis_core::Condition, c2: &legalis_core::Condition) -> bool {
    use legalis_core::Condition;
    match (c1, c2) {
        (Condition::Age { .. }, Condition::Age { .. }) => true,
        (Condition::Income { .. }, Condition::Income { .. }) => true,
        (Condition::HasAttribute { key: k1 }, Condition::HasAttribute { key: k2 }) => k1 == k2,
        (
            Condition::AttributeEquals { key: k1, .. },
            Condition::AttributeEquals { key: k2, .. },
        ) => k1 == k2,
        (Condition::DateRange { .. }, Condition::DateRange { .. }) => true,
        (Condition::Geographic { .. }, Condition::Geographic { .. }) => true,
        (Condition::EntityRelationship { .. }, Condition::EntityRelationship { .. }) => true,
        (Condition::ResidencyDuration { .. }, Condition::ResidencyDuration { .. }) => true,
        (Condition::Duration { .. }, Condition::Duration { .. }) => true,
        (Condition::Percentage { .. }, Condition::Percentage { .. }) => true,
        (Condition::SetMembership { .. }, Condition::SetMembership { .. }) => true,
        (Condition::Pattern { .. }, Condition::Pattern { .. }) => true,
        (Condition::Calculation { .. }, Condition::Calculation { .. }) => true,
        (Condition::Custom { description: d1 }, Condition::Custom { description: d2 }) => {
            string_similarity(d1, d2) > 0.7
        }
        (Condition::And(l1, r1), Condition::And(l2, r2)) => {
            conditions_are_similar(l1, l2) && conditions_are_similar(r1, r2)
        }
        (Condition::Or(l1, r1), Condition::Or(l2, r2)) => {
            conditions_are_similar(l1, l2) && conditions_are_similar(r1, r2)
        }
        (Condition::Not(c1), Condition::Not(c2)) => conditions_are_similar(c1, c2),
        _ => false,
    }
}
/// Finds pairs of statutes with high semantic similarity (potential duplicates).
///
/// Returns a list of statute pairs with similarity scores above the threshold.
pub fn find_similar_statutes(
    statutes: &[Statute],
    threshold: f64,
) -> Vec<(String, String, SimilarityScore)> {
    let mut similar_pairs = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let similarity = semantic_similarity(&statutes[i], &statutes[j]);
            if similarity.0 >= threshold {
                similar_pairs.push((statutes[i].id.clone(), statutes[j].id.clone(), similarity));
            }
        }
    }
    similar_pairs
}
/// Common ambiguous legal terms and their potential meanings.
const AMBIGUOUS_LEGAL_TERMS: &[(&str, &[&str])] = &[
    ("person", &["natural person", "legal person", "corporation"]),
    ("child", &["minor", "dependent", "offspring"]),
    (
        "residence",
        &["domicile", "dwelling", "temporary residence"],
    ),
    ("income", &["gross income", "net income", "taxable income"]),
    ("tax", &["income tax", "sales tax", "property tax"]),
    (
        "benefit",
        &["welfare benefit", "tax benefit", "employment benefit"],
    ),
    (
        "disability",
        &[
            "physical disability",
            "mental disability",
            "learning disability",
        ],
    ),
    (
        "family",
        &["immediate family", "extended family", "household"],
    ),
    (
        "spouse",
        &["legal spouse", "common-law spouse", "domestic partner"],
    ),
    (
        "property",
        &[
            "real property",
            "personal property",
            "intellectual property",
        ],
    ),
];
/// Finds ambiguous terms in a set of statutes.
///
/// This function identifies terms that may have multiple meanings
/// and suggests disambiguations based on common legal usage.
pub fn find_ambiguous_terms(statutes: &[Statute]) -> Vec<AmbiguousTerm> {
    let mut ambiguous_terms = HashMap::new();
    for statute in statutes {
        for (term, suggestions) in AMBIGUOUS_LEGAL_TERMS {
            if statute.title.to_lowercase().contains(term) {
                let entry = ambiguous_terms
                    .entry(term.to_string())
                    .or_insert_with(|| AmbiguousTerm::new(*term));
                if !entry.statute_ids.contains(&statute.id) {
                    entry.statute_ids.push(statute.id.clone());
                }
                if !entry.contexts.contains(&statute.title) {
                    entry.contexts.push(statute.title.clone());
                }
                for suggestion in *suggestions {
                    if !entry.suggestions.contains(&suggestion.to_string()) {
                        entry.suggestions.push(suggestion.to_string());
                    }
                }
            }
        }
        if statute.effect.description.to_lowercase().contains("person") {
            let entry = ambiguous_terms
                .entry("person".to_string())
                .or_insert_with(|| AmbiguousTerm::new("person"));
            if !entry.statute_ids.contains(&statute.id) {
                entry.statute_ids.push(statute.id.clone());
            }
            if !entry.contexts.contains(&statute.effect.description) {
                entry.contexts.push(statute.effect.description.clone());
            }
        }
    }
    ambiguous_terms.into_values().collect()
}
/// Generates a term disambiguation report for a set of statutes.
pub fn term_disambiguation_report(statutes: &[Statute]) -> String {
    let ambiguous_terms = find_ambiguous_terms(statutes);
    if ambiguous_terms.is_empty() {
        return "# Term Disambiguation Report\n\nNo ambiguous terms found.\n".to_string();
    }
    let mut report = String::new();
    report.push_str("# Term Disambiguation Report\n\n");
    report.push_str(&format!(
        "Found {} ambiguous terms:\n\n",
        ambiguous_terms.len()
    ));
    for term in &ambiguous_terms {
        report.push_str(&format!("## Term: \"{}\"\n", term.term));
        report.push_str(&format!(
            "- Used in {} statute(s): {}\n",
            term.statute_ids.len(),
            term.statute_ids.join(", ")
        ));
        if !term.contexts.is_empty() {
            report.push_str("- Contexts:\n");
            for context in &term.contexts {
                report.push_str(&format!("  - {}\n", context));
            }
        }
        if !term.suggestions.is_empty() {
            report.push_str("- Suggested disambiguations:\n");
            for suggestion in &term.suggestions {
                report.push_str(&format!("  - {}\n", suggestion));
            }
        }
        report.push('\n');
    }
    report
}
/// Validates cross-references between statutes.
///
/// This function checks that all statute references in conditions
/// point to valid existing statutes.
pub fn validate_cross_references(statutes: &[Statute]) -> Vec<CrossReferenceError> {
    let mut errors = Vec::new();
    let statute_ids: HashSet<&str> = statutes.iter().map(|s| s.id.as_str()).collect();
    for statute in statutes {
        let references = extract_statute_references_from_conditions(&statute.preconditions);
        for reference in references {
            if !statute_ids.contains(reference.as_str()) {
                errors.push(CrossReferenceError {
                    source_statute_id: statute.id.clone(),
                    referenced_statute_id: reference.clone(),
                    error_type: CrossReferenceErrorType::NotFound,
                });
            }
        }
    }
    errors
}
/// Extracts statute references from a list of conditions.
pub(super) fn extract_statute_references_from_conditions(
    conditions: &[legalis_core::Condition],
) -> Vec<String> {
    let mut refs = Vec::new();
    for condition in conditions {
        extract_refs_from_single_condition(condition, &mut refs);
    }
    refs
}
/// Recursively extracts references from a single condition.
fn extract_refs_from_single_condition(condition: &legalis_core::Condition, refs: &mut Vec<String>) {
    use legalis_core::Condition;
    match condition {
        Condition::Custom { description } => {
            if let Some(statute_ref) = description.strip_prefix("statute:") {
                refs.push(statute_ref.trim().to_string());
            }
        }
        Condition::And(left, right) | Condition::Or(left, right) => {
            extract_refs_from_single_condition(left, refs);
            extract_refs_from_single_condition(right, refs);
        }
        Condition::Not(inner) => {
            extract_refs_from_single_condition(inner, refs);
        }
        _ => {}
    }
}
/// Generates a cross-reference validation report.
pub fn cross_reference_report(statutes: &[Statute]) -> String {
    let errors = validate_cross_references(statutes);
    if errors.is_empty() {
        return "# Cross-Reference Validation Report\n\nAll cross-references are valid.\n"
            .to_string();
    }
    let mut report = String::new();
    report.push_str("# Cross-Reference Validation Report\n\n");
    report.push_str(&format!(
        "Found {} cross-reference error(s):\n\n",
        errors.len()
    ));
    for error in &errors {
        report.push_str(&format!("- {}\n", error));
    }
    report
}
/// Common term variations that should be consistent.
const TERM_VARIATIONS: &[(&str, &[&str])] = &[
    ("applicant", &["applicant", "appellant", "petitioner"]),
    ("minor", &["minor", "child", "juvenile", "underage person"]),
    ("guardian", &["guardian", "custodian", "caretaker"]),
    ("income", &["income", "earnings", "revenue", "compensation"]),
    ("residence", &["residence", "domicile", "dwelling", "home"]),
    (
        "employer",
        &["employer", "company", "business", "organization"],
    ),
    (
        "employee",
        &["employee", "worker", "staff member", "personnel"],
    ),
    (
        "benefit",
        &["benefit", "entitlement", "allowance", "payment"],
    ),
    ("disabled", &["disabled", "handicapped", "impaired"]),
    ("spouse", &["spouse", "partner", "husband", "wife"]),
];
/// Checks for terminology consistency across statutes.
///
/// This function identifies where similar terms are used inconsistently
/// and suggests a canonical term for each concept.
pub fn check_terminology_consistency(statutes: &[Statute]) -> Vec<TerminologyInconsistency> {
    let mut inconsistencies = Vec::new();
    for (canonical, variations) in TERM_VARIATIONS {
        let mut found_variations = HashMap::new();
        for statute in statutes {
            let text = format!("{} {}", statute.title, statute.effect.description).to_lowercase();
            for variation in *variations {
                if text.contains(variation) {
                    found_variations
                        .entry(variation.to_string())
                        .or_insert_with(Vec::new)
                        .push(statute.id.clone());
                }
            }
        }
        if found_variations.len() > 1 {
            let mut inconsistency = TerminologyInconsistency::new(*canonical);
            for (variation, statute_ids) in found_variations {
                inconsistency = inconsistency.with_variation(&variation);
                for id in statute_ids {
                    inconsistency = inconsistency.with_statute_id(id);
                }
            }
            inconsistencies.push(inconsistency);
        }
    }
    inconsistencies
}
/// Generates a terminology consistency report.
pub fn terminology_consistency_report(statutes: &[Statute]) -> String {
    let inconsistencies = check_terminology_consistency(statutes);
    if inconsistencies.is_empty() {
        return "# Terminology Consistency Report\n\nTerminology is consistent across all statutes.\n"
            .to_string();
    }
    let mut report = String::new();
    report.push_str("# Terminology Consistency Report\n\n");
    report.push_str(&format!(
        "Found {} terminology inconsistenc(ies):\n\n",
        inconsistencies.len()
    ));
    for inconsistency in &inconsistencies {
        report.push_str(&format!(
            "## Inconsistent use of \"{}\"\n",
            inconsistency.canonical_term
        ));
        report.push_str(&format!(
            "- Found {} variation(s): {}\n",
            inconsistency.variations.len(),
            inconsistency.variations.join(", ")
        ));
        report.push_str(&format!(
            "- Used in {} statute(s): {}\n",
            inconsistency.statute_ids.len(),
            inconsistency.statute_ids.join(", ")
        ));
        report.push_str(&format!(
            "- Recommendation: Use \"{}\" consistently\n\n",
            inconsistency.canonical_term
        ));
    }
    report
}
/// Generates a SARIF (Static Analysis Results Interchange Format) report.
///
/// SARIF is a standard JSON format for static analysis results,
/// supported by many IDEs and CI/CD tools.
pub fn generate_sarif_report(
    result: &VerificationResult,
    tool_name: &str,
    tool_version: &str,
) -> Result<String, serde_json::Error> {
    use serde_json::json;
    let mut results_array = Vec::new();
    for error in &result.errors {
        let (rule_id, message) = match error {
            VerificationError::CircularReference { message } => {
                ("circular-reference", message.clone())
            }
            VerificationError::DeadStatute { statute_id } => (
                "dead-statute",
                format!("Statute '{}' can never be satisfied", statute_id),
            ),
            VerificationError::ConstitutionalConflict {
                statute_id,
                principle,
            } => (
                "constitutional-conflict",
                format!(
                    "Statute '{}' conflicts with constitutional principle '{}'",
                    statute_id, principle
                ),
            ),
            VerificationError::LogicalContradiction { message } => {
                ("logical-contradiction", message.clone())
            }
            VerificationError::Ambiguity { message } => ("ambiguity", message.clone()),
            VerificationError::UnreachableCode { message } => ("unreachable-code", message.clone()),
        };
        results_array.push(json!(
            { "ruleId" : rule_id, "level" : "error", "message" : { "text" :
            message } }
        ));
    }
    for warning in &result.warnings {
        results_array.push(json!(
            { "ruleId" : "warning", "level" : "warning", "message" : { "text" :
            warning } }
        ));
    }
    for suggestion in &result.suggestions {
        results_array.push(json!(
            { "ruleId" : "suggestion", "level" : "note", "message" : { "text" :
            suggestion } }
        ));
    }
    let sarif = json!(
        { "version" : "2.1.0", "$schema" :
        "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "runs" : [{ "tool" : { "driver" : { "name" : tool_name, "version" : tool_version,
        "informationUri" : "https://github.com/yourusername/legalis-rs", "rules" : [{
        "id" : "circular-reference", "name" : "CircularReference", "shortDescription" : {
        "text" : "Circular reference detected between statutes" }, "fullDescription" : {
        "text" :
        "A circular reference occurs when statutes reference each other in a cycle, potentially causing infinite loops."
        }, "helpUri" : "https://docs.legalis-rs.org/errors/circular-reference" }, { "id"
        : "dead-statute", "name" : "DeadStatute", "shortDescription" : { "text" :
        "Statute can never be satisfied" }, "fullDescription" : { "text" :
        "A statute is dead when its preconditions can never be satisfied simultaneously."
        }, "helpUri" : "https://docs.legalis-rs.org/errors/dead-statute" }, { "id" :
        "constitutional-conflict", "name" : "ConstitutionalConflict", "shortDescription"
        : { "text" : "Statute conflicts with constitutional principle" },
        "fullDescription" : { "text" :
        "A statute violates one or more constitutional principles." }, "helpUri" :
        "https://docs.legalis-rs.org/errors/constitutional-conflict" }, { "id" :
        "logical-contradiction", "name" : "LogicalContradiction", "shortDescription" : {
        "text" : "Logical contradiction between statutes" }, "fullDescription" : { "text"
        : "Two or more statutes have contradictory effects under the same conditions." },
        "helpUri" : "https://docs.legalis-rs.org/errors/logical-contradiction" }, { "id"
        : "ambiguity", "name" : "Ambiguity", "shortDescription" : { "text" :
        "Ambiguity detected in statute" }, "fullDescription" : { "text" :
        "A statute contains ambiguous language or conditions that may lead to multiple interpretations."
        }, "helpUri" : "https://docs.legalis-rs.org/errors/ambiguity" }] } }, "results" :
        results_array }] }
    );
    serde_json::to_string_pretty(&sarif)
}
/// Converts verification results to IDE diagnostics.
pub fn to_ide_diagnostics(result: &VerificationResult) -> Vec<IdeDiagnostic> {
    let mut diagnostics = Vec::new();
    for error in &result.errors {
        let severity_level = match error.severity() {
            Severity::Critical => "error",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "information",
        };
        let code = match error {
            VerificationError::CircularReference { .. } => "L001",
            VerificationError::DeadStatute { .. } => "L002",
            VerificationError::ConstitutionalConflict { .. } => "L003",
            VerificationError::LogicalContradiction { .. } => "L004",
            VerificationError::Ambiguity { .. } => "L005",
            VerificationError::UnreachableCode { .. } => "L006",
        };
        diagnostics.push(IdeDiagnostic::new(severity_level, error.to_string()).with_code(code));
    }
    for warning in &result.warnings {
        diagnostics.push(IdeDiagnostic::new("warning", warning));
    }
    for suggestion in &result.suggestions {
        diagnostics.push(IdeDiagnostic::new("hint", suggestion));
    }
    diagnostics
}
/// Generates LSP-compatible diagnostic JSON output.
pub fn generate_lsp_diagnostics(result: &VerificationResult) -> Result<String, serde_json::Error> {
    let diagnostics = to_ide_diagnostics(result);
    serde_json::to_string_pretty(&diagnostics)
}
/// Generates quick fixes for common verification errors.
pub fn generate_quick_fixes(error: &VerificationError) -> Vec<QuickFix> {
    match error {
        VerificationError::CircularReference { message } => {
            vec![
                QuickFix::new(
                    "Break circular reference",
                    format!("Remove circular dependency: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
        VerificationError::DeadStatute { statute_id } => {
            vec![
                QuickFix::new(
                    "Fix unsatisfiable conditions",
                    format!("Review and fix conditions in statute {}", statute_id),
                )
                .with_kind("quickfix"),
            ]
        }
        VerificationError::ConstitutionalConflict {
            statute_id,
            principle,
        } => {
            vec![
                QuickFix::new(
                    "Resolve constitutional conflict",
                    format!(
                        "Update statute {} to comply with principle: {}",
                        statute_id, principle
                    ),
                )
                .with_kind("quickfix"),
            ]
        }
        VerificationError::LogicalContradiction { message } => {
            vec![
                QuickFix::new(
                    "Resolve logical contradiction",
                    format!("Fix contradictory logic: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
        VerificationError::Ambiguity { message } => {
            vec![
                QuickFix::new(
                    "Clarify ambiguous language",
                    format!("Make language more specific: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
        VerificationError::UnreachableCode { message } => {
            vec![
                QuickFix::new(
                    "Remove unreachable code",
                    format!("Delete or refactor unreachable code: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
    }
}
/// Checks if an LTL formula holds in a transition system.
///
/// This is a simplified model checker that verifies LTL properties
/// over finite traces. For production use, consider using a dedicated
/// model checker like SPIN or NuSMV.
pub fn verify_ltl(system: &TransitionSystem, formula: &LtlFormula) -> bool {
    for initial_id in &system.initial_states {
        if let Some(initial_state) = system.states.get(initial_id) {
            let mut visited = HashSet::new();
            if !check_ltl_from_state(system, initial_state, formula, &mut visited) {
                return false;
            }
        }
    }
    true
}
/// Helper function to check LTL from a specific state.
#[allow(dead_code)]
fn check_ltl_from_state(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &LtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return true;
    }
    visited.insert(state.id.clone());
    match formula {
        LtlFormula::Atom(prop) => state.satisfies(prop),
        LtlFormula::Not(f) => !check_ltl_from_state(system, state, f, visited),
        LtlFormula::And(left, right) => {
            check_ltl_from_state(system, state, left, visited)
                && check_ltl_from_state(system, state, right, visited)
        }
        LtlFormula::Or(left, right) => {
            check_ltl_from_state(system, state, left, visited)
                || check_ltl_from_state(system, state, right, visited)
        }
        LtlFormula::Implies(left, right) => {
            !check_ltl_from_state(system, state, left, visited)
                || check_ltl_from_state(system, state, right, visited)
        }
        LtlFormula::Next(f) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors
                .iter()
                .all(|s| check_ltl_from_state(system, s, f, visited))
        }
        LtlFormula::Eventually(f) => {
            check_eventually(system, state, f, visited, &mut HashSet::new())
        }
        LtlFormula::Always(f) => check_always(system, state, f, visited),
        LtlFormula::Until(left, right) => {
            check_until(system, state, left, right, visited, &mut HashSet::new())
        }
        LtlFormula::Release(left, right) => {
            let not_p = LtlFormula::not(*left.clone());
            let not_q = LtlFormula::not(*right.clone());
            !check_until(system, state, &not_p, &not_q, visited, &mut HashSet::new())
        }
    }
}
#[allow(dead_code)]
fn check_eventually(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &LtlFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    if path_visited.contains(&state.id) {
        return false;
    }
    path_visited.insert(state.id.clone());
    if check_ltl_from_state(system, state, formula, visited) {
        return true;
    }
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_eventually(system, s, formula, visited, path_visited))
}
#[allow(dead_code)]
fn check_always(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &LtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if !check_ltl_from_state(system, state, formula, visited) {
        return false;
    }
    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return true;
    }
    successors
        .iter()
        .all(|s| check_always(system, s, formula, visited))
}
#[allow(dead_code)]
fn check_until(
    system: &TransitionSystem,
    state: &TemporalState,
    left: &LtlFormula,
    right: &LtlFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    if path_visited.contains(&state.id) {
        return false;
    }
    path_visited.insert(state.id.clone());
    if check_ltl_from_state(system, state, right, visited) {
        return true;
    }
    if !check_ltl_from_state(system, state, left, visited) {
        return false;
    }
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_until(system, s, left, right, visited, path_visited))
}
/// Checks if a CTL formula holds in a transition system.
pub fn verify_ctl(system: &TransitionSystem, formula: &CtlFormula) -> bool {
    for initial_id in &system.initial_states {
        if let Some(initial_state) = system.states.get(initial_id)
            && !check_ctl_from_state(system, initial_state, formula)
        {
            return false;
        }
    }
    true
}
#[allow(dead_code)]
fn check_ctl_from_state(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
) -> bool {
    match formula {
        CtlFormula::Atom(prop) => state.satisfies(prop),
        CtlFormula::Not(f) => !check_ctl_from_state(system, state, f),
        CtlFormula::And(left, right) => {
            check_ctl_from_state(system, state, left) && check_ctl_from_state(system, state, right)
        }
        CtlFormula::Or(left, right) => {
            check_ctl_from_state(system, state, left) || check_ctl_from_state(system, state, right)
        }
        CtlFormula::Implies(left, right) => {
            !check_ctl_from_state(system, state, left) || check_ctl_from_state(system, state, right)
        }
        CtlFormula::ExistsNext(f) => {
            let successors = system.successors(&state.id);
            successors
                .iter()
                .any(|s| check_ctl_from_state(system, s, f))
        }
        CtlFormula::AllNext(f) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors
                .iter()
                .all(|s| check_ctl_from_state(system, s, f))
        }
        CtlFormula::ExistsEventually(f) => {
            check_ctl_exists_eventually(system, state, f, &mut HashSet::new())
        }
        CtlFormula::AllEventually(f) => {
            check_ctl_all_eventually(system, state, f, &mut HashSet::new())
        }
        CtlFormula::ExistsAlways(f) => {
            check_ctl_exists_always(system, state, f, &mut HashSet::new())
        }
        CtlFormula::AllAlways(f) => check_ctl_all_always(system, state, f, &mut HashSet::new()),
        CtlFormula::ExistsUntil(left, right) => {
            check_ctl_exists_until(system, state, left, right, &mut HashSet::new())
        }
        CtlFormula::AllUntil(left, right) => {
            check_ctl_all_until(system, state, left, right, &mut HashSet::new())
        }
    }
}
#[allow(dead_code)]
fn check_ctl_exists_eventually(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());
    if check_ctl_from_state(system, state, formula) {
        return true;
    }
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_ctl_exists_eventually(system, s, formula, visited))
}
#[allow(dead_code)]
fn check_ctl_all_eventually(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());
    if check_ctl_from_state(system, state, formula) {
        return true;
    }
    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return false;
    }
    successors
        .iter()
        .all(|s| check_ctl_all_eventually(system, s, formula, visited))
}
#[allow(dead_code)]
fn check_ctl_exists_always(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if !check_ctl_from_state(system, state, formula) {
        return false;
    }
    if visited.contains(&state.id) {
        return true;
    }
    visited.insert(state.id.clone());
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_ctl_exists_always(system, s, formula, visited))
}
#[allow(dead_code)]
fn check_ctl_all_always(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if !check_ctl_from_state(system, state, formula) {
        return false;
    }
    if visited.contains(&state.id) {
        return true;
    }
    visited.insert(state.id.clone());
    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return true;
    }
    successors
        .iter()
        .all(|s| check_ctl_all_always(system, s, formula, visited))
}
#[allow(dead_code)]
fn check_ctl_exists_until(
    system: &TransitionSystem,
    state: &TemporalState,
    left: &CtlFormula,
    right: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());
    if check_ctl_from_state(system, state, right) {
        return true;
    }
    if !check_ctl_from_state(system, state, left) {
        return false;
    }
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_ctl_exists_until(system, s, left, right, visited))
}
#[allow(dead_code)]
fn check_ctl_all_until(
    system: &TransitionSystem,
    state: &TemporalState,
    left: &CtlFormula,
    right: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());
    if check_ctl_from_state(system, state, right) {
        return true;
    }
    if !check_ctl_from_state(system, state, left) {
        return false;
    }
    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return false;
    }
    successors
        .iter()
        .all(|s| check_ctl_all_until(system, s, left, right, visited))
}
/// Verifies deadlines in a transition system.
pub fn verify_deadlines(
    system: &TransitionSystem,
    deadlines: &[Deadline],
) -> DeadlineVerificationResult {
    let mut violations = Vec::new();
    for deadline in deadlines {
        for initial_id in &system.initial_states {
            if let Some(initial_state) = system.states.get(initial_id) {
                let steps = count_steps_to_event(
                    system,
                    initial_state,
                    &deadline.event,
                    &mut HashSet::new(),
                );
                if let Some(actual_steps) = steps {
                    if actual_steps > deadline.max_steps {
                        violations.push(DeadlineViolation {
                            deadline_id: deadline.id.clone(),
                            actual_steps,
                            max_steps: deadline.max_steps,
                            description: format!(
                                "Event '{}' occurred after {} steps (deadline: {} steps)",
                                deadline.event, actual_steps, deadline.max_steps
                            ),
                        });
                    }
                } else if deadline.max_steps < usize::MAX {
                    violations.push(DeadlineViolation {
                        deadline_id: deadline.id.clone(),
                        actual_steps: usize::MAX,
                        max_steps: deadline.max_steps,
                        description: format!(
                            "Event '{}' never occurs (deadline: {} steps)",
                            deadline.event, deadline.max_steps
                        ),
                    });
                }
            }
        }
    }
    DeadlineVerificationResult {
        passed: violations.is_empty(),
        violations,
    }
}
#[allow(dead_code)]
fn count_steps_to_event(
    system: &TransitionSystem,
    state: &TemporalState,
    event: &str,
    visited: &mut HashSet<String>,
) -> Option<usize> {
    if visited.contains(&state.id) {
        return None;
    }
    visited.insert(state.id.clone());
    if state.satisfies(event) {
        return Some(0);
    }
    let successors = system.successors(&state.id);
    let mut min_steps = None;
    for successor in successors {
        if let Some(steps) = count_steps_to_event(system, successor, event, visited) {
            let total = steps + 1;
            min_steps = Some(min_steps.map_or(total, |current: usize| current.min(total)));
        }
    }
    min_steps
}
/// Verifies sequence constraints in a transition system.
pub fn verify_sequences(
    system: &TransitionSystem,
    constraints: &[SequenceConstraint],
) -> SequenceVerificationResult {
    let mut violations = Vec::new();
    for constraint in constraints {
        for initial_id in &system.initial_states {
            if let Some(initial_state) = system.states.get(initial_id)
                && !check_sequence(
                    system,
                    initial_state,
                    &constraint.events,
                    0,
                    constraint.strict,
                    &mut HashSet::new(),
                )
            {
                violations.push(SequenceViolation {
                    constraint_id: constraint.id.clone(),
                    description: format!(
                        "Required event sequence {:?} was not followed",
                        constraint.events
                    ),
                    violating_events: constraint.events.clone(),
                });
                break;
            }
        }
    }
    SequenceVerificationResult {
        passed: violations.is_empty(),
        violations,
    }
}
#[allow(dead_code)]
fn check_sequence(
    system: &TransitionSystem,
    state: &TemporalState,
    events: &[String],
    current_index: usize,
    strict: bool,
    visited: &mut HashSet<(String, usize)>,
) -> bool {
    let key = (state.id.clone(), current_index);
    if visited.contains(&key) {
        return false;
    }
    visited.insert(key);
    if current_index >= events.len() {
        return true;
    }
    let current_event = &events[current_index];
    if state.satisfies(current_event) {
        let successors = system.successors(&state.id);
        return successors
            .iter()
            .any(|s| check_sequence(system, s, events, current_index + 1, strict, visited))
            || (current_index + 1 >= events.len());
    }
    if strict {
        return false;
    }
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_sequence(system, s, events, current_index, strict, visited))
}
/// Verifies a CTL* formula on a transition system.
///
/// CTL* combines the expressiveness of both CTL and LTL, allowing
/// arbitrary nesting of path quantifiers and temporal operators.
pub fn verify_ctl_star(system: &TransitionSystem, formula: &CtlStarFormula) -> bool {
    for initial_id in &system.initial_states {
        if let Some(initial_state) = system.states.get(initial_id)
            && !check_ctl_star_state(system, initial_state, formula, &mut HashSet::new())
        {
            return false;
        }
    }
    true
}
#[allow(dead_code)]
pub(super) fn check_ctl_star_state(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlStarFormula,
    visited: &mut HashSet<String>,
) -> bool {
    match formula {
        CtlStarFormula::Atom(prop) => state.satisfies(prop),
        CtlStarFormula::Not(f) => !check_ctl_star_state(system, state, f, visited),
        CtlStarFormula::And(left, right) => {
            check_ctl_star_state(system, state, left, visited)
                && check_ctl_star_state(system, state, right, visited)
        }
        CtlStarFormula::Or(left, right) => {
            check_ctl_star_state(system, state, left, visited)
                || check_ctl_star_state(system, state, right, visited)
        }
        CtlStarFormula::Implies(left, right) => {
            !check_ctl_star_state(system, state, left, visited)
                || check_ctl_star_state(system, state, right, visited)
        }
        CtlStarFormula::Exists(path) => check_ctl_star_exists_path(system, state, path, visited),
        CtlStarFormula::All(path) => check_ctl_star_all_paths(system, state, path, visited),
    }
}
#[allow(dead_code)]
fn check_ctl_star_exists_path(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
) -> bool {
    check_ctl_star_path(system, state, path, visited, &mut HashSet::new())
}
#[allow(dead_code)]
fn check_ctl_star_all_paths(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
) -> bool {
    check_ctl_star_path_universal(system, state, path, visited, &mut HashSet::new())
}
