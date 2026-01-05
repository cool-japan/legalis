//! Conversation management for multi-turn LLM interactions.
//!
//! This module provides tools for managing conversation history,
//! context windows, and multi-turn dialogue with LLMs.

use crate::{LLMProvider, TokenEstimator};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;

/// Role of a message in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System message (instructions, context)
    System,
    /// User message
    User,
    /// Assistant message
    Assistant,
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender
    pub role: Role,
    /// Content of the message
    pub content: String,
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Timestamp when message was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Message {
    /// Creates a new message with the given role and content.
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            metadata: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Creates a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }

    /// Creates a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    /// Creates an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    /// Adds metadata to the message.
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Estimates token count for this message.
    pub fn estimate_tokens(&self) -> usize {
        TokenEstimator::estimate_tokens(&self.content)
    }
}

/// Strategy for managing context window when it exceeds limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextStrategy {
    /// Drop oldest messages (except system)
    DropOldest,
    /// Truncate from the middle (keep beginning and end)
    TruncateMiddle,
    /// Summarize old messages (requires ConversationSummarizer)
    Summarize,
    /// Error when limit exceeded
    Error,
}

/// Conversation summarizer trait.
#[async_trait::async_trait]
pub trait ConversationSummarizer: Send + Sync {
    /// Summarizes a set of messages into a single condensed message.
    async fn summarize(&self, messages: &[Message]) -> Result<String>;
}

/// LLM-based conversation summarizer.
pub struct LLMSummarizer<P> {
    provider: P,
    summary_prompt: String,
}

impl<P: LLMProvider> LLMSummarizer<P> {
    /// Creates a new LLM-based summarizer.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            summary_prompt: "Summarize the following conversation concisely, preserving key information and context:".to_string(),
        }
    }

    /// Sets a custom summary prompt.
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.summary_prompt = prompt.into();
        self
    }
}

#[async_trait::async_trait]
impl<P: LLMProvider> ConversationSummarizer for LLMSummarizer<P> {
    async fn summarize(&self, messages: &[Message]) -> Result<String> {
        if messages.is_empty() {
            return Ok(String::new());
        }

        let conversation_text = messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::System => "System",
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                };
                format!("{}: {}", role, msg.content)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!("{}\n\n{}", self.summary_prompt, conversation_text);
        self.provider.generate_text(&prompt).await
    }
}

/// Configuration for conversation management.
#[derive(Debug, Clone)]
pub struct ConversationConfig {
    /// Maximum number of messages to keep in history
    pub max_messages: Option<usize>,
    /// Maximum total tokens across all messages
    pub max_tokens: Option<usize>,
    /// Strategy for handling context overflow
    pub context_strategy: ContextStrategy,
    /// System prompt to prepend to all conversations
    pub system_prompt: Option<String>,
    /// Number of old messages to summarize when using Summarize strategy
    pub summarize_count: usize,
}

impl Default for ConversationConfig {
    fn default() -> Self {
        Self {
            max_messages: Some(100),
            max_tokens: Some(4000),
            context_strategy: ContextStrategy::DropOldest,
            system_prompt: None,
            summarize_count: 5,
        }
    }
}

impl ConversationConfig {
    /// Creates a new configuration with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets maximum messages.
    pub fn with_max_messages(mut self, max: usize) -> Self {
        self.max_messages = Some(max);
        self
    }

    /// Sets maximum tokens.
    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = Some(max);
        self
    }

    /// Sets context strategy.
    pub fn with_context_strategy(mut self, strategy: ContextStrategy) -> Self {
        self.context_strategy = strategy;
        self
    }

    /// Sets system prompt.
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Sets the number of messages to summarize.
    pub fn with_summarize_count(mut self, count: usize) -> Self {
        self.summarize_count = count;
        self
    }
}

/// A conversation with message history and context management.
#[derive(Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique identifier for this conversation
    pub id: String,
    /// Messages in the conversation
    messages: VecDeque<Message>,
    /// Configuration
    #[serde(skip)]
    config: ConversationConfig,
    /// Total token count estimate
    total_tokens: usize,
    /// Optional summarizer for context management
    #[serde(skip)]
    summarizer: Option<Arc<dyn ConversationSummarizer>>,
}

impl std::fmt::Debug for Conversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conversation")
            .field("id", &self.id)
            .field("messages", &self.messages)
            .field("config", &self.config)
            .field("total_tokens", &self.total_tokens)
            .field("summarizer", &self.summarizer.is_some())
            .finish()
    }
}

