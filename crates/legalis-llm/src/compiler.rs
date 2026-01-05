//! Law compiler using LLM for natural language processing.

use crate::LLMProvider;
use anyhow::Result;
use legalis_core::Statute;

/// Compiles natural language legal text into structured Statute objects.
pub struct LawCompiler<P: LLMProvider> {
    provider: P,
}

impl<P: LLMProvider> LawCompiler<P> {
    /// Creates a new LawCompiler with the given LLM provider.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Compiles natural language statute text into a structured Statute.
    pub async fn compile(&self, raw_text: &str) -> Result<Statute> {
        let system_prompt = r#"You are a 'Legal Compiler'. Convert natural language statute text into Rust structures.
Mark any interpretive or discretionary parts as 'JudicialDiscretion'.
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
}"#;

        let prompt = format!(
            "{}\n\nParse the following statute:\n\n{}",
            system_prompt, raw_text
        );

        self.provider.generate_structured(&prompt).await
    }

    /// Analyzes a statute for potential issues and ambiguities.
    pub async fn analyze(&self, statute: &Statute) -> Result<AnalysisReport> {
        let statute_json = serde_json::to_string_pretty(statute)?;

        let prompt = format!(
            r#"Analyze the following statute for:
1. Logical consistency
2. Ambiguous language that might require judicial interpretation
3. Potential conflicts with common legal principles
4. Missing conditions or edge cases

Statute:
{}

Respond with JSON:
{{
    "issues": ["list of identified issues"],
    "ambiguities": ["list of ambiguous terms or phrases"],
    "recommendations": ["list of recommendations"],
    "discretion_points": ["areas requiring human judgment"]
}}"#,
            statute_json
        );

        self.provider.generate_structured(&prompt).await
    }

    /// Generates a human-readable explanation of a statute.
    pub async fn explain(&self, statute: &Statute) -> Result<String> {
        let statute_json = serde_json::to_string_pretty(statute)?;

        let prompt = format!(
            r#"Explain the following statute in plain language that a non-lawyer can understand.
Include:
1. Who this law applies to
2. What conditions must be met
3. What happens when conditions are met
4. Any areas where human judgment is required

Statute:
{}"#,
            statute_json
        );

        self.provider.generate_text(&prompt).await
    }

    /// Compiles multiple statutes in batch.
    ///
    /// This is more efficient than calling compile() multiple times as it can
    /// batch requests to the LLM provider.
    pub async fn compile_batch(&self, raw_texts: &[String]) -> Result<Vec<Result<Statute>>> {
        let mut results = Vec::new();

        for text in raw_texts {
            let result = self.compile(text).await;
            results.push(result);
        }

        Ok(results)
    }

    /// Compiles multiple statutes in batch with parallelism.
    ///
    /// Uses concurrent requests to speed up batch compilation.
    pub async fn compile_batch_parallel(
        &self,
        raw_texts: &[String],
        max_concurrent: usize,
    ) -> Result<Vec<Result<Statute>>> {
        use futures::stream::{self, StreamExt};

        let results = stream::iter(raw_texts.iter())
            .map(|text| async move { self.compile(text).await })
            .buffer_unordered(max_concurrent)
            .collect::<Vec<_>>()
            .await;

        Ok(results)
    }
}

/// Compilation cache for statutes.
pub struct CompilationCache {
    cache: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Statute>>>,
}

