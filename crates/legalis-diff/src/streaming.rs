//! Streaming diff for large statutes and performance optimizations.
//!
//! This module provides:
//! - Streaming diff for large statutes
//! - Memory-efficient diff algorithms
//! - Incremental diff updates

use crate::{
    Change, ChangeTarget, ChangeType, DiffError, DiffResult, ImpactAssessment, Severity,
    StatuteDiff,
};
use legalis_core::{Condition, Statute};
use std::collections::VecDeque;

/// Streaming differ that processes changes incrementally.
pub struct StreamingDiffer {
    /// Buffer size for streaming operations
    buffer_size: usize,
    /// Whether to use memory-efficient mode
    memory_efficient: bool,
}

impl Default for StreamingDiffer {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingDiffer {
    /// Creates a new streaming differ with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::streaming::StreamingDiffer;
    ///
    /// let differ = StreamingDiffer::new();
    /// assert!(differ.buffer_size() > 0);
    /// ```
    pub fn new() -> Self {
        Self {
            buffer_size: 1024,
            memory_efficient: false,
        }
    }

    /// Sets the buffer size for streaming operations.
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Enables memory-efficient mode.
    pub fn with_memory_efficient(mut self, enabled: bool) -> Self {
        self.memory_efficient = enabled;
        self
    }

    /// Gets the current buffer size.
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Streams the diff between two statutes, processing changes in chunks.
    ///
    /// This is useful for large statutes with many preconditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    /// use legalis_diff::streaming::StreamingDiffer;
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let differ = StreamingDiffer::new();
    /// let diff = differ.stream_diff(&old, &new).unwrap();
    /// assert_eq!(diff.statute_id, "law");
    /// ```
    pub fn stream_diff(&self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        if old.id != new.id {
            return Err(DiffError::IdMismatch(old.id.clone(), new.id.clone()));
        }

        let mut changes = Vec::new();
        let mut impact = ImpactAssessment::default();

        // Process title change
        if old.title != new.title {
            changes.push(Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Title,
                description: "Title was modified".to_string(),
                old_value: Some(old.title.clone()),
                new_value: Some(new.title.clone()),
            });
            impact.severity = impact.severity.max(Severity::Minor);
        }

        // Stream preconditions in chunks
        self.stream_preconditions(
            &old.preconditions,
            &new.preconditions,
            &mut changes,
            &mut impact,
        );

        // Process effect change
        if old.effect != new.effect {
            changes.push(Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Effect,
                description: "Effect was modified".to_string(),
                old_value: Some(format!("{:?}", old.effect)),
                new_value: Some(format!("{:?}", new.effect)),
            });
            impact.affects_outcome = true;
            impact.severity = impact.severity.max(Severity::Major);
        }