impl Conversation {
    /// Creates a new conversation with the given ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self::with_config(id, ConversationConfig::default())
    }

    /// Creates a new conversation with custom configuration.
    pub fn with_config(id: impl Into<String>, config: ConversationConfig) -> Self {
        let mut conv = Self {
            id: id.into(),
            messages: VecDeque::new(),
            config,
            total_tokens: 0,
            summarizer: None,
        };

        // Add system prompt if configured
        if let Some(ref system_prompt) = conv.config.system_prompt {
            conv.add_message(Message::system(system_prompt));
        }

        conv
    }

    /// Sets the conversation summarizer.
    pub fn with_summarizer<S: ConversationSummarizer + 'static>(mut self, summarizer: S) -> Self {
        self.summarizer = Some(Arc::new(summarizer));
        self
    }

    /// Summarizes old messages to save context space.
    pub async fn summarize_old_messages(&mut self) -> Result<()> {
        if let Some(ref summarizer) = self.summarizer {
            let count = self.config.summarize_count.min(self.messages.len());
            if count == 0 {
                return Ok(());
            }

            // Extract messages to summarize (skip system messages)
            let to_summarize: Vec<Message> = self
                .messages
                .iter()
                .filter(|m| m.role != Role::System)
                .take(count)
                .cloned()
                .collect();

            if to_summarize.is_empty() {
                return Ok(());
            }

            // Generate summary
            let summary = summarizer.summarize(&to_summarize).await?;

            // Remove the old messages and add summary as system message
            let mut removed_count = 0;
            self.messages.retain(|m| {
                if removed_count < count && m.role != Role::System {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });

            // Add summary as a system message at the beginning (after initial system prompt)
            let insert_pos = if self.messages.front().map(|m| m.role) == Some(Role::System) {
                1
            } else {
                0
            };

            self.messages.insert(
                insert_pos,
                Message::system(format!("[Summary of previous conversation]\n{}", summary)),
            );

            self.recalculate_tokens();
        }

        Ok(())
    }

    /// Adds a message to the conversation.
    pub fn add_message(&mut self, message: Message) {
        let tokens = message.estimate_tokens();
        self.total_tokens += tokens;
        self.messages.push_back(message);
        self.apply_limits();
    }

    /// Adds a user message.
    pub fn add_user(&mut self, content: impl Into<String>) {
        self.add_message(Message::user(content));
    }

    /// Adds an assistant message.
    pub fn add_assistant(&mut self, content: impl Into<String>) {
        self.add_message(Message::assistant(content));
    }

    /// Returns all messages in the conversation.
    pub fn messages(&self) -> &VecDeque<Message> {
        &self.messages
    }

    /// Returns the number of messages.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns whether the conversation is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Returns estimated total tokens.
    pub fn total_tokens(&self) -> usize {
        self.total_tokens
    }

    /// Clears all messages except system prompt.
    pub fn clear(&mut self) {
        self.messages.retain(|m| m.role == Role::System);
        self.recalculate_tokens();
    }

    /// Formats the conversation as a prompt string.
    pub fn format_as_prompt(&self) -> String {
        self.messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::System => "System",
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                };
                format!("{}: {}", role, msg.content)
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Returns the last user message, if any.
    pub fn last_user_message(&self) -> Option<&Message> {
        self.messages.iter().rev().find(|m| m.role == Role::User)
    }

    /// Returns the last assistant message, if any.
    pub fn last_assistant_message(&self) -> Option<&Message> {
        self.messages
            .iter()
            .rev()
            .find(|m| m.role == Role::Assistant)
    }

    /// Creates a branch (copy) of this conversation.
    pub fn branch(&self, new_id: impl Into<String>) -> Self {
        Self {
            id: new_id.into(),
            messages: self.messages.clone(),
            config: self.config.clone(),
            total_tokens: self.total_tokens,
            summarizer: self.summarizer.clone(),
        }
    }

    /// Applies message and token limits based on configuration.
    fn apply_limits(&mut self) {
        // Check message limit
        if let Some(max_messages) = self.config.max_messages {
            while self.messages.len() > max_messages {
                self.drop_oldest_non_system();
            }
        }

        // Check token limit
        if let Some(max_tokens) = self.config.max_tokens {
            while self.total_tokens > max_tokens && self.messages.len() > 1 {
                match self.config.context_strategy {
                    ContextStrategy::DropOldest => {
                        self.drop_oldest_non_system();
                    }
                    ContextStrategy::TruncateMiddle => {
                        self.truncate_middle();
                    }
                    ContextStrategy::Summarize => {
                        // Note: Summarization is async, so we fall back to dropping
                        // The user should call summarize_old_messages() explicitly
                        self.drop_oldest_non_system();
                    }
                    ContextStrategy::Error => {
                        break; // Don't auto-trim in error mode
                    }
                }
            }
        }
    }

    /// Drops the oldest non-system message.
    fn drop_oldest_non_system(&mut self) {
        // Find first non-system message
        let pos = self.messages.iter().position(|m| m.role != Role::System);

        if let Some(pos) = pos {
            if let Some(removed) = self.messages.remove(pos) {
                self.total_tokens = self.total_tokens.saturating_sub(removed.estimate_tokens());
            }
        }
    }

    /// Truncates messages from the middle, keeping first and last messages.
    fn truncate_middle(&mut self) {
        if self.messages.len() <= 2 {
            return;
        }

        // Keep first (system) and last few messages
        let system_messages: Vec<_> = self
            .messages
            .iter()
            .filter(|m| m.role == Role::System)
            .cloned()
            .collect();

        let recent_count = 3;
        let recent_messages: Vec<_> = self
            .messages
            .iter()
            .rev()
            .take(recent_count)
            .cloned()
            .collect();

        self.messages.clear();
        self.messages.extend(system_messages);
        self.messages.extend(recent_messages.into_iter().rev());
        self.recalculate_tokens();
    }

    /// Recalculates total token count.
    fn recalculate_tokens(&mut self) {
        self.total_tokens = self.messages.iter().map(|m| m.estimate_tokens()).sum();
    }
}

