//! Simulation-as-a-Service API
//!
//! This module provides an API for running simulations as a service,
//! including job queuing, result storage, and webhook notifications.

use crate::{SimResult, SimulationMetrics};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Priority level for simulation jobs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum JobPriority {
    /// Low priority (background jobs)
    Low = 0,
    /// Normal priority (default)
    #[default]
    Normal = 1,
    /// High priority (urgent jobs)
    High = 2,
    /// Critical priority (immediate execution)
    Critical = 3,
}

/// Status of a simulation job
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    /// Job is queued and waiting for execution
    Queued,
    /// Job is currently running
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed with an error
    Failed,
    /// Job was cancelled
    Cancelled,
}

/// A simulation job in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationJob {
    /// Unique job identifier
    pub id: Uuid,
    /// Job name/description
    pub name: String,
    /// Job priority
    pub priority: JobPriority,
    /// Job status
    pub status: JobStatus,
    /// Time job was created
    pub created_at: DateTime<Utc>,
    /// Time job started running (if applicable)
    pub started_at: Option<DateTime<Utc>>,
    /// Time job completed (if applicable)
    pub completed_at: Option<DateTime<Utc>>,
    /// Webhook URL to call when job completes
    pub webhook_url: Option<String>,
    /// Job configuration (JSON serialized)
    pub config: serde_json::Value,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl SimulationJob {
    /// Create a new simulation job
    pub fn new(name: impl Into<String>, config: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            priority: JobPriority::default(),
            status: JobStatus::Queued,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            webhook_url: None,
            config,
            error: None,
        }
    }

    /// Set the job priority
    pub fn with_priority(mut self, priority: JobPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set the webhook URL
    pub fn with_webhook(mut self, url: impl Into<String>) -> Self {
        self.webhook_url = Some(url.into());
        self
    }

    /// Mark job as started
    pub fn start(&mut self) {
        self.status = JobStatus::Running;
        self.started_at = Some(Utc::now());
    }

    /// Mark job as completed
    pub fn complete(&mut self) {
        self.status = JobStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Mark job as failed with an error message
    pub fn fail(&mut self, error: impl Into<String>) {
        self.status = JobStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error = Some(error.into());
    }

    /// Mark job as cancelled
    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
        self.completed_at = Some(Utc::now());
    }

    /// Calculate job duration in milliseconds
    pub fn duration_ms(&self) -> Option<i64> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).num_milliseconds()),
            _ => None,
        }
    }
}

// Internal wrapper for priority queue ordering
#[derive(Debug)]
struct JobWrapper {
    job: SimulationJob,
    sequence: u64, // For stable ordering within same priority
}

impl PartialEq for JobWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.job.priority == other.job.priority && self.sequence == other.sequence
    }
}

impl Eq for JobWrapper {}

impl PartialOrd for JobWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JobWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then lower sequence (earlier submission)
        match self.job.priority.cmp(&other.job.priority) {
            std::cmp::Ordering::Equal => other.sequence.cmp(&self.sequence),
            other => other,
        }
    }
}

/// Job queue for managing simulation jobs
#[derive(Debug)]
pub struct JobQueue {
    queue: BinaryHeap<JobWrapper>,
    jobs: HashMap<Uuid, SimulationJob>,
    sequence_counter: u64,
}

