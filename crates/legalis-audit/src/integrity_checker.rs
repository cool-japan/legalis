//! Background integrity checking daemon.
//!
//! This module provides a background service that periodically verifies
//! the integrity of the audit trail.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;

/// Configuration for background integrity checking.
#[derive(Debug, Clone)]
pub struct IntegrityCheckerConfig {
    /// Interval between integrity checks (seconds)
    pub check_interval_secs: u64,
    /// Enable detailed logging
    pub enable_logging: bool,
    /// Stop on first error
    pub stop_on_error: bool,
    /// Maximum records to check per batch
    pub batch_size: Option<usize>,
}

impl Default for IntegrityCheckerConfig {
    fn default() -> Self {
        Self {
            check_interval_secs: 300, // 5 minutes
            enable_logging: true,
            stop_on_error: false,
            batch_size: Some(10000),
        }
    }
}

impl IntegrityCheckerConfig {
    /// Creates a new integrity checker configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the check interval.
    pub fn with_interval(mut self, secs: u64) -> Self {
        self.check_interval_secs = secs;
        self
    }

    /// Enables or disables logging.
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }

    /// Sets whether to stop on first error.
    pub fn stop_on_error(mut self, stop: bool) -> Self {
        self.stop_on_error = stop;
        self
    }

    /// Sets the batch size for checking.
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }
}

/// Integrity check result.
#[derive(Debug, Clone)]
pub struct IntegrityCheckResult {
    /// Timestamp of the check
    pub timestamp: DateTime<Utc>,
    /// Total records checked
    pub records_checked: usize,
    /// Number of errors found
    pub errors_found: usize,
    /// Check duration (milliseconds)
    pub duration_ms: u64,
    /// Whether the check passed
    pub passed: bool,
    /// Error details if any
    pub errors: Vec<IntegrityError>,
}

/// Integrity error details.
#[derive(Debug, Clone)]
pub struct IntegrityError {
    /// Record ID with the error
    pub record_id: uuid::Uuid,
    /// Error type
    pub error_type: IntegrityErrorType,
    /// Error message
    pub message: String,
}

/// Type of integrity error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityErrorType {
    /// Hash mismatch
    HashMismatch,
    /// Broken chain link
    BrokenChain,
    /// Missing record
    MissingRecord,
    /// Timestamp anomaly
    TimestampAnomaly,
}

/// Background integrity checker.
pub struct IntegrityChecker {
    config: IntegrityCheckerConfig,
    sender: mpsc::Sender<IntegrityCommand>,
    results: Arc<RwLock<Vec<IntegrityCheckResult>>>,
}

enum IntegrityCommand {
    Check,
    Shutdown,
}

impl IntegrityChecker {
    /// Creates a new background integrity checker.
    pub fn new<F>(config: IntegrityCheckerConfig, check_fn: F) -> Self
    where
        F: Fn() -> AuditResult<Vec<AuditRecord>> + Send + Sync + 'static,
    {
        let (sender, receiver) = mpsc::channel(10);
        let results = Arc::new(RwLock::new(Vec::new()));
        let results_clone = Arc::clone(&results);
        let config_clone = config.clone();

        tokio::spawn(async move {
            IntegrityWorker::new(receiver, check_fn, config_clone, results_clone)
                .run()
                .await;
        });

        Self {
            config,
            sender,
            results,
        }
    }

    /// Triggers a manual integrity check.
    pub async fn check_now(&self) -> AuditResult<()> {
        self.sender
            .send(IntegrityCommand::Check)
            .await
            .map_err(|_| {
                AuditError::StorageError("Integrity checker channel closed".to_string())
            })?;
        Ok(())
    }

    /// Shuts down the background checker.
    pub async fn shutdown(&self) -> AuditResult<()> {
        self.sender
            .send(IntegrityCommand::Shutdown)
            .await
            .map_err(|_| {
                AuditError::StorageError("Integrity checker channel closed".to_string())
            })?;
        Ok(())
    }

    /// Gets all check results.
    pub async fn get_results(&self) -> Vec<IntegrityCheckResult> {
        self.results.read().await.clone()
    }

    /// Gets the latest check result.
    pub async fn get_latest_result(&self) -> Option<IntegrityCheckResult> {
        self.results.read().await.last().cloned()
    }

    /// Clears all check results.
    pub async fn clear_results(&self) {
        self.results.write().await.clear();
    }

    /// Returns the checker configuration.
    pub fn config(&self) -> &IntegrityCheckerConfig {
        &self.config
    }
}

/// Internal integrity checker worker.
struct IntegrityWorker<F>
where
    F: Fn() -> AuditResult<Vec<AuditRecord>> + Send + Sync,
{
    receiver: mpsc::Receiver<IntegrityCommand>,
    check_fn: F,
    config: IntegrityCheckerConfig,
    results: Arc<RwLock<Vec<IntegrityCheckResult>>>,
}

