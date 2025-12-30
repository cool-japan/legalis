//! Prompt template system for structured prompt generation.
//!
//! This module provides a flexible template system for creating and managing
//! prompts with variable substitution, versioning, and domain-specific templates.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A prompt template with variable substitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Template name
    pub name: String,
    /// Template content with placeholders in {{variable}} format
    pub template: String,
    /// Optional description
    pub description: Option<String>,
    /// Template version
    pub version: u32,
    /// Default values for variables
    pub defaults: HashMap<String, String>,
}

impl PromptTemplate {
    /// Creates a new prompt template.
    pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            template: template.into(),
            description: None,
            version: 1,
            defaults: HashMap::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the version.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Adds a default value for a variable.
    pub fn with_default(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.defaults.insert(key.into(), value.into());
        self
    }

    /// Renders the template with the given variables.
    ///
    /// Variables should be provided in {{variable}} format in the template.
    /// Missing variables will use defaults if available, otherwise return an error.
    pub fn render(&self, variables: &HashMap<String, String>) -> Result<String> {
        let mut result = self.template.clone();

        // Find all variables in the template
        let vars = self.extract_variables();

        for var in vars {
            let value = variables
                .get(&var)
                .or_else(|| self.defaults.get(&var))
                .ok_or_else(|| anyhow!("Missing variable: {}", var))?;

            let placeholder = format!("{{{{{}}}}}", var);
            result = result.replace(&placeholder, value);
        }

        // Check for any remaining unresolved variables
        if result.contains("{{") {
            let remaining = self.extract_variables_from(&result);
            if !remaining.is_empty() {
                return Err(anyhow!("Unresolved variables: {:?}", remaining));
            }
        }

        Ok(result)
    }

    /// Renders the template with builder-style variable assignment.
    pub fn render_with(
        &self,
        builder: impl FnOnce(TemplateBuilder) -> TemplateBuilder,
    ) -> Result<String> {
        let template_builder = TemplateBuilder::new();
        let template_builder = builder(template_builder);
        self.render(&template_builder.variables)
    }

    /// Extracts all variable names from the template.
    pub fn extract_variables(&self) -> Vec<String> {
        self.extract_variables_from(&self.template)
    }

    fn extract_variables_from(&self, text: &str) -> Vec<String> {
        let mut vars = Vec::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '{' && chars.peek() == Some(&'{') {
                chars.next(); // consume second '{'

                let mut var_name = String::new();
                while let Some(c) = chars.next() {
                    if c == '}' && chars.peek() == Some(&'}') {
                        chars.next(); // consume second '}'
                        vars.push(var_name.trim().to_string());
                        break;
                    }
                    var_name.push(c);
                }
            }
        }

        vars
    }

    /// Lists all required variables (those without defaults).
    pub fn required_variables(&self) -> Vec<String> {
        self.extract_variables()
            .into_iter()
            .filter(|var| !self.defaults.contains_key(var))
            .collect()
    }

    /// Validates that all required variables are provided.
    pub fn validate_variables(&self, variables: &HashMap<String, String>) -> Result<()> {
        let required = self.required_variables();
        let missing: Vec<_> = required
            .iter()
            .filter(|var| !variables.contains_key(*var))
            .collect();

        if !missing.is_empty() {
            return Err(anyhow!("Missing required variables: {:?}", missing));
        }

        Ok(())
    }
}

/// Builder for template variables.
pub struct TemplateBuilder {
    variables: HashMap<String, String>,
}

impl TemplateBuilder {
    /// Creates a new template builder.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Sets a variable value.
    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A registry for managing multiple prompt templates.
pub struct TemplateRegistry {
    templates: HashMap<String, Vec<PromptTemplate>>,
}

impl TemplateRegistry {
    /// Creates a new template registry.
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Registers a template.
    pub fn register(&mut self, template: PromptTemplate) {
        self.templates
            .entry(template.name.clone())
            .or_default()
            .push(template);
    }

    /// Gets the latest version of a template by name.
    pub fn get(&self, name: &str) -> Option<&PromptTemplate> {
        self.templates
            .get(name)
            .and_then(|versions| versions.iter().max_by_key(|t| t.version))
    }

