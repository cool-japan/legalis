//! Simulation Orchestration
//!
//! This module provides advanced job orchestration features including
//! retry logic, timeout handling, batch execution, and parameter sweeps.

use crate::SimulationJob;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Retry configuration for failed jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay in milliseconds
    pub initial_delay_ms: u64,
    /// Backoff multiplier (e.g., 2.0 for exponential backoff)
    pub backoff_multiplier: f64,
    /// Maximum delay in milliseconds
    pub max_delay_ms: u64,
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: 60000,
        }
    }

    /// Create retry config with exponential backoff
    pub fn exponential(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: 60000,
        }
    }

    /// Create retry config with linear backoff
    pub fn linear(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: 1000,
            backoff_multiplier: 1.0,
            max_delay_ms: 60000,
        }
    }

    /// Calculate delay for a given attempt number
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0;
        }

        let delay =
            self.initial_delay_ms as f64 * self.backoff_multiplier.powi((attempt - 1) as i32);
        delay.min(self.max_delay_ms as f64) as u64
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self::new(3)
    }
}

/// Job execution attempt record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAttempt {
    /// Attempt number (1-indexed)
    pub attempt: u32,
    /// When this attempt started
    pub started_at: DateTime<Utc>,
    /// When this attempt ended
    pub ended_at: Option<DateTime<Utc>>,
    /// Error message if failed
    pub error: Option<String>,
    /// Whether this attempt succeeded
    pub succeeded: bool,
}

impl ExecutionAttempt {
    /// Create a new execution attempt
    pub fn new(attempt: u32) -> Self {
        Self {
            attempt,
            started_at: Utc::now(),
            ended_at: None,
            error: None,
            succeeded: false,
        }
    }

    /// Mark attempt as successful
    pub fn succeed(&mut self) {
        self.succeeded = true;
        self.ended_at = Some(Utc::now());
    }

    /// Mark attempt as failed
    pub fn fail(&mut self, error: impl Into<String>) {
        self.succeeded = false;
        self.error = Some(error.into());
        self.ended_at = Some(Utc::now());
    }

    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> Option<i64> {
        self.ended_at
            .map(|end| (end - self.started_at).num_milliseconds())
    }
}

/// Retryable job wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryableJob {
    /// The underlying job
    pub job: SimulationJob,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Execution attempts
    pub attempts: Vec<ExecutionAttempt>,
    /// Next retry time (if scheduled)
    pub next_retry_at: Option<DateTime<Utc>>,
}

impl RetryableJob {
    /// Create a new retryable job
    pub fn new(job: SimulationJob, retry_config: RetryConfig) -> Self {
        Self {
            job,
            retry_config,
            attempts: Vec::new(),
            next_retry_at: None,
        }
    }

    /// Start a new attempt
    pub fn start_attempt(&mut self) -> u32 {
        let attempt_num = (self.attempts.len() + 1) as u32;
        let attempt = ExecutionAttempt::new(attempt_num);
        self.attempts.push(attempt);
        attempt_num
    }

    /// Mark current attempt as successful
    pub fn succeed_attempt(&mut self) {
        if let Some(attempt) = self.attempts.last_mut() {
            attempt.succeed();
        }
    }

    /// Mark current attempt as failed and schedule retry if possible
    pub fn fail_attempt(&mut self, error: impl Into<String>) -> bool {
        if let Some(attempt) = self.attempts.last_mut() {
            attempt.fail(error);
        }

        // Check if we should retry
        if self.attempts.len() < self.retry_config.max_attempts as usize {
            let delay_ms = self
                .retry_config
                .calculate_delay(self.attempts.len() as u32);
            self.next_retry_at = Some(Utc::now() + Duration::milliseconds(delay_ms as i64));
            true
        } else {
            false
        }
    }

    /// Check if ready for retry
    pub fn ready_for_retry(&self) -> bool {
        if let Some(next_retry) = self.next_retry_at {
            Utc::now() >= next_retry
        } else {
            false
        }
    }

    /// Get total attempts
    pub fn total_attempts(&self) -> u32 {
        self.attempts.len() as u32
    }

    /// Check if job has exhausted retries
    pub fn exhausted(&self) -> bool {
        self.attempts.len() >= self.retry_config.max_attempts as usize
            && self.attempts.last().map(|a| !a.succeeded).unwrap_or(false)
    }
}

