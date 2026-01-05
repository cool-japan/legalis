//! Bloom filter for fast record existence checks.
//!
//! This module provides a Bloom filter implementation for quickly checking
//! if audit records exist without loading all records into memory.
//!
//! Bloom filters are probabilistic data structures that:
//! - Never produce false negatives (if it says "no", it's definitely not there)
//! - May produce false positives (if it says "yes", it might be there)
//! - Use much less memory than storing all IDs

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

/// A simple Bloom filter for UUID record IDs.
pub struct BloomFilter {
    /// Bit array
    bits: Vec<bool>,
    /// Number of hash functions
    num_hashes: usize,
    /// Size of bit array
    size: usize,
}

impl BloomFilter {
    /// Creates a new Bloom filter.
    ///
    /// # Arguments
    /// * `expected_items` - Expected number of items to store
    /// * `false_positive_rate` - Desired false positive rate (e.g., 0.01 for 1%)
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        // Calculate optimal size and number of hashes
        let size = Self::optimal_size(expected_items, false_positive_rate);
        let num_hashes = Self::optimal_num_hashes(size, expected_items);

        Self {
            bits: vec![false; size],
            num_hashes,
            size,
        }
    }

    /// Creates a Bloom filter with specific size and number of hashes.
    pub fn with_params(size: usize, num_hashes: usize) -> Self {
        Self {
            bits: vec![false; size],
            num_hashes,
            size,
        }
    }

    /// Calculates optimal bit array size.
    fn optimal_size(n: usize, p: f64) -> usize {
        let ln2_squared = std::f64::consts::LN_2 * std::f64::consts::LN_2;
        let size = -(n as f64 * p.ln()) / ln2_squared;
        size.ceil() as usize
    }

    /// Calculates optimal number of hash functions.
    fn optimal_num_hashes(m: usize, n: usize) -> usize {
        let k = (m as f64 / n as f64) * std::f64::consts::LN_2;
        k.ceil().max(1.0) as usize
    }

    /// Adds an item to the Bloom filter.
    pub fn add(&mut self, id: &Uuid) {
        for i in 0..self.num_hashes {
            let index = self.hash(id, i) % self.size;
            self.bits[index] = true;
        }
    }

    /// Checks if an item might be in the Bloom filter.
    ///
    /// Returns:
    /// - `true`: Item might be present (may be false positive)
    /// - `false`: Item is definitely not present
    pub fn contains(&self, id: &Uuid) -> bool {
        for i in 0..self.num_hashes {
            let index = self.hash(id, i) % self.size;
            if !self.bits[index] {
                return false;
            }
        }
        true
    }

    /// Computes hash for an item with a given seed.
    fn hash(&self, id: &Uuid, seed: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Estimates the current false positive rate.
    pub fn estimated_false_positive_rate(&self, items_added: usize) -> f64 {
        if items_added == 0 {
            return 0.0;
        }

        let k = self.num_hashes as f64;
        let m = self.size as f64;
        let n = items_added as f64;

        (1.0 - (-k * n / m).exp()).powf(k)
    }

    /// Gets the number of bits set in the filter.
    pub fn count_set_bits(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }

    /// Gets the fill rate of the filter (0.0 to 1.0).
    pub fn fill_rate(&self) -> f64 {
        self.count_set_bits() as f64 / self.size as f64
    }

    /// Clears all bits in the filter.
    pub fn clear(&mut self) {
        self.bits.fill(false);
    }

    /// Gets statistics about the filter.
    pub fn stats(&self) -> BloomFilterStats {
        BloomFilterStats {
            size: self.size,
            num_hashes: self.num_hashes,
            bits_set: self.count_set_bits(),
            fill_rate: self.fill_rate(),
        }
    }
}

/// Statistics about a Bloom filter.
#[derive(Debug, Clone)]
pub struct BloomFilterStats {
    /// Size of bit array
    pub size: usize,
    /// Number of hash functions
    pub num_hashes: usize,
    /// Number of bits set
    pub bits_set: usize,
    /// Fill rate (0.0 to 1.0)
    pub fill_rate: f64,
}