impl JobQueue {
    /// Create a new job queue
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            jobs: HashMap::new(),
            sequence_counter: 0,
        }
    }

    /// Submit a new job to the queue
    pub fn submit(&mut self, job: SimulationJob) -> Uuid {
        let id = job.id;
        let wrapper = JobWrapper {
            job: job.clone(),
            sequence: self.sequence_counter,
        };
        self.sequence_counter += 1;
        self.queue.push(wrapper);
        self.jobs.insert(id, job);
        id
    }

    /// Get the next job from the queue (highest priority)
    pub fn pop(&mut self) -> Option<SimulationJob> {
        self.queue.pop().map(|wrapper| {
            let mut job = wrapper.job;
            job.start();
            self.jobs.insert(job.id, job.clone());
            job
        })
    }

    /// Get a job by ID
    pub fn get(&self, id: &Uuid) -> Option<&SimulationJob> {
        self.jobs.get(id)
    }

    /// Get a mutable reference to a job by ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut SimulationJob> {
        self.jobs.get_mut(id)
    }

    /// Cancel a job by ID
    pub fn cancel(&mut self, id: &Uuid) -> bool {
        if let Some(job) = self.jobs.get_mut(id)
            && job.status == JobStatus::Queued
        {
            job.cancel();
            return true;
        }
        false
    }

    /// Get all jobs
    pub fn all_jobs(&self) -> Vec<&SimulationJob> {
        self.jobs.values().collect()
    }

    /// Get jobs by status
    pub fn jobs_by_status(&self, status: JobStatus) -> Vec<&SimulationJob> {
        self.jobs
            .values()
            .filter(|job| job.status == status)
            .collect()
    }

    /// Get queue size
    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }

    /// Get total job count
    pub fn total_jobs(&self) -> usize {
        self.jobs.len()
    }

    /// Clear completed jobs
    pub fn clear_completed(&mut self) {
        self.jobs
            .retain(|_, job| job.status != JobStatus::Completed);
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Storage for simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// Unique result identifier
    pub id: Uuid,
    /// Associated job ID
    pub job_id: Uuid,
    /// Simulation metrics
    pub metrics: SimulationMetrics,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Timestamp when result was stored
    pub stored_at: DateTime<Utc>,
}

impl SimulationResult {
    /// Create a new simulation result
    pub fn new(job_id: Uuid, metrics: SimulationMetrics) -> Self {
        Self {
            id: Uuid::new_v4(),
            job_id,
            metrics,
            metadata: HashMap::new(),
            stored_at: Utc::now(),
        }
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

/// Result storage system
#[derive(Debug)]
pub struct ResultStorage {
    results: HashMap<Uuid, SimulationResult>,
    storage_path: Option<PathBuf>,
}

impl ResultStorage {
    /// Create a new result storage
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            storage_path: None,
        }
    }

    /// Create result storage with file persistence
    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        Self {
            results: HashMap::new(),
            storage_path: Some(path.into()),
        }
    }

    /// Store a simulation result
    pub fn store(&mut self, result: SimulationResult) -> SimResult<Uuid> {
        let id = result.id;

        // Store in memory
        self.results.insert(id, result.clone());

        // Persist to disk if path is configured
        if let Some(ref path) = self.storage_path {
            let file_path = path.join(format!("{}.json", id));
            let json = serde_json::to_string_pretty(&result)?;
            std::fs::create_dir_all(path)?;
            std::fs::write(&file_path, json)?;
        }

        Ok(id)
    }

    /// Get a result by ID
    pub fn get(&self, id: &Uuid) -> Option<&SimulationResult> {
        self.results.get(id)
    }

    /// Get all results for a job
    pub fn get_by_job(&self, job_id: &Uuid) -> Vec<&SimulationResult> {
        self.results
            .values()
            .filter(|r| r.job_id == *job_id)
            .collect()
    }

    /// Get all results
    pub fn all_results(&self) -> Vec<&SimulationResult> {
        self.results.values().collect()
    }

    /// Delete a result by ID
    pub fn delete(&mut self, id: &Uuid) -> bool {
        // Delete from disk if path is configured
        if let Some(ref path) = self.storage_path {
            let file_path = path.join(format!("{}.json", id));
            let _ = std::fs::remove_file(file_path);
        }

        self.results.remove(id).is_some()
    }

    /// Load results from disk
    pub fn load_all(&mut self) -> SimResult<usize> {
        if let Some(ref path) = self.storage_path {
            if !path.exists() {
                return Ok(0);
            }

            let mut count = 0;
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let json = std::fs::read_to_string(&path)?;
                    let result: SimulationResult = serde_json::from_str(&json)?;
                    self.results.insert(result.id, result);
                    count += 1;
                }
            }

            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// Clear all results
    pub fn clear(&mut self) {
        self.results.clear();
    }

    /// Get result count
    pub fn count(&self) -> usize {
        self.results.len()
    }
}

