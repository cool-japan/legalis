//! Anthropic Claude provider implementation.

use super::common::extract_json;
use super::sse::parse_anthropic_sse_stream;
use crate::{LLMConfig, LLMProvider, TextStream};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Anthropic Claude client.
pub struct AnthropicClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl AnthropicClient {
    /// Creates a new Anthropic client.
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
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

#[async_trait]
impl LLMProvider for AnthropicClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: self.config.max_tokens,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            system: self.config.system_prompt.clone(),
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        anthropic_response
            .content
            .first()
            .map(|c| c.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Anthropic"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Anthropic"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        #[derive(Serialize)]
        struct AnthropicStreamRequest {
            model: String,
            max_tokens: u32,
            messages: Vec<AnthropicMessage>,
            #[serde(skip_serializing_if = "Option::is_none")]
            system: Option<String>,
            stream: bool,
        }

        let request = AnthropicStreamRequest {
            model: self.model.clone(),
            max_tokens: self.config.max_tokens,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            system: self.config.system_prompt.clone(),
            stream: true,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Anthropic API")?;

        // Get the byte stream from the response
        let byte_stream = response.bytes_stream();

        // Convert to text stream with SSE parsing for Anthropic format
        let text_stream = parse_anthropic_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}
