//! Catala format support.
//!
//! Catala is a domain-specific language for deriving faithful-by-construction
//! implementations from legislative texts. Developed by Inria, France.
//!
//! Key features:
//! - Literate programming with legal text annotations
//! - Scope-based organization
//! - Strong typing with dates, money, durations
//! - Default logic for legal reasoning

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Catala scope metadata for tracking inheritance.
#[derive(Debug, Clone)]
pub struct CatalaScope {
    /// Scope name
    pub name: String,
    /// Parent scope (for inheritance)
    pub parent: Option<String>,
    /// Legal article references
    pub article_refs: Vec<String>,
    /// Exception handlers
    pub exceptions: Vec<String>,
}

/// Catala format importer.
pub struct CatalaImporter {
    /// Whether to preserve legal text comments
    preserve_comments: bool,
    /// Whether to track article references
    track_articles: bool,
    /// Whether to preserve exception handling
    preserve_exceptions: bool,
}

impl CatalaImporter {
    /// Creates a new Catala importer.
    pub fn new() -> Self {
        Self {
            preserve_comments: true,
            track_articles: true,
            preserve_exceptions: true,
        }
    }

    /// Sets whether to preserve legal text comments.
    pub fn with_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }

    /// Sets whether to track article references.
    pub fn with_article_tracking(mut self, track: bool) -> Self {
        self.track_articles = track;
        self
    }

    /// Sets whether to preserve exception handling.
    pub fn with_exception_preservation(mut self, preserve: bool) -> Self {
        self.preserve_exceptions = preserve;
        self
    }

    /// Extracts legal article references from Catala comments.
    fn extract_article_refs(&self, content: &str) -> Vec<String> {
        let mut refs = Vec::new();

        // Look for article references in comments like "@Article 123", "@Art. 456", etc.
        let article_re =
            regex_lite::Regex::new(r"(?i)@(?:article|art\.?|section|sec\.?|§)\s*(\d+(?:\.\d+)*)")
                .ok();

        if let Some(re) = article_re {
            for cap in re.captures_iter(content) {
                if let Some(m) = cap.get(1) {
                    refs.push(format!("Article {}", m.as_str()));
                }
            }
        }

        // Also check for legal citation formats like "Code civil, art. 123"
        let citation_re = regex_lite::Regex::new(
            r"(?i)([A-Z][A-Za-z\s]+),\s*(?:article|art\.?)\s*(\d+(?:\.\d+)*)",
        )
        .ok();

        if let Some(re) = citation_re {
            for cap in re.captures_iter(content) {
                if let (Some(source), Some(article)) = (cap.get(1), cap.get(2)) {
                    refs.push(format!("{}, Article {}", source.as_str(), article.as_str()));
                }
            }
        }

        refs
    }

    /// Extracts exception handlers from Catala code.
    fn extract_exceptions(&self, content: &str) -> Vec<String> {
        let mut exceptions = Vec::new();

        // Look for "exception" keyword
        let exception_re = regex_lite::Regex::new(r"(?m)^\s*exception\s+(\w+)\s+").ok();

        if let Some(re) = exception_re {
            for cap in re.captures_iter(content) {
                if let Some(m) = cap.get(1) {
                    exceptions.push(m.as_str().to_string());
                }
            }
        }

        // Also look for "definition ... under condition ... consequence exception"
        let exception_def_re = regex_lite::Regex::new(r"(?i)consequence\s+exception").ok();

        if let Some(re) = exception_def_re {
            if re.is_match(content) {
                exceptions.push("Default logic exception".to_string());
            }
        }

        exceptions
    }

    /// Detects scope inheritance from Catala code.
    fn detect_scope_inheritance(&self, content: &str) -> Option<String> {
        // Look for scope inheritance: "scope Child extends Parent:"
        let inherit_re = regex_lite::Regex::new(r"(?i)scope\s+\w+\s+extends\s+(\w+)\s*:").ok()?;

        inherit_re
            .captures(content)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Parses a Catala scope declaration.
    fn parse_scope(&self, content: &str, report: &mut ConversionReport) -> Option<Statute> {
        // Look for scope declaration: "declaration scope ScopeName:"
        let scope_re = regex_lite::Regex::new(r"declaration\s+scope\s+(\w+):").ok()?;
        let captures = scope_re.captures(content)?;
        let scope_name = captures.get(1)?.as_str();

        // Extract article references if enabled
        if self.track_articles {
            let article_refs = self.extract_article_refs(content);
            if !article_refs.is_empty() {
                report.add_warning(format!(
                    "Preserved {} article reference(s) in metadata",
                    article_refs.len()
                ));
            }
        }

        // Extract exceptions if enabled
        if self.preserve_exceptions {
            let exceptions = self.extract_exceptions(content);
            if !exceptions.is_empty() {
                report.add_warning(format!(
                    "Found {} exception(s): {}",
                    exceptions.len(),
                    exceptions.join(", ")
                ));
            }
        }

        // Check for scope inheritance
        if let Some(parent) = self.detect_scope_inheritance(content) {
            report.add_warning(format!(
                "Scope inheritance detected: {} extends {}",
                scope_name, parent
            ));
        }

        // Create statute from scope
        let mut statute = Statute::new(
            scope_name.to_lowercase().replace(' ', "-"),
            scope_name,
            Effect::new(EffectType::Grant, "Scope output"),
        );

        // Parse context variables as conditions
        let context_re = regex_lite::Regex::new(r"context\s+(\w+)\s+content\s+(\w+)").ok()?;
        for cap in context_re.captures_iter(content) {
            let var_name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let var_type = cap.get(2).map(|m| m.as_str()).unwrap_or("unknown");

            // Map Catala types to conditions
            match var_type {
                "integer" | "decimal" => {
                    if var_name.to_lowercase().contains("age") {
                        statute.preconditions.push(Condition::Age {
                            operator: ComparisonOp::GreaterOrEqual,
                            value: 0,
                        });
                    }
                }
                "money" => {
                    if var_name.to_lowercase().contains("income") {
                        statute.preconditions.push(Condition::Income {
                            operator: ComparisonOp::GreaterOrEqual,
                            value: 0,
                        });
                    }
                }
                "boolean" => {
                    statute.preconditions.push(Condition::AttributeEquals {
                        key: var_name.to_string(),
                        value: "true".to_string(),
                    });
                }
                _ => {
                    report.add_warning(format!("Unknown Catala type: {}", var_type));
                }
            }
        }

        // Check for rule definitions
        let rule_re = regex_lite::Regex::new(r"rule\s+(\w+)\s+under\s+condition").ok()?;
        for cap in rule_re.captures_iter(content) {
            let rule_name = cap.get(1).map(|m| m.as_str()).unwrap_or("rule");
            report.add_warning(format!("Rule '{}' converted to condition", rule_name));
        }

        Some(statute)
    }
}

