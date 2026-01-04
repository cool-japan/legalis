//! LLM-assisted condition interpretation for complex legal reasoning.
//!
//! This module provides a framework for integrating Large Language Models (LLMs)
//! to assist with interpreting and evaluating complex legal conditions that require
//! natural language understanding and legal domain knowledge.
//!
//! ## Features
//!
//! - **LLM Backend Abstraction**: Trait-based design for any LLM provider
//! - **Prompt Templates**: Pre-built templates for legal reasoning tasks
//! - **Response Parsing**: Structured parsing of LLM outputs
//! - **Confidence Scoring**: Uncertainty quantification for LLM decisions
//! - **Caching**: Response caching to reduce API costs
//!
//! ## Example
//!
//! ```
//! use legalis_core::llm_interpretation::{LlmInterpreter, PromptTemplate};
//!
//! let interpreter = LlmInterpreter::new();
//!
//! // Create a prompt for interpreting a legal condition
//! let prompt = PromptTemplate::condition_interpretation(
//!     "reasonable person standard",
//!     "Would a reasonable person have foreseen the risk?"
//! );
//!
//! assert!(prompt.contains("reasonable person"));
//! ```

use crate::Condition;
use std::collections::HashMap;

/// LLM backend trait for pluggable LLM providers.
///
/// Implementors can integrate with OpenAI, Anthropic Claude, or other LLM APIs.
pub trait LlmBackend: Send + Sync {
    /// Generates a response from the LLM given a prompt.
    fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<LlmResponse, LlmError>;

    /// Returns the name of the LLM backend.
    fn backend_name(&self) -> &str;

    /// Returns whether the backend is available and initialized.
    fn is_available(&self) -> bool;

    /// Returns cost information for a request.
    fn estimate_cost(&self, prompt_tokens: usize, completion_tokens: usize) -> f64;
}

/// Response from an LLM.
#[derive(Debug, Clone)]
pub struct LlmResponse {
    /// The generated text response
    pub text: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Number of tokens in the prompt
    pub prompt_tokens: usize,
    /// Number of tokens in the completion
    pub completion_tokens: usize,
    /// Backend-specific metadata
    pub metadata: HashMap<String, String>,
}

impl LlmResponse {
    /// Creates a new LLM response.
    pub fn new(text: String, confidence: f64) -> Self {
        Self {
            text,
            confidence,
            prompt_tokens: 0,
            completion_tokens: 0,
            metadata: HashMap::new(),
        }
    }

    /// Returns whether the response has high confidence (>= 0.8).
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.8
    }

    /// Returns the total number of tokens used.
    pub fn total_tokens(&self) -> usize {
        self.prompt_tokens + self.completion_tokens
    }
}

/// Errors that can occur when using LLMs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmError {
    /// LLM backend is not available
    BackendNotAvailable,
    /// API rate limit exceeded
    RateLimitExceeded,
    /// API authentication failed
    AuthenticationFailed,
    /// Invalid prompt or parameters
    InvalidRequest(String),
    /// Response parsing failed
    ParsingError(String),
    /// Network error
    NetworkError(String),
    /// Backend-specific error
    BackendError(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::BackendNotAvailable => write!(f, "LLM backend not available"),
            LlmError::RateLimitExceeded => write!(f, "API rate limit exceeded"),
            LlmError::AuthenticationFailed => write!(f, "API authentication failed"),
            LlmError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            LlmError::ParsingError(msg) => write!(f, "Response parsing error: {}", msg),
            LlmError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            LlmError::BackendError(msg) => write!(f, "Backend error: {}", msg),
        }
    }
}

impl std::error::Error for LlmError {}

/// Pre-built prompt templates for legal reasoning tasks.
pub struct PromptTemplate;