impl Default for ResultStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Webhook notification system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotification {
    /// Job ID
    pub job_id: Uuid,
    /// Job status
    pub status: JobStatus,
    /// Result ID (if completed successfully)
    pub result_id: Option<Uuid>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl WebhookNotification {
    /// Create a new webhook notification
    pub fn new(job_id: Uuid, status: JobStatus) -> Self {
        Self {
            job_id,
            status,
            result_id: None,
            error: None,
            timestamp: Utc::now(),
        }
    }

    /// Set result ID
    pub fn with_result_id(mut self, result_id: Uuid) -> Self {
        self.result_id = Some(result_id);
        self
    }

    /// Set error message
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

/// Webhook delivery system
#[derive(Debug)]
pub struct WebhookDelivery {
    pending: VecDeque<(String, WebhookNotification)>,
    delivered: Vec<(String, WebhookNotification, DateTime<Utc>)>,
}

impl WebhookDelivery {
    /// Create a new webhook delivery system
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            delivered: Vec::new(),
        }
    }

    /// Queue a webhook notification
    pub fn queue(&mut self, url: String, notification: WebhookNotification) {
        self.pending.push_back((url, notification));
    }

    /// Get next pending webhook
    pub fn pop_pending(&mut self) -> Option<(String, WebhookNotification)> {
        self.pending.pop_front()
    }

    /// Mark a webhook as delivered
    pub fn mark_delivered(&mut self, url: String, notification: WebhookNotification) {
        self.delivered.push((url, notification, Utc::now()));
    }

    /// Get pending count
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Get delivered count
    pub fn delivered_count(&self) -> usize {
        self.delivered.len()
    }

    /// Get all delivered webhooks
    pub fn get_delivered(&self) -> &[(String, WebhookNotification, DateTime<Utc>)] {
        &self.delivered
    }
}

impl Default for WebhookDelivery {
    fn default() -> Self {
        Self::new()
    }
}

/// Comparison API for comparing simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationComparison {
    /// Comparison ID
    pub id: Uuid,
    /// Result IDs being compared
    pub result_ids: Vec<Uuid>,
    /// Comparison metrics
    pub metrics: HashMap<String, serde_json::Value>,
    /// Timestamp
    pub created_at: DateTime<Utc>,
}

impl SimulationComparison {
    /// Create a new comparison
    pub fn new(result_ids: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            result_ids,
            metrics: HashMap::new(),
            created_at: Utc::now(),
        }
    }

    /// Add a metric to the comparison
    pub fn add_metric(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.metrics.insert(key.into(), value);
    }
}

/// API for comparing simulation results
#[derive(Debug)]
pub struct ComparisonAPI {
    comparisons: HashMap<Uuid, SimulationComparison>,
}

impl ComparisonAPI {
    /// Create a new comparison API
    pub fn new() -> Self {
        Self {
            comparisons: HashMap::new(),
        }
    }

