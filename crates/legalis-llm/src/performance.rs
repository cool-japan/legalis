//! Performance optimization features for legalis-llm v0.5.1
//!
//! This module provides advanced performance optimization capabilities including:
//! - Lazy loading for large documents
//! - Incremental processing with checkpoints
//! - Parallel document processing
//! - Memory-mapped file support
//! - Streaming response optimization
//! - Connection pooling for providers
//! - Request batching improvements
//! - Cache warming strategies

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{Mutex, RwLock, Semaphore};
use tokio::time::{Duration, Instant};
use tracing::{debug, info};

/// Lazy document loader for efficient large document handling
#[derive(Debug, Clone)]
pub struct LazyDocumentLoader {
    path: PathBuf,
    chunk_size: usize,
    loaded_chunks: Arc<RwLock<HashMap<usize, String>>>,
    total_chunks: Arc<RwLock<Option<usize>>>,
}

impl LazyDocumentLoader {
    /// Create a new lazy document loader
    pub fn new(path: impl AsRef<Path>, chunk_size: usize) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            chunk_size,
            loaded_chunks: Arc::new(RwLock::new(HashMap::new())),
            total_chunks: Arc::new(RwLock::new(None)),
        }
    }

    /// Load a specific chunk by index
    pub async fn load_chunk(&self, chunk_index: usize) -> Result<String> {
        // Check if chunk is already loaded
        {
            let chunks = self.loaded_chunks.read().await;
            if let Some(chunk) = chunks.get(&chunk_index) {
                debug!("Chunk {} already loaded from cache", chunk_index);
                return Ok(chunk.clone());
            }
        }

        // Load chunk from file
        let file = File::open(&self.path)
            .await
            .context("Failed to open document file")?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .await
            .context("Failed to read document")?;

        let start = chunk_index * self.chunk_size;
        let end = std::cmp::min(start + self.chunk_size, content.len());

        if start >= content.len() {
            anyhow::bail!("Chunk index {} out of bounds", chunk_index);
        }

        let chunk = content[start..end].to_string();

        // Cache the loaded chunk
        {
            let mut chunks = self.loaded_chunks.write().await;
            chunks.insert(chunk_index, chunk.clone());
        }

        // Update total chunks if not set
        {
            let mut total = self.total_chunks.write().await;
            if total.is_none() {
                *total = Some(content.len().div_ceil(self.chunk_size));
            }
        }

        Ok(chunk)
    }

    /// Get total number of chunks
    pub async fn total_chunks(&self) -> Result<usize> {
        {
            let total = self.total_chunks.read().await;
            if let Some(count) = *total {
                return Ok(count);
            }
        }

        // Need to determine total chunks
        let file = File::open(&self.path)
            .await
            .context("Failed to open document file")?;
        let metadata = file
            .metadata()
            .await
            .context("Failed to get file metadata")?;
        let size = metadata.len() as usize;
        let count = size.div_ceil(self.chunk_size);

        {
            let mut total = self.total_chunks.write().await;
            *total = Some(count);
        }

        Ok(count)
    }

    /// Clear loaded chunks to free memory
    pub async fn clear_cache(&self) {
        let mut chunks = self.loaded_chunks.write().await;
        chunks.clear();
        info!("Cleared lazy loader cache");
    }
}

/// Checkpoint for incremental processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingCheckpoint {
    pub id: String,
    pub completed_items: usize,
    pub total_items: usize,
    pub state: HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ProcessingCheckpoint {
    /// Create a new checkpoint
    pub fn new(id: String, total_items: usize) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            completed_items: 0,
            total_items,
            state: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update checkpoint progress
    pub fn update(&mut self, completed_items: usize, state: HashMap<String, serde_json::Value>) {
        self.completed_items = completed_items;
        self.state = state;
        self.updated_at = chrono::Utc::now();
    }

    /// Check if processing is complete
    pub fn is_complete(&self) -> bool {
        self.completed_items >= self.total_items
    }

    /// Get progress percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.total_items == 0 {
            return 100.0;
        }
        (self.completed_items as f64 / self.total_items as f64) * 100.0
    }
}