/// A conversation manager that integrates with an LLM provider.
pub struct ConversationManager<P> {
    provider: P,
    conversation: Conversation,
}

impl<P: LLMProvider> ConversationManager<P> {
    /// Creates a new conversation manager.
    pub fn new(provider: P, conversation_id: impl Into<String>) -> Self {
        Self {
            provider,
            conversation: Conversation::new(conversation_id),
        }
    }

    /// Creates a new conversation manager with custom configuration.
    pub fn with_config(
        provider: P,
        conversation_id: impl Into<String>,
        config: ConversationConfig,
    ) -> Self {
        Self {
            provider,
            conversation: Conversation::with_config(conversation_id, config),
        }
    }

    /// Sends a user message and gets an assistant response.
    pub async fn send(&mut self, user_message: impl Into<String>) -> Result<String> {
        let user_msg = user_message.into();
        self.conversation.add_user(&user_msg);

        let prompt = self.conversation.format_as_prompt();
        let response = self.provider.generate_text(&prompt).await?;

        self.conversation.add_assistant(&response);
        Ok(response)
    }

    /// Gets the conversation history.
    pub fn conversation(&self) -> &Conversation {
        &self.conversation
    }

    /// Gets mutable access to the conversation.
    pub fn conversation_mut(&mut self) -> &mut Conversation {
        &mut self.conversation
    }

    /// Resets the conversation (keeps system prompt if any).
    pub fn reset(&mut self) {
        self.conversation.clear();
    }
}

/// Conversation persistence trait.
#[async_trait::async_trait]
pub trait ConversationStore: Send + Sync {
    /// Saves a conversation.
    async fn save(&self, conversation: &Conversation) -> Result<()>;

    /// Loads a conversation by ID.
    async fn load(&self, id: &str) -> Result<Option<Conversation>>;

    /// Deletes a conversation.
    async fn delete(&self, id: &str) -> Result<()>;

    /// Lists all conversation IDs.
    async fn list(&self) -> Result<Vec<String>>;
}

/// In-memory conversation store.
#[derive(Default)]
pub struct InMemoryStore {
    conversations:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Conversation>>>,
}

impl InMemoryStore {
    /// Creates a new in-memory store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl ConversationStore for InMemoryStore {
    async fn save(&self, conversation: &Conversation) -> Result<()> {
        let mut store = self.conversations.write().await;
        store.insert(conversation.id.clone(), conversation.clone());
        Ok(())
    }

    async fn load(&self, id: &str) -> Result<Option<Conversation>> {
        let store = self.conversations.read().await;
        Ok(store.get(id).cloned())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let mut store = self.conversations.write().await;
        store.remove(id);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<String>> {
        let store = self.conversations.read().await;
        Ok(store.keys().cloned().collect())
    }
}

/// File-based conversation store (JSON).
pub struct FileStore {
    base_path: std::path::PathBuf,
}

impl FileStore {
    /// Creates a new file store at the given path.
    pub fn new(base_path: impl Into<std::path::PathBuf>) -> Result<Self> {
        let base_path = base_path.into();
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    fn conversation_path(&self, id: &str) -> std::path::PathBuf {
        self.base_path.join(format!("{}.json", id))
    }
}

#[async_trait::async_trait]
impl ConversationStore for FileStore {
    async fn save(&self, conversation: &Conversation) -> Result<()> {
        let path = self.conversation_path(&conversation.id);
        let json = serde_json::to_string_pretty(conversation)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    async fn load(&self, id: &str) -> Result<Option<Conversation>> {
        let path = self.conversation_path(id);
        if !path.exists() {
            return Ok(None);
        }

        let json = tokio::fs::read_to_string(path).await?;
        let conversation = serde_json::from_str(&json)?;
        Ok(Some(conversation))
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let path = self.conversation_path(id);
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<String>> {
        let mut ids = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.base_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    ids.push(name.trim_end_matches(".json").to_string());
                }
            }
        }

