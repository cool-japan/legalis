//! Context management for long legal documents.
//!
//! This module provides advanced context management strategies for handling
//! long legal documents, including sliding windows, hierarchical summarization,
//! RAG-based context building, importance scoring, and automatic pruning.

use crate::{
    LLMProvider,
    rag::{ChunkingStrategy, DocumentChunk},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Sliding window context manager for long documents.
pub struct SlidingWindowContext {
    /// Window size in tokens
    window_size: usize,
    /// Overlap between windows
    #[allow(dead_code)]
    overlap: usize,
    /// Current chunks in the window
    chunks: VecDeque<DocumentChunk>,
    /// Total tokens in current window
    current_tokens: usize,
}

impl SlidingWindowContext {
    /// Creates a new sliding window context manager.
    pub fn new(window_size: usize, overlap: usize) -> Self {
        Self {
            window_size,
            overlap,
            chunks: VecDeque::new(),
            current_tokens: 0,
        }
    }

    /// Adds a chunk to the window.
    pub fn add_chunk(&mut self, chunk: DocumentChunk) {
        let chunk_tokens = chunk.content.split_whitespace().count();

        // Add new chunk
        self.chunks.push_back(chunk);
        self.current_tokens += chunk_tokens;

        // Remove old chunks if window is too large
        while self.current_tokens > self.window_size && !self.chunks.is_empty() {
            if let Some(removed) = self.chunks.pop_front() {
                let removed_tokens = removed.content.split_whitespace().count();
                self.current_tokens = self.current_tokens.saturating_sub(removed_tokens);
            }
        }
    }

    /// Gets the current window content.
    pub fn get_window(&self) -> String {
        self.chunks
            .iter()
            .map(|chunk| &chunk.content)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Gets the number of chunks in the window.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Gets the current token count.
    pub fn token_count(&self) -> usize {
        self.current_tokens
    }

    /// Clears the window.
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.current_tokens = 0;
    }
}

/// Hierarchical summary node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryNode {
    /// Node level (0 = leaf, higher = more abstract)
    pub level: usize,
    /// Summary text
    pub summary: String,
    /// Children summaries (if not leaf)
    #[allow(clippy::vec_box)]
    pub children: Vec<Box<SummaryNode>>,
    /// Original text (for leaf nodes)
    pub original_text: Option<String>,
}

/// Hierarchical summarization manager.
pub struct HierarchicalSummarizer<P> {
    provider: P,
    chunk_size: usize,
    summarization_ratio: f64,
}

impl<P: LLMProvider> HierarchicalSummarizer<P> {
    /// Creates a new hierarchical summarizer.
    ///
    /// # Arguments
    /// * `provider` - LLM provider for generating summaries
    /// * `chunk_size` - Target size for chunks (in characters)
    /// * `summarization_ratio` - Target ratio of summary to original (e.g., 0.3 for 30%)
    pub fn new(provider: P, chunk_size: usize, summarization_ratio: f64) -> Self {
        Self {
            provider,
            chunk_size,
            summarization_ratio,
        }
    }

    /// Creates a hierarchical summary of a long document.
    pub async fn summarize_hierarchical(&self, document: &str) -> Result<SummaryNode> {
        // Split document into chunks
        let chunks = self.chunk_document(document);

        // Create leaf nodes
        let mut leaf_summaries = Vec::new();
        for chunk in chunks {
            let summary = self.summarize_chunk(&chunk).await?;
            leaf_summaries.push(Box::new(SummaryNode {
                level: 0,
                summary,
                children: Vec::new(),
                original_text: Some(chunk),
            }));
        }

        // Build hierarchy
        self.build_hierarchy(leaf_summaries, 1).await
    }

    #[allow(clippy::vec_box)]
    fn build_hierarchy(
        &self,
        nodes: Vec<Box<SummaryNode>>,
        level: usize,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<SummaryNode>> + Send + '_>> {
        Box::pin(async move {
            if nodes.len() == 1 {
                // Base case: single node
                let mut node = *nodes.into_iter().next().unwrap();
                node.level = level;
                return Ok(node);
            }

            // Group nodes and summarize each group
            let group_size = 3; // Summarize groups of 3
            let mut parent_nodes = Vec::new();

            for group in nodes.chunks(group_size) {
                let combined_text = group
                    .iter()
                    .map(|n| &n.summary)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("\n\n");

                let parent_summary = self.summarize_chunk(&combined_text).await?;

                parent_nodes.push(Box::new(SummaryNode {
                    level,
                    summary: parent_summary,
                    children: group.to_vec(),
                    original_text: None,
                }));
            }

            // Recursively build higher levels
            self.build_hierarchy(parent_nodes, level + 1).await
        })
    }

    async fn summarize_chunk(&self, chunk: &str) -> Result<String> {
        let target_length = (chunk.len() as f64 * self.summarization_ratio) as usize;

        let prompt = format!(
            r#"Summarize the following legal text to approximately {} characters while preserving key legal points.

Text:
{}

Provide a concise summary that captures the essential legal content."#,
            target_length, chunk
        );

        self.provider
            .generate_text(&prompt)
            .await
            .context("Failed to summarize chunk")
    }

