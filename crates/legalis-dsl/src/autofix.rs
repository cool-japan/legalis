//! Auto-fix suggestions for common DSL errors and anti-patterns
//!
//! This module provides automatic fix suggestions for common issues found
//! in legal DSL code, including syntax errors, deprecated patterns, and
//! code quality improvements.

use crate::ast::*;
use regex::Regex;
use std::collections::HashMap;

/// Represents a suggested fix for a DSL issue
#[derive(Debug, Clone, PartialEq)]
pub struct Fix {
    /// Description of the fix
    pub description: String,
    /// The original problematic code
    pub original: String,
    /// The suggested replacement
    pub replacement: String,
    /// Location in the source (line number, if available)
    pub location: Option<usize>,
    /// Category of the fix
    pub category: FixCategory,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

/// Category of auto-fix
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixCategory {
    /// Syntax error fix
    Syntax,
    /// Deprecated pattern replacement
    Deprecation,
    /// Code style improvement
    Style,
    /// Performance optimization
    Performance,
    /// Semantic error fix
    Semantic,
    /// Best practice suggestion
    BestPractice,
}

/// Auto-fix analyzer
#[derive(Debug)]
pub struct AutoFixer {
    patterns: Vec<FixPattern>,
}

/// A pattern for detecting and fixing issues
#[derive(Debug, Clone)]
pub struct FixPattern {
    /// Pattern to match the issue
    pub matcher: Regex,
    /// Description of the issue
    pub description: String,
    /// Template for the fix
    pub fix_template: String,
    /// Category of the fix
    pub category: FixCategory,
    /// Confidence level
    pub confidence: f64,
}

impl Default for AutoFixer {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoFixer {
    /// Creates a new auto-fixer with default patterns
    pub fn new() -> Self {
        let mut fixer = Self {
            patterns: Vec::new(),
        };
        fixer.load_default_patterns();
        fixer
    }