        Ok(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::user("Hello");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content, "Hello");
        assert!(msg.metadata.is_none());
    }

    #[test]
    fn test_message_with_metadata() {
        let metadata = serde_json::json!({"key": "value"});
        let msg = Message::user("Hello").with_metadata(metadata.clone());
        assert_eq!(msg.metadata, Some(metadata));
    }

    #[test]
    fn test_conversation_basic() {
        let mut conv = Conversation::new("test-1");
        assert_eq!(conv.id, "test-1");
        assert!(conv.is_empty());

        conv.add_user("Hello");
        assert_eq!(conv.len(), 1);

        conv.add_assistant("Hi there!");
        assert_eq!(conv.len(), 2);
    }

    #[test]
    fn test_conversation_with_system_prompt() {
        let config = ConversationConfig::new().with_system_prompt("You are a helpful assistant");

        let conv = Conversation::with_config("test-2", config);
        assert_eq!(conv.len(), 1);
        assert_eq!(conv.messages[0].role, Role::System);
    }

    #[test]
    fn test_conversation_max_messages() {
        let config = ConversationConfig::new().with_max_messages(3);
        let mut conv = Conversation::with_config("test-3", config);

        conv.add_user("Message 1");
        conv.add_assistant("Response 1");
        conv.add_user("Message 2");
        conv.add_assistant("Response 2");

        // Should keep only last 3 messages
        assert_eq!(conv.len(), 3);
    }

    #[test]
    fn test_conversation_format_as_prompt() {
        let mut conv = Conversation::new("test-4");
        conv.add_user("Hello");
        conv.add_assistant("Hi!");

        let prompt = conv.format_as_prompt();
        assert!(prompt.contains("User: Hello"));
        assert!(prompt.contains("Assistant: Hi!"));
    }

    #[test]
    fn test_conversation_last_messages() {
        let mut conv = Conversation::new("test-5");
        conv.add_user("First user message");
        conv.add_assistant("First assistant message");
        conv.add_user("Second user message");

        assert_eq!(
            conv.last_user_message().unwrap().content,
            "Second user message"
        );
        assert_eq!(
            conv.last_assistant_message().unwrap().content,
            "First assistant message"
        );
    }

    #[test]
    fn test_conversation_branch() {
        let mut conv = Conversation::new("original");
        conv.add_user("Hello");
        conv.add_assistant("Hi!");

        let branch = conv.branch("branch-1");
        assert_eq!(branch.id, "branch-1");
        assert_eq!(branch.len(), conv.len());
    }

    #[test]
    fn test_conversation_clear() {
        let config = ConversationConfig::new().with_system_prompt("System message");
        let mut conv = Conversation::with_config("test-6", config);

        conv.add_user("User message");
        conv.add_assistant("Assistant message");
        assert_eq!(conv.len(), 3);

        conv.clear();
        // Should keep system message
        assert_eq!(conv.len(), 1);
        assert_eq!(conv.messages[0].role, Role::System);
    }

    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryStore::new();
        let conv = Conversation::new("test-conv");

        store.save(&conv).await.unwrap();

        let loaded = store.load("test-conv").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id, "test-conv");

        let list = store.list().await.unwrap();
        assert_eq!(list.len(), 1);

        store.delete("test-conv").await.unwrap();
        let loaded = store.load("test-conv").await.unwrap();
        assert!(loaded.is_none());
    }

    #[tokio::test]
    async fn test_file_store() {
        let temp_dir = std::env::temp_dir().join("legalis-test-conversations");
        let store = FileStore::new(&temp_dir).unwrap();

        let mut conv = Conversation::new("file-test");
        conv.add_user("Hello");

        store.save(&conv).await.unwrap();

        let loaded = store.load("file-test").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id, "file-test");

        store.delete("file-test").await.unwrap();
        let loaded = store.load("file-test").await.unwrap();
        assert!(loaded.is_none());

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
