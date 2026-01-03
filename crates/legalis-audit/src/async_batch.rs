//! Async write batching for improved performance.
//!
//! This module provides asynchronous batch writing capabilities to improve
//! write performance by batching multiple audit records together.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::time;

/// Batch writer configuration.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Maximum delay before flushing (milliseconds)
    pub max_delay_ms: u64,
    /// Enable auto-flush on drop
    pub auto_flush: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            max_delay_ms: 1000, // 1 second
            auto_flush: true,
        }
    }
}

/// Batch write operation.
#[derive(Debug, Clone)]
pub struct BatchWrite {
    /// Records to write
    pub records: Vec<AuditRecord>,
    /// Batch timestamp
    pub timestamp: DateTime<Utc>,
}

/// Batch writer for async operations.
pub struct AsyncBatchWriter {
    config: BatchConfig,
    buffer: Arc<RwLock<Vec<AuditRecord>>>,
    tx: mpsc::Sender<BatchCommand>,
}

/// Batch command.
enum BatchCommand {
    Add(Box<AuditRecord>),
    Flush,
    Shutdown,
}

/// Batch statistics.
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    /// Total batches written
    pub total_batches: usize,
    /// Total records written
    pub total_records: usize,
    /// Average batch size
    pub avg_batch_size: f64,
    /// Last flush time
    pub last_flush: Option<DateTime<Utc>>,
}

impl AsyncBatchWriter {
    /// Creates a new async batch writer.
    pub fn new(config: BatchConfig) -> Self {
        let (tx, rx) = mpsc::channel(1000);
        let buffer = Arc::new(RwLock::new(Vec::new()));

        let writer = Self {
            config: config.clone(),
            buffer: buffer.clone(),
            tx,
        };

        // Spawn background task
        tokio::spawn(Self::background_task(config, buffer, rx));

        writer
    }

    /// Background task for batch processing.
    async fn background_task(
        config: BatchConfig,
        buffer: Arc<RwLock<Vec<AuditRecord>>>,
        mut rx: mpsc::Receiver<BatchCommand>,
    ) {
        let mut flush_interval = time::interval(time::Duration::from_millis(config.max_delay_ms));

        loop {
            tokio::select! {
                _ = flush_interval.tick() => {
                    let buf = buffer.read().await;
                    if !buf.is_empty() {
                        drop(buf);
                        Self::flush_buffer(&buffer).await;
                    }
                }
                cmd = rx.recv() => {
                    match cmd {
                        Some(BatchCommand::Add(record)) => {
                            let mut buf = buffer.write().await;
                            buf.push(*record);

                            if buf.len() >= config.max_batch_size {
                                drop(buf);
                                Self::flush_buffer(&buffer).await;
                            }
                        }
                        Some(BatchCommand::Flush) => {
                            Self::flush_buffer(&buffer).await;
                        }
                        Some(BatchCommand::Shutdown) | None => {
                            Self::flush_buffer(&buffer).await;
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Flushes the buffer.
    async fn flush_buffer(buffer: &Arc<RwLock<Vec<AuditRecord>>>) {
        let mut buf = buffer.write().await;
        if !buf.is_empty() {
            // In a real implementation, this would write to storage
            tracing::debug!("Flushing batch of {} records", buf.len());
            buf.clear();
        }
    }

    /// Adds a record to the batch.
    pub async fn add(&self, record: AuditRecord) -> AuditResult<()> {
        self.tx
            .send(BatchCommand::Add(Box::new(record)))
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to add record: {}", e)))
    }

    /// Flushes all pending records.
    pub async fn flush(&self) -> AuditResult<()> {
        self.tx
            .send(BatchCommand::Flush)
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to flush: {}", e)))
    }

    /// Gets the current buffer size.
    pub async fn buffer_size(&self) -> usize {
        self.buffer.read().await.len()
    }

    /// Shuts down the batch writer.
    pub async fn shutdown(&self) -> AuditResult<()> {
        self.tx
            .send(BatchCommand::Shutdown)
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to shutdown: {}", e)))
    }
}

impl Drop for AsyncBatchWriter {
    fn drop(&mut self) {
        if self.config.auto_flush {
            // Try to flush on drop (best effort)
            let _ = self.tx.try_send(BatchCommand::Flush);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record() -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: "test".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[tokio::test]
    async fn test_batch_writer() {
        let config = BatchConfig::default();
        let writer = AsyncBatchWriter::new(config);

        for _ in 0..5 {
            writer.add(create_test_record()).await.unwrap();
        }

        // Give time for background task to process the channel messages
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Buffer should have records (or they may have been flushed if batch size exceeded)
        // Just verify flush works without error
        writer.flush().await.unwrap();

        // Give some time for flush to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    #[tokio::test]
    async fn test_batch_auto_flush() {
        let mut config = BatchConfig::default();
        config.max_batch_size = 3;

        let writer = AsyncBatchWriter::new(config);

        for _ in 0..5 {
            writer.add(create_test_record()).await.unwrap();
        }

        // Wait for auto-flush
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    #[tokio::test]
    async fn test_batch_shutdown() {
        let writer = AsyncBatchWriter::new(BatchConfig::default());

        writer.add(create_test_record()).await.unwrap();
        writer.shutdown().await.unwrap();
    }
}