    /// Gets a specific version of a template.
    pub fn get_version(&self, name: &str, version: u32) -> Option<&PromptTemplate> {
        self.templates
            .get(name)?
            .iter()
            .find(|t| t.version == version)
    }

    /// Lists all template names.
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Gets all versions of a template.
    pub fn get_all_versions(&self, name: &str) -> Option<&Vec<PromptTemplate>> {
        self.templates.get(name)
    }

    /// Creates a registry with common legal templates.
    pub fn with_legal_templates() -> Self {
        let mut registry = Self::new();

        // Statute compilation template
        registry.register(
            PromptTemplate::new(
                "compile_statute",
                r#"You are a 'Legal Compiler'. Convert the following natural language statute text into a structured JSON format.

Mark any interpretive or discretionary parts as 'JudicialDiscretion'.

Statute text:
{{statute_text}}

Respond with valid JSON matching this structure:
{
    "id": "statute-id",
    "title": "Statute Title",
    "preconditions": [],
    "effect": {
        "effect_type": "Grant|Revoke|Obligation|Prohibition|MonetaryTransfer|StatusChange|Custom",
        "description": "Effect description",
        "parameters": {}
    },
    "discretion_logic": null or "description of discretionary element"
}"#,
            )
            .with_description("Compiles natural language statute into structured format"),
        );

        // Statute analysis template
        registry.register(
            PromptTemplate::new(
                "analyze_statute",
                r#"Analyze the following statute for:
1. Logical consistency
2. Ambiguous language that might require judicial interpretation
3. Potential conflicts with common legal principles
4. Missing conditions or edge cases

Statute:
{{statute_json}}

Respond with JSON:
{
    "issues": ["list of identified issues"],
    "ambiguities": ["list of ambiguous terms or phrases"],
    "recommendations": ["list of recommendations"],
    "discretion_points": ["areas requiring human judgment"]
}"#,
            )
            .with_description("Analyzes a statute for potential issues"),
        );

        // Statute explanation template
        registry.register(
            PromptTemplate::new(
                "explain_statute",
                r#"Explain the following statute in plain language that a non-lawyer can understand.
Include:
1. Who this law applies to
2. What conditions must be met
3. What happens when conditions are met
4. Any areas where human judgment is required

Statute:
{{statute_json}}

Provide a clear, concise explanation."#,
            )
            .with_description("Generates plain language explanation of a statute"),
        );

        // Contract analysis template
        registry.register(
            PromptTemplate::new(
                "analyze_contract",
                r#"Analyze the following contract and identify:
1. Key obligations for each party
2. Potential risks or unfavorable terms
3. Ambiguous clauses that may lead to disputes
4. Missing standard provisions

Contract:
{{contract_text}}

Provide analysis in JSON format:
{
    "obligations": {"party1": [...], "party2": [...]},
    "risks": [...],
    "ambiguities": [...],
    "missing_provisions": [...]
}"#,
            )
            .with_description("Analyzes a contract for key terms and risks"),
        );

        registry
    }

