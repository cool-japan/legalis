//! Transformation pipeline for customizing legal document conversions.
//!
//! This module provides:
//! - Custom transformation hooks for modifying statutes during conversion
//! - Pre/post processing plugins for source and output text
//! - Content normalization rules for standardizing legal text
//! - Identifier mapping tables for renaming identifiers between formats
//! - Conditional transformation logic for applying transformations based on context

use crate::{ConversionReport, InteropError, InteropResult, LegalFormat};
use legalis_core::Statute;
use regex_lite::Regex;
use std::collections::HashMap;
use std::sync::Arc;

/// A transformation hook that modifies a statute during conversion.
pub type TransformationHook =
    Arc<dyn Fn(&mut Statute, &TransformContext) -> InteropResult<()> + Send + Sync>;

/// A pre-processing plugin that modifies source text before parsing.
pub type PreProcessor = Arc<dyn Fn(&str, &TransformContext) -> InteropResult<String> + Send + Sync>;

/// A post-processing plugin that modifies output text after generation.
pub type PostProcessor =
    Arc<dyn Fn(&str, &TransformContext) -> InteropResult<String> + Send + Sync>;

/// A custom condition function for evaluating conditions.
pub type ConditionFn = Arc<dyn Fn(&Statute, &TransformContext) -> bool + Send + Sync>;

/// Context information available during transformation.
#[derive(Debug, Clone)]
pub struct TransformContext {
    /// Source format
    pub source_format: Option<LegalFormat>,
    /// Target format
    pub target_format: Option<LegalFormat>,
    /// Conversion phase
    pub phase: TransformPhase,
    /// Custom metadata for the transformation
    pub metadata: HashMap<String, String>,
}