impl CompilationCache {
    /// Creates a new compilation cache.
    pub fn new() -> Self {
        Self {
            cache: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Gets a cached statute by text hash.
    pub fn get(&self, text: &str) -> Option<Statute> {
        let key = Self::hash_text(text);
        self.cache.lock().unwrap().get(&key).cloned()
    }

    /// Stores a compiled statute in the cache.
    pub fn put(&self, text: &str, statute: Statute) {
        let key = Self::hash_text(text);
        self.cache.lock().unwrap().insert(key, statute);
    }

    /// Clears the cache.
    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }

    /// Gets the cache size.
    pub fn len(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.lock().unwrap().is_empty()
    }

    fn hash_text(text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

impl Default for CompilationCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Law compiler with caching support.
pub struct CachedLawCompiler<P: LLMProvider> {
    compiler: LawCompiler<P>,
    cache: CompilationCache,
}

impl<P: LLMProvider> CachedLawCompiler<P> {
    /// Creates a new cached law compiler.
    pub fn new(provider: P) -> Self {
        Self {
            compiler: LawCompiler::new(provider),
            cache: CompilationCache::new(),
        }
    }

    /// Compiles a statute with caching.
    pub async fn compile(&self, raw_text: &str) -> Result<Statute> {
        // Check cache first
        if let Some(cached) = self.cache.get(raw_text) {
            tracing::debug!("Cache hit for statute compilation");
            return Ok(cached);
        }

        // Cache miss - compile and store
        let statute = self.compiler.compile(raw_text).await?;
        self.cache.put(raw_text, statute.clone());

        Ok(statute)
    }

    /// Compiles multiple statutes in batch with caching.
    pub async fn compile_batch(&self, raw_texts: &[String]) -> Result<Vec<Result<Statute>>> {
        let mut results = Vec::new();

        for text in raw_texts {
            let result = self.compile(text).await;
            results.push(result);
        }

        Ok(results)
    }

    /// Analyzes a statute.
    pub async fn analyze(&self, statute: &Statute) -> Result<AnalysisReport> {
        self.compiler.analyze(statute).await
    }

    /// Explains a statute.
    pub async fn explain(&self, statute: &Statute) -> Result<String> {
        self.compiler.explain(statute).await
    }

    /// Clears the compilation cache.
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Gets cache statistics.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Report from statute analysis.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisReport {
    /// Identified issues
    pub issues: Vec<String>,
    /// Ambiguous terms or phrases
    pub ambiguities: Vec<String>,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
    /// Points requiring human judgment
    pub discretion_points: Vec<String>,
}

/// Incremental compilation support for law compiler.
pub mod incremental {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    /// Tracks which parts of a statute have changed.
    #[derive(Debug, Clone)]
    pub struct ChangeSet {
        /// Modified text sections (key: section ID, value: new text)
        pub modified: HashMap<String, String>,
        /// Deleted section IDs
        pub deleted: Vec<String>,
        /// New text sections (key: section ID, value: text)
        pub added: HashMap<String, String>,
    }

    impl ChangeSet {
        /// Creates an empty change set.
        pub fn new() -> Self {
            Self {
                modified: HashMap::new(),
                deleted: Vec::new(),
                added: HashMap::new(),
            }
        }

        /// Adds a modified section.
        pub fn modify(mut self, id: impl Into<String>, text: impl Into<String>) -> Self {
            self.modified.insert(id.into(), text.into());
            self
        }

        /// Adds a deleted section.
        pub fn delete(mut self, id: impl Into<String>) -> Self {
            self.deleted.push(id.into());
            self
        }

        /// Adds a new section.
        pub fn add(mut self, id: impl Into<String>, text: impl Into<String>) -> Self {
            self.added.insert(id.into(), text.into());
            self
        }

        /// Checks if this change set is empty.
        pub fn is_empty(&self) -> bool {
            self.modified.is_empty() && self.deleted.is_empty() && self.added.is_empty()
        }
    }

    impl Default for ChangeSet {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Incremental compiler that only recompiles changed sections.
    pub struct IncrementalCompiler<P: LLMProvider> {
        compiler: LawCompiler<P>,
        compiled_sections: Arc<Mutex<HashMap<String, Statute>>>,
    }

    impl<P: LLMProvider> IncrementalCompiler<P> {
        /// Creates a new incremental compiler.
        pub fn new(provider: P) -> Self {
            Self {
                compiler: LawCompiler::new(provider),
                compiled_sections: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Compiles only the changed sections.
        pub async fn compile_incremental(
            &self,
            changes: &ChangeSet,
        ) -> Result<HashMap<String, Statute>> {
            let mut results = HashMap::new();

            // Compile modified sections
            for (id, text) in &changes.modified {
                tracing::debug!("Incrementally compiling modified section: {}", id);
                let statute = self.compiler.compile(text).await?;
                results.insert(id.clone(), statute.clone());

                // Update cache
                self.compiled_sections
                    .lock()
                    .unwrap()
                    .insert(id.clone(), statute);
            }

            // Compile new sections
            for (id, text) in &changes.added {
                tracing::debug!("Compiling new section: {}", id);
                let statute = self.compiler.compile(text).await?;
                results.insert(id.clone(), statute.clone());

                // Update cache
                self.compiled_sections
                    .lock()
                    .unwrap()
                    .insert(id.clone(), statute);
            }

            // Remove deleted sections
            for id in &changes.deleted {
                tracing::debug!("Removing deleted section: {}", id);
                self.compiled_sections.lock().unwrap().remove(id);
            }

            Ok(results)
        }

        /// Gets a compiled section by ID.
        pub fn get_section(&self, id: &str) -> Option<Statute> {
            self.compiled_sections.lock().unwrap().get(id).cloned()
        }

        /// Gets all compiled sections.
        pub fn get_all_sections(&self) -> HashMap<String, Statute> {
            self.compiled_sections.lock().unwrap().clone()
        }

        /// Clears all cached sections.
        pub fn clear(&self) {
            self.compiled_sections.lock().unwrap().clear();
        }
    }
}

/// Compilation pipeline with configurable stages.
pub mod pipeline {
    use super::*;
    use std::sync::Arc;

    /// A stage in the compilation pipeline.
    #[async_trait::async_trait]
    pub trait CompilationStage: Send + Sync {
        /// Processes the input text and returns transformed text.
        async fn process(&self, input: String) -> Result<String>;

        /// Returns the name of this stage.
        fn name(&self) -> &str;
    }

    /// Pre-processor for normalizing input text.
    pub struct NormalizationStage;

    #[async_trait::async_trait]
    impl CompilationStage for NormalizationStage {
        async fn process(&self, input: String) -> Result<String> {
            // Normalize whitespace, remove extra newlines, etc.
            let normalized = input
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            Ok(normalized)
        }

        fn name(&self) -> &str {
            "Normalization"
        }
    }

    /// Stage that enriches text with context.
    pub struct EnrichmentStage {
        context: String,
    }

    impl EnrichmentStage {
        /// Creates a new enrichment stage with the given context.
        pub fn new(context: impl Into<String>) -> Self {
            Self {
                context: context.into(),
            }
        }
    }

    #[async_trait::async_trait]
    impl CompilationStage for EnrichmentStage {
        async fn process(&self, input: String) -> Result<String> {
            Ok(format!("Context: {}\n\n{}", self.context, input))
        }

        fn name(&self) -> &str {
            "Enrichment"
        }
    }

    /// Custom processing stage using a closure.
    pub struct CustomStage<F>
    where
        F: Fn(String) -> Result<String> + Send + Sync,
    {
        name: String,
        processor: F,
    }

    impl<F> CustomStage<F>
    where
        F: Fn(String) -> Result<String> + Send + Sync,
    {
        /// Creates a new custom stage.
        pub fn new(name: impl Into<String>, processor: F) -> Self {
            Self {
                name: name.into(),
                processor,
            }
        }
    }

    #[async_trait::async_trait]
    impl<F> CompilationStage for CustomStage<F>
    where
        F: Fn(String) -> Result<String> + Send + Sync,
    {
        async fn process(&self, input: String) -> Result<String> {
            (self.processor)(input)
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    /// Compilation pipeline with multiple stages.
    pub struct CompilationPipeline<P: LLMProvider> {
        compiler: LawCompiler<P>,
        stages: Vec<Arc<dyn CompilationStage>>,
    }

    impl<P: LLMProvider> CompilationPipeline<P> {
        /// Creates a new compilation pipeline.
        pub fn new(provider: P) -> Self {
            Self {
                compiler: LawCompiler::new(provider),
                stages: Vec::new(),
            }
        }

        /// Adds a stage to the pipeline.
        pub fn add_stage<S: CompilationStage + 'static>(mut self, stage: S) -> Self {
            self.stages.push(Arc::new(stage));
            self
        }

        /// Compiles text through the pipeline.
        pub async fn compile(&self, raw_text: &str) -> Result<Statute> {
            let mut text = raw_text.to_string();

            // Process through all stages
            for stage in &self.stages {
                tracing::debug!("Processing through stage: {}", stage.name());
                text = stage.process(text).await?;
            }

            // Final compilation
            self.compiler.compile(&text).await
        }

        /// Compiles batch through the pipeline.
        pub async fn compile_batch(&self, raw_texts: &[String]) -> Result<Vec<Result<Statute>>> {
            let mut results = Vec::new();

            for text in raw_texts {
                let result = self.compile(text).await;
                results.push(result);
            }

            Ok(results)
        }
    }
}

/// Pre and post processors for custom compilation logic.
pub mod processors {
    use super::*;

    /// Pre-processor that runs before compilation.
    pub trait PreProcessor: Send + Sync {
        /// Processes raw text before compilation.
        fn process(&self, text: &str) -> Result<String>;
    }

    /// Post-processor that runs after compilation.
    pub trait PostProcessor: Send + Sync {
        /// Processes a compiled statute.
        fn process(&self, statute: Statute) -> Result<Statute>;
    }

    /// Pre-processor that removes comments.
    pub struct CommentRemovalProcessor;

    impl PreProcessor for CommentRemovalProcessor {
        fn process(&self, text: &str) -> Result<String> {
            let result = text
                .lines()
                .filter(|line| !line.trim().starts_with("//") && !line.trim().starts_with("#"))
                .collect::<Vec<_>>()
                .join("\n");

            Ok(result)
        }
    }

    /// Pre-processor that validates input format.
    pub struct ValidationProcessor {
        min_length: usize,
    }

    impl ValidationProcessor {
        /// Creates a new validation processor.
        pub fn new(min_length: usize) -> Self {
            Self { min_length }
        }
    }

    impl PreProcessor for ValidationProcessor {
        fn process(&self, text: &str) -> Result<String> {
            if text.len() < self.min_length {
                return Err(anyhow::anyhow!(
                    "Input text too short: {} < {}",
                    text.len(),
                    self.min_length
                ));
            }

            Ok(text.to_string())
        }
    }

    /// Post-processor that enriches statute metadata.
    pub struct MetadataEnricher {
        jurisdiction: Option<String>,
    }

    impl MetadataEnricher {
        /// Creates a new metadata enricher.
        pub fn new() -> Self {
            Self { jurisdiction: None }
        }

        /// Sets the jurisdiction.
        pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
            self.jurisdiction = Some(jurisdiction.into());
            self
        }
    }

    impl Default for MetadataEnricher {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PostProcessor for MetadataEnricher {
        fn process(&self, mut statute: Statute) -> Result<Statute> {
            if let Some(ref jurisdiction) = self.jurisdiction {
                statute.jurisdiction = Some(jurisdiction.clone());
            }

            Ok(statute)
        }
    }

    /// Compiler with pre and post processors.
    pub struct ProcessorCompiler<P: LLMProvider> {
        compiler: LawCompiler<P>,
        preprocessors: Vec<Box<dyn PreProcessor>>,
        postprocessors: Vec<Box<dyn PostProcessor>>,
    }

    impl<P: LLMProvider> ProcessorCompiler<P> {
        /// Creates a new processor compiler.
        pub fn new(provider: P) -> Self {
            Self {
                compiler: LawCompiler::new(provider),
                preprocessors: Vec::new(),
                postprocessors: Vec::new(),
            }
        }

        /// Adds a pre-processor.
        pub fn add_preprocessor<T: PreProcessor + 'static>(mut self, processor: T) -> Self {
            self.preprocessors.push(Box::new(processor));
            self
        }

        /// Adds a post-processor.
        pub fn add_postprocessor<T: PostProcessor + 'static>(mut self, processor: T) -> Self {
            self.postprocessors.push(Box::new(processor));
            self
        }

        /// Compiles text with pre and post processing.
        pub async fn compile(&self, raw_text: &str) -> Result<Statute> {
            // Run pre-processors
            let mut text = raw_text.to_string();
            for processor in &self.preprocessors {
                text = processor.process(&text)?;
            }

            // Compile
            let mut statute = self.compiler.compile(&text).await?;

            // Run post-processors
            for processor in &self.postprocessors {
                statute = processor.process(statute)?;
            }

            Ok(statute)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;
    use legalis_core::{Effect, EffectType};

    #[tokio::test]
    async fn test_law_compiler_with_mock() {
        let mock_response = r#"{
            "id": "test-statute-1",
            "title": "Test Statute",
            "preconditions": [],
            "effect": {
                "effect_type": "Grant",
                "description": "Test effect",
                "parameters": {}
            },
            "discretion_logic": null,
            "temporal_validity": {
                "effective_date": null,
                "expiry_date": null,
                "enacted_at": null,
                "amended_at": null
            },
            "version": 1,
            "jurisdiction": null,
            "derives_from": [],
            "applies_to": [],
            "exceptions": []
        }"#;

        let provider = MockProvider::new().with_response("Parse", mock_response);
        let compiler = LawCompiler::new(provider);

        let result = compiler.compile("Test statute text").await;
        assert!(result.is_ok());

        let statute = result.unwrap();
        assert_eq!(statute.id, "test-statute-1");
    }

    #[tokio::test]
    async fn test_analysis_report() {
        let mock_response = r#"{
            "issues": ["No expiration date specified"],
            "ambiguities": ["'reasonable time' is undefined"],
            "recommendations": ["Add specific time limits"],
            "discretion_points": ["Determining what constitutes 'reasonable'"]
        }"#;

        let provider = MockProvider::new().with_response("Analyze", mock_response);
        let compiler = LawCompiler::new(provider);

        let statute = Statute::new(
            "test-1",
            "Test",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let report = compiler.analyze(&statute).await;
        assert!(report.is_ok());

        let report = report.unwrap();
        assert!(!report.issues.is_empty());
    }
}