    /// Compare two or more simulation results
    pub fn compare(&mut self, storage: &ResultStorage, result_ids: Vec<Uuid>) -> SimResult<Uuid> {
        if result_ids.len() < 2 {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Need at least 2 results to compare".to_string(),
            ));
        }

        // Verify all results exist
        for id in &result_ids {
            if storage.get(id).is_none() {
                return Err(crate::SimulationError::ExecutionError(format!(
                    "Result {} not found",
                    id
                )));
            }
        }

        let mut comparison = SimulationComparison::new(result_ids.clone());

        // Calculate comparison metrics
        let results: Vec<_> = result_ids.iter().filter_map(|id| storage.get(id)).collect();

        // Compare total applications
        let applications: Vec<_> = results
            .iter()
            .map(|r| r.metrics.total_applications)
            .collect();
        comparison.add_metric("total_applications", serde_json::json!(applications));

        // Compare deterministic vs discretion counts
        let deterministic: Vec<_> = results
            .iter()
            .map(|r| r.metrics.deterministic_count)
            .collect();
        comparison.add_metric("deterministic_count", serde_json::json!(deterministic));

        let discretion: Vec<_> = results.iter().map(|r| r.metrics.discretion_count).collect();
        comparison.add_metric("discretion_count", serde_json::json!(discretion));

        // Compare averages
        if !applications.is_empty() {
            let avg = applications.iter().sum::<usize>() as f64 / applications.len() as f64;
            comparison.add_metric("avg_applications", serde_json::json!(avg));
        }

        let id = comparison.id;
        self.comparisons.insert(id, comparison);
        Ok(id)
    }

    /// Get a comparison by ID
    pub fn get(&self, id: &Uuid) -> Option<&SimulationComparison> {
        self.comparisons.get(id)
    }

    /// Get all comparisons
    pub fn all_comparisons(&self) -> Vec<&SimulationComparison> {
        self.comparisons.values().collect()
    }

    /// Delete a comparison
    pub fn delete(&mut self, id: &Uuid) -> bool {
        self.comparisons.remove(id).is_some()
    }
}

impl Default for ComparisonAPI {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulation-as-a-Service API
#[derive(Debug)]
pub struct SimulationAPI {
    job_queue: Arc<Mutex<JobQueue>>,
    result_storage: Arc<Mutex<ResultStorage>>,
    webhook_delivery: Arc<Mutex<WebhookDelivery>>,
    comparison_api: Arc<Mutex<ComparisonAPI>>,
}

impl SimulationAPI {
    /// Create a new simulation API
    pub fn new() -> Self {
        Self {
            job_queue: Arc::new(Mutex::new(JobQueue::new())),
            result_storage: Arc::new(Mutex::new(ResultStorage::new())),
            webhook_delivery: Arc::new(Mutex::new(WebhookDelivery::new())),
            comparison_api: Arc::new(Mutex::new(ComparisonAPI::new())),
        }
    }

    /// Create API with persistent storage
    pub fn with_storage(path: impl Into<PathBuf>) -> Self {
        Self {
            job_queue: Arc::new(Mutex::new(JobQueue::new())),
            result_storage: Arc::new(Mutex::new(ResultStorage::with_path(path))),
            webhook_delivery: Arc::new(Mutex::new(WebhookDelivery::new())),
            comparison_api: Arc::new(Mutex::new(ComparisonAPI::new())),
        }
    }

    /// Submit a simulation job
    pub fn submit_job(&self, job: SimulationJob) -> Uuid {
        let mut queue = self.job_queue.lock().unwrap();
        queue.submit(job)
    }

    /// Get next job from queue
    pub fn get_next_job(&self) -> Option<SimulationJob> {
        let mut queue = self.job_queue.lock().unwrap();
        queue.pop()
    }

    /// Get job status
    pub fn get_job(&self, id: &Uuid) -> Option<SimulationJob> {
        let queue = self.job_queue.lock().unwrap();
        queue.get(id).cloned()
    }

    /// Cancel a job
    pub fn cancel_job(&self, id: &Uuid) -> bool {
        let mut queue = self.job_queue.lock().unwrap();
        queue.cancel(id)
    }