    /// Creates a registry with common coding templates.
    pub fn with_coding_templates() -> Self {
        let mut registry = Self::new();

        registry.register(
            PromptTemplate::new(
                "code_review",
                r#"Review the following code and provide feedback on:
1. Code quality and style
2. Potential bugs or issues
3. Performance concerns
4. Security vulnerabilities
5. Suggestions for improvement

Language: {{language}}

Code:
{{code}}

Provide detailed review."#,
            )
            .with_description("Reviews code for quality, bugs, and improvements")
            .with_default("language", "Rust"),
        );

        registry.register(
            PromptTemplate::new(
                "generate_tests",
                r#"Generate comprehensive unit tests for the following code.
Include edge cases, error conditions, and normal operation.

Language: {{language}}
Testing Framework: {{framework}}

Code:
{{code}}

Generate test code."#,
            )
            .with_description("Generates unit tests for given code")
            .with_default("language", "Rust")
            .with_default("framework", "built-in"),
        );

        registry
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A/B testing for prompts.
pub mod ab_testing {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Variant in an A/B test.
    #[derive(Debug, Clone)]
    pub struct PromptVariant {
        pub name: String,
        pub template: PromptTemplate,
        pub weight: f64,
    }

    /// Statistics for a variant.
    #[derive(Debug, Clone, Default)]
    pub struct VariantStats {
        pub requests: usize,
        pub successes: usize,
        pub failures: usize,
        pub total_latency_ms: u128,
    }

    impl VariantStats {
        pub fn success_rate(&self) -> f64 {
            if self.requests == 0 {
                0.0
            } else {
                (self.successes as f64 / self.requests as f64) * 100.0
            }
        }

        pub fn avg_latency_ms(&self) -> f64 {
            if self.requests == 0 {
                0.0
            } else {
                self.total_latency_ms as f64 / self.requests as f64
            }
        }
    }

    /// A/B test configuration.
    pub struct ABTest {
        name: String,
        variants: Vec<PromptVariant>,
        stats: Arc<std::sync::Mutex<HashMap<String, VariantStats>>>,
        selection_counter: Arc<AtomicUsize>,
    }

    impl ABTest {
        /// Creates a new A/B test.
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                variants: Vec::new(),
                stats: Arc::new(std::sync::Mutex::new(HashMap::new())),
                selection_counter: Arc::new(AtomicUsize::new(0)),
            }
        }

        /// Adds a variant to the test.
        pub fn add_variant(
            mut self,
            name: impl Into<String>,
            template: PromptTemplate,
            weight: f64,
        ) -> Self {
            let variant_name = name.into();
            self.variants.push(PromptVariant {
                name: variant_name.clone(),
                template,
                weight,
            });
            self.stats
                .lock()
                .unwrap()
                .insert(variant_name, VariantStats::default());
            self
        }

        /// Selects a variant based on weights (weighted random selection).
        pub fn select_variant(&self) -> Option<&PromptVariant> {
            if self.variants.is_empty() {
                return None;
            }

            // Simple round-robin for now (can be enhanced with true weighted random)
            let counter = self.selection_counter.fetch_add(1, Ordering::SeqCst);
            let total_weight: f64 = self.variants.iter().map(|v| v.weight).sum();

            if total_weight == 0.0 {
                // Equal distribution if no weights
                Some(&self.variants[counter % self.variants.len()])
            } else {
                // Weighted selection
                let mut cumulative = 0.0;
                let target = (counter as f64 % total_weight) / total_weight * total_weight;

                for variant in &self.variants {
                    cumulative += variant.weight;
                    if target < cumulative {
                        return Some(variant);
                    }
                }

                self.variants.last()
            }
        }

        /// Records a successful request for a variant.
        pub fn record_success(&self, variant_name: &str, latency_ms: u128) {
            let mut stats = self.stats.lock().unwrap();
            if let Some(variant_stats) = stats.get_mut(variant_name) {
                variant_stats.requests += 1;
                variant_stats.successes += 1;
                variant_stats.total_latency_ms += latency_ms;
            }
        }

        /// Records a failed request for a variant.
        pub fn record_failure(&self, variant_name: &str, latency_ms: u128) {
            let mut stats = self.stats.lock().unwrap();
            if let Some(variant_stats) = stats.get_mut(variant_name) {
                variant_stats.requests += 1;
                variant_stats.failures += 1;
                variant_stats.total_latency_ms += latency_ms;
            }
        }

        /// Gets statistics for all variants.
        pub fn get_stats(&self) -> HashMap<String, VariantStats> {
            self.stats.lock().unwrap().clone()
        }

        /// Gets the winning variant based on success rate.
        pub fn get_winner(&self) -> Option<(String, VariantStats)> {
            let stats = self.stats.lock().unwrap();
            stats
                .iter()
                .max_by(|a, b| {
                    a.1.success_rate()
                        .partial_cmp(&b.1.success_rate())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(name, stats)| (name.clone(), stats.clone()))
        }

        /// Generates a report of the A/B test.
        pub fn generate_report(&self) -> String {
            let mut report = format!("A/B Test Report: {}\n", self.name);
            report.push_str("=".repeat(50).as_str());
            report.push('\n');

            let stats = self.stats.lock().unwrap();

            for (variant_name, variant_stats) in stats.iter() {
                report.push_str(&format!("\nVariant: {}\n", variant_name));
                report.push_str(&format!("  Requests: {}\n", variant_stats.requests));
                report.push_str(&format!("  Successes: {}\n", variant_stats.successes));
                report.push_str(&format!("  Failures: {}\n", variant_stats.failures));
                report.push_str(&format!(
                    "  Success Rate: {:.2}%\n",
                    variant_stats.success_rate()
                ));
                report.push_str(&format!(
                    "  Avg Latency: {:.2}ms\n",
                    variant_stats.avg_latency_ms()
                ));
            }

            if let Some((winner_name, winner_stats)) = self.get_winner() {
                report.push_str(&format!(
                    "\nWinner: {} ({:.2}% success rate)\n",
                    winner_name,
                    winner_stats.success_rate()
                ));
            }

            report
        }
    }
}

/// Prompt optimization suggestions and analysis.
pub mod optimization {
    use super::*;

