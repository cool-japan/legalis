//! Natural Language to DSL Translation
//!
//! This module provides functionality to translate natural language descriptions
//! of legal rules into the Legalis DSL syntax.

use regex::Regex;

/// Represents a pattern for matching natural language to DSL constructs
#[derive(Debug, Clone)]
pub struct NLPattern {
    /// The regex pattern to match
    pub pattern: Regex,
    /// The DSL template to generate
    pub template: String,
    /// Named groups in the pattern
    pub groups: Vec<String>,
}

/// Natural Language to DSL translator
#[derive(Debug)]
pub struct NLTranslator {
    patterns: Vec<NLPattern>,
}

impl Default for NLTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl NLTranslator {
    /// Creates a new translator with default patterns
    pub fn new() -> Self {
        let mut translator = Self {
            patterns: Vec::new(),
        };
        translator.load_default_patterns();
        translator
    }

    /// Loads default patterns for common legal constructs
    fn load_default_patterns(&mut self) {
        // Simple statute pattern: "if <condition> then <effect>"
        self.add_pattern(
            r"(?i)if\s+(?P<condition>.+?)\s+then\s+(?P<effect>.+)",
            "WHEN {{condition}}\nTHEN {{effect}}",
            vec!["condition".to_string(), "effect".to_string()],
        );

        // Age-based condition: "if age is greater than <num>"
        self.add_pattern(
            r"(?i)if\s+age\s+is\s+(?:greater|more)\s+than\s+(?P<age>\d+)",
            "WHEN AGE > {{age}}",
            vec!["age".to_string()],
        );

        // Age-based condition: "if age is less than <num>"
        self.add_pattern(
            r"(?i)if\s+age\s+is\s+(?:less|fewer)\s+than\s+(?P<age>\d+)",
            "WHEN AGE < {{age}}",
            vec!["age".to_string()],
        );

        // Age-based condition: "if age is at least <num>"
        self.add_pattern(
            r"(?i)if\s+age\s+is\s+at\s+least\s+(?P<age>\d+)",
            "WHEN AGE >= {{age}}",
            vec!["age".to_string()],
        );

        // Age range: "if age is between <min> and <max>"
        self.add_pattern(
            r"(?i)if\s+age\s+is\s+between\s+(?P<min>\d+)\s+and\s+(?P<max>\d+)",
            "WHEN AGE BETWEEN {{min}} AND {{max}}",
            vec!["min".to_string(), "max".to_string()],
        );

        // Income condition: "if income is greater than <amount>"
        self.add_pattern(
            r"(?i)if\s+income\s+is\s+(?:greater|more)\s+than\s+(?P<amount>\d+)",
            "WHEN INCOME > {{amount}}",
            vec!["amount".to_string()],
        );

        // Income condition: "if income is less than <amount>"
        self.add_pattern(
            r"(?i)if\s+income\s+is\s+(?:less|fewer)\s+than\s+(?P<amount>\d+)",
            "WHEN INCOME < {{amount}}",
            vec!["amount".to_string()],
        );

        // Has attribute: "if person has <attribute>"
        self.add_pattern(
            r"(?i)if\s+(?:person|applicant|individual)\s+has\s+(?P<attribute>\w+)",
            "WHEN HAS {{attribute}}",
            vec!["attribute".to_string()],
        );

        // Grant effect: "grant <benefit>"
        self.add_pattern(
            r"(?i)grant\s+(?P<benefit>.+?)(?:\.|$)",
            "THEN GRANT \"{{benefit}}\"",
            vec!["benefit".to_string()],
        );

        // Revoke effect: "revoke <privilege>"
        self.add_pattern(
            r"(?i)revoke\s+(?P<privilege>.+?)(?:\.|$)",
            "THEN REVOKE \"{{privilege}}\"",
            vec!["privilege".to_string()],
        );

        // Obligation effect: "must <action>"
        self.add_pattern(
            r"(?i)must\s+(?P<action>.+?)(?:\.|$)",
            "THEN OBLIGATION \"{{action}}\"",
            vec!["action".to_string()],
        );

        // Prohibition effect: "cannot <action>"
        self.add_pattern(
            r"(?i)(?:cannot|must\s+not|shall\s+not)\s+(?P<action>.+?)(?:\.|$)",
            "THEN PROHIBITION \"{{action}}\"",
            vec!["action".to_string()],
        );

        // Jurisdiction: "in <jurisdiction>"
        self.add_pattern(
            r"(?i)in\s+(?P<jurisdiction>[\w\-]+)",
            "JURISDICTION \"{{jurisdiction}}\"",
            vec!["jurisdiction".to_string()],
        );

        // Effective date: "effective from <date>"
        self.add_pattern(
            r"(?i)effective\s+from\s+(?P<date>\d{4}-\d{2}-\d{2})",
            "EFFECTIVE_DATE {{date}}",
            vec!["date".to_string()],
        );

        // Expiry date: "expires on <date>"
        self.add_pattern(
            r"(?i)expires\s+on\s+(?P<date>\d{4}-\d{2}-\d{2})",
            "EXPIRY_DATE {{date}}",
            vec!["date".to_string()],
        );

        // Version: "version <number>"
        self.add_pattern(
            r"(?i)version\s+(?P<version>\d+(?:\.\d+)?)",
            "VERSION {{version}}",
            vec!["version".to_string()],
        );

        // And condition: "<cond1> and <cond2>"
        self.add_pattern(
            r"(?i)(?P<cond1>.+?)\s+and\s+(?P<cond2>.+)",
            "{{cond1}} AND {{cond2}}",
            vec!["cond1".to_string(), "cond2".to_string()],
        );

        // Or condition: "<cond1> or <cond2>"
        self.add_pattern(
            r"(?i)(?P<cond1>.+?)\s+or\s+(?P<cond2>.+)",
            "{{cond1}} OR {{cond2}}",
            vec!["cond1".to_string(), "cond2".to_string()],
        );
    }

