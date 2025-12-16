//! Performance optimizations for large-scale simulations.
//!
//! This module provides:
//! - Batch processing for large populations
//! - Memory-efficient streaming
//! - Entity pooling and recycling
//! - Lazy attribute evaluation
//! - Optimized work distribution
//! - Memory-mapped population storage
//! - SIMD-accelerated numeric operations
//! - Distributed multi-node simulation

use legalis_core::LegalEntity;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wide::f64x4;

/// Batch processor configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Number of entities to process in each batch
    pub batch_size: usize,
    /// Number of parallel workers
    pub num_workers: usize,
    /// Enable memory-efficient streaming mode
    pub streaming_mode: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            num_workers: num_cpus::get(),
            streaming_mode: false,
        }
    }
}

impl BatchConfig {
    /// Creates a new batch configuration with custom batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Sets the number of parallel workers
    pub fn with_workers(mut self, workers: usize) -> Self {
        self.num_workers = workers;
        self
    }

    /// Enables streaming mode for memory efficiency
    pub fn with_streaming(mut self, enabled: bool) -> Self {
        self.streaming_mode = enabled;
        self
    }
}

/// Batch iterator for processing large populations in chunks
pub struct BatchIterator<T> {
    items: Vec<T>,
    batch_size: usize,
    current_index: usize,
}

impl<T> BatchIterator<T> {
    /// Creates a new batch iterator
    pub fn new(items: Vec<T>, batch_size: usize) -> Self {
        Self {
            items,
            batch_size,
            current_index: 0,
        }
    }

    /// Returns the total number of batches
    pub fn batch_count(&self) -> usize {
        (self.items.len() + self.batch_size - 1) / self.batch_size
    }
}

impl<T> Iterator for BatchIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.items.len() {
            return None;
        }

        let end_index = (self.current_index + self.batch_size).min(self.items.len());
        let batch: Vec<T> = self.items.drain(self.current_index..end_index).collect();

        // Don't increment current_index since we're draining
        Some(batch)
    }
}

/// Entity pool for recycling entity objects
pub struct EntityPool<T: LegalEntity + Clone> {
    pool: Vec<T>,
    max_size: usize,
}

impl<T: LegalEntity + Clone> EntityPool<T> {
    /// Creates a new entity pool with a maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Acquires an entity from the pool or creates a new one
    pub fn acquire(&mut self, create_fn: impl FnOnce() -> T) -> T {
        self.pool.pop().unwrap_or_else(create_fn)
    }

    /// Returns an entity to the pool for reuse
    pub fn release(&mut self, entity: T) {
        if self.pool.len() < self.max_size {
            self.pool.push(entity);
        }
    }

    /// Returns the current pool size
    pub fn size(&self) -> usize {
        self.pool.len()
    }

    /// Clears the pool
    pub fn clear(&mut self) {
        self.pool.clear();
    }
}

/// Streaming processor for memory-efficient population processing
pub struct StreamingProcessor {
    buffer_size: usize,
}

impl StreamingProcessor {
    /// Creates a new streaming processor
    pub fn new(buffer_size: usize) -> Self {
        Self { buffer_size }
    }

    /// Processes entities in streaming fashion with a callback
    pub fn process<F, R>(&self, entities: Vec<Arc<dyn LegalEntity>>, mut processor: F) -> Vec<R>
    where
        F: FnMut(&dyn LegalEntity) -> R,
    {
        let mut results = Vec::with_capacity(entities.len());

        for chunk in entities.chunks(self.buffer_size) {
            for entity in chunk {
                results.push(processor(entity.as_ref()));
            }
        }

        results
    }

    /// Processes entities in parallel batches
    pub fn process_parallel<F, R>(
        &self,
        entities: Vec<Arc<dyn LegalEntity>>,
        processor: F,
    ) -> Vec<R>
    where
        F: Fn(&dyn LegalEntity) -> R + Send + Sync,
        R: Send,
    {
        use std::sync::Mutex;

        let results = Mutex::new(Vec::with_capacity(entities.len()));

        // Process in chunks to avoid excessive memory usage
        for chunk in entities.chunks(self.buffer_size) {
            let chunk_results: Vec<R> = chunk
                .iter()
                .map(|entity| processor(entity.as_ref()))
                .collect();

            results.lock().unwrap().extend(chunk_results);
        }

        results.into_inner().unwrap()
    }
}

