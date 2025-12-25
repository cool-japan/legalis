//! Cancellation support for LLM streaming operations.
//!
//! This module provides utilities for gracefully cancelling long-running
//! LLM operations, particularly streaming requests.

use crate::TextStream;
use anyhow::Result;
use futures::stream::StreamExt;
use tokio_util::sync::CancellationToken;

/// A wrapper that adds cancellation support to text streams.
pub struct CancellableStream {
    stream: TextStream,
    token: CancellationToken,
}

impl CancellableStream {
    /// Creates a new cancellable stream.
    pub fn new(stream: TextStream, token: CancellationToken) -> Self {
        Self { stream, token }
    }

    /// Gets the cancellation token.
    pub fn token(&self) -> &CancellationToken {
        &self.token
    }

    /// Consumes the stream, returning the inner stream with cancellation checking.
    pub fn into_stream(self) -> TextStream {
        let token = self.token;
        Box::pin(self.stream.take_while(move |_| {
            let is_cancelled = token.is_cancelled();
            futures::future::ready(!is_cancelled)
        }))
    }
}

/// Extension trait for adding cancellation support to streams.
pub trait CancellableStreamExt: Sized {
    /// Adds a cancellation token to this stream.
    fn with_cancellation(self, token: CancellationToken) -> CancellableStream;
}

impl CancellableStreamExt for TextStream {
    fn with_cancellation(self, token: CancellationToken) -> CancellableStream {
        CancellableStream::new(self, token)
    }
}

/// Context for managing cancellable LLM operations.
pub struct CancellationContext {
    token: CancellationToken,
}

impl CancellationContext {
    /// Creates a new cancellation context.
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    /// Gets a reference to the cancellation token.
    pub fn token(&self) -> &CancellationToken {
        &self.token
    }

    /// Cancels all operations associated with this context.
    pub fn cancel(&self) {
        self.token.cancel();
    }

    /// Checks if the operation has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }

    /// Creates a child cancellation token that is cancelled when either
    /// this token or its parent is cancelled.
    pub fn child_token(&self) -> CancellationToken {
        self.token.child_token()
    }
}

impl Default for CancellationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for creating timeout-based cancellation.
pub struct TimeoutCancellation {
    context: CancellationContext,
}

impl TimeoutCancellation {
    /// Creates a new timeout cancellation that will cancel after the specified duration.
    pub fn new(duration: std::time::Duration) -> Self {
        let context = CancellationContext::new();
        let token = context.token().clone();

        // Spawn a task to cancel after the timeout
        tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            token.cancel();
        });

        Self { context }
    }

    /// Gets the cancellation context.
    pub fn context(&self) -> &CancellationContext {
        &self.context
    }

    /// Gets the cancellation token.
    pub fn token(&self) -> &CancellationToken {
        self.context.token()
    }
}

/// Wrapper for a stream that supports timeout cancellation.
pub fn with_timeout(stream: TextStream, duration: std::time::Duration) -> TextStream {
    let timeout_cancel = TimeoutCancellation::new(duration);
    let token = timeout_cancel.token().clone();

    Box::pin(stream.take_while(move |_| {
        let is_cancelled = token.is_cancelled();
        futures::future::ready(!is_cancelled)
    }))
}

/// Merges multiple cancellation tokens into a single one.
/// The returned token will be cancelled when any of the input tokens is cancelled.
pub fn merge_tokens(tokens: Vec<CancellationToken>) -> CancellationToken {
    let merged = CancellationToken::new();

    for token in tokens {
        let merged_clone = merged.clone();
        tokio::spawn(async move {
            token.cancelled().await;
            merged_clone.cancel();
        });
    }

    merged
}

/// Guards a stream with multiple cancellation conditions.
pub struct CancellationGuard {
    tokens: Vec<CancellationToken>,
}

impl CancellationGuard {
    /// Creates a new guard with no tokens.
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Adds a cancellation token to the guard.
    pub fn with_token(mut self, token: CancellationToken) -> Self {
        self.tokens.push(token);
        self
    }

    /// Adds a timeout to the guard.
    pub fn with_timeout(self, duration: std::time::Duration) -> Self {
        let timeout_cancel = TimeoutCancellation::new(duration);
        self.with_token(timeout_cancel.token().clone())
    }

    /// Applies the guard to a stream.
    pub fn guard(self, stream: TextStream) -> TextStream {
        if self.tokens.is_empty() {
            return stream;
        }

        let merged = merge_tokens(self.tokens);
        Box::pin(stream.take_while(move |_| {
            let is_cancelled = merged.is_cancelled();
            futures::future::ready(!is_cancelled)
        }))
    }
}

