//! LLM provider implementations.

use crate::{LLMConfig, LLMProvider, StreamChunk, TextStream};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// OpenAI (or compatible API) client.
pub struct OpenAiClient {
    api_key: String,
    model: String,
    base_url: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl OpenAiClient {
    /// Creates a new OpenAI client.
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            client: reqwest::Client::new(),
            config: LLMConfig::default(),
        }
    }

    /// Sets a custom base URL (for OpenAI-compatible APIs).
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
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[async_trait]
impl LLMProvider for OpenAiClient {
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
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;

        // Try to extract JSON from the response
        let json_str = extract_json(&text).unwrap_or(&text);

        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
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
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to OpenAI API")?;

        // Get the byte stream from the response
        let byte_stream = response.bytes_stream();

        // Convert to text stream with SSE parsing
        let text_stream = parse_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

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

/// Mistral AI client.
pub struct MistralClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
    config: LLMConfig,
}

impl MistralClient {
    /// Creates a new Mistral AI client.
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
impl LLMProvider for MistralClient {
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
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Mistral AI API")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse Mistral AI response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("No response from Mistral AI"))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        let json_str = extract_json(&text).unwrap_or(&text);
        serde_json::from_str(json_str).context("Failed to parse structured response")
    }

    fn provider_name(&self) -> &str {
        "Mistral AI"
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
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request to Mistral AI API")?;

        let byte_stream = response.bytes_stream();
        let text_stream = parse_sse_stream(byte_stream);

        Ok(Box::pin(text_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

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

/// Mock LLM provider for testing.
#[derive(Clone)]
pub struct MockProvider {
    responses: std::collections::HashMap<String, String>,
}

impl MockProvider {
    /// Creates a new mock provider.
    pub fn new() -> Self {
        Self {
            responses: std::collections::HashMap::new(),
        }
    }

    /// Adds a mock response for a given prompt pattern.
    pub fn with_response(
        mut self,
        pattern: impl Into<String>,
        response: impl Into<String>,
    ) -> Self {
        self.responses.insert(pattern.into(), response.into());
        self
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LLMProvider for MockProvider {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        for (pattern, response) in &self.responses {
            if prompt.contains(pattern) {
                return Ok(response.clone());
            }
        }
        Ok("Mock response: No matching pattern found".to_string())
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        serde_json::from_str(&text).context("Failed to parse mock response as JSON")
    }

    fn provider_name(&self) -> &str {
        "Mock"
    }

    fn model_name(&self) -> &str {
        "mock-v1"
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        // Get the full response first
        let text = self.generate_text(prompt).await?;

        // Split into chunks (simulate streaming by splitting at word boundaries)
        let words: Vec<&str> = text.split_whitespace().collect();
        let chunk_size = 5; // 5 words per chunk

        let mut chunks = Vec::new();
        let total_chunks = words.len().div_ceil(chunk_size);

        for (i, word_chunk) in words.chunks(chunk_size).enumerate() {
            let content = if i == 0 {
                word_chunk.join(" ")
            } else {
                format!(" {}", word_chunk.join(" "))
            };

            let is_final = i == total_chunks - 1;
            let mut chunk = StreamChunk::new(content);
            chunk.is_final = is_final;
            chunks.push(chunk);
        }

        // Convert to stream
        use futures::stream;
        let stream = stream::iter(chunks.into_iter().map(Ok));

        Ok(Box::pin(stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

/// Parses Server-Sent Events (SSE) stream into StreamChunks.
///
/// This function properly handles:
/// - Buffering incomplete lines across byte chunks
/// - Parsing "data: " prefixed SSE messages
/// - Handling `[DONE]` completion marker
/// - JSON parsing of OpenAI streaming responses
/// - Error propagation with context
fn parse_sse_stream(
    byte_stream: impl futures::Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> impl futures::Stream<Item = Result<StreamChunk>> + Send {
    use futures::stream;

    // SSE response structures
    #[derive(Deserialize)]
    struct StreamResponse {
        choices: Vec<StreamChoice>,
    }

    #[derive(Deserialize)]
    struct StreamChoice {
        delta: Delta,
        finish_reason: Option<String>,
    }

    #[derive(Deserialize)]
    struct Delta {
        content: Option<String>,
    }

    // State for buffering across chunks
    struct ParserState {
        buffer: String,
    }

    let initial_state = ParserState {
        buffer: String::new(),
    };

    byte_stream
        .scan(initial_state, |state, byte_result| {
            // Convert reqwest error to anyhow error
            let bytes = match byte_result {
                Ok(b) => b,
                Err(e) => {
                    return futures::future::ready(Some(vec![Err(anyhow!("Stream error: {}", e))]));
                }
            };

            // Append to buffer
            match String::from_utf8(bytes.to_vec()) {
                Ok(text) => state.buffer.push_str(&text),
                Err(e) => {
                    return futures::future::ready(Some(vec![Err(anyhow!(
                        "UTF-8 decode error: {}",
                        e
                    ))]));
                }
            }

            let mut chunks = Vec::new();

            // Process complete lines
            while let Some(newline_pos) = state.buffer.find('\n') {
                let line = state.buffer[..newline_pos].trim().to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                // Skip empty lines
                if line.is_empty() {
                    continue;
                }

                // Parse SSE format: "data: <json>" or "data: [DONE]"
                if let Some(data) = line.strip_prefix("data: ") {
                    // Check for completion marker
                    if data == "[DONE]" {
                        chunks.push(Ok(StreamChunk::final_chunk("")));
                        continue;
                    }

                    // Parse JSON response
                    match serde_json::from_str::<StreamResponse>(data) {
                        Ok(response) => {
                            if let Some(choice) = response.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    let is_final = choice.finish_reason.is_some();
                                    let mut chunk = StreamChunk::new(content.clone());
                                    chunk.is_final = is_final;
                                    chunks.push(Ok(chunk));
                                } else if choice.finish_reason.is_some() {
                                    // Final chunk with no content
                                    chunks.push(Ok(StreamChunk::final_chunk("")));
                                }
                            }
                        }
                        Err(e) => {
                            // Log parse error but continue streaming
                            tracing::debug!("Failed to parse SSE JSON: {} for data: {}", e, data);
                        }
                    }
                }
            }

            futures::future::ready(Some(chunks))
        })
        .flat_map(stream::iter)
}

/// Parses Anthropic SSE stream into StreamChunks.
fn parse_anthropic_sse_stream(
    byte_stream: impl futures::Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> impl futures::Stream<Item = Result<StreamChunk>> + Send {
    use futures::stream;

    #[derive(Deserialize)]
    struct AnthropicStreamEvent {
        #[serde(rename = "type")]
        event_type: String,
        #[serde(default)]
        delta: Option<AnthropicDelta>,
    }

    #[derive(Deserialize)]
    struct AnthropicDelta {
        #[serde(rename = "type")]
        #[allow(dead_code)]
        delta_type: String,
        text: Option<String>,
    }

    struct ParserState {
        buffer: String,
    }

    let initial_state = ParserState {
        buffer: String::new(),
    };

    byte_stream
        .scan(initial_state, |state, byte_result| {
            let bytes = match byte_result {
                Ok(b) => b,
                Err(e) => {
                    return futures::future::ready(Some(vec![Err(anyhow!("Stream error: {}", e))]));
                }
            };

            match String::from_utf8(bytes.to_vec()) {
                Ok(text) => state.buffer.push_str(&text),
                Err(e) => {
                    return futures::future::ready(Some(vec![Err(anyhow!(
                        "UTF-8 decode error: {}",
                        e
                    ))]));
                }
            }

            let mut chunks = Vec::new();

            while let Some(newline_pos) = state.buffer.find('\n') {
                let line = state.buffer[..newline_pos].trim().to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    match serde_json::from_str::<AnthropicStreamEvent>(data) {
                        Ok(event) => {
                            if event.event_type == "content_block_delta" {
                                if let Some(delta) = event.delta {
                                    if let Some(text) = delta.text {
                                        chunks.push(Ok(StreamChunk::new(text)));
                                    }
                                }
                            } else if event.event_type == "message_stop" {
                                chunks.push(Ok(StreamChunk::final_chunk("")));
                            }
                        }
                        Err(e) => {
                            tracing::debug!(
                                "Failed to parse Anthropic SSE JSON: {} for data: {}",
                                e,
                                data
                            );
                        }
                    }
                }
            }

            futures::future::ready(Some(chunks))
        })
        .flat_map(stream::iter)
}

/// Parses Gemini streaming responses (newline-delimited JSON) into StreamChunks.
///
/// Gemini's streaming API returns JSON objects separated by newlines.
/// Each JSON object has the same structure as the non-streaming response.
fn parse_gemini_stream(
    byte_stream: impl futures::Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> impl futures::Stream<Item = Result<StreamChunk>> + Send {
    use futures::stream;

    struct ParserState {
        buffer: String,
    }

    let initial_state = ParserState {
        buffer: String::new(),
    };

    byte_stream
        .scan(initial_state, |state, byte_result| {
            let bytes = match byte_result {
                Ok(b) => b,
                Err(e) => {
                    return futures::future::ready(Some(vec![Err(anyhow!("Stream error: {}", e))]));
                }
            };

            match String::from_utf8(bytes.to_vec()) {
                Ok(text) => state.buffer.push_str(&text),
                Err(e) => {
                    return futures::future::ready(Some(vec![Err(anyhow!(
                        "UTF-8 decode error: {}",
                        e
                    ))]));
                }
            }

            let mut chunks = Vec::new();

            // Process complete JSON objects (separated by newlines)
            while let Some(newline_pos) = state.buffer.find('\n') {
                let line = state.buffer[..newline_pos].trim().to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                // Parse the JSON response
                match serde_json::from_str::<GeminiResponse>(&line) {
                    Ok(response) => {
                        if let Some(candidate) = response.candidates.first() {
                            if let Some(part) = candidate.content.parts.first() {
                                if !part.text.is_empty() {
                                    chunks.push(Ok(StreamChunk::new(part.text.clone())));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::debug!("Failed to parse Gemini JSON: {} for line: {}", e, line);
                        // Don't fail the stream for parse errors, just log and continue
                    }
                }
            }

            futures::future::ready(Some(chunks))
        })
        .flat_map(stream::iter)
}

/// Extracts JSON from a text that might contain markdown code blocks or other content.
fn extract_json(text: &str) -> Option<&str> {
    // Try to find JSON in code blocks first
    if let Some(start) = text.find("```json") {
        let content_start = start + 7;
        if let Some(end) = text[content_start..].find("```") {
            return Some(text[content_start..content_start + end].trim());
        }
    }

    // Try to find JSON in generic code blocks
    if let Some(start) = text.find("```") {
        let content_start = text[start + 3..].find('\n').map(|i| start + 3 + i + 1)?;
        if let Some(end) = text[content_start..].find("```") {
            return Some(text[content_start..content_start + end].trim());
        }
    }

    // Try to find raw JSON object
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if end > start {
                return Some(&text[start..=end]);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_code_block() {
        let text = r#"Here is the JSON:
```json
{"key": "value"}
```
"#;
        assert_eq!(extract_json(text), Some(r#"{"key": "value"}"#));
    }

    #[test]
    fn test_extract_json_raw() {
        let text = r#"The result is {"key": "value"} as expected."#;
        assert_eq!(extract_json(text), Some(r#"{"key": "value"}"#));
    }

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockProvider::new().with_response("test", r#"{"result": "success"}"#);

        let response = provider
            .generate_text("This is a test prompt")
            .await
            .unwrap();
        assert!(response.contains("success"));
    }

    #[tokio::test]
    async fn test_mock_provider_streaming() {
        use futures::StreamExt;

        let provider = MockProvider::new()
            .with_response("test", "This is a test response with multiple words");

        assert!(provider.supports_streaming());

        let mut stream = provider
            .generate_text_stream("This is a test prompt")
            .await
            .unwrap();

        let mut collected = String::new();
        let mut chunk_count = 0;
        let mut saw_final = false;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.unwrap();
            collected.push_str(&chunk.content);
            chunk_count += 1;

            if chunk.is_final {
                saw_final = true;
            }
        }

        assert!(chunk_count > 0, "Should have received at least one chunk");
        assert!(saw_final, "Should have seen a final chunk");
        assert_eq!(
            collected.trim(),
            "This is a test response with multiple words",
            "Collected text should match original response"
        );
    }

    #[tokio::test]
    async fn test_stream_chunk_builder() {
        let chunk = StreamChunk::new("test content").with_token_count(42);

        assert_eq!(chunk.content, "test content");
        assert!(!chunk.is_final);
        assert_eq!(chunk.token_count, Some(42));

        let final_chunk = StreamChunk::final_chunk("final");
        assert!(final_chunk.is_final);
    }
}