/// Timeout configuration for jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Timeout duration in milliseconds
    pub timeout_ms: u64,
    /// Action to take on timeout
    pub action: TimeoutAction,
}

impl TimeoutConfig {
    /// Create a new timeout configuration
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            timeout_ms,
            action: TimeoutAction::Fail,
        }
    }

    /// Set timeout action
    pub fn with_action(mut self, action: TimeoutAction) -> Self {
        self.action = action;
        self
    }

    /// Check if a job has timed out
    pub fn is_timed_out(&self, started_at: DateTime<Utc>) -> bool {
        let elapsed = (Utc::now() - started_at).num_milliseconds();
        elapsed > self.timeout_ms as i64
    }
}

/// Action to take when a job times out
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeoutAction {
    /// Fail the job
    Fail,
    /// Cancel the job
    Cancel,
    /// Retry the job
    Retry,
}

/// Job with timeout tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedJob {
    /// The underlying job
    pub job: SimulationJob,
    /// Timeout configuration
    pub timeout_config: TimeoutConfig,
    /// Whether timeout has been triggered
    pub timed_out: bool,
}

impl TimedJob {
    /// Create a new timed job
    pub fn new(job: SimulationJob, timeout_config: TimeoutConfig) -> Self {
        Self {
            job,
            timeout_config,
            timed_out: false,
        }
    }

    /// Check if job has timed out
    pub fn check_timeout(&mut self) -> bool {
        if self.timed_out {
            return true;
        }

        if let Some(started_at) = self.job.started_at {
            self.timed_out = self.timeout_config.is_timed_out(started_at);
            self.timed_out
        } else {
            false
        }
    }
}

/// Job dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDependency {
    /// Job ID that this job depends on
    pub depends_on: Uuid,
    /// Whether to wait for success or just completion
    pub wait_for_success: bool,
}

impl JobDependency {
    /// Create a new dependency
    pub fn new(depends_on: Uuid) -> Self {
        Self {
            depends_on,
            wait_for_success: true,
        }
    }

    /// Create a dependency that only waits for completion
    pub fn completion_only(depends_on: Uuid) -> Self {
        Self {
            depends_on,
            wait_for_success: false,
        }
    }
}

/// Job with dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependentJob {
    /// The underlying job
    pub job: SimulationJob,
    /// Dependencies
    pub dependencies: Vec<JobDependency>,
}

impl DependentJob {
    /// Create a new dependent job
    pub fn new(job: SimulationJob) -> Self {
        Self {
            job,
            dependencies: Vec::new(),
        }
    }

    /// Add a dependency
    pub fn add_dependency(mut self, dependency: JobDependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Add a simple dependency (wait for success)
    pub fn depends_on(mut self, job_id: Uuid) -> Self {
        self.dependencies.push(JobDependency::new(job_id));
        self
    }
}

/// Batch job executor
#[derive(Debug)]
pub struct BatchExecutor {
    jobs: Vec<DependentJob>,
    completed: HashMap<Uuid, bool>, // job_id -> success
}

impl BatchExecutor {
    /// Create a new batch executor
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            completed: HashMap::new(),
        }
    }

    /// Add a job to the batch
    pub fn add_job(&mut self, job: DependentJob) {
        self.jobs.push(job);
    }

    /// Get jobs ready to execute
    pub fn get_ready_jobs(&self) -> Vec<&DependentJob> {
        self.jobs
            .iter()
            .filter(|job| {
                // Skip if already completed
                if self.completed.contains_key(&job.job.id) {
                    return false;
                }

                // Check if all dependencies are satisfied
                job.dependencies
                    .iter()
                    .all(|dep| match self.completed.get(&dep.depends_on) {
                        Some(&success) => !dep.wait_for_success || success,
                        None => false,
                    })
            })
            .collect()
    }

    /// Mark a job as completed
    pub fn mark_completed(&mut self, job_id: Uuid, success: bool) {
        self.completed.insert(job_id, success);
    }

    /// Check if all jobs are completed
    pub fn is_complete(&self) -> bool {
        self.jobs.len() == self.completed.len()
    }

    /// Get completion statistics
    pub fn get_stats(&self) -> BatchStats {
        let total = self.jobs.len();
        let completed = self.completed.len();
        let succeeded = self.completed.values().filter(|&&s| s).count();
        let failed = self.completed.values().filter(|&&s| !s).count();

        BatchStats {
            total,
            completed,
            succeeded,
            failed,
            pending: total - completed,
        }
    }
}

