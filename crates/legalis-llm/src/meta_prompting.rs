//! Meta-Prompting System
//!
//! Self-improving prompt generation where the LLM generates, evaluates, and optimizes prompts.
//! Enables automated prompt engineering and continuous improvement.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Meta-prompt for generating task-specific prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPrompt {
    /// Task description
    pub task: String,
    /// Domain context
    pub domain: String,
    /// Quality criteria
    pub criteria: Vec<QualityCriterion>,
    /// Examples (optional)
    pub examples: Vec<PromptExample>,
    /// Constraints
    pub constraints: Vec<String>,
}

impl MetaPrompt {
    /// Creates a new meta-prompt.
    pub fn new(task: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            task: task.into(),
            domain: domain.into(),
            criteria: Vec::new(),
            examples: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a quality criterion.
    pub fn with_criterion(mut self, criterion: QualityCriterion) -> Self {
        self.criteria.push(criterion);
        self
    }

    /// Adds an example.
    pub fn with_example(mut self, example: PromptExample) -> Self {
        self.examples.push(example);
        self
    }

    /// Adds a constraint.
    pub fn with_constraint(mut self, constraint: impl Into<String>) -> Self {
        self.constraints.push(constraint.into());
        self
    }

    /// Generates a meta-prompt string for the LLM.
    pub fn to_llm_prompt(&self) -> String {
        let mut prompt = String::new();

        prompt.push_str("Generate an effective prompt for the following task:\n\n");
        prompt.push_str(&format!("Task: {}\n", self.task));
        prompt.push_str(&format!("Domain: {}\n\n", self.domain));

        if !self.criteria.is_empty() {
            prompt.push_str("Quality Criteria:\n");
            for criterion in &self.criteria {
                prompt.push_str(&format!(
                    "- {}: {}\n",
                    criterion.name, criterion.description
                ));
            }
            prompt.push('\n');
        }

        if !self.examples.is_empty() {
            prompt.push_str("Examples:\n");
            for (i, example) in self.examples.iter().enumerate() {
                prompt.push_str(&format!("{}. Input: {}\n", i + 1, example.input));
                prompt.push_str(&format!("   Expected: {}\n", example.expected_output));
            }
            prompt.push('\n');
        }

        if !self.constraints.is_empty() {
            prompt.push_str("Constraints:\n");
            for constraint in &self.constraints {
                prompt.push_str(&format!("- {}\n", constraint));
            }
            prompt.push('\n');
        }

        prompt.push_str("Generate a clear, effective prompt that addresses the task, ");
        prompt.push_str("meets the quality criteria, and respects the constraints.\n\n");
        prompt.push_str("Prompt:");

        prompt
    }
}

/// Quality criterion for prompt evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCriterion {
    /// Criterion name
    pub name: String,
    /// Description
    pub description: String,
    /// Weight (0.0-1.0)
    pub weight: f64,
}

impl QualityCriterion {
    /// Creates a new quality criterion.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            weight: 1.0,
        }
    }

    /// Sets the weight.
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }
}

/// Example for prompt generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptExample {
    /// Example input
    pub input: String,
    /// Expected output
    pub expected_output: String,
}

impl PromptExample {
    /// Creates a new prompt example.
    pub fn new(input: impl Into<String>, expected_output: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            expected_output: expected_output.into(),
        }
    }
}

/// Generated prompt with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPrompt {
    /// The generated prompt text
    pub prompt: String,
    /// Quality score (0.0-1.0)
    pub quality_score: f64,
    /// Generation metadata
    pub metadata: HashMap<String, String>,
    /// Evaluation metrics
    pub metrics: PromptMetrics,
}

impl GeneratedPrompt {
    /// Creates a new generated prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            quality_score: 0.0,
            metadata: HashMap::new(),
            metrics: PromptMetrics::default(),
        }
    }

    /// Sets the quality score.
    pub fn with_quality_score(mut self, score: f64) -> Self {
        self.quality_score = score.clamp(0.0, 1.0);
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Sets the metrics.
    pub fn with_metrics(mut self, metrics: PromptMetrics) -> Self {
        self.metrics = metrics;
        self
    }
}

/// Metrics for prompt evaluation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromptMetrics {
    /// Clarity score (0.0-1.0)
    pub clarity: f64,
    /// Specificity score (0.0-1.0)
    pub specificity: f64,
    /// Completeness score (0.0-1.0)
    pub completeness: f64,
    /// Conciseness score (0.0-1.0)
    pub conciseness: f64,
    /// Effectiveness score (0.0-1.0)
    pub effectiveness: f64,
}

