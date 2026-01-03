//! Async write batching for high-performance audit logging.
//!
//! This module provides async batching capabilities to improve write performance
//! by accumulating audit records and writing them in batches.

use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio::time::interval;

/// Configuration for batch writing.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of records in a batch
    pub max_batch_size: usize,
    /// Maximum time to wait before flushing a partial batch
    pub max_batch_delay: Duration,
    /// Channel capacity for pending records
    pub channel_capacity: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            max_batch_delay: Duration::from_secs(5),
            channel_capacity: 1000,
        }
    }
}

impl BatchConfig {
    /// Creates a new batch configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum batch size.
    pub fn with_max_batch_size(mut self, size: usize) -> Self {
        self.max_batch_size = size.max(1);
        self
    }

    /// Sets the maximum batch delay.
    pub fn with_max_batch_delay(mut self, delay: Duration) -> Self {
        self.max_batch_delay = delay;
        self
    }

    /// Sets the channel capacity.
    pub fn with_channel_capacity(mut self, capacity: usize) -> Self {
        self.channel_capacity = capacity.max(1);
        self
    }
}

/// Batch writer for async audit record writing.
pub struct BatchWriter {
    sender: mpsc::Sender<BatchCommand>,
    config: BatchConfig,
}

enum BatchCommand {
    Write(
        Box<AuditRecord>,
        tokio::sync::oneshot::Sender<AuditResult<()>>,
    ),
    Flush(tokio::sync::oneshot::Sender<AuditResult<()>>),
    Shutdown,
}

impl BatchWriter {
    /// Creates a new batch writer.
    pub fn new(storage: Box<dyn AuditStorage>, config: BatchConfig) -> Self {
        let (sender, receiver) = mpsc::channel(config.channel_capacity);

        let worker_config = config.clone();
        tokio::spawn(async move {
            BatchWorker::new(storage, receiver, worker_config)
                .run()
                .await;
        });

        Self { sender, config }
    }

    /// Writes a record asynchronously (non-blocking).
    pub async fn write(&self, record: AuditRecord) -> AuditResult<()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender
            .send(BatchCommand::Write(Box::new(record), tx))
            .await
            .map_err(|_| AuditError::StorageError("Batch writer channel closed".to_string()))?;
        rx.await
            .map_err(|_| AuditError::StorageError("Response channel closed".to_string()))?
    }

    /// Flushes any pending records.
    pub async fn flush(&self) -> AuditResult<()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender
            .send(BatchCommand::Flush(tx))
            .await
            .map_err(|_| AuditError::StorageError("Batch writer channel closed".to_string()))?;
        rx.await
            .map_err(|_| AuditError::StorageError("Response channel closed".to_string()))?
    }

    /// Shuts down the batch writer gracefully.
    pub async fn shutdown(&self) -> AuditResult<()> {
        self.sender
            .send(BatchCommand::Shutdown)
            .await
            .map_err(|_| AuditError::StorageError("Batch writer channel closed".to_string()))?;
        Ok(())
    }

    /// Returns the batch configuration.
    pub fn config(&self) -> &BatchConfig {
        &self.config
    }
}

/// Internal batch worker.
struct BatchWorker {
    storage: Arc<Mutex<Box<dyn AuditStorage>>>,
    receiver: mpsc::Receiver<BatchCommand>,
    config: BatchConfig,
    pending_batch: Vec<(
        Box<AuditRecord>,
        tokio::sync::oneshot::Sender<AuditResult<()>>,
    )>,
}

impl BatchWorker {
    fn new(
        storage: Box<dyn AuditStorage>,
        receiver: mpsc::Receiver<BatchCommand>,
        config: BatchConfig,
    ) -> Self {
        let capacity = config.max_batch_size;
        Self {
            storage: Arc::new(Mutex::new(storage)),
            receiver,
            config,
            pending_batch: Vec::with_capacity(capacity),
        }
    }

    async fn run(mut self) {
        let mut flush_timer = interval(self.config.max_batch_delay);
        flush_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                // Handle incoming commands
                cmd = self.receiver.recv() => {
                    match cmd {
                        Some(BatchCommand::Write(record, response_tx)) => {
                            self.pending_batch.push((record, response_tx));

                            // Flush if batch is full
                            if self.pending_batch.len() >= self.config.max_batch_size {
                                self.flush_batch().await;
                            }
                        }
                        Some(BatchCommand::Flush(response_tx)) => {
                            self.flush_batch().await;
                            let _ = response_tx.send(Ok(()));
                        }
                        Some(BatchCommand::Shutdown) => {
                            self.flush_batch().await;
                            break;
                        }
                        None => {
                            // Channel closed
                            self.flush_batch().await;
                            break;
                        }
                    }
                }

                // Periodic flush
                _ = flush_timer.tick() => {
                    if !self.pending_batch.is_empty() {
                        self.flush_batch().await;
                    }
                }
            }
        }
    }

    async fn flush_batch(&mut self) {
        if self.pending_batch.is_empty() {
            return;
        }

        let batch = std::mem::replace(
            &mut self.pending_batch,
            Vec::with_capacity(self.config.max_batch_size),
        );

        let mut storage = self.storage.lock().await;

        for (record, response_tx) in batch {
            let result = storage.store(*record);
            let _ = response_tx.send(result);
        }

        tracing::debug!("Flushed batch of records");
    }
}

/// Statistics for batch writing.
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    /// Total records written
    pub total_records: usize,
    /// Total batches flushed
    pub total_batches: usize,
    /// Average batch size
    pub avg_batch_size: f64,
    /// Records currently pending
    pub pending_records: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_batch_writer_basic() {
        let storage = Box::new(MemoryStorage::new());
        let config = BatchConfig::new().with_max_batch_size(10);
        let writer = BatchWriter::new(storage, config);

        let record = create_test_record();
        writer.write(record).await.unwrap();

        writer.flush().await.unwrap();
        writer.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_batch_writer_multiple_records() {
        let storage = Box::new(MemoryStorage::new());
        // Use shorter batch delay to speed up test
        let config = BatchConfig::new()
            .with_max_batch_size(5)
            .with_max_batch_delay(Duration::from_millis(100));
        let writer = BatchWriter::new(storage, config);

        for _ in 0..10 {
            let record = create_test_record();
            writer.write(record).await.unwrap();
        }

        writer.flush().await.unwrap();
        writer.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_batch_writer_auto_flush() {
        let storage = Box::new(MemoryStorage::new());
        let config = BatchConfig::new()
            .with_max_batch_size(3)
            .with_max_batch_delay(Duration::from_millis(100));
        let writer = BatchWriter::new(storage, config);

        // Write 2 records (below batch size)
        for _ in 0..2 {
            let record = create_test_record();
            writer.write(record).await.unwrap();
        }

        // Wait for auto-flush
        tokio::time::sleep(Duration::from_millis(150)).await;

        writer.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_batch_config() {
        let config = BatchConfig::new()
            .with_max_batch_size(50)
            .with_max_batch_delay(Duration::from_secs(10))
            .with_channel_capacity(500);

        assert_eq!(config.max_batch_size, 50);
        assert_eq!(config.max_batch_delay, Duration::from_secs(10));
        assert_eq!(config.channel_capacity, 500);
    }

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }
}
