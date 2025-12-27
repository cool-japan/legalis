//! Legalis-LLM: LLM integration layer for Legalis-RS.
//!
//! This crate provides an abstraction layer for LLM providers,
//! enabling pluggable AI models (OpenAI, Anthropic, Gemini, Local LLMs).

mod advanced_prompting;
mod batch;
mod benchmark;
mod cache;
mod cancellation;
mod compiler;
mod compliance;
mod conversation;
mod distributed;
mod edge;
mod embeddings;
mod evaluation;
mod functions;
mod gpu_scheduler;
mod hot_swap;
mod legal;
mod legal_context;
mod legal_prompting;
mod memory;
mod multi_agent;
mod multimodal;
mod observability;
mod providers;
mod quantization;
mod rag;
mod regression;
mod resilience;
mod router;
mod safety;
mod structured_output;
mod templates;
mod testing;
mod token_tracker;
mod validation;

pub use advanced_prompting::*;
pub use batch::*;
pub use benchmark::*;
pub use cache::*;
pub use cancellation::*;
pub use compiler::*;
pub use compliance::*;
pub use conversation::*;
pub use distributed::*;
pub use edge::*;
pub use embeddings::*;
pub use evaluation::*;
pub use functions::*;
pub use gpu_scheduler::*;
pub use hot_swap::*;
pub use legal::*;
pub use legal_context::*;
pub use legal_prompting::*;
pub use memory::*;
pub use multi_agent::*;
pub use multimodal::*;
pub use observability::*;
pub use providers::*;
pub use quantization::*;
pub use rag::*;
pub use regression::*;
pub use resilience::*;
pub use router::*;
pub use safety::*;
pub use structured_output::*;
pub use templates::*;
pub use testing::*;
pub use token_tracker::*;
pub use validation::*;

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

/// Progress tracking for LLM operations.
pub mod progress {
    use super::*;
    use futures::stream::StreamExt;
    use std::sync::Arc;

    /// Progress events during LLM operations.
    #[derive(Debug, Clone)]
    pub enum ProgressEvent {
        /// Request started
        Started { prompt_length: usize },
        /// Streaming chunk received
        ChunkReceived {
            chunk_size: usize,
            total_size: usize,
        },
        /// Request completed
        Completed {
            total_tokens: Option<usize>,
            duration_ms: u128,
        },
        /// Request failed
        Failed { error: String },
    }

    /// Callback for progress events.
    pub type ProgressCallback = Arc<dyn Fn(ProgressEvent) + Send + Sync>;

    /// Wraps an LLM provider with progress tracking.
    pub struct ProgressProvider<P> {
        provider: P,
        callback: Option<ProgressCallback>,
    }

    impl<P> ProgressProvider<P> {
        /// Creates a new progress provider without callbacks.
        pub fn new(provider: P) -> Self {
            Self {
                provider,
                callback: None,
            }
        }

        /// Sets the progress callback.
        pub fn with_callback<F>(mut self, callback: F) -> Self
        where
            F: Fn(ProgressEvent) + Send + Sync + 'static,
        {
            self.callback = Some(Arc::new(callback));
            self
        }

        fn emit(&self, event: ProgressEvent) {
            if let Some(ref callback) = self.callback {
                callback(event);
            }
        }

        /// Gets a reference to the underlying provider.
        pub fn provider(&self) -> &P {
            &self.provider
        }
    }

    #[async_trait]
    impl<P: LLMProvider> LLMProvider for ProgressProvider<P> {
        async fn generate_text(&self, prompt: &str) -> Result<String> {
            let start = std::time::Instant::now();
            self.emit(ProgressEvent::Started {
                prompt_length: prompt.len(),
            });

            match self.provider.generate_text(prompt).await {
                Ok(response) => {
                    let duration_ms = start.elapsed().as_millis();
                    self.emit(ProgressEvent::Completed {
                        total_tokens: None,
                        duration_ms,
                    });
                    Ok(response)
                }
                Err(e) => {
                    self.emit(ProgressEvent::Failed {
                        error: e.to_string(),
                    });
                    Err(e)
                }
            }
        }

        async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
            let start = std::time::Instant::now();
            self.emit(ProgressEvent::Started {
                prompt_length: prompt.len(),
            });

            match self.provider.generate_structured::<T>(prompt).await {
                Ok(response) => {
                    let duration_ms = start.elapsed().as_millis();
                    self.emit(ProgressEvent::Completed {
                        total_tokens: None,
                        duration_ms,
                    });
                    Ok(response)
                }
                Err(e) => {
                    self.emit(ProgressEvent::Failed {
                        error: e.to_string(),
                    });
                    Err(e)
                }
            }
        }

        async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
            use std::sync::{Arc as StdArc, Mutex as StdMutex};

            self.emit(ProgressEvent::Started {
                prompt_length: prompt.len(),
            });