    /// Types of optimization suggestions.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SuggestionType {
        /// Prompt is too long and should be shortened
        TooLong,
        /// Prompt is too short and lacks detail
        TooShort,
        /// Prompt contains redundant information
        Redundant,
        /// Prompt could be more specific
        VagueInstructions,
        /// Prompt should include examples
        MissingExamples,
        /// Prompt should specify output format
        MissingFormat,
        /// Prompt contains ambiguous language
        AmbiguousLanguage,
        /// Prompt could benefit from chain-of-thought prompting
        UseChainOfThought,
        /// Prompt should be split into multiple steps
        SplitIntoSteps,
        /// Prompt could use a template
        UseTemplate,
    }

    /// An optimization suggestion for a prompt.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OptimizationSuggestion {
        /// Type of suggestion
        pub suggestion_type: SuggestionType,
        /// Severity level (0-100, higher is more important)
        pub severity: u8,
        /// Description of the issue
        pub description: String,
        /// Suggested improvement
        pub improvement: Option<String>,
        /// Example of improved prompt
        pub example: Option<String>,
    }

    impl OptimizationSuggestion {
        /// Creates a new optimization suggestion.
        pub fn new(
            suggestion_type: SuggestionType,
            severity: u8,
            description: impl Into<String>,
        ) -> Self {
            Self {
                suggestion_type,
                severity: severity.min(100),
                description: description.into(),
                improvement: None,
                example: None,
            }
        }

        /// Adds an improvement suggestion.
        pub fn with_improvement(mut self, improvement: impl Into<String>) -> Self {
            self.improvement = Some(improvement.into());
            self
        }

        /// Adds an example.
        pub fn with_example(mut self, example: impl Into<String>) -> Self {
            self.example = Some(example.into());
            self
        }
    }

