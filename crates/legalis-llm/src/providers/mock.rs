//! Mock LLM provider for testing.

use crate::{LLMProvider, StreamChunk, TextStream};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockProvider::new().with_response("test", r#"{"result": "success"}"#);

        let response = provider
            .generate_text("This is a test prompt")
            .await
            .expect("mock generate_text should not fail");
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
            .expect("mock generate_text_stream should not fail");

        let mut collected = String::new();
        let mut chunk_count = 0;
        let mut saw_final = false;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.expect("stream chunk should not fail");
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
    }
}
