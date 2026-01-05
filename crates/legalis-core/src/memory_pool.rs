//! Memory pool management for efficient allocation in batch operations.
//!
//! This module provides memory pools that reduce allocation overhead when
//! processing large batches of entities or conditions. Pools pre-allocate
//! memory and reuse it across multiple operations.
//!
//! ## Features
//!
//! - **Object Pools**: Reusable pools of pre-allocated objects
//! - **Buffer Pools**: Byte buffer pools for string and binary data
//! - **Statistics Tracking**: Monitor pool usage and efficiency
//! - **Thread-Safe**: Optional thread-safe pools (requires "parallel" feature)
//!
//! ## Example
//!
//! ```
//! use legalis_core::memory_pool::{MemoryPool, PoolConfig};
//!
//! // Create a pool for Vec<bool> objects
//! let mut pool = MemoryPool::<Vec<bool>>::new(PoolConfig {
//!     initial_capacity: 10,
//!     max_capacity: 100,
//! });
//!
//! // Acquire an object from the pool
//! let mut vec = pool.acquire();
//! vec.push(true);
//! vec.push(false);
//!
//! // Return it to the pool for reuse
//! pool.release(vec);
//!
//! // Statistics
//! let stats = pool.stats();
//! assert_eq!(stats.total_acquisitions, 1);
//! assert_eq!(stats.total_releases, 1);
//! ```

use std::collections::VecDeque;

/// Configuration for memory pools.
#[derive(Debug, Clone, Copy)]
pub struct PoolConfig {
    /// Initial number of pre-allocated objects
    pub initial_capacity: usize,
    /// Maximum number of pooled objects (0 = unlimited)
    pub max_capacity: usize,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            initial_capacity: 10,
            max_capacity: 100,
        }
    }
}

/// Statistics for memory pool usage.
///
/// # Example
///
/// ```
/// use legalis_core::memory_pool::PoolStats;
///
/// let mut stats = PoolStats::new();
/// stats.record_acquisition();
/// stats.record_acquisition();
/// stats.record_release();
///
/// assert_eq!(stats.total_acquisitions, 2);
/// assert_eq!(stats.total_releases, 1);
/// assert_eq!(stats.current_usage(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total number of object acquisitions
    pub total_acquisitions: u64,
    /// Total number of object releases
    pub total_releases: u64,
    /// Number of new allocations (not from pool)
    pub allocations: u64,
    /// Number of objects currently in use
    pub in_use: usize,
    /// Peak number of objects in use simultaneously
    pub peak_usage: usize,
}

impl PoolStats {
    /// Creates new pool statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an object acquisition.
    pub fn record_acquisition(&mut self) {
        self.total_acquisitions += 1;
        self.in_use += 1;
        if self.in_use > self.peak_usage {
            self.peak_usage = self.in_use;
        }
    }

    /// Records an object release.
    pub fn record_release(&mut self) {
        self.total_releases += 1;
        if self.in_use > 0 {
            self.in_use -= 1;
        }
    }

    /// Records a new allocation.
    pub fn record_allocation(&mut self) {
        self.allocations += 1;
    }

    /// Returns the current number of objects in use.
    pub fn current_usage(&self) -> usize {
        self.in_use
    }

    /// Returns the reuse rate (0.0 to 1.0).
    ///
    /// Higher values indicate better pool efficiency.
    pub fn reuse_rate(&self) -> f64 {
        if self.total_acquisitions == 0 {
            return 0.0;
        }
        let reused = self.total_acquisitions.saturating_sub(self.allocations);
        reused as f64 / self.total_acquisitions as f64
    }

    /// Resets all statistics.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

/// Trait for objects that can be reset before being returned to the pool.
pub trait Reusable {
    /// Resets the object to its initial state.
    fn reset(&mut self);
}

impl<T> Reusable for Vec<T> {
    fn reset(&mut self) {
        self.clear();
    }
}

impl Reusable for String {
    fn reset(&mut self) {
        self.clear();
    }
}

/// A memory pool for reusable objects.
///
/// # Example
///
/// ```
/// use legalis_core::memory_pool::{MemoryPool, PoolConfig};
///
/// let mut pool = MemoryPool::<Vec<i32>>::with_config(PoolConfig {
///     initial_capacity: 5,
///     max_capacity: 20,
/// });
///
/// let mut v1 = pool.acquire();
/// v1.push(1);
/// v1.push(2);
///
/// pool.release(v1);
///
/// let v2 = pool.acquire(); // Reuses the released vector
/// assert!(v2.is_empty()); // Vector was reset
/// ```
#[derive(Debug)]
pub struct MemoryPool<T: Reusable + Default> {
    pool: VecDeque<T>,
    config: PoolConfig,
    stats: PoolStats,
}

impl<T: Reusable + Default> MemoryPool<T> {
    /// Creates a new memory pool with default configuration.
    pub fn new(config: PoolConfig) -> Self {
        let mut pool = VecDeque::with_capacity(config.initial_capacity);

        // Pre-allocate objects
        for _ in 0..config.initial_capacity {
            pool.push_back(T::default());
        }

        Self {
            pool,
            config,
            stats: PoolStats::new(),
        }
    }

    /// Creates a new memory pool with custom configuration.
    pub fn with_config(config: PoolConfig) -> Self {
        Self::new(config)
    }

    /// Acquires an object from the pool.
    ///
    /// If the pool is empty, allocates a new object.
    pub fn acquire(&mut self) -> T {
        self.stats.record_acquisition();

        if let Some(obj) = self.pool.pop_front() {
            obj
        } else {
            self.stats.record_allocation();
            T::default()
        }
    }