    /// Result of prompt analysis.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PromptAnalysis {
        /// The analyzed prompt
        pub prompt: String,
        /// Character count
        pub char_count: usize,
        /// Word count
        pub word_count: usize,
        /// Estimated token count (rough approximation)
        pub estimated_tokens: usize,
        /// Optimization suggestions
        pub suggestions: Vec<OptimizationSuggestion>,
        /// Overall quality score (0-100)
        pub quality_score: u8,
    }

    impl PromptAnalysis {
        /// Returns high-severity suggestions (severity >= 70).
        pub fn high_severity_suggestions(&self) -> Vec<&OptimizationSuggestion> {
            self.suggestions
                .iter()
                .filter(|s| s.severity >= 70)
                .collect()
        }

        /// Returns medium-severity suggestions (40 <= severity < 70).
        pub fn medium_severity_suggestions(&self) -> Vec<&OptimizationSuggestion> {
            self.suggestions
                .iter()
                .filter(|s| s.severity >= 40 && s.severity < 70)
                .collect()
        }

        /// Returns low-severity suggestions (severity < 40).
        pub fn low_severity_suggestions(&self) -> Vec<&OptimizationSuggestion> {
            self.suggestions
                .iter()
                .filter(|s| s.severity < 40)
                .collect()
        }
    }

    /// Analyzer for prompt optimization.
    pub struct PromptOptimizer {
        /// Minimum recommended prompt length
        min_length: usize,
        /// Maximum recommended prompt length
        max_length: usize,
    }

    impl PromptOptimizer {
        /// Creates a new prompt optimizer with default settings.
        pub fn new() -> Self {
            Self {
                min_length: 50,
                max_length: 4000,
            }
        }

        /// Sets the minimum recommended length.
        pub fn with_min_length(mut self, min_length: usize) -> Self {
            self.min_length = min_length;
            self
        }

        /// Sets the maximum recommended length.
        pub fn with_max_length(mut self, max_length: usize) -> Self {
            self.max_length = max_length;
            self
        }

        /// Analyzes a prompt and returns optimization suggestions.
        pub fn analyze(&self, prompt: &str) -> PromptAnalysis {
            let char_count = prompt.len();
            let word_count = prompt.split_whitespace().count();
            let estimated_tokens = (word_count as f64 * 1.3) as usize; // Rough estimate

            let mut suggestions = Vec::new();

            // Check length
            if char_count < self.min_length {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::TooShort,
                        80,
                        format!(
                            "Prompt is quite short ({} characters). Consider adding more detail.",
                            char_count
                        ),
                    )
                    .with_improvement(
                        "Add specific instructions, context, and desired output format.",
                    ),
                );
            } else if char_count > self.max_length {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::TooLong,
                        70,
                        format!(
                            "Prompt is quite long ({} characters). Consider breaking it down.",
                            char_count
                        ),
                    )
                    .with_improvement("Split complex prompts into multiple smaller steps."),
                );
            }

            // Check for vague language
            let vague_words = [
                "maybe",
                "perhaps",
                "somehow",
                "something",
                "stuff",
                "things",
            ];
            let vague_count = vague_words
                .iter()
                .filter(|word| prompt.to_lowercase().contains(*word))
                .count();

            if vague_count > 0 {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::VagueInstructions,
                        60,
                        format!(
                            "Prompt contains {} vague word(s). Be more specific.",
                            vague_count
                        ),
                    )
                    .with_improvement("Replace vague language with concrete instructions."),
                );
            }

            // Check for output format specification
            if !prompt.to_lowercase().contains("format")
                && !prompt.to_lowercase().contains("json")
                && !prompt.to_lowercase().contains("xml")
                && !prompt.to_lowercase().contains("markdown")
            {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::MissingFormat,
                        50,
                        "Prompt doesn't specify an output format.",
                    )
                    .with_improvement("Add instructions like 'Respond in JSON format' or 'Use markdown formatting'."),
                );
            }

            // Check for examples
            if !prompt.to_lowercase().contains("example")
                && !prompt.to_lowercase().contains("for instance")
                && word_count > 20
            {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::MissingExamples,
                        40,
                        "Consider adding examples to clarify expectations.",
                    )
                    .with_improvement("Include one or two examples of desired output."),
                );
            }

            // Check for question marks (possible ambiguity)
            let question_count = prompt.matches('?').count();
            if question_count > 3 {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::AmbiguousLanguage,
                        55,
                        format!(
                            "Prompt contains {} questions. Use declarative instructions instead.",
                            question_count
                        ),
                    )
                    .with_improvement("Replace questions with direct instructions."),
                );
            }

            // Suggest chain-of-thought for complex tasks
            if word_count > 100 && !prompt.to_lowercase().contains("step by step") {
                suggestions.push(
                    OptimizationSuggestion::new(
                        SuggestionType::UseChainOfThought,
                        45,
                        "For complex tasks, consider requesting step-by-step reasoning.",
                    )
                    .with_improvement(
                        "Add: 'Think through this step by step' or 'Show your reasoning'.",
                    ),
                );
            }

            // Calculate quality score
            let base_score = 100;
            let penalty = suggestions
                .iter()
                .map(|s| s.severity as i32 / 10)
                .sum::<i32>();
            let quality_score = (base_score - penalty).max(0) as u8;

            PromptAnalysis {
                prompt: prompt.to_string(),
                char_count,
                word_count,
                estimated_tokens,
                suggestions,
                quality_score,
            }
        }

        /// Suggests template-based alternatives for common patterns.
        pub fn suggest_template(&self, prompt: &str) -> Option<String> {
            let lower = prompt.to_lowercase();

            if lower.contains("compile") && lower.contains("law") {
                return Some("compile_statute".to_string());
            }

            if lower.contains("analyze") && (lower.contains("statute") || lower.contains("law")) {
                return Some("analyze_statute".to_string());
            }

            if lower.contains("explain") && (lower.contains("statute") || lower.contains("law")) {
                return Some("explain_statute".to_string());
            }

            if lower.contains("review") && lower.contains("code") {
                return Some("code_review".to_string());
            }

            if lower.contains("generate") && lower.contains("test") {
                return Some("generate_tests".to_string());
            }

            None
        }
    }

    impl Default for PromptOptimizer {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_render() {
        let template = PromptTemplate::new("test", "Hello, {{name}}! You are {{age}} years old.");

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("age".to_string(), "30".to_string());

        let result = template.render(&vars).unwrap();
        assert_eq!(result, "Hello, Alice! You are 30 years old.");
    }

    #[test]
    fn test_template_with_defaults() {
        let template = PromptTemplate::new("test", "Hello, {{name}}! Language: {{language}}")
            .with_default("language", "English");

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Bob".to_string());

        let result = template.render(&vars).unwrap();
        assert_eq!(result, "Hello, Bob! Language: English");
    }

    #[test]
    fn test_template_missing_variable() {
        let template = PromptTemplate::new("test", "Hello, {{name}}!");

        let vars = HashMap::new();
        let result = template.render(&vars);
        assert!(result.is_err());
    }

    #[test]
    fn test_template_extract_variables() {
        let template = PromptTemplate::new("test", "{{var1}} and {{var2}} but not { var3 }");

        let vars = template.extract_variables();
        assert_eq!(vars, vec!["var1", "var2"]);
    }

    #[test]
    fn test_template_required_variables() {
        let template = PromptTemplate::new("test", "{{required}} and {{optional}}")
            .with_default("optional", "default");

        let required = template.required_variables();
        assert_eq!(required, vec!["required"]);
    }

    #[test]
    fn test_template_builder() {
        let template = PromptTemplate::new("test", "{{a}} + {{b}} = {{c}}");

        let result = template
            .render_with(|b| b.set("a", "1").set("b", "2").set("c", "3"))
            .unwrap();

        assert_eq!(result, "1 + 2 = 3");
    }

    #[test]
    fn test_registry_basic() {
        let mut registry = TemplateRegistry::new();

        let template = PromptTemplate::new("greeting", "Hello, {{name}}!");
        registry.register(template);

        let retrieved = registry.get("greeting");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "greeting");
    }

    #[test]
    fn test_registry_versioning() {
        let mut registry = TemplateRegistry::new();

        registry.register(PromptTemplate::new("test", "Version 1").with_version(1));
        registry.register(PromptTemplate::new("test", "Version 2").with_version(2));

        let latest = registry.get("test").unwrap();
        assert_eq!(latest.version, 2);

        let v1 = registry.get_version("test", 1).unwrap();
        assert_eq!(v1.template, "Version 1");
    }

    #[test]
    fn test_legal_templates() {
        let registry = TemplateRegistry::with_legal_templates();

        assert!(registry.get("compile_statute").is_some());
        assert!(registry.get("analyze_statute").is_some());
        assert!(registry.get("explain_statute").is_some());
        assert!(registry.get("analyze_contract").is_some());
    }

    #[test]
    fn test_coding_templates() {
        let registry = TemplateRegistry::with_coding_templates();

        assert!(registry.get("code_review").is_some());
        assert!(registry.get("generate_tests").is_some());

        // Test with defaults
        let code_review = registry.get("code_review").unwrap();
        let mut vars = HashMap::new();
        vars.insert("code".to_string(), "fn main() {}".to_string());

        let result = code_review.render(&vars).unwrap();
        assert!(result.contains("Rust")); // Default language
    }
}

