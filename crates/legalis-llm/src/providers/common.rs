//! Common types shared across provider implementations.

use serde::{Deserialize, Serialize};

/// Standard OpenAI-compatible chat request.
#[derive(Serialize)]
pub(super) struct ChatRequest {
    pub(super) model: String,
    pub(super) messages: Vec<ChatMessage>,
    pub(super) max_tokens: u32,
    pub(super) temperature: f32,
}

/// A chat message with role and content.
#[derive(Serialize, Deserialize)]
pub(super) struct ChatMessage {
    pub(super) role: String,
    pub(super) content: String,
}

/// OpenAI-compatible chat response.
#[derive(Deserialize)]
pub(super) struct ChatResponse {
    pub(super) choices: Vec<ChatChoice>,
}

/// A single choice in a chat response.
#[derive(Deserialize)]
pub(super) struct ChatChoice {
    pub(super) message: ChatMessage,
}

/// Extracts JSON from a text that might contain markdown code blocks or other content.
pub(super) fn extract_json(text: &str) -> Option<&str> {
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
    if let Some(start) = text.find('{')
        && let Some(end) = text.rfind('}')
        && end > start
    {
        return Some(&text[start..=end]);
    }

    None
}