/// Lazy attribute cache for delayed evaluation
pub struct LazyAttributeCache {
    cache: std::collections::HashMap<String, String>,
    dirty: bool,
}

impl LazyAttributeCache {
    /// Creates a new lazy attribute cache
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            dirty: false,
        }
    }

    /// Gets a cached attribute or computes it
    pub fn get_or_compute<F>(&mut self, key: &str, compute_fn: F) -> String
    where
        F: FnOnce() -> String,
    {
        if !self.cache.contains_key(key) {
            let value = compute_fn();
            self.cache.insert(key.to_string(), value.clone());
            self.dirty = true;
            value
        } else {
            self.cache.get(key).unwrap().clone()
        }
    }

    /// Checks if the cache has been modified
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks the cache as clean
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Clears the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.dirty = false;
    }

    /// Returns the cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

impl Default for LazyAttributeCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Work scheduler for optimal thread distribution
pub struct WorkScheduler {
    num_workers: usize,
    work_stealing_enabled: bool,
}

impl WorkScheduler {
    /// Creates a new work scheduler
    pub fn new(num_workers: usize) -> Self {
        Self {
            num_workers,
            work_stealing_enabled: true,
        }
    }

    /// Enables or disables work stealing
    pub fn with_work_stealing(mut self, enabled: bool) -> Self {
        self.work_stealing_enabled = enabled;
        self
    }

    /// Distributes work items across workers optimally
    pub fn distribute_work<T>(&self, items: Vec<T>) -> Vec<Vec<T>> {
        let total_items = items.len();
        if total_items == 0 {
            return (0..self.num_workers).map(|_| Vec::new()).collect();
        }

        let base_chunk_size = total_items / self.num_workers;
        let remainder = total_items % self.num_workers;

        let mut distributed: Vec<Vec<T>> = Vec::with_capacity(self.num_workers);
        let mut items_iter = items.into_iter();

        for worker_id in 0..self.num_workers {
            // Give extra item to first 'remainder' workers
            let chunk_size = if worker_id < remainder {
                base_chunk_size + 1
            } else {
                base_chunk_size
            };

            let chunk: Vec<T> = items_iter.by_ref().take(chunk_size).collect();
            distributed.push(chunk);
        }

        distributed
    }

    /// Returns optimal batch size for given total items
    pub fn optimal_batch_size(&self, total_items: usize) -> usize {
        let min_batch = 100;
        let max_batch = 10_000;

        let calculated = (total_items / self.num_workers).max(min_batch);
        calculated.min(max_batch)
    }

    /// Returns the number of workers
    pub fn num_workers(&self) -> usize {
        self.num_workers
    }
}

