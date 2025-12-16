//! Stress tests for verifying system behavior under load.
//!
//! These tests validate:
//! - Large population handling
//! - Memory limits
//! - Performance characteristics
//! - Parallel processing scalability

#[cfg(test)]
mod tests {
    use crate::{
        BatchConfig, BatchIterator, CheckpointManager, DirtyTracker, EntityPool, StreamingProcessor,
    };
    use legalis_core::{BasicEntity, LegalEntity};
    use std::sync::Arc;

    #[test]
    fn stress_test_large_batch_processing() {
        // Test processing 100,000 items in batches
        let items: Vec<i32> = (0..100_000).collect();
        let config = BatchConfig::default().with_batch_size(1000);

        let mut batch_iter = BatchIterator::new(items, config.batch_size);
        let mut total_processed = 0;

        while let Some(batch) = batch_iter.next() {
            total_processed += batch.len();
        }

        assert_eq!(total_processed, 100_000);
    }

    #[test]
    fn stress_test_entity_pool_recycling() {
        // Test recycling entities by acquiring multiple at once
        let mut pool = EntityPool::new(1000);
        let mut entities = Vec::new();

        // Acquire 1000 entities
        for _ in 0..1000 {
            entities.push(pool.acquire(|| BasicEntity::new()));
        }

        // Release them all back to pool
        for entity in entities {
            pool.release(entity);
        }

        // Pool should now be at capacity
        assert_eq!(pool.size(), 1000);

        // Acquire and release many more times to verify recycling
        let mut ids_seen = std::collections::HashSet::new();
        for _ in 0..5_000 {
            let entity = pool.acquire(|| BasicEntity::new());
            ids_seen.insert(entity.id());
            pool.release(entity);
        }

        // Should have reused entities (fewer unique IDs than total acquisitions)
        // Since we're recycling from a pool of 1000, we should see ~1000 unique IDs
        assert!(ids_seen.len() <= 1100); // Allow some margin for new creations
    }

    #[test]
    fn stress_test_dirty_tracking_scale() {
        // Test tracking 50,000 dirty entities
        let mut tracker = DirtyTracker::new();
        let ids: Vec<uuid::Uuid> = (0..50_000).map(|_| uuid::Uuid::new_v4()).collect();

        tracker.mark_many_dirty(ids.clone());
        assert_eq!(tracker.dirty_count(), 50_000);

        // Verify all IDs are tracked
        for id in &ids[0..1000] {
            // Spot check first 1000
            assert!(tracker.is_dirty(id));
        }

        tracker.clear();
        assert_eq!(tracker.dirty_count(), 0);
    }

    #[test]
    fn stress_test_checkpoint_manager_capacity() {
        // Test checkpoint manager with many checkpoints
        let mut manager = CheckpointManager::with_max_checkpoints(100);

        // Create 200 checkpoints
        for i in 0..200 {
            let checkpoint =
                crate::Checkpoint::new(format!("cp-{}", i), crate::SimulationMetrics::new());
            manager.save(checkpoint);
        }

        // Should only keep last 100
        assert_eq!(manager.count(), 100);

        // Latest checkpoints should be available
        for i in 100..200 {
            assert!(manager.load(&format!("cp-{}", i)).is_some());
        }

        // Earliest checkpoints should be removed
        for i in 0..100 {
            assert!(manager.load(&format!("cp-{}", i)).is_none());
        }
    }

    #[test]
    fn stress_test_streaming_processor_large_population() {
        // Test streaming processor with 20,000 entities
        let processor = StreamingProcessor::new(1000);

        let entities: Vec<Arc<dyn legalis_core::LegalEntity>> = (0..20_000)
            .map(|_| Arc::new(BasicEntity::new()) as Arc<dyn legalis_core::LegalEntity>)
            .collect();

        let results = processor.process(entities, |entity| entity.id());

        assert_eq!(results.len(), 20_000);
    }

    #[test]
    #[ignore] // Ignore by default as this is memory-intensive
    fn stress_test_memory_limit_million_entities() {
        // Test with 1 million entities - only run manually
        let mut pool = EntityPool::new(10_000);
        let mut count = 0;

        for _ in 0..1_000_000 {
            let entity = pool.acquire(|| BasicEntity::new());
            count += 1;
            pool.release(entity);
        }

        assert_eq!(count, 1_000_000);
        assert_eq!(pool.size(), 10_000);
    }

    #[test]
    fn stress_test_concurrent_dirty_tracking() {
        // Test thread-safe dirty tracking (simulated)
        let mut trackers: Vec<DirtyTracker> = (0..10).map(|_| DirtyTracker::new()).collect();

        // Each tracker handles 5,000 entities
        for tracker in trackers.iter_mut() {
            let ids: Vec<uuid::Uuid> = (0..5_000).map(|_| uuid::Uuid::new_v4()).collect();
            tracker.mark_many_dirty(ids);
            assert_eq!(tracker.dirty_count(), 5_000);
        }

        // Verify total across all trackers
        let total_dirty: usize = trackers.iter().map(|t| t.dirty_count()).sum();
        assert_eq!(total_dirty, 50_000);
    }
}