impl PromptMetrics {
    /// Calculates the overall score.
    pub fn overall_score(&self) -> f64 {
        (self.clarity
            + self.specificity
            + self.completeness
            + self.conciseness
            + self.effectiveness)
            / 5.0
    }
}

/// Meta-prompting engine for automated prompt generation
pub struct MetaPromptingEngine {
    /// History of generated prompts
    history: Vec<GeneratedPrompt>,
    /// Best performing prompts by task
    best_prompts: HashMap<String, GeneratedPrompt>,
}

impl MetaPromptingEngine {
    /// Creates a new meta-prompting engine.
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            best_prompts: HashMap::new(),
        }
    }

    /// Evaluates a generated prompt.
    pub fn evaluate_prompt(&self, prompt: &str) -> PromptMetrics {
        let mut metrics = PromptMetrics::default();

        // Clarity: check for clear instructions
        metrics.clarity = self.evaluate_clarity(prompt);

        // Specificity: check for specific details
        metrics.specificity = self.evaluate_specificity(prompt);

        // Completeness: check for necessary components
        metrics.completeness = self.evaluate_completeness(prompt);

        // Conciseness: check length and redundancy
        metrics.conciseness = self.evaluate_conciseness(prompt);

        // Effectiveness: heuristic based on structure
        metrics.effectiveness = self.evaluate_effectiveness(prompt);

        metrics
    }

    fn evaluate_clarity(&self, prompt: &str) -> f64 {
        let mut score: f64 = 0.5;

        // Has clear imperative verbs
        let imperative_verbs = [
            "analyze",
            "explain",
            "describe",
            "identify",
            "determine",
            "evaluate",
        ];
        if imperative_verbs
            .iter()
            .any(|v| prompt.to_lowercase().contains(v))
        {
            score += 0.2;
        }

        // Avoids ambiguous language
        let ambiguous = ["maybe", "perhaps", "possibly", "might"];
        if !ambiguous.iter().any(|a| prompt.to_lowercase().contains(a)) {
            score += 0.2;
        }

        // Has clear structure
        if prompt.contains(':') || prompt.contains('\n') {
            score += 0.1;
        }

        score.min(1.0)
    }

    fn evaluate_specificity(&self, prompt: &str) -> f64 {
        let mut score: f64 = 0.3;

        // Has specific domain terms
        let word_count = prompt.split_whitespace().count();
        if word_count > 10 {
            score += 0.2;
        }

        // Has examples or constraints
        if prompt.to_lowercase().contains("example") || prompt.to_lowercase().contains("constraint")
        {
            score += 0.3;
        }

        // Has numbers or specific criteria
        if prompt.chars().any(|c| c.is_numeric()) {
            score += 0.2;
        }

        score.min(1.0)
    }

    fn evaluate_completeness(&self, prompt: &str) -> f64 {
        let mut score: f64 = 0.4;

        // Has task description
        if prompt.len() > 20 {
            score += 0.2;
        }

        // Has context
        if prompt.to_lowercase().contains("context") || prompt.split('\n').count() > 1 {
            score += 0.2;
        }

        // Has output format specification
        if prompt.to_lowercase().contains("format") || prompt.to_lowercase().contains("output") {
            score += 0.2;
        }

        score.min(1.0)
    }

    fn evaluate_conciseness(&self, prompt: &str) -> f64 {
        let word_count = prompt.split_whitespace().count();

        if word_count > 500 {
            0.3 // Too long
        } else if word_count > 200 {
            0.6 // Long but acceptable
        } else if word_count > 50 {
            1.0 // Good length
        } else if word_count > 10 {
            0.8 // Acceptable
        } else {
            0.4 // Too short
        }
    }

    fn evaluate_effectiveness(&self, prompt: &str) -> f64 {
        let mut score: f64 = 0.5;

        // Has call to action
        if prompt.ends_with('?') || prompt.ends_with('.') {
            score += 0.2;
        }

        // Has structured sections
        let section_markers = ['\n', ':', '-', '•'];
        if section_markers.iter().any(|m| prompt.contains(*m)) {
            score += 0.2;
        }

        // Avoids negative constructions
        let negatives = ["don't", "do not", "avoid", "never"];
        let negative_count = negatives
            .iter()
            .filter(|n| prompt.to_lowercase().contains(*n))
            .count();
        if negative_count == 0 {
            score += 0.1;
        }

        score.min(1.0)
    }

    /// Registers a generated prompt with its performance.
    pub fn register_prompt(&mut self, task: impl Into<String>, prompt: GeneratedPrompt) {
        let task_key = task.into();

        // Update best prompt if this one is better
        let should_update = self
            .best_prompts
            .get(&task_key)
            .map(|best| prompt.quality_score > best.quality_score)
            .unwrap_or(true);

        if should_update {
            self.best_prompts.insert(task_key.clone(), prompt.clone());
        }

        self.history.push(prompt);
    }

    /// Gets the best prompt for a task.
    pub fn get_best_prompt(&self, task: &str) -> Option<&GeneratedPrompt> {
        self.best_prompts.get(task)
    }

    /// Gets improvement suggestions for a prompt.
    pub fn suggest_improvements(&self, prompt: &str) -> Vec<String> {
        let metrics = self.evaluate_prompt(prompt);
        let mut suggestions = Vec::new();

        if metrics.clarity < 0.7 {
            suggestions.push("Add clear, imperative instructions".to_string());
            suggestions.push("Remove ambiguous language".to_string());
        }

        if metrics.specificity < 0.7 {
            suggestions.push("Include specific domain terminology".to_string());
            suggestions.push("Add concrete examples".to_string());
        }

        if metrics.completeness < 0.7 {
            suggestions.push("Specify the desired output format".to_string());
            suggestions.push("Add necessary context".to_string());
        }

        if metrics.conciseness < 0.7 {
            let word_count = prompt.split_whitespace().count();
            if word_count > 300 {
                suggestions.push("Reduce verbosity and redundancy".to_string());
            } else {
                suggestions.push("Add more detail to the task description".to_string());
            }
        }

        if metrics.effectiveness < 0.7 {
            suggestions.push("Structure the prompt with clear sections".to_string());
            suggestions.push("Frame instructions positively".to_string());
        }

        suggestions
    }

    /// Gets statistics about prompt generation.
    pub fn statistics(&self) -> MetaPromptingStatistics {
        let total_prompts = self.history.len();
        let avg_quality = if total_prompts > 0 {
            self.history.iter().map(|p| p.quality_score).sum::<f64>() / total_prompts as f64
        } else {
            0.0
        };

        let best_tasks = self.best_prompts.len();

        MetaPromptingStatistics {
            total_prompts,
            avg_quality,
            best_tasks,
        }
    }
}