    /// Adds a custom pattern to the translator
    pub fn add_pattern(&mut self, pattern: &str, template: &str, groups: Vec<String>) {
        if let Ok(regex) = Regex::new(pattern) {
            self.patterns.push(NLPattern {
                pattern: regex,
                template: template.to_string(),
                groups,
            });
        }
    }

    /// Translates a natural language description to DSL
    pub fn translate(&self, text: &str) -> TranslationResult {
        let mut dsl_parts = Vec::new();
        let mut confidence = 1.0;
        let mut unmatched_parts = Vec::new();

        let lines: Vec<&str> = text.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let mut matched = false;
            for pattern in &self.patterns {
                if let Some(captures) = pattern.pattern.captures(trimmed) {
                    let mut dsl = pattern.template.clone();

                    // Replace placeholders with captured groups
                    for group_name in &pattern.groups {
                        if let Some(capture) = captures.name(group_name) {
                            let placeholder = format!("{{{{{}}}}}", group_name);
                            dsl = dsl.replace(&placeholder, capture.as_str());
                        }
                    }

                    dsl_parts.push(dsl);
                    matched = true;
                    break;
                }
            }

            if !matched {
                unmatched_parts.push(trimmed.to_string());
                confidence *= 0.8; // Reduce confidence for each unmatched part
            }
        }

