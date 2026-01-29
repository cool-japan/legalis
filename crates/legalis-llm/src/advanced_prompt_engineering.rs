//! Advanced Prompt Engineering features for legalis-llm v0.5.2
//!
//! This module provides sophisticated prompt engineering capabilities including:
//! - Dynamic prompt assembly from templates
//! - Context-aware prompt selection
//! - Prompt performance analytics
//! - Automatic prompt refinement based on feedback
//! - Few-shot learning prompt generation
//! - Chain-of-thought prompt builders
//! - Multi-turn conversation optimization
//! - Domain-specific prompt libraries expansion

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Dynamic prompt template with variable substitution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPromptTemplate {
    pub id: String,
    pub name: String,
    pub template: String,
    pub required_variables: Vec<String>,
    pub optional_variables: Vec<String>,
    pub domain: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl DynamicPromptTemplate {
    /// Create a new dynamic prompt template
    pub fn new(
        id: String,
        name: String,
        template: String,
        required_variables: Vec<String>,
        domain: String,
    ) -> Self {
        Self {
            id,
            name,
            template,
            required_variables,
            optional_variables: Vec::new(),
            domain,
            metadata: HashMap::new(),
        }
    }

    /// Add optional variable
    pub fn with_optional_variable(mut self, variable: String) -> Self {
        self.optional_variables.push(variable);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Assemble prompt from template with variables
    pub fn assemble(&self, variables: &HashMap<String, String>) -> Result<String> {
        // Check required variables
        for var in &self.required_variables {
            if !variables.contains_key(var) {
                anyhow::bail!("Missing required variable: {}", var);
            }
        }

        let mut result = self.template.clone();

        // Substitute variables
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        Ok(result)
    }
}

/// Context-aware prompt selector
pub struct ContextAwareSelector {
    templates: Arc<RwLock<HashMap<String, DynamicPromptTemplate>>>,
    selection_history: Arc<RwLock<Vec<SelectionRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SelectionRecord {
    template_id: String,
    context: PromptContext,
    selected_at: chrono::DateTime<chrono::Utc>,
    performance_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptContext {
    pub domain: String,
    pub task_type: String,
    pub complexity: String,
    pub user_expertise: String,
    pub additional_context: HashMap<String, String>,
}

impl ContextAwareSelector {
    /// Create a new context-aware selector
    pub fn new() -> Self {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
            selection_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add template to selector
    pub async fn add_template(&self, template: DynamicPromptTemplate) -> Result<()> {
        let mut templates = self.templates.write().await;
        templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// Select best template for context
    pub async fn select_template(&self, context: &PromptContext) -> Result<DynamicPromptTemplate> {
        let templates = self.templates.read().await;

        // Filter by domain
        let candidates: Vec<&DynamicPromptTemplate> = templates
            .values()
            .filter(|t| t.domain == context.domain)
            .collect();

        if candidates.is_empty() {
            anyhow::bail!("No templates found for domain: {}", context.domain);
        }

        // Select best match (for now, just return the first one)
        // In a real implementation, this would use a more sophisticated scoring algorithm
        let selected = candidates[0].clone();

        // Record selection
        let mut history = self.selection_history.write().await;
        history.push(SelectionRecord {
            template_id: selected.id.clone(),
            context: context.clone(),
            selected_at: chrono::Utc::now(),
            performance_score: None,
        });

        info!(
            "Selected template: {} for domain: {}",
            selected.id, context.domain
        );
        Ok(selected)
    }

    /// Update performance score for last selection
    pub async fn update_performance(&self, score: f64) -> Result<()> {
        let mut history = self.selection_history.write().await;
        if let Some(last) = history.last_mut() {
            last.performance_score = Some(score);
            debug!("Updated performance score: {}", score);
        }
        Ok(())
    }

    /// Get selection statistics
    pub async fn get_statistics(&self) -> PromptSelectionStatistics {
        let history = self.selection_history.read().await;
        let total_selections = history.len();
        let scored_selections = history
            .iter()
            .filter(|r| r.performance_score.is_some())
            .count();

        let avg_score = if scored_selections > 0 {
            let sum: f64 = history.iter().filter_map(|r| r.performance_score).sum();
            sum / scored_selections as f64
        } else {
            0.0
        };

        PromptSelectionStatistics {
            total_selections,
            scored_selections,
            average_score: avg_score,
        }
    }
}

impl Default for ContextAwareSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSelectionStatistics {
    pub total_selections: usize,
    pub scored_selections: usize,
    pub average_score: f64,
}

/// Prompt performance analytics tracker
pub struct PromptPerformanceAnalytics {
    metrics: Arc<RwLock<HashMap<String, Vec<PerformanceMetric>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub prompt_id: String,
    pub latency_ms: u128,
    pub token_count: usize,
    pub quality_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl PromptPerformanceAnalytics {
    /// Create a new performance analytics tracker
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record a performance metric
    pub async fn record(&self, metric: PerformanceMetric) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        metrics
            .entry(metric.prompt_id.clone())
            .or_insert_with(Vec::new)
            .push(metric);
        Ok(())
    }

    /// Get analytics for a specific prompt
    pub async fn get_analytics(&self, prompt_id: &str) -> Result<PromptAnalytics> {
        let metrics = self.metrics.read().await;
        let prompt_metrics = metrics.get(prompt_id).context("Prompt not found")?;

        if prompt_metrics.is_empty() {
            anyhow::bail!("No metrics available for prompt: {}", prompt_id);
        }

        let total_count = prompt_metrics.len();
        let avg_latency =
            prompt_metrics.iter().map(|m| m.latency_ms).sum::<u128>() / total_count as u128;
        let avg_tokens = prompt_metrics.iter().map(|m| m.token_count).sum::<usize>() / total_count;
        let avg_quality =
            prompt_metrics.iter().map(|m| m.quality_score).sum::<f64>() / total_count as f64;

        Ok(PromptAnalytics {
            prompt_id: prompt_id.to_string(),
            total_invocations: total_count,
            average_latency_ms: avg_latency,
            average_tokens: avg_tokens,
            average_quality_score: avg_quality,
        })
    }

    /// Get all analytics
    pub async fn get_all_analytics(&self) -> HashMap<String, PromptAnalytics> {
        let metrics = self.metrics.read().await;
        let mut result = HashMap::new();

        for (prompt_id, _) in metrics.iter() {
            if let Ok(analytics) = self.get_analytics(prompt_id).await {
                result.insert(prompt_id.clone(), analytics);
            }
        }

        result
    }
}

impl Default for PromptPerformanceAnalytics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptAnalytics {
    pub prompt_id: String,
    pub total_invocations: usize,
    pub average_latency_ms: u128,
    pub average_tokens: usize,
    pub average_quality_score: f64,
}

/// Automatic prompt refiner based on feedback
pub struct PromptRefiner {
    refinement_history: Arc<RwLock<Vec<RefinementRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementRecord {
    pub original_prompt: String,
    pub refined_prompt: String,
    pub refinement_reason: String,
    pub feedback_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PromptRefiner {
    /// Create a new prompt refiner
    pub fn new() -> Self {
        Self {
            refinement_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Refine a prompt based on feedback
    pub async fn refine(&self, prompt: &str, feedback: &PromptFeedback) -> Result<String> {
        let refined = if feedback.clarity_score < 0.5 {
            self.improve_clarity(prompt)?
        } else if feedback.completeness_score < 0.5 {
            self.improve_completeness(prompt)?
        } else if feedback.specificity_score < 0.5 {
            self.improve_specificity(prompt)?
        } else {
            prompt.to_string()
        };

        // Record refinement
        let mut history = self.refinement_history.write().await;
        history.push(RefinementRecord {
            original_prompt: prompt.to_string(),
            refined_prompt: refined.clone(),
            refinement_reason: self.determine_reason(feedback),
            feedback_score: feedback.overall_score(),
            timestamp: chrono::Utc::now(),
        });

        Ok(refined)
    }

    fn improve_clarity(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "Please provide a clear and precise response to the following:\n\n{}",
            prompt
        ))
    }

    fn improve_completeness(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "{}\n\nPlease provide a comprehensive answer covering all relevant aspects.",
            prompt
        ))
    }

    fn improve_specificity(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "{}\n\nPlease be specific in your response with concrete examples and details.",
            prompt
        ))
    }

    fn determine_reason(&self, feedback: &PromptFeedback) -> String {
        if feedback.clarity_score < 0.5 {
            "Improved clarity".to_string()
        } else if feedback.completeness_score < 0.5 {
            "Improved completeness".to_string()
        } else if feedback.specificity_score < 0.5 {
            "Improved specificity".to_string()
        } else {
            "No refinement needed".to_string()
        }
    }

    /// Get refinement history
    pub async fn get_history(&self) -> Vec<RefinementRecord> {
        let history = self.refinement_history.read().await;
        history.clone()
    }
}

impl Default for PromptRefiner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptFeedback {
    pub clarity_score: f64,
    pub completeness_score: f64,
    pub specificity_score: f64,
}

impl PromptFeedback {
    pub fn overall_score(&self) -> f64 {
        (self.clarity_score + self.completeness_score + self.specificity_score) / 3.0
    }
}

/// Few-shot learning prompt generator
pub struct FewShotGenerator {
    examples: Arc<RwLock<HashMap<String, Vec<FewShotPromptExample>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotPromptExample {
    pub input: String,
    pub output: String,
    pub explanation: Option<String>,
}

impl FewShotGenerator {
    /// Create a new few-shot generator
    pub fn new() -> Self {
        Self {
            examples: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add example for a task
    pub async fn add_example(&self, task_id: &str, example: FewShotPromptExample) -> Result<()> {
        let mut examples = self.examples.write().await;
        examples
            .entry(task_id.to_string())
            .or_insert_with(Vec::new)
            .push(example);
        Ok(())
    }

    /// Generate few-shot prompt
    pub async fn generate_prompt(
        &self,
        task_id: &str,
        query: &str,
        num_shots: usize,
    ) -> Result<String> {
        let examples = self.examples.read().await;
        let task_examples = examples.get(task_id).context("Task not found")?;

        if task_examples.is_empty() {
            anyhow::bail!("No examples available for task: {}", task_id);
        }

        let mut prompt = String::from("Here are some examples:\n\n");

        let shots = std::cmp::min(num_shots, task_examples.len());
        for (i, example) in task_examples.iter().take(shots).enumerate() {
            prompt.push_str(&format!("Example {}:\n", i + 1));
            prompt.push_str(&format!("Input: {}\n", example.input));
            prompt.push_str(&format!("Output: {}\n", example.output));
            if let Some(ref explanation) = example.explanation {
                prompt.push_str(&format!("Explanation: {}\n", explanation));
            }
            prompt.push('\n');
        }

        prompt.push_str(&format!(
            "Now, please process the following:\nInput: {}\nOutput:",
            query
        ));

        Ok(prompt)
    }

    /// Get number of examples for a task
    pub async fn example_count(&self, task_id: &str) -> usize {
        let examples = self.examples.read().await;
        examples.get(task_id).map_or(0, |v| v.len())
    }
}

impl Default for FewShotGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Chain-of-thought prompt builder
pub struct ChainOfThoughtBuilder {
    steps: Vec<String>,
    reasoning_prefix: String,
}

impl ChainOfThoughtBuilder {
    /// Create a new chain-of-thought builder
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            reasoning_prefix: "Let's think step by step:".to_string(),
        }
    }

    /// Add a reasoning step
    pub fn add_step(mut self, step: String) -> Self {
        self.steps.push(step);
        self
    }

    /// Set custom reasoning prefix
    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.reasoning_prefix = prefix;
        self
    }

    /// Build the prompt
    pub fn build(&self, query: &str) -> String {
        let mut prompt = format!("{}\n\n", query);
        prompt.push_str(&format!("{}\n\n", self.reasoning_prefix));

        for (i, step) in self.steps.iter().enumerate() {
            prompt.push_str(&format!("{}. {}\n", i + 1, step));
        }

        prompt
    }
}

impl Default for ChainOfThoughtBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-turn conversation optimizer
pub struct ConversationOptimizer {
    max_context_length: usize,
    compression_threshold: f64,
}

impl ConversationOptimizer {
    /// Create a new conversation optimizer
    pub fn new(max_context_length: usize, compression_threshold: f64) -> Self {
        Self {
            max_context_length,
            compression_threshold,
        }
    }

    /// Optimize conversation history
    pub fn optimize(&self, history: &[ConversationTurn]) -> Vec<ConversationTurn> {
        let total_length: usize = history.iter().map(|t| t.content.len()).sum();

        if total_length <= self.max_context_length {
            return history.to_vec();
        }

        // Calculate compression ratio needed
        let ratio = self.max_context_length as f64 / total_length as f64;

        if ratio < self.compression_threshold {
            // Aggressive compression: keep only most recent turns
            let keep_count = (history.len() as f64 * ratio).ceil() as usize;
            history[history.len().saturating_sub(keep_count)..].to_vec()
        } else {
            // Moderate compression: summarize older turns
            self.summarize_history(history)
        }
    }

    fn summarize_history(&self, history: &[ConversationTurn]) -> Vec<ConversationTurn> {
        if history.len() <= 2 {
            return history.to_vec();
        }

        let mut result = Vec::new();

        // Keep first turn (usually contains important context)
        result.push(history[0].clone());

        // Summarize middle turns
        if history.len() > 3 {
            let middle_content = format!("[Summarized {} previous turns]", history.len() - 2);
            result.push(ConversationTurn {
                role: "system".to_string(),
                content: middle_content,
            });
        }

        // Keep last turn
        result.push(history[history.len() - 1].clone());

        result
    }

    /// Estimate token count
    pub fn estimate_tokens(&self, history: &[ConversationTurn]) -> usize {
        // Rough estimate: ~4 characters per token
        let total_chars: usize = history.iter().map(|t| t.content.len()).sum();
        total_chars / 4
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub role: String,
    pub content: String,
}

/// Domain-specific prompt library
pub struct DomainPromptLibrary {
    prompts: Arc<RwLock<HashMap<String, Vec<DynamicPromptTemplate>>>>,
}

impl DomainPromptLibrary {
    /// Create a new domain prompt library
    pub fn new() -> Self {
        Self {
            prompts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add prompt to library
    pub async fn add_prompt(&self, domain: &str, prompt: DynamicPromptTemplate) -> Result<()> {
        let mut prompts = self.prompts.write().await;
        prompts
            .entry(domain.to_string())
            .or_insert_with(Vec::new)
            .push(prompt);
        Ok(())
    }

    /// Get prompts for domain
    pub async fn get_prompts(&self, domain: &str) -> Vec<DynamicPromptTemplate> {
        let prompts = self.prompts.read().await;
        prompts.get(domain).cloned().unwrap_or_default()
    }

    /// Search prompts by name
    pub async fn search(&self, query: &str) -> Vec<DynamicPromptTemplate> {
        let prompts = self.prompts.read().await;
        let mut results = Vec::new();

        for templates in prompts.values() {
            for template in templates {
                if template.name.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(template.clone());
                }
            }
        }

        results
    }

    /// Get all domains
    pub async fn get_domains(&self) -> Vec<String> {
        let prompts = self.prompts.read().await;
        prompts.keys().cloned().collect()
    }

    /// Get library statistics
    pub async fn get_statistics(&self) -> LibraryStatistics {
        let prompts = self.prompts.read().await;
        let total_domains = prompts.len();
        let total_prompts: usize = prompts.values().map(|v| v.len()).sum();

        LibraryStatistics {
            total_domains,
            total_prompts,
        }
    }
}

impl Default for DomainPromptLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStatistics {
    pub total_domains: usize,
    pub total_prompts: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_prompt_template() {
        let template = DynamicPromptTemplate::new(
            "test-1".to_string(),
            "Test Template".to_string(),
            "Hello {{name}}, you are {{age}} years old.".to_string(),
            vec!["name".to_string(), "age".to_string()],
            "general".to_string(),
        );

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("age".to_string(), "30".to_string());

        let result = template.assemble(&vars).expect("Failed to assemble");
        assert_eq!(result, "Hello Alice, you are 30 years old.");
    }

    #[test]
    fn test_dynamic_prompt_template_missing_variable() {
        let template = DynamicPromptTemplate::new(
            "test-2".to_string(),
            "Test Template".to_string(),
            "Hello {{name}}".to_string(),
            vec!["name".to_string()],
            "general".to_string(),
        );

        let vars = HashMap::new();
        let result = template.assemble(&vars);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_context_aware_selector() {
        let selector = ContextAwareSelector::new();

        let template = DynamicPromptTemplate::new(
            "legal-1".to_string(),
            "Legal Analysis".to_string(),
            "Analyze the following legal case: {{case}}".to_string(),
            vec!["case".to_string()],
            "legal".to_string(),
        );

        selector
            .add_template(template)
            .await
            .expect("Failed to add template");

        let context = PromptContext {
            domain: "legal".to_string(),
            task_type: "analysis".to_string(),
            complexity: "high".to_string(),
            user_expertise: "expert".to_string(),
            additional_context: HashMap::new(),
        };

        let selected = selector
            .select_template(&context)
            .await
            .expect("Failed to select");
        assert_eq!(selected.id, "legal-1");

        selector
            .update_performance(0.85)
            .await
            .expect("Failed to update");
        let stats = selector.get_statistics().await;
        assert_eq!(stats.total_selections, 1);
        assert_eq!(stats.scored_selections, 1);
    }

    #[tokio::test]
    async fn test_prompt_performance_analytics() {
        let analytics = PromptPerformanceAnalytics::new();

        let metric1 = PerformanceMetric {
            prompt_id: "test-prompt".to_string(),
            latency_ms: 100,
            token_count: 50,
            quality_score: 0.8,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        let metric2 = PerformanceMetric {
            prompt_id: "test-prompt".to_string(),
            latency_ms: 200,
            token_count: 60,
            quality_score: 0.9,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        analytics.record(metric1).await.expect("Failed to record");
        analytics.record(metric2).await.expect("Failed to record");

        let result = analytics
            .get_analytics("test-prompt")
            .await
            .expect("Failed to get analytics");
        assert_eq!(result.total_invocations, 2);
        assert_eq!(result.average_latency_ms, 150);
        assert_eq!(result.average_tokens, 55);
        assert!((result.average_quality_score - 0.85).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_prompt_refiner() {
        let refiner = PromptRefiner::new();

        let feedback = PromptFeedback {
            clarity_score: 0.3,
            completeness_score: 0.8,
            specificity_score: 0.7,
        };

        let original = "What is the law?";
        let refined = refiner
            .refine(original, &feedback)
            .await
            .expect("Failed to refine");

        assert!(refined.contains("clear and precise"));
        assert!(refined != original);

        let history = refiner.get_history().await;
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn test_few_shot_generator() {
        let generator = FewShotGenerator::new();

        let example1 = FewShotPromptExample {
            input: "What is 2+2?".to_string(),
            output: "4".to_string(),
            explanation: Some("Addition of two and two".to_string()),
        };

        let example2 = FewShotPromptExample {
            input: "What is 3+3?".to_string(),
            output: "6".to_string(),
            explanation: None,
        };

        generator
            .add_example("math", example1)
            .await
            .expect("Failed to add");
        generator
            .add_example("math", example2)
            .await
            .expect("Failed to add");

        let prompt = generator
            .generate_prompt("math", "What is 5+5?", 2)
            .await
            .expect("Failed to generate");

        assert!(prompt.contains("Example 1"));
        assert!(prompt.contains("Example 2"));
        assert!(prompt.contains("What is 5+5?"));
        assert_eq!(generator.example_count("math").await, 2);
    }

    #[test]
    fn test_chain_of_thought_builder() {
        let builder = ChainOfThoughtBuilder::new()
            .add_step("Identify the key elements".to_string())
            .add_step("Analyze relationships".to_string())
            .add_step("Draw conclusions".to_string());

        let prompt = builder.build("Solve this problem");

        assert!(prompt.contains("Let's think step by step"));
        assert!(prompt.contains("1. Identify the key elements"));
        assert!(prompt.contains("2. Analyze relationships"));
        assert!(prompt.contains("3. Draw conclusions"));
    }

    #[test]
    fn test_conversation_optimizer() {
        let optimizer = ConversationOptimizer::new(100, 0.5);

        let turns = vec![
            ConversationTurn {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
            ConversationTurn {
                role: "assistant".to_string(),
                content: "Hi there!".to_string(),
            },
            ConversationTurn {
                role: "user".to_string(),
                content: "How are you?".to_string(),
            },
        ];

        let optimized = optimizer.optimize(&turns);
        assert!(!optimized.is_empty());

        let tokens = optimizer.estimate_tokens(&turns);
        assert!(tokens > 0);
    }

    #[tokio::test]
    async fn test_domain_prompt_library() {
        let library = DomainPromptLibrary::new();

        let template1 = DynamicPromptTemplate::new(
            "legal-1".to_string(),
            "Contract Review".to_string(),
            "Review this contract: {{contract}}".to_string(),
            vec!["contract".to_string()],
            "legal".to_string(),
        );

        let template2 = DynamicPromptTemplate::new(
            "legal-2".to_string(),
            "Case Analysis".to_string(),
            "Analyze this case: {{case}}".to_string(),
            vec!["case".to_string()],
            "legal".to_string(),
        );

        library
            .add_prompt("legal", template1)
            .await
            .expect("Failed to add");
        library
            .add_prompt("legal", template2)
            .await
            .expect("Failed to add");

        let prompts = library.get_prompts("legal").await;
        assert_eq!(prompts.len(), 2);

        let search_results = library.search("Contract").await;
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "Contract Review");

        let domains = library.get_domains().await;
        assert_eq!(domains.len(), 1);

        let stats = library.get_statistics().await;
        assert_eq!(stats.total_domains, 1);
        assert_eq!(stats.total_prompts, 2);
    }

    #[test]
    fn test_prompt_feedback_overall_score() {
        let feedback = PromptFeedback {
            clarity_score: 0.8,
            completeness_score: 0.9,
            specificity_score: 0.7,
        };

        let score = feedback.overall_score();
        assert!((score - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_conversation_optimizer_aggressive_compression() {
        let optimizer = ConversationOptimizer::new(50, 0.5);

        let turns: Vec<ConversationTurn> = (0..10)
            .map(|i| ConversationTurn {
                role: "user".to_string(),
                content: format!("This is a very long message number {}", i),
            })
            .collect();

        let optimized = optimizer.optimize(&turns);
        assert!(optimized.len() < turns.len());
    }
}