/// Prompt optimization utilities for improving prompt quality.
pub struct PromptOptimizer {
    /// Target model (for model-specific optimizations)
    target_model: Option<String>,
}

impl PromptOptimizer {
    /// Creates a new prompt optimizer.
    pub fn new() -> Self {
        Self { target_model: None }
    }

    /// Sets the target model for optimizations.
    pub fn with_target_model(mut self, model: impl Into<String>) -> Self {
        self.target_model = Some(model.into());
        self
    }

    /// Optimizes a prompt by applying best practices.
    pub fn optimize(&self, prompt: &str) -> String {
        let mut optimized = prompt.to_string();

        // Apply optimization rules
        optimized = self.add_clarity_markers(&optimized);
        optimized = self.structure_instructions(&optimized);
        optimized = self.add_output_format_guidance(&optimized);

        optimized
    }

    /// Adds clarity markers to make instructions more explicit.
    fn add_clarity_markers(&self, prompt: &str) -> String {
        // If prompt doesn't start with clear instruction, add one
        if !prompt.trim_start().starts_with("You are")
            && !prompt.trim_start().starts_with("Please")
            && !prompt.trim_start().starts_with("Task:")
        {
            format!("Task: {}", prompt)
        } else {
            prompt.to_string()
        }
    }

    /// Structures instructions for better comprehension.
    fn structure_instructions(&self, prompt: &str) -> String {
        // If prompt is very long, suggest breaking into sections
        if prompt.len() > 500 && !prompt.contains("##") && !prompt.contains("---") {
            // Add section markers if not present
            prompt.to_string()
        } else {
            prompt.to_string()
        }
    }