impl TransformContext {
    /// Creates a new transformation context.
    pub fn new(
        source: Option<LegalFormat>,
        target: Option<LegalFormat>,
        phase: TransformPhase,
    ) -> Self {
        Self {
            source_format: source,
            target_format: target,
            phase,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the context.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Phase of the transformation pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformPhase {
    /// Before parsing (source text preprocessing)
    PreParse,
    /// After parsing (statute transformation)
    PostParse,
    /// Before generation (statute transformation)
    PreGenerate,
    /// After generation (output text postprocessing)
    PostGenerate,
}

/// A transformation pipeline that applies custom transformations during conversion.
pub struct TransformationPipeline {
    /// Pre-processing plugins
    preprocessors: Vec<PreProcessor>,
    /// Post-processing plugins
    postprocessors: Vec<PostProcessor>,
    /// Transformation hooks for statutes
    hooks: Vec<TransformationHook>,
    /// Content normalization rules
    normalizer: ContentNormalizer,
    /// Identifier mapping tables
    identifier_mapper: IdentifierMapper,
    /// Conditional transformation rules
    conditional_rules: Vec<ConditionalRule>,
}

impl Default for TransformationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformationPipeline {
    /// Creates a new transformation pipeline.
    pub fn new() -> Self {
        Self {
            preprocessors: Vec::new(),
            postprocessors: Vec::new(),
            hooks: Vec::new(),
            normalizer: ContentNormalizer::new(),
            identifier_mapper: IdentifierMapper::new(),
            conditional_rules: Vec::new(),
        }
    }

    /// Adds a pre-processing plugin.
    pub fn add_preprocessor<F>(&mut self, processor: F)
    where
        F: Fn(&str, &TransformContext) -> InteropResult<String> + Send + Sync + 'static,
    {
        self.preprocessors.push(Arc::new(processor));
    }

    /// Adds a post-processing plugin.
    pub fn add_postprocessor<F>(&mut self, processor: F)
    where
        F: Fn(&str, &TransformContext) -> InteropResult<String> + Send + Sync + 'static,
    {
        self.postprocessors.push(Arc::new(processor));
    }

    /// Adds a transformation hook.
    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Statute, &TransformContext) -> InteropResult<()> + Send + Sync + 'static,
    {
        self.hooks.push(Arc::new(hook));
    }

    /// Adds a normalization rule.
    pub fn add_normalization_rule(&mut self, rule: NormalizationRule) {
        self.normalizer.add_rule(rule);
    }

    /// Adds an identifier mapping.
    pub fn add_identifier_mapping(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.identifier_mapper.add_mapping(from, to);
    }

    /// Adds a conditional transformation rule.
    pub fn add_conditional_rule(&mut self, rule: ConditionalRule) {
        self.conditional_rules.push(rule);
    }

    /// Returns a mutable reference to the normalizer.
    pub fn normalizer_mut(&mut self) -> &mut ContentNormalizer {
        &mut self.normalizer
    }

    /// Returns a mutable reference to the identifier mapper.
    pub fn identifier_mapper_mut(&mut self) -> &mut IdentifierMapper {
        &mut self.identifier_mapper
    }

    /// Applies pre-processing to source text.
    pub fn preprocess(&self, source: &str, context: &TransformContext) -> InteropResult<String> {
        let mut result = source.to_string();

        // Apply normalization if in pre-parse phase
        if context.phase == TransformPhase::PreParse {
            result = self.normalizer.normalize(&result)?;
        }

        // Apply preprocessors
        for processor in &self.preprocessors {
            result = processor(&result, context)?;
        }

        Ok(result)
    }

    /// Applies post-processing to output text.
    pub fn postprocess(&self, output: &str, context: &TransformContext) -> InteropResult<String> {
        let mut result = output.to_string();

        // Apply postprocessors
        for processor in &self.postprocessors {
            result = processor(&result, context)?;
        }

        Ok(result)
    }

    /// Applies transformation hooks to statutes.
    pub fn transform_statutes(
        &self,
        statutes: &mut [Statute],
        context: &TransformContext,
    ) -> InteropResult<()> {
        for statute in statutes.iter_mut() {
            // Apply identifier mappings
            self.identifier_mapper.apply(statute)?;

            // Apply conditional rules
            for rule in &self.conditional_rules {
                if rule.condition.evaluate(statute, context) {
                    (rule.action)(statute, context)?;
                }
            }

            // Apply hooks
            for hook in &self.hooks {
                hook(statute, context)?;
            }
        }

        Ok(())
    }

    /// Creates a builder for constructing transformation pipelines.
    pub fn builder() -> TransformationPipelineBuilder {
        TransformationPipelineBuilder::new()
    }
}

/// Builder for transformation pipelines.
pub struct TransformationPipelineBuilder {
    pipeline: TransformationPipeline,
}

impl TransformationPipelineBuilder {
    /// Creates a new pipeline builder.
    pub fn new() -> Self {
        Self {
            pipeline: TransformationPipeline::new(),
        }
    }

    /// Adds a pre-processing plugin.
    pub fn with_preprocessor<F>(mut self, processor: F) -> Self
    where
        F: Fn(&str, &TransformContext) -> InteropResult<String> + Send + Sync + 'static,
    {
        self.pipeline.add_preprocessor(processor);
        self
    }

    /// Adds a post-processing plugin.
    pub fn with_postprocessor<F>(mut self, processor: F) -> Self
    where
        F: Fn(&str, &TransformContext) -> InteropResult<String> + Send + Sync + 'static,
    {
        self.pipeline.add_postprocessor(processor);
        self
    }

    /// Adds a transformation hook.
    pub fn with_hook<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut Statute, &TransformContext) -> InteropResult<()> + Send + Sync + 'static,
    {
        self.pipeline.add_hook(hook);
        self
    }

    /// Adds a normalization rule.
    pub fn with_normalization_rule(mut self, rule: NormalizationRule) -> Self {
        self.pipeline.add_normalization_rule(rule);
        self
    }

    /// Adds an identifier mapping.
    pub fn with_identifier_mapping(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        self.pipeline.add_identifier_mapping(from, to);
        self
    }

    /// Adds a conditional transformation rule.
    pub fn with_conditional_rule(mut self, rule: ConditionalRule) -> Self {
        self.pipeline.add_conditional_rule(rule);
        self
    }

    /// Builds the transformation pipeline.
    pub fn build(self) -> TransformationPipeline {
        self.pipeline
    }
}

impl Default for TransformationPipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Content normalizer for standardizing legal text.
#[derive(Clone)]
pub struct ContentNormalizer {
    rules: Vec<NormalizationRule>,
}

impl ContentNormalizer {
    /// Creates a new content normalizer.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Creates a normalizer with default rules.
    pub fn with_defaults() -> Self {
        let mut normalizer = Self::new();
        normalizer.add_rule(NormalizationRule::NormalizeWhitespace);
        normalizer.add_rule(NormalizationRule::NormalizeQuotes);
        normalizer.add_rule(NormalizationRule::RemoveComments);
        normalizer
    }