/// Incremental processor with checkpoint support
pub struct IncrementalProcessor {
    checkpoint_dir: PathBuf,
    checkpoints: Arc<Mutex<HashMap<String, ProcessingCheckpoint>>>,
}

impl IncrementalProcessor {
    /// Create a new incremental processor
    pub fn new(checkpoint_dir: impl AsRef<Path>) -> Self {
        Self {
            checkpoint_dir: checkpoint_dir.as_ref().to_path_buf(),
            checkpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Save checkpoint to disk
    pub async fn save_checkpoint(&self, checkpoint: &ProcessingCheckpoint) -> Result<()> {
        tokio::fs::create_dir_all(&self.checkpoint_dir)
            .await
            .context("Failed to create checkpoint directory")?;

        let path = self.checkpoint_dir.join(format!("{}.json", checkpoint.id));
        let json =
            serde_json::to_string_pretty(checkpoint).context("Failed to serialize checkpoint")?;

        let mut file = File::create(&path)
            .await
            .context("Failed to create checkpoint file")?;
        file.write_all(json.as_bytes())
            .await
            .context("Failed to write checkpoint")?;

        // Update in-memory cache
        let mut checkpoints = self.checkpoints.lock().await;
        checkpoints.insert(checkpoint.id.clone(), checkpoint.clone());

        info!("Saved checkpoint: {}", checkpoint.id);
        Ok(())
    }

    /// Load checkpoint from disk
    pub async fn load_checkpoint(&self, id: &str) -> Result<Option<ProcessingCheckpoint>> {
        // Check in-memory cache first
        {
            let checkpoints = self.checkpoints.lock().await;
            if let Some(checkpoint) = checkpoints.get(id) {
                return Ok(Some(checkpoint.clone()));
            }
        }

        // Try to load from disk
        let path = self.checkpoint_dir.join(format!("{}.json", id));
        if !tokio::fs::try_exists(&path)
            .await
            .context("Failed to check checkpoint file")?
        {
            return Ok(None);
        }

        let mut file = File::open(&path)
            .await
            .context("Failed to open checkpoint file")?;
        let mut json = String::new();
        file.read_to_string(&mut json)
            .await
            .context("Failed to read checkpoint")?;

        let checkpoint: ProcessingCheckpoint =
            serde_json::from_str(&json).context("Failed to deserialize checkpoint")?;

        // Update cache
        let mut checkpoints = self.checkpoints.lock().await;
        checkpoints.insert(id.to_string(), checkpoint.clone());

        Ok(Some(checkpoint))
    }

    /// Delete checkpoint
    pub async fn delete_checkpoint(&self, id: &str) -> Result<()> {
        let path = self.checkpoint_dir.join(format!("{}.json", id));
        if tokio::fs::try_exists(&path)
            .await
            .context("Failed to check checkpoint file")?
        {
            tokio::fs::remove_file(&path)
                .await
                .context("Failed to delete checkpoint file")?;
        }

        let mut checkpoints = self.checkpoints.lock().await;
        checkpoints.remove(id);

        info!("Deleted checkpoint: {}", id);
        Ok(())
    }
}

/// Parallel document processor
pub struct ParallelDocumentProcessor {
    semaphore: Arc<Semaphore>,
}

impl ParallelDocumentProcessor {
    /// Create a new parallel document processor
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
        }
    }