    /// Adds output format guidance if missing.
    fn add_output_format_guidance(&self, prompt: &str) -> String {
        // Check if prompt specifies output format
        let has_format_instruction = prompt.to_lowercase().contains("respond with")
            || prompt.to_lowercase().contains("output format")
            || prompt.to_lowercase().contains("format:")
            || prompt.contains("JSON")
            || prompt.contains("```");

        if !has_format_instruction && prompt.len() > 100 {
            format!("{}\n\nPlease provide a clear, structured response.", prompt)
        } else {
            prompt.to_string()
        }
    }

    /// Analyzes a prompt and provides optimization suggestions.
    pub fn analyze(&self, prompt: &str) -> PromptAnalysis {
        let mut suggestions = Vec::new();
        let mut score: f64 = 100.0;

        // Check length
        if prompt.len() < 20 {
            suggestions.push("Prompt is very short. Consider adding more context.".to_string());
            score -= 20.0;
        } else if prompt.len() > 2000 {
            suggestions
                .push("Prompt is very long. Consider breaking it into smaller parts.".to_string());
            score -= 10.0;
        }

        // Check for clarity
        if !prompt.contains("?") && !prompt.to_lowercase().contains("please") {
            suggestions.push("Consider phrasing as a clear question or request.".to_string());
            score -= 15.0;
        }

        // Check for examples
        if prompt.len() > 200 && !prompt.to_lowercase().contains("example") {
            suggestions.push("Consider adding examples for better results.".to_string());
            score -= 10.0;
        }

        // Check for output format specification
        if !prompt.to_lowercase().contains("format") && !prompt.contains("JSON") {
            suggestions.push("Consider specifying the desired output format.".to_string());
            score -= 10.0;
        }

        // Check for role specification (best practice)
        if !prompt.to_lowercase().contains("you are") && prompt.len() > 100 {
            suggestions
                .push("Consider specifying a role (e.g., 'You are an expert...').".to_string());
            score -= 10.0;
        }

        PromptAnalysis {
            quality_score: score.max(0.0),
            suggestions,
            estimated_tokens: self.estimate_tokens(prompt),
            complexity: self.estimate_complexity(prompt),
        }
    }

    /// Estimates the number of tokens in a prompt.
    fn estimate_tokens(&self, prompt: &str) -> usize {
        // Rough estimation: ~4 characters per token
        (prompt.len() as f64 / 4.0).ceil() as usize
    }