            let stream = self.provider.generate_text_stream(prompt).await?;
            let callback = self.callback.clone();
            let total_size = StdArc::new(StdMutex::new(0usize));

            // Wrap the stream to track progress
            let wrapped_stream = stream.map(move |result| {
                match &result {
                    Ok(chunk) => {
                        let mut size = total_size.lock().unwrap();
                        *size += chunk.content.len();
                        if let Some(ref cb) = callback {
                            cb(ProgressEvent::ChunkReceived {
                                chunk_size: chunk.content.len(),
                                total_size: *size,
                            });
                        }
                    }
                    Err(e) => {
                        if let Some(ref cb) = callback {
                            cb(ProgressEvent::Failed {
                                error: e.to_string(),
                            });
                        }
                    }
                }
                result
            });

            Ok(Box::pin(wrapped_stream))
        }

        fn provider_name(&self) -> &str {
            self.provider.provider_name()
        }

        fn model_name(&self) -> &str {
            self.provider.model_name()
        }

        fn supports_streaming(&self) -> bool {
            self.provider.supports_streaming()
        }
    }
}

/// Stream combinators for transforming text streams.
pub mod stream_utils {
    use super::*;
    use futures::stream::StreamExt;

    /// Takes the first N chunks from a stream.
    pub fn take(stream: TextStream, n: usize) -> TextStream {
        Box::pin(stream.take(n))
    }

    /// Skips the first N chunks from a stream.
    pub fn skip(stream: TextStream, n: usize) -> TextStream {
        Box::pin(stream.skip(n))
    }

    /// Maps each chunk's content using the provided function.
    pub fn map_content<F>(stream: TextStream, f: F) -> TextStream
    where
        F: Fn(String) -> String + Send + 'static,
    {
        Box::pin(stream.map(move |result| {
            result.map(|mut chunk| {
                chunk.content = f(chunk.content);
                chunk
            })
        }))
    }

    /// Filters chunks based on a predicate.
    pub fn filter<F>(stream: TextStream, predicate: F) -> TextStream
    where
        F: Fn(&StreamChunk) -> bool + Send + 'static,
    {
        Box::pin(stream.filter(move |result| {
            let keep = match result {
                Ok(chunk) => predicate(chunk),
                Err(_) => true, // Always keep errors
            };
            futures::future::ready(keep)
        }))
    }

    /// Collects all chunks into a single string.
    pub async fn collect_to_string(mut stream: TextStream) -> Result<String> {
        let mut result = String::new();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            result.push_str(&chunk.content);
        }
        Ok(result)
    }

    /// Taps into the stream to execute side effects without modifying it.
    pub fn tap<F>(stream: TextStream, f: F) -> TextStream
    where
        F: Fn(&StreamChunk) + Send + 'static,
    {
        Box::pin(stream.map(move |result| {
            if let Ok(ref chunk) = result {
                f(chunk);
            }
            result
        }))
    }

    /// Adds a prefix to each chunk's content.
    pub fn prefix(stream: TextStream, prefix: String) -> TextStream {
        let mut is_first = true;
        Box::pin(stream.map(move |result| {
            result.map(|mut chunk| {
                if is_first && !chunk.content.is_empty() {
                    chunk.content = format!("{}{}", prefix, chunk.content);
                    is_first = false;
                }
                chunk
            })
        }))
    }

    /// Adds a suffix to the final chunk's content.
    pub fn suffix(stream: TextStream, suffix: String) -> TextStream {
        Box::pin(stream.map(move |result| {
            result.map(|mut chunk| {
                if chunk.is_final {
                    chunk.content.push_str(&suffix);
                }
                chunk
            })
        }))
    }

    /// Buffers chunks until a certain size is reached or stream ends.
    pub fn buffer_chunks(stream: TextStream, buffer_size: usize) -> TextStream {
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let buffer = Arc::new(Mutex::new(String::new()));

        Box::pin(stream.filter_map(move |result| {
            let buffer = buffer.clone();
            async move {
                match result {
                    Ok(chunk) => {
                        let mut buf = buffer.lock().await;
                        buf.push_str(&chunk.content);

                        if chunk.is_final || buf.len() >= buffer_size {
                            let content = std::mem::take(&mut *buf);
                            Some(Ok(StreamChunk {
                                content,
                                is_final: chunk.is_final,
                                token_count: chunk.token_count,
                            }))
                        } else {
                            None
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            }
        }))
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

    #[tokio::test]
    async fn test_stream_collect() {
        use futures::stream;

        let chunks = vec![
            Ok(StreamChunk::new("Hello ")),
            Ok(StreamChunk::new("World")),
            Ok(StreamChunk::final_chunk("!")),
        ];

        let stream: TextStream = Box::pin(stream::iter(chunks));
        let result = stream_utils::collect_to_string(stream).await.unwrap();

        assert_eq!(result, "Hello World!");
    }
}