impl Default for BatchExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Batch execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStats {
    pub total: usize,
    pub completed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub pending: usize,
}

/// Parameter sweep configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSweep {
    /// Parameter name
    pub name: String,
    /// Values to sweep
    pub values: Vec<serde_json::Value>,
}

impl ParameterSweep {
    /// Create a new parameter sweep
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: Vec::new(),
        }
    }

    /// Add a value to the sweep
    pub fn add_value(mut self, value: serde_json::Value) -> Self {
        self.values.push(value);
        self
    }

    /// Add multiple values
    pub fn with_values(mut self, values: Vec<serde_json::Value>) -> Self {
        self.values = values;
        self
    }

    /// Create a numeric range sweep
    pub fn numeric_range(name: impl Into<String>, start: f64, end: f64, steps: usize) -> Self {
        let mut sweep = Self::new(name);
        let step_size = (end - start) / (steps - 1).max(1) as f64;

        for i in 0..steps {
            let value = start + step_size * i as f64;
            sweep.values.push(serde_json::json!(value));
        }

        sweep
    }
}

/// Multi-parameter sweep orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSweepOrchestrator {
    /// Parameter sweeps
    pub sweeps: Vec<ParameterSweep>,
    /// Base job configuration
    pub base_config: serde_json::Value,
}

impl ParameterSweepOrchestrator {
    /// Create a new orchestrator
    pub fn new(base_config: serde_json::Value) -> Self {
        Self {
            sweeps: Vec::new(),
            base_config,
        }
    }

    /// Add a parameter sweep
    pub fn add_sweep(mut self, sweep: ParameterSweep) -> Self {
        self.sweeps.push(sweep);
        self
    }

    /// Generate all job configurations
    pub fn generate_configs(&self) -> Vec<serde_json::Value> {
        if self.sweeps.is_empty() {
            return vec![self.base_config.clone()];
        }

        self.generate_recursive(0, self.base_config.clone())
    }

    fn generate_recursive(
        &self,
        sweep_idx: usize,
        current_config: serde_json::Value,
    ) -> Vec<serde_json::Value> {
        if sweep_idx >= self.sweeps.len() {
            return vec![current_config];
        }

        let sweep = &self.sweeps[sweep_idx];
        let mut configs = Vec::new();

        for value in &sweep.values {
            let mut config = current_config.clone();
            // Update the parameter in the config
            if let Some(obj) = config.as_object_mut() {
                obj.insert(sweep.name.clone(), value.clone());
            }

            // Recurse to next parameter
            configs.extend(self.generate_recursive(sweep_idx + 1, config));
        }

        configs
    }

    /// Get total number of configurations
    pub fn total_configs(&self) -> usize {
        if self.sweeps.is_empty() {
            return 1;
        }

        self.sweeps.iter().map(|s| s.values.len()).product()
    }
}

/// Job execution history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHistory {
    /// Job ID
    pub job_id: Uuid,
    /// All execution attempts
    pub attempts: Vec<ExecutionAttempt>,
    /// Final status
    pub final_status: String,
    /// Total duration across all attempts
    pub total_duration_ms: i64,
}

impl ExecutionHistory {
    /// Create from retryable job
    pub fn from_retryable_job(job: &RetryableJob, final_status: impl Into<String>) -> Self {
        let total_duration = job.attempts.iter().filter_map(|a| a.duration_ms()).sum();

        Self {
            job_id: job.job.id,
            attempts: job.attempts.clone(),
            final_status: final_status.into(),
            total_duration_ms: total_duration,
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.attempts.is_empty() {
            return 0.0;
        }

        let successes = self.attempts.iter().filter(|a| a.succeeded).count();
        successes as f64 / self.attempts.len() as f64
    }

    /// Get average duration per attempt
    pub fn avg_duration_ms(&self) -> f64 {
        if self.attempts.is_empty() {
            return 0.0;
        }

        let total: i64 = self.attempts.iter().filter_map(|a| a.duration_ms()).sum();
        total as f64 / self.attempts.len() as f64
    }
}

/// Execution history tracker
#[derive(Debug)]
pub struct HistoryTracker {
    histories: HashMap<Uuid, ExecutionHistory>,
}

impl HistoryTracker {
    /// Create a new history tracker
    pub fn new() -> Self {
        Self {
            histories: HashMap::new(),
        }
    }

    /// Add a history record
    pub fn add_history(&mut self, history: ExecutionHistory) {
        self.histories.insert(history.job_id, history);
    }