    /// Estimates prompt complexity.
    fn estimate_complexity(&self, prompt: &str) -> PromptComplexity {
        let word_count = prompt.split_whitespace().count();
        let has_code =
            prompt.contains("```") || prompt.contains("fn ") || prompt.contains("class ");
        let has_structure = prompt.contains("##") || prompt.contains("1.") || prompt.contains("-");

        if word_count > 300 || has_code {
            PromptComplexity::High
        } else if word_count > 100 || has_structure {
            PromptComplexity::Medium
        } else {
            PromptComplexity::Low
        }
    }

    /// Compresses a prompt while preserving key information.
    pub fn compress(&self, prompt: &str, target_ratio: f64) -> String {
        let target_len = (prompt.len() as f64 * target_ratio) as usize;

        if prompt.len() <= target_len {
            return prompt.to_string();
        }

        // Simple compression: remove extra whitespace and redundant words
        let compressed = prompt.split_whitespace().collect::<Vec<_>>().join(" ");

        if compressed.len() <= target_len {
            compressed
        } else {
            // Truncate to target length, preserving sentence boundaries
            let truncated = &compressed[..target_len.min(compressed.len())];
            if let Some(last_period) = truncated.rfind('.') {
                truncated[..=last_period].to_string()
            } else {
                format!("{}...", truncated)
            }
        }
    }
}

impl Default for PromptOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis result for a prompt.
#[derive(Debug, Clone)]
pub struct PromptAnalysis {
    /// Quality score (0-100)
    pub quality_score: f64,
    /// Optimization suggestions
    pub suggestions: Vec<String>,
    /// Estimated token count
    pub estimated_tokens: usize,
    /// Prompt complexity
    pub complexity: PromptComplexity,
}

/// Prompt complexity levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptComplexity {
    /// Simple, short prompts
    Low,
    /// Moderate complexity
    Medium,
    /// Complex, detailed prompts
    High,
}

#[cfg(test)]
mod optimizer_tests {
    use super::*;

    #[test]
    fn test_prompt_optimizer_basic() {
        let optimizer = PromptOptimizer::new();
        let prompt = "What is Rust?";
        let optimized = optimizer.optimize(prompt);

        assert!(optimized.contains("Task:") || optimized.contains("What is Rust?"));
    }

    #[test]
    fn test_prompt_analysis_short() {
        let optimizer = PromptOptimizer::new();
        let analysis = optimizer.analyze("Hi");

        assert!(analysis.quality_score < 100.0);
        assert!(!analysis.suggestions.is_empty());
    }

    #[test]
    fn test_prompt_analysis_good() {
        let optimizer = PromptOptimizer::new();
        let prompt = "You are an expert programmer. Please explain how Rust's ownership system works. Provide examples in your response. Format: Use code blocks for examples.";
        let analysis = optimizer.analyze(prompt);

        assert!(analysis.quality_score > 70.0);
    }

    #[test]
    fn test_token_estimation() {
        let optimizer = PromptOptimizer::new();
        let analysis = optimizer.analyze("This is a test prompt.");

        assert!(analysis.estimated_tokens > 0);
        assert!(analysis.estimated_tokens < 100);
    }

    #[test]
    fn test_complexity_estimation() {
        let optimizer = PromptOptimizer::new();

        let simple = optimizer.analyze("Hi");
        assert_eq!(simple.complexity, PromptComplexity::Low);

        let complex = optimizer.analyze(&"word ".repeat(350));
        assert_eq!(complex.complexity, PromptComplexity::High);
    }

    #[test]
    fn test_prompt_compression() {
        let optimizer = PromptOptimizer::new();
        let long_prompt = "This is a very long prompt that contains a lot of redundant information and should be compressed to make it more efficient.";
        let compressed = optimizer.compress(long_prompt, 0.5);

        assert!(compressed.len() < long_prompt.len());
    }

    #[test]
    fn test_compress_preserves_sentences() {
        let optimizer = PromptOptimizer::new();
        let prompt = "First sentence. Second sentence. Third sentence. Fourth sentence.";
        let compressed = optimizer.compress(prompt, 0.4);

        // Should end with a period (sentence boundary)
        assert!(compressed.ends_with('.') || compressed.ends_with("..."));
    }
}