    /// Loads default fix patterns
    fn load_default_patterns(&mut self) {
        // Fix missing quotes in JURISDICTION
        self.add_pattern(
            r"JURISDICTION\s+([A-Za-z0-9\-]+)\s*(?:\n|$)",
            "Add quotes around jurisdiction identifier",
            "JURISDICTION \"$1\"",
            FixCategory::Syntax,
            0.95,
        );

        // Fix deprecated THEN syntax without effect type
        self.add_pattern(
            r"THEN\s+(.+?)(?:\s*$)",
            "Specify effect type (GRANT, REVOKE, OBLIGATION, or PROHIBITION)",
            "THEN GRANT $1",
            FixCategory::Deprecation,
            0.85,
        );

        // Fix redundant AND in conditions
        self.add_pattern(
            r"WHEN\s+(.+?)\s+AND\s+AND\s+(.+)",
            "Remove duplicate AND operator",
            "WHEN $1 AND $2",
            FixCategory::Syntax,
            0.98,
        );

        // Fix redundant OR in conditions
        self.add_pattern(
            r"WHEN\s+(.+?)\s+OR\s+OR\s+(.+)",
            "Remove duplicate OR operator",
            "WHEN $1 OR $2",
            FixCategory::Syntax,
            0.98,
        );

        // Fix malformed BETWEEN without AND
        self.add_pattern(
            r"BETWEEN\s+(\d+)\s+(\d+)",
            "Add AND keyword in BETWEEN expression",
            "BETWEEN $1 AND $2",
            FixCategory::Syntax,
            0.95,
        );

        // Fix unquoted string values in conditions
        self.add_pattern(
            r"([A-Za-z_]\w+)\s*=\s*([A-Za-z_]\w+)\s*(?:\n|AND|OR|$)",
            "Quote string value in comparison",
            "$1 = \"$2\"",
            FixCategory::Syntax,
            0.80,
        );

        // Fix missing colon after statute ID
        self.add_pattern(
            r"STATUTE\s+([A-Za-z0-9\-_]+)\s+(.+?)(?:\s*\{|$)",
            "Add colon after statute ID",
            "STATUTE $1: $2",
            FixCategory::Syntax,
            0.95,
        );

        // Fix inconsistent spacing in conditions
        self.add_pattern(
            r"AGE>=(\d+)",
            "Add space around comparison operator",
            "AGE >= $1",
            FixCategory::Style,
            0.90,
        );

        self.add_pattern(
            r"AGE<=(\d+)",
            "Add space around comparison operator",
            "AGE <= $1",
            FixCategory::Style,
            0.90,
        );

        self.add_pattern(
            r"AGE>(\d+)",
            "Add space around comparison operator",
            "AGE > $1",
            FixCategory::Style,
            0.90,
        );

        self.add_pattern(
            r"AGE<(\d+)",
            "Add space around comparison operator",
            "AGE < $1",
            FixCategory::Style,
            0.90,
        );

        // Fix reversed BETWEEN range
        self.add_pattern(
            r"BETWEEN\s+(\d+)\s+AND\s+(\d+)",
            "Ensure BETWEEN range is in correct order (min AND max)",
            "", // Special handling needed
            FixCategory::Semantic,
            0.85,
        );

        // Fix missing opening brace (simplified pattern)
        self.add_pattern(
            r"STATUTE\s+([A-Za-z0-9\-_]+):\s*(.+?)\s+WHEN",
            "Add opening brace after statute title",
            "STATUTE $1: $2 { WHEN",
            FixCategory::Syntax,
            0.95,
        );

        // Fix double negation
        self.add_pattern(
            r"NOT\s+NOT\s+(.+)",
            "Remove double negation",
            "$1",
            FixCategory::Style,
            0.95,
        );

        // Fix tautology (always true)
        self.add_pattern(
            r"(.+?)\s+OR\s+NOT\s+\1",
            "Remove tautology (always true)",
            "true",
            FixCategory::Semantic,
            0.90,
        );

        // Fix contradiction (always false)
        self.add_pattern(
            r"(.+?)\s+AND\s+NOT\s+\1",
            "Remove contradiction (always false)",
            "false",
            FixCategory::Semantic,
            0.90,
        );

        // Fix deprecated EFFECTIVE syntax
        self.add_pattern(
            r"EFFECTIVE\s+(\d{4}-\d{2}-\d{2})",
            "Use EFFECTIVE_DATE instead of EFFECTIVE",
            "EFFECTIVE_DATE $1",
            FixCategory::Deprecation,
            0.95,
        );

        // Fix deprecated EXPIRY syntax
        self.add_pattern(
            r"EXPIRY\s+(\d{4}-\d{2}-\d{2})",
            "Use EXPIRY_DATE instead of EXPIRY",
            "EXPIRY_DATE $1",
            FixCategory::Deprecation,
            0.95,
        );

        // Fix missing VERSION number type
        self.add_pattern(
            r#"VERSION\s+"([0-9.]+)""#,
            "Remove quotes from version number",
            "VERSION $1",
            FixCategory::Syntax,
            0.95,
        );

        // Fix incorrect IN syntax with single value
        self.add_pattern(
            r"IN\s+([^(][^\s,)]+)",
            "Wrap IN value in parentheses",
            "IN ($1)",
            FixCategory::Syntax,
            0.90,
        );
    }

    /// Adds a custom fix pattern
    #[allow(dead_code)]
    pub fn add_pattern(
        &mut self,
        pattern: &str,
        description: &str,
        fix_template: &str,
        category: FixCategory,
        confidence: f64,
    ) {
        if let Ok(regex) = Regex::new(pattern) {
            self.patterns.push(FixPattern {
                matcher: regex,
                description: description.to_string(),
                fix_template: fix_template.to_string(),
                category,
                confidence,
            });
        }
    }

    /// Analyzes DSL code and suggests fixes
    pub fn analyze(&self, code: &str) -> Vec<Fix> {
        let mut fixes = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            for pattern in &self.patterns {
                if let Some(captures) = pattern.matcher.captures(line) {
                    let original = captures.get(0).unwrap().as_str().to_string();
                    let mut replacement = pattern.fix_template.clone();

                    // Replace capture groups
                    for i in 1..captures.len() {
                        if let Some(capture) = captures.get(i) {
                            let placeholder = format!("${}", i);
                            replacement = replacement.replace(&placeholder, capture.as_str());
                        }
                    }

                    // Special handling for reversed BETWEEN range
                    if pattern.description.contains("BETWEEN range") {
                        if let (Some(min_cap), Some(max_cap)) = (captures.get(1), captures.get(2))
                            && let (Ok(min), Ok(max)) = (
                                min_cap.as_str().parse::<i64>(),
                                max_cap.as_str().parse::<i64>(),
                            )
                            && min > max
                        {
                            replacement = format!("BETWEEN {} AND {}", max, min);
                            fixes.push(Fix {
                                description: pattern.description.clone(),
                                original: original.clone(),
                                replacement,
                                location: Some(line_num + 1),
                                category: pattern.category,
                                confidence: pattern.confidence,
                            });
                        }
                        continue;
                    }

                    if !replacement.is_empty() && replacement != original {
                        fixes.push(Fix {
                            description: pattern.description.clone(),
                            original,
                            replacement,
                            location: Some(line_num + 1),
                            category: pattern.category,
                            confidence: pattern.confidence,
                        });
                    }
                }
            }
        }