    /// Adds a normalization rule.
    pub fn add_rule(&mut self, rule: NormalizationRule) {
        self.rules.push(rule);
    }

    /// Normalizes text according to configured rules.
    pub fn normalize(&self, text: &str) -> InteropResult<String> {
        let mut result = text.to_string();

        for rule in &self.rules {
            result = rule.apply(&result)?;
        }

        Ok(result)
    }
}

impl Default for ContentNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

/// A normalization rule for content.
#[derive(Debug, Clone)]
pub enum NormalizationRule {
    /// Normalize whitespace (collapse multiple spaces, trim lines)
    NormalizeWhitespace,
    /// Convert all quotes to straight quotes
    NormalizeQuotes,
    /// Remove comments (lines starting with #, //, etc.)
    RemoveComments,
    /// Convert to lowercase
    Lowercase,
    /// Convert to uppercase
    Uppercase,
    /// Custom regex-based replacement
    RegexReplace {
        pattern: String,
        replacement: String,
    },
}

impl NormalizationRule {
    /// Applies the normalization rule to text.
    pub fn apply(&self, text: &str) -> InteropResult<String> {
        match self {
            NormalizationRule::NormalizeWhitespace => {
                let lines: Vec<String> = text
                    .lines()
                    .map(|line| {
                        // Collapse multiple spaces
                        let mut result = String::new();
                        let mut prev_space = false;
                        for ch in line.chars() {
                            if ch.is_whitespace() {
                                if !prev_space {
                                    result.push(' ');
                                    prev_space = true;
                                }
                            } else {
                                result.push(ch);
                                prev_space = false;
                            }
                        }
                        result.trim().to_string()
                    })
                    .filter(|line| !line.is_empty())
                    .collect();
                Ok(lines.join("\n"))
            }
            NormalizationRule::NormalizeQuotes => {
                let result = text
                    .replace(['\u{201C}', '\u{201D}'], "\"") // left/right double quotation marks
                    .replace(['\u{2018}', '\u{2019}'], "'"); // left/right single quotation marks
                Ok(result)
            }
            NormalizationRule::RemoveComments => {
                let lines: Vec<String> = text
                    .lines()
                    .filter(|line| {
                        let trimmed = line.trim();
                        !trimmed.starts_with('#') && !trimmed.starts_with("//")
                    })
                    .map(|s| s.to_string())
                    .collect();
                Ok(lines.join("\n"))
            }
            NormalizationRule::Lowercase => Ok(text.to_lowercase()),
            NormalizationRule::Uppercase => Ok(text.to_uppercase()),
            NormalizationRule::RegexReplace {
                pattern,
                replacement,
            } => {
                let re = Regex::new(pattern)
                    .map_err(|e| InteropError::ConversionError(format!("Invalid regex: {}", e)))?;
                Ok(re.replace_all(text, replacement.as_str()).to_string())
            }
        }
    }
}

/// Identifier mapper for renaming identifiers between formats.
#[derive(Clone, Default)]
pub struct IdentifierMapper {
    mappings: HashMap<String, String>,
}

impl IdentifierMapper {
    /// Creates a new identifier mapper.
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Adds a mapping from one identifier to another.
    pub fn add_mapping(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.mappings.insert(from.into(), to.into());
    }

    /// Adds multiple mappings at once.
    pub fn add_mappings(&mut self, mappings: HashMap<String, String>) {
        self.mappings.extend(mappings);
    }

    /// Returns the mapped identifier if it exists.
    pub fn get_mapping(&self, identifier: &str) -> Option<&str> {
        self.mappings.get(identifier).map(|s| s.as_str())
    }

