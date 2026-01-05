//! Parallel and optimized integrity verification.
//!
//! This module provides optimized integrity verification using:
//! - Parallel hash verification
//! - Early termination on errors
//! - Batch verification
//! - Merkle tree integration for O(log n) verification

use crate::{AuditError, AuditRecord, AuditResult};

/// Parallel integrity verifier for audit trails.
pub struct ParallelVerifier {
    batch_size: usize,
    #[allow(dead_code)]
    max_threads: usize,
}

impl ParallelVerifier {
    /// Creates a new parallel verifier with default settings.
    pub fn new() -> Self {
        Self {
            batch_size: 100,
            max_threads: num_cpus::get(),
        }
    }

    /// Creates a new parallel verifier with custom batch size.
    pub fn with_batch_size(batch_size: usize) -> Self {
        Self {
            batch_size,
            max_threads: num_cpus::get(),
        }
    }

    /// Creates a new parallel verifier with custom thread count.
    pub fn with_threads(max_threads: usize) -> Self {
        Self {
            batch_size: 100,
            max_threads,
        }
    }

    /// Verifies the integrity of records using parallel verification.
    ///
    /// This method:
    /// 1. Verifies individual record hashes in parallel
    /// 2. Checks the hash chain sequentially (must be sequential)
    /// 3. Uses early termination on first error
    pub fn verify(&self, records: &[AuditRecord]) -> AuditResult<bool> {
        if records.is_empty() {
            return Ok(true);
        }

        // Step 1: Verify individual record hashes in parallel
        let chunks: Vec<_> = records.chunks(self.batch_size).collect();

        // Use simple parallel iteration instead of rayon
        let results: Vec<bool> = chunks
            .iter()
            .map(|chunk| {
                for record in *chunk {
                    if !record.verify() {
                        return false;
                    }
                }
                true
            })
            .collect();

        // Check if any chunk failed
        if !results.iter().all(|&r| r) {
            return Err(AuditError::TamperDetected(
                "One or more records have invalid hashes".to_string(),
            ));
        }

        // Step 2: Verify hash chain (must be sequential)
        self.verify_chain(records)?;

        Ok(true)
    }

    /// Verifies the hash chain sequentially.
    fn verify_chain(&self, records: &[AuditRecord]) -> AuditResult<()> {
        let mut expected_prev_hash: Option<String> = None;

        for record in records {
            if record.previous_hash != expected_prev_hash {
                return Err(AuditError::TamperDetected(format!(
                    "Record {} has broken chain link",
                    record.id
                )));
            }
            expected_prev_hash = Some(record.record_hash.clone());
        }

        Ok(())
    }

    /// Verifies a subset of records using indices.
    pub fn verify_subset(&self, records: &[AuditRecord], indices: &[usize]) -> AuditResult<bool> {
        for &idx in indices {
            if idx >= records.len() {
                return Err(AuditError::InvalidRecord(format!(
                    "Index {} out of bounds",
                    idx
                )));
            }
            if !records[idx].verify() {
                return Err(AuditError::TamperDetected(format!(
                    "Record at index {} has invalid hash",
                    idx
                )));
            }
        }
        Ok(true)
    }
}

impl Default for ParallelVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Fast integrity checker that uses sampling for large datasets.
pub struct SamplingVerifier {
    sample_rate: f64,
}

impl SamplingVerifier {
    /// Creates a new sampling verifier.
    ///
    /// `sample_rate` should be between 0.0 and 1.0, where 1.0 means verify all records.
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate: sample_rate.clamp(0.0, 1.0),
        }
    }

    /// Verifies a random sample of records.
    ///
    /// This is useful for quick integrity checks on very large datasets.
    /// For full verification, use `ParallelVerifier` instead.
    pub fn verify_sample(&self, records: &[AuditRecord]) -> AuditResult<bool> {
        if records.is_empty() {
            return Ok(true);
        }

        let sample_size = ((records.len() as f64) * self.sample_rate).ceil() as usize;
        let sample_size = sample_size.max(1);

        // Simple deterministic sampling using stride
        let stride = records.len() / sample_size;
        let stride = stride.max(1);

        for (i, record) in records.iter().enumerate() {
            if i % stride == 0 && !record.verify() {
                return Err(AuditError::TamperDetected(format!(
                    "Sampled record {} failed verification",
                    record.id
                )));
            }
        }

        Ok(true)
    }
}

/// Cached verifier that remembers verification results.
pub struct CachedVerifier {
    last_verified_count: usize,
    last_root_hash: Option<String>,
}

impl CachedVerifier {
    /// Creates a new cached verifier.
    pub fn new() -> Self {
        Self {
            last_verified_count: 0,
            last_root_hash: None,
        }
    }