    /// Returns an object to the pool for reuse.
    ///
    /// The object is reset before being stored.
    /// If the pool is at max capacity, the object is dropped.
    pub fn release(&mut self, mut obj: T) {
        self.stats.record_release();
        obj.reset();

        if self.config.max_capacity == 0 || self.pool.len() < self.config.max_capacity {
            self.pool.push_back(obj);
        }
    }

    /// Returns the number of available objects in the pool.
    pub fn available(&self) -> usize {
        self.pool.len()
    }

    /// Returns pool statistics.
    pub fn stats(&self) -> &PoolStats {
        &self.stats
    }

    /// Clears all pooled objects.
    pub fn clear(&mut self) {
        self.pool.clear();
    }

    /// Pre-allocates additional objects.
    pub fn grow(&mut self, additional: usize) {
        for _ in 0..additional {
            if self.config.max_capacity == 0 || self.pool.len() < self.config.max_capacity {
                self.pool.push_back(T::default());
            } else {
                break;
            }
        }
    }
}

impl<T: Reusable + Default> Default for MemoryPool<T> {
    fn default() -> Self {
        Self::new(PoolConfig::default())
    }
}

/// A buffer pool specialized for byte buffers.
///
/// # Example
///
/// ```
/// use legalis_core::memory_pool::BufferPool;
///
/// let mut pool = BufferPool::new(1024, 10);
///
/// let mut buf = pool.acquire();
/// buf.extend_from_slice(b"Hello");
///
/// pool.release(buf);
/// ```
#[derive(Debug)]
pub struct BufferPool {
    pool: MemoryPool<Vec<u8>>,
    buffer_size: usize,
}

impl BufferPool {
    /// Creates a new buffer pool.
    ///
    /// # Arguments
    ///
    /// * `buffer_size` - Size of each buffer in bytes
    /// * `pool_size` - Number of buffers to pre-allocate
    pub fn new(buffer_size: usize, pool_size: usize) -> Self {
        let config = PoolConfig {
            initial_capacity: pool_size,
            max_capacity: pool_size * 2,
        };

        let mut pool = MemoryPool::new(config);

        // Pre-allocate buffers with capacity
        for _ in 0..pool_size {
            let mut buf = Vec::with_capacity(buffer_size);
            buf.clear();
            pool.release(buf);
        }

        Self { pool, buffer_size }
    }

    /// Acquires a buffer from the pool.
    pub fn acquire(&mut self) -> Vec<u8> {
        let mut buf = self.pool.acquire();
        buf.reserve(self.buffer_size);
        buf
    }

    /// Returns a buffer to the pool.
    pub fn release(&mut self, buf: Vec<u8>) {
        self.pool.release(buf);
    }

    /// Returns pool statistics.
    pub fn stats(&self) -> &PoolStats {
        self.pool.stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_stats() {
        let mut stats = PoolStats::new();
        stats.record_acquisition();
        stats.record_acquisition();
        stats.record_release();

        assert_eq!(stats.total_acquisitions, 2);
        assert_eq!(stats.total_releases, 1);
        assert_eq!(stats.current_usage(), 1);
    }

    #[test]
    fn test_reuse_rate() {
        let mut stats = PoolStats::new();
        stats.record_acquisition();
        stats.record_allocation();
        stats.record_acquisition();
        stats.record_acquisition();

        // 3 acquisitions, 1 allocation => 2/3 reuse rate
        assert!((stats.reuse_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_memory_pool_basic() {
        let mut pool: MemoryPool<Vec<i32>> = MemoryPool::new(PoolConfig {
            initial_capacity: 5,
            max_capacity: 10,
        });

        assert_eq!(pool.available(), 5);

        let mut v = pool.acquire();
        assert_eq!(pool.available(), 4);

        v.push(1);
        v.push(2);

        pool.release(v);
        assert_eq!(pool.available(), 5);

        let v2 = pool.acquire();
        assert!(v2.is_empty()); // Should be reset
    }

    #[test]
    fn test_pool_overflow() {
        let mut pool: MemoryPool<Vec<i32>> = MemoryPool::new(PoolConfig {
            initial_capacity: 2,
            max_capacity: 3,
        });

        let v1 = pool.acquire();
        let v2 = pool.acquire();
        let v3 = pool.acquire();
        let v4 = pool.acquire(); // Allocates new

        pool.release(v1);
        pool.release(v2);
        pool.release(v3);
        pool.release(v4); // Exceeds max, gets dropped

        assert_eq!(pool.available(), 3); // Max capacity
    }

    #[test]
    fn test_buffer_pool() {
        let mut pool = BufferPool::new(1024, 5);

        let mut buf = pool.acquire();
        buf.extend_from_slice(b"Hello, world!");

        pool.release(buf);

        let buf2 = pool.acquire();
        assert!(buf2.is_empty());
        assert!(buf2.capacity() >= 1024);
    }

    #[test]
    fn test_pool_grow() {
        let mut pool: MemoryPool<Vec<i32>> = MemoryPool::new(PoolConfig {
            initial_capacity: 2,
            max_capacity: 10,
        });

        assert_eq!(pool.available(), 2);
        pool.grow(5);
        assert_eq!(pool.available(), 7);
    }

    #[test]
    fn test_pool_clear() {
        let mut pool: MemoryPool<Vec<i32>> = MemoryPool::new(PoolConfig {
            initial_capacity: 5,
            max_capacity: 10,
        });

        pool.clear();
        assert_eq!(pool.available(), 0);
    }
}