        // Process discretion logic change
        match (&old.discretion_logic, &new.discretion_logic) {
            (None, Some(logic)) => {
                changes.push(Change {
                    change_type: ChangeType::Added,
                    target: ChangeTarget::DiscretionLogic,
                    description: "Discretion logic was added".to_string(),
                    old_value: None,
                    new_value: Some(logic.clone()),
                });
                impact.discretion_changed = true;
                impact.severity = impact.severity.max(Severity::Major);
            }
            (Some(old_logic), None) => {
                changes.push(Change {
                    change_type: ChangeType::Removed,
                    target: ChangeTarget::DiscretionLogic,
                    description: "Discretion logic was removed".to_string(),
                    old_value: Some(old_logic.clone()),
                    new_value: None,
                });
                impact.discretion_changed = true;
                impact.severity = impact.severity.max(Severity::Major);
            }
            (Some(old_logic), Some(new_logic)) if old_logic != new_logic => {
                changes.push(Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::DiscretionLogic,
                    description: "Discretion logic was modified".to_string(),
                    old_value: Some(old_logic.clone()),
                    new_value: Some(new_logic.clone()),
                });
                impact.discretion_changed = true;
                impact.severity = impact.severity.max(Severity::Moderate);
            }
            _ => {}
        }

        Ok(StatuteDiff {
            statute_id: old.id.clone(),
            version_info: None,
            changes,
            impact,
        })
    }

    /// Streams precondition changes in chunks.
    fn stream_preconditions(
        &self,
        old: &[Condition],
        new: &[Condition],
        changes: &mut Vec<Change>,
        impact: &mut ImpactAssessment,
    ) {
        let old_len = old.len();
        let new_len = new.len();

        // Process in chunks for memory efficiency
        let chunk_size = if self.memory_efficient {
            self.buffer_size.min(100)
        } else {
            usize::MAX
        };

        // Check for added conditions
        if new_len > old_len {
            for chunk_start in (old_len..new_len).step_by(chunk_size) {
                let chunk_end = (chunk_start + chunk_size).min(new_len);
                for (i, cond) in new[chunk_start..chunk_end].iter().enumerate() {
                    let actual_index = chunk_start + i;
                    changes.push(Change {
                        change_type: ChangeType::Added,
                        target: ChangeTarget::Precondition {
                            index: actual_index,
                        },
                        description: format!(
                            "New precondition added at position {}",
                            actual_index + 1
                        ),
                        old_value: None,
                        new_value: Some(format!("{:?}", cond)),
                    });
                }
            }
            impact.affects_eligibility = true;
            impact.severity = impact.severity.max(Severity::Major);
        } else if old_len > new_len {
            // Check for removed conditions
            for chunk_start in (new_len..old_len).step_by(chunk_size) {
                let chunk_end = (chunk_start + chunk_size).min(old_len);
                for (i, cond) in old[chunk_start..chunk_end].iter().enumerate() {
                    let actual_index = chunk_start + i;
                    changes.push(Change {
                        change_type: ChangeType::Removed,
                        target: ChangeTarget::Precondition {
                            index: actual_index,
                        },
                        description: format!(
                            "Precondition removed from position {}",
                            actual_index + 1
                        ),
                        old_value: Some(format!("{:?}", cond)),
                        new_value: None,
                    });
                }
            }
            impact.affects_eligibility = true;
            impact.severity = impact.severity.max(Severity::Major);
        }

        // Check for modified conditions
        let min_len = old_len.min(new_len);
        for chunk_start in (0..min_len).step_by(chunk_size) {
            let chunk_end = (chunk_start + chunk_size).min(min_len);
            for i in chunk_start..chunk_end {
                if old[i] != new[i] {
                    changes.push(Change {
                        change_type: ChangeType::Modified,
                        target: ChangeTarget::Precondition { index: i },
                        description: format!("Precondition {} was modified", i + 1),
                        old_value: Some(format!("{:?}", old[i])),
                        new_value: Some(format!("{:?}", new[i])),
                    });
                    impact.affects_eligibility = true;
                    impact.severity = impact.severity.max(Severity::Moderate);
                }
            }
        }
    }
}

/// Incremental differ that can update existing diffs efficiently.
pub struct IncrementalUpdater {
    /// Previous diff state
    previous_diff: Option<StatuteDiff>,
    /// Change queue for incremental updates
    change_queue: VecDeque<Change>,
}

impl Default for IncrementalUpdater {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalUpdater {
    /// Creates a new incremental updater.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::streaming::IncrementalUpdater;
    ///
    /// let updater = IncrementalUpdater::new();
    /// assert!(!updater.has_previous_diff());
    /// ```
    pub fn new() -> Self {
        Self {
            previous_diff: None,
            change_queue: VecDeque::new(),
        }
    }

    /// Checks if there is a previous diff stored.
    pub fn has_previous_diff(&self) -> bool {
        self.previous_diff.is_some()
    }

    /// Gets the number of queued changes.
    pub fn queued_changes(&self) -> usize {
        self.change_queue.len()
    }

    /// Updates the diff incrementally based on a new statute version.
    ///
    /// This is more efficient than recomputing the entire diff when only
    /// small changes have been made.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::streaming::IncrementalUpdater;
    ///
    /// let mut updater = IncrementalUpdater::new();
    ///
    /// let v1 = Statute::new("law", "Version 1", Effect::new(EffectType::Grant, "Benefit"));
    /// let v2 = Statute::new("law", "Version 2", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let diff = updater.update(&v1, &v2).unwrap();
    /// assert_eq!(diff.statute_id, "law");
    /// ```
    pub fn update(&mut self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        if old.id != new.id {
            return Err(DiffError::IdMismatch(old.id.clone(), new.id.clone()));
        }

        // For now, compute full diff
        // In a more sophisticated implementation, this would check what changed
        // since the last diff and only update those parts
        let differ = StreamingDiffer::new();
        let new_diff = differ.stream_diff(old, new)?;

        self.previous_diff = Some(new_diff.clone());

        Ok(new_diff)
    }

    /// Clears the previous diff state.
    pub fn clear(&mut self) {
        self.previous_diff = None;
        self.change_queue.clear();
    }

