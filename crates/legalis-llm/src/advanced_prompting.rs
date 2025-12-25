//! Advanced prompting techniques for improved LLM reasoning.
//!
//! This module provides implementations of chain-of-thought prompting,
//! self-consistency decoding, and other advanced prompting strategies.

use crate::{LLMProvider, Result};
use std::collections::HashMap;

/// Chain-of-thought prompting builder.
///
/// Encourages the model to show its reasoning step-by-step.
pub struct ChainOfThought {
    prompt: String,
    use_few_shot: bool,
    examples: Vec<(String, String)>, // (question, reasoning + answer)
}

impl ChainOfThought {
    /// Creates a new chain-of-thought prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            use_few_shot: false,
            examples: Vec::new(),
        }
    }

    /// Enables few-shot learning with examples.
    pub fn with_examples(mut self, examples: Vec<(String, String)>) -> Self {
        self.use_few_shot = !examples.is_empty();
        self.examples = examples;
        self
    }

    /// Adds a single example.
    pub fn add_example(
        mut self,
        question: impl Into<String>,
        reasoning: impl Into<String>,
    ) -> Self {
        self.examples.push((question.into(), reasoning.into()));
        self.use_few_shot = true;
        self
    }

    /// Builds the final prompt with chain-of-thought instructions.
    pub fn build(&self) -> String {
        let mut prompt = String::new();

        if self.use_few_shot && !self.examples.is_empty() {
            prompt.push_str("Here are some examples of step-by-step reasoning:\n\n");

            for (i, (question, reasoning)) in self.examples.iter().enumerate() {
                prompt.push_str(&format!("Example {}:\n", i + 1));
                prompt.push_str(&format!("Question: {}\n", question));
                prompt.push_str(&format!("Answer: {}\n\n", reasoning));
            }
        }

        prompt.push_str("Now, solve this problem step by step:\n");
        prompt.push_str(&format!("Question: {}\n", self.prompt));
        prompt.push_str("Answer: Let's think step by step.\n");

        prompt
    }
}

/// Self-consistency decoding implementation.
///
/// Generates multiple reasoning paths and selects the most consistent answer.
pub struct SelfConsistency<P: LLMProvider> {
    provider: P,
    num_samples: usize,
    temperature: f32,
}

impl<P: LLMProvider> SelfConsistency<P> {
    /// Creates a new self-consistency decoder.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            num_samples: 5,
            temperature: 0.7,
        }
    }

    /// Sets the number of samples to generate.
    pub fn with_samples(mut self, num_samples: usize) -> Self {
        self.num_samples = num_samples;
        self
    }

    /// Sets the temperature for sampling.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    /// Generates multiple answers and returns the most common one.
    pub async fn generate(&self, prompt: &str) -> Result<SelfConsistencyResult> {
        let mut answers = Vec::new();

        for _ in 0..self.num_samples {
            let response = self.provider.generate_text(prompt).await?;
            let answer = extract_final_answer(&response);
            answers.push(answer);
        }

        let result = Self::select_most_consistent(&answers);
        Ok(result)
    }

    /// Selects the most consistent answer from multiple samples.
    fn select_most_consistent(answers: &[String]) -> SelfConsistencyResult {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for answer in answers {
            *counts.entry(answer.clone()).or_insert(0) += 1;
        }

        let (most_common, count) = counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .unwrap_or_else(|| (String::new(), 0));

        let confidence = count as f64 / answers.len() as f64;

        SelfConsistencyResult {
            answer: most_common,
            confidence,
            all_answers: answers.to_vec(),
        }
    }
}

/// Result from self-consistency decoding.
#[derive(Debug, Clone)]
pub struct SelfConsistencyResult {
    /// The most consistent answer
    pub answer: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// All generated answers
    pub all_answers: Vec<String>,
}

/// Extracts the final answer from a reasoning chain.
fn extract_final_answer(response: &str) -> String {
    // Look for common answer indicators
    let markers = [
        "Therefore,",
        "So,",
        "Thus,",
        "The answer is",
        "Final answer:",
    ];

    for marker in &markers {
        if let Some(pos) = response.rfind(marker) {
            let answer = &response[pos + marker.len()..];
            return answer.trim().to_string();
        }
    }

    // If no marker found, use the last sentence
    response
        .split('.')
        .last()
        .unwrap_or(response)
        .trim()
        .to_string()
}

/// Tree-of-thought prompting builder.
///
/// Explores multiple reasoning paths like a search tree.
pub struct TreeOfThought {
    prompt: String,
    branches_per_step: usize,
    max_depth: usize,
}

impl TreeOfThought {
    /// Creates a new tree-of-thought prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            branches_per_step: 3,
            max_depth: 3,
        }
    }

    /// Sets the number of branches to explore at each step.
    pub fn with_branches(mut self, branches: usize) -> Self {
        self.branches_per_step = branches;
        self
    }

    /// Sets the maximum depth of the reasoning tree.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Builds the tree-of-thought prompt.
    pub fn build(&self) -> String {
        format!(
            "Explore multiple reasoning paths for this problem:\n\n\
             Problem: {}\n\n\
             Generate {} different approaches to solve this problem.\n\
             For each approach, think through {} steps of reasoning.\n\
             Evaluate which approach is most promising and provide the final answer.",
            self.prompt, self.branches_per_step, self.max_depth
        )
    }
}

/// Constitutional AI prompting helpers.
///
/// Adds ethical and safety guidelines to prompts.
pub struct ConstitutionalPrompt {
    prompt: String,
    principles: Vec<String>,
}