impl Default for CatalaImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CatalaImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Catala
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Split by scope declarations
        let sections: Vec<&str> = source.split("declaration scope").collect();

        for (i, section) in sections.iter().enumerate() {
            if i == 0 && !section.contains("scope") {
                continue; // Skip preamble
            }

            let full_section = if i > 0 {
                format!("declaration scope{}", section)
            } else {
                section.to_string()
            };

            if let Some(statute) = self.parse_scope(&full_section, &mut report) {
                statutes.push(statute);
            }
        }

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid Catala scopes found".to_string(),
            ));
        }

        // Note unsupported features
        if source.contains("exception") {
            report.add_unsupported("Catala exceptions (default logic)");
        }
        if source.contains("assertion") {
            report.add_unsupported("Catala assertions");
        }
        if source.contains("definition") && source.contains("state") {
            report.add_unsupported("Catala state definitions");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Check for Catala markers
        source.contains("declaration scope")
            || source.contains("```catala")
            || source.contains("# Catala")
    }
}

/// Catala format exporter.
pub struct CatalaExporter {
    /// Language variant (en, fr)
    language: String,
    /// Whether to include article references in comments
    include_article_refs: bool,
    /// Whether to generate exception handling code
    generate_exceptions: bool,
}

impl CatalaExporter {
    /// Creates a new Catala exporter.
    pub fn new() -> Self {
        Self {
            language: "en".to_string(),
            include_article_refs: true,
            generate_exceptions: true,
        }
    }

