//! Azure OpenAI provider implementation.

use super::common::{ChatMessage, ChatRequest, ChatResponse, extract_json};
use super::sse::parse_sse_stream;
use crate::{LLMConfig, LLMProvider, TextStream};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Azure OpenAI client.
///
/// Azure OpenAI uses a different authentication method and endpoint structure
/// compared to standard OpenAI.
pub struct AzureOpenAiClient {
    api_key: String,
    deployment_name: String,
    endpoint: String,
    api_version: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl AzureOpenAiClient {
    /// Creates a new Azure OpenAI client.
    ///
    /// # Arguments
    /// * `api_key` - Azure OpenAI API key
    /// * `deployment_name` - Name of the deployed model
    /// * `endpoint` - Azure OpenAI endpoint (e.g., "<https://your-resource.openai.azure.com>")
    pub fn new(
        api_key: impl Into<String>,
        deployment_name: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            deployment_name: deployment_name.into(),
            endpoint: endpoint.into(),
            api_version: "2024-02-15-preview".to_string(),
            client: reqwest::Client::new(),
            config: LLMConfig::default(),
        }
    }

    /// Sets a custom API version.
    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: LLMConfig) -> Self {
        self.config = config;
        self
    }

    fn chat_completions_url(&self) -> String {
        format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, self.deployment_name, self.api_version
        )
    }
}

#[async_trait]
impl LLMProvider for AzureOpenAiClient {
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
            model: self.deployment_name.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let response = self
            .client
            .post(self.chat_completions_url())
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Azure OpenAI API")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse Azure OpenAI response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("No response from Azure OpenAI"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Azure OpenAI"
    }

    fn model_name(&self) -> &str {
        &self.deployment_name
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
            model: self.deployment_name.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: true,
        };

        let response = self
            .client
            .post(self.chat_completions_url())
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Azure OpenAI API")?;

        let byte_stream = response.bytes_stream();
        let text_stream = parse_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}