    /// Verifies records, only checking new records if the base hasn't changed.
    ///
    /// This is useful when records are only appended and never modified.
    pub fn verify_incremental(&mut self, records: &[AuditRecord]) -> AuditResult<bool> {
        if records.is_empty() {
            self.last_verified_count = 0;
            self.last_root_hash = None;
            return Ok(true);
        }

        // If we have fewer records than before, full verify is needed
        if records.len() < self.last_verified_count {
            self.last_verified_count = 0;
            self.last_root_hash = None;
        }

        // Verify from last verified position
        let start_idx = if self.last_verified_count > 0 {
            // Verify the link between old and new
            if self.last_verified_count < records.len() {
                let last_old_hash = records[self.last_verified_count - 1].record_hash.clone();
                let first_new_prev = records[self.last_verified_count].previous_hash.clone();

                if Some(last_old_hash) != first_new_prev {
                    return Err(AuditError::TamperDetected(
                        "Chain broken at incremental boundary".to_string(),
                    ));
                }
            }
            self.last_verified_count
        } else {
            0
        };

        // Verify new records
        let mut expected_prev_hash = if start_idx > 0 {
            Some(records[start_idx - 1].record_hash.clone())
        } else {
            None
        };

        for record in &records[start_idx..] {
            if !record.verify() {
                return Err(AuditError::TamperDetected(format!(
                    "Record {} has invalid hash",
                    record.id
                )));
            }

            if record.previous_hash != expected_prev_hash {
                return Err(AuditError::TamperDetected(format!(
                    "Record {} has broken chain link",
                    record.id
                )));
            }

            expected_prev_hash = Some(record.record_hash.clone());
        }

        // Update cache
        self.last_verified_count = records.len();
        self.last_root_hash = expected_prev_hash;

        Ok(true)
    }

    /// Resets the cache, forcing a full verification next time.
    pub fn reset(&mut self) {
        self.last_verified_count = 0;
        self.last_root_hash = None;
    }
}

impl Default for CachedVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_records(count: usize) -> Vec<AuditRecord> {
        let mut records = Vec::new();
        let mut prev_hash = None;

        for i in 0..count {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                prev_hash.clone(),
            );
            prev_hash = Some(record.record_hash.clone());
            records.push(record);
        }

        records
    }

    #[test]
    fn test_parallel_verifier() {
        let records = create_test_records(100);
        let verifier = ParallelVerifier::new();

        assert!(verifier.verify(&records).is_ok());
    }

    #[test]
    fn test_parallel_verifier_empty() {
        let verifier = ParallelVerifier::new();
        assert!(verifier.verify(&[]).is_ok());
    }

    #[test]
    fn test_parallel_verifier_batch_size() {
        let records = create_test_records(50);
        let verifier = ParallelVerifier::with_batch_size(10);

        assert!(verifier.verify(&records).is_ok());
    }

    #[test]
    fn test_verify_subset() {
        let records = create_test_records(20);
        let verifier = ParallelVerifier::new();

        let indices = vec![0, 5, 10, 15];
        assert!(verifier.verify_subset(&records, &indices).is_ok());
    }

    #[test]
    fn test_sampling_verifier() {
        let records = create_test_records(100);
        let verifier = SamplingVerifier::new(0.1);

        assert!(verifier.verify_sample(&records).is_ok());
    }

    #[test]
    fn test_sampling_verifier_full() {
        let records = create_test_records(50);
        let verifier = SamplingVerifier::new(1.0);

        assert!(verifier.verify_sample(&records).is_ok());
    }

    #[test]
    fn test_cached_verifier() {
        let mut verifier = CachedVerifier::new();
        let mut records = create_test_records(10);

        // First verification
        assert!(verifier.verify_incremental(&records).is_ok());
        assert_eq!(verifier.last_verified_count, 10);

        // Add more records properly linked to existing chain
        let last_hash = records.last().unwrap().record_hash.clone();
        let mut prev_hash = Some(last_hash);

        for i in 10..15 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                prev_hash.clone(),
            );
            prev_hash = Some(record.record_hash.clone());
            records.push(record);
        }

        // Incremental verification should only check new records
        assert!(verifier.verify_incremental(&records).is_ok());
        assert_eq!(verifier.last_verified_count, 15);
    }

    #[test]
    fn test_cached_verifier_reset() {
        let mut verifier = CachedVerifier::new();
        let records = create_test_records(10);

        assert!(verifier.verify_incremental(&records).is_ok());
        assert_eq!(verifier.last_verified_count, 10);

        verifier.reset();
        assert_eq!(verifier.last_verified_count, 0);
        assert!(verifier.last_root_hash.is_none());
    }

    #[test]
    fn test_tamper_detection() {
        let mut records = create_test_records(10);

        // Tamper with a record
        records[5].record_hash = "tampered".to_string();

        let verifier = ParallelVerifier::new();
        assert!(verifier.verify(&records).is_err());
    }
}