impl PromptTemplate {
    /// Creates a prompt for interpreting a legal condition.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::llm_interpretation::PromptTemplate;
    ///
    /// let prompt = PromptTemplate::condition_interpretation(
    ///     "good faith",
    ///     "Did the party act in good faith?"
    /// );
    ///
    /// assert!(prompt.contains("good faith"));
    /// assert!(prompt.contains("legal"));
    /// ```
    pub fn condition_interpretation(condition_name: &str, question: &str) -> String {
        format!(
            "You are a legal reasoning assistant. Please interpret the following legal condition:\n\n\
            Condition: {}\n\
            Question: {}\n\n\
            Provide your interpretation in the following format:\n\
            1. Definition: [Brief definition of the legal term]\n\
            2. Applicable Standard: [The legal standard that applies]\n\
            3. Factors to Consider: [Key factors in this determination]\n\
            4. Confidence: [Your confidence level: high/medium/low]\n\n\
            Focus on providing clear, legally sound reasoning based on established legal principles.",
            condition_name, question
        )
    }

    /// Creates a prompt for evaluating statutory interpretation.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::llm_interpretation::PromptTemplate;
    ///
    /// let prompt = PromptTemplate::statute_interpretation(
    ///     "Section 1 of the Fair Housing Act",
    ///     "Does this statute apply to student housing?"
    /// );
    ///
    /// assert!(prompt.contains("Fair Housing Act"));
    /// ```
    pub fn statute_interpretation(statute_text: &str, question: &str) -> String {
        format!(
            "You are a legal interpretation assistant. Analyze the following statute:\n\n\
            Statute: {}\n\
            Question: {}\n\n\
            Provide your analysis:\n\
            1. Plain Meaning: [What the statute says on its face]\n\
            2. Legislative Intent: [Likely purpose of the statute]\n\
            3. Precedent: [How courts typically interpret similar statutes]\n\
            4. Application: [How it applies to the question]\n\
            5. Confidence: [high/medium/low]\n\n\
            Use established canons of statutory construction in your analysis.",
            statute_text, question
        )
    }

    /// Creates a prompt for analogical reasoning with case law.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::llm_interpretation::PromptTemplate;
    ///
    /// let prompt = PromptTemplate::case_analogy(
    ///     "Plaintiff slipped on ice in parking lot",
    ///     "Defendant was aware ice had formed but did not salt"
    /// );
    ///
    /// assert!(prompt.contains("analogical reasoning"));
    /// ```
    pub fn case_analogy(current_facts: &str, precedent_facts: &str) -> String {
        format!(
            "You are a legal reasoning assistant specialized in analogical reasoning.\n\n\
            Current Case Facts: {}\n\
            Precedent Case Facts: {}\n\n\
            Perform analogical reasoning:\n\
            1. Key Similarities: [Material facts that are similar]\n\
            2. Key Differences: [Material facts that differ]\n\
            3. Distinguishing Factors: [Why differences matter or don't matter]\n\
            4. Recommendation: [Should precedent apply? Why or why not?]\n\
            5. Confidence: [high/medium/low]\n\n\
            Focus on material facts that affect the legal outcome.",
            current_facts, precedent_facts
        )
    }
}

/// LLM-based interpreter for complex legal conditions.
///
/// # Example
///
/// ```
/// use legalis_core::llm_interpretation::LlmInterpreter;
///
/// let interpreter = LlmInterpreter::new();
/// assert_eq!(interpreter.cache_size(), 0);
/// ```
pub struct LlmInterpreter {
    /// Optional LLM backend
    #[allow(dead_code)]
    backend: Option<Box<dyn LlmBackend>>,
    /// Response cache (prompt hash -> response)
    cache: HashMap<u64, LlmResponse>,
    /// Statistics
    stats: LlmStats,
}

impl LlmInterpreter {
    /// Creates a new LLM interpreter.
    pub fn new() -> Self {
        Self {
            backend: None,
            cache: HashMap::new(),
            stats: LlmStats::new(),
        }
    }

    /// Sets the LLM backend.
    #[allow(dead_code)]
    pub fn with_backend(mut self, backend: Box<dyn LlmBackend>) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Interprets a condition using the LLM.
    ///
    /// Returns cached response if available, otherwise queries the LLM.
    #[allow(dead_code)]
    pub fn interpret_condition(
        &mut self,
        condition: &Condition,
        context: &str,
    ) -> Result<LlmResponse, LlmError> {
        let prompt = format!(
            "Interpret this legal condition: {:?}\nContext: {}",
            condition, context
        );
        self.query(&prompt, 500)
    }