        fixes
    }

    /// Analyzes a LegalDocument AST and suggests semantic fixes
    pub fn analyze_ast(&self, doc: &LegalDocument) -> Vec<Fix> {
        let mut fixes = Vec::new();

        for statute in &doc.statutes {
            // Check for empty statutes
            if statute.conditions.is_empty() && statute.effects.is_empty() {
                fixes.push(Fix {
                    description: "Statute has no conditions or effects".to_string(),
                    original: format!("STATUTE {}", statute.id),
                    replacement: format!("// TODO: Add conditions and effects to {}", statute.id),
                    location: None,
                    category: FixCategory::BestPractice,
                    confidence: 0.95,
                });
            }

            // Check for conditions without effects
            if !statute.conditions.is_empty() && statute.effects.is_empty() {
                fixes.push(Fix {
                    description: "Statute has conditions but no effects".to_string(),
                    original: format!("STATUTE {}", statute.id),
                    replacement: format!(
                        "// Add THEN clause to specify effects for {}",
                        statute.id
                    ),
                    location: None,
                    category: FixCategory::BestPractice,
                    confidence: 0.90,
                });
            }

            // Check for unreachable effects (contradictory conditions)
            fixes.extend(self.check_contradictions(&statute.conditions, &statute.id));

            // Check for redundant conditions
            fixes.extend(self.check_redundant_conditions(&statute.conditions, &statute.id));
        }

        fixes
    }

    /// Checks for contradictory conditions
    fn check_contradictions(&self, _conditions: &[ConditionNode], _statute_id: &str) -> Vec<Fix> {
        // This is a simplified check - a full implementation would use SMT solver
        // For now, we just check for obvious contradictions like "AGE > 18 AND AGE < 18"

        // TODO: Implement proper contradiction detection

        Vec::new()
    }

    /// Checks for redundant conditions
    fn check_redundant_conditions(
        &self,
        conditions: &[ConditionNode],
        statute_id: &str,
    ) -> Vec<Fix> {
        let mut fixes = Vec::new();

        // Check for duplicate conditions
        let mut seen = HashMap::new();
        for (i, cond) in conditions.iter().enumerate() {
            let key = format!("{:?}", cond);
            if let Some(prev_idx) = seen.get(&key) {
                fixes.push(Fix {
                    description: format!(
                        "Duplicate condition in statute {} (condition {} is same as {})",
                        statute_id,
                        i + 1,
                        prev_idx + 1
                    ),
                    original: format!("{:?}", cond),
                    replacement: "// Remove duplicate condition".to_string(),
                    location: None,
                    category: FixCategory::Style,
                    confidence: 0.85,
                });
            }
            seen.insert(key, i);
        }

        fixes
    }

    /// Applies all high-confidence fixes automatically
    pub fn apply_fixes(&self, code: &str, min_confidence: f64) -> String {
        let fixes = self.analyze(code);
        let mut result = code.to_string();

        // Apply fixes in reverse order (by line number) to preserve positions
        let mut sorted_fixes: Vec<_> = fixes
            .into_iter()
            .filter(|f| f.confidence >= min_confidence)
            .collect();
        sorted_fixes.sort_by(|a, b| b.location.cmp(&a.location));

        for fix in sorted_fixes {
            result = result.replace(&fix.original, &fix.replacement);
        }

        result
    }

    /// Generates a fix report
    pub fn generate_report(&self, code: &str) -> FixReport {
        let fixes = self.analyze(code);

        let mut by_category = HashMap::new();
        for fix in &fixes {
            by_category
                .entry(fix.category)
                .or_insert_with(Vec::new)
                .push(fix.clone());
        }

        FixReport {
            total_issues: fixes.len(),
            fixes,
            by_category,
        }
    }
}

/// Report of all detected issues and suggested fixes
#[derive(Debug)]
pub struct FixReport {
    /// Total number of issues found
    pub total_issues: usize,
    /// All suggested fixes
    pub fixes: Vec<Fix>,
    /// Fixes grouped by category
    pub by_category: HashMap<FixCategory, Vec<Fix>>,
}

impl FixReport {
    /// Formats the report as human-readable text
    pub fn format(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Found {} issues:\n\n", self.total_issues));

        for (category, fixes) in &self.by_category {
            output.push_str(&format!("{:?} ({} issues):\n", category, fixes.len()));
            for fix in fixes {
                output.push_str(&format!(
                    "  Line {}: {}\n",
                    fix.location.unwrap_or(0),
                    fix.description
                ));
                output.push_str(&format!("    Original: {}\n", fix.original));
                output.push_str(&format!("    Fix:      {}\n", fix.replacement));
                output.push_str(&format!("    Confidence: {:.0}%\n", fix.confidence * 100.0));
                output.push('\n');
            }
        }

        output
    }