    /// Get history for a job
    pub fn get_history(&self, job_id: &Uuid) -> Option<&ExecutionHistory> {
        self.histories.get(job_id)
    }

    /// Get all histories
    pub fn all_histories(&self) -> Vec<&ExecutionHistory> {
        self.histories.values().collect()
    }

    /// Get overall statistics
    pub fn get_overall_stats(&self) -> HistoryStats {
        let total_jobs = self.histories.len();
        let successful = self
            .histories
            .values()
            .filter(|h| h.final_status == "completed")
            .count();
        let failed = self
            .histories
            .values()
            .filter(|h| h.final_status == "failed")
            .count();

        let avg_attempts = if total_jobs > 0 {
            self.histories
                .values()
                .map(|h| h.attempts.len())
                .sum::<usize>() as f64
                / total_jobs as f64
        } else {
            0.0
        };

        let avg_duration = if total_jobs > 0 {
            self.histories
                .values()
                .map(|h| h.total_duration_ms)
                .sum::<i64>() as f64
                / total_jobs as f64
        } else {
            0.0
        };

        HistoryStats {
            total_jobs,
            successful,
            failed,
            avg_attempts,
            avg_duration_ms: avg_duration,
        }
    }
}

impl Default for HistoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Overall history statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_jobs: usize,
    pub successful: usize,
    pub failed: usize,
    pub avg_attempts: f64,
    pub avg_duration_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_exponential() {
        let config = RetryConfig::exponential(5);
        assert_eq!(config.calculate_delay(0), 0);
        assert_eq!(config.calculate_delay(1), 1000);
        assert_eq!(config.calculate_delay(2), 2000);
        assert_eq!(config.calculate_delay(3), 4000);
        assert_eq!(config.calculate_delay(4), 8000);
    }

    #[test]
    fn test_retry_config_linear() {
        let config = RetryConfig::linear(5);
        assert_eq!(config.calculate_delay(0), 0);
        assert_eq!(config.calculate_delay(1), 1000);
        assert_eq!(config.calculate_delay(2), 1000);
        assert_eq!(config.calculate_delay(3), 1000);
    }

    #[test]
    fn test_retry_config_max_delay() {
        let mut config = RetryConfig::exponential(10);
        config.max_delay_ms = 5000;

        assert_eq!(config.calculate_delay(1), 1000);
        assert_eq!(config.calculate_delay(2), 2000);
        assert_eq!(config.calculate_delay(3), 4000);
        assert_eq!(config.calculate_delay(4), 5000); // capped
        assert_eq!(config.calculate_delay(5), 5000); // capped
    }

    #[test]
    fn test_execution_attempt() {
        let mut attempt = ExecutionAttempt::new(1);
        assert!(!attempt.succeeded);
        assert!(attempt.error.is_none());

        attempt.succeed();
        assert!(attempt.succeeded);
        assert!(attempt.duration_ms().is_some());
    }

    #[test]
    fn test_execution_attempt_failure() {
        let mut attempt = ExecutionAttempt::new(1);
        attempt.fail("Test error");

        assert!(!attempt.succeeded);
        assert_eq!(attempt.error, Some("Test error".to_string()));
        assert!(attempt.duration_ms().is_some());
    }

    #[test]
    fn test_retryable_job() {
        let job = SimulationJob::new("test", serde_json::json!({}));
        let config = RetryConfig::new(3);
        let mut retryable = RetryableJob::new(job, config);

        assert_eq!(retryable.total_attempts(), 0);

        retryable.start_attempt();
        assert_eq!(retryable.total_attempts(), 1);

        let should_retry = retryable.fail_attempt("Error 1");
        assert!(should_retry);
        assert!(retryable.next_retry_at.is_some());
    }

    #[test]
    fn test_retryable_job_exhaustion() {
        let job = SimulationJob::new("test", serde_json::json!({}));
        let config = RetryConfig::new(2);
        let mut retryable = RetryableJob::new(job, config);

        retryable.start_attempt();
        let should_retry1 = retryable.fail_attempt("Error 1");
        assert!(should_retry1);

        retryable.start_attempt();
        let should_retry2 = retryable.fail_attempt("Error 2");
        assert!(!should_retry2);
        assert!(retryable.exhausted());
    }

