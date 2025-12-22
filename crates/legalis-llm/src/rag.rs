//! Retrieval Augmented Generation (RAG) support.
//!
//! This module provides tools for chunking documents, storing them with embeddings,
//! retrieving relevant context, and augmenting LLM prompts with retrieved information.

use crate::{Embedding, EmbeddingProvider, LLMProvider};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A chunk of a document with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    /// Unique identifier for this chunk
    pub id: String,
    /// The text content
    pub content: String,
    /// Source document ID
    pub document_id: String,
    /// Chunk index within the document
    pub chunk_index: usize,
    /// Optional metadata (title, author, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Embedding vector (if computed)
    #[serde(skip)]
    pub embedding: Option<Embedding>,
}

impl DocumentChunk {
    /// Creates a new document chunk.
    pub fn new(
        id: impl Into<String>,
        content: impl Into<String>,
        document_id: impl Into<String>,
        chunk_index: usize,
    ) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            document_id: document_id.into(),
            chunk_index,
            metadata: None,
            embedding: None,
        }
    }

    /// Adds metadata to the chunk.
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Adds an embedding to the chunk.
    pub fn with_embedding(mut self, embedding: Embedding) -> Self {
        self.embedding = Some(embedding);
        self
    }
}

/// Strategy for chunking documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkingStrategy {
    /// Fixed-size chunks (by character count)
    FixedSize { size: usize },
    /// Sliding window with overlap
    SlidingWindow { size: usize, overlap: usize },
    /// Split by sentences
    Sentences { max_sentences: usize },
    /// Split by paragraphs
    Paragraphs,
}

/// Document chunker.
pub struct DocumentChunker {
    strategy: ChunkingStrategy,
}

impl DocumentChunker {
    /// Creates a new chunker with the given strategy.
    pub fn new(strategy: ChunkingStrategy) -> Self {
        Self { strategy }
    }

    /// Chunks a document into smaller pieces.
    pub fn chunk(&self, document_id: &str, content: &str) -> Vec<DocumentChunk> {
        match self.strategy {
            ChunkingStrategy::FixedSize { size } => {
                self.chunk_fixed_size(document_id, content, size)
            }
            ChunkingStrategy::SlidingWindow { size, overlap } => {
                self.chunk_sliding_window(document_id, content, size, overlap)
            }
            ChunkingStrategy::Sentences { max_sentences } => {
                self.chunk_sentences(document_id, content, max_sentences)
            }
            ChunkingStrategy::Paragraphs => self.chunk_paragraphs(document_id, content),
        }
    }

    fn chunk_fixed_size(
        &self,
        document_id: &str,
        content: &str,
        size: usize,
    ) -> Vec<DocumentChunk> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut chunk_index = 0;