/// Bloom filter builder for audit trails.
pub struct BloomFilterBuilder {
    expected_items: Option<usize>,
    false_positive_rate: f64,
}

impl BloomFilterBuilder {
    /// Creates a new Bloom filter builder.
    pub fn new() -> Self {
        Self {
            expected_items: None,
            false_positive_rate: 0.01, // 1% default
        }
    }

    /// Sets the expected number of items.
    pub fn expected_items(mut self, n: usize) -> Self {
        self.expected_items = Some(n);
        self
    }

    /// Sets the desired false positive rate.
    pub fn false_positive_rate(mut self, rate: f64) -> Self {
        self.false_positive_rate = rate.clamp(0.0001, 0.1);
        self
    }

    /// Builds the Bloom filter.
    pub fn build(self) -> BloomFilter {
        let expected = self.expected_items.unwrap_or(1000);
        BloomFilter::new(expected, self.false_positive_rate)
    }
}

impl Default for BloomFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_basic() {
        let mut filter = BloomFilter::new(100, 0.01);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let _id3 = Uuid::new_v4();

        filter.add(&id1);
        filter.add(&id2);

        assert!(filter.contains(&id1));
        assert!(filter.contains(&id2));
        // _id3 was not added, so it should return false (unless false positive)
        // We can't assert !filter.contains(&_id3) because it might be a false positive
    }

    #[test]
    fn test_bloom_filter_no_false_negatives() {
        let mut filter = BloomFilter::new(100, 0.01);

        let ids: Vec<Uuid> = (0..50).map(|_| Uuid::new_v4()).collect();

        for id in &ids {
            filter.add(id);
        }

        // All added IDs must be found (no false negatives)
        for id in &ids {
            assert!(filter.contains(id), "False negative detected!");
        }
    }

    #[test]
    fn test_bloom_filter_fill_rate() {
        let mut filter = BloomFilter::new(100, 0.01);

        let initial_fill_rate = filter.fill_rate();
        assert_eq!(initial_fill_rate, 0.0);

        for _ in 0..10 {
            filter.add(&Uuid::new_v4());
        }

        let filled_rate = filter.fill_rate();
        assert!(filled_rate > 0.0);
        assert!(filled_rate <= 1.0);
    }

    #[test]
    fn test_bloom_filter_clear() {
        let mut filter = BloomFilter::new(100, 0.01);

        let id = Uuid::new_v4();
        filter.add(&id);
        assert!(filter.contains(&id));

        filter.clear();
        assert!(!filter.contains(&id));
        assert_eq!(filter.fill_rate(), 0.0);
    }

    #[test]
    fn test_bloom_filter_stats() {
        let mut filter = BloomFilter::new(100, 0.01);

        for _ in 0..10 {
            filter.add(&Uuid::new_v4());
        }

        let stats = filter.stats();
        assert!(stats.size > 0);
        assert!(stats.num_hashes > 0);
        assert!(stats.bits_set > 0);
        assert!(stats.fill_rate > 0.0);
    }

    #[test]
    fn test_bloom_filter_builder() {
        let filter = BloomFilterBuilder::new()
            .expected_items(500)
            .false_positive_rate(0.001)
            .build();

        assert!(filter.size > 0);
        assert!(filter.num_hashes > 0);
    }

    #[test]
    fn test_estimated_false_positive_rate() {
        let filter = BloomFilter::new(100, 0.01);
        let rate = filter.estimated_false_positive_rate(50);

        // Rate should be between 0 and 1
        assert!(rate >= 0.0);
        assert!(rate <= 1.0);
    }

    #[test]
    fn test_optimal_size_calculation() {
        let size = BloomFilter::optimal_size(1000, 0.01);
        assert!(size > 0);

        // Larger expected items should result in larger size
        let size2 = BloomFilter::optimal_size(10000, 0.01);
        assert!(size2 > size);
    }

    #[test]
    fn test_optimal_num_hashes() {
        let num = BloomFilter::optimal_num_hashes(1000, 100);
        assert!(num > 0);
    }
}
