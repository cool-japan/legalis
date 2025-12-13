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

    async fn generate_text_stream(&self, _prompt: &str) -> Result<TextStream> {
        // Anthropic streaming would be implemented similarly to OpenAI
        // For now, return an error indicating it's not yet implemented
        Err(anyhow!(
            "Streaming not yet implemented for Anthropic provider"
        ))
    }

    fn supports_streaming(&self) -> bool {
        false // Will be true once implemented
    }
}

/// Mock LLM provider for testing.
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