    #[test]
    fn test_timeout_config() {
        let config = TimeoutConfig::new(5000);
        let started = Utc::now() - Duration::seconds(10);
        assert!(config.is_timed_out(started));

        let recent = Utc::now() - Duration::seconds(1);
        assert!(!config.is_timed_out(recent));
    }

    #[test]
    fn test_job_dependency() {
        let dep_id = Uuid::new_v4();
        let dep = JobDependency::new(dep_id);

        assert_eq!(dep.depends_on, dep_id);
        assert!(dep.wait_for_success);
    }

    #[test]
    fn test_dependent_job() {
        let job = SimulationJob::new("test", serde_json::json!({}));
        let dep_id = Uuid::new_v4();
        let dependent = DependentJob::new(job).depends_on(dep_id);

        assert_eq!(dependent.dependencies.len(), 1);
        assert_eq!(dependent.dependencies[0].depends_on, dep_id);
    }

    #[test]
    fn test_batch_executor() {
        let mut executor = BatchExecutor::new();

        let job1 = SimulationJob::new("job1", serde_json::json!({}));
        let job1_id = job1.id;
        executor.add_job(DependentJob::new(job1));

        let job2 = SimulationJob::new("job2", serde_json::json!({}));
        executor.add_job(DependentJob::new(job2).depends_on(job1_id));

        // Initially only job1 should be ready
        let ready = executor.get_ready_jobs();
        assert_eq!(ready.len(), 1);

        // After completing job1, job2 should be ready
        executor.mark_completed(job1_id, true);
        let ready = executor.get_ready_jobs();
        assert_eq!(ready.len(), 1);
    }

    #[test]
    fn test_batch_executor_stats() {
        let mut executor = BatchExecutor::new();

        let job1 = SimulationJob::new("job1", serde_json::json!({}));
        let job2 = SimulationJob::new("job2", serde_json::json!({}));

        executor.add_job(DependentJob::new(job1.clone()));
        executor.add_job(DependentJob::new(job2.clone()));

        executor.mark_completed(job1.id, true);
        executor.mark_completed(job2.id, false);

        let stats = executor.get_stats();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.completed, 2);
        assert_eq!(stats.succeeded, 1);
        assert_eq!(stats.failed, 1);
    }

    #[test]
    fn test_parameter_sweep() {
        let sweep = ParameterSweep::numeric_range("learning_rate", 0.001, 0.1, 5);

        assert_eq!(sweep.name, "learning_rate");
        assert_eq!(sweep.values.len(), 5);
    }

    #[test]
    fn test_parameter_sweep_orchestrator() {
        let base = serde_json::json!({"base": "config"});
        let orchestrator = ParameterSweepOrchestrator::new(base)
            .add_sweep(
                ParameterSweep::new("param1")
                    .with_values(vec![serde_json::json!(1), serde_json::json!(2)]),
            )
            .add_sweep(
                ParameterSweep::new("param2")
                    .with_values(vec![serde_json::json!("a"), serde_json::json!("b")]),
            );

        let configs = orchestrator.generate_configs();
        assert_eq!(configs.len(), 4); // 2 * 2
        assert_eq!(orchestrator.total_configs(), 4);
    }

    #[test]
    fn test_execution_history() {
        let job = SimulationJob::new("test", serde_json::json!({}));
        let config = RetryConfig::new(3);
        let mut retryable = RetryableJob::new(job, config);

        retryable.start_attempt();
        retryable.fail_attempt("Error 1");

        retryable.start_attempt();
        retryable.succeed_attempt();

        let history = ExecutionHistory::from_retryable_job(&retryable, "completed");
        assert_eq!(history.attempts.len(), 2);
        assert_eq!(history.final_status, "completed");
    }

    #[test]
    fn test_history_tracker() {
        let mut tracker = HistoryTracker::new();

        let history1 = ExecutionHistory {
            job_id: Uuid::new_v4(),
            attempts: vec![ExecutionAttempt::new(1)],
            final_status: "completed".to_string(),
            total_duration_ms: 1000,
        };

        let history2 = ExecutionHistory {
            job_id: Uuid::new_v4(),
            attempts: vec![ExecutionAttempt::new(1)],
            final_status: "failed".to_string(),
            total_duration_ms: 500,
        };

        tracker.add_history(history1.clone());
        tracker.add_history(history2);

        let stats = tracker.get_overall_stats();
        assert_eq!(stats.total_jobs, 2);
        assert_eq!(stats.successful, 1);
        assert_eq!(stats.failed, 1);
    }
}