impl ConstitutionalPrompt {
    /// Creates a new constitutional prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            principles: Vec::new(),
        }
    }

    /// Adds a principle to guide the response.
    pub fn add_principle(mut self, principle: impl Into<String>) -> Self {
        self.principles.push(principle.into());
        self
    }

    /// Adds default ethical principles.
    pub fn with_default_principles(mut self) -> Self {
        self.principles.extend([
            "Be helpful and harmless".to_string(),
            "Respect privacy and confidentiality".to_string(),
            "Avoid bias and discrimination".to_string(),
            "Provide accurate and truthful information".to_string(),
        ]);
        self
    }

    /// Builds the prompt with constitutional guidelines.
    pub fn build(&self) -> String {
        let mut prompt = String::new();

        if !self.principles.is_empty() {
            prompt.push_str("Please follow these principles in your response:\n");
            for (i, principle) in self.principles.iter().enumerate() {
                prompt.push_str(&format!("{}. {}\n", i + 1, principle));
            }
            prompt.push('\n');
        }

        prompt.push_str(&self.prompt);
        prompt
    }
}

/// ReAct (Reasoning + Acting) pattern builder.
///
/// Alternates between reasoning and action steps.
pub struct ReActPrompt {
    task: String,
    available_actions: Vec<String>,
}

impl ReActPrompt {
    /// Creates a new ReAct prompt.
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            task: task.into(),
            available_actions: Vec::new(),
        }
    }

    /// Adds an available action.
    pub fn add_action(mut self, action: impl Into<String>) -> Self {
        self.available_actions.push(action.into());
        self
    }

    /// Builds the ReAct prompt.
    pub fn build(&self) -> String {
        let mut prompt = String::new();

        prompt.push_str("You will alternate between Thought, Action, and Observation.\n\n");

        if !self.available_actions.is_empty() {
            prompt.push_str("Available actions:\n");
            for action in &self.available_actions {
                prompt.push_str(&format!("- {}\n", action));
            }
            prompt.push('\n');
        }

        prompt.push_str(&format!("Task: {}\n\n", self.task));
        prompt.push_str("Thought 1:");

        prompt
    }
}

/// Prompt compression helper.
///
/// Reduces prompt length while preserving meaning.
pub fn compress_prompt(prompt: &str, max_length: usize) -> String {
    if prompt.len() <= max_length {
        return prompt.to_string();
    }

    // Simple compression: keep first and last parts, summarize middle
    let keep_start = max_length / 3;
    let keep_end = max_length / 3;

    let start = &prompt[..keep_start.min(prompt.len())];
    let end_start = prompt.len().saturating_sub(keep_end);
    let end = &prompt[end_start..];

    format!("{}... [content summarized] ...{}", start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_of_thought_basic() {
        let cot = ChainOfThought::new("What is 2 + 2?");
        let prompt = cot.build();

        assert!(prompt.contains("step by step"));
        assert!(prompt.contains("What is 2 + 2?"));
    }

    #[test]
    fn test_chain_of_thought_with_examples() {
        let cot = ChainOfThought::new("What is 5 + 3?").add_example(
            "What is 2 + 2?",
            "First, I start with 2. Then I add 2 more. That gives me 4.",
        );

        let prompt = cot.build();

        assert!(prompt.contains("Example 1"));
        assert!(prompt.contains("What is 2 + 2?"));
        assert!(prompt.contains("What is 5 + 3?"));
    }

    #[test]
    fn test_extract_final_answer() {
        let response = "Let me think. First we add 2 and 2. Therefore, the answer is 4.";
        let answer = extract_final_answer(response);
        assert!(answer.contains("4"));
    }

    #[test]
    fn test_tree_of_thought_build() {
        let tot = TreeOfThought::new("Solve the puzzle")
            .with_branches(4)
            .with_max_depth(5);

        let prompt = tot.build();

        assert!(prompt.contains("4 different approaches"));
        assert!(prompt.contains("5 steps"));
    }

    #[test]
    fn test_constitutional_prompt() {
        let prompt = ConstitutionalPrompt::new("Give me advice")
            .add_principle("Be honest")
            .add_principle("Be respectful")
            .build();

        assert!(prompt.contains("Be honest"));
        assert!(prompt.contains("Be respectful"));
        assert!(prompt.contains("Give me advice"));
    }

    #[test]
    fn test_constitutional_prompt_defaults() {
        let prompt = ConstitutionalPrompt::new("Help me")
            .with_default_principles()
            .build();

        assert!(prompt.contains("helpful and harmless"));
        assert!(prompt.contains("privacy"));
    }

    #[test]
    fn test_react_prompt() {
        let prompt = ReActPrompt::new("Find information about AI")
            .add_action("Search")
            .add_action("Read")
            .build();

        assert!(prompt.contains("Thought, Action, and Observation"));
        assert!(prompt.contains("Search"));
        assert!(prompt.contains("Read"));
        assert!(prompt.contains("Find information about AI"));
    }

    #[test]
    fn test_compress_prompt() {
        let long_prompt = "a".repeat(1000);
        let compressed = compress_prompt(&long_prompt, 100);

        assert!(compressed.len() <= 150); // Some overhead for ellipsis
        assert!(compressed.contains("..."));
    }

    #[test]
    fn test_compress_prompt_no_compression_needed() {
        let short_prompt = "Short prompt";
        let compressed = compress_prompt(short_prompt, 100);

        assert_eq!(compressed, short_prompt);
    }

    #[test]
    fn test_self_consistency_select() {
        use crate::testing::FixtureProvider;

        let answers = vec![
            "42".to_string(),
            "42".to_string(),
            "43".to_string(),
            "42".to_string(),
        ];

        let result = SelfConsistency::<FixtureProvider>::select_most_consistent(&answers);

        assert_eq!(result.answer, "42");
        assert_eq!(result.confidence, 0.75); // 3 out of 4
        assert_eq!(result.all_answers.len(), 4);
    }
}
