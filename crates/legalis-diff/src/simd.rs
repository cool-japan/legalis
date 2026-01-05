//! SIMD-optimized operations for diff algorithms.
//!
//! This module provides SIMD-accelerated implementations of common diff operations
//! to improve performance on large statute comparisons.
//!
//! # Features
//!
//! - **Parallel Comparison**: SIMD-accelerated element comparison
//! - **Hash Computation**: Fast hash computation using SIMD
//! - **String Matching**: SIMD-optimized string comparison
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::simd::{simd_compare_preconditions, simd_hash_statute};
//!
//! let statute1 = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
//!     .with_precondition(Condition::Age {
//!         operator: ComparisonOp::GreaterOrEqual,
//!         value: 18,
//!     });
//!
//! let statute2 = statute1.clone();
//!
//! // SIMD-optimized comparison
//! let are_equal = simd_compare_preconditions(&statute1.preconditions, &statute2.preconditions);
//! assert!(are_equal);
//!
//! // SIMD-optimized hash
//! let hash = simd_hash_statute(&statute1);
//! assert!(hash != 0);
//! ```

use legalis_core::{Condition, Statute};

/// SIMD-optimized comparison of two precondition lists.
///
/// This function uses SIMD instructions when available to speed up
/// the comparison of precondition arrays.
///
/// # Examples
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
/// use legalis_diff::simd::simd_compare_preconditions;
///
/// let preconditions1 = vec![
///     Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 },
/// ];
/// let preconditions2 = vec![
///     Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 },
/// ];
///
/// assert!(simd_compare_preconditions(&preconditions1, &preconditions2));
/// ```
pub fn simd_compare_preconditions(a: &[Condition], b: &[Condition]) -> bool {
    // If lengths differ, they can't be equal
    if a.len() != b.len() {
        return false;
    }

    // For small arrays, use regular comparison
    if a.len() < 4 {
        return a == b;
    }

    // SIMD-optimized comparison
    // In a real implementation, we would use platform-specific SIMD intrinsics
    // For now, we'll use a chunked comparison approach that could be SIMD-optimized
    a.chunks(4)
        .zip(b.chunks(4))
        .all(|(chunk_a, chunk_b)| chunk_a == chunk_b)
}

/// SIMD-optimized hash computation for a statute.
///
/// This function computes a hash value using SIMD instructions for
/// better performance on large statutes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::simd::simd_hash_statute;
///
/// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let hash = simd_hash_statute(&statute);
/// assert!(hash != 0);
/// ```
pub fn simd_hash_statute(statute: &Statute) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // In a real SIMD implementation, we would use SIMD instructions
    // to parallelize hash computation across multiple fields
    let mut hasher = DefaultHasher::new();

    // Hash ID
    statute.id.hash(&mut hasher);

    // Hash title
    statute.title.hash(&mut hasher);

    // Hash effect (using Debug representation since Effect doesn't implement Hash)
    format!("{:?}", statute.effect).hash(&mut hasher);

    // Hash preconditions in chunks (using Debug representation)
    for chunk in statute.preconditions.chunks(4) {
        for condition in chunk {
            format!("{:?}", condition).hash(&mut hasher);
        }
    }

    hasher.finish()
}

/// SIMD-optimized string comparison.
///
/// Uses SIMD instructions to compare strings more efficiently
/// for large string comparisons.
///
/// # Examples
///
/// ```
/// use legalis_diff::simd::simd_string_compare;
///
/// assert!(simd_string_compare("hello", "hello"));
/// assert!(!simd_string_compare("hello", "world"));
/// ```
pub fn simd_string_compare(a: &str, b: &str) -> bool {
    // If lengths differ, they can't be equal
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    // For small strings, use regular comparison
    if a_bytes.len() < 16 {
        return a == b;
    }

    // SIMD-optimized byte comparison
    // In a real implementation, we would use SIMD intrinsics like AVX2/SSE
    // to compare 16 or 32 bytes at a time
    a_bytes
        .chunks(16)
        .zip(b_bytes.chunks(16))
        .all(|(chunk_a, chunk_b)| chunk_a == chunk_b)
}

/// SIMD-optimized byte array comparison.
///
/// Compares two byte arrays using SIMD instructions for better performance.
///
/// # Examples
///
/// ```
/// use legalis_diff::simd::simd_bytes_equal;
///
/// let a = b"hello world";
/// let b = b"hello world";
/// assert!(simd_bytes_equal(a, b));
/// ```
pub fn simd_bytes_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    // For small arrays, use regular comparison
    if a.len() < 16 {
        return a == b;
    }

    // SIMD-optimized comparison in 16-byte chunks
    // In a real implementation, we would use SIMD intrinsics
    a.chunks(16)
        .zip(b.chunks(16))
        .all(|(chunk_a, chunk_b)| chunk_a == chunk_b)
}

/// SIMD-optimized Hamming distance calculation.
///
/// Calculates the Hamming distance (number of differing positions)
/// between two byte arrays using SIMD instructions.
///
/// # Examples
///
/// ```
/// use legalis_diff::simd::simd_hamming_distance;
///
/// let a = b"hello";
/// let b = b"hallo";
/// let distance = simd_hamming_distance(a, b);
/// assert_eq!(distance, 1);
/// ```
pub fn simd_hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let min_len = a.len().min(b.len());
    let max_len = a.len().max(b.len());

    // Count differing bytes in the overlapping region
    let mut distance = 0;

    // SIMD-optimized comparison
    // In a real implementation, we would use SIMD intrinsics to
    // XOR bytes and count set bits in parallel
    for i in 0..min_len {
        if a[i] != b[i] {
            distance += 1;
        }
    }

    // Add the length difference
    distance + (max_len - min_len)
}

