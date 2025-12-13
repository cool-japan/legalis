//! Legalis-LLM: LLM integration layer for Legalis-RS.
//!
//! This crate provides an abstraction layer for LLM providers,
//! enabling pluggable AI models (OpenAI, Anthropic, Gemini, Local LLMs).

mod compiler;
mod providers;

pub use compiler::*;
pub use providers::*;

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::Stream;
use serde::de::DeserializeOwned;
use std::pin::Pin;

/// Streaming text chunk from an LLM.
#[derive(Debug, Clone)]
pub struct StreamChunk {
    /// The text content
    pub content: String,
    /// Whether this is the final chunk
    pub is_final: bool,
    /// Token count (if available)
    pub token_count: Option<usize>,
}

impl StreamChunk {
    /// Creates a new stream chunk.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_final: false,
            token_count: None,
        }
    }

    /// Creates a final stream chunk.
    pub fn final_chunk(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_final: true,
            token_count: None,
        }
    }

    /// Adds token count information.
    pub fn with_token_count(mut self, count: usize) -> Self {
        self.token_count = Some(count);
        self
    }
}

/// Type alias for stream of text chunks.
pub type TextStream = Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>;

/// LLM provider abstraction trait.
///
/// This trait allows swapping between different LLM providers
/// without changing the application logic.
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Generates text from a prompt.
    async fn generate_text(&self, prompt: &str) -> Result<String>;

    /// Generates structured data (JSON) from a prompt.
    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T>;

    /// Generates text as a stream of chunks.
    ///
    /// This enables real-time streaming of responses for better UX.
    /// If streaming is not supported, returns an error.
    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream>;

    /// Returns the name of this provider.
    fn provider_name(&self) -> &str;

    /// Returns the model being used.
    fn model_name(&self) -> &str;

    /// Returns whether this provider supports streaming.
    fn supports_streaming(&self) -> bool {
        false
    }
}

/// Configuration for LLM requests.
#[derive(Debug, Clone)]
pub struct LLMConfig {
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// Temperature for sampling (0.0 - 1.0)
    pub temperature: f32,
    /// System prompt to prepend
    pub system_prompt: Option<String>,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            max_tokens: 4096,
            temperature: 0.7,
            system_prompt: None,
        }
    }
}

impl LLMConfig {
    /// Creates a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum tokens.
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Sets the temperature.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }

    /// Sets the system prompt.
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_defaults() {
        let config = LLMConfig::default();
        assert_eq!(config.max_tokens, 4096);
        assert!((config.temperature - 0.7).abs() < f32::EPSILON);
        assert!(config.system_prompt.is_none());
    }

    #[test]
    fn test_llm_config_builder() {
        let config = LLMConfig::new()
            .with_max_tokens(2048)
            .with_temperature(0.5)
            .with_system_prompt("Test system prompt");

        assert_eq!(config.max_tokens, 2048);
        assert!((config.temperature - 0.5).abs() < f32::EPSILON);
        assert_eq!(config.system_prompt, Some("Test system prompt".to_string()));
    }

    #[test]
    fn test_temperature_clamping() {
        let config = LLMConfig::new().with_temperature(1.5);
        assert!((config.temperature - 1.0).abs() < f32::EPSILON);

        let config = LLMConfig::new().with_temperature(-0.5);
        assert!(config.temperature.abs() < f32::EPSILON);
    }
}