    fn chunk_document(&self, document: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for paragraph in document.split("\n\n") {
            if current_chunk.len() + paragraph.len() > self.chunk_size && !current_chunk.is_empty()
            {
                chunks.push(current_chunk);
                current_chunk = String::new();
            }
            if !current_chunk.is_empty() {
                current_chunk.push_str("\n\n");
            }
            current_chunk.push_str(paragraph);
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }

    /// Gets a summary at a specific level of detail.
    pub fn get_summary_at_level(&self, root: &SummaryNode, target_level: usize) -> Vec<String> {
        let mut summaries = Vec::new();
        self.collect_summaries_at_level(root, target_level, &mut summaries);
        summaries
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_summaries_at_level(
        &self,
        node: &SummaryNode,
        target_level: usize,
        summaries: &mut Vec<String>,
    ) {
        if node.level == target_level {
            summaries.push(node.summary.clone());
        } else if node.level < target_level {
            for child in &node.children {
                self.collect_summaries_at_level(child, target_level, summaries);
            }
        }
    }
}

/// Context importance scorer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextImportance {
    /// Text chunk
    pub text: String,
    /// Importance score (0.0 - 1.0)
    pub score: f64,
    /// Reason for importance
    pub reason: String,
}

/// Context importance scorer for prioritizing relevant content.
pub struct ContextImportanceScorer<P> {
    provider: P,
}

impl<P: LLMProvider> ContextImportanceScorer<P> {
    /// Creates a new context importance scorer.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Scores chunks by their importance to a query.
    pub async fn score_chunks(
        &self,
        chunks: &[DocumentChunk],
        query: &str,
    ) -> Result<Vec<ContextImportance>> {
        let chunks_text = chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| format!("Chunk {}: {}", i + 1, chunk.content))
            .collect::<Vec<_>>()
            .join("\n\n");

        let prompt = format!(
            r#"Score the importance of each chunk for answering the following query.

Query: {query}

Chunks:
{chunks}

Provide importance scores in the following JSON format:
{{
    "scores": [
        {{
            "text": "Chunk text",
            "score": 0.85,
            "reason": "Why this chunk is important"
        }}
    ]
}}

Score from 0.0 (not relevant) to 1.0 (highly relevant)."#,
            query = query,
            chunks = chunks_text
        );

        #[derive(Deserialize)]
        struct ScoresResponse {
            scores: Vec<ContextImportance>,
        }

        let response: ScoresResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to score chunks")?;

        Ok(response.scores)
    }
}

/// Automatic context pruner.
pub struct ContextPruner {
    /// Maximum context size in tokens
    max_tokens: usize,
    /// Minimum importance threshold
    min_importance: f64,
}

impl ContextPruner {
    /// Creates a new context pruner.
    pub fn new(max_tokens: usize, min_importance: f64) -> Self {
        Self {
            max_tokens,
            min_importance,
        }
    }

    /// Prunes context based on importance scores and token limits.
    pub fn prune(&self, mut scored_chunks: Vec<ContextImportance>) -> Vec<ContextImportance> {
        // Filter by minimum importance
        scored_chunks.retain(|chunk| chunk.score >= self.min_importance);

        // Sort by importance (descending)
        scored_chunks.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Take chunks until token limit
        let mut total_tokens = 0;
        let mut pruned = Vec::new();

        for chunk in scored_chunks {
            let chunk_tokens = chunk.text.split_whitespace().count();
            if total_tokens + chunk_tokens <= self.max_tokens {
                total_tokens += chunk_tokens;
                pruned.push(chunk);
            } else {
                break;
            }
        }

        pruned
    }

    /// Gets the maximum token limit.
    pub fn max_tokens(&self) -> usize {
        self.max_tokens
    }

    /// Gets the minimum importance threshold.
    pub fn min_importance(&self) -> f64 {
        self.min_importance
    }
}

/// RAG-based context builder for legal documents.
pub struct RagContextBuilder<P> {
    provider: P,
    chunking_strategy: ChunkingStrategy,
    max_context_chunks: usize,
}