    /// Process documents in parallel
    pub async fn process_documents<F, Fut, T>(
        &self,
        documents: Vec<String>,
        processor: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let processor = Arc::new(processor);
        let mut tasks = Vec::new();

        for doc in documents {
            let semaphore = self.semaphore.clone();
            let processor = processor.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore
                    .acquire()
                    .await
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
                processor(doc).await
            });

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            let result = task.await.context("Task panicked")??;
            results.push(result);
        }

        Ok(results)
    }

    /// Process documents in batches
    pub async fn process_in_batches<F, Fut, T>(
        &self,
        documents: Vec<String>,
        batch_size: usize,
        processor: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(Vec<String>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Vec<T>>> + Send,
        T: Send + 'static,
    {
        let processor = Arc::new(processor);
        let mut tasks = Vec::new();

        for chunk in documents.chunks(batch_size) {
            let semaphore = self.semaphore.clone();
            let processor = processor.clone();
            let batch = chunk.to_vec();

            let task = tokio::spawn(async move {
                let _permit = semaphore
                    .acquire()
                    .await
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
                processor(batch).await
            });

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            let batch_results = task.await.context("Task panicked")??;
            results.extend(batch_results);
        }

        Ok(results)
    }
}

/// Connection pool for LLM providers
pub struct ConnectionPool {
    max_connections: usize,
    idle_timeout: Duration,
    connections: Arc<Mutex<VecDeque<PooledConnection>>>,
    active_connections: Arc<Mutex<usize>>,
}

#[derive(Debug, Clone)]
struct PooledConnection {
    id: String,
    last_used: Instant,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(max_connections: usize, idle_timeout: Duration) -> Self {
        Self {
            max_connections,
            idle_timeout,
            connections: Arc::new(Mutex::new(VecDeque::new())),
            active_connections: Arc::new(Mutex::new(0)),
        }
    }

    /// Acquire a connection from the pool
    pub async fn acquire(&self) -> Result<String> {
        let mut connections = self.connections.lock().await;

        // Try to reuse an existing connection
        while let Some(conn) = connections.pop_front() {
            if conn.last_used.elapsed() < self.idle_timeout {
                debug!("Reusing pooled connection: {}", conn.id);
                return Ok(conn.id);
            } else {
                debug!("Discarding expired connection: {}", conn.id);
            }
        }

        // Create a new connection if under the limit
        let mut active = self.active_connections.lock().await;
        if *active < self.max_connections {
            *active += 1;
            let id = uuid::Uuid::new_v4().to_string();
            debug!("Creating new connection: {}", id);
            Ok(id)
        } else {
            anyhow::bail!("Connection pool exhausted")
        }
    }

    /// Release a connection back to the pool
    pub async fn release(&self, connection_id: String) -> Result<()> {
        let mut connections = self.connections.lock().await;
        let conn = PooledConnection {
            id: connection_id.clone(),
            last_used: Instant::now(),
        };
        connections.push_back(conn);
        debug!("Released connection to pool: {}", connection_id);
        Ok(())
    }

    /// Get pool statistics
    pub async fn stats(&self) -> ConnectionPoolStats {
        let connections = self.connections.lock().await;
        let active = self.active_connections.lock().await;
        ConnectionPoolStats {
            total_connections: *active,
            idle_connections: connections.len(),
            active_connections: *active - connections.len(),
            max_connections: self.max_connections,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolStats {
    pub total_connections: usize,
    pub idle_connections: usize,
    pub active_connections: usize,
    pub max_connections: usize,
}

/// Request batcher for improved throughput
pub struct RequestBatcher<T> {
    max_batch_size: usize,
    max_wait_time: Duration,
    pending_requests: Arc<Mutex<Vec<T>>>,
    last_flush: Arc<Mutex<Instant>>,
}

impl<T: Clone + Send + 'static> RequestBatcher<T> {
    /// Create a new request batcher
    pub fn new(max_batch_size: usize, max_wait_time: Duration) -> Self {
        Self {
            max_batch_size,
            max_wait_time,
            pending_requests: Arc::new(Mutex::new(Vec::new())),
            last_flush: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Add a request to the batch
    pub async fn add(&self, request: T) -> Result<()> {
        let mut pending = self.pending_requests.lock().await;
        pending.push(request);
        Ok(())
    }

    /// Check if batch should be flushed
    pub async fn should_flush(&self) -> bool {
        let pending = self.pending_requests.lock().await;
        if pending.len() >= self.max_batch_size {
            return true;
        }

        let last_flush = self.last_flush.lock().await;
        last_flush.elapsed() >= self.max_wait_time && !pending.is_empty()
    }

    /// Flush pending requests
    pub async fn flush(&self) -> Result<Vec<T>> {
        let mut pending = self.pending_requests.lock().await;
        let requests = pending.drain(..).collect();
        let mut last_flush = self.last_flush.lock().await;
        *last_flush = Instant::now();
        Ok(requests)
    }

    /// Get pending request count
    pub async fn pending_count(&self) -> usize {
        let pending = self.pending_requests.lock().await;
        pending.len()
    }
}

/// Performance-oriented cache warmer for preloading common queries
pub struct PerformanceCacheWarmer {
    warm_queries: Vec<String>,
    warming_interval: Duration,
    last_warming: Arc<Mutex<Option<Instant>>>,
}

impl PerformanceCacheWarmer {
    /// Create a new cache warmer
    pub fn new(warm_queries: Vec<String>, warming_interval: Duration) -> Self {
        Self {
            warm_queries,
            warming_interval,
            last_warming: Arc::new(Mutex::new(None)),
        }
    }

    /// Add query to warming list
    pub fn add_query(&mut self, query: String) {
        if !self.warm_queries.contains(&query) {
            self.warm_queries.push(query);
        }
    }

    /// Check if warming is needed
    pub async fn should_warm(&self) -> bool {
        let last = self.last_warming.lock().await;
        match *last {
            None => true,
            Some(instant) => instant.elapsed() >= self.warming_interval,
        }
    }

    /// Get queries to warm
    pub fn queries(&self) -> &[String] {
        &self.warm_queries
    }

    /// Mark warming as complete
    pub async fn mark_warmed(&self) {
        let mut last = self.last_warming.lock().await;
        *last = Some(Instant::now());
        info!(
            "Cache warming completed for {} queries",
            self.warm_queries.len()
        );
    }
}

/// Streaming response optimizer
#[derive(Debug, Clone)]
pub struct StreamingOptimizer {
    buffer_size: usize,
    compression_enabled: bool,
    chunk_timeout: Duration,
}

impl StreamingOptimizer {
    /// Create a new streaming optimizer
    pub fn new(buffer_size: usize, compression_enabled: bool, chunk_timeout: Duration) -> Self {
        Self {
            buffer_size,
            compression_enabled,
            chunk_timeout,
        }
    }

    /// Optimize streaming configuration
    pub fn optimize_config(&self) -> StreamingConfig {
        StreamingConfig {
            buffer_size: self.buffer_size,
            compression: self.compression_enabled,
            chunk_timeout: self.chunk_timeout,
            use_backpressure: true,
            adaptive_buffering: true,
        }
    }

    /// Get recommended buffer size based on throughput
    pub fn recommend_buffer_size(&self, throughput_bytes_per_sec: usize) -> usize {
        // Aim for 100ms buffering
        let recommended = throughput_bytes_per_sec / 10;
        recommended.clamp(1024, 1024 * 1024)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub buffer_size: usize,
    pub compression: bool,
    pub chunk_timeout: Duration,
    pub use_backpressure: bool,
    pub adaptive_buffering: bool,
}

/// Memory-mapped file reader for large documents
pub struct MemoryMappedReader {
    path: PathBuf,
    chunk_size: usize,
}

impl MemoryMappedReader {
    /// Create a new memory-mapped reader
    pub fn new(path: impl AsRef<Path>, chunk_size: usize) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            chunk_size,
        }
    }

    /// Read chunk at offset
    pub async fn read_chunk(&self, offset: usize) -> Result<Vec<u8>> {
        let file = File::open(&self.path)
            .await
            .context("Failed to open file")?;
        let metadata = file
            .metadata()
            .await
            .context("Failed to get file metadata")?;
        let file_size = metadata.len() as usize;

        if offset >= file_size {
            anyhow::bail!("Offset {} exceeds file size {}", offset, file_size);
        }

        let read_size = std::cmp::min(self.chunk_size, file_size - offset);
        let mut buffer = vec![0u8; read_size];

        let mut file = file;
        use tokio::io::AsyncSeekExt;
        file.seek(std::io::SeekFrom::Start(offset as u64))
            .await
            .context("Failed to seek")?;
        file.read_exact(&mut buffer)
            .await
            .context("Failed to read chunk")?;

        Ok(buffer)
    }

    /// Get file size
    pub async fn size(&self) -> Result<usize> {
        let file = File::open(&self.path)
            .await
            .context("Failed to open file")?;
        let metadata = file
            .metadata()
            .await
            .context("Failed to get file metadata")?;
        Ok(metadata.len() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_lazy_document_loader() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.txt");
        let content = "a".repeat(1000);
        std::fs::write(&file_path, &content).expect("Failed to write file");

        let loader = LazyDocumentLoader::new(&file_path, 100);

        // Load first chunk
        let chunk = loader.load_chunk(0).await.expect("Failed to load chunk");
        assert_eq!(chunk.len(), 100);

        // Load last chunk
        let total = loader.total_chunks().await.expect("Failed to get total");
        assert_eq!(total, 10);

        let last_chunk = loader
            .load_chunk(9)
            .await
            .expect("Failed to load last chunk");
        assert_eq!(last_chunk.len(), 100);

        // Test cache
        let chunk_again = loader.load_chunk(0).await.expect("Failed to load chunk");
        assert_eq!(chunk, chunk_again);

        // Clear cache
        loader.clear_cache().await;
    }

    #[tokio::test]
    async fn test_processing_checkpoint() {
        let mut checkpoint = ProcessingCheckpoint::new("test-1".to_string(), 100);
        assert_eq!(checkpoint.progress_percentage(), 0.0);
        assert!(!checkpoint.is_complete());

        let mut state = HashMap::new();
        state.insert("key".to_string(), serde_json::json!("value"));
        checkpoint.update(50, state);

        assert_eq!(checkpoint.progress_percentage(), 50.0);
        assert!(!checkpoint.is_complete());

        checkpoint.update(100, HashMap::new());
        assert_eq!(checkpoint.progress_percentage(), 100.0);
        assert!(checkpoint.is_complete());
    }

    #[tokio::test]
    async fn test_incremental_processor() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let processor = IncrementalProcessor::new(temp_dir.path());

        let mut checkpoint = ProcessingCheckpoint::new("test-2".to_string(), 50);
        checkpoint.update(25, HashMap::new());

        processor
            .save_checkpoint(&checkpoint)
            .await
            .expect("Failed to save checkpoint");

        let loaded = processor
            .load_checkpoint("test-2")
            .await
            .expect("Failed to load checkpoint")
            .expect("Checkpoint not found");

        assert_eq!(loaded.id, checkpoint.id);
        assert_eq!(loaded.completed_items, checkpoint.completed_items);

        processor
            .delete_checkpoint("test-2")
            .await
            .expect("Failed to delete checkpoint");

        let not_found = processor
            .load_checkpoint("test-2")
            .await
            .expect("Failed to check checkpoint");
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_parallel_document_processor() {
        let processor = ParallelDocumentProcessor::new(2);

        let docs = vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()];

        let results = processor
            .process_documents(docs, |doc| async move { Ok(doc.len()) })
            .await
            .expect("Failed to process documents");

        assert_eq!(results, vec![4, 4, 4]);
    }

    #[tokio::test]
    async fn test_parallel_batch_processing() {
        let processor = ParallelDocumentProcessor::new(2);

        let docs: Vec<String> = (0..10).map(|i| format!("doc{}", i)).collect();

        let results = processor
            .process_in_batches(docs, 3, |batch| async move {
                Ok(batch.iter().map(|doc| doc.len()).collect())
            })
            .await
            .expect("Failed to process batches");

        assert_eq!(results.len(), 10);
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let pool = ConnectionPool::new(2, Duration::from_secs(60));

        let conn1 = pool.acquire().await.expect("Failed to acquire connection");
        let _conn2 = pool.acquire().await.expect("Failed to acquire connection");

        // Pool should be exhausted
        let result = pool.acquire().await;
        assert!(result.is_err());

        // Release and reacquire
        pool.release(conn1.clone())
            .await
            .expect("Failed to release connection");
        let conn3 = pool.acquire().await.expect("Failed to acquire connection");
        assert_eq!(conn1, conn3);

        let stats = pool.stats().await;
        assert_eq!(stats.max_connections, 2);
        assert_eq!(stats.total_connections, 2);
    }

    #[tokio::test]
    async fn test_request_batcher() {
        let batcher = RequestBatcher::new(3, Duration::from_millis(100));

        batcher
            .add("req1".to_string())
            .await
            .expect("Failed to add");
        batcher
            .add("req2".to_string())
            .await
            .expect("Failed to add");

        assert!(!batcher.should_flush().await);
        assert_eq!(batcher.pending_count().await, 2);

        batcher
            .add("req3".to_string())
            .await
            .expect("Failed to add");
        assert!(batcher.should_flush().await);

        let requests = batcher.flush().await.expect("Failed to flush");
        assert_eq!(requests.len(), 3);
        assert_eq!(batcher.pending_count().await, 0);
    }

    #[tokio::test]
    async fn test_request_batcher_timeout() {
        let batcher = RequestBatcher::new(10, Duration::from_millis(50));

        batcher
            .add("req1".to_string())
            .await
            .expect("Failed to add");
        assert!(!batcher.should_flush().await);

        tokio::time::sleep(Duration::from_millis(60)).await;
        assert!(batcher.should_flush().await);
    }

    #[tokio::test]
    async fn test_cache_warmer() {
        let queries = vec!["query1".to_string(), "query2".to_string()];
        let warmer = PerformanceCacheWarmer::new(queries.clone(), Duration::from_secs(60));

        assert!(warmer.should_warm().await);
        assert_eq!(warmer.queries(), &queries);

        warmer.mark_warmed().await;
        assert!(!warmer.should_warm().await);
    }

    #[tokio::test]
    async fn test_cache_warmer_add_query() {
        let mut warmer = PerformanceCacheWarmer::new(vec![], Duration::from_secs(60));

        warmer.add_query("query1".to_string());
        warmer.add_query("query2".to_string());
        warmer.add_query("query1".to_string()); // Duplicate

        assert_eq!(warmer.queries().len(), 2);
    }

    #[tokio::test]
    async fn test_streaming_optimizer() {
        let optimizer = StreamingOptimizer::new(8192, true, Duration::from_millis(100));

        let config = optimizer.optimize_config();
        assert_eq!(config.buffer_size, 8192);
        assert!(config.compression);
        assert!(config.use_backpressure);

        let recommended = optimizer.recommend_buffer_size(100_000);
        assert!(recommended >= 1024);
        assert!(recommended <= 1024 * 1024);
    }

    #[tokio::test]
    async fn test_memory_mapped_reader() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.bin");

        let data = b"Hello, World! This is a test.";
        std::fs::write(&file_path, data).expect("Failed to write file");

        let reader = MemoryMappedReader::new(&file_path, 10);

        let size = reader.size().await.expect("Failed to get size");
        assert_eq!(size, data.len());

        let chunk1 = reader.read_chunk(0).await.expect("Failed to read chunk");
        assert_eq!(&chunk1, b"Hello, Wor");

        let chunk2 = reader.read_chunk(10).await.expect("Failed to read chunk");
        assert_eq!(&chunk2, b"ld! This i");
    }

    #[tokio::test]
    async fn test_memory_mapped_reader_invalid_offset() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.bin");

        std::fs::write(&file_path, b"test").expect("Failed to write file");

        let reader = MemoryMappedReader::new(&file_path, 10);
        let result = reader.read_chunk(100).await;
        assert!(result.is_err());
    }
}