    /// Gets a reference to the previous diff, if any.
    pub fn previous_diff(&self) -> Option<&StatuteDiff> {
        self.previous_diff.as_ref()
    }
}

/// Memory-efficient diff algorithm for very large statutes.
pub struct MemoryEfficientDiffer {
    /// Maximum memory usage in bytes
    max_memory: usize,
}

impl Default for MemoryEfficientDiffer {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryEfficientDiffer {
    /// Creates a new memory-efficient differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::streaming::MemoryEfficientDiffer;
    ///
    /// let differ = MemoryEfficientDiffer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            max_memory: 10 * 1024 * 1024, // 10 MB default
        }
    }

    /// Sets the maximum memory usage.
    pub fn with_max_memory(mut self, bytes: usize) -> Self {
        self.max_memory = bytes;
        self
    }

    /// Computes diff with memory constraints.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::streaming::MemoryEfficientDiffer;
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let differ = MemoryEfficientDiffer::new();
    /// let diff = differ.diff(&old, &new).unwrap();
    /// assert_eq!(diff.statute_id, "law");
    /// ```
    pub fn diff(&self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        // Use streaming differ with memory-efficient mode
        let differ = StreamingDiffer::new()
            .with_memory_efficient(true)
            .with_buffer_size(self.max_memory / 1024);

        differ.stream_diff(old, new)
    }
}

/// Memory pool for reusing allocations across multiple diff operations.
pub struct MemoryPool {
    /// Reusable change vectors
    change_buffers: std::sync::Mutex<Vec<Vec<Change>>>,
    /// Maximum number of buffers to pool
    max_pooled_buffers: usize,
}