impl Default for CancellationGuard {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to collect a stream with cancellation support.
pub async fn collect_with_cancellation(
    mut stream: TextStream,
    token: &CancellationToken,
) -> Result<String> {
    let mut result = String::new();

    while let Some(chunk_result) = stream.next().await {
        if token.is_cancelled() {
            return Err(anyhow::anyhow!("Operation cancelled"));
        }

        let chunk = chunk_result?;
        result.push_str(&chunk.content);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StreamChunk;
    use futures::stream;

    #[tokio::test]
    async fn test_cancellation_context() {
        let ctx = CancellationContext::new();
        assert!(!ctx.is_cancelled());

        ctx.cancel();
        assert!(ctx.is_cancelled());
    }

    #[tokio::test]
    async fn test_child_token() {
        let ctx = CancellationContext::new();
        let child = ctx.child_token();

        assert!(!ctx.is_cancelled());
        assert!(!child.is_cancelled());

        ctx.cancel();
        assert!(ctx.is_cancelled());
        // Small delay for propagation
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        assert!(child.is_cancelled());
    }

    #[tokio::test]
    async fn test_stream_cancellation() {
        let chunks = vec![
            Ok(StreamChunk::new("Hello ")),
            Ok(StreamChunk::new("World")),
            Ok(StreamChunk::final_chunk("!")),
        ];

        let stream: TextStream = Box::pin(stream::iter(chunks));
        let ctx = CancellationContext::new();

        // Cancel immediately
        ctx.cancel();

        let guarded = CancellableStream::new(stream, ctx.token().clone()).into_stream();
        let collected: Vec<_> = guarded.collect().await;

        // Stream should be empty because it was cancelled before any chunks
        assert_eq!(collected.len(), 0);
    }

    #[tokio::test]
    async fn test_timeout_cancellation() {
        use std::time::Duration;

        let chunks = vec![
            Ok(StreamChunk::new("Hello ")),
            Ok(StreamChunk::new("World")),
            Ok(StreamChunk::final_chunk("!")),
        ];

        // Create a slow stream
        let stream: TextStream = Box::pin(stream::iter(chunks).then(|chunk| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            chunk
        }));

        // Set a very short timeout (50ms)
        let guarded = with_timeout(stream, Duration::from_millis(50));
        let collected: Vec<_> = guarded.collect().await;

        // Should timeout before getting all chunks
        assert!(collected.len() < 3);
    }

    #[tokio::test]
    async fn test_merge_tokens() {
        let token1 = CancellationToken::new();
        let token2 = CancellationToken::new();
        let token3 = CancellationToken::new();

        let merged = merge_tokens(vec![token1.clone(), token2.clone(), token3.clone()]);

        assert!(!merged.is_cancelled());

        // Cancel one token
        token2.cancel();

        // Small delay for async propagation
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert!(merged.is_cancelled());
    }

    #[tokio::test]
    async fn test_cancellation_guard() {
        use std::time::Duration;

        let chunks = vec![
            Ok(StreamChunk::new("Hello ")),
            Ok(StreamChunk::new("World")),
            Ok(StreamChunk::final_chunk("!")),
        ];

        let stream: TextStream = Box::pin(stream::iter(chunks).then(|chunk| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            chunk
        }));

        let guard = CancellationGuard::new().with_timeout(Duration::from_millis(150));

        let guarded = guard.guard(stream);
        let collected: Vec<_> = guarded.collect().await;

        // Should get at least one chunk but not all three
        assert!(!collected.is_empty() && collected.len() < 3);
    }

    #[tokio::test]
    async fn test_collect_with_cancellation() {
        let chunks = vec![
            Ok(StreamChunk::new("Hello ")),
            Ok(StreamChunk::new("World")),
            Ok(StreamChunk::final_chunk("!")),
        ];

        let stream: TextStream = Box::pin(stream::iter(chunks));
        let token = CancellationToken::new();

        let result = collect_with_cancellation(stream, &token).await.unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[tokio::test]
    async fn test_collect_with_cancellation_cancelled() {
        let chunks = vec![
            Ok(StreamChunk::new("Hello ")),
            Ok(StreamChunk::new("World")),
            Ok(StreamChunk::final_chunk("!")),
        ];

        let stream: TextStream = Box::pin(stream::iter(chunks));
        let token = CancellationToken::new();

        // Cancel before collecting
        token.cancel();

        let result = collect_with_cancellation(stream, &token).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cancelled"));
    }
}