impl Default for MetaPromptingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about meta-prompting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPromptingStatistics {
    /// Total number of prompts generated
    pub total_prompts: usize,
    /// Average quality score
    pub avg_quality: f64,
    /// Number of tasks with best prompts
    pub best_tasks: usize,
}

/// Legal-specific meta-prompting
pub struct LegalMetaPrompting;

impl LegalMetaPrompting {
    /// Creates a meta-prompt for legal document analysis.
    pub fn document_analysis_meta_prompt(document_type: &str) -> MetaPrompt {
        MetaPrompt::new(
            format!("Analyze {} documents", document_type),
            "Legal Document Analysis",
        )
        .with_criterion(QualityCriterion::new(
            "Accuracy",
            "Extract accurate legal information",
        ))
        .with_criterion(QualityCriterion::new(
            "Completeness",
            "Identify all relevant clauses and provisions",
        ))
        .with_criterion(QualityCriterion::new(
            "Structure",
            "Present analysis in a structured format",
        ))
        .with_constraint("Must include citations to specific clauses")
        .with_constraint("Must identify potential legal issues")
    }

    /// Creates a meta-prompt for contract drafting.
    pub fn contract_drafting_meta_prompt(contract_type: &str) -> MetaPrompt {
        MetaPrompt::new(
            format!("Draft {} contracts", contract_type),
            "Contract Drafting",
        )
        .with_criterion(QualityCriterion::new(
            "Legal Soundness",
            "Include necessary legal provisions",
        ))
        .with_criterion(QualityCriterion::new(
            "Clarity",
            "Use clear, unambiguous language",
        ))
        .with_criterion(QualityCriterion::new(
            "Enforceability",
            "Ensure provisions are legally enforceable",
        ))
        .with_constraint("Must include standard clauses for this contract type")
        .with_constraint("Must be jurisdiction-appropriate")
    }

