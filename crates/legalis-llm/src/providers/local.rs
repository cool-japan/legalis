//! Local LLM provider implementations (Ollama and llama.cpp).

use super::common::{ChatMessage, ChatRequest, ChatResponse, extract_json};
use super::sse::parse_sse_stream;
use crate::{LLMConfig, LLMProvider, StreamChunk, TextStream};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Ollama client for local LLM support.
///
/// Ollama provides an OpenAI-compatible API for running local models
/// like Llama 2, Mistral, Phi, and others.
pub struct OllamaClient {
    model: String,
    base_url: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl OllamaClient {
    /// Creates a new Ollama client.
    ///
    /// # Arguments
    /// * `model` - The model name (e.g., "llama2", "mistral", "phi")
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            base_url: "http://localhost:11434".to_string(),
            client: reqwest::Client::new(),
            config: LLMConfig::default(),
        }
    }

    /// Sets a custom base URL for the Ollama server.
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: LLMConfig) -> Self {
        self.config = config;
        self
    }
}

#[async_trait]
impl LLMProvider for OllamaClient {
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
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Ollama API")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("No response from Ollama"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Ollama"
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
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Ollama API")?;

        let byte_stream = response.bytes_stream();
        let text_stream = parse_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

/// llama.cpp server client for direct local LLM inference.
///
/// This client connects to a llama.cpp server instance (llama-server).
/// llama.cpp is a C++ implementation that enables running LLaMA and other models
/// locally with CPU or GPU acceleration.
///
/// To use this client, you must first start a llama.cpp server:
/// ```bash
/// llama-server -m /path/to/model.gguf --port 8080
/// ```
pub struct LlamaCppClient {
    model: String,
    base_url: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl LlamaCppClient {
    /// Creates a new llama.cpp client.
    ///
    /// # Arguments
    /// * `model` - The model name or identifier (for display purposes)
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            base_url: "http://localhost:8080".to_string(),
            client: reqwest::Client::new(),
            config: LLMConfig::default(),
        }
    }

    /// Sets a custom base URL for the llama.cpp server.
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: LLMConfig) -> Self {
        self.config = config;
        self
    }
}

#[derive(Serialize)]
struct LlamaCppRequest {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_prompt: Option<String>,
    n_predict: i32,
    temperature: f32,
    stream: bool,
}

#[derive(Deserialize)]
struct LlamaCppResponse {
    content: String,
}

#[derive(Deserialize)]
struct LlamaCppStreamChunk {
    content: String,
    stop: bool,
}

#[async_trait]
impl LLMProvider for LlamaCppClient {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let request = LlamaCppRequest {
            prompt: prompt.to_string(),
            system_prompt: self.config.system_prompt.clone(),
            n_predict: self.config.max_tokens as i32,
            temperature: self.config.temperature,
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/completion", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to llama.cpp server")?;

        let llama_response: LlamaCppResponse = response
            .json()
            .await
            .context("Failed to parse llama.cpp response")?;

        Ok(llama_response.content)
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "llama.cpp"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let request = LlamaCppRequest {
            prompt: prompt.to_string(),
            system_prompt: self.config.system_prompt.clone(),
            n_predict: self.config.max_tokens as i32,
            temperature: self.config.temperature,
            stream: true,
        };

        let response = self
            .client
            .post(format!("{}/completion", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to llama.cpp server")?;

        let byte_stream = response.bytes_stream();

        // Parse the streaming response from llama.cpp
        use futures::stream;
        let text_stream = byte_stream
            .map(|result| {
                match result {
                    Ok(bytes) => {
                        // Convert bytes to string
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            // llama.cpp sends JSON chunks separated by newlines
                            let lines: Vec<&str> = text.lines().collect();
                            let mut chunks = Vec::new();

                            for line in lines {
                                if line.trim().is_empty() {
                                    continue;
                                }

                                // Try to parse as JSON
                                if let Ok(chunk) = serde_json::from_str::<LlamaCppStreamChunk>(line)
                                {
                                    let is_final = chunk.stop;
                                    if !chunk.content.is_empty() {
                                        chunks.push(Ok(if is_final {
                                            StreamChunk::final_chunk(chunk.content)
                                        } else {
                                            StreamChunk::new(chunk.content)
                                        }));
                                    }
                                }
                            }

                            chunks
                        } else {
                            Vec::new()
                        }
                    }
                    Err(e) => vec![Err(anyhow!("Stream error: {}", e))],
                }
            })
            .flat_map(stream::iter);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}