impl<P: LLMProvider + Clone> RagContextBuilder<P> {
    /// Creates a new RAG context builder.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            chunking_strategy: ChunkingStrategy::SlidingWindow {
                size: 512,
                overlap: 128,
            },
            max_context_chunks: 5,
        }
    }

    /// Sets the chunking strategy.
    pub fn with_chunking_strategy(mut self, strategy: ChunkingStrategy) -> Self {
        self.chunking_strategy = strategy;
        self
    }

    /// Sets the maximum number of context chunks.
    pub fn with_max_chunks(mut self, max_chunks: usize) -> Self {
        self.max_context_chunks = max_chunks;
        self
    }

    /// Builds context for a query from a document.
    pub async fn build_context(&self, document: &str, query: &str) -> Result<String> {
        // Chunk the document
        let chunks = self.chunk_document(document);

        // Score chunks by relevance
        let scorer = ContextImportanceScorer::new(self.provider.clone());
        let mut scored_chunks = scorer.score_chunks(&chunks, query).await?;

        // Sort by score and take top chunks
        scored_chunks.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        scored_chunks.truncate(self.max_context_chunks);

        // Build context from top chunks
        let context = scored_chunks
            .iter()
            .map(|c| &c.text)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        Ok(context)
    }

    fn chunk_document(&self, document: &str) -> Vec<DocumentChunk> {
        match self.chunking_strategy {
            ChunkingStrategy::FixedSize { size } => self.chunk_fixed_size(document, size),
            ChunkingStrategy::SlidingWindow { size, overlap } => {
                self.chunk_sliding_window(document, size, overlap)
            }
            ChunkingStrategy::Sentences { max_sentences } => {
                self.chunk_by_sentences(document, max_sentences)
            }
            ChunkingStrategy::Paragraphs => self.chunk_by_paragraphs(document),
        }
    }

    fn chunk_fixed_size(&self, document: &str, size: usize) -> Vec<DocumentChunk> {
        let mut chunks = Vec::new();
        for (i, text_chunk) in document
            .chars()
            .collect::<Vec<_>>()
            .chunks(size)
            .enumerate()
        {
            chunks.push(DocumentChunk {
                id: format!("chunk_{}", i),
                content: text_chunk.iter().collect(),
                document_id: "document".to_string(),
                chunk_index: i,
                metadata: None,
                embedding: None,
            });
        }
        chunks
    }

    fn chunk_sliding_window(
        &self,
        document: &str,
        size: usize,
        overlap: usize,
    ) -> Vec<DocumentChunk> {
        let mut chunks = Vec::new();
        let chars: Vec<char> = document.chars().collect();
        let mut idx = 0;
        let mut i = 0;

        while i < chars.len() {
            let end = (i + size).min(chars.len());
            let chunk_text: String = chars[i..end].iter().collect();
            chunks.push(DocumentChunk {
                id: format!("chunk_{}", idx),
                content: chunk_text,
                document_id: "document".to_string(),
                chunk_index: idx,
                metadata: None,
                embedding: None,
            });
            idx += 1;
            i += size - overlap;
        }

        chunks
    }

    fn chunk_by_sentences(&self, document: &str, max_sentences: usize) -> Vec<DocumentChunk> {
        let sentences: Vec<&str> = document.split('.').collect();
        let mut chunks = Vec::new();
        let mut idx = 0;

        for sentences_group in sentences.chunks(max_sentences) {
            let content = sentences_group.join(".");
            if !content.trim().is_empty() {
                chunks.push(DocumentChunk {
                    id: format!("chunk_{}", idx),
                    content,
                    document_id: "document".to_string(),
                    chunk_index: idx,
                    metadata: None,
                    embedding: None,
                });
                idx += 1;
            }
        }

        chunks
    }

    fn chunk_by_paragraphs(&self, document: &str) -> Vec<DocumentChunk> {
        let mut chunks = Vec::new();
        for (idx, paragraph) in document.split("\n\n").enumerate() {
            if !paragraph.trim().is_empty() {
                chunks.push(DocumentChunk {
                    id: format!("chunk_{}", idx),
                    content: paragraph.to_string(),
                    document_id: "document".to_string(),
                    chunk_index: idx,
                    metadata: None,
                    embedding: None,
                });
            }
        }
        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_context() {
        let mut window = SlidingWindowContext::new(100, 20);

        let chunk1 = DocumentChunk {
            id: "chunk1".to_string(),
            content: "First chunk with some text".to_string(),
            document_id: "doc1".to_string(),
            chunk_index: 0,
            metadata: None,
            embedding: None,
        };

        let chunk2 = DocumentChunk {
            id: "chunk2".to_string(),
            content: "Second chunk with more text".to_string(),
            document_id: "doc1".to_string(),
            chunk_index: 1,
            metadata: None,
            embedding: None,
        };

        window.add_chunk(chunk1);
        assert_eq!(window.chunk_count(), 1);

        window.add_chunk(chunk2);
        assert_eq!(window.chunk_count(), 2);

        let content = window.get_window();
        assert!(content.contains("First chunk"));
        assert!(content.contains("Second chunk"));
    }

    #[test]
    fn test_context_pruner() {
        let pruner = ContextPruner::new(100, 0.5);

        let chunks = vec![
            ContextImportance {
                text: "High importance chunk".to_string(),
                score: 0.9,
                reason: "Very relevant".to_string(),
            },
            ContextImportance {
                text: "Low importance chunk".to_string(),
                score: 0.3,
                reason: "Not very relevant".to_string(),
            },
            ContextImportance {
                text: "Medium importance chunk".to_string(),
                score: 0.6,
                reason: "Somewhat relevant".to_string(),
            },
        ];

        let pruned = pruner.prune(chunks);

        // Should filter out the low importance chunk
        assert_eq!(pruned.len(), 2);
        assert!(pruned.iter().all(|c| c.score >= 0.5));
    }

    #[test]
    fn test_summary_node_creation() {
        let node = SummaryNode {
            level: 0,
            summary: "Test summary".to_string(),
            children: Vec::new(),
            original_text: Some("Original text".to_string()),
        };

        assert_eq!(node.level, 0);
        assert!(node.original_text.is_some());
    }
}