        while start < content.len() {
            let end = (start + size).min(content.len());
            let chunk_content = &content[start..end];

            if !chunk_content.trim().is_empty() {
                let chunk = DocumentChunk::new(
                    format!("{}-chunk-{}", document_id, chunk_index),
                    chunk_content,
                    document_id,
                    chunk_index,
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            start = end;
        }

        chunks
    }

    fn chunk_sliding_window(
        &self,
        document_id: &str,
        content: &str,
        size: usize,
        overlap: usize,
    ) -> Vec<DocumentChunk> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut chunk_index = 0;
        let step = size.saturating_sub(overlap);

        while start < content.len() {
            let end = (start + size).min(content.len());
            let chunk_content = &content[start..end];

            if !chunk_content.trim().is_empty() {
                let chunk = DocumentChunk::new(
                    format!("{}-chunk-{}", document_id, chunk_index),
                    chunk_content,
                    document_id,
                    chunk_index,
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            if end >= content.len() {
                break;
            }

            start += step;
        }

        chunks
    }

    fn chunk_sentences(
        &self,
        document_id: &str,
        content: &str,
        max_sentences: usize,
    ) -> Vec<DocumentChunk> {
        let sentences = Self::split_sentences(content);
        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        for chunk_sentences in sentences.chunks(max_sentences) {
            let chunk_content = chunk_sentences.join(" ");
            if !chunk_content.trim().is_empty() {
                let chunk = DocumentChunk::new(
                    format!("{}-chunk-{}", document_id, chunk_index),
                    chunk_content,
                    document_id,
                    chunk_index,
                );
                chunks.push(chunk);
                chunk_index += 1;
            }
        }

        chunks
    }

    fn chunk_paragraphs(&self, document_id: &str, content: &str) -> Vec<DocumentChunk> {
        let paragraphs: Vec<&str> = content.split("\n\n").collect();
        let mut chunks = Vec::new();

        for (chunk_index, paragraph) in paragraphs.iter().enumerate() {
            if !paragraph.trim().is_empty() {
                let chunk = DocumentChunk::new(
                    format!("{}-chunk-{}", document_id, chunk_index),
                    *paragraph,
                    document_id,
                    chunk_index,
                );
                chunks.push(chunk);
            }
        }

        chunks
    }

    fn split_sentences(text: &str) -> Vec<String> {
        // Simple sentence splitting (can be improved with NLP libraries)
        let mut sentences = Vec::new();
        let mut current = String::new();

        for c in text.chars() {
            current.push(c);
            if matches!(c, '.' | '!' | '?') {
                sentences.push(current.trim().to_string());
                current.clear();
            }
        }

        if !current.trim().is_empty() {
            sentences.push(current.trim().to_string());
        }

        sentences
    }
}

/// A retrieved chunk with its relevance score.
#[derive(Debug, Clone)]
pub struct RetrievedChunk {
    /// The document chunk
    pub chunk: DocumentChunk,
    /// Relevance score (0.0 - 1.0, higher is more relevant)
    pub score: f32,
}

/// Document store abstraction for RAG.
#[async_trait::async_trait]
pub trait DocumentStore: Send + Sync {
    /// Stores a document chunk with its embedding.
    async fn store(&self, chunk: DocumentChunk) -> Result<()>;

    /// Stores multiple chunks at once.
    async fn store_batch(&self, chunks: Vec<DocumentChunk>) -> Result<()> {
        for chunk in chunks {
            self.store(chunk).await?;
        }
        Ok(())
    }

    /// Retrieves the most relevant chunks for a query.
    async fn retrieve(
        &self,
        query_embedding: &Embedding,
        top_k: usize,
    ) -> Result<Vec<RetrievedChunk>>;

    /// Deletes all chunks for a document.
    async fn delete_document(&self, document_id: &str) -> Result<()>;

    /// Returns the total number of stored chunks.
    async fn count(&self) -> Result<usize>;
}

/// In-memory document store.
#[derive(Default)]
pub struct InMemoryDocumentStore {
    chunks: Arc<tokio::sync::RwLock<Vec<DocumentChunk>>>,
}

impl InMemoryDocumentStore {
    /// Creates a new in-memory document store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl DocumentStore for InMemoryDocumentStore {
    async fn store(&self, chunk: DocumentChunk) -> Result<()> {
        let mut chunks = self.chunks.write().await;
        chunks.push(chunk);
        Ok(())
    }

    async fn retrieve(
        &self,
        query_embedding: &Embedding,
        top_k: usize,
    ) -> Result<Vec<RetrievedChunk>> {
        let chunks = self.chunks.read().await;
        let mut scored_chunks = Vec::new();

        for chunk in chunks.iter() {
            if let Some(ref chunk_embedding) = chunk.embedding {
                let score = query_embedding.cosine_similarity(chunk_embedding)?;
                scored_chunks.push(RetrievedChunk {
                    chunk: chunk.clone(),
                    score,
                });
            }
        }

        // Sort by score descending
        scored_chunks.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        Ok(scored_chunks.into_iter().take(top_k).collect())
    }

    async fn delete_document(&self, document_id: &str) -> Result<()> {
        let mut chunks = self.chunks.write().await;
        chunks.retain(|c| c.document_id != document_id);
        Ok(())
    }

    async fn count(&self) -> Result<usize> {
        let chunks = self.chunks.read().await;
        Ok(chunks.len())
    }
}

/// RAG pipeline configuration.
#[derive(Debug, Clone)]
pub struct RAGConfig {
    /// Number of chunks to retrieve
    pub top_k: usize,
    /// Minimum similarity score to include (0.0 - 1.0)
    pub min_score: f32,
    /// Whether to include chunk metadata in context
    pub include_metadata: bool,
    /// Maximum total characters in retrieved context
    pub max_context_length: usize,
}

impl Default for RAGConfig {
    fn default() -> Self {
        Self {
            top_k: 5,
            min_score: 0.3,
            include_metadata: true,
            max_context_length: 2000,
        }
    }
}

impl RAGConfig {
    /// Creates a new RAG configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of chunks to retrieve.
    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }

    /// Sets the minimum similarity score.
    pub fn with_min_score(mut self, score: f32) -> Self {
        self.min_score = score.clamp(0.0, 1.0);
        self
    }

    /// Sets whether to include metadata.
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// Sets maximum context length.
    pub fn with_max_context_length(mut self, length: usize) -> Self {
        self.max_context_length = length;
        self
    }
}

/// RAG pipeline for retrieval-augmented generation.
pub struct RAGPipeline<E, S, L> {
    embedding_provider: E,
    document_store: Arc<S>,
    llm_provider: L,
    config: RAGConfig,
}

impl<E, S, L> RAGPipeline<E, S, L>
where
    E: EmbeddingProvider,
    S: DocumentStore,
    L: LLMProvider,
{
    /// Creates a new RAG pipeline.
    pub fn new(
        embedding_provider: E,
        document_store: Arc<S>,
        llm_provider: L,
        config: RAGConfig,
    ) -> Self {
        Self {
            embedding_provider,
            document_store,
            llm_provider,
            config,
        }
    }

    /// Indexes a document by chunking and storing it.
    pub async fn index_document(
        &self,
        document_id: &str,
        content: &str,
        chunking_strategy: ChunkingStrategy,
    ) -> Result<usize> {
        let chunker = DocumentChunker::new(chunking_strategy);
        let mut chunks = chunker.chunk(document_id, content);

        // Generate embeddings for all chunks
        for chunk in &mut chunks {
            let embedding = self.embedding_provider.embed(&chunk.content).await?;
            chunk.embedding = Some(embedding);
        }

        let chunk_count = chunks.len();
        self.document_store.store_batch(chunks).await?;

        Ok(chunk_count)
    }

    /// Retrieves relevant context for a query.
    pub async fn retrieve(&self, query: &str) -> Result<Vec<RetrievedChunk>> {
        let query_embedding = self.embedding_provider.embed(query).await?;
        let mut retrieved = self
            .document_store
            .retrieve(&query_embedding, self.config.top_k)
            .await?;

        // Filter by minimum score
        retrieved.retain(|r| r.score >= self.config.min_score);

        // Limit total context length
        let mut total_length = 0;
        retrieved.retain(|r| {
            total_length += r.chunk.content.len();
            total_length <= self.config.max_context_length
        });

        Ok(retrieved)
    }

    /// Generates a response using RAG (retrieval + generation).
    pub async fn generate(&self, query: &str) -> Result<String> {
        let retrieved = self.retrieve(query).await?;

        if retrieved.is_empty() {
            // No relevant context found, use query directly
            return self.llm_provider.generate_text(query).await;
        }

        // Build context from retrieved chunks
        let context = self.build_context(&retrieved);

        // Augment prompt with context
        let augmented_prompt = format!(
            "Context:\n{}\n\nQuestion: {}\n\nAnswer based on the context above:",
            context, query
        );

        self.llm_provider.generate_text(&augmented_prompt).await
    }

    /// Builds context string from retrieved chunks.
    fn build_context(&self, chunks: &[RetrievedChunk]) -> String {
        chunks
            .iter()
            .enumerate()
            .map(|(i, retrieved)| {
                let mut context = format!("[{}] {}", i + 1, retrieved.chunk.content);

                if self.config.include_metadata {
                    if let Some(ref metadata) = retrieved.chunk.metadata {
                        context.push_str(&format!("\n(Metadata: {})", metadata));
                    }
                }

                context
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Returns statistics about the document store.
    pub async fn stats(&self) -> Result<RAGStats> {
        let total_chunks = self.document_store.count().await?;
        Ok(RAGStats { total_chunks })
    }
}

/// Statistics about the RAG system.
#[derive(Debug, Clone)]
pub struct RAGStats {
    /// Total number of chunks in the store
    pub total_chunks: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_fixed_size() {
        let chunker = DocumentChunker::new(ChunkingStrategy::FixedSize { size: 20 });
        let chunks = chunker.chunk("doc1", "This is a test document with some content.");

        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|c| c.content.len() <= 20));
    }

    #[test]
    fn test_chunk_sliding_window() {
        let chunker = DocumentChunker::new(ChunkingStrategy::SlidingWindow {
            size: 20,
            overlap: 5,
        });
        let chunks = chunker.chunk("doc1", "This is a test document with some content.");

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_chunk_paragraphs() {
        let chunker = DocumentChunker::new(ChunkingStrategy::Paragraphs);
        let content = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let chunks = chunker.chunk("doc1", content);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].chunk_index, 0);
        assert_eq!(chunks[1].chunk_index, 1);
        assert_eq!(chunks[2].chunk_index, 2);
    }

    #[test]
    fn test_chunk_sentences() {
        let chunker = DocumentChunker::new(ChunkingStrategy::Sentences { max_sentences: 2 });
        let content = "First sentence. Second sentence. Third sentence. Fourth sentence.";
        let chunks = chunker.chunk("doc1", content);

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_document_chunk_creation() {
        let chunk = DocumentChunk::new("chunk1", "Test content", "doc1", 0);
        assert_eq!(chunk.id, "chunk1");
        assert_eq!(chunk.content, "Test content");
        assert_eq!(chunk.document_id, "doc1");
        assert_eq!(chunk.chunk_index, 0);
    }

    #[test]
    fn test_document_chunk_with_metadata() {
        let metadata = serde_json::json!({"author": "Test Author"});
        let chunk =
            DocumentChunk::new("chunk1", "Content", "doc1", 0).with_metadata(metadata.clone());
        assert_eq!(chunk.metadata, Some(metadata));
    }

    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryDocumentStore::new();

        let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);
        let chunk =
            DocumentChunk::new("chunk1", "Test", "doc1", 0).with_embedding(embedding.clone());

        store.store(chunk).await.unwrap();
        assert_eq!(store.count().await.unwrap(), 1);

        let query_embedding = Embedding::new(vec![0.1, 0.2, 0.3]);
        let retrieved = store.retrieve(&query_embedding, 5).await.unwrap();
        assert_eq!(retrieved.len(), 1);

        store.delete_document("doc1").await.unwrap();
        assert_eq!(store.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_rag_pipeline_stats() {
        // Test without requiring actual embedding provider
        let store = Arc::new(InMemoryDocumentStore::new());

        let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);
        let chunk = DocumentChunk::new("chunk1", "Test", "doc1", 0).with_embedding(embedding);

        store.store(chunk).await.unwrap();
        let count = store.count().await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_rag_config() {
        let config = RAGConfig::new()
            .with_top_k(10)
            .with_min_score(0.5)
            .with_metadata(false)
            .with_max_context_length(1000);

        assert_eq!(config.top_k, 10);
        assert!((config.min_score - 0.5).abs() < f32::EPSILON);
        assert!(!config.include_metadata);
        assert_eq!(config.max_context_length, 1000);
    }
}