    /// Complete a job and store result
    pub fn complete_job(&self, job_id: Uuid, metrics: SimulationMetrics) -> SimResult<Uuid> {
        // Update job status
        {
            let mut queue = self.job_queue.lock().unwrap();
            if let Some(job) = queue.get_mut(&job_id) {
                job.complete();
            }
        }

        // Store result
        let result = SimulationResult::new(job_id, metrics);
        let result_id = {
            let mut storage = self.result_storage.lock().unwrap();
            storage.store(result)?
        };

        // Queue webhook notification
        {
            let queue = self.job_queue.lock().unwrap();
            if let Some(job) = queue.get(&job_id)
                && let Some(ref webhook_url) = job.webhook_url
            {
                let notification = WebhookNotification::new(job_id, JobStatus::Completed)
                    .with_result_id(result_id);
                let mut delivery = self.webhook_delivery.lock().unwrap();
                delivery.queue(webhook_url.clone(), notification);
            }
        }

        Ok(result_id)
    }

    /// Fail a job
    pub fn fail_job(&self, job_id: Uuid, error: impl Into<String>) -> SimResult<()> {
        let error_msg = error.into();

        // Update job status
        {
            let mut queue = self.job_queue.lock().unwrap();
            if let Some(job) = queue.get_mut(&job_id) {
                job.fail(error_msg.clone());
            }
        }

        // Queue webhook notification
        {
            let queue = self.job_queue.lock().unwrap();
            if let Some(job) = queue.get(&job_id)
                && let Some(ref webhook_url) = job.webhook_url
            {
                let notification =
                    WebhookNotification::new(job_id, JobStatus::Failed).with_error(error_msg);
                let mut delivery = self.webhook_delivery.lock().unwrap();
                delivery.queue(webhook_url.clone(), notification);
            }
        }

        Ok(())
    }

    /// Get simulation result
    pub fn get_result(&self, id: &Uuid) -> Option<SimulationResult> {
        let storage = self.result_storage.lock().unwrap();
        storage.get(id).cloned()
    }

    /// Get all results for a job
    pub fn get_job_results(&self, job_id: &Uuid) -> Vec<SimulationResult> {
        let storage = self.result_storage.lock().unwrap();
        storage.get_by_job(job_id).into_iter().cloned().collect()
    }

    /// Compare simulation results
    pub fn compare_results(&self, result_ids: Vec<Uuid>) -> SimResult<Uuid> {
        let storage = self.result_storage.lock().unwrap();
        let mut comparison_api = self.comparison_api.lock().unwrap();
        comparison_api.compare(&storage, result_ids)
    }

    /// Get comparison
    pub fn get_comparison(&self, id: &Uuid) -> Option<SimulationComparison> {
        let comparison_api = self.comparison_api.lock().unwrap();
        comparison_api.get(id).cloned()
    }

    /// Get next pending webhook
    pub fn get_next_webhook(&self) -> Option<(String, WebhookNotification)> {
        let mut delivery = self.webhook_delivery.lock().unwrap();
        delivery.pop_pending()
    }

    /// Mark webhook as delivered
    pub fn mark_webhook_delivered(&self, url: String, notification: WebhookNotification) {
        let mut delivery = self.webhook_delivery.lock().unwrap();
        delivery.mark_delivered(url, notification);
    }

    /// Get queue statistics
    pub fn get_queue_stats(&self) -> QueueStats {
        let queue = self.job_queue.lock().unwrap();
        QueueStats {
            total_jobs: queue.total_jobs(),
            queued: queue.jobs_by_status(JobStatus::Queued).len(),
            running: queue.jobs_by_status(JobStatus::Running).len(),
            completed: queue.jobs_by_status(JobStatus::Completed).len(),
            failed: queue.jobs_by_status(JobStatus::Failed).len(),
            cancelled: queue.jobs_by_status(JobStatus::Cancelled).len(),
        }
    }
}

impl Default for SimulationAPI {
    fn default() -> Self {
        Self::new()
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub total_jobs: usize,
    pub queued: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub cancelled: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_priority() {
        assert!(JobPriority::Critical > JobPriority::High);
        assert!(JobPriority::High > JobPriority::Normal);
        assert!(JobPriority::Normal > JobPriority::Low);
    }

    #[test]
    fn test_job_creation() {
        let config = serde_json::json!({"test": "data"});
        let job = SimulationJob::new("test job", config.clone());

        assert_eq!(job.name, "test job");
        assert_eq!(job.status, JobStatus::Queued);
        assert_eq!(job.priority, JobPriority::Normal);
        assert_eq!(job.config, config);
    }

    #[test]
    fn test_job_with_priority() {
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config).with_priority(JobPriority::High);

        assert_eq!(job.priority, JobPriority::High);
    }