        TranslationResult {
            dsl: dsl_parts.join("\n"),
            confidence,
            unmatched_parts,
        }
    }

    /// Translates a full statute description to DSL
    pub fn translate_statute(&self, id: &str, title: &str, body: &str) -> String {
        let translation = self.translate(body);

        format!(
            "STATUTE {}: \"{}\" {{\n{}\n}}",
            id,
            title,
            translation
                .dsl
                .lines()
                .map(|line| format!("    {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Translates a condition description to DSL
    pub fn translate_condition(&self, text: &str) -> Option<String> {
        // Try to match condition-specific patterns
        for pattern in &self.patterns {
            if pattern.template.contains("WHEN") {
                if let Some(captures) = pattern.pattern.captures(text) {
                    let mut dsl = pattern.template.clone();

                    for group_name in &pattern.groups {
                        if let Some(capture) = captures.name(group_name) {
                            let placeholder = format!("{{{{{}}}}}", group_name);
                            dsl = dsl.replace(&placeholder, capture.as_str());
                        }
                    }

                    return Some(dsl);
                }
            }
        }
        None
    }

    /// Translates an effect description to DSL
    pub fn translate_effect(&self, text: &str) -> Option<String> {
        // Try to match effect-specific patterns
        for pattern in &self.patterns {
            if pattern.template.contains("THEN") {
                if let Some(captures) = pattern.pattern.captures(text) {
                    let mut dsl = pattern.template.clone();

                    for group_name in &pattern.groups {
                        if let Some(capture) = captures.name(group_name) {
                            let placeholder = format!("{{{{{}}}}}", group_name);
                            dsl = dsl.replace(&placeholder, capture.as_str());
                        }
                    }

                    return Some(dsl);
                }
            }
        }
        None
    }
}

/// Result of a natural language translation
#[derive(Debug, Clone)]
pub struct TranslationResult {
    /// The generated DSL code
    pub dsl: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Parts of the input that couldn't be matched
    pub unmatched_parts: Vec<String>,
}

/// Builder for creating custom translation contexts
#[derive(Debug)]
pub struct TranslatorBuilder {
    patterns: Vec<(String, String, Vec<String>)>,
}

impl Default for TranslatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslatorBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Adds a custom pattern
    pub fn add_pattern(mut self, pattern: &str, template: &str, groups: Vec<String>) -> Self {
        self.patterns
            .push((pattern.to_string(), template.to_string(), groups));
        self
    }

    /// Builds the translator
    pub fn build(self) -> NLTranslator {
        let mut translator = NLTranslator::new();

        for (pattern, template, groups) in self.patterns {
            translator.add_pattern(&pattern, &template, groups);
        }

        translator
    }
}

/// Common templates for frequently used legal constructs
pub struct CommonTemplates;

impl CommonTemplates {
    /// Age eligibility template
    pub fn age_eligibility(min_age: u32) -> String {
        format!("WHEN AGE >= {}", min_age)
    }

    /// Income threshold template
    pub fn income_threshold(max_income: u32) -> String {
        format!("WHEN INCOME < {}", max_income)
    }

    /// Citizenship requirement template
    pub fn citizenship_requirement() -> String {
        "WHEN HAS citizen".to_string()
    }

    /// Grant benefit template
    pub fn grant_benefit(benefit: &str) -> String {
        format!("THEN GRANT \"{}\"", benefit)
    }

    /// Revoke privilege template
    pub fn revoke_privilege(privilege: &str) -> String {
        format!("THEN REVOKE \"{}\"", privilege)
    }

    /// Obligation template
    pub fn obligation(action: &str) -> String {
        format!("THEN OBLIGATION \"{}\"", action)
    }

    /// Prohibition template
    pub fn prohibition(action: &str) -> String {
        format!("THEN PROHIBITION \"{}\"", action)
    }

    /// Effective date template
    pub fn effective_date(date: &str) -> String {
        format!("EFFECTIVE_DATE {}", date)
    }

    /// Jurisdiction template
    pub fn jurisdiction(jurisdiction: &str) -> String {
        format!("JURISDICTION \"{}\"", jurisdiction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_age_condition() {
        let translator = NLTranslator::new();
        let result = translator.translate("if age is greater than 18");

        assert!(result.dsl.contains("WHEN AGE > 18"));
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_translate_age_range() {
        let translator = NLTranslator::new();
        let result = translator.translate("if age is between 18 and 65");

        assert!(result.dsl.contains("WHEN AGE BETWEEN 18 AND 65"));
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_translate_has_attribute() {
        let translator = NLTranslator::new();
        let result = translator.translate("if person has citizen");

        assert!(result.dsl.contains("WHEN HAS citizen"));
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_translate_grant_effect() {
        let translator = NLTranslator::new();
        let result = translator.translate("grant voting rights");

        assert!(result.dsl.contains("THEN GRANT \"voting rights\""));
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_translate_full_statute() {
        let translator = NLTranslator::new();
        let dsl = translator.translate_statute(
            "voting-rights",
            "Voting Rights",
            "if age is at least 18\ngrant voting rights",
        );

        assert!(dsl.contains("STATUTE voting-rights"));
        assert!(dsl.contains("\"Voting Rights\""));
        assert!(dsl.contains("WHEN AGE >= 18"));
        assert!(dsl.contains("THEN GRANT \"voting rights\""));
    }

    #[test]
    fn test_translate_condition_only() {
        let translator = NLTranslator::new();
        let result = translator.translate_condition("if age is greater than 21");

        assert!(result.is_some());
        assert!(result.unwrap().contains("WHEN AGE > 21"));
    }

    #[test]
    fn test_translate_effect_only() {
        let translator = NLTranslator::new();
        let result = translator.translate_effect("must pay taxes");

        assert!(result.is_some());
        assert!(result.unwrap().contains("THEN OBLIGATION \"pay taxes\""));
    }

    #[test]
    fn test_common_templates() {
        assert_eq!(CommonTemplates::age_eligibility(18), "WHEN AGE >= 18");
        assert_eq!(
            CommonTemplates::income_threshold(50000),
            "WHEN INCOME < 50000"
        );
        assert_eq!(
            CommonTemplates::citizenship_requirement(),
            "WHEN HAS citizen"
        );
        assert_eq!(
            CommonTemplates::grant_benefit("healthcare"),
            "THEN GRANT \"healthcare\""
        );
    }

    #[test]
    fn test_custom_pattern() {
        let mut translator = NLTranslator::new();
        translator.add_pattern(
            r"(?i)if\s+score\s+is\s+(?P<score>\d+)",
            "WHEN SCORE = {{score}}",
            vec!["score".to_string()],
        );

        let result = translator.translate("if score is 100");
        assert!(result.dsl.contains("WHEN SCORE = 100"));
    }

    #[test]
    fn test_multiple_conditions() {
        let translator = NLTranslator::new();
        let result = translator.translate("if age is greater than 18\nif person has citizen");

        assert!(result.dsl.contains("WHEN AGE > 18"));
        assert!(result.dsl.contains("WHEN HAS citizen"));
    }

    #[test]
    fn test_unmatched_parts() {
        let translator = NLTranslator::new();
        let result = translator.translate("this is some random text that won't match");

        assert!(result.confidence < 1.0);
        assert!(!result.unmatched_parts.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let translator = TranslatorBuilder::new()
            .add_pattern(
                r"(?i)when\s+(?P<field>\w+)\s+equals\s+(?P<value>\w+)",
                "WHEN {{field}} = {{value}}",
                vec!["field".to_string(), "value".to_string()],
            )
            .build();

        let result = translator.translate("when status equals active");
        assert!(result.dsl.contains("WHEN status = active"));
    }
}