    /// Applies identifier mappings to a statute.
    pub fn apply(&self, statute: &mut Statute) -> InteropResult<()> {
        // Map statute ID
        if let Some(new_id) = self.get_mapping(&statute.id) {
            statute.id = new_id.to_string();
        }

        // Map effect description
        if let Some(new_description) = self.get_mapping(&statute.effect.description) {
            statute.effect.description = new_description.to_string();
        }

        // Map parameter keys and values
        let mut new_params = HashMap::new();
        for (key, value) in &statute.effect.parameters {
            let new_key = self.get_mapping(key).unwrap_or(key).to_string();
            let new_value = self.get_mapping(value).unwrap_or(value).to_string();
            new_params.insert(new_key, new_value);
        }
        statute.effect.parameters = new_params;

        Ok(())
    }
}

/// A conditional transformation rule.
pub struct ConditionalRule {
    /// Condition that must be satisfied
    pub condition: Condition,
    /// Action to apply when condition is true
    pub action: TransformationHook,
}

impl ConditionalRule {
    /// Creates a new conditional rule.
    pub fn new<F>(condition: Condition, action: F) -> Self
    where
        F: Fn(&mut Statute, &TransformContext) -> InteropResult<()> + Send + Sync + 'static,
    {
        Self {
            condition,
            action: Arc::new(action),
        }
    }
}

/// A condition for conditional transformation.
#[derive(Clone)]
pub enum Condition {
    /// Always true
    Always,
    /// Never true
    Never,
    /// Source format matches
    SourceFormat(LegalFormat),
    /// Target format matches
    TargetFormat(LegalFormat),
    /// Statute ID matches regex
    StatuteIdMatches(String),
    /// Statute title matches regex
    StatuteTitleMatches(String),
    /// Effect type matches
    EffectTypeMatches(String),
    /// Has specific metadata key
    HasMetadata(String),
    /// Metadata value matches
    MetadataMatches { key: String, value: String },
    /// AND combination of conditions
    And(Vec<Condition>),
    /// OR combination of conditions
    Or(Vec<Condition>),
    /// NOT condition
    Not(Box<Condition>),
    /// Custom condition function
    Custom(ConditionFn),
}

impl Condition {
    /// Evaluates the condition against a statute and context.
    pub fn evaluate(&self, statute: &Statute, context: &TransformContext) -> bool {
        match self {
            Condition::Always => true,
            Condition::Never => false,
            Condition::SourceFormat(format) => context.source_format == Some(*format),
            Condition::TargetFormat(format) => context.target_format == Some(*format),
            Condition::StatuteIdMatches(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(&statute.id)
                } else {
                    false
                }
            }
            Condition::StatuteTitleMatches(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(&statute.title)
                } else {
                    false
                }
            }
            Condition::EffectTypeMatches(pattern) => {
                let effect_str = format!("{:?}", statute.effect.effect_type);
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(&effect_str)
                } else {
                    false
                }
            }
            Condition::HasMetadata(key) => context.metadata.contains_key(key),
            Condition::MetadataMatches { key, value } => context.metadata.get(key) == Some(value),
            Condition::And(conditions) => conditions.iter().all(|c| c.evaluate(statute, context)),
            Condition::Or(conditions) => conditions.iter().any(|c| c.evaluate(statute, context)),
            Condition::Not(condition) => !condition.evaluate(statute, context),
            Condition::Custom(func) => func(statute, context),
        }
    }

    /// Creates an AND combination of conditions.
    pub fn and(conditions: Vec<Condition>) -> Self {
        Condition::And(conditions)
    }

    /// Creates an OR combination of conditions.
    pub fn or(conditions: Vec<Condition>) -> Self {
        Condition::Or(conditions)
    }

    /// Creates a NOT condition.
    pub fn negate(condition: Condition) -> Self {
        Condition::Not(Box::new(condition))
    }
}

/// Extension trait for LegalConverter to support transformation pipelines.
pub trait TransformationSupport {
    /// Converts with a transformation pipeline.
    fn convert_with_pipeline(
        &mut self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
        pipeline: &TransformationPipeline,
    ) -> InteropResult<(String, ConversionReport)>;
}