    #[test]
    fn test_job_with_webhook() {
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config).with_webhook("https://example.com/webhook");

        assert_eq!(
            job.webhook_url,
            Some("https://example.com/webhook".to_string())
        );
    }

    #[test]
    fn test_job_lifecycle() {
        let config = serde_json::json!({});
        let mut job = SimulationJob::new("test", config);

        assert_eq!(job.status, JobStatus::Queued);
        assert!(job.started_at.is_none());

        job.start();
        assert_eq!(job.status, JobStatus::Running);
        assert!(job.started_at.is_some());

        job.complete();
        assert_eq!(job.status, JobStatus::Completed);
        assert!(job.completed_at.is_some());
        assert!(job.duration_ms().is_some());
    }

    #[test]
    fn test_job_failure() {
        let config = serde_json::json!({});
        let mut job = SimulationJob::new("test", config);

        job.start();
        job.fail("Test error");

        assert_eq!(job.status, JobStatus::Failed);
        assert_eq!(job.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_job_queue_submit() {
        let mut queue = JobQueue::new();
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config);
        let id = job.id;

        let submitted_id = queue.submit(job);
        assert_eq!(id, submitted_id);
        assert_eq!(queue.total_jobs(), 1);
    }

    #[test]
    fn test_job_queue_priority() {
        let mut queue = JobQueue::new();

        let low = SimulationJob::new("low", serde_json::json!({})).with_priority(JobPriority::Low);
        let high =
            SimulationJob::new("high", serde_json::json!({})).with_priority(JobPriority::High);
        let normal =
            SimulationJob::new("normal", serde_json::json!({})).with_priority(JobPriority::Normal);

        queue.submit(low);
        queue.submit(high.clone());
        queue.submit(normal);

        // Should get high priority first
        let next = queue.pop().unwrap();
        assert_eq!(next.name, "high");
        assert_eq!(next.status, JobStatus::Running);
    }

    #[test]
    fn test_job_queue_cancel() {
        let mut queue = JobQueue::new();
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config);
        let id = queue.submit(job);

        assert!(queue.cancel(&id));

        let cancelled = queue.get(&id).unwrap();
        assert_eq!(cancelled.status, JobStatus::Cancelled);
    }

    #[test]
    fn test_result_storage() {
        let mut storage = ResultStorage::new();
        let metrics = SimulationMetrics::default();
        let job_id = Uuid::new_v4();
        let result = SimulationResult::new(job_id, metrics);

        let id = storage.store(result).unwrap();
        assert!(storage.get(&id).is_some());
        assert_eq!(storage.count(), 1);
    }