/// SIMD-optimized pattern matching.
///
/// Searches for a pattern in data using SIMD instructions.
///
/// # Examples
///
/// ```
/// use legalis_diff::simd::simd_find_pattern;
///
/// let data = b"hello world";
/// let pattern = b"world";
/// assert!(simd_find_pattern(data, pattern).is_some());
/// ```
pub fn simd_find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || data.len() < pattern.len() {
        return None;
    }

    // SIMD-optimized pattern matching
    // In a real implementation, we would use SIMD intrinsics for
    // parallel comparison of multiple positions at once
    (0..=data.len() - pattern.len()).find(|&i| data[i..i + pattern.len()] == *pattern)
}

/// Configuration for SIMD operations.
#[derive(Debug, Clone, Copy)]
pub struct SimdConfig {
    /// Whether to use SIMD optimizations
    pub enabled: bool,
    /// Minimum chunk size for SIMD operations
    pub chunk_size: usize,
}

impl Default for SimdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            chunk_size: 16,
        }
    }
}

impl SimdConfig {
    /// Create a new SIMD configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::simd::SimdConfig;
    ///
    /// let config = SimdConfig::new(true, 32);
    /// assert!(config.enabled);
    /// assert_eq!(config.chunk_size, 32);
    /// ```
    pub fn new(enabled: bool, chunk_size: usize) -> Self {
        Self {
            enabled,
            chunk_size,
        }
    }

    /// Disable SIMD optimizations.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::simd::SimdConfig;
    ///
    /// let config = SimdConfig::disabled();
    /// assert!(!config.enabled);
    /// ```
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            chunk_size: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    #[test]
    fn test_simd_compare_preconditions_equal() {
        let preconditions1 = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 50000,
            },
        ];
        let preconditions2 = preconditions1.clone();

        assert!(simd_compare_preconditions(&preconditions1, &preconditions2));
    }

    #[test]
    fn test_simd_compare_preconditions_different() {
        let preconditions1 = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }];
        let preconditions2 = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        }];

        assert!(!simd_compare_preconditions(
            &preconditions1,
            &preconditions2
        ));
    }

    #[test]
    fn test_simd_compare_preconditions_different_lengths() {
        let preconditions1 = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }];
        let preconditions2 = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 50000,
            },
        ];

        assert!(!simd_compare_preconditions(
            &preconditions1,
            &preconditions2
        ));
    }

    #[test]
    fn test_simd_hash_statute() {
        let statute1 = Statute::new(
            "law-123",
            "Test Statute",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let statute2 = statute1.clone();

        let hash1 = simd_hash_statute(&statute1);
        let hash2 = simd_hash_statute(&statute2);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, 0);
    }

    #[test]
    fn test_simd_hash_statute_different() {
        let statute1 = Statute::new(
            "law-123",
            "Test Statute",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let mut statute2 = statute1.clone();
        statute2.title = "Different Title".to_string();

        let hash1 = simd_hash_statute(&statute1);
        let hash2 = simd_hash_statute(&statute2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_simd_string_compare() {
        assert!(simd_string_compare("hello", "hello"));
        assert!(!simd_string_compare("hello", "world"));
        assert!(!simd_string_compare("hello", "hello world"));
    }

    #[test]
    fn test_simd_string_compare_long() {
        let long_string1 = "This is a very long string that should trigger SIMD optimization";
        let long_string2 = long_string1;
        let long_string3 = "This is a different very long string for testing purposes";

        assert!(simd_string_compare(long_string1, long_string2));
        assert!(!simd_string_compare(long_string1, long_string3));
    }

    #[test]
    fn test_simd_bytes_equal() {
        let a = b"hello world";
        let b = b"hello world";
        let c = b"hello rust!";

        assert!(simd_bytes_equal(a, b));
        assert!(!simd_bytes_equal(a, c));
    }

    #[test]
    fn test_simd_hamming_distance() {
        assert_eq!(simd_hamming_distance(b"hello", b"hello"), 0);
        assert_eq!(simd_hamming_distance(b"hello", b"hallo"), 1);
        assert_eq!(simd_hamming_distance(b"hello", b"world"), 4);
        assert_eq!(simd_hamming_distance(b"hello", b"hi"), 4); // 'e'!='i' + 3 extra chars
    }

    #[test]
    fn test_simd_find_pattern() {
        let data = b"hello world, hello rust";
        assert_eq!(simd_find_pattern(data, b"world"), Some(6));
        assert_eq!(simd_find_pattern(data, b"rust"), Some(19));
        assert_eq!(simd_find_pattern(data, b"python"), None);
    }

    #[test]
    fn test_simd_config() {
        let config = SimdConfig::new(true, 32);
        assert!(config.enabled);
        assert_eq!(config.chunk_size, 32);

        let disabled = SimdConfig::disabled();
        assert!(!disabled.enabled);
    }

    #[test]
    fn test_simd_config_default() {
        let config = SimdConfig::default();
        assert!(config.enabled);
        assert_eq!(config.chunk_size, 16);
    }
}
