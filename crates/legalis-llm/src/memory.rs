//! Memory-augmented generation for LLMs.
//!
//! This module provides memory capabilities for LLMs, allowing them to
//! maintain context across multiple interactions and retrieve relevant
//! information from past conversations.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A memory entry in the memory store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique ID of the memory
    pub id: String,
    /// Content of the memory
    pub content: String,
    /// Timestamp when the memory was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Importance score (0.0 - 1.0)
    pub importance: f32,
    /// Embedding vector for semantic search (optional)
    pub embedding: Option<Vec<f32>>,
    /// Metadata tags
    pub tags: Vec<String>,
}

impl MemoryEntry {
    /// Creates a new memory entry.
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            timestamp: chrono::Utc::now(),
            importance: 0.5,
            embedding: None,
            tags: Vec::new(),
        }
    }

    /// Sets the importance score.
    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    /// Sets the embedding vector.
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    /// Adds tags to the memory.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Computes recency score (decays with time).
    pub fn recency_score(&self) -> f32 {
        let now = chrono::Utc::now();
        let age_hours = (now - self.timestamp).num_hours() as f32;

        // Exponential decay: score = e^(-age/decay_constant)
        // decay_constant = 24 hours
        (-age_hours / 24.0).exp()
    }

    /// Computes relevance score given a query embedding.
    pub fn relevance_score(&self, query_embedding: &[f32]) -> f32 {
        match &self.embedding {
            Some(emb) => cosine_similarity(emb, query_embedding),
            None => 0.0,
        }
    }

    /// Computes overall retrieval score combining recency, importance, and relevance.
    pub fn retrieval_score(&self, query_embedding: Option<&[f32]>) -> f32 {
        let recency = self.recency_score();
        let importance = self.importance;
        let relevance = query_embedding
            .map(|q| self.relevance_score(q))
            .unwrap_or(0.5);

        // Weighted combination
        0.3 * recency + 0.3 * importance + 0.4 * relevance
    }
}

/// Cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

/// Memory store for storing and retrieving memories.
pub struct MemoryStore {
    /// Short-term memory (recent interactions)
    short_term: VecDeque<MemoryEntry>,
    /// Long-term memory (important memories)
    long_term: Vec<MemoryEntry>,
    /// Maximum size of short-term memory
    max_short_term: usize,
    /// Maximum size of long-term memory
    max_long_term: usize,
    /// Importance threshold for promoting to long-term memory
    importance_threshold: f32,
}

impl MemoryStore {
    /// Creates a new memory store.
    pub fn new(max_short_term: usize, max_long_term: usize) -> Self {
        Self {
            short_term: VecDeque::with_capacity(max_short_term),
            long_term: Vec::with_capacity(max_long_term),
            max_short_term,
            max_long_term,
            importance_threshold: 0.7,
        }
    }