impl TransformationSupport for crate::LegalConverter {
    fn convert_with_pipeline(
        &mut self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
        pipeline: &TransformationPipeline,
    ) -> InteropResult<(String, ConversionReport)> {
        // Pre-processing
        let preprocess_context =
            TransformContext::new(Some(from), Some(to), TransformPhase::PreParse);
        let preprocessed = pipeline.preprocess(source, &preprocess_context)?;

        // Import
        let (mut statutes, mut report) = self.import(&preprocessed, from)?;

        // Transform statutes
        let transform_context =
            TransformContext::new(Some(from), Some(to), TransformPhase::PostParse);
        pipeline.transform_statutes(&mut statutes, &transform_context)?;

        // Export
        let (output, export_report) = self.export(&statutes, to)?;

        // Post-processing
        let postprocess_context =
            TransformContext::new(Some(from), Some(to), TransformPhase::PostGenerate);
        let postprocessed = pipeline.postprocess(&output, &postprocess_context)?;

        // Merge reports
        report.target_format = Some(to);
        report
            .unsupported_features
            .extend(export_report.unsupported_features);
        report.warnings.extend(export_report.warnings);
        report.confidence = (report.confidence * export_report.confidence).max(0.0);

        Ok((postprocessed, report))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_normalization_whitespace() {
        let normalizer = ContentNormalizer::with_defaults();
        let input = "  multiple   spaces   here  \n\n  and   here  ";
        let result = normalizer.normalize(input).unwrap();
        assert!(!result.contains("  "));
    }

    #[test]
    fn test_normalization_quotes() {
        let rule = NormalizationRule::NormalizeQuotes;
        // Using Unicode escape sequences for curly quotes
        let input = "\u{201C}curly quotes\u{201D} and \u{2018}single curly\u{2019}";
        let result = rule.apply(input).unwrap();
        assert_eq!(result, "\"curly quotes\" and 'single curly'");
    }

    #[test]
    fn test_normalization_remove_comments() {
        let rule = NormalizationRule::RemoveComments;
        let input = "code here\n# comment\nmore code\n// another comment";
        let result = rule.apply(input).unwrap();
        assert!(!result.contains("comment"));
        assert!(result.contains("code here"));
        assert!(result.contains("more code"));
    }

    #[test]
    fn test_normalization_case() {
        let lower = NormalizationRule::Lowercase;
        let upper = NormalizationRule::Uppercase;

        assert_eq!(lower.apply("TeSt").unwrap(), "test");
        assert_eq!(upper.apply("TeSt").unwrap(), "TEST");
    }

    #[test]
    fn test_normalization_regex() {
        let rule = NormalizationRule::RegexReplace {
            pattern: r"\d+".to_string(),
            replacement: "X".to_string(),
        };
        let result = rule.apply("age >= 18 and year > 2020").unwrap();
        assert_eq!(result, "age >= X and year > X");
    }

    #[test]
    fn test_identifier_mapper() {
        let mut mapper = IdentifierMapper::new();
        mapper.add_mapping("old_id", "new_id");
        mapper.add_mapping("old_description", "new_description");

        let mut statute = Statute::new(
            "old_id",
            "Test",
            Effect::new(EffectType::Grant, "old_description"),
        );

        mapper.apply(&mut statute).unwrap();

        assert_eq!(statute.id, "new_id");
        assert_eq!(statute.effect.description, "new_description");
    }

    #[test]
    fn test_identifier_mapper_parameters() {
        let mut mapper = IdentifierMapper::new();
        mapper.add_mapping("age", "person_age");
        mapper.add_mapping("18", "21");

        let mut effect = Effect::new(EffectType::Grant, "test");
        effect
            .parameters
            .insert("age".to_string(), "18".to_string());
        let mut statute = Statute::new("test", "Test", effect);

        mapper.apply(&mut statute).unwrap();

        assert_eq!(
            statute.effect.parameters.get("person_age"),
            Some(&"21".to_string())
        );
    }

    #[test]
    fn test_condition_always_never() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let context = TransformContext::new(None, None, TransformPhase::PostParse);

        assert!(Condition::Always.evaluate(&statute, &context));
        assert!(!Condition::Never.evaluate(&statute, &context));
    }