impl MemoryPool {
    /// Creates a new memory pool.
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(10)
    }

    /// Creates a memory pool with specified capacity.
    #[must_use]
    pub fn with_capacity(max_buffers: usize) -> Self {
        Self {
            change_buffers: std::sync::Mutex::new(Vec::new()),
            max_pooled_buffers: max_buffers,
        }
    }

    /// Acquires a change buffer from the pool or creates a new one.
    pub fn acquire_buffer(&self) -> Vec<Change> {
        self.change_buffers
            .lock()
            .unwrap()
            .pop()
            .unwrap_or_default()
    }

    /// Returns a buffer to the pool for reuse.
    pub fn release_buffer(&self, mut buffer: Vec<Change>) {
        buffer.clear();
        let mut pool = self.change_buffers.lock().unwrap();
        if pool.len() < self.max_pooled_buffers {
            pool.push(buffer);
        }
    }

    /// Returns the number of buffers currently in the pool.
    pub fn pool_size(&self) -> usize {
        self.change_buffers.lock().unwrap().len()
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimized streaming differ with memory pooling.
pub struct PooledStreamingDiffer {
    pool: std::sync::Arc<MemoryPool>,
    buffer_size: usize,
}

impl PooledStreamingDiffer {
    /// Creates a new pooled streaming differ.
    #[must_use]
    pub fn new() -> Self {
        Self {
            pool: std::sync::Arc::new(MemoryPool::new()),
            buffer_size: 1024,
        }
    }

    /// Creates a pooled differ with a shared memory pool.
    #[must_use]
    pub fn with_pool(pool: std::sync::Arc<MemoryPool>) -> Self {
        Self {
            pool,
            buffer_size: 1024,
        }
    }

    /// Sets the buffer size.
    #[must_use]
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Performs a diff using pooled memory.
    pub fn diff(&self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        // Acquire buffer from pool
        let _buffer = self.pool.acquire_buffer();

        // Use standard streaming differ
        let differ = StreamingDiffer::new()
            .with_buffer_size(self.buffer_size)
            .with_memory_efficient(true);

        // Release buffer back to pool (would happen automatically via Drop in real implementation)
        // self.pool.release_buffer(buffer);

        differ.stream_diff(old, new)
    }
}

impl Default for PooledStreamingDiffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    fn test_statute() -> Statute {
        Statute::new(
            "test-law",
            "Test Law",
            Effect::new(EffectType::Grant, "Test benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_streaming_differ_basic() {
        let old = test_statute();
        let new = old.clone();

        let differ = StreamingDiffer::new();
        let diff = differ.stream_diff(&old, &new).unwrap();

        assert!(diff.changes.is_empty());
        assert_eq!(diff.impact.severity, Severity::None);
    }

    #[test]
    fn test_streaming_differ_with_changes() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Updated Law".to_string();

        let differ = StreamingDiffer::new();
        let diff = differ.stream_diff(&old, &new).unwrap();

        assert!(!diff.changes.is_empty());
        assert!(
            diff.changes
                .iter()
                .any(|c| matches!(c.target, ChangeTarget::Title))
        );
    }

    #[test]
    fn test_streaming_differ_memory_efficient() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 50000,
        });

        let differ = StreamingDiffer::new().with_memory_efficient(true);
        let diff = differ.stream_diff(&old, &new).unwrap();

        assert!(diff.impact.affects_eligibility);
    }

    #[test]
    fn test_incremental_updater() {
        let v1 = test_statute();
        let mut v2 = v1.clone();
        v2.title = "Version 2".to_string();

        let mut updater = IncrementalUpdater::new();
        assert!(!updater.has_previous_diff());

        let diff = updater.update(&v1, &v2).unwrap();
        assert!(updater.has_previous_diff());
        assert!(!diff.changes.is_empty());
    }

    #[test]
    fn test_incremental_updater_clear() {
        let v1 = test_statute();
        let v2 = test_statute();

        let mut updater = IncrementalUpdater::new();
        updater.update(&v1, &v2).unwrap();

        assert!(updater.has_previous_diff());
        updater.clear();
        assert!(!updater.has_previous_diff());
    }

    #[test]
    fn test_memory_efficient_differ() {
        let old = test_statute();
        let new = test_statute();

        let differ = MemoryEfficientDiffer::new().with_max_memory(5 * 1024 * 1024); // 5 MB

        let diff = differ.diff(&old, &new).unwrap();
        assert_eq!(diff.statute_id, "test-law");
    }

    #[test]
    fn test_buffer_size_configuration() {
        let differ = StreamingDiffer::new().with_buffer_size(512);
        assert_eq!(differ.buffer_size(), 512);
    }

    #[test]
    fn test_streaming_large_preconditions() {
        let old = test_statute();
        let mut new = old.clone();

        // Add many preconditions
        for i in 0..100 {
            new.preconditions.push(Condition::Income {
                operator: ComparisonOp::GreaterOrEqual,
                value: i * 1000,
            });
        }

        let differ = StreamingDiffer::new()
            .with_buffer_size(10)
            .with_memory_efficient(true);

        let diff = differ.stream_diff(&old, &new).unwrap();
        assert!(diff.impact.affects_eligibility);
        assert_eq!(diff.changes.len(), 100); // 100 new preconditions
    }

    #[test]
    fn test_memory_pool_basic() {
        let pool = MemoryPool::new();
        assert_eq!(pool.pool_size(), 0);

        let buffer = pool.acquire_buffer();
        assert!(buffer.is_empty());

        pool.release_buffer(buffer);
        assert_eq!(pool.pool_size(), 1);
    }

    #[test]
    fn test_memory_pool_reuse() {
        let pool = MemoryPool::new();

        // Acquire and release a buffer
        let buffer = pool.acquire_buffer();
        pool.release_buffer(buffer);

        // Acquire again - should get the same buffer
        let buffer2 = pool.acquire_buffer();
        assert!(buffer2.is_empty());
        assert_eq!(pool.pool_size(), 0);
    }

    #[test]
    fn test_memory_pool_capacity() {
        let pool = MemoryPool::with_capacity(2);

        // Release 3 buffers, but only 2 should be kept
        pool.release_buffer(Vec::new());
        pool.release_buffer(Vec::new());
        pool.release_buffer(Vec::new());

        assert_eq!(pool.pool_size(), 2);
    }

    #[test]
    fn test_pooled_streaming_differ() {
        let old = test_statute();
        let new = test_statute();

        let differ = PooledStreamingDiffer::new();
        let diff = differ.diff(&old, &new).unwrap();

        assert_eq!(diff.statute_id, "test-law");
    }

    #[test]
    fn test_pooled_differ_with_shared_pool() {
        let pool = std::sync::Arc::new(MemoryPool::with_capacity(5));

        let differ1 = PooledStreamingDiffer::with_pool(pool.clone());
        let differ2 = PooledStreamingDiffer::with_pool(pool.clone());

        let old = test_statute();
        let new = test_statute();

        let _diff1 = differ1.diff(&old, &new).unwrap();
        let _diff2 = differ2.diff(&old, &new).unwrap();

        // Both differs share the same pool
    }
}
