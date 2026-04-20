//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ChangeType, StatuteDiff};

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
    html.push_str("<div class=\"header\">\n");
    html.push_str(&format!(
        "<h1>Diff for {} (Severity: {:?})</h1>\n",
        diff.statute_id, diff.impact.severity
    ));
    html.push_str("</div>\n");
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
    html.push_str(
        &format!(
            "<div class=\"stat\"><span class=\"stat-label\">Added:</span> <span class=\"stat-value add\">+{}</span></div>\n",
            added_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"stat\"><span class=\"stat-label\">Removed:</span> <span class=\"stat-value remove\">-{}</span></div>\n",
            removed_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"stat\"><span class=\"stat-label\">Modified:</span> <span class=\"stat-value modify\">~{}</span></div>\n",
            modified_count
        ),
    );
    html.push_str("</div>\n");
    html.push_str("<div class=\"diff-view\">\n");
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
pub(super) fn syntax_highlight(text: &str) -> String {
    let mut result = text.to_string();
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
    for op in &[">=", "<=", "==", "!=", ">", "<"] {
        result = result.replace(op, &format!("<span class=\"operator\">{}</span>", op));
    }
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
}