    #[test]
    fn test_condition_format() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let context = TransformContext::new(
            Some(LegalFormat::Catala),
            Some(LegalFormat::L4),
            TransformPhase::PostParse,
        );

        assert!(Condition::SourceFormat(LegalFormat::Catala).evaluate(&statute, &context));
        assert!(Condition::TargetFormat(LegalFormat::L4).evaluate(&statute, &context));
        assert!(!Condition::SourceFormat(LegalFormat::L4).evaluate(&statute, &context));
    }

    #[test]
    fn test_condition_regex() {
        let statute = Statute::new(
            "test-123",
            "Test Statute",
            Effect::new(EffectType::Grant, "test"),
        );
        let context = TransformContext::new(None, None, TransformPhase::PostParse);

        let id_condition = Condition::StatuteIdMatches(r"test-\d+".to_string());
        assert!(id_condition.evaluate(&statute, &context));

        let title_condition = Condition::StatuteTitleMatches(r"Test.*".to_string());
        assert!(title_condition.evaluate(&statute, &context));
    }

    #[test]
    fn test_condition_metadata() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let mut context = TransformContext::new(None, None, TransformPhase::PostParse);
        context
            .metadata
            .insert("jurisdiction".to_string(), "US".to_string());

        assert!(Condition::HasMetadata("jurisdiction".to_string()).evaluate(&statute, &context));
        assert!(!Condition::HasMetadata("missing".to_string()).evaluate(&statute, &context));

        let matches = Condition::MetadataMatches {
            key: "jurisdiction".to_string(),
            value: "US".to_string(),
        };
        assert!(matches.evaluate(&statute, &context));
    }

    #[test]
    fn test_condition_and_or_not() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let context =
            TransformContext::new(Some(LegalFormat::Catala), None, TransformPhase::PostParse);

        let and = Condition::and(vec![
            Condition::SourceFormat(LegalFormat::Catala),
            Condition::Always,
        ]);
        assert!(and.evaluate(&statute, &context));

        let or = Condition::or(vec![
            Condition::Never,
            Condition::SourceFormat(LegalFormat::Catala),
        ]);
        assert!(or.evaluate(&statute, &context));

        let not = Condition::negate(Condition::Never);
        assert!(not.evaluate(&statute, &context));
    }

    #[test]
    fn test_pipeline_builder() {
        let pipeline = TransformationPipeline::builder()
            .with_preprocessor(|text, _| Ok(text.to_uppercase()))
            .with_postprocessor(|text, _| Ok(text.to_lowercase()))
            .with_hook(|statute, _| {
                statute.title = statute.title.to_uppercase();
                Ok(())
            })
            .with_normalization_rule(NormalizationRule::NormalizeWhitespace)
            .with_identifier_mapping("old", "new")
            .build();

        assert_eq!(pipeline.preprocessors.len(), 1);
        assert_eq!(pipeline.postprocessors.len(), 1);
        assert_eq!(pipeline.hooks.len(), 1);
        assert_eq!(pipeline.normalizer.rules.len(), 1);
        assert_eq!(pipeline.identifier_mapper.mappings.len(), 1);
    }

    #[test]
    fn test_pipeline_preprocess() {
        let mut pipeline = TransformationPipeline::new();
        pipeline.add_preprocessor(|text, _| Ok(text.to_uppercase()));
        pipeline.add_preprocessor(|text, _| Ok(format!("PREFIX: {}", text)));

        let context = TransformContext::new(None, None, TransformPhase::PreParse);
        let result = pipeline.preprocess("test", &context).unwrap();

        assert!(result.starts_with("PREFIX:"));
        assert!(result.contains("TEST"));
    }

    #[test]
    fn test_pipeline_postprocess() {
        let mut pipeline = TransformationPipeline::new();
        pipeline.add_postprocessor(|text, _| Ok(text.to_lowercase()));
        pipeline.add_postprocessor(|text, _| Ok(format!("{} SUFFIX", text)));

        let context = TransformContext::new(None, None, TransformPhase::PostGenerate);
        let result = pipeline.postprocess("TEST", &context).unwrap();

        assert_eq!(result, "test SUFFIX");
    }

    #[test]
    fn test_pipeline_transform_statutes() {
        let mut pipeline = TransformationPipeline::new();
        pipeline.add_hook(|statute, _| {
            statute.title = statute.title.to_uppercase();
            Ok(())
        });

        let context = TransformContext::new(None, None, TransformPhase::PostParse);
        let mut statutes = vec![Statute::new(
            "test",
            "test title",
            Effect::new(EffectType::Grant, "test"),
        )];

        pipeline
            .transform_statutes(&mut statutes, &context)
            .unwrap();

        assert_eq!(statutes[0].title, "TEST TITLE");
    }

    #[test]
    fn test_conditional_rule() {
        let rule = ConditionalRule::new(
            Condition::SourceFormat(LegalFormat::Catala),
            |statute, _| {
                statute.title = "MODIFIED".to_string();
                Ok(())
            },
        );

        let mut pipeline = TransformationPipeline::new();
        pipeline.add_conditional_rule(rule);

        let context =
            TransformContext::new(Some(LegalFormat::Catala), None, TransformPhase::PostParse);
        let mut statutes = vec![Statute::new(
            "test",
            "original",
            Effect::new(EffectType::Grant, "test"),
        )];

        pipeline
            .transform_statutes(&mut statutes, &context)
            .unwrap();

        assert_eq!(statutes[0].title, "MODIFIED");
    }

    #[test]
    fn test_conditional_rule_not_triggered() {
        let rule = ConditionalRule::new(Condition::SourceFormat(LegalFormat::L4), |statute, _| {
            statute.title = "MODIFIED".to_string();
            Ok(())
        });

        let mut pipeline = TransformationPipeline::new();
        pipeline.add_conditional_rule(rule);

        let context =
            TransformContext::new(Some(LegalFormat::Catala), None, TransformPhase::PostParse);
        let mut statutes = vec![Statute::new(
            "test",
            "original",
            Effect::new(EffectType::Grant, "test"),
        )];

        pipeline
            .transform_statutes(&mut statutes, &context)
            .unwrap();

        assert_eq!(statutes[0].title, "original");
    }

    #[test]
    fn test_pipeline_with_all_features() {
        let mut pipeline = TransformationPipeline::new();

        // Add preprocessor
        pipeline.add_preprocessor(|text, _| Ok(text.trim().to_string()));

        // Add normalizer
        pipeline.add_normalization_rule(NormalizationRule::NormalizeWhitespace);

        // Add identifier mapping
        pipeline.add_identifier_mapping("old_id", "new_id");

        // Add conditional rule
        pipeline.add_conditional_rule(ConditionalRule::new(Condition::Always, |statute, _| {
            statute.title = statute.title.to_uppercase();
            Ok(())
        }));

        // Add hook
        pipeline.add_hook(|statute, _| {
            statute.id = format!("transformed_{}", statute.id);
            Ok(())
        });

        // Add postprocessor
        pipeline.add_postprocessor(|text, _| Ok(format!("// Output\n{}", text)));

        // Test preprocessing
        let preprocess_ctx = TransformContext::new(None, None, TransformPhase::PreParse);
        let preprocessed = pipeline.preprocess("  test  ", &preprocess_ctx).unwrap();
        assert!(!preprocessed.starts_with("  "));

        // Test statute transformation
        let transform_ctx = TransformContext::new(None, None, TransformPhase::PostParse);
        let mut statutes = vec![Statute::new(
            "new_id",
            "test",
            Effect::new(EffectType::Grant, "test"),
        )];
        pipeline
            .transform_statutes(&mut statutes, &transform_ctx)
            .unwrap();
        assert_eq!(statutes[0].title, "TEST");
        assert_eq!(statutes[0].id, "transformed_new_id");

        // Test postprocessing
        let postprocess_ctx = TransformContext::new(None, None, TransformPhase::PostGenerate);
        let postprocessed = pipeline.postprocess("output", &postprocess_ctx).unwrap();
        assert!(postprocessed.starts_with("// Output"));
    }
}