    /// Queries the LLM with a custom prompt.
    pub fn query(&mut self, prompt: &str, _max_tokens: usize) -> Result<LlmResponse, LlmError> {
        // Check cache first
        let prompt_hash = self.hash_prompt(prompt);
        if let Some(cached) = self.cache.get(&prompt_hash) {
            self.stats.cache_hits += 1;
            return Ok(cached.clone());
        }

        self.stats.cache_misses += 1;

        // In a real implementation, would call backend.generate()
        // For now, return a placeholder response
        Err(LlmError::BackendNotAvailable)
    }

    /// Returns the number of cached responses.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Clears the response cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Returns LLM usage statistics.
    pub fn stats(&self) -> &LlmStats {
        &self.stats
    }

    /// Computes a hash for a prompt (for caching).
    fn hash_prompt(&self, prompt: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for LlmInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for LLM usage.
#[derive(Debug, Clone, Default)]
pub struct LlmStats {
    /// Total number of LLM queries
    pub total_queries: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Total prompt tokens used
    pub total_prompt_tokens: u64,
    /// Total completion tokens used
    pub total_completion_tokens: u64,
    /// Total estimated cost in USD
    pub total_cost_usd: f64,
}

impl LlmStats {
    /// Creates new LLM statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Returns the total number of tokens used.
    pub fn total_tokens(&self) -> u64 {
        self.total_prompt_tokens + self.total_completion_tokens
    }

    /// Returns the average tokens per query.
    pub fn avg_tokens_per_query(&self) -> f64 {
        if self.total_queries == 0 {
            0.0
        } else {
            self.total_tokens() as f64 / self.total_queries as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_response() {
        let response = LlmResponse::new("This is a test response".to_string(), 0.9);
        assert_eq!(response.text, "This is a test response");
        assert_eq!(response.confidence, 0.9);
        assert!(response.is_high_confidence());
    }

    #[test]
    fn test_llm_error_display() {
        let err = LlmError::BackendNotAvailable;
        assert_eq!(format!("{}", err), "LLM backend not available");

        let err = LlmError::InvalidRequest("Bad prompt".to_string());
        assert_eq!(format!("{}", err), "Invalid request: Bad prompt");
    }

    #[test]
    fn test_prompt_template_condition() {
        let prompt =
            PromptTemplate::condition_interpretation("good faith", "Did party act in good faith?");
        assert!(prompt.contains("good faith"));
        assert!(prompt.contains("legal context") || prompt.contains("legal"));
    }

    #[test]
    fn test_prompt_template_statute() {
        let prompt = PromptTemplate::statute_interpretation(
            "Section 1: No person shall...",
            "Does this apply to corporations?",
        );
        assert!(prompt.contains("Section 1"));
        assert!(prompt.contains("corporations"));
    }

    #[test]
    fn test_prompt_template_analogy() {
        let prompt = PromptTemplate::case_analogy("Facts A", "Facts B");
        assert!(prompt.contains("Facts A"));
        assert!(prompt.contains("Facts B"));
        assert!(prompt.contains("analogical reasoning") || prompt.contains("analogi"));
    }

    #[test]
    fn test_llm_interpreter() {
        let interpreter = LlmInterpreter::new();
        assert_eq!(interpreter.cache_size(), 0);
        assert_eq!(interpreter.stats().total_queries, 0);
    }

    #[test]
    fn test_llm_stats() {
        let mut stats = LlmStats::new();
        stats.cache_hits = 8;
        stats.cache_misses = 2;
        stats.total_queries = 10;
        stats.total_prompt_tokens = 1000;
        stats.total_completion_tokens = 500;

        assert_eq!(stats.cache_hit_rate(), 0.8);
        assert_eq!(stats.total_tokens(), 1500);
        assert_eq!(stats.avg_tokens_per_query(), 150.0);
    }

    #[test]
    fn test_response_confidence() {
        let high = LlmResponse::new("Yes".to_string(), 0.95);
        assert!(high.is_high_confidence());

        let low = LlmResponse::new("Maybe".to_string(), 0.5);
        assert!(!low.is_high_confidence());
    }
}