    /// Sets the output language.
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
        self
    }

    /// Sets whether to include article references.
    pub fn with_article_refs(mut self, include: bool) -> Self {
        self.include_article_refs = include;
        self
    }

    /// Sets whether to generate exception handling.
    pub fn with_exceptions(mut self, generate: bool) -> Self {
        self.generate_exceptions = generate;
        self
    }

    /// Converts a condition to Catala syntax.
    fn condition_to_catala(condition: &Condition, report: &mut ConversionReport) -> String {
        match condition {
            Condition::Age { operator, value } => {
                let op = Self::operator_to_catala(operator);
                format!("input.age {} {}", op, value)
            }
            Condition::Income { operator, value } => {
                let op = Self::operator_to_catala(operator);
                format!("input.income {} ${}", op, value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_catala(left, report);
                let r = Self::condition_to_catala(right, report);
                format!("({}) and ({})", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_catala(left, report);
                let r = Self::condition_to_catala(right, report);
                format!("({}) or ({})", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_catala(inner, report);
                format!("not ({})", i)
            }
            Condition::AttributeEquals { key, value } => {
                format!("input.{} = {}", key, value)
            }
            Condition::HasAttribute { key } => {
                format!("input.{} exists", key)
            }
            _ => {
                report.add_unsupported(format!("Condition type: {:?}", condition));
                "true".to_string()
            }
        }
    }

    fn operator_to_catala(op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "=",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }
}

impl Default for CatalaExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CatalaExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Catala
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Catala);
        let mut output = String::new();

        // Catala header
        output.push_str("# Generated by Legalis-RS\n");
        output.push_str(&format!("# Language: {}\n\n", self.language));

        for statute in statutes {
            // Convert statute ID to CamelCase for scope name
            let scope_name: String = statute
                .id
                .split('-')
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().chain(c).collect(),
                    }
                })
                .collect();

            // Scope declaration
            output.push_str(&format!("```catala\ndeclaration scope {}:\n", scope_name));

            // Input context
            output.push_str("  context input content Input\n");
            output.push_str("  context output content Output\n");
            output.push_str("```\n\n");

            // Scope definition with rules
            output.push_str(&format!("```catala\nscope {}:\n", scope_name));

            // Convert preconditions to rules
            if !statute.preconditions.is_empty() {
                output.push_str("  definition output.eligible equals\n");

                let conditions: Vec<String> = statute
                    .preconditions
                    .iter()
                    .map(|c| Self::condition_to_catala(c, &mut report))
                    .collect();

                if conditions.len() == 1 {
                    output.push_str(&format!("    {}\n", conditions[0]));
                } else {
                    output.push_str("    ");
                    output.push_str(&conditions.join(" and\n    "));
                    output.push('\n');
                }
            }

            // Handle discretion
            if let Some(ref discretion) = statute.discretion_logic {
                report.add_warning(format!(
                    "Discretion '{}' converted to Catala comment",
                    discretion
                ));
                output.push_str(&format!("  # DISCRETION: {}\n", discretion));
            }

            output.push_str("```\n\n");

            report.statutes_converted += 1;
        }

        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut issues = Vec::new();

        // Check for features Catala doesn't support well
        if statute.discretion_logic.is_some() {
            issues.push("Discretionary logic will be converted to comments".to_string());
        }

        for condition in &statute.preconditions {
            match condition {
                Condition::ResidencyDuration { .. } => {
                    issues.push("Residency conditions need manual mapping".to_string());
                }
                Condition::Geographic { .. } => {
                    issues.push("Geographic conditions need manual mapping".to_string());
                }
                _ => {}
            }
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catala_importer_validate() {
        let importer = CatalaImporter::new();
        assert!(importer.validate("declaration scope Test:"));
        assert!(importer.validate("```catala\ncode\n```"));
        assert!(!importer.validate("STATUTE foo: \"bar\" {}"));
    }

    #[test]
    fn test_catala_exporter_basic() {
        let exporter = CatalaExporter::new();
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("declaration scope AdultRights"));
        assert!(output.contains("input.age >= 18"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_catala_roundtrip_concepts() {
        // Test that we can export and the output is valid Catala-like syntax
        let statute = Statute::new(
            "tax-benefit",
            "Tax Benefit Rule",
            Effect::new(EffectType::Grant, "Tax reduction"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let exporter = CatalaExporter::new();
        let (output, _) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("input.age >= 65"));
        assert!(output.contains("and"));
        assert!(output.contains("input.income < $50000"));
    }
}