    #[test]
    fn test_result_storage_get_by_job() {
        let mut storage = ResultStorage::new();
        let job_id = Uuid::new_v4();

        let result1 = SimulationResult::new(job_id, SimulationMetrics::default());
        let result2 = SimulationResult::new(job_id, SimulationMetrics::default());

        storage.store(result1).unwrap();
        storage.store(result2).unwrap();

        let results = storage.get_by_job(&job_id);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_webhook_notification() {
        let job_id = Uuid::new_v4();
        let result_id = Uuid::new_v4();

        let notification =
            WebhookNotification::new(job_id, JobStatus::Completed).with_result_id(result_id);

        assert_eq!(notification.job_id, job_id);
        assert_eq!(notification.status, JobStatus::Completed);
        assert_eq!(notification.result_id, Some(result_id));
    }

    #[test]
    fn test_webhook_delivery() {
        let mut delivery = WebhookDelivery::new();
        let notification = WebhookNotification::new(Uuid::new_v4(), JobStatus::Completed);

        delivery.queue("https://example.com".to_string(), notification.clone());
        assert_eq!(delivery.pending_count(), 1);

        let (url, notif) = delivery.pop_pending().unwrap();
        assert_eq!(url, "https://example.com");
        assert_eq!(delivery.pending_count(), 0);

        delivery.mark_delivered(url, notif);
        assert_eq!(delivery.delivered_count(), 1);
    }

    #[test]
    fn test_comparison_api() {
        let mut storage = ResultStorage::new();
        let mut api = ComparisonAPI::new();

        let job_id1 = Uuid::new_v4();
        let job_id2 = Uuid::new_v4();

        let result1 = SimulationResult::new(job_id1, SimulationMetrics::default());
        let result2 = SimulationResult::new(job_id2, SimulationMetrics::default());

        let id1 = storage.store(result1).unwrap();
        let id2 = storage.store(result2).unwrap();

        let comparison_id = api.compare(&storage, vec![id1, id2]).unwrap();
        let comparison = api.get(&comparison_id).unwrap();

        assert_eq!(comparison.result_ids.len(), 2);
        assert!(comparison.metrics.contains_key("total_applications"));
    }

    #[test]
    fn test_comparison_api_requires_two_results() {
        let storage = ResultStorage::new();
        let mut api = ComparisonAPI::new();

        let result = api.compare(&storage, vec![Uuid::new_v4()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_simulation_api() {
        let api = SimulationAPI::new();
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config);

        api.submit_job(job);

        let stats = api.get_queue_stats();
        assert_eq!(stats.total_jobs, 1);
        assert_eq!(stats.queued, 1);
    }

    #[test]
    fn test_simulation_api_complete_job() {
        let api = SimulationAPI::new();
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config);
        let job_id = job.id;

        api.submit_job(job);

        let metrics = SimulationMetrics::default();
        let result_id = api.complete_job(job_id, metrics).unwrap();

        assert!(api.get_result(&result_id).is_some());

        let stats = api.get_queue_stats();
        assert_eq!(stats.completed, 1);
    }

    #[test]
    fn test_simulation_api_fail_job() {
        let api = SimulationAPI::new();
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config);

        let job_id = api.submit_job(job);
        api.fail_job(job_id, "Test error").unwrap();

        let stats = api.get_queue_stats();
        assert_eq!(stats.failed, 1);
    }

    #[test]
    fn test_simulation_api_with_webhook() {
        let api = SimulationAPI::new();
        let config = serde_json::json!({});
        let job = SimulationJob::new("test", config).with_webhook("https://example.com/webhook");
        let job_id = job.id;

        api.submit_job(job);

        let metrics = SimulationMetrics::default();
        api.complete_job(job_id, metrics).unwrap();

        let webhook = api.get_next_webhook();
        assert!(webhook.is_some());

        let (url, notification) = webhook.unwrap();
        assert_eq!(url, "https://example.com/webhook");
        assert_eq!(notification.status, JobStatus::Completed);
    }

    #[test]
    fn test_simulation_api_compare_results() {
        let api = SimulationAPI::new();

        let config = serde_json::json!({});
        let job1 = SimulationJob::new("test1", config.clone());
        let job2 = SimulationJob::new("test2", config);
        let job_id1 = job1.id;
        let job_id2 = job2.id;

        api.submit_job(job1);
        api.submit_job(job2);

        let result_id1 = api
            .complete_job(job_id1, SimulationMetrics::default())
            .unwrap();
        let result_id2 = api
            .complete_job(job_id2, SimulationMetrics::default())
            .unwrap();

        let comparison_id = api.compare_results(vec![result_id1, result_id2]).unwrap();
        let comparison = api.get_comparison(&comparison_id).unwrap();

        assert_eq!(comparison.result_ids.len(), 2);
    }
}
