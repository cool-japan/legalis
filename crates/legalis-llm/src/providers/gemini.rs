//! Google Gemini provider implementation.

use super::common::extract_json;
use super::sse::parse_gemini_stream;
use crate::{LLMConfig, LLMProvider, TextStream};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Google Gemini client.
pub struct GeminiClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl GeminiClient {
    /// Creates a new Gemini client.
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
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiSystemInstruction>,
    generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
struct GeminiSystemInstruction {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GeminiGenerationConfig {
    temperature: f32,
    max_output_tokens: u32,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent,
}

#[derive(Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiPart>,
}

#[async_trait]
impl LLMProvider for GeminiClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let contents = vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: prompt.to_string(),
            }],
        }];

        let system_instruction =
            self.config
                .system_prompt
                .as_ref()
                .map(|sys_prompt| GeminiSystemInstruction {
                    parts: vec![GeminiPart {
                        text: sys_prompt.clone(),
                    }],
                });

        let request = GeminiRequest {
            contents,
            system_instruction,
            generation_config: GeminiGenerationConfig {
                temperature: self.config.temperature,
                max_output_tokens: self.config.max_tokens,
            },
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini response")?;

        gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| anyhow!("No response from Gemini"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Google Gemini"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let contents = vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: prompt.to_string(),
            }],
        }];

        let system_instruction =
            self.config
                .system_prompt
                .as_ref()
                .map(|sys_prompt| GeminiSystemInstruction {
                    parts: vec![GeminiPart {
                        text: sys_prompt.clone(),
                    }],
                });

        let request = GeminiRequest {
            contents,
            system_instruction,
            generation_config: GeminiGenerationConfig {
                temperature: self.config.temperature,
                max_output_tokens: self.config.max_tokens,
            },
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?key={}",
            self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Gemini API")?;

        if !response.status().is_success() {
            return Err(anyhow!("Gemini API error: {}", response.status()));
        }

        let byte_stream = response.bytes_stream();
        Ok(Box::pin(parse_gemini_stream(byte_stream)))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}