    /// Sets the importance threshold for long-term memory promotion.
    pub fn with_importance_threshold(mut self, threshold: f32) -> Self {
        self.importance_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Adds a memory to the store.
    pub fn add(&mut self, memory: MemoryEntry) {
        // Check if memory should go to long-term storage
        if memory.importance >= self.importance_threshold {
            self.add_to_long_term(memory);
        } else {
            self.add_to_short_term(memory);
        }
    }

    /// Adds a memory to short-term storage.
    fn add_to_short_term(&mut self, memory: MemoryEntry) {
        if self.short_term.len() >= self.max_short_term {
            // Remove oldest memory
            if let Some(old) = self.short_term.pop_front() {
                // Promote to long-term if important enough
                if old.importance >= self.importance_threshold {
                    self.add_to_long_term(old);
                }
            }
        }
        self.short_term.push_back(memory);
    }

    /// Adds a memory to long-term storage.
    fn add_to_long_term(&mut self, memory: MemoryEntry) {
        if self.long_term.len() >= self.max_long_term {
            // Remove least important memory
            if let Some(idx) = self
                .long_term
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.importance
                        .partial_cmp(&b.importance)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
            {
                self.long_term.remove(idx);
            }
        }
        self.long_term.push(memory);
    }

    /// Retrieves memories based on a query.
    pub fn retrieve(&self, query_embedding: Option<&[f32]>, limit: usize) -> Vec<MemoryEntry> {
        let mut all_memories: Vec<MemoryEntry> = self
            .short_term
            .iter()
            .chain(self.long_term.iter())
            .cloned()
            .collect();

        // Score and sort memories
        all_memories.sort_by(|a, b| {
            let score_a = a.retrieval_score(query_embedding);
            let score_b = b.retrieval_score(query_embedding);
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        all_memories.into_iter().take(limit).collect()
    }

    /// Retrieves recent memories.
    pub fn retrieve_recent(&self, limit: usize) -> Vec<MemoryEntry> {
        self.short_term.iter().rev().take(limit).cloned().collect()
    }

    /// Retrieves important memories.
    pub fn retrieve_important(&self, limit: usize) -> Vec<MemoryEntry> {
        let mut memories = self.long_term.clone();
        memories.sort_by(|a, b| {
            b.importance
                .partial_cmp(&a.importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        memories.into_iter().take(limit).collect()
    }

    /// Returns the total number of memories.
    pub fn len(&self) -> usize {
        self.short_term.len() + self.long_term.len()
    }

    /// Returns true if the memory store is empty.
    pub fn is_empty(&self) -> bool {
        self.short_term.is_empty() && self.long_term.is_empty()
    }

    /// Clears all memories.
    pub fn clear(&mut self) {
        self.short_term.clear();
        self.long_term.clear();
    }

    /// Summarizes memories into a compact representation.
    pub fn summarize(&self) -> String {
        let recent: Vec<_> = self.retrieve_recent(3);
        let important: Vec<_> = self.retrieve_important(3);

        let mut summary = String::new();

        if !recent.is_empty() {
            summary.push_str("Recent context:\n");
            for memory in recent {
                summary.push_str(&format!("- {}\n", memory.content));
            }
            summary.push('\n');
        }

        if !important.is_empty() {
            summary.push_str("Important information:\n");
            for memory in important {
                summary.push_str(&format!("- {}\n", memory.content));
            }
        }

        summary
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new(50, 100)
    }
}

/// Strategy for retrieving memories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrievalStrategy {
    /// Retrieve most recent memories
    Recent,
    /// Retrieve most important memories
    Important,
    /// Retrieve most relevant memories (requires embeddings)
    Relevant,
    /// Combine all factors (recency, importance, relevance)
    Combined,
}

/// Configuration for memory-augmented generation.
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Maximum memories to retrieve
    pub max_retrieve: usize,
    /// Retrieval strategy
    pub strategy: RetrievalStrategy,
    /// Whether to include memory summary in prompts
    pub include_summary: bool,
    /// Whether to automatically extract important facts
    pub auto_extract_facts: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_retrieve: 5,
            strategy: RetrievalStrategy::Combined,
            include_summary: true,
            auto_extract_facts: false,
        }
    }
}

impl MemoryConfig {
    /// Creates a new memory configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of memories to retrieve.
    pub fn with_max_retrieve(mut self, max: usize) -> Self {
        self.max_retrieve = max;
        self
    }

    /// Sets the retrieval strategy.
    pub fn with_strategy(mut self, strategy: RetrievalStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Enables or disables memory summary inclusion.
    pub fn with_summary(mut self, include: bool) -> Self {
        self.include_summary = include;
        self
    }

    /// Enables or disables automatic fact extraction.
    pub fn with_auto_extract(mut self, enable: bool) -> Self {
        self.auto_extract_facts = enable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_entry_creation() {
        let memory = MemoryEntry::new("test-1", "This is a test memory")
            .with_importance(0.8)
            .with_tags(vec!["test".to_string(), "example".to_string()]);

        assert_eq!(memory.id, "test-1");
        assert_eq!(memory.content, "This is a test memory");
        assert_eq!(memory.importance, 0.8);
        assert_eq!(memory.tags.len(), 2);
    }

    #[test]
    fn test_recency_score() {
        let memory = MemoryEntry::new("test", "content");
        let score = memory.recency_score();

        // Recent memory should have high recency score
        assert!(score > 0.9);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&c, &d).abs() < 0.001);
    }

    #[test]
    fn test_memory_store_add() {
        let mut store = MemoryStore::new(5, 10);

        store.add(MemoryEntry::new("1", "First memory").with_importance(0.5));
        store.add(MemoryEntry::new("2", "Second memory").with_importance(0.9));

        assert_eq!(store.len(), 2);
        assert_eq!(store.short_term.len(), 1);
        assert_eq!(store.long_term.len(), 1);
    }

    #[test]
    fn test_memory_store_overflow() {
        let mut store = MemoryStore::new(3, 3);

        for i in 0..5 {
            store.add(
                MemoryEntry::new(format!("{}", i), format!("Memory {}", i)).with_importance(0.5),
            );
        }

        // Should maintain maximum capacity
        assert!(store.len() <= 6);
    }

    #[test]
    fn test_retrieve_recent() {
        let mut store = MemoryStore::new(10, 10);

        for i in 0..5 {
            store.add(
                MemoryEntry::new(format!("{}", i), format!("Memory {}", i)).with_importance(0.5),
            );
        }

        let recent = store.retrieve_recent(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].id, "4"); // Most recent first
    }

    #[test]
    fn test_retrieve_important() {
        let mut store = MemoryStore::new(10, 10);

        store.add(MemoryEntry::new("1", "Low importance").with_importance(0.3));
        store.add(MemoryEntry::new("2", "High importance").with_importance(0.9));
        store.add(MemoryEntry::new("3", "Medium importance").with_importance(0.6));

        let important = store.retrieve_important(2);
        assert_eq!(important.len(), 1); // Only one made it to long-term
        assert_eq!(important[0].id, "2");
    }

    #[test]
    fn test_memory_summarize() {
        let mut store = MemoryStore::new(10, 10);

        store.add(MemoryEntry::new("1", "User prefers dark mode").with_importance(0.8));
        store.add(MemoryEntry::new("2", "User works in software").with_importance(0.9));

        let summary = store.summarize();
        assert!(summary.contains("dark mode") || summary.contains("software"));
    }

    #[test]
    fn test_memory_config() {
        let config = MemoryConfig::new()
            .with_max_retrieve(10)
            .with_strategy(RetrievalStrategy::Recent)
            .with_summary(false);

        assert_eq!(config.max_retrieve, 10);
        assert_eq!(config.strategy, RetrievalStrategy::Recent);
        assert!(!config.include_summary);
    }

    #[test]
    fn test_memory_store_clear() {
        let mut store = MemoryStore::new(10, 10);

        store.add(MemoryEntry::new("1", "Test"));
        assert!(!store.is_empty());

        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }
}
