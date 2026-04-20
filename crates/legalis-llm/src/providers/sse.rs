//! Server-Sent Events (SSE) stream parsing utilities.

use crate::StreamChunk;
use anyhow::{Result, anyhow};
use bytes::Bytes;
use futures::stream::StreamExt;
use serde::Deserialize;

/// Parses Server-Sent Events (SSE) stream into StreamChunks.
///
/// This function properly handles:
/// - Buffering incomplete lines across byte chunks
/// - Parsing "data: " prefixed SSE messages
/// - Handling `[DONE]` completion marker
/// - JSON parsing of OpenAI streaming responses
/// - Error propagation with context
pub(super) fn parse_sse_stream(
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
pub(super) fn parse_anthropic_sse_stream(
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
                                if let Some(delta) = event.delta
                                    && let Some(text) = delta.text
                                {
                                    chunks.push(Ok(StreamChunk::new(text)));
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
pub(super) fn parse_gemini_stream(
    byte_stream: impl futures::Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
) -> impl futures::Stream<Item = Result<StreamChunk>> + Send {
    use futures::stream;

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

    #[derive(Deserialize)]
    struct GeminiPart {
        text: String,
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
                        if let Some(candidate) = response.candidates.first()
                            && let Some(part) = candidate.content.parts.first()
                            && !part.text.is_empty()
                        {
                            chunks.push(Ok(StreamChunk::new(part.text.clone())));
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