impl<F> IntegrityWorker<F>
where
    F: Fn() -> AuditResult<Vec<AuditRecord>> + Send + Sync,
{
    fn new(
        receiver: mpsc::Receiver<IntegrityCommand>,
        check_fn: F,
        config: IntegrityCheckerConfig,
        results: Arc<RwLock<Vec<IntegrityCheckResult>>>,
    ) -> Self {
        Self {
            receiver,
            check_fn,
            config,
            results,
        }
    }

    async fn run(mut self) {
        let mut check_timer = interval(Duration::from_secs(self.config.check_interval_secs));
        check_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                cmd = self.receiver.recv() => {
                    match cmd {
                        Some(IntegrityCommand::Check) => {
                            self.perform_check().await;
                        }
                        Some(IntegrityCommand::Shutdown) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }

                _ = check_timer.tick() => {
                    self.perform_check().await;
                }
            }
        }
    }

    async fn perform_check(&self) {
        let start = std::time::Instant::now();
        let timestamp = Utc::now();

        if self.config.enable_logging {
            tracing::info!("Starting background integrity check");
        }

        let records = match (self.check_fn)() {
            Ok(records) => records,
            Err(e) => {
                tracing::error!("Failed to retrieve records for integrity check: {}", e);
                return;
            }
        };

        let mut errors = Vec::new();
        let mut expected_prev_hash: Option<String> = None;
        let records_checked = records.len();

        for record in &records {
            // Verify record hash
            if !record.verify() {
                errors.push(IntegrityError {
                    record_id: record.id,
                    error_type: IntegrityErrorType::HashMismatch,
                    message: format!("Record {} has invalid hash", record.id),
                });

                if self.config.stop_on_error {
                    break;
                }
            }

            // Verify chain
            if record.previous_hash != expected_prev_hash {
                errors.push(IntegrityError {
                    record_id: record.id,
                    error_type: IntegrityErrorType::BrokenChain,
                    message: format!("Record {} has broken chain link", record.id),
                });

                if self.config.stop_on_error {
                    break;
                }
            }

            expected_prev_hash = Some(record.record_hash.clone());
        }

        let duration = start.elapsed();
        let passed = errors.is_empty();

        let result = IntegrityCheckResult {
            timestamp,
            records_checked,
            errors_found: errors.len(),
            duration_ms: duration.as_millis() as u64,
            passed,
            errors,
        };

        if self.config.enable_logging {
            if passed {
                tracing::info!(
                    "Integrity check passed: {} records checked in {}ms",
                    records_checked,
                    result.duration_ms
                );
            } else {
                tracing::warn!(
                    "Integrity check found {} errors in {} records ({}ms)",
                    result.errors_found,
                    records_checked,
                    result.duration_ms
                );
            }
        }

        // Store result
        let mut results = self.results.write().await;
        results.push(result);

        // Keep only last 100 results
        let len = results.len();
        if len > 100 {
            results.drain(0..len - 100);
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

    #[tokio::test]
    async fn test_integrity_checker_basic() {
        let records = vec![create_test_record()];
        let records_clone = records.clone();

        let config = IntegrityCheckerConfig::new()
            .with_interval(60)
            .with_logging(false);

        let checker = IntegrityChecker::new(config, move || Ok(records_clone.clone()));

        // Trigger manual check
        checker.check_now().await.unwrap();

        // Wait for check to complete
        tokio::time::sleep(Duration::from_millis(100)).await;

        let results = checker.get_results().await;
        assert!(!results.is_empty());
        assert!(results[0].passed);

        checker.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_integrity_checker_multiple_checks() {
        let records = vec![create_test_record(), create_test_record()];
        let records_clone = records.clone();

        let config = IntegrityCheckerConfig::new()
            .with_interval(3600) // 1 hour to avoid periodic checks during test
            .with_logging(false);

        let checker = IntegrityChecker::new(config, move || Ok(records_clone.clone()));

        // Trigger multiple manual checks
        for _ in 0..3 {
            checker.check_now().await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let results = checker.get_results().await;
        assert!(results.len() >= 3); // At least 3 checks

        checker.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_integrity_checker_latest_result() {
        let records = vec![create_test_record()];
        let records_clone = records.clone();

        let config = IntegrityCheckerConfig::new().with_logging(false);

        let checker = IntegrityChecker::new(config, move || Ok(records_clone.clone()));

        checker.check_now().await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        let latest = checker.get_latest_result().await;
        assert!(latest.is_some());
        assert!(latest.unwrap().passed);

        checker.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_integrity_checker_clear_results() {
        let records = vec![create_test_record()];
        let records_clone = records.clone();

        let config = IntegrityCheckerConfig::new().with_logging(false);

        let checker = IntegrityChecker::new(config, move || Ok(records_clone.clone()));

        checker.check_now().await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        checker.clear_results().await;
        let results = checker.get_results().await;
        assert!(results.is_empty());

        checker.shutdown().await.unwrap();
    }
}