    /// Creates a meta-prompt for legal research.
    pub fn legal_research_meta_prompt(topic: &str) -> MetaPrompt {
        MetaPrompt::new(format!("Research {}", topic), "Legal Research")
            .with_criterion(QualityCriterion::new(
                "Thoroughness",
                "Cover all relevant legal authorities",
            ))
            .with_criterion(QualityCriterion::new(
                "Citation",
                "Properly cite all sources",
            ))
            .with_criterion(QualityCriterion::new(
                "Analysis",
                "Provide analytical synthesis",
            ))
            .with_constraint("Must include primary sources (statutes, case law)")
            .with_constraint("Must be jurisdiction-specific")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_prompt_creation() {
        let meta_prompt = MetaPrompt::new("Analyze contracts", "Legal")
            .with_criterion(QualityCriterion::new("Accuracy", "Be accurate"))
            .with_example(PromptExample::new("Contract A", "Analysis A"))
            .with_constraint("Must be concise");

        assert_eq!(meta_prompt.task, "Analyze contracts");
        assert_eq!(meta_prompt.domain, "Legal");
        assert_eq!(meta_prompt.criteria.len(), 1);
        assert_eq!(meta_prompt.examples.len(), 1);
        assert_eq!(meta_prompt.constraints.len(), 1);
    }

    #[test]
    fn test_meta_prompt_to_llm_prompt() {
        let meta_prompt = MetaPrompt::new("Summarize case law", "Legal")
            .with_criterion(QualityCriterion::new("Brevity", "Be concise"));

        let llm_prompt = meta_prompt.to_llm_prompt();
        assert!(llm_prompt.contains("Summarize case law"));
        assert!(llm_prompt.contains("Legal"));
        assert!(llm_prompt.contains("Brevity"));
    }

    #[test]
    fn test_quality_criterion() {
        let criterion = QualityCriterion::new("Clarity", "Clear instructions").with_weight(0.8);

        assert_eq!(criterion.name, "Clarity");
        assert!((criterion.weight - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_prompt_example() {
        let example = PromptExample::new("Input text", "Expected output");

        assert_eq!(example.input, "Input text");
        assert_eq!(example.expected_output, "Expected output");
    }

    #[test]
    fn test_generated_prompt() {
        let prompt = GeneratedPrompt::new("Analyze this contract")
            .with_quality_score(0.85)
            .with_metadata("version", "1.0");

        assert_eq!(prompt.prompt, "Analyze this contract");
        assert!((prompt.quality_score - 0.85).abs() < f64::EPSILON);
        assert_eq!(prompt.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_prompt_metrics() {
        let metrics = PromptMetrics {
            clarity: 0.9,
            specificity: 0.8,
            completeness: 0.85,
            conciseness: 0.75,
            effectiveness: 0.8,
        };

        let overall = metrics.overall_score();
        assert!((overall - 0.82).abs() < 0.01);
    }

    #[test]
    fn test_meta_prompting_engine() {
        let mut engine = MetaPromptingEngine::new();

        let prompt1 = GeneratedPrompt::new("Prompt 1").with_quality_score(0.7);
        let prompt2 = GeneratedPrompt::new("Prompt 2").with_quality_score(0.9);

        engine.register_prompt("task1", prompt1);
        engine.register_prompt("task1", prompt2.clone());

        let best = engine.get_best_prompt("task1").unwrap();
        assert_eq!(best.prompt, "Prompt 2");
        assert!((best.quality_score - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn test_prompt_evaluation() {
        let engine = MetaPromptingEngine::new();

        let good_prompt = "Analyze the contract carefully and identify all key clauses including specific terms like consideration, parties, and obligations. \
                          Provide a structured summary with citations and examples.";
        let metrics = engine.evaluate_prompt(good_prompt);

        assert!(metrics.clarity > 0.5);
        assert!(metrics.specificity > 0.3); // Lower threshold for specificity
        assert!(metrics.overall_score() > 0.5);
    }

    #[test]
    fn test_improvement_suggestions() {
        let engine = MetaPromptingEngine::new();

        let weak_prompt = "Do something";
        let suggestions = engine.suggest_improvements(weak_prompt);

        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_legal_meta_prompting() {
        let meta_prompt = LegalMetaPrompting::document_analysis_meta_prompt("employment");

        assert!(meta_prompt.task.contains("employment"));
        assert_eq!(meta_prompt.domain, "Legal Document Analysis");
        assert!(!meta_prompt.criteria.is_empty());
        assert!(!meta_prompt.constraints.is_empty());
    }

    #[test]
    fn test_contract_drafting_meta_prompt() {
        let meta_prompt = LegalMetaPrompting::contract_drafting_meta_prompt("NDA");

        assert!(meta_prompt.task.contains("NDA"));
        assert!(
            meta_prompt
                .criteria
                .iter()
                .any(|c| c.name == "Legal Soundness")
        );
    }

    #[test]
    fn test_statistics() {
        let mut engine = MetaPromptingEngine::new();

        engine.register_prompt("task1", GeneratedPrompt::new("P1").with_quality_score(0.8));
        engine.register_prompt("task2", GeneratedPrompt::new("P2").with_quality_score(0.9));

        let stats = engine.statistics();
        assert_eq!(stats.total_prompts, 2);
        assert!((stats.avg_quality - 0.85).abs() < 0.01);
        assert_eq!(stats.best_tasks, 2);
    }
}