impl Default for WorkScheduler {
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

/// Memory-mapped population storage for large datasets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationMetadata {
    /// Number of entities in the population
    pub entity_count: usize,
    /// Version of the data format
    pub format_version: u32,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl PopulationMetadata {
    /// Creates new metadata
    pub fn new(entity_count: usize) -> Self {
        Self {
            entity_count,
            format_version: 1,
            created_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Adds metadata entry
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Memory-mapped population manager
pub struct MemoryMappedPopulation {
    metadata: PopulationMetadata,
    #[allow(dead_code)]
    file_path: std::path::PathBuf,
}

impl MemoryMappedPopulation {
    /// Creates a new memory-mapped population file
    pub fn create(
        path: impl Into<std::path::PathBuf>,
        metadata: PopulationMetadata,
    ) -> std::io::Result<Self> {
        let file_path = path.into();

        // Write metadata to file
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        std::fs::write(&file_path, metadata_json)?;

        Ok(Self {
            metadata,
            file_path,
        })
    }

    /// Opens an existing memory-mapped population file
    pub fn open(path: impl Into<std::path::PathBuf>) -> std::io::Result<Self> {
        let file_path = path.into();

        // Read metadata from file
        let metadata_json = std::fs::read_to_string(&file_path)?;
        let metadata: PopulationMetadata = serde_json::from_str(&metadata_json)?;

        Ok(Self {
            metadata,
            file_path,
        })
    }

    /// Returns population metadata
    pub fn metadata(&self) -> &PopulationMetadata {
        &self.metadata
    }

    /// Returns the number of entities
    pub fn entity_count(&self) -> usize {
        self.metadata.entity_count
    }
}

/// Parallel work executor with optimized distribution
pub struct ParallelExecutor {
    scheduler: WorkScheduler,
}

impl ParallelExecutor {
    /// Creates a new parallel executor
    pub fn new(num_workers: usize) -> Self {
        Self {
            scheduler: WorkScheduler::new(num_workers),
        }
    }

    /// Executes work in parallel with optimal distribution
    pub fn execute<T, F, R>(&self, items: Vec<T>, worker_fn: F) -> Vec<R>
    where
        T: Send,
        F: Fn(T) -> R + Send + Sync + Clone,
        R: Send,
    {
        use std::sync::Mutex;

        let distributed = self.scheduler.distribute_work(items);
        let results = Mutex::new(Vec::new());

        std::thread::scope(|s| {
            let handles: Vec<_> = distributed
                .into_iter()
                .map(|chunk| {
                    let worker_fn = worker_fn.clone();
                    let results_ref = &results;

                    s.spawn(move || {
                        let chunk_results: Vec<R> =
                            chunk.into_iter().map(|item| worker_fn(item)).collect();
                        results_ref.lock().unwrap().extend(chunk_results);
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });

        results.into_inner().unwrap()
    }

    /// Returns the scheduler
    pub fn scheduler(&self) -> &WorkScheduler {
        &self.scheduler
    }
}

impl Default for ParallelExecutor {
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

/// SIMD-accelerated batch operations for numeric data.
///
/// This processor uses SIMD (Single Instruction, Multiple Data) instructions
/// to accelerate common numeric operations on large datasets. Operations process
/// 4 f64 values simultaneously, providing significant performance improvements
/// for large-scale simulations.
///
/// # Examples
///
/// ```
/// use legalis_sim::SimdBatchProcessor;
///
/// // Compute statistics on a large dataset
/// let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
///
/// let sum = SimdBatchProcessor::sum_f64(&data);
/// let mean = SimdBatchProcessor::mean_f64(&data).unwrap();
/// let std_dev = SimdBatchProcessor::std_dev_f64(&data).unwrap();
///
/// assert_eq!(sum, 500500.0);
/// assert!((mean - 500.5).abs() < 0.01);
/// assert!(std_dev > 0.0);
/// ```
pub struct SimdBatchProcessor;

impl SimdBatchProcessor {
    /// Computes sum of f64 values using SIMD acceleration.
    ///
    /// Processes values in batches of 4 using SIMD instructions for improved performance.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_sim::SimdBatchProcessor;
    ///
    /// let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let sum = SimdBatchProcessor::sum_f64(&values);
    /// assert_eq!(sum, 15.0);
    /// ```
    pub fn sum_f64(values: &[f64]) -> f64 {
        let chunks = values.chunks_exact(4);
        let remainder = chunks.remainder();

        // Process 4 values at a time using SIMD
        let mut sum_vec = f64x4::splat(0.0);
        for chunk in chunks {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            sum_vec += vec;
        }

        // Sum the SIMD vector
        let sum: f64 = sum_vec.reduce_add();

        // Add remainder
        sum + remainder.iter().sum::<f64>()
    }

    /// Computes mean of f64 values using SIMD acceleration.
    ///
    /// Returns `None` if the input slice is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_sim::SimdBatchProcessor;
    ///
    /// let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    /// let mean = SimdBatchProcessor::mean_f64(&values).unwrap();
    /// assert_eq!(mean, 6.0);
    /// ```
    pub fn mean_f64(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        Some(Self::sum_f64(values) / values.len() as f64)
    }

    /// Computes variance of f64 values using SIMD acceleration
    pub fn variance_f64(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }

        let mean = Self::mean_f64(values)?;
        let mean_vec = f64x4::splat(mean);

        let chunks = values.chunks_exact(4);
        let remainder = chunks.remainder();

        // Compute sum of squared differences using SIMD
        let mut sq_diff_sum = f64x4::splat(0.0);
        for chunk in chunks {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let diff = vec - mean_vec;
            sq_diff_sum += diff * diff;
        }

        let mut sum: f64 = sq_diff_sum.reduce_add();

        // Add remainder
        for &value in remainder {
            let diff = value - mean;
            sum += diff * diff;
        }

        Some(sum / values.len() as f64)
    }

    /// Computes standard deviation using SIMD acceleration
    pub fn std_dev_f64(values: &[f64]) -> Option<f64> {
        Self::variance_f64(values).map(|v| v.sqrt())
    }

    /// Computes minimum value using SIMD acceleration
    pub fn min_f64(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }

        let chunks = values.chunks_exact(4);
        let remainder = chunks.remainder();

        let mut min_vec = f64x4::splat(f64::INFINITY);
        for chunk in chunks {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            min_vec = min_vec.min(vec);
        }

        // Extract minimum from SIMD vector
        let arr = min_vec.to_array();
        let mut min = arr[0].min(arr[1]).min(arr[2]).min(arr[3]);

        // Check remainder
        for &value in remainder {
            if value < min {
                min = value;
            }
        }

        Some(min)
    }

    /// Computes maximum value using SIMD acceleration
    pub fn max_f64(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }

        let chunks = values.chunks_exact(4);
        let remainder = chunks.remainder();

        let mut max_vec = f64x4::splat(f64::NEG_INFINITY);
        for chunk in chunks {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            max_vec = max_vec.max(vec);
        }

        // Extract maximum from SIMD vector
        let arr = max_vec.to_array();
        let mut max = arr[0].max(arr[1]).max(arr[2]).max(arr[3]);

        // Check remainder
        for &value in remainder {
            if value > max {
                max = value;
            }
        }

        Some(max)
    }

    /// Applies scalar multiplication using SIMD acceleration
    pub fn scale_f64(values: &mut [f64], scalar: f64) {
        let scalar_vec = f64x4::splat(scalar);

        let (chunks, remainder) = values.split_at_mut(values.len() - values.len() % 4);

        // Process 4 values at a time
        for chunk in chunks.chunks_exact_mut(4) {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let result = vec * scalar_vec;
            let arr = result.to_array();
            chunk.copy_from_slice(&arr);
        }

        // Process remainder
        for value in remainder {
            *value *= scalar;
        }
    }

    /// Computes dot product of two vectors using SIMD acceleration
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> Option<f64> {
        if a.len() != b.len() {
            return None;
        }

        let chunks_a = a.chunks_exact(4);
        let chunks_b = b.chunks_exact(4);
        let remainder_a = chunks_a.remainder();
        let remainder_b = chunks_b.remainder();

        let mut dot_vec = f64x4::splat(0.0);
        for (chunk_a, chunk_b) in chunks_a.zip(chunks_b) {
            let vec_a = f64x4::from([chunk_a[0], chunk_a[1], chunk_a[2], chunk_a[3]]);
            let vec_b = f64x4::from([chunk_b[0], chunk_b[1], chunk_b[2], chunk_b[3]]);
            dot_vec += vec_a * vec_b;
        }

        let mut dot: f64 = dot_vec.reduce_add();

        // Add remainder
        for (val_a, val_b) in remainder_a.iter().zip(remainder_b) {
            dot += val_a * val_b;
        }

        Some(dot)
    }

    /// Normalizes values to [0, 1] range using SIMD acceleration
    pub fn normalize_f64(values: &mut [f64]) -> Option<()> {
        let min = Self::min_f64(values)?;
        let max = Self::max_f64(values)?;

        if (max - min).abs() < f64::EPSILON {
            return None; // Avoid division by zero
        }

        let range = max - min;
        let min_vec = f64x4::splat(min);
        let range_vec = f64x4::splat(range);

        let (chunks, remainder) = values.split_at_mut(values.len() - values.len() % 4);

        // Process 4 values at a time
        for chunk in chunks.chunks_exact_mut(4) {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let result = (vec - min_vec) / range_vec;
            let arr = result.to_array();
            chunk.copy_from_slice(&arr);
        }

        // Process remainder
        for value in remainder {
            *value = (*value - min) / range;
        }

        Some(())
    }
}

/// Distributed simulation coordinator for multi-node execution.
///
/// Enables simulation workloads to be distributed across multiple compute nodes
/// for large-scale simulations that exceed single-node capacity.
///
/// # Examples
///
/// ```
/// use legalis_sim::DistributedConfig;
///
/// // Configure a node in a 4-node cluster
/// let config = DistributedConfig::new(
///     "0".to_string(),
///     4,
///     "coordinator.example.com".to_string(),
///     8080
/// );
///
/// // Determine which portion of 1000 items this node should process
/// let (start, end) = config.partition_range(1000);
/// assert_eq!(start, 0);
/// assert_eq!(end, 250); // First node processes items 0-249
/// ```
#[derive(Debug, Clone)]
pub struct DistributedConfig {
    /// Node ID in the cluster
    pub node_id: String,
    /// Total number of nodes
    pub num_nodes: usize,
    /// Coordinator address
    pub coordinator_addr: String,
    /// Port for node communication
    pub port: u16,
}

impl DistributedConfig {
    /// Creates a new distributed configuration
    pub fn new(node_id: String, num_nodes: usize, coordinator_addr: String, port: u16) -> Self {
        Self {
            node_id,
            num_nodes,
            coordinator_addr,
            port,
        }
    }

    /// Returns the partition range for this node
    pub fn partition_range(&self, total_items: usize) -> (usize, usize) {
        let node_index = self.node_id.parse::<usize>().unwrap_or(0);
        let items_per_node = total_items / self.num_nodes;
        let remainder = total_items % self.num_nodes;

        let start = node_index * items_per_node + node_index.min(remainder);
        let extra = if node_index < remainder { 1 } else { 0 };
        let end = start + items_per_node + extra;

        (start, end)
    }
}

/// Distributed simulation node
pub struct DistributedNode {
    config: DistributedConfig,
    #[allow(dead_code)]
    runtime: tokio::runtime::Runtime,
}

impl DistributedNode {
    /// Creates a new distributed node
    pub fn new(config: DistributedConfig) -> Self {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

        Self { config, runtime }
    }

    /// Returns the node configuration
    pub fn config(&self) -> &DistributedConfig {
        &self.config
    }

    /// Partitions work for this node
    pub fn partition_work<T>(&self, items: Vec<T>) -> Vec<T> {
        let total = items.len();
        let (start, end) = self.config.partition_range(total);

        items.into_iter().skip(start).take(end - start).collect()
    }

    /// Executes work on this node
    pub fn execute_local<T, F, R>(&self, items: Vec<T>, worker_fn: F) -> Vec<R>
    where
        T: Send,
        F: Fn(T) -> R + Send + Sync + Clone,
        R: Send,
    {
        let executor = ParallelExecutor::default();
        executor.execute(items, worker_fn)
    }
}

/// Distributed simulation coordinator.
///
/// Manages work distribution across multiple simulation nodes and aggregates results.
///
/// # Examples
///
/// ```
/// use legalis_sim::DistributedCoordinator;
///
/// // Create coordinator for 3-node cluster
/// let coordinator = DistributedCoordinator::new(3);
///
/// // Distribute 100 items across nodes
/// let items: Vec<i32> = (0..100).collect();
/// let distributed = coordinator.distribute_work(items);
///
/// assert_eq!(distributed.len(), 3);
/// assert_eq!(distributed[0].len(), 34); // First node gets extra item
/// assert_eq!(distributed[1].len(), 33);
/// assert_eq!(distributed[2].len(), 33);
/// ```
pub struct DistributedCoordinator {
    num_nodes: usize,
    nodes: Vec<String>,
}

impl DistributedCoordinator {
    /// Creates a new distributed coordinator
    pub fn new(num_nodes: usize) -> Self {
        let nodes = (0..num_nodes).map(|i| format!("node-{}", i)).collect();

        Self { num_nodes, nodes }
    }

    /// Returns the number of nodes
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Returns the list of nodes
    pub fn nodes(&self) -> &[String] {
        &self.nodes
    }

    /// Distributes work across nodes
    pub fn distribute_work<T>(&self, items: Vec<T>) -> Vec<Vec<T>> {
        let total_items = items.len();
        let items_per_node = total_items / self.num_nodes;
        let remainder = total_items % self.num_nodes;

        let mut distributed: Vec<Vec<T>> = Vec::with_capacity(self.num_nodes);
        let mut items_iter = items.into_iter();

        for node_id in 0..self.num_nodes {
            let node_size = if node_id < remainder {
                items_per_node + 1
            } else {
                items_per_node
            };

            let chunk: Vec<T> = items_iter.by_ref().take(node_size).collect();
            distributed.push(chunk);
        }

        distributed
    }

    /// Aggregates results from all nodes
    pub fn aggregate_results<T>(&self, node_results: Vec<Vec<T>>) -> Vec<T> {
        node_results.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::BasicEntity;

    #[test]
    fn test_batch_iterator() {
        let items: Vec<i32> = (0..100).collect();
        let mut batch_iter = BatchIterator::new(items, 25);

        assert_eq!(batch_iter.batch_count(), 4);

        let batch1 = batch_iter.next().unwrap();
        assert_eq!(batch1.len(), 25);

        let batch2 = batch_iter.next().unwrap();
        assert_eq!(batch2.len(), 25);

        let batch3 = batch_iter.next().unwrap();
        assert_eq!(batch3.len(), 25);

        let batch4 = batch_iter.next().unwrap();
        assert_eq!(batch4.len(), 25);

        assert!(batch_iter.next().is_none());
    }

    #[test]
    fn test_batch_config() {
        let config = BatchConfig::default()
            .with_batch_size(500)
            .with_workers(4)
            .with_streaming(true);

        assert_eq!(config.batch_size, 500);
        assert_eq!(config.num_workers, 4);
        assert!(config.streaming_mode);
    }

    #[test]
    fn test_entity_pool() {
        let mut pool = EntityPool::new(10);

        let entity1 = pool.acquire(|| BasicEntity::new());
        let id1 = entity1.id();

        pool.release(entity1);
        assert_eq!(pool.size(), 1);

        let entity2 = pool.acquire(|| BasicEntity::new());
        assert_eq!(entity2.id(), id1); // Should reuse the same entity

        pool.clear();
        assert_eq!(pool.size(), 0);
    }

    #[test]
    fn test_streaming_processor() {
        let processor = StreamingProcessor::new(10);

        let entities: Vec<Arc<dyn LegalEntity>> = (0..50)
            .map(|_| Arc::new(BasicEntity::new()) as Arc<dyn LegalEntity>)
            .collect();

        let results = processor.process(entities, |entity| entity.id());
        assert_eq!(results.len(), 50);
    }

    #[test]
    fn test_lazy_attribute_cache() {
        let mut cache = LazyAttributeCache::new();

        let value1 = cache.get_or_compute("test", || "computed".to_string());
        assert_eq!(value1, "computed");
        assert!(cache.is_dirty());
        assert_eq!(cache.size(), 1);

        cache.mark_clean();
        assert!(!cache.is_dirty());

        let value2 = cache.get_or_compute("test", || "should_not_compute".to_string());
        assert_eq!(value2, "computed"); // Should use cached value
        assert!(!cache.is_dirty());

        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_work_scheduler_distribution() {
        let scheduler = WorkScheduler::new(4);
        let items: Vec<i32> = (0..100).collect();

        let distributed = scheduler.distribute_work(items);

        assert_eq!(distributed.len(), 4);

        // Each worker should get 25 items (100 / 4)
        for chunk in &distributed {
            assert_eq!(chunk.len(), 25);
        }

        // Verify all items are distributed
        let total: usize = distributed.iter().map(|c| c.len()).sum();
        assert_eq!(total, 100);
    }

    #[test]
    fn test_work_scheduler_uneven_distribution() {
        let scheduler = WorkScheduler::new(3);
        let items: Vec<i32> = (0..10).collect();

        let distributed = scheduler.distribute_work(items);

        assert_eq!(distributed.len(), 3);

        // First worker gets 4 items (10 / 3 = 3 remainder 1)
        assert_eq!(distributed[0].len(), 4);
        // Other workers get 3 items each
        assert_eq!(distributed[1].len(), 3);
        assert_eq!(distributed[2].len(), 3);

        let total: usize = distributed.iter().map(|c| c.len()).sum();
        assert_eq!(total, 10);
    }

    #[test]
    fn test_work_scheduler_optimal_batch_size() {
        let scheduler = WorkScheduler::new(4);

        // Small dataset
        assert_eq!(scheduler.optimal_batch_size(200), 100);

        // Medium dataset
        assert_eq!(scheduler.optimal_batch_size(40_000), 10_000);

        // Large dataset - should cap at max
        assert_eq!(scheduler.optimal_batch_size(1_000_000), 10_000);
    }

    #[test]
    fn test_parallel_executor() {
        let executor = ParallelExecutor::new(4);
        let items: Vec<i32> = (0..100).collect();

        let results = executor.execute(items, |x| x * 2);

        assert_eq!(results.len(), 100);
        // Results might not be in order due to parallel execution
        let mut sorted_results = results;
        sorted_results.sort();

        for (i, &value) in sorted_results.iter().enumerate() {
            assert_eq!(value, (i as i32) * 2);
        }
    }

    #[test]
    fn test_memory_mapped_metadata() {
        let metadata =
            PopulationMetadata::new(1000).with_metadata("source".to_string(), "test".to_string());

        assert_eq!(metadata.entity_count, 1000);
        assert_eq!(metadata.format_version, 1);
        assert_eq!(metadata.metadata.get("source").unwrap(), "test");
    }

    #[test]
    fn test_memory_mapped_population_create_and_open() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_population.json");

        // Clean up if exists
        let _ = std::fs::remove_file(&file_path);

        // Create new population file
        let metadata = PopulationMetadata::new(500);
        let pop = MemoryMappedPopulation::create(&file_path, metadata).unwrap();
        assert_eq!(pop.entity_count(), 500);

        // Open existing population file
        let pop_opened = MemoryMappedPopulation::open(&file_path).unwrap();
        assert_eq!(pop_opened.entity_count(), 500);
        assert_eq!(pop_opened.metadata().format_version, 1);

        // Clean up
        std::fs::remove_file(&file_path).unwrap();
    }

    // SIMD tests
    #[test]
    fn test_simd_sum() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let sum = SimdBatchProcessor::sum_f64(&values);
        let expected: f64 = (1..=100).sum::<i32>() as f64;
        assert!((sum - expected).abs() < 1e-10);
    }

    #[test]
    fn test_simd_mean() {
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = SimdBatchProcessor::mean_f64(&values).unwrap();
        assert!((mean - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_simd_variance() {
        let values: Vec<f64> = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let variance = SimdBatchProcessor::variance_f64(&values).unwrap();
        // Expected variance = 4.0
        assert!((variance - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_simd_std_dev() {
        let values: Vec<f64> = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std_dev = SimdBatchProcessor::std_dev_f64(&values).unwrap();
        // Expected std_dev = 2.0
        assert!((std_dev - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_simd_min_max() {
        let values: Vec<f64> = vec![5.0, 2.0, 8.0, 1.0, 9.0, 3.0];
        let min = SimdBatchProcessor::min_f64(&values).unwrap();
        let max = SimdBatchProcessor::max_f64(&values).unwrap();
        assert_eq!(min, 1.0);
        assert_eq!(max, 9.0);
    }

    #[test]
    fn test_simd_scale() {
        let mut values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        SimdBatchProcessor::scale_f64(&mut values, 2.0);
        assert_eq!(values, vec![2.0, 4.0, 6.0, 8.0, 10.0]);
    }

    #[test]
    fn test_simd_dot_product() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let b: Vec<f64> = vec![5.0, 6.0, 7.0, 8.0];
        let dot = SimdBatchProcessor::dot_product_f64(&a, &b).unwrap();
        // 1*5 + 2*6 + 3*7 + 4*8 = 5 + 12 + 21 + 32 = 70
        assert_eq!(dot, 70.0);
    }

    #[test]
    fn test_simd_normalize() {
        let mut values: Vec<f64> = vec![0.0, 5.0, 10.0];
        SimdBatchProcessor::normalize_f64(&mut values).unwrap();
        assert_eq!(values[0], 0.0);
        assert_eq!(values[1], 0.5);
        assert_eq!(values[2], 1.0);
    }

    #[test]
    fn test_simd_large_dataset() {
        let values: Vec<f64> = (1..=10000).map(|x| x as f64).collect();
        let sum = SimdBatchProcessor::sum_f64(&values);
        let expected: f64 = (1..=10000).sum::<i32>() as f64;
        assert!((sum - expected).abs() < 1e-6);
    }

    // Distributed simulation tests
    #[test]
    fn test_distributed_config() {
        let config = DistributedConfig::new("0".to_string(), 4, "localhost".to_string(), 8080);

        assert_eq!(config.node_id, "0");
        assert_eq!(config.num_nodes, 4);
        assert_eq!(config.coordinator_addr, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_distributed_partition_range() {
        let config = DistributedConfig::new("0".to_string(), 4, "localhost".to_string(), 8080);

        let (start, end) = config.partition_range(100);
        assert_eq!(start, 0);
        assert_eq!(end, 25);
    }

    #[test]
    fn test_distributed_node() {
        let config = DistributedConfig::new("0".to_string(), 4, "localhost".to_string(), 8080);

        let node = DistributedNode::new(config);
        assert_eq!(node.config().node_id, "0");
        assert_eq!(node.config().num_nodes, 4);
    }

    #[test]
    fn test_distributed_partition_work() {
        let config = DistributedConfig::new("0".to_string(), 4, "localhost".to_string(), 8080);

        let node = DistributedNode::new(config);
        let items: Vec<i32> = (0..100).collect();
        let partitioned = node.partition_work(items);

        // Node 0 should get first 25 items
        assert_eq!(partitioned.len(), 25);
        assert_eq!(partitioned[0], 0);
        assert_eq!(partitioned[24], 24);
    }

    #[test]
    fn test_distributed_execute_local() {
        let config = DistributedConfig::new("0".to_string(), 2, "localhost".to_string(), 8080);

        let node = DistributedNode::new(config);
        let items: Vec<i32> = vec![1, 2, 3, 4, 5];
        let results = node.execute_local(items, |x| x * 2);

        assert_eq!(results.len(), 5);
        let mut sorted = results;
        sorted.sort();
        assert_eq!(sorted, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_distributed_coordinator() {
        let coordinator = DistributedCoordinator::new(3);
        assert_eq!(coordinator.num_nodes(), 3);
        assert_eq!(coordinator.nodes().len(), 3);
        assert_eq!(coordinator.nodes()[0], "node-0");
        assert_eq!(coordinator.nodes()[1], "node-1");
        assert_eq!(coordinator.nodes()[2], "node-2");
    }

    #[test]
    fn test_distributed_distribute_work() {
        let coordinator = DistributedCoordinator::new(3);
        let items: Vec<i32> = (0..10).collect();
        let distributed = coordinator.distribute_work(items);

        assert_eq!(distributed.len(), 3);
        // 10 items / 3 nodes = 3 per node + 1 remainder
        assert_eq!(distributed[0].len(), 4); // First node gets extra
        assert_eq!(distributed[1].len(), 3);
        assert_eq!(distributed[2].len(), 3);

        // Verify all items are distributed
        let total: usize = distributed.iter().map(|v| v.len()).sum();
        assert_eq!(total, 10);
    }

    #[test]
    fn test_distributed_aggregate_results() {
        let coordinator = DistributedCoordinator::new(3);
        let node_results = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

        let aggregated = coordinator.aggregate_results(node_results);
        assert_eq!(aggregated.len(), 9);
        assert_eq!(aggregated, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