    /// Returns high-confidence fixes (>= 0.9)
    pub fn high_confidence_fixes(&self) -> Vec<&Fix> {
        self.fixes.iter().filter(|f| f.confidence >= 0.9).collect()
    }

    /// Returns fixes by category
    pub fn fixes_by_category(&self, category: FixCategory) -> Vec<&Fix> {
        self.fixes
            .iter()
            .filter(|f| f.category == category)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_missing_jurisdiction_quotes() {
        let fixer = AutoFixer::new();
        let code = "JURISDICTION US-CA\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].category, FixCategory::Syntax);
        assert!(fixes[0].replacement.contains("\"US-CA\""));
    }

    #[test]
    fn test_fix_redundant_and() {
        let fixer = AutoFixer::new();
        let code = "WHEN AGE > 18 AND AND INCOME < 50000\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].category, FixCategory::Syntax);
        assert!(
            fixes[0]
                .replacement
                .contains("WHEN AGE > 18 AND INCOME < 50000")
        );
    }

    #[test]
    fn test_fix_malformed_between() {
        let fixer = AutoFixer::new();
        let code = "BETWEEN 18 65\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert!(fixes[0].replacement.contains("AND"));
    }

    #[test]
    fn test_fix_reversed_between_range() {
        let fixer = AutoFixer::new();
        let code = "BETWEEN 65 AND 18\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].category, FixCategory::Semantic);
        assert_eq!(fixes[0].replacement, "BETWEEN 18 AND 65");
    }

    #[test]
    fn test_fix_spacing_in_operators() {
        let fixer = AutoFixer::new();
        let code = "AGE>=18\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].category, FixCategory::Style);
        assert!(fixes[0].replacement.contains("AGE >= 18"));
    }

    #[test]
    fn test_fix_deprecated_effective() {
        let fixer = AutoFixer::new();
        let code = "EFFECTIVE 2024-01-01\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].category, FixCategory::Deprecation);
        assert!(fixes[0].replacement.contains("EFFECTIVE_DATE"));
    }

    #[test]
    fn test_apply_fixes() {
        let fixer = AutoFixer::new();
        let code = "JURISDICTION US-CA\nEFFECTIVE 2024-01-01\n";
        let fixed = fixer.apply_fixes(code, 0.9);

        assert!(fixed.contains("\"US-CA\""));
        assert!(fixed.contains("EFFECTIVE_DATE"));
    }

    #[test]
    fn test_generate_report() {
        let fixer = AutoFixer::new();
        let code = "JURISDICTION US-CA\nAGE>=18\n";
        let report = fixer.generate_report(code);

        assert_eq!(report.total_issues, 2);
        assert!(report.by_category.contains_key(&FixCategory::Syntax));
        assert!(report.by_category.contains_key(&FixCategory::Style));
    }

    #[test]
    fn test_high_confidence_fixes() {
        let fixer = AutoFixer::new();
        let code = "JURISDICTION US-CA\nAGE>=18\n";
        let report = fixer.generate_report(code);
        let high_conf = report.high_confidence_fixes();

        assert!(!high_conf.is_empty());
        assert!(high_conf.iter().all(|f| f.confidence >= 0.9));
    }

    #[test]
    fn test_fix_double_negation() {
        let fixer = AutoFixer::new();
        let code = "NOT NOT citizen\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].category, FixCategory::Style);
        assert_eq!(fixes[0].replacement, "citizen");
    }

    #[test]
    fn test_fix_version_quotes() {
        let fixer = AutoFixer::new();
        let code = "VERSION \"1.0\"\n";
        let fixes = fixer.analyze(code);

        assert!(!fixes.is_empty());
        assert_eq!(fixes[0].replacement, "VERSION 1.0");
    }

    #[test]
    fn test_format_report() {
        let fixer = AutoFixer::new();
        let code = "JURISDICTION US-CA\n";
        let report = fixer.generate_report(code);
        let formatted = report.format();

        assert!(formatted.contains("Found 1 issues"));
        assert!(formatted.contains("Syntax"));
    }
}
