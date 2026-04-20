//! HuggingFace Inference API provider implementation.

use super::common::extract_json;
use crate::{LLMConfig, LLMProvider, TextStream};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// HuggingFace Inference API client.
pub struct HuggingFaceClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl HuggingFaceClient {
    /// Creates a new HuggingFace client.
    ///
    /// # Arguments
    /// * `api_key` - HuggingFace API token
    /// * `model` - Model ID (e.g., "gpt2", "meta-llama/Llama-2-7b-chat-hf")
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

    fn inference_url(&self) -> String {
        format!("https://api-inference.huggingface.co/models/{}", self.model)
    }
}

#[async_trait]
impl LLMProvider for HuggingFaceClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        #[derive(Serialize)]
        struct HfRequest {
            inputs: String,
            parameters: HfParameters,
        }

        #[derive(Serialize)]
        struct HfParameters {
            max_new_tokens: u32,
            temperature: f32,
            return_full_text: bool,
        }

        #[derive(Deserialize)]
        struct HfResponse {
            #[serde(default)]
            generated_text: String,
        }

        let mut full_prompt = prompt.to_string();
        if let Some(ref system_prompt) = self.config.system_prompt {
            full_prompt = format!("{}\n\n{}", system_prompt, prompt);
        }

        let request = HfRequest {
            inputs: full_prompt,
            parameters: HfParameters {
                max_new_tokens: self.config.max_tokens,
                temperature: self.config.temperature,
                return_full_text: false,
            },
        };

        let response = self
            .client
            .post(self.inference_url())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to HuggingFace API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("HuggingFace API error {}: {}", status, error_text));
        }

        // HuggingFace returns an array of responses
        let hf_responses: Vec<HfResponse> = response
            .json()
            .await
            .context("Failed to parse HuggingFace response")?;

        hf_responses
            .first()
            .map(|r| r.generated_text.clone())
            .ok_or_else(|| anyhow!("No response from HuggingFace"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "HuggingFace"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_text_stream(&self, _prompt: &str) -> Result<TextStream> {
        // HuggingFace Inference API doesn't support streaming in the same way
        // We'll return an error indicating streaming is not supported
        Err(anyhow!(
            "Streaming is not supported for HuggingFace Inference API"
        ))
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}
