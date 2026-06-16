//! Cohere and Perplexity provider implementations.

use super::common::{ChatMessage, ChatRequest, ChatResponse, extract_json};
use super::sse::parse_sse_stream;
use crate::{LLMConfig, LLMProvider, TextStream};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Cohere client for natural language understanding.
///
/// Cohere specializes in NLU tasks like classification, semantic search, and generation.
pub struct CohereClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl CohereClient {
    /// Creates a new Cohere client.
    ///
    /// # Arguments
    /// * `api_key` - Cohere API key
    /// * `model` - Model name (e.g., "command", "command-light", "command-r-plus")
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::new(),
            config: LLMConfig::default(),
        }
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: LLMConfig) -> Self {
        self.config = config;
        self
    }
}

#[derive(Serialize)]
struct CohereRequest {
    model: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preamble: Option<String>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct CohereResponse {
    text: String,
}

#[async_trait]
impl LLMProvider for CohereClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let request = CohereRequest {
            model: self.model.clone(),
            message: prompt.to_string(),
            preamble: self.config.system_prompt.clone(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let response = self
            .client
            .post("https://api.cohere.ai/v1/chat")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Cohere API")?;

        let cohere_response: CohereResponse = response
            .json()
            .await
            .context("Failed to parse Cohere response")?;

        Ok(cohere_response.text)
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Cohere"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        #[derive(Serialize)]
        struct CohereStreamRequest<'a> {
            model: &'a str,
            message: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            preamble: Option<&'a str>,
            max_tokens: u32,
            temperature: f32,
            stream: bool,
        }

        let request = CohereStreamRequest {
            model: &self.model,
            message: prompt,
            preamble: self.config.system_prompt.as_deref(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: true,
        };

        let response = self
            .client
            .post("https://api.cohere.ai/v1/chat")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Cohere API")?;

        let byte_stream = response.bytes_stream();
        let text_stream = parse_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

/// Perplexity AI client for web-grounded responses.
///
/// Perplexity provides LLMs with real-time web search integration.
/// Uses OpenAI-compatible API.
pub struct PerplexityClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl PerplexityClient {
    /// Creates a new Perplexity client.
    ///
    /// # Arguments
    /// * `api_key` - Perplexity API key
    /// * `model` - Model name (e.g., "llama-3.1-sonar-small-128k-online", "llama-3.1-sonar-large-128k-online")
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::new(),
            config: LLMConfig::default(),
        }
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: LLMConfig) -> Self {
        self.config = config;
        self
    }
}

#[async_trait]
impl LLMProvider for PerplexityClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let mut messages = Vec::new();

        if let Some(ref system_prompt) = self.config.system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: system_prompt.clone(),
            });
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let response = self
            .client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Perplexity API")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse Perplexity response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("No response from Perplexity"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Perplexity"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let mut messages = Vec::new();

        if let Some(ref system_prompt) = self.config.system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: system_prompt.clone(),
            });
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        #[derive(Serialize)]
        struct StreamRequest {
            model: String,
            messages: Vec<ChatMessage>,
            max_tokens: u32,
            temperature: f32,
            stream: bool,
        }

        let request = StreamRequest {
            model: self.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: true,
        };

        let response = self
            .client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Perplexity API")?;

        let byte_stream = response.bytes_stream();
        let text_stream = parse_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohere_supports_streaming() {
        let client = CohereClient::new("test-key", "command");
        assert!(client.supports_streaming());
    }

    #[test]
    fn test_cohere_stream_request_serialization() {
        #[derive(serde::Serialize)]
        struct CohereStreamRequest<'a> {
            model: &'a str,
            message: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            preamble: Option<&'a str>,
            max_tokens: u32,
            temperature: f32,
            stream: bool,
        }

        let req = CohereStreamRequest {
            model: "command",
            message: "hello",
            preamble: None,
            max_tokens: 100,
            temperature: 0.7,
            stream: true,
        };

        let val = serde_json::to_value(&req).unwrap();
        assert_eq!(val["stream"], serde_json::json!(true));
        assert_eq!(val["model"], serde_json::json!("command"));
        assert_eq!(val["message"], serde_json::json!("hello"));
    }
}
